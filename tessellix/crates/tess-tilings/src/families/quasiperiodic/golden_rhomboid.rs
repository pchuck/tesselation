//! Golden rhomboid-triangle — the Andritz φ-substitution.
//!
//! The Bielefeld Tilings Encyclopedia entry "golden-rhomboid-triangle":
//! three prototiles on the φ lattice, inflation λ = φ, category
//! Quasiperiodic; the rhomboid was discovered by Andritz.
//!
//!   Y  equilateral triangle, side 1       (area 1, in √3/4 units)
//!   O  equilateral triangle, side φ       (area φ²)
//!   R  60°/120° rhomboid, sides 1 × φ     (area 2φ)
//!
//! Substitution (area identities exact, in √3/4 units):
//!
//!   Y → O            φ² · 1          = φ²
//!   O → O + Y + R    φ² + 1 + 2φ     = φ⁴ = φ² · φ²
//!   R → R + 2O       2φ + 2φ²        = 2φ · φ²
//!
//! Count matrix in (R, O, Y) order (row = children, column = parent):
//!
//!   M = [[1, 1, 0],
//!        [2, 1, 1],
//!        [0, 1, 0]]
//!
//! characteristic polynomial λ³ − 2λ² − 2λ + 1 = 0; the
//! Perron–Frobenius eigenvalue is φ², an EXACT root (the battery pins
//! λ⁶ − 2λ⁴ − 2λ² + 1 = 0 at λ = φ² in Q(√5)).
//!
//! **Exact arithmetic.** A point is the integer 4-tuple
//! (a0, a1, b0, b1) ⇔ x = (a0 + a1·φ)/2, y = (b0 + b1·φ)·√3/2 — each
//! axis carries Q(φ), the full coordinate ring Q(√5, √3). Multiplication
//! by φ is linear: φ·(a0, a1) = (a1, a0 + a1) (φ² = φ + 1). Every
//! geometric fact (frames, rules, growth, all test predicates) is exact
//! integer arithmetic; f64 appears only at emission and in the view
//! anchor.
//!
//! **Frame semantics.** The 18 anchored orientation classes — Y_k
//! (k = 0..5, classes 0..5), O_k (6..11), and the 6 rhomboid classes
//! 12..17 (R_A = rotations of R₀, R_B = rotations of its mirror) — are
//! the substitution's working classes (FRAMES and RULES below). The Y
//! and O frames are equilateral triangles: a 120° rotation preserves
//! their vertex set, so {0,2,4}/{1,3,5} and {6,8,10}/{7,9,11} each
//! collapse to one unanchored set, while the 6 rhomboid frames have no
//! 120° self-symmetry and stay 6 distinct. Quotienting by the 120°
//! rotation gives 10 unanchored sets (2 + 2 + 6), used only as the
//! distinctness keys substitution conjugation resolves to — the dumped
//! RULES table is already resolved (e.g. rule 12's O-child is class 7,
//! not 9).
//!
//! **Covered domain.** Growth starts from the single seed rhomboid R₀
//! (class 12) at the origin: depth d tiles φ^d·R₀ exactly (region
//! corners = φ^d applied to R₀'s corners), and ⋃_d φ^d·R₀ is the 60°
//! wedge W = {y ≥ 0, y ≤ √3·x}. In the (s, t) coordinates
//! s = x − y/√3, t = 2y/(√3·φ) the wedge is the first quadrant and
//! φ^d·R₀ is the square [0, φ^d]², so a view hull with
//! s_max = x₁ − y₀/√3 and t_max = 2y₁/(√3·φ) is covered at the smallest
//! d with φ^d ≥ max(s_max, t_max). Views disjoint from W return empty
//! (background). The depth is capped at MAX_DEPTH = 12 (N₁₂ = 134209 ≤
//! MAX_TILES < N₁₃ = 351363).
//!
//! **T-junctions are intrinsic** (not a defect): the first appear at
//! depth 2 (2 T-points, 5 incidences) because a child's φ-edge carries
//! the parent's unit split points. At every T-point the angle closes
//! EXACTLY to π on both sides of the passing edge's line — in π/3
//! units, the passing edge counting as a straight angle (3) and every
//! corner its exact 60°/120° — pinned in the battery. The tiling is a
//! valid gap-free, overlap-free partition but is NOT face-to-edge.
//!
//! **Non-ergodic orbit.** From the seed, the reachable classes are
//! exactly {0, 1, 6, 7, 12, 14, 15, 16}; classes 2–5, 8–11, 13, 17
//! never appear (pinned by the depth-4 and depth-12 class censuses).
//!
//! **Cross-check (float, report only).** The Bielefeld patch.pdf fill
//! census: 1525 Y : 4012 O : 2477 R, rhomboid direction-pair classes
//! 1025/938/514 — verified by the reference battery
//! (`tests/golden_rhomboid_battery_reference.py`) against the parsed
//! PDF data; not part of the exact tests here.
//!
//! **Scope.** The substitution mechanics are fully verified in exact
//! arithmetic (battery in `mod golden_rhomboid_tests`, ported from the
//! salvage battery); the quasiperiodic-diffraction claim follows the
//! golden-family convention (see golden_bee's scope note).

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

// ---------------------------------------------------------------------------
// Exact arithmetic: points of the φ lattice as integer 4-tuples
// ---------------------------------------------------------------------------

/// φ as f64 — real embedding; emission and the view anchor only.
const PHI_FLT: f64 = 1.618033988749895;
/// √3 as f64 — same.
const SQRT3_FLT: f64 = 1.7320508075688772;
/// ln φ, for the depth anchor.
const LN_PHI: f64 = 0.48121182505960347;
/// Max refinement depth: N₁₂ = 134209 ≤ MAX_TILES < N₁₃ = 351363.
const MAX_DEPTH: u32 = 12;

/// A point of the φ lattice: (a0, a1, b0, b1) ⇔
/// x = (a0 + a1·φ)/2, y = (b0 + b1·φ)·√3/2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt(i64, i64, i64, i64);

impl Pt {
    const fn add(self, o: Self) -> Self {
        Pt(self.0 + o.0, self.1 + o.1, self.2 + o.2, self.3 + o.3)
    }
    #[cfg(test)]
    const fn sub(self, o: Self) -> Self {
        Pt(self.0 - o.0, self.1 - o.1, self.2 - o.2, self.3 - o.3)
    }
    /// Multiplication by φ: φ·(a0, a1) = (a1, a0 + a1), φ² = φ + 1.
    const fn phi(self) -> Self {
        Pt(self.1, self.0 + self.1, self.3, self.2 + self.3)
    }
    /// Real embedding (emission and test floats only).
    fn val(self) -> Vec2 {
        Vec2::new(
            0.5 * (self.0 as f64 + self.1 as f64 * PHI_FLT),
            0.5 * SQRT3_FLT * (self.2 as f64 + self.3 as f64 * PHI_FLT),
        )
    }
}

const PT0: Pt = Pt(0, 0, 0, 0);

// ---------------------------------------------------------------------------
// The 18 anchored orientation classes and the substitution rules
// ---------------------------------------------------------------------------

/// The 18 anchored orientation classes, CCW, all vertices exact.
/// Y_k = 0..5, O_k = 6..11, R_A_k = 12..14 (rotations of R₀), R_B_k =
/// 15..17 (rotations of the mirror of R₀; the origin is the LAST vertex
/// for 15..17, first for 12..14). The 12 triangular frames are padded
/// to 4 vertices with a repeat of the origin; `FRAME_VERTS` gives the
/// true vertex count — use `verts` everywhere.
const FRAMES: [[Pt; 4]; 18] = [
    // class 0 (Y)
    [
        Pt(0, 0, 0, 0),
        Pt(2, 0, 0, 0),
        Pt(1, 0, 1, 0),
        Pt(0, 0, 0, 0),
    ],
    // class 1 (Y)
    [
        Pt(0, 0, 0, 0),
        Pt(1, 0, 1, 0),
        Pt(-1, 0, 1, 0),
        Pt(0, 0, 0, 0),
    ],
    // class 2 (Y)
    [
        Pt(0, 0, 0, 0),
        Pt(-1, 0, 1, 0),
        Pt(-2, 0, 0, 0),
        Pt(0, 0, 0, 0),
    ],
    // class 3 (Y)
    [
        Pt(0, 0, 0, 0),
        Pt(-2, 0, 0, 0),
        Pt(-1, 0, -1, 0),
        Pt(0, 0, 0, 0),
    ],
    // class 4 (Y)
    [
        Pt(0, 0, 0, 0),
        Pt(-1, 0, -1, 0),
        Pt(1, 0, -1, 0),
        Pt(0, 0, 0, 0),
    ],
    // class 5 (Y)
    [
        Pt(0, 0, 0, 0),
        Pt(1, 0, -1, 0),
        Pt(2, 0, 0, 0),
        Pt(0, 0, 0, 0),
    ],
    // class 6 (O)
    [
        Pt(0, 0, 0, 0),
        Pt(0, 2, 0, 0),
        Pt(0, 1, 0, 1),
        Pt(0, 0, 0, 0),
    ],
    // class 7 (O)
    [
        Pt(0, 0, 0, 0),
        Pt(0, 1, 0, 1),
        Pt(0, -1, 0, 1),
        Pt(0, 0, 0, 0),
    ],
    // class 8 (O)
    [
        Pt(0, 0, 0, 0),
        Pt(0, -1, 0, 1),
        Pt(0, -2, 0, 0),
        Pt(0, 0, 0, 0),
    ],
    // class 9 (O)
    [
        Pt(0, 0, 0, 0),
        Pt(0, -2, 0, 0),
        Pt(0, -1, 0, -1),
        Pt(0, 0, 0, 0),
    ],
    // class 10 (O)
    [
        Pt(0, 0, 0, 0),
        Pt(0, -1, 0, -1),
        Pt(0, 1, 0, -1),
        Pt(0, 0, 0, 0),
    ],
    // class 11 (O)
    [
        Pt(0, 0, 0, 0),
        Pt(0, 1, 0, -1),
        Pt(0, 2, 0, 0),
        Pt(0, 0, 0, 0),
    ],
    // class 12 (R)
    [
        Pt(0, 0, 0, 0),
        Pt(2, 0, 0, 0),
        Pt(2, 1, 0, 1),
        Pt(0, 1, 0, 1),
    ],
    // class 13 (R)
    [
        Pt(0, 0, 0, 0),
        Pt(1, 0, 1, 0),
        Pt(1, -1, 1, 1),
        Pt(0, -1, 0, 1),
    ],
    // class 14 (R)
    [
        Pt(0, 0, 0, 0),
        Pt(-1, 0, 1, 0),
        Pt(-1, -2, 1, 0),
        Pt(0, -2, 0, 0),
    ],
    // class 15 (R)
    [
        Pt(0, 1, 0, -1),
        Pt(2, 1, 0, -1),
        Pt(2, 0, 0, 0),
        Pt(0, 0, 0, 0),
    ],
    // class 16 (R)
    [
        Pt(0, 2, 0, 0),
        Pt(1, 2, 1, 0),
        Pt(1, 0, 1, 0),
        Pt(0, 0, 0, 0),
    ],
    // class 17 (R)
    [
        Pt(0, 1, 0, 1),
        Pt(-1, 1, 1, 1),
        Pt(-1, 0, 1, 0),
        Pt(0, 0, 0, 0),
    ],
];

/// True vertex count per class (3 for the triangles, 4 for the rhomboids).
const FRAME_VERTS: [usize; 18] = [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4];

/// Substitution rules over the 18 anchored classes. `RULES[c]` lists
/// the children of a parent of class c as (child class, exact offset);
/// the children tile φ·(pos + FRAMES[c]) exactly. `RULES[c]` has
/// RULE_N[c] meaningful entries (1 for the Y rules, 3 for the rest);
/// the other slots are padding, never read.
const RULES: [[(u8, Pt); 3]; 18] = [
    // rule 0 (Y0):
    [
        (6, Pt(0, 0, 0, 0)),
        (0, Pt(0, 0, 0, 0)),
        (0, Pt(0, 0, 0, 0)),
    ],
    // rule 1 (Y1):
    [
        (7, Pt(0, 0, 0, 0)),
        (0, Pt(0, 0, 0, 0)),
        (0, Pt(0, 0, 0, 0)),
    ],
    // rule 2 (Y2):
    [
        (6, Pt(0, -2, 0, 0)),
        (0, Pt(0, 0, 0, 0)),
        (0, Pt(0, 0, 0, 0)),
    ],
    // rule 3 (Y3):
    [
        (7, Pt(0, -1, 0, -1)),
        (0, Pt(0, 0, 0, 0)),
        (0, Pt(0, 0, 0, 0)),
    ],
    // rule 4 (Y4):
    [
        (6, Pt(0, -1, 0, -1)),
        (0, Pt(0, 0, 0, 0)),
        (0, Pt(0, 0, 0, 0)),
    ],
    // rule 5 (Y5):
    [
        (7, Pt(0, 1, 0, -1)),
        (0, Pt(0, 0, 0, 0)),
        (0, Pt(0, 0, 0, 0)),
    ],
    // rule 6 (O0 → O Y R_B0):
    [
        (6, Pt(0, 0, 0, 0)),
        (0, Pt(0, 1, 0, 1)),
        (15, Pt(0, 1, 0, 1)),
    ],
    // rule 7 (O1 → O Y R_B1):
    [
        (7, Pt(0, 0, 0, 0)),
        (1, Pt(0, -1, 0, 1)),
        (16, Pt(0, -1, 0, 1)),
    ],
    // rule 8 (O2 → O Y R_B2):
    [
        (6, Pt(0, -2, 0, 0)),
        (0, Pt(-2, -2, 0, 0)),
        (17, Pt(0, -2, 0, 0)),
    ],
    // rule 9 (O3 → O Y R_B0):
    [
        (7, Pt(0, -1, 0, -1)),
        (1, Pt(-1, -1, -1, -1)),
        (15, Pt(-2, -2, 0, 0)),
    ],
    // rule 10 (O4 → O Y R_B1):
    [
        (6, Pt(0, -1, 0, -1)),
        (0, Pt(-1, 1, -1, -1)),
        (16, Pt(-1, -1, -1, -1)),
    ],
    // rule 11 (O5 → O Y R_B2):
    [
        (7, Pt(0, 1, 0, -1)),
        (1, Pt(1, 2, -1, 0)),
        (17, Pt(1, 1, -1, -1)),
    ],
    // rule 12 (R_A0 → O O R_B1):
    [
        (6, Pt(0, 0, 0, 0)),
        (7, Pt(0, 2, 0, 0)),
        (16, Pt(0, 1, 0, 1)),
    ],
    // rule 13 (R_A1 → O O R_B2):
    [
        (7, Pt(0, 0, 0, 0)),
        (6, Pt(0, -1, 0, 1)),
        (17, Pt(0, -1, 0, 1)),
    ],
    // rule 14 (R_A2 → O O R_B0):
    [
        (6, Pt(0, -2, 0, 0)),
        (7, Pt(0, -2, 0, 0)),
        (15, Pt(-2, -3, 0, 1)),
    ],
    // rule 15 (R_B0 → O O R_A2):
    [
        (7, Pt(0, 1, 0, -1)),
        (6, Pt(0, 1, 0, -1)),
        (14, Pt(1, 3, -1, -1)),
    ],
    // rule 16 (R_B1 → O O R_A0):
    [
        (6, Pt(0, 0, 0, 0)),
        (7, Pt(0, 2, 0, 0)),
        (12, Pt(0, 2, 0, 0)),
    ],
    // rule 17 (R_B2 → O O R_A1):
    [
        (7, Pt(0, 0, 0, 0)),
        (6, Pt(0, -1, 0, 1)),
        (13, Pt(0, 1, 0, 1)),
    ],
];

/// Number of real children per rule (1 for the Y rules, 3 for the rest).
const RULE_N: [u8; 18] = [1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3];

/// The true vertices of a class frame (the pad slot skipped).
fn verts(cls: u8) -> &'static [Pt] {
    &FRAMES[cls as usize][..FRAME_VERTS[cls as usize]]
}

// ---------------------------------------------------------------------------
// Growth
// ---------------------------------------------------------------------------

/// One exact tile at any depth: orientation class + anchor position.
/// The tile is {pos + v : v ∈ FRAMES[cls]} at unit scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GrPiece {
    cls: u8,
    pos: Pt,
}

/// One refinement step: the children tile φ·(pos + FRAMES[cls]) exactly.
fn refine(tiles: &[GrPiece]) -> Vec<GrPiece> {
    let mut out = Vec::with_capacity(tiles.len() * 3);
    for t in tiles {
        let base = t.pos.phi();
        for k in 0..RULE_N[t.cls as usize] {
            let (cc, off) = RULES[t.cls as usize][k as usize];
            out.push(GrPiece {
                cls: cc,
                pos: base.add(off),
            });
        }
    }
    out
}

// ---------------------------------------------------------------------------
// The tiling
// ---------------------------------------------------------------------------

/// Golden rhomboid-triangle — see the module docs.
///
/// Parameters: the shared similarity map with the default scale 1.0 —
/// natural for an unbounded tiling (the view anchor is the wedge, not a
/// finite patch to frame).
pub struct GoldenRhomboidTiling;

impl Tiling for GoldenRhomboidTiling {
    fn id(&self) -> &'static str {
        "golden-rhomboid"
    }

    fn name(&self) -> &'static str {
        "Golden rhomboid-triangle (Andritz φ-substitution)"
    }

    fn description(&self) -> &'static str {
        "Bielefeld golden-rhomboid-triangle (Andritz): equilateral \
         triangles side 1 and φ plus a 60°/120° rhomboid 1×φ; Y→O, \
         O→O+Y+R, R→R+2O; φ-inflation in exact Q(√5) 4-tuple points; 18 \
         anchored orientation classes (10 unanchored sets under the 120° \
         quotient); single-seed growth, depth d covers φ^d·R₀, the union \
         is the 60° wedge {y ≥ 0, y ≤ √3·x} (views outside are background); \
         T-junctions intrinsic, every T-point π-closed in π/3 units"
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
            "quasiperiodic (Andritz φ-substitution, Bielefeld \
             golden-rhomboid-triangle); single-seed unbounded growth — the \
             60° wedge {y ≥ 0, y ≤ √3·x} is covered (depth-capped), views \
             outside the wedge are background",
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

        // Covered domain: W = {y ≥ 0, y ≤ √3·x} (convex). An
        // axis-aligned box is disjoint from W iff x₁ < 0, or y₁ < 0, or
        // y₀ > √3·x₁ — necessary, since a box meeting W contains a point
        // with x ≥ 0, y ≥ 0 and y ≤ √3·x. Outside the wedge is
        // background.
        if rmax.x < 0.0 || rmax.y < 0.0 || rmin.y > SQRT3_FLT * rmax.x {
            return TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: None,
                message: None,
            };
        }

        // View-anchored depth: in the (s, t) coordinates
        // s = x − y/√3, t = 2y/(√3·φ) the wedge is the first quadrant
        // and φ^d·R₀ is the square [0, φ^d]² (R₀ is [0, 1]² there), so
        // the smallest d with φ^d ≥ max(s_max, t_max) covers
        // view ∩ W.
        let s_max = rmax.x - rmin.y / SQRT3_FLT;
        let t_max = 2.0 * rmax.y / (SQRT3_FLT * PHI_FLT);
        let need = s_max.max(t_max);
        let d_cov = if need > 1.0 {
            (need.ln() / LN_PHI).ceil() as u32
        } else {
            0
        };
        let d = d_cov.min(MAX_DEPTH);

        let mut pieces: Vec<GrPiece> = vec![GrPiece { cls: 12, pos: PT0 }];
        for _ in 0..d {
            pieces = refine(&pieces);
        }
        if pieces.len() > MAX_TILES {
            // Unreachable: d ≤ MAX_DEPTH bounds the count at
            // N₁₂ = 134209 < MAX_TILES. Defensive cap anyway.
            return TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: None,
                message: Some("view too deep for the tile cap"),
            };
        }

        // Emission: exact points → f64 embedding, culled to the view.
        // proto = orientation class, seed = class (one hue per
        // orientation class; there is no size hierarchy).
        let mut tiles = Vec::new();
        for t in &pieces {
            let pts: Vec<Vec2> = verts(t.cls).iter().map(|v| t.pos.add(*v).val()).collect();
            let poly = Polygon::new(pts).transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto: t.cls,
                    seed: t.cls as u32,
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

    // Window fully inside the wedge W and inside φ⁴·R₀:
    // s = x − y/√3 ∈ [1 − 1.5/√3, 5 − 0.2/√3] ≈ [0.134, 4.885],
    // t = 2y/(√3·φ) ∈ [0.146, 1.090]  →  depth 4, 61 tiles.
    const WIN: [f64; 4] = [1.0, 0.2, 5.0, 1.5];

    #[test]
    fn golden_rhomboid_covers_region() {
        check_covers(
            &GoldenRhomboidTiling,
            "golden-rhomboid",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }

    #[test]
    fn golden_rhomboid_rotated_scaled() {
        // The cell-space test rectangle [2.15, 2.85] × [1.05, 1.75]
        // (center (2.5, 1.4), half-extent 0.35), viewed at scale 0.6,
        // rotation 25°: its axis-aligned world hull is
        // ≈ [0.725388, 1.283537] × [1.116151, 1.674300]. The cell-space
        // hull of the view's inverse corners is
        // ≈ [1.88184, 3.11805] × [0.78191, 2.01811] — inside the wedge
        // and inside φ³·R₀ (depth 3, 23 tiles).
        let th = 25.0f64.to_radians();
        let (st, ct) = th.sin_cos();
        let cell = [(2.15, 1.05), (2.85, 1.05), (2.85, 1.75), (2.15, 1.75)];
        let mut x0 = f64::MAX;
        let mut x1 = f64::MIN;
        let mut y0 = f64::MAX;
        let mut y1 = f64::MIN;
        for &(cx, cy) in cell.iter() {
            // world = scale · Rot(θ) · cell (the shared similarity map)
            let wx = 0.6 * (ct * cx - st * cy);
            let wy = 0.6 * (st * cx + ct * cy);
            x0 = x0.min(wx);
            x1 = x1.max(wx);
            y0 = y0.min(wy);
            y1 = y1.max(wy);
        }
        check_rotated(
            &GoldenRhomboidTiling,
            "golden-rhomboid(rot25,scale0.6)",
            BBox::new(Vec2::new(x0, y0), Vec2::new(x1, y1)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn golden_rhomboid_deterministic() {
        check_det(
            &GoldenRhomboidTiling,
            "golden-rhomboid",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }
}

// ---------------------------------------------------------------------------
// Exact-arithmetic battery (port of the salvage battery; every predicate
// integer-exact, no floats)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod golden_rhomboid_tests {
    use super::*;
    use std::cmp::Ordering;
    use std::collections::{HashMap, HashSet};

    /// An element of Q(√5): u + v·φ (integers).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct P(i64, i64);

    impl P {
        const fn add(self, o: Self) -> Self {
            P(self.0 + o.0, self.1 + o.1)
        }
        const fn sub(self, o: Self) -> Self {
            P(self.0 - o.0, self.1 - o.1)
        }
        /// (u + vφ)(u₂ + v₂φ) = uu₂ + vv₂ + (uv₂ + vu₂ + vv₂)φ.
        const fn mul(self, o: Self) -> Self {
            let (u, v) = (self.0, self.1);
            let (u2, v2) = (o.0, o.1);
            P(u * u2 + v * v2, u * v2 + v * u2 + v * v2)
        }
    }

    /// Exact sign of u + v·φ (u, v integers): −1/0/+1.
    fn cmp_phi(u: i64, v: i64) -> i8 {
        if u == 0 && v == 0 {
            return 0;
        }
        if v == 0 {
            return if u > 0 { 1 } else { -1 };
        }
        if v < 0 {
            return -cmp_phi(-u, -v);
        }
        if u >= 0 {
            return 1;
        }
        let a = -u; // sign of a − v·φ, a > 0, v > 0
        let t = 2 * a - v;
        if t <= 0 {
            return 1;
        }
        if t * t < 5 * v * v {
            1
        } else {
            -1
        }
    }

    /// Exact lexicographic order (x then y) of two 4-tuple points.
    fn cmp_pt(a: Pt, b: Pt) -> i8 {
        let c = cmp_phi(a.0 - b.0, a.1 - b.1);
        if c != 0 {
            c
        } else {
            cmp_phi(a.2 - b.2, a.3 - b.3)
        }
    }

    fn ord3(c: i8) -> Ordering {
        match c {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            _ => Ordering::Greater,
        }
    }

    /// 2D cross product of the 4-tuple vectors p, q → P (exact).
    fn cross(p: Pt, q: Pt) -> P {
        P(p.0, p.1)
            .mul(P(q.2, q.3))
            .sub(P(q.0, q.1).mul(P(p.2, p.3)))
    }

    /// Exact sign of cross(b−a, c−a): −1/0/+1.
    fn orient(a: Pt, b: Pt, c: Pt) -> i8 {
        let v = cross(b.sub(a), c.sub(a));
        cmp_phi(v.0, v.1)
    }

    /// 4× the Euclidean dot product of 4-tuples u, v, as a P pair
    /// (the coordinates carry the 1/2 factor, the y-axis the √3/2
    /// factor): 4·dot = pair(ux)·pair(vx) + 3·pair(uy)·pair(vy).
    fn dot4(u: Pt, v: Pt) -> P {
        let dx = P(u.0, u.1).mul(P(v.0, v.1));
        let dy = P(u.2, u.3).mul(P(v.2, v.3));
        dx.add(P(3 * dy.0, 3 * dy.1))
    }

    /// 4·|q−p|² as a P pair (the y-axis carries √3, so |dy|²
    /// contributes 3·|dy-pair|²).
    fn edge_len2(p: Pt, q: Pt) -> P {
        let (dx0, dx1) = (q.0 - p.0, q.1 - p.1);
        let (dy0, dy1) = (q.2 - p.2, q.3 - p.3);
        let u = dx0 * dx0 + dx1 * dx1 + 3 * dy0 * dy0 + 3 * dy1 * dy1;
        let v = 2 * dx0 * dx1 + dx1 * dx1 + 6 * dy0 * dy1 + 3 * dy1 * dy1;
        P(u, v)
    }

    /// Shoelace sum S with area = (√3/8)·(u + vφ) for CCW polygons.
    fn shoelace(pts: &[Pt]) -> P {
        let mut s = P(0, 0);
        let n = pts.len();
        for i in 0..n {
            s = s.add(cross(pts[i], pts[(i + 1) % n]));
        }
        s
    }

    /// p in convex CCW polygon (closed boundary allowed).
    fn in_convex_closed(pts: &[Pt], p: Pt) -> bool {
        for i in 0..pts.len() {
            if orient(pts[i], pts[(i + 1) % pts.len()], p) < 0 {
                return false;
            }
        }
        true
    }

    /// p strictly inside convex CCW polygon.
    fn in_convex_strict(pts: &[Pt], p: Pt) -> bool {
        for i in 0..pts.len() {
            if orient(pts[i], pts[(i + 1) % pts.len()], p) <= 0 {
                return false;
            }
        }
        true
    }

    /// Proper crossing of the open segments a1a2 and b1b2 (exact).
    fn proper_cross(a1: Pt, a2: Pt, b1: Pt, b2: Pt) -> bool {
        let o1 = orient(a1, a2, b1);
        let o2 = orient(a1, a2, b2);
        if o1 == 0 || o2 == 0 {
            return false;
        }
        let o3 = orient(b1, b2, a1);
        let o4 = orient(b1, b2, a2);
        if o3 == 0 || o4 == 0 {
            return false;
        }
        (o1 > 0) != (o2 > 0) && (o3 > 0) != (o4 > 0)
    }

    /// v collinear with pq and strictly between (T-junction witness).
    /// No frame edge has equal x-pair endpoints, so the x-pair test is
    /// the between-test (a vertical edge would return false here —
    /// none exists among the 18 classes).
    fn vertex_strictly_on_seg(v: Pt, p: Pt, q: Pt) -> bool {
        if orient(p, q, v) != 0 {
            return false;
        }
        if cmp_phi(p.0 - q.0, p.1 - q.1) == 0 {
            return false;
        }
        let (lo, hi) = if cmp_pt(p, q) < 0 { (p, q) } else { (q, p) };
        cmp_pt(lo, v) < 0 && cmp_pt(v, hi) < 0
    }

    /// Collinear-contact classification of two closed segments.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Overlap {
        None,
        Point,
        FullA,
        FullB,
        Partial,
    }

    fn seg_overlap_full(a1: Pt, a2: Pt, b1: Pt, b2: Pt) -> Overlap {
        if orient(a1, a2, b1) != 0 || orient(a1, a2, b2) != 0 {
            return Overlap::None;
        }
        let (a1o, a2o) = if cmp_pt(a1, a2) < 0 {
            (a1, a2)
        } else {
            (a2, a1)
        };
        let (b1o, b2o) = if cmp_pt(b1, b2) < 0 {
            (b1, b2)
        } else {
            (b2, b1)
        };
        // lo is the LATER start, hi the EARLIER end.
        let lo = if cmp_pt(a1o, b1o) > 0 { a1o } else { b1o };
        let hi = if cmp_pt(a2o, b2o) < 0 { a2o } else { b2o };
        if cmp_pt(lo, hi) > 0 {
            return Overlap::None;
        }
        if cmp_pt(lo, hi) == 0 {
            return Overlap::Point;
        }
        if lo == a1o && hi == a2o {
            return Overlap::FullA;
        }
        if lo == b1o && hi == b2o {
            return Overlap::FullB;
        }
        Overlap::Partial
    }

    /// Consecutive cyclic edge pairs of a polygon.
    fn edges_of(pts: &[Pt]) -> Vec<(Pt, Pt)> {
        let n = pts.len();
        (0..n).map(|i| (pts[i], pts[(i + 1) % n])).collect()
    }

    /// Exact translation normalization: subtract the exact-min vertex,
    /// then sort the set by the exact lexicographic key — the
    /// unanchored-set distinctness key.
    fn norm_key(pts: &[Pt]) -> Vec<Pt> {
        let mut best = pts[0];
        for &p in pts.iter() {
            if cmp_pt(p, best) < 0 {
                best = p;
            }
        }
        let mut shifted: Vec<Pt> = pts.iter().map(|p| p.sub(best)).collect();
        shifted.sort_by(|a, b| ord3(cmp_pt(*a, *b)));
        shifted
    }

    /// Interior angle at vertex i of a class frame, in units of π/3
    /// (exact: the corner is exactly 60°/120°, guarded by
    /// 4·dot² == |w1|²·|w2|² in the 4× units).
    fn angle_units(cls: u8) -> [i8; 4] {
        let fr = verts(cls);
        let n = fr.len();
        let mut out = [1i8; 4];
        for i in 0..n {
            let prev = fr[(i + n - 1) % n];
            let cur = fr[i];
            let nxt = fr[(i + 1) % n];
            let w1 = cur.sub(prev); // into the vertex
            let w2 = nxt.sub(cur); // out of the vertex
            let d = dot4(w1, w2);
            let l1 = edge_len2(prev, cur);
            let l2 = edge_len2(cur, nxt);
            assert_eq!(
                P(4, 0).mul(d.mul(d)),
                l1.mul(l2),
                "corner not exactly 60/120 (class {cls}, vertex {i})"
            );
            // w1 in, w2 out: angle(w1,w2) = π − interior, so
            // d > 0  <=>  interior angle 120° (2 units).
            out[i] = if cmp_phi(d.0, d.1) > 0 { 2 } else { 1 };
        }
        out
    }

    // --- (1) frame shapes -------------------------------------------------

    #[test]
    fn frames_exact_shapes() {
        for c in 0..18u8 {
            let fr = verts(c);
            let s = shoelace(fr);
            assert!(cmp_phi(s.0, s.1) > 0, "class {c} not CCW");
            let want = if c < 6 {
                P(2, 0)
            } else if c < 12 {
                P(2, 2)
            } else {
                P(0, 4)
            };
            assert_eq!(s, want, "class {c} shoelace (area (√3/8)·S)");
            let ls: Vec<P> = (0..fr.len())
                .map(|i| edge_len2(fr[i], fr[(i + 1) % fr.len()]))
                .collect();
            let lwant = if c < 6 {
                vec![P(4, 0); 3]
            } else if c < 12 {
                vec![P(4, 4); 3]
            } else {
                vec![P(4, 0), P(4, 4), P(4, 0), P(4, 4)]
            };
            assert_eq!(ls, lwant, "class {c} edge lengths (4×|e|²)");
        }
        // Corner angles in units of π/3.
        for c in 0..18u8 {
            let fr = verts(c);
            let n = fr.len();
            let mut units: Vec<i8> = Vec::new();
            for i in 0..n {
                let w1 = fr[i].sub(fr[(i + n - 1) % n]);
                let w2 = fr[(i + 1) % n].sub(fr[i]);
                let d = dot4(w1, w2);
                let l1 = edge_len2(fr[(i + n - 1) % n], fr[i]);
                let l2 = edge_len2(fr[i], fr[(i + 1) % n]);
                assert_eq!(
                    P(4, 0).mul(d.mul(d)),
                    l1.mul(l2),
                    "corner not exactly 60/120 (class {c}, vertex {i})"
                );
                units.push(if cmp_phi(d.0, d.1) > 0 { 2 } else { 1 });
            }
            if c < 12 {
                assert_eq!(units, vec![1; n], "class {c} corner units");
            } else {
                let mut sorted = units.clone();
                sorted.sort();
                assert_eq!(sorted, vec![1, 1, 2, 2], "class {c} corner units");
            }
        }
    }

    // --- (2) unanchored-set census ----------------------------------------

    #[test]
    fn unanchored_frame_census_is_ten() {
        // Y: quotient by the 120° rotation — {0,2,4} and {1,3,5}
        // share one vertex set each.
        let y0 = norm_key(verts(0));
        let y1 = norm_key(verts(1));
        assert_eq!(y0, norm_key(verts(2)));
        assert_eq!(y0, norm_key(verts(4)));
        assert_eq!(y1, norm_key(verts(3)));
        assert_eq!(y1, norm_key(verts(5)));
        assert_ne!(y0, y1);
        // O: {6,8,10} and {7,9,11}.
        let o0 = norm_key(verts(6));
        let o1 = norm_key(verts(7));
        assert_eq!(o0, norm_key(verts(8)));
        assert_eq!(o0, norm_key(verts(10)));
        assert_eq!(o1, norm_key(verts(9)));
        assert_eq!(o1, norm_key(verts(11)));
        assert_ne!(o0, o1);
        // R: all 6 distinct, and no cross-family set collision.
        let all: Vec<Vec<Pt>> = (0..18).map(|c| norm_key(verts(c as u8))).collect();
        let mut seen: Vec<Vec<Pt>> = Vec::new();
        for k in all.iter() {
            if !seen.contains(k) {
                seen.push(k.clone());
            }
        }
        assert_eq!(seen.len(), 10, "unanchored sets (want 2 Y + 2 O + 6 R)");
        for r in [0u8, 1, 6, 7, 12, 13, 14, 15, 16, 17] {
            assert!(seen.contains(&norm_key(verts(r))), "orbit rep {r} missing");
        }
    }

    // --- (3) every rule tiles its inflated parent exactly ------------------

    fn children_of(entries: &[(u8, Pt); 3], n: u8) -> Vec<Vec<Pt>> {
        let mut out = Vec::new();
        for k in 0..n {
            let (cc, off) = entries[k as usize];
            out.push(verts(cc).iter().map(|v| off.add(*v)).collect());
        }
        out
    }

    fn verify_dissection(parent_cls: u8, entries: &[(u8, Pt); 3], n: u8, label: &str) {
        let parent = verts(parent_cls);
        let inflated: Vec<Pt> = parent.iter().map(|p| p.phi()).collect();
        assert_eq!(
            shoelace(&inflated),
            P(1, 1).mul(shoelace(parent)),
            "{label}: area law"
        );
        let ch = children_of(entries, n);
        // (a) child area sum
        let mut tot = P(0, 0);
        for poly in &ch {
            tot = tot.add(shoelace(poly));
        }
        assert_eq!(tot, P(1, 1).mul(shoelace(parent)), "{label}: area sum");
        // (b) each child inside the inflated parent (convex, vertex PIP)
        for poly in &ch {
            for &v in poly.iter() {
                assert!(
                    in_convex_closed(&inflated, v),
                    "{label}: child vertex outside parent"
                );
            }
        }
        // (c) pairwise: no proper crossing, no partial collinear
        //     overlap, no vertex strictly inside
        for i in 0..ch.len() {
            for j in (i + 1)..ch.len() {
                for &(a1, a2) in edges_of(&ch[i]).iter() {
                    for &(b1, b2) in edges_of(&ch[j]).iter() {
                        assert!(
                            !proper_cross(a1, a2, b1, b2),
                            "{label}: proper crossing ({i}/{j})"
                        );
                        let ov = seg_overlap_full(a1, a2, b1, b2);
                        assert!(
                            matches!(
                                ov,
                                Overlap::None | Overlap::Point | Overlap::FullA | Overlap::FullB
                            ),
                            "{label}: collinear overlap {ov:?} ({i}/{j})"
                        );
                    }
                }
                for &v in ch[i].iter() {
                    assert!(
                        !in_convex_strict(&ch[j], v),
                        "{label}: vertex strictly inside ({i}/{j})"
                    );
                }
                for &v in ch[j].iter() {
                    assert!(
                        !in_convex_strict(&ch[i], v),
                        "{label}: vertex strictly inside ({i}/{j})"
                    );
                }
            }
        }
        // (d) boundary: the once-occurring directed child edges
        //     partition each inflated parent edge (co-oriented,
        //     sequential, exact length)
        let mut cnt: HashMap<(Pt, Pt), i32> = HashMap::new();
        for poly in &ch {
            for (a, b) in edges_of(poly) {
                *cnt.entry((a, b)).or_insert(0) += 1;
            }
        }
        let boundary: Vec<(Pt, Pt)> = cnt
            .iter()
            .filter(|(_, n)| **n == 1)
            .map(|(ab, _)| *ab)
            .collect();
        for (p0, q0) in edges_of(&inflated) {
            let d = q0.sub(p0);
            let mut line_edges: Vec<(Pt, Pt)> = Vec::new();
            for &(a, b) in boundary.iter() {
                if orient(p0, q0, a) == 0 && orient(p0, q0, b) == 0 && orient(a, b, p0) == 0 {
                    // Same line: must be co-oriented (a→b parallel to
                    // P→Q).
                    let t = cross(d, b.sub(a));
                    assert_eq!(cmp_phi(t.0, t.1), 0, "{label}: boundary edge skew");
                    line_edges.push((a, b));
                }
            }
            assert!(!line_edges.is_empty(), "{label}: parent edge uncovered");
            // Exact projection parameter of (t − P) along d — a
            // positive scalar multiple of the along-edge parameter for
            // any d direction, so the order is correct for −x/−y
            // edges too.
            let along = |t: Pt| {
                let r = t.sub(p0);
                P(r.0, r.1)
                    .mul(P(d.0, d.1))
                    .add(P(3 * r.2, 3 * r.3).mul(P(3 * d.2, 3 * d.3)))
            };
            line_edges.sort_by(|ab, cd| {
                ord3(cmp_phi(
                    along(ab.0).sub(along(cd.0)).0,
                    along(ab.0).sub(along(cd.0)).1,
                ))
            });
            let ddom = d.0 != 0 || d.1 != 0;
            let want = if ddom {
                cmp_phi(d.0, d.1)
            } else {
                cmp_phi(d.2, d.3)
            };
            let mut tot_len = P(0, 0);
            let mut prev: Option<P> = None;
            for &(a, b) in line_edges.iter() {
                let va = if ddom {
                    P(a.0 - p0.0, a.1 - p0.1)
                } else {
                    P(a.2 - p0.2, a.3 - p0.3)
                };
                let vb = if ddom {
                    P(b.0 - p0.0, b.1 - p0.1)
                } else {
                    P(b.2 - p0.2, b.3 - p0.3)
                };
                // The edge must point along +d: sign of the edge vector
                // (b − a) on the dominant axis matches d's own.
                let sgn = if ddom {
                    cmp_phi(b.0 - a.0, b.1 - a.1)
                } else {
                    cmp_phi(b.2 - a.2, b.3 - a.3)
                };
                assert_eq!(sgn, want, "{label}: boundary edge anti-oriented");
                if let Some(pv) = prev {
                    assert_eq!(va, pv, "{label}: boundary gap/overlap on parent edge");
                }
                prev = Some(vb);
                tot_len = tot_len.add(vb.sub(va));
            }
            let wlen = if ddom { P(d.0, d.1) } else { P(d.2, d.3) };
            assert_eq!(tot_len, wlen, "{label}: boundary length mismatch");
        }
    }

    #[test]
    fn all_18_rules_tile_the_inflated_parent() {
        for c in 0..18u8 {
            verify_dissection(
                c,
                &RULES[c as usize],
                RULE_N[c as usize],
                &format!("rule {c}"),
            );
        }
    }

    // --- (4) growth: counts, area law, PF eigenvalue -----------------------

    /// Pinned (R, O, Y) count vectors, depth 0..12.
    const PINNED: [(i64, i64, i64); 13] = [
        (1, 0, 0),
        (1, 2, 0),
        (3, 4, 2),
        (7, 12, 4),
        (19, 30, 12),
        (49, 80, 30),
        (129, 208, 80),
        (337, 546, 208),
        (883, 1428, 546),
        (2311, 3740, 1428),
        (6051, 9790, 3740),
        (15841, 25632, 9790),
        (41473, 67104, 25632),
    ];
    /// Pinned totals N_d = r + o + y, depth 0..12.
    const PINNED_TOTALS: [usize; 13] = [
        1, 3, 9, 23, 61, 159, 417, 1091, 2857, 7479, 19581, 51263, 134209,
    ];

    fn census(tiles: &[GrPiece]) -> (i64, i64, i64) {
        let mut r = 0i64;
        let mut o = 0i64;
        let mut y = 0i64;
        for t in tiles {
            match t.cls {
                0..=5 => y += 1,
                6..=11 => o += 1,
                _ => r += 1,
            }
        }
        (r, o, y)
    }

    fn shoelace_total(tiles: &[GrPiece]) -> P {
        let mut s = P(0, 0);
        for t in tiles {
            s = s.add(shoelace(verts(t.cls)));
        }
        s
    }

    /// φ^k as a P pair, k ≥ 0 (power iteration, base (0,1) = φ).
    fn phi_pow_pair(k: u32) -> P {
        let mut r = P(1, 0);
        for _ in 0..k {
            r = r.mul(P(0, 1));
        }
        r
    }

    #[test]
    fn growth_counts_area_law_and_pf() {
        let mut tiles = vec![GrPiece { cls: 12, pos: PT0 }];
        for d in 0..13u32 {
            let (r, o, y) = census(&tiles);
            let (pr, po, py) = PINNED[d as usize];
            assert_eq!((r, o, y), (pr, po, py), "depth {d} count vector");
            assert_eq!(tiles.len(), PINNED_TOTALS[d as usize], "depth {d} total");
            // Recurrence read off the rules: R' = nO + nR,
            // O' = nY + nO + 2nR, Y' = nO.
            if d < 12 {
                let (nr, no, ny) = PINNED[(d + 1) as usize];
                assert_eq!(
                    (nr, no, ny),
                    (po + pr, py + po + 2 * pr, po),
                    "depth {d} recurrence"
                );
            }
            // Exact shoelace total: (0,4)·φ^{2d} (R₀ area (√3/8)·4,
            // ×φ² per depth).
            assert_eq!(
                shoelace_total(&tiles),
                P(0, 4).mul(phi_pow_pair(2 * d)),
                "depth {d} area law"
            );
            if d < 12 {
                tiles = refine(&tiles);
            }
        }
        // PF: characteristic polynomial λ³ − 2λ² − 2λ + 1; φ² is an
        // EXACT root (φ⁴ = 3φ+2, φ⁶ = 8φ+5 in Q(√5)).
        let ph2 = phi_pow_pair(2);
        let ph4 = phi_pow_pair(4);
        let ph6 = phi_pow_pair(6);
        assert_eq!(ph2, P(1, 1), "φ² = (1,1)");
        assert_eq!(ph4, P(2, 3), "φ⁴ = (2,3)");
        assert_eq!(ph6, P(5, 8), "φ⁶ = (5,8)");
        let p = ph6.sub(ph4.mul(P(2, 0))).sub(ph2.mul(P(2, 0))).add(P(1, 0));
        assert_eq!(p, P(0, 0), "φ² is an exact root of λ³−2λ²−2λ+1");
    }

    // --- (5) containment in the φ^d region ---------------------------------

    /// φ^d·R₀: the region corners at depth d (R₀ = class 12).
    fn region_corners(d: u32) -> Vec<Pt> {
        FRAMES[12]
            .iter()
            .map(|v| {
                let mut p = *v;
                for _ in 0..d {
                    p = p.phi();
                }
                p
            })
            .collect()
    }

    #[test]
    fn growth_stays_in_phi_d_region() {
        let mut tiles = vec![GrPiece { cls: 12, pos: PT0 }];
        for d in 0..13u32 {
            let reg = region_corners(d);
            for t in &tiles {
                for &v in verts(t.cls).iter() {
                    assert!(
                        in_convex_closed(&reg, t.pos.add(v)),
                        "depth {d}: tile outside φ^d·R₀"
                    );
                }
            }
            if d < 12 {
                tiles = refine(&tiles);
            }
        }
    }

    // --- (6) no crossings / no overlaps / T-junction closure ----------------

    fn tile_polys(tiles: &[GrPiece]) -> Vec<Vec<Pt>> {
        tiles
            .iter()
            .map(|t| verts(t.cls).iter().map(|v| t.pos.add(*v)).collect())
            .collect()
    }

    /// Sign of the side of line(line_p, line_q) that the convex tile
    /// poly lies on (convexity makes it the same for every off-line
    /// vertex); 0 only for a degenerate line-lying tile.
    fn off_line_vertex(line_p: Pt, line_q: Pt, poly: &[Pt]) -> i8 {
        for &v in poly.iter() {
            let o = orient(line_p, line_q, v);
            if o != 0 {
                return o;
            }
        }
        0
    }

    /// All T-junction witnesses (point, vertex_tile, passing_tile): a
    /// vertex of tile i strictly in the interior of an edge of tile j.
    fn collect_t_incidences(polys: &[Vec<Pt>]) -> Vec<(Pt, usize, usize)> {
        let edges: Vec<Vec<(Pt, Pt)>> = polys.iter().map(|p| edges_of(p)).collect();
        let mut inc = Vec::new();
        let n = polys.len();
        for i in 0..n {
            for j in (i + 1)..n {
                for &v in polys[i].iter() {
                    for &(b1, b2) in edges[j].iter() {
                        if vertex_strictly_on_seg(v, b1, b2) {
                            inc.push((v, i, j));
                        }
                    }
                }
                for &v in polys[j].iter() {
                    for &(a1, a2) in edges[i].iter() {
                        if vertex_strictly_on_seg(v, a1, a2) {
                            inc.push((v, j, i));
                        }
                    }
                }
            }
        }
        inc
    }

    /// Exact T-junction angle closure: at every T-point the angles close
    /// to EXACTLY π on both sides of the passing edge's line — the
    /// passing edge counts as a straight angle (3 units of π/3) on its
    /// side, every corner meeting at the point its exact 60°/120°.
    /// Returns the number of T-points.
    fn check_t_point_closure(
        tiles: &[GrPiece],
        polys: &[Vec<Pt>],
        inc: &[(Pt, usize, usize)],
        label: &str,
    ) -> usize {
        if inc.is_empty() {
            return 0;
        }
        let edges: Vec<Vec<(Pt, Pt)>> = polys.iter().map(|p| edges_of(p)).collect();
        // vmap: point → [(tile, vertex index)]
        let mut vmap: HashMap<Pt, Vec<(usize, usize)>> = HashMap::new();
        for (i, poly) in polys.iter().enumerate() {
            for (vi, v) in poly.iter().enumerate() {
                vmap.entry(*v).or_default().push((i, vi));
            }
        }
        let mut bypt: HashMap<Pt, Vec<(usize, usize)>> = HashMap::new();
        for &(p, i, j) in inc.iter() {
            bypt.entry(p).or_default().push((i, j));
        }
        for (p, ij) in bypt.iter() {
            // Passing edges, deduped (several vertex tiles may sit on
            // the same passing edge).
            let mut passings: Vec<(Pt, Pt, usize)> = Vec::new();
            let mut seen: HashSet<(usize, Pt, Pt)> = HashSet::new();
            for &(_, j) in ij.iter() {
                for &(b1, b2) in edges[j].iter() {
                    if vertex_strictly_on_seg(*p, b1, b2) {
                        let key = if cmp_pt(b1, b2) < 0 {
                            (j, b1, b2)
                        } else {
                            (j, b2, b1)
                        };
                        if seen.insert(key) {
                            passings.push((b1, b2, j));
                        }
                        break;
                    }
                }
            }
            assert!(
                !passings.is_empty(),
                "{label}: T-point with no passing edge"
            );
            let (l1, l2, _) = passings[0];
            for &(m1, m2, _) in passings.iter() {
                assert!(
                    orient(l1, l2, m1) == 0 && orient(l1, l2, m2) == 0,
                    "{label}: non-collinear passing edges at T-point"
                );
            }
            let pset: HashSet<usize> = passings.iter().map(|&(_, _, j)| j).collect();
            // filling[1] = side +1, filling[0] = side −1 (units π/3)
            let mut filling = [0i64; 2];
            for &(_, _, j) in passings.iter() {
                let s = off_line_vertex(l1, l2, &polys[j]);
                assert!(s != 0, "{label}: degenerate passing tile");
                filling[((s + 1) as usize) / 2] += 3;
            }
            if let Some(corners) = vmap.get(p) {
                for &(k, vi) in corners.iter() {
                    assert!(
                        !pset.contains(&k),
                        "{label}: corner and passing edge of one tile at T-point"
                    );
                    let poly = &polys[k];
                    let nn = poly.len();
                    let s1 = orient(l1, l2, poly[(vi + nn - 1) % nn]);
                    let s2 = orient(l1, l2, poly[(vi + 1) % nn]);
                    let side = if s1 == 0 && s2 == 0 {
                        panic!("{label}: degenerate corner at T-point (tile {k})")
                    } else if s1 == 0 {
                        s2
                    } else if s2 == 0 {
                        s1
                    } else if s1 == s2 {
                        s1
                    } else {
                        panic!("{label}: corner cone straddles T-line (overlap) (tile {k})")
                    };
                    filling[((side + 1) as usize) / 2] += angle_units(tiles[k].cls)[vi] as i64;
                }
            }
            assert_eq!(
                filling[1], 3,
                "{label}: T-point angle not closed, + side: {filling:?}"
            );
            assert_eq!(
                filling[0], 3,
                "{label}: T-point angle not closed, − side: {filling:?}"
            );
        }
        bypt.len()
    }

    /// Full O(n²) pairwise exact check: no proper crossings, no
    /// overlaps (collinear edge contacts require the tiles on OPPOSITE
    /// sides of the shared line), no vertex strictly inside another
    /// tile, and exact angle closure at every T-junction. Returns
    /// (tj-incidences, T-points).
    fn no_crossings_no_overlaps(tiles: &[GrPiece], label: &str) -> (i64, usize) {
        let polys = tile_polys(tiles);
        let edges: Vec<Vec<(Pt, Pt)>> = polys.iter().map(|p| edges_of(p)).collect();
        let n = tiles.len();
        let mut ncross = 0i64;
        let mut ntp = 0i64;
        let mut ntj = 0i64;
        for i in 0..n {
            for j in (i + 1)..n {
                for &(a1, a2) in edges[i].iter() {
                    for &(b1, b2) in edges[j].iter() {
                        if proper_cross(a1, a2, b1, b2) {
                            ncross += 1;
                        }
                        let ov = seg_overlap_full(a1, a2, b1, b2);
                        if matches!(ov, Overlap::Partial | Overlap::FullA | Overlap::FullB) {
                            // Collinear positive-length contact: legal
                            // only with the tiles on opposite sides of
                            // the shared line. The orient signs are
                            // per-edge-direction, so fold in the
                            // direction agreement via the sign of the
                            // exact dot of the direction vectors.
                            let si = off_line_vertex(a1, a2, &polys[i]);
                            let sj = off_line_vertex(b1, b2, &polys[j]);
                            let dd = dot4(a2.sub(a1), b2.sub(b1));
                            let sd = cmp_phi(dd.0, dd.1);
                            if sd == 0 || si * sj * sd >= 0 {
                                ntp += 1;
                            }
                        }
                    }
                }
                for &v in polys[i].iter() {
                    if in_convex_strict(&polys[j], v) {
                        ntp += 1;
                    }
                }
                for &v in polys[j].iter() {
                    if in_convex_strict(&polys[i], v) {
                        ntp += 1;
                    }
                }
                for &v in polys[i].iter() {
                    for &(b1, b2) in edges[j].iter() {
                        if vertex_strictly_on_seg(v, b1, b2) {
                            ntj += 1;
                        }
                    }
                }
                for &v in polys[j].iter() {
                    for &(a1, a2) in edges[i].iter() {
                        if vertex_strictly_on_seg(v, a1, a2) {
                            ntj += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(ncross, 0, "{label}: proper crossings");
        assert_eq!(ntp, 0, "{label}: overlaps/strict-inside");
        let inc = collect_t_incidences(&polys);
        let tp = check_t_point_closure(tiles, &polys, &inc, label);
        (ntj, tp)
    }

    #[test]
    fn no_crossings_no_overlaps_t_closure() {
        let mut tiles = vec![GrPiece { cls: 12, pos: PT0 }];
        for d in 0..5u32 {
            if (2..=4).contains(&d) {
                let (ntj, tp) = no_crossings_no_overlaps(&tiles, &format!("depth {d}"));
                // Pinned T-configurations: (T-points, tj-incidences).
                let want = match d {
                    2 => (2, 5),
                    3 => (8, 20),
                    _ => (30, 72),
                };
                assert_eq!((tp, ntj as usize), want, "depth {d} T-config");
            }
            if d < 4 {
                tiles = refine(&tiles);
            }
        }
    }

    // --- (7) the boundary partitions the region edge-to-edge ----------------

    fn check_boundary(tiles: &[GrPiece], d: u32, label: &str) {
        let mut cnt: HashMap<(Pt, Pt), i32> = HashMap::new();
        for t in tiles {
            let poly: Vec<Pt> = verts(t.cls).iter().map(|v| t.pos.add(*v)).collect();
            for (a, b) in edges_of(&poly) {
                *cnt.entry((a, b)).or_insert(0) += 1;
            }
        }
        let boundary: Vec<(Pt, Pt)> = cnt
            .iter()
            .filter(|(_, n)| **n == 1)
            .map(|(ab, _)| *ab)
            .collect();
        let reg = region_corners(d);
        for (p0, q0) in edges_of(&reg) {
            let dd = q0.sub(p0);
            let mut le: Vec<(Pt, Pt)> = Vec::new();
            for &(a, b) in boundary.iter() {
                if orient(p0, q0, a) == 0 && orient(p0, q0, b) == 0 {
                    le.push((a, b));
                }
            }
            assert!(!le.is_empty(), "{label}: uncovered region edge");
            let along = |t: Pt| {
                let r = t.sub(p0);
                P(r.0, r.1)
                    .mul(P(dd.0, dd.1))
                    .add(P(3 * r.2, 3 * r.3).mul(P(3 * dd.2, 3 * dd.3)))
            };
            le.sort_by(|ab, cd| {
                ord3(cmp_phi(
                    along(ab.0).sub(along(cd.0)).0,
                    along(ab.0).sub(along(cd.0)).1,
                ))
            });
            let ddom = dd.0 != 0 || dd.1 != 0;
            let want = if ddom {
                cmp_phi(dd.0, dd.1)
            } else {
                cmp_phi(dd.2, dd.3)
            };
            let mut tot = P(0, 0);
            let mut prev: Option<P> = None;
            for &(a, b) in le.iter() {
                let va = if ddom {
                    P(a.0 - p0.0, a.1 - p0.1)
                } else {
                    P(a.2 - p0.2, a.3 - p0.3)
                };
                let vb = if ddom {
                    P(b.0 - p0.0, b.1 - p0.1)
                } else {
                    P(b.2 - p0.2, b.3 - p0.3)
                };
                let sgn = if ddom {
                    cmp_phi(b.0 - a.0, b.1 - a.1)
                } else {
                    cmp_phi(b.2 - a.2, b.3 - a.3)
                };
                assert_eq!(sgn, want, "{label}: anti-oriented boundary edge");
                if let Some(pv) = prev {
                    assert_eq!(va, pv, "{label}: boundary gap/overlap");
                }
                prev = Some(vb);
                tot = tot.add(vb.sub(va));
            }
            let wlen = if ddom { P(dd.0, dd.1) } else { P(dd.2, dd.3) };
            assert_eq!(tot, wlen, "{label}: boundary length mismatch");
        }
    }

    #[test]
    fn boundary_partitions_region() {
        let mut tiles = vec![GrPiece { cls: 12, pos: PT0 }];
        for d in 0..13u32 {
            if d == 4 || d == 12 {
                check_boundary(&tiles, d, &format!("depth {d} boundary"));
            }
            if d < 12 {
                tiles = refine(&tiles);
            }
        }
    }

    // --- (8) class censuses + position checksum ------------------------------

    fn class_counts(tiles: &[GrPiece]) -> Vec<u32> {
        let mut cc = vec![0u32; 18];
        for t in tiles {
            cc[t.cls as usize] += 1;
        }
        cc
    }

    fn pos_checksum(tiles: &[GrPiece]) -> Pt {
        let mut s = PT0;
        for t in tiles {
            s = s.add(t.pos);
        }
        s
    }

    #[test]
    fn d4_class_census() {
        let mut tiles = vec![GrPiece { cls: 12, pos: PT0 }];
        for _ in 0..4 {
            tiles = refine(&tiles);
        }
        // Non-ergodic orbit: only classes {0,1,6,7,12,14,15,16} appear.
        assert_eq!(
            class_counts(&tiles),
            vec![6, 6, 0, 0, 0, 0, 15, 15, 0, 0, 0, 0, 3, 0, 2, 7, 7, 0],
            "depth-4 class census"
        );
        assert_eq!(tiles.len(), 61, "depth-4 total");
    }

    #[test]
    fn d12_census_and_position_checksum() {
        let mut tiles = vec![GrPiece { cls: 12, pos: PT0 }];
        for _ in 0..12 {
            tiles = refine(&tiles);
        }
        assert_eq!(
            class_counts(&tiles),
            vec![
                12816, 12816, 0, 0, 0, 0, 33552, 33552, 0, 0, 0, 0, 5731, 0, 5730, 15006, 15006, 0,
            ],
            "depth-12 class census"
        );
        assert_eq!(tiles.len(), 134209, "depth-12 total");
        assert_eq!(
            pos_checksum(&tiles),
            Pt(21554678, 34898364, 9630594, 15619664),
            "depth-12 position checksum"
        );
    }
}
