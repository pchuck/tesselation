//! Generalized GLB binary tiling, n = 7 (`glb-binary-n7`).
//!
//! The Pautze/Hibma *generalized binary* family: the n = 5 member is the
//! classic GLB binary tiling with two prototiles; for general n the
//! inflation multiplier is √(2+2cos(π/n)) and odd n needs (n−1)/2
//! prototiles. This module lands **n = 7**: three rhombus prototiles
//! (acute angles 2π/14, 4π/14, 6π/14), inflation λ = √(2+2cos(π/7)) =
//! 2cos(π/14) ≈ 1.94986, substitution matrix
//! M = [[2,1,0],[1,2,1],[0,1,3]] with Perron eigenvalue λ² = b²+b+1
//! (b = 2cos(2π/7)) — λ itself has degree 6 with conjugates
//! ±1.9499, ±1.5637, ±0.8678, two outside the unit circle: the inflation
//! is **non-Pisot** (GoLa92), so there is no pure-point spectrum claim.
//!
//! The presentation is **plane-filling**: every query window is covered
//! exactly once by a seed-lattice hierarchy of the certified per-level
//! refinement (see *Presentation* below).
//!
//! # Construction (all exact, in K₇ twist coordinates)
//!
//! A point carries twist coordinates (k₁, k₂) ∈ K₇² — K₇ = Q(b), the
//! real cubic subfield of Q(ζ₁₄) — meaning physical (x, y) =
//! (val(k₁), sin(π/7)·val(k₂)), stored dyadically (value = integer
//! Z[b] coefficients over 2^e, normalized per point). λ is *not* in K₇,
//! but the inflation's action on twist coordinates is: the half-step
//! map φ = λ·rot(π/14) acts by the pinned K₇ matrix
//! [[λ²/2, −(2−b)/4], [1, λ²/2]] — that is why the twist form closes.
//! All rotations used by the construction are even π/14 classes, and
//! each acts K₇-linearly on twist coordinates (table [`ROT_C`]).
//!
//! The rule (fan subdivision): the children of a rule-k tile are the
//! ccw sectors (n−k−1, k, n−k+1, k)·(π/n) around a fan center; sector a
//! is filled by the rhombus of shape min(a, n−a) at angle a·π/n;
//! a ∈ {0, n} sectors are empty. Sectors pair into *canonical fan
//! patches* ([`PCAN`]). A unit parent's children tile exactly the
//! λ-inflated parent φ(parent); the placement is anchored at an acute
//! corner j (turn test cls[j+1]−cls[j] ≡ 2s mod 28) as
//! `rot(cls[j]+θ)·P_can + φ(v_j) + rot(cls[j])·t`, with (θ, t) the
//! certified survivor frames ([`FRAMES`]).
//!
//! **Two-choice pools** (a derivation result beyond the published odd-k
//! scheme, forced by meshing at depth ≥ 2): parent shape 1 accepts only
//! rule-1 fans; shapes 2 and 3 each accept two rules ({5, 2} and
//! {3, 4}). Two mirror frames fi ∈ {0, 1} per rule give each parent 2
//! (shape 1) or 4 (shapes 2, 3) deduplicated candidate placements. The
//! boundary of every placement is a 16-walk staircase of the inflated
//! parent, with an eps bit per side recording the bend; meshing across
//! a shared parent edge requires complementary eps bits.
//!
//! # Certified generator
//!
//! Per level: deterministic DFS with backtracking over parents in
//! canonical order and candidates in fixed (k, anchor, fi) order,
//! pruning on (a) all-pairs interior disjointness of the candidate tile
//! sets and (b) eps-complementarity on shared parent edges. The σ²/σ³
//! batteries (scratch `suff_test.py`/`d_pin3.py`) certify that these
//! two local constraints are *sufficient*: an assembly passing them at
//! depths 2 and 3 passes the full global validator (coverage, area law,
//! vertex census) with zero failures, and the greedy-first variant
//! (no backtracking) is refuted — it sticks at parent 5. Depth-4 DFS
//! (`d_pin3b`) extends the certificate; the K₇-twist census over 114
//! certified patches shows every vertex has dyadic denominator ≤ 2
//! (D = 2, stable through depth 4) and every edge on the even π/7 grid.
//!
//! # Presentation
//!
//! The assembly is built twist-exactly in *assembly coordinates* from
//! seed parents on a rhombus lattice: cell (m, n) is the seed parent V
//! (the unit shape-1 rhombus with edge classes (0, 2, 14, 16), whose
//! φ-image is the seed big tile B₀ = big_tile(0,1)) translated by the
//! exact `Tw` offsets m·(v₁−v₀) + n·(v₂−v₁) — the V-lattice tiles the
//! plane, and neighboring cells share full unit edges. Depth d inflates
//! d times (tiles stay unit-edge; the covered region grows by λ per
//! level); the cell presentation scales the depth-d assembly by
//! λ^−(d−1) and un-rotates by (d−1)·π/14, so the covered cell domain is
//! *depth-invariant*: φ(seed-lattice region) − C₀. Every query window
//! W is covered exactly once by selecting the lattice cells whose
//! closed rhombus intersects φ⁻¹(W + C₀) (cells intersecting a target
//! cover it — the lattice tiles the plane), making the presentation
//! **plane-filling**: no empty background, no finite domain, at any
//! scale, rotation or pan. Depth follows a per-view tile budget:
//! d = max{ d' ∈ [3, 5] : area·λ^{2(d'−1)}/0.75 ≤ 2500 }, floored at
//! D_MIN = 3 (the ±20-unit box at unit scale runs the floor at ≈ 35 k
//! tiles, far under MAX_TILES); the cap D_MAX = 5 is the deepest level
//! the refinement constructs *and* certifies (at depth 6 the candidate
//! search exhausts: a legal patch need not admit a legal refinement —
//! see [`D_MAX`]). An area pre-guard fails extreme
//! zoom-outs before any work. Every level is refined in full: the
//! eps-complementarity that meshes adjacent parents is a global
//! constraint cascade (a parent's valid candidate set depends on its
//! edge-neighbors' choices through arbitrarily long adjacency chains),
//! so spatial culling of the refinement frontier is unsound — the
//! unculled parity battery pins this. The view window is applied only
//! at emission. (Geometrically, each big-tile side still
//! bends off its straight λ-edge by a 2-step U(h−1)+U(h+1) walk of the
//! exact λ-chord identity λU(h) = U(h−1)+U(h+1), deviating by at most
//! sin(π/14), and adjacent parents' bends interlock — complementary
//! eps bits — so the union of staircase regions equals the union of
//! λ-rhombi exactly — across lattice-cell seams by the same predicate
//! that meshes parents inside one lineage.)
//!
//! # Classification scope note
//!
//! Discovered by Stefan Pautze and Tjipke Hibma. Bielefeld tags:
//! finite local complexity, finite rotations, polytopal tiles, rhomb
//! tiles. The inflation is non-PV (two conjugates of λ outside the unit
//! circle), the alternation condition fails, and no patch of the family
//! carries Dₙ symmetry (Pautze's observation) — mirroring the
//! golden-triangle scope convention, no pure-point/diffraction claim is
//! made here. Pautze publishes the odd-k scheme ([Pau2017]); the
//! two-choice pool extension required for compatible meshing at depth
//! ≥ 2 is this repo's derivation, certified by the batteries above.
//!
//! References: [Pau2017] S. Pautze, *Cyclotomic aperiodic substitution
//! tilings*, Symmetry 9(2), 2017, doi.org/10.3390/sym9020019;
//! [Hib2015] T. Hibma, *Generalization of Non-periodic Rhomb Substitution
//! Tilings*, arXiv:1509.02053, 2015; [GoLa92] E. O. Gahler,
//! J. Liu, in *Quasicrystals and Incommensurate Structures*, 1992.

use std::cmp::Ordering;

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamId, ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;

use crate::fields::K7;
use crate::lattice::{similarity_map, MAX_TILES, ROTATION_ID, SCALE_ID};

// ---------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------

/// n = 7: the family parameter (classes live mod 2n = 28 on the π/14
/// grid; the construction only ever uses even classes).
const TWO_N: i32 = 28;

/// λ = 2cos(π/14) = √(2+2cos(π/7)) — the inflation multiplier.
const LAM: f64 = 1.949_855_824_363_619_4;
/// sin(π/7) = √((2−b)/4) — the twist y-axis scale.
const S1: f64 = 0.433_883_739_818_503_85;
/// π/14 — one orientation class.
const ALPHA: f64 = std::f64::consts::PI / 14.0;

/// Presentation depth floor: views whose emitted estimate busts
/// [`VIEW_BUDGET`] at every certified depth (large zoom-outs) run here —
/// the full depth-3 assembly per seed (35 tiles per lattice cell).
const D_MIN: usize = 3;
/// Presentation depth cap: the deepest level the refinement constructs
/// *and* certifies. Depth 5 (462 tiles) passes the full staircase
/// boundary certificate; at depth 6 the per-parent candidate search
/// exhausts — a legal depth-5 patch need not admit any legal depth-6
/// refinement, because eps-complementarity cascades through arbitrarily
/// long adjacency chains (the unculled-parity-battery lesson) and no
/// cheap local rule selects an extendable branch. Tiles are unit-edge
/// at every level, so depth only sets cell-frame fineness: depth 5
/// resolves ≈ 13 tiles per cell unit.
const D_MAX: usize = 5;

// ---------------------------------------------------------------------
// Plane-filling presentation (seed-lattice hierarchy)
// ---------------------------------------------------------------------

/// Mean raw unit-tile area over the depth-3..5 substitution mix,
/// measured on lattice patches (0.681 / 0.719 / 0.746 — pinning the
/// mid value 0.75 over-estimates emitted counts by ≤ 4% at the deep
/// levels, ≤ 10% at depth 3). Used only for the depth-band estimate.
const TILE_AVG: f64 = 0.75;

/// Lower bound on the substitution-mix mean raw tile area (measured
/// 0.681 at depth 3, rounding margin) — the denominator that makes the
/// area pre-guard's count estimate conservative: guard-pass ⇒ the true
/// emitted count stays under [`MAX_TILES`], so the post-emission guard
/// is a backstop that cannot fire.
const TILE_AVG_MIN: f64 = 0.68;

/// Per-view emitted-tile budget: the depth rule picks the largest
/// d ∈ [D_MIN, D_MAX] whose emitted estimate stays under this bound
/// (measured picks: covers 2 071, det 1 104, default CLI pane ≈ 800;
/// rotated/±20-unit boxes fall back to D_MIN at 11.9 k / 34.5 k — legal
/// under [`MAX_TILES`], culling still at emission only). Set to 2 500 —
/// under the default pane's depth-5 estimate (2 597) — because the
/// depth-5 refinement's backtracking cost is arrangement-dependent
/// (measured 0.7–14 ms/parent across patches): the 2 500 cap keeps the
/// default pane at depth 4 while small zoom-ins (≤ 2.5 k estimate, few
/// lattice seeds) still reach the certified depth 5.
const VIEW_BUDGET: f64 = 2_500.0;

/// Float safety margin of the seed-cell selection SAT test (target =
/// φ⁻¹ of the query window): cells whose closed rhombus intersects the
/// target already cover it (the seed lattice tiles the plane), so the
/// margin only absorbs float rounding in the corner transforms.
const SEED_MARGIN: f64 = 0.05;

// ---------------------------------------------------------------------
// Exact twist arithmetic
// ---------------------------------------------------------------------

/// An exact twist point/vector: physical (x, y) = (val(k₁), S1·val(k₂))
/// divided by 2^e, with (k₁, k₂) ∈ K₇². Normalized: e is maximal, i.e.
/// the coefficients are not all even (zero is kept at e = 0) — so two
/// `Tw` are equal (derived `PartialEq`) exactly when their physical
/// points are equal, and `key()` is a canonical hash/dedup key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Tw {
    k1: K7,
    k2: K7,
    e: i32,
}

impl Tw {
    /// Normalizing constructor.
    fn new(k1: K7, k2: K7, e: i32) -> Tw {
        let mut t = Tw { k1, k2, e };
        if t.k1.is_zero() && t.k2.is_zero() {
            return Tw {
                k1: K7::ZERO,
                k2: K7::ZERO,
                e: 0,
            };
        }
        while t.e > 0
            && t.k1.0.iter().chain(t.k2.0.iter()).all(|c| c % 2 == 0)
        {
            for c in t.k1.0.iter_mut() {
                *c /= 2;
            }
            for c in t.k2.0.iter_mut() {
                *c /= 2;
            }
            t.e -= 1;
        }
        debug_assert!(
            t.e <= SCALE_E_MAX
                && t.k1.0.iter().chain(t.k2.0.iter()).all(|c| c.abs() < COEF_MAX),
            "twist coefficients out of bound: {t:?}"
        );
        t
    }

    /// Scale the coefficients up to exponent `e` (≥ self.e) — exact.
    fn align(self, e: i32) -> Tw {
        debug_assert!(e >= self.e, "twist align must go up");
        let sh = (e - self.e) as u32;
        if sh == 0 {
            return self;
        }
        Tw {
            k1: K7([
                self.k1.0[0] << sh,
                self.k1.0[1] << sh,
                self.k1.0[2] << sh,
            ]),
            k2: K7([
                self.k2.0[0] << sh,
                self.k2.0[1] << sh,
                self.k2.0[2] << sh,
            ]),
            e,
        }
    }

    fn add(self, o: Tw) -> Tw {
        let e = self.e.max(o.e);
        let a = self.align(e);
        let b = o.align(e);
        Tw::new(a.k1.add(b.k1), a.k2.add(b.k2), e)
    }

    fn sub(self, o: Tw) -> Tw {
        let e = self.e.max(o.e);
        let a = self.align(e);
        let b = o.align(e);
        Tw::new(a.k1.sub(b.k1), a.k2.sub(b.k2), e)
    }

    /// Canonical key for dedup/hash use (normalized form).
    fn key(self) -> ([i64; 3], [i64; 3], i32) {
        (self.k1.0, self.k2.0, self.e)
    }

    /// Physical coordinates (emission/geometry use only).
    fn vec2(self) -> Vec2 {
        let f = (2.0_f64).powi(self.e);
        Vec2::new(self.k1.float() / f, self.k2.float() * S1 / f)
    }
}

/// Bound on normalized dyadic exponents (measured D = 2; slack for the
/// φ/rot rounding steps).
const SCALE_E_MAX: i32 = 6;
/// Bound on normalized Z[b] coefficients (measured max 112 at D_MAX;
/// ~2³³ headroom — far under the i64 product limits).
const COEF_MAX: i64 = 1 << 40;

/// Z[b] triple at scale 4 (value = coeffs/4) — the table format below.
type S4 = [i64; 3];

/// Canonical undirected parent-edge key: the two endpoint `Tw::key()`
/// tuples in sorted order (eps-complementarity lookup in `refine_level`).
type EK = (([i64; 3], [i64; 3], i32), ([i64; 3], [i64; 3], i32));

/// Apply the even-class rotation `cls` (a multiple of π/7, stored as the
/// raw class 0..27) to a twist point: rot acts K₇-linearly,
/// (k₁, k₂) ↦ (A·k₁ + B·k₂, C·k₁ + A·k₂) with A = cos, B = −sin(π/7)·sin,
/// C = sin/sin(π/7) — table entries at scale 4, so the exponent gains 2.
fn rot_tw(cls: usize, p: Tw) -> Tw {
    let r = &ROT_C[cls / 2];
    let a = K7(r[0]);
    let k1 = p.k1.mul(a).add(p.k2.mul(K7(r[1])));
    let k2 = p.k1.mul(K7(r[2])).add(p.k2.mul(a));
    Tw::new(k1, k2, p.e + 2)
}

/// The half-step inflation φ = λ·rot(π/14) on twist coordinates:
/// (k₁, k₂) ↦ (λ²/2·k₁ − (2−b)/4·k₂, k₁ + λ²/2·k₂) — entries in K₇ at
/// scale 4 ([`PHI_TW`]).
fn phi_tw(p: Tw) -> Tw {
    let a = K7(PHI_TW.0);
    let k1 = p.k1.mul(a).add(p.k2.mul(K7(PHI_TW.1)));
    let k2 = p.k1.mul(K7(PHI_TW.2)).add(p.k2.mul(a));
    Tw::new(k1, k2, p.e + 2)
}

/// Exact cross sign of two twist *vectors* (∝ the physical cross by the
/// positive factor sin(π/7)·2^(−2e)) — the sine-free orientation test.
fn cross_sign(d: Tw, v: Tw) -> Ordering {
    let e = d.e.max(v.e);
    let d = d.align(e);
    let v = v.align(e);
    d.k1.mul(v.k2).sub(d.k2.mul(v.k1)).sign()
}

/// Convex ccw polygons with interior-disjoint interiors (boundary
/// contact allowed): some directed edge of one has the whole other
/// polygon on it or strictly to its right (certified scratch
/// `interior_disjoint` convention).
fn interior_disjoint(a: &[Tw; 4], b: &[Tw; 4]) -> bool {
    fn sep(p: &[Tw; 4], q: &[Tw; 4]) -> bool {
        (0..4).any(|i| {
            let d = p[(i + 1) % 4].sub(p[i]);
            q.iter()
                .all(|w| cross_sign(d, w.sub(p[i])) != Ordering::Greater)
        })
    }
    sep(a, b) || sep(b, a)
}

// ---------------------------------------------------------------------
// Certified tables (scale-4 Z[b]; Python field-exact dump, re-verified
// against hand trigonometry — see the brief)
// ---------------------------------------------------------------------

/// φ on twist coordinates: [[λ²/2, −(2−b)/4], [1, λ²/2]] at scale 4
/// (entries ([2,2,2], [−2,1,0], [4,0,0])/4).
const PHI_TW: (S4, S4, S4) = ([2, 2, 2], [-2, 1, 0], [4, 0, 0]);

/// Even-class rotations, indexed by class/2 (class = 2·index):
/// entries (A, B, C) at scale 4 with
/// A = cos(class·π/14), B = −sin(π/7)·sin(class·π/14), C = sin/sin(π/7).
const ROT_C: [[S4; 3]; 14] = [
    [[4, 0, 0], [0, 0, 0], [0, 0, 0]],      // class 0
    [[-2, 2, 2], [-2, 1, 0], [4, 0, 0]],    // class 2
    [[0, 2, 0], [3, -1, -2], [-4, 4, 4]],   // class 4
    [[4, 0, -2], [-2, -1, 1], [4, 4, 0]],   // class 6
    [[-4, 0, 2], [-2, -1, 1], [4, 4, 0]],   // class 8
    [[0, -2, 0], [3, -1, -2], [-4, 4, 4]],  // class 10
    [[2, -2, -2], [-2, 1, 0], [4, 0, 0]],   // class 12
    [[-4, 0, 0], [0, 0, 0], [0, 0, 0]],     // class 14
    [[2, -2, -2], [2, -1, 0], [-4, 0, 0]],  // class 16
    [[0, -2, 0], [-3, 1, 2], [4, -4, -4]],  // class 18
    [[-4, 0, 2], [2, 1, -1], [-4, -4, 0]],  // class 20
    [[4, 0, -2], [2, 1, -1], [-4, -4, 0]],  // class 22
    [[0, 2, 0], [-3, 1, 2], [4, -4, -4]],   // class 24
    [[-2, 2, 2], [2, -1, 0], [-4, 0, 0]],   // class 26
];

/// One certified survivor frame of rule k: rotation θ (even class) and
/// translation t (twist at scale 4) such that the canonical fan patch
/// `rot(θ)·P_can(k) + t` tiles the seed big tile big_tile(0, s_k); eps
/// is the level-1 boundary staircase's bend bits in big-tile side order.
struct Frame {
    theta: i32,
    t: (S4, S4),
    eps: [u8; 4],
}

/// The survivor frames per rule k ∈ 1..=5 (index k−1), two per rule;
/// rule 6 has none (it never enters a pool).
const FRAMES: [[Frame; 2]; 5] = [
    // k = 1
    [
        Frame { theta: 0, t: ([2, 2, 2], [4, 0, 0]), eps: [0, 0, 1, 1] },
        Frame { theta: 14, t: ([-2, 4, 2], [0, 4, 4]), eps: [1, 1, 0, 0] },
    ],
    // k = 2
    [
        Frame { theta: 26, t: ([4, 2, 0], [-4, 4, 4]), eps: [0, 0, 1, 1] },
        Frame { theta: 12, t: ([2, 2, 0], [8, 4, 0]), eps: [1, 1, 0, 0] },
    ],
    // k = 3
    [
        Frame { theta: 24, t: ([8, 0, -2], [4, 4, 0]), eps: [0, 0, 1, 1] },
        Frame { theta: 10, t: ([-6, 2, 4], [8, 4, 0]), eps: [1, 1, 0, 0] },
    ],
    // k = 4
    [
        Frame { theta: 14, t: ([0, 0, 2], [4, 4, 0]), eps: [0, 1, 1, 0] },
        Frame { theta: 0, t: ([2, 2, 0], [8, 4, 0]), eps: [1, 0, 0, 1] },
    ],
    // k = 5
    [
        Frame { theta: 10, t: ([8, 0, -2], [4, 4, 0]), eps: [0, 1, 1, 0] },
        Frame { theta: 24, t: ([-2, 4, 2], [0, 4, 4]), eps: [1, 0, 0, 1] },
    ],
];

/// One canonical fan tile of rule k: shape 1..3, edge classes (corner
/// order, mod 28) and the four twist vertices at scale 4, rooted at O.
struct PTile {
    shape: u8,
    cls: [i32; 4],
    v: [(S4, S4); 4],
}

/// Canonical fan patches per rule k ∈ 1..=5 (index k−1). Rule 1 has
/// three tiles (its 4th sector is the empty a = n sector); the unused
/// slot carries shape 0.
const PCAN: [[PTile; 4]; 5] = [
    // k = 1: sectors a = 5, 1, (empty), 1
    [
        PTile { shape: 2, cls: [4, 14, 18, 0], v: [
            ([0, 0, 0], [0, 0, 0]), ([0, 2, 0], [-4, 4, 4]),
            ([-4, 2, 0], [-4, 4, 4]), ([-4, 0, 0], [0, 0, 0])] },
        PTile { shape: 1, cls: [14, 16, 0, 2], v: [
            ([0, 0, 0], [0, 0, 0]), ([-4, 0, 0], [0, 0, 0]),
            ([-2, -2, -2], [-4, 0, 0]), ([2, -2, -2], [-4, 0, 0])] },
        PTile { shape: 1, cls: [2, 4, 16, 18], v: [
            ([0, 0, 0], [0, 0, 0]), ([-2, 2, 2], [4, 0, 0]),
            ([-2, 4, 2], [0, 4, 4]), ([0, 2, 0], [-4, 4, 4])] },
        PTile { shape: 0, cls: [0; 4], v: [
            ([0, 0, 0], [0, 0, 0]); 4] },
    ],
    // k = 2: sectors a = 4, 2, 4, 2
    [
        PTile { shape: 3, cls: [8, 16, 22, 2], v: [
            ([0, 0, 0], [0, 0, 0]), ([-4, 0, 2], [4, 4, 0]),
            ([-2, -2, 0], [0, 4, 0]), ([2, -2, -2], [-4, 0, 0])] },
        PTile { shape: 2, cls: [16, 20, 2, 6], v: [
            ([0, 0, 0], [0, 0, 0]), ([2, -2, -2], [-4, 0, 0]),
            ([-2, -2, 0], [-8, -4, 0]), ([-4, 0, 2], [-4, -4, 0])] },
        PTile { shape: 1, cls: [20, 4, 6, 18], v: [
            ([0, 0, 0], [0, 0, 0]), ([-4, 0, 2], [-4, -4, 0]),
            ([-4, 2, 2], [-8, 0, 4]), ([0, 2, 0], [-4, 4, 4])] },
        PTile { shape: 2, cls: [4, 8, 18, 22], v: [
            ([0, 0, 0], [0, 0, 0]), ([0, 2, 0], [-4, 4, 4]),
            ([-4, 2, 2], [0, 8, 4]), ([-4, 0, 2], [4, 4, 0])] },
    ],
    // k = 3: sectors a = 3, 3, 2, 3
    [
        PTile { shape: 3, cls: [12, 18, 26, 4], v: [
            ([0, 0, 0], [0, 0, 0]), ([2, -2, -2], [4, 0, 0]),
            ([2, -4, -2], [8, -4, -4]), ([0, -2, 0], [4, -4, -4])] },
        PTile { shape: 3, cls: [18, 24, 4, 10], v: [
            ([0, 0, 0], [0, 0, 0]), ([0, -2, 0], [4, -4, -4]),
            ([0, 0, 0], [8, -8, -8]), ([0, 2, 0], [4, -4, -4])] },
        PTile { shape: 2, cls: [24, 6, 10, 20], v: [
            ([0, 0, 0], [0, 0, 0]), ([0, 2, 0], [4, -4, -4]),
            ([4, 2, -2], [8, 0, -4]), ([4, 0, -2], [4, 4, 0])] },
        PTile { shape: 3, cls: [6, 12, 20, 26], v: [
            ([0, 0, 0], [0, 0, 0]), ([4, 0, -2], [4, 4, 0]),
            ([6, -2, -4], [8, 4, 0]), ([2, -2, -2], [4, 0, 0])] },
    ],
    // k = 4: sectors a = 2, 4, 4, 4
    [
        PTile { shape: 2, cls: [16, 20, 2, 6], v: [
            ([0, 0, 0], [0, 0, 0]), ([2, -2, -2], [-4, 0, 0]),
            ([-2, -2, 0], [-8, -4, 0]), ([-4, 0, 2], [-4, -4, 0])] },
        PTile { shape: 3, cls: [20, 0, 6, 14], v: [
            ([0, 0, 0], [0, 0, 0]), ([-4, 0, 2], [-4, -4, 0]),
            ([0, 0, 2], [-4, -4, 0]), ([4, 0, 0], [0, 0, 0])] },
        PTile { shape: 3, cls: [0, 8, 14, 22], v: [
            ([0, 0, 0], [0, 0, 0]), ([4, 0, 0], [0, 0, 0]),
            ([0, 0, 2], [4, 4, 0]), ([-4, 0, 2], [4, 4, 0])] },
        PTile { shape: 3, cls: [8, 16, 22, 2], v: [
            ([0, 0, 0], [0, 0, 0]), ([-4, 0, 2], [4, 4, 0]),
            ([-2, -2, 0], [0, 4, 0]), ([2, -2, -2], [-4, 0, 0])] },
    ],
    // k = 5: sectors a = 1, 2, 3, 2
    [
        PTile { shape: 1, cls: [20, 22, 6, 8], v: [
            ([0, 0, 0], [0, 0, 0]), ([-4, 0, 2], [-4, -4, 0]),
            ([0, 0, 0], [-8, -8, 0]), ([4, 0, -2], [-4, -4, 0])] },
        PTile { shape: 2, cls: [22, 4, 8, 18], v: [
            ([0, 0, 0], [0, 0, 0]), ([4, 0, -2], [-4, -4, 0]),
            ([4, 2, -2], [-8, 0, 4]), ([0, 2, 0], [-4, 4, 4])] },
        PTile { shape: 3, cls: [4, 10, 18, 24], v: [
            ([0, 0, 0], [0, 0, 0]), ([0, 2, 0], [-4, 4, 4]),
            ([0, 0, 0], [-8, 8, 8]), ([0, -2, 0], [-4, 4, 4])] },
        PTile { shape: 2, cls: [10, 20, 24, 6], v: [
            ([0, 0, 0], [0, 0, 0]), ([0, -2, 0], [-4, 4, 4]),
            ([-4, -2, 2], [-8, 0, 4]), ([-4, 0, 2], [-4, -4, 0])] },
    ],
];

/// The two-choice pools: parent shape s (1-based) → fan rules. The
/// scratch model's ROS is {1: [1, 6], 2: [5, 2], 3: [3, 4]}; rule 6 has
/// no certified survivor frame (the level-1 enumeration finds none), so
/// shape 1 effectively admits only rule 1 — the 0 entry skips it.
const POOL: [[usize; 2]; 4] = [
    [0, 0],   // unused index
    [1, 0],   // shape 1 → {rule 1} (rule 6: no survivor frames)
    [5, 2],   // shape 2 → {rule 5, rule 2}
    [3, 4],   // shape 3 → {rule 3, rule 4}
];

/// The canonical seed parent V: the unit shape-1 rhombus with edge
/// classes (0, 2, 14, 16) — its φ-image is the seed big tile
/// big_tile(0, 1), and φ(V)'s twist cross equals λ² = (1,1,1)ᵦ exactly.
const SEED_V: [(S4, S4); 4] = [
    ([0, 0, 0], [0, 0, 0]),
    ([4, 0, 0], [0, 0, 0]),
    ([2, 2, 2], [4, 0, 0]),
    ([-2, 2, 2], [4, 0, 0]),
];

// ---------------------------------------------------------------------
// Placed tiles and candidate generation
// ---------------------------------------------------------------------

/// A placed unit tile: shape 1..3, edge classes (corner order, mod 28),
/// exact twist vertices, and a float bbox with slack for cheap rejects.
#[derive(Clone, Debug)]
struct TileR {
    shape: u8,
    cls: [i32; 4],
    v: [Tw; 4],
    bbox: (Vec2, Vec2),
}

impl TileR {
    fn new(shape: u8, cls: [i32; 4], v: [Tw; 4]) -> TileR {
        let mut min = v[0].vec2();
        let mut max = min;
        for w in &v[1..] {
            let p = w.vec2();
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
        }
        let slack = 1e-9;
        TileR {
            shape,
            cls,
            v,
            bbox: (
                Vec2::new(min.x - slack, min.y - slack),
                Vec2::new(max.x + slack, max.y + slack),
            ),
        }
    }

    /// The corner-order side index of the undirected edge a-b, if the
    /// tile has it (either direction). Test-side (the production
    /// conflict test reads precomputed undirected edge keys).
    #[cfg(test)]
    fn side_of(&self, a: Tw, b: Tw) -> Option<usize> {
        let (ka, kb) = (a.key(), b.key());
        for i in 0..4 {
            let p = self.v[i].key();
            let q = self.v[(i + 1) % 4].key();
            if (p == ka && q == kb) || (p == kb && q == ka) {
                return Some(i);
            }
        }
        None
    }
}

/// One candidate placement of a parent's children.
struct Cand {
    tiles: Vec<TileR>,
    /// Bend bits of the placement's boundary staircase, in the parent's
    /// corner-order side indexing (eps_corner[side]).
    eps: [u8; 4],
}

fn pcan_tiles(k: usize) -> impl Iterator<Item = &'static PTile> {
    PCAN[k - 1].iter().filter(|t| t.shape != 0)
}

/// All deduplicated candidate placements of `parent` — the certified
/// port of the scratch `place_child_gen` candidate space: rules from
/// the shape pool × acute-corner anchors (turn test) × the two mirror
/// frames, deduplicated by exact tile-set key. The certified counts are
/// 2 (shape 1) and 4 (shapes 2, 3).
fn candidates(parent: &TileR) -> Vec<Cand> {
    let delta = (parent.cls[1] - parent.cls[0]).rem_euclid(TWO_N);
    debug_assert!(
        delta % 2 == 0 && delta > 0 && delta < TWO_N,
        "parent not a ccw unit rhombus: delta {delta}"
    );
    // Corner angle at corner 0 in classes: 2s (acute) or 2(7−s) obtuse;
    // reduce against π = 14 classes, not the full turn.
    let s = (delta.min(TWO_N / 2 - delta) / 2) as usize;
    debug_assert!((1..=3).contains(&s), "impossible rhombus shape s = {s}");
    let mut out: Vec<Cand> = Vec::new();
    let mut keys: Vec<Vec<i64>> = Vec::new();
    for pool_i in 0..2 {
        let k = POOL[s][pool_i];
        if k == 0 {
            continue; // single-rule pool (shape 1)
        }
        for j in 0..4 {
            // Anchor turn test: the acute corner.
            if (parent.cls[(j + 1) % 4] - parent.cls[j]).rem_euclid(TWO_N) != 2 * s as i32 {
                continue;
            }
            let p = parent.cls[j];
            for fi in 0..2 {
                let fr = &FRAMES[k - 1][fi];
                let rot_c = ((p + fr.theta).rem_euclid(TWO_N)) as usize;
                debug_assert!(rot_c % 2 == 0, "odd placement rotation");
                // T = φ(v_j) + rot(p)·t_k
                let phi_vj = phi_tw(parent.v[j]);
                let t_k = Tw::new(K7(fr.t.0), K7(fr.t.1), 2);
                let rp_t = rot_tw(p as usize, t_k);
                let tt = phi_vj.add(rp_t);
                let tiles: Vec<TileR> = pcan_tiles(k)
                    .map(|pt| {
                        let cls = [
                            (pt.cls[0] + rot_c as i32) % TWO_N,
                            (pt.cls[1] + rot_c as i32) % TWO_N,
                            (pt.cls[2] + rot_c as i32) % TWO_N,
                            (pt.cls[3] + rot_c as i32) % TWO_N,
                        ];
                        let v = pt.v.map(|(a, b)| {
                            let q = rot_tw(rot_c, Tw::new(K7(a), K7(b), 2));
                            q.add(tt)
                        });
                        TileR::new(pt.shape, cls, v)
                    })
                    .collect();
                // eps in the parent's corner-order side indexing
                // (scratch compat: parent side ia reads big-tile-side
                // order bit eps0[(ia - j) % 4]).
                let eps = [
                    fr.eps[(4 + 0 - j) % 4],
                    fr.eps[(4 + 1 - j) % 4],
                    fr.eps[(4 + 2 - j) % 4],
                    fr.eps[(4 + 3 - j) % 4],
                ];
                // Dedup by exact tile-set key (certified equivalences
                // (fi0, j+2) ≡ (fi1, j) collapse here).
                let mut key: Vec<i64> = Vec::with_capacity(tiles.len() * 29);
                let mut tkeys: Vec<Vec<i64>> = tiles
                    .iter()
                    .map(|t| {
                        let mut tk = vec![t.shape as i64];
                        for w in &t.v {
                            let (a, b, e) = w.key();
                            tk.extend_from_slice(&a);
                            tk.extend_from_slice(&b);
                            tk.push(e as i64);
                        }
                        tk
                    })
                    .collect();
                tkeys.sort();
                for tk in &tkeys {
                    key.extend_from_slice(tk);
                }
                if !keys.contains(&key) {
                    keys.push(key);
                    out.push(Cand { tiles, eps });
                }
            }
        }
    }
    // The certified candidate space: 2 placements (shape 1, single rule)
    // or 4 (shapes 2, 3 — two rules × two deduplicated frames).
    debug_assert!(
        out.len() == if s == 1 { 2 } else { 4 },
        "certified candidate count for shape {s}: {}",
        out.len()
    );
    out
}

// ---------------------------------------------------------------------
// Per-level refinement (deterministic DFS with backtracking)
// ---------------------------------------------------------------------

/// Bucket size for the parent φ-image spatial hash, scaled per level to
/// the parents themselves: refined kids tile φ(parent) exactly (the
/// fan-area law λ² per level with all-pairs interior disjointness), so
/// candidate tiles stay inside φ(parent)'s bbox and any pair the
/// conflict predicate can reject has overlapping φ-image bboxes —
/// center displacement at most the max φ-image bbox diagonal. Buckets
/// of that size (3×3 scans) therefore see every interacting pair while
/// staying O(mesh neighborhood) at plane-scale patch densities; a fixed
/// bucket size collects Θ(patch) neighbors per scan and turns deep
/// levels quadratic (measured: 2.6 ms/parent at σ⁴ density vs ~7 µs
/// with level-scaled buckets).
const HASH_CELL_MARGIN: f64 = 1.25;

fn bbox_intersect(a: &(Vec2, Vec2), b: &(Vec2, Vec2)) -> bool {
    a.0.x <= b.1.x && b.0.x <= a.1.x && a.0.y <= b.1.y && b.0.y <= a.1.y
}

/// One refinement level: choose a consistent candidate for every parent
/// and return the chosen children (the next level's parents).
/// Deterministic: parents in canonical order, candidates in generation
/// order, backtracking on conflict. Refinement is always total — the
/// eps-complementarity constraints that mesh neighboring parents
/// cascade along the adjacency graph, so refining a spatial subset
/// changes the kept parents' valid refinements (the σ² census and the
/// full-assembly parity battery pin this); the finite domain makes
/// full refinement affordable at every depth.

fn refine_level(parents: &[TileR]) -> Option<Vec<TileR>> {
    let cands: Vec<Vec<Cand>> = parents.iter().map(candidates).collect();
    debug_assert!(cands.iter().all(|c| !c.is_empty()), "parent without candidates");

    // Spatial hash over φ-image bbox centers (3×3 neighborhood scans),
    // bucket size scaled to this level's parents (see HASH_CELL_MARGIN:
    // kids stay inside φ(parent), so max-φ-image-diagonal buckets see
    // every pair the conflict predicate can reject).
    let mut max_diag = 0.0f64;
    let bboxes: Vec<(Vec2, Vec2)> = parents
        .iter()
        .map(|p| {
            let bb = phi_image_bbox(p);
            max_diag = max_diag.max((bb.1.x - bb.0.x).hypot(bb.1.y - bb.0.y));
            bb
        })
        .collect();
    let cell = if max_diag > 0.0 {
        max_diag * HASH_CELL_MARGIN + 1e-6
    } else {
        1.0
    };
    let mut hash: std::collections::HashMap<(i64, i64), Vec<usize>> =
        std::collections::HashMap::new();
    let centers: Vec<(i64, i64)> = bboxes
        .iter()
        .enumerate()
        .map(|(t, bb)| {
            let cx = ((bb.0.x + bb.1.x) / 2.0 / cell).floor() as i64;
            let cy = ((bb.0.y + bb.1.y) / 2.0 / cell).floor() as i64;
            hash.entry((cx, cy)).or_default().push(t);
            (cx, cy)
        })
        .collect();
    let mut assign: Vec<Option<usize>> = vec![None; parents.len()];

    // Per-parent undirected edge keys (sorted endpoint-key pairs): the
    // eps-complementarity lookup without rebuilding Tw keys per pair.
    let ek: Vec<[EK; 4]> = parents
        .iter()
        .map(|p| {
            let mut e = [(p.v[0].key(), p.v[0].key()), (p.v[0].key(), p.v[0].key()), (p.v[0].key(), p.v[0].key()), (p.v[0].key(), p.v[0].key())];
            for i in 0..4 {
                let (ka, kb) = (p.v[i].key(), p.v[(i + 1) % 4].key());
                e[i] = if ka <= kb { (ka, kb) } else { (kb, ka) };
            }
            e
        })
        .collect();

    // Conflict test: candidate tiles vs the assigned neighbors' tiles
    // (interior disjointness) + eps-complementarity on shared parent
    // edges — the certified σ²/σ³-sufficient pair of constraints.
    // Pairs with disjoint parent φ-image bboxes are pre-skipped: kids
    // stay inside φ(parent) (the fan fills it, area law λ² per level),
    // so overlapping kids or a shared parent edge force overlapping
    // φ-image bboxes — far pairs can never conflict.
    fn conflicts(
        cands: &[Vec<Cand>],
        assign: &[Option<usize>],
        pboxes: &[(Vec2, Vec2)],
        ek: &[[EK; 4]],
        t: usize,
        ci: usize,
        nbs: &[usize],
    ) -> bool {
        let cand = &cands[t][ci];
        for &j in nbs {
            if !bbox_intersect(&pboxes[t], &pboxes[j]) {
                continue;
            }
            let cj = &cands[j][assign[j].unwrap()];
            // (a) tile-pair disjointness (bbox reject first)
            for ta in &cand.tiles {
                for tb in &cj.tiles {
                    if bbox_intersect(&ta.bbox, &tb.bbox)
                        && !interior_disjoint(&ta.v, &tb.v)
                    {
                        return true;
                    }
                }
            }
            // (b) eps-complementarity on shared parent edges: the two
            // parents' corner-order side indices of the shared undirected
            // edge, each reading its candidate's eps bits (scratch
            // compat: bit sum must be exactly 1) — matched via the
            // precomputed undirected edge keys.
            for ia in 0..4 {
                if let Some(sj) = ek[j].iter().position(|&e| e == ek[t][ia]) {
                    if cand.eps[ia] as u16 + cj.eps[sj] as u16 != 1 {
                        return true;
                    }
                }
            }
        }
        false
    }

    // DFS over parents in canonical order — iterative (explicit
    // candidate-progress stack): plane-scale patches refine thousands of
    // parents per level, far past safe recursion depth. The search
    // semantics are exactly the certified recursive ones: parents in
    // slice order, candidates in generation order, backtracking on
    // conflict. Neighbor lists are computed once per parent: in
    // slice-order DFS the assigned set at every visit of t is exactly
    // {j < t}, independent of the candidates chosen.
    let mut nbs_of: Vec<Vec<usize>> = Vec::with_capacity(parents.len());
    for (t, &(cx, cy)) in centers.iter().enumerate() {
        let mut nbs = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(list) = hash.get(&(cx + dx, cy + dy)) {
                    for &j in list {
                        if j < t {
                            nbs.push(j);
                        }
                    }
                }
            }
        }
        nbs_of.push(nbs);
    }
    let mut next_ci: Vec<usize> = vec![0; parents.len()];
    let mut t = 0usize;
    loop {
        if t == parents.len() {
            break; // every parent assigned consistently
        }
        let mut placed = false;
        while next_ci[t] < cands[t].len() {
            let ci = next_ci[t];
            next_ci[t] += 1;
            if conflicts(
                &cands,
                &assign,
                &bboxes,
                &ek,
                t,
                ci,
                &nbs_of[t],
            ) {
                continue;
            }
            assign[t] = Some(ci);
            if t + 1 < parents.len() {
                next_ci[t + 1] = 0;
            }
            t += 1;
            placed = true;
            break;
        }
        if !placed {
            if t == 0 {
                return None; // no consistent refinement — must never happen
            }
            t -= 1;
            assign[t] = None;
        }
    }
    let mut out = Vec::new();
    for (t, ci) in assign.iter().enumerate() {
        out.extend(cands[t][ci.unwrap()].tiles.iter().cloned());
    }
    Some(out)
}

/// One float φ step λ·rot(π/14) — hash-bucket use only.
fn phi_float(p: Vec2) -> Vec2 {
    let (s, c) = ALPHA.sin_cos();
    Vec2::new(c * p.x - s * p.y, s * p.x + c * p.y) * LAM
}

/// The λ-rhombus φ(parent) bbox (float; hash-bucket use only).
fn phi_image_bbox(t: &TileR) -> (Vec2, Vec2) {
    let pts: Vec<Vec2> = t.v.iter().map(|w| phi_float(w.vec2())).collect();
    let mut min = pts[0];
    let mut max = min;
    for p in &pts[1..] {
        min.x = min.x.min(p.x);
        min.y = min.y.min(p.y);
        max.x = max.x.max(p.x);
        max.y = max.y.max(p.y);
    }
    (min, max)
}

/// The full depth-d assembly: level 1..d refines the seed parent V,
/// refining every parent at every level (always total — see
/// [`refine_level`]; the eps-cascade makes spatial culling unsound, and
/// the finite domain keeps the full assembly within the tile budget).
/// Test-side since the plane-filling upgrade: production builds via
/// [`assemble_lattice`] over the seed lattice (identical cascade; with
/// the single seed parent V this function is exactly that case).
#[cfg(test)]
fn assemble(depth: usize) -> Option<Vec<TileR>> {
    let seed = TileR::new(
        1,
        [0, 2, TWO_N / 2, TWO_N / 2 + 2],
        SEED_V.map(|(a, b)| Tw::new(K7(a), K7(b), 2)),
    );
    let mut parents = vec![seed];
    for _ in 0..depth {
        parents = refine_level(&parents)?;
    }
    Some(parents)
}

// ---------------------------------------------------------------------
// Seed-lattice hierarchy (plane-filling parent patch)
// ---------------------------------------------------------------------

/// Separation gap between two convex CCW polygons over their edge
/// normals: ≤ 0 ⇒ the polygons intersect, > 0 ⇒ they are separated by
/// at least that distance (float geometry — seed selection only).
fn sat_gap(poly_p: &[Vec2; 4], poly_q: &[Vec2; 4]) -> f64 {
    let mut best = f64::NEG_INFINITY;
    for poly in [poly_p, poly_q] {
        for i in 0..4 {
            let a = poly[i];
            let b = poly[(i + 1) % 4];
            let (nx, ny) = (b.y - a.y, -(b.x - a.x));
            let len = nx.hypot(ny);
            if len < 1e-12 {
                continue;
            }
            let (ux, uy) = (nx / len, ny / len);
            let (mut pmin, mut pmax) = (f64::MAX, f64::MIN);
            for v in poly_p {
                let d = v.x * ux + v.y * uy;
                pmin = pmin.min(d);
                pmax = pmax.max(d);
            }
            let (mut qmin, mut qmax) = (f64::MAX, f64::MIN);
            for v in poly_q {
                let d = v.x * ux + v.y * uy;
                qmin = qmin.min(d);
                qmax = qmax.max(d);
            }
            best = best.max((qmin - pmax).max(pmin - qmax));
        }
    }
    best
}

/// The plane parent patch: every lattice cell of the seed-rhombus
/// tiling (cell (m, n) = V + m·(v₁−v₀) + n·(v₂−v₁), offsets exact in
/// `Tw`) whose closed rhombus lies within [`SEED_MARGIN`] of `target`
/// (the φ⁻¹ image of the query window — float polygon). Cells
/// intersecting the target already cover it (the lattice tiles the
/// plane), so the margin only absorbs rounding; the superset (m, n)
/// range comes from the target corners' (s, t) coordinates ± 2.
///
/// The offsets are exact `Tw::add` chains (no float touches the
/// vertices); the per-cell basis offsets are accumulated once per row
/// to keep the construction linear.
fn seed_lattice(target: &[Vec2; 4]) -> Vec<TileR> {
    let v: Vec<Tw> = SEED_V.iter().map(|&(a, b)| Tw::new(K7(a), K7(b), 2)).collect();
    let vf: Vec<Vec2> = v.iter().map(|w| w.vec2()).collect();
    let a = vf[1] - vf[0];
    let b = vf[2] - vf[1];
    // (s, t) coordinates of the target corners in the lattice basis.
    let det = a.x * b.y - a.y * b.x;
    let (mut smn, mut smx, mut tmn, mut tmx) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    for p in target {
        let s = (p.x * b.y - p.y * b.x) / det;
        let t = (a.x * p.y - a.y * p.x) / det;
        smn = smn.min(s);
        smx = smx.max(s);
        tmn = tmn.min(t);
        tmx = tmx.max(t);
    }
    let (m0, m1) = (smn.floor() as i64 - 2, smx.ceil() as i64 + 2);
    let (n0, n1) = (tmn.floor() as i64 - 2, tmx.ceil() as i64 + 2);
    let cls = [0, 2, TWO_N / 2, TWO_N / 2 + 2];
    let mut out = Vec::new();
    // Exact basis offsets: oa(m) = m·(v₁−v₀), ob(n) = n·(v₂−v₁),
    // accumulated incrementally with exact Tw arithmetic.
    let mut oa = Tw::new(K7::ZERO, K7::ZERO, 0);
    for _ in m0..0 {
        oa = oa.sub(v[1].sub(v[0]));
    }
    for _ in 0..m0 {
        oa = oa.add(v[1].sub(v[0]));
    }
    for m in m0..m1 {
        let mut ob = oa;
        for _ in n0..0 {
            ob = ob.sub(v[2].sub(v[1]));
        }
        for _ in 0..n0 {
            ob = ob.add(v[2].sub(v[1]));
        }
        for n in n0..n1 {
            let cell = [
                vf[0] + a * (m as f64) + b * (n as f64),
                vf[1] + a * (m as f64) + b * (n as f64),
                vf[2] + a * (m as f64) + b * (n as f64),
                vf[3] + a * (m as f64) + b * (n as f64),
            ];
            if sat_gap(&cell, target) <= SEED_MARGIN {
                out.push(TileR::new(
                    1,
                    cls,
                    [
                        v[0].add(ob),
                        v[1].add(ob),
                        v[2].add(ob),
                        v[3].add(ob),
                    ],
                ));
            }
            ob = ob.add(v[2].sub(v[1]));
        }
        oa = oa.add(v[1].sub(v[0]));
    }
    out
}

/// The plane patch at depth d: refine the seed-lattice hierarchy d
/// times (the same certified per-level DFS as [`assemble`], over the
/// whole parent set — seams between lattice cells mesh by the same
/// eps-complementarity predicate used inside every lineage).
fn assemble_lattice(seeds: &[TileR], depth: usize) -> Option<Vec<TileR>> {
    let mut parents = seeds.to_vec();
    for _ in 0..depth {
        parents = refine_level(&parents)?;
    }
    Some(parents)
}

// ---------------------------------------------------------------------
// The tiling proper
// ---------------------------------------------------------------------

/// Stable id for the substitution-depth param (the only `Int` param
/// on this tiling): 0 = the view-budget rule, 1..=5 = pinned depth.
pub const DEPTH_ID: ParamId = ParamId::new("depth");

const PARAMS: [ParamSpec; 3] = [
    // Plane-filling presentation: every query window is covered, so the
    // scale only sets tile fineness. At the default 2.75 the standard
    // 20/scale-tall CLI pane (7.27 world units) renders the depth-4
    // hierarchy at ≈ 0.8 k tiles (budget band O(10³–10⁴)).
    ParamSpec::float(SCALE_ID, "scale", 0.1, 50.0, 0.05, 2.75),
    ParamSpec::angle(ROTATION_ID, "rotation (deg)", 0.0, 360.0, 5.0, 0.0),
    // Depth selection: 0 (default) = the view-budget rule below;
    // 1..=5 pins the substitution level outright. Depth is a pure
    // generation-budget knob — every geometry constant below stays.
    ParamSpec::int(DEPTH_ID, "depth", 0, D_MAX as i64, 1, 0),
];

pub struct GlbBinaryN7Tiling;

impl Tiling for GlbBinaryN7Tiling {
    fn id(&self) -> &'static str {
        "glb-binary-n7"
    }

    fn name(&self) -> &'static str {
        "GLB binary, generalized (n = 7)"
    }

    fn description(&self) -> &'static str {
        "Generalized GLB binary substitution (Pautze/Hibma cyclotomic \
         family) at n = 7: three rhombi (acute angles π/7, 2π/7, 3π/7), \
         inflation λ = √(2+2cos(π/7)) = 2cos(π/14) ≈ 1.9499 — non-Pisot \
         (conjugates ±1.5637 outside the unit circle, no pure-point \
         claim); substitution M = [[2,1,0],[1,2,1],[0,1,3]], PF \
         eigenvalue λ² = b²+b+1 ∈ K₇ = Q(2cos(2π/7)); two-choice rule \
         pools (shape 2 → {5,2}, shape 3 → {3,4}) forced by compatible \
         meshing at depth ≥ 2; certified generator = deterministic DFS \
         (all-pairs interior disjointness + eps-complementary staircase \
         meshing on shared parent edges), K₇-twist exact arithmetic \
         (every vertex dyadic denominator ≤ 2), even π/7 edge grid \
         throughout; plane-filling presentation = seed-rhombus lattice \
         of the certified seed parent refined d ∈ [3,5] levels (the \
         same DFS over the whole parent set meshes cell seams), seeds \
         chosen so φ⁻¹ of any query window is covered; depth = largest \
         d with emitted estimate ≤ 2500 (floored at 3) — or pinned by the `depth` \
         param to 1..=5 (0 = the budget rule), culling at emission only"
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
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None)
            .with_notes("aperiodic; local 7-fold symmetry; non-Pisot inflation");
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };
        let empty = |message: Option<&'static str>| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message,
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Query box in the recentered cell frame.
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
        if (region.max.x - region.min.x) * (region.max.y - region.min.y) <= 1e-12 {
            return empty(Some("empty view"));
        }

        // Presentation recentering C₀: the bbox center of the seed big
        // tile φ(V) — the probe/preview convention, kept from the
        // certified island presentation.
        let cverts: Vec<Vec2> = SEED_V
            .iter()
            .map(|&(a, b)| phi_tw(Tw::new(K7(a), K7(b), 2)).vec2())
            .collect();
        let dmin = cverts
            .iter()
            .fold(Vec2::new(f64::MAX, f64::MAX), |m, p| {
                Vec2::new(m.x.min(p.x), m.y.min(p.y))
            });
        let dmax = cverts
            .iter()
            .fold(Vec2::new(f64::MIN, f64::MIN), |m, p| {
                Vec2::new(m.x.max(p.x), m.y.max(p.y))
            });
        let center = Vec2::new((dmin.x + dmax.x) / 2.0, (dmin.y + dmax.y) / 2.0);

        // Depth selection. The `depth` param pins the substitution level
        // outright (1..=5); 0 (the default) keeps the view-budget rule,
        // byte-identical to the unparameterized path. Anything else is a
        // named refusal, never a silent snap. The cell-frame emitted
        // count is ≈ region area × λ^{2(d−1)} / TILE_AVG (level-d tiles
        // are unit-edge rhombi scaled by λ^−(d−1); TILE_AVG is the
        // measured substitution-mix mean).
        let depth_req = params.int(DEPTH_ID, 0);
        let d = if depth_req != 0 {
            if depth_req < 1 || depth_req > D_MAX as i64 {
                return fail("unsupported depth — must be 0 (auto) or 1..=5");
            }
            depth_req as usize
        } else {
            let area = (region.max.x - region.min.x) * (region.max.y - region.min.y);
            let mut d = D_MIN;
            for cand_d in (D_MIN..=D_MAX).rev() {
                let est = area * LAM.powi((cand_d as i32 - 1) * 2) / TILE_AVG;
                if est <= VIEW_BUDGET {
                    d = cand_d;
                    break;
                }
            }
            d
        };

        // Area pre-guard (mirrors lattice.rs): the emitted count is
        // ≈ region_area·λ^{2(d−1)}/(mix-mean raw tile area) with the
        // measured mean ≥ TILE_AVG_MIN at every certified depth, so
        // dividing by s²·λ^(−2(d−1))·TILE_AVG_MIN lower-bounds it —
        // views whose lower bound busts MAX_TILES refuse before any
        // refinement work (threshold at default scale, depth floored
        // at D_MIN = 3: MAX_TILES·s²·λ⁻⁴·0.68 ≈ 89 000 world units²).
        let scl = params.float(SCALE_ID, 1.0);
        let lamd = LAM.powi(-((d - 1) as i32));
        if query.region.area() / (scl * scl * lamd * lamd * TILE_AVG_MIN)
            > MAX_TILES as f64
        {
            return fail("too many tiles for this view — zoom in");
        }

        // Assembly-coordinate transform: cell = rot(−(d−1)α)·(asm·
        // λ^−(d−1)) − C₀.
        let rot_fwd = (d - 1) as f64 * ALPHA;
        let lam_d = LAM.powi((d - 1) as i32);

        // Plane parent patch: the covered cell domain at any depth is
        // φ(seed-lattice region) − C₀ (the level-d patch spans
        // λ^d·rot(dα)·seeds, and the presentation transform divides by
        // λ^{d−1}·rot((d−1)α)), so the seeds must cover φ⁻¹ of the
        // query window — a depth-invariant rotated box. Build the seed
        // lattice over it, then run the certified per-level cascade.
        let mut target = [Vec2::ZERO; 4];
        for (i, corner) in region.corners().iter().enumerate() {
            let (s, c) = ALPHA.sin_cos();
            let q = *corner + center;
            target[i] = Vec2::new(c * q.x + s * q.y, -s * q.x + c * q.y) / LAM;
        }
        let seeds = seed_lattice(&target);
        let tiles = match assemble_lattice(&seeds, d) {
            None => return fail("internal error: refinement failed"),
            Some(t) if t.is_empty() => return empty(None),
            Some(t) => t,
        };

        // Emission: exact twist → float, the presentation transform, the
        // world map, cull to the query region; post-guard on the count.
        let to_cell = |p: Vec2| -> Vec2 {
            let (s, c) = (-rot_fwd).sin_cos();
            Vec2::new(c * p.x - s * p.y, s * p.x + c * p.y) / lam_d - center
        };
        let mut out = Vec::new();
        for t in &tiles {
            let pts: Vec<Vec2> = t.v.iter().map(|&w| to_cell(w.vec2())).collect();
            let poly = Polygon::from_slice(&pts).transform(&map);
            if poly.aabb().intersects(query.region) {
                out.push(Tile {
                    proto: t.shape - 1,
                    seed: (t.shape - 1) as u32,
                    poly,
                });
            }
        }
        if out.len() > MAX_TILES {
            return fail("too many tiles for this view — zoom in");
        }

        TilingResult {
            tiles: out,
            symmetry: meta,
            periodic: None,
            message: None,
        }
    }
}

// ---------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{check_covers, check_det, check_rotated};
    use tess_core::params::{ParamKind, ParamValue};
    use std::sync::{Mutex, OnceLock};

    /// Shared lazy assembly cache: `depth_counts_area_law` (all depths)
    /// and `k7_twist_certificate_port` (depth 4) both refine the same
    /// seed cascade; memoize per depth so each refinement runs once per
    /// test binary. Returns a clone — callers own their copy.
    static ASSEMBLE_CACHE: OnceLock<Mutex<std::collections::HashMap<usize, Vec<TileR>>>> =
        OnceLock::new();

    fn assemble_shared(depth: usize) -> Vec<TileR> {
        let cache = ASSEMBLE_CACHE
            .get_or_init(|| Mutex::new(std::collections::HashMap::new()));
        {
            // Fast path: no lock held during generation below.
            if let Some(tiles) = cache.lock().unwrap().get(&depth) {
                return tiles.clone();
            }
        }
        // Double-checked memo: compute outside the lock so parallel
        // tests (libtest worker pool) never serialize on the cache.
        let tiles = assemble(depth)
            .unwrap_or_else(|| panic!("depth {depth} refinement failed"));
        let mut guard = cache.lock().unwrap();
        guard.entry(depth).or_insert_with(|| tiles.clone());
        tiles
    }

    /// Shared lazy generate() cache for the plane-battery windows shared
    /// across `plane_coverage_census`, `view_budget_pins` and
    /// `far_view_covered` (covers / rotated / det / box20@1 / clipane /
    /// far): each test re-ran the same parameterized generation. Keyed on
    /// the exact scale, rotation and window — the same inputs the tests
    /// passed. Returns a clone — callers own their copy.
    static GENERATE_CACHE: OnceLock<
        Mutex<std::collections::HashMap<(u64, u64, u64, u64, u64, u64), TilingResult>>,
    > = OnceLock::new();

    fn generate_cached(sc: f64, rot: f64, world: BBox) -> TilingResult {
        let key = (
            sc.to_bits(),
            rot.to_bits(),
            world.min.x.to_bits(),
            world.min.y.to_bits(),
            world.max.x.to_bits(),
            world.max.y.to_bits(),
        );
        let cache = GENERATE_CACHE
            .get_or_init(|| Mutex::new(std::collections::HashMap::new()));
        {
            if let Some(r) = cache.lock().unwrap().get(&key) {
                return r.clone();
            }
        }
        // Double-checked memo: compute outside the lock (see
        // `assemble_shared`) so parallel tests never serialize on the cache.
        let t = GlbBinaryN7Tiling;
        let mut pp = ParamSet::default_for(t.params());
        pp.set(SCALE_ID, ParamValue::Float(sc));
        pp.set(ROTATION_ID, ParamValue::Angle(rot));
        let r = t.generate(&pp, &Query { region: world });
        let mut guard = cache.lock().unwrap();
        guard.entry(key).or_insert_with(|| r.clone());
        r
    }

    /// Total twist shoelace of a tile set, every vertex aligned to the
    /// set's maximal exponent `e` (true value = result / 2^(2e)) — the
    /// area-law probe, exact in K₇.
    fn twist_shoelace(tiles: &[TileR]) -> (K7, i32) {
        let e = tiles
            .iter()
            .flat_map(|t| t.v.iter().map(|w| w.e))
            .max()
            .unwrap();
        let mut s = K7::ZERO;
        for t in tiles {
            let v: Vec<Tw> = t.v.iter().map(|w| w.align(e)).collect();
            for i in 0..4 {
                let p = v[i];
                let q = v[(i + 1) % 4];
                s = s.add(p.k1.mul(q.k2)).sub(p.k2.mul(q.k1));
            }
        }
        (s, e)
    }

    /// The area-law expectation 2·λ^(2d) as an exact K₇ element
    /// (λ² = b²+b+1 = (1,1,1), raised by repeated field multiplication).
    fn area_law_expected(d: usize, e: i32) -> K7 {
        let lam2 = K7([1, 1, 1]);
        let mut lam2d = K7::ONE;
        for _ in 0..d {
            lam2d = lam2d.mul(lam2);
        }
        lam2d.scale(2).scale(1i64 << (2 * e))
    }

    /// Every edge direction of every tile sits on the even π/14 class
    /// grid (the angles are exact class multiples; float rounding is
    /// ~1e-15 against a class spacing of π/14 ≈ 0.22).
    fn assert_even_edge_grid(tiles: &[TileR]) {
        for t in tiles {
            for i in 0..4 {
                let a = t.v[i].vec2();
                let b = t.v[(i + 1) % 4].vec2();
                let cls = ((b.y - a.y).atan2(b.x - a.x) / ALPHA).round() as i32;
                assert_eq!(
                    cls.rem_euclid(TWO_N) % 2,
                    0,
                    "odd edge class {cls} on tile {t:?}"
                );
            }
        }
    }

    /// eps-complementarity over every shared undirected parent edge
    /// (the [`refine_level`] meshing predicate, standalone).
    fn eps_compat(parents: &[TileR], pick: &[usize], cands: &[Vec<Cand>]) -> bool {
        for x in 0..parents.len() {
            for y in (x + 1)..parents.len() {
                let px = &parents[x];
                let py = &parents[y];
                for ia in 0..4 {
                    if let Some(sj) = py.side_of(px.v[ia], px.v[(ia + 1) % 4]) {
                        let a = cands[x][pick[x]].eps[ia];
                        let b = cands[y][pick[y]].eps[sj];
                        if a as u16 + b as u16 != 1 {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn seed_parent() -> TileR {
        TileR::new(
            1,
            [0, 2, TWO_N / 2, TWO_N / 2 + 2],
            SEED_V.map(|(a, b)| Tw::new(K7(a), K7(b), 2)),
        )
    }

    /// φ(V) is the pinned seed big tile B₀: exact twist vertices, the
    /// λ² edge cross, and the 3.425 × 1.650 float bbox.
    #[test]
    fn phi_seed_is_big_tile() {
        let b0: Vec<Tw> = SEED_V
            .iter()
            .map(|&(a, b)| phi_tw(Tw::new(K7(a), K7(b), 2)))
            .collect();
        // Pinned normalized twist vertices of φ(V). φ maps e = 2 → e = 4;
// Tw::new normalizes the (k₁, k₂) pair jointly (divide by 2 only when
// BOTH are all-even), so odd k₁ coefficients pin k₂ at e = 1 with even
// coefficients — e.g. v₁ k₂ = (2,0,0): physical y = S1·2/2 = sin(π/7).
        let pin = [
            Tw { k1: K7([0, 0, 0]), k2: K7([0, 0, 0]), e: 0 },
            Tw { k1: K7([1, 1, 1]), k2: K7([2, 0, 0]), e: 1 },
            Tw { k1: K7([0, 3, 2]), k2: K7([2, 2, 2]), e: 1 },
            Tw { k1: K7([-1, 2, 1]), k2: K7([0, 2, 2]), e: 1 },
        ];
        assert_eq!(b0, pin, "φ(V) twist vertices");

        // The O-rooted side vectors cross to λ² = (1,1,1) in value. The side
        // vectors sit at e = 1 (each coefficient carries 2^e), so the
        // coefficient-space cross is λ²·2^{2e} = 4·(1,1,1).
        let o = b0[0].align(1);
        let d1 = b0[1].align(1).sub(o);
        let d2 = b0[3].align(1).sub(o);
        let cross = d1.k1.mul(d2.k2).sub(d1.k2.mul(d2.k1));
        assert_eq!(cross, K7([4, 4, 4]), "B₀ edge cross must equal λ²·2^(2e)");

        // Float bbox pin (informational presentation numbers).
        let f: Vec<Vec2> = b0.iter().map(|w| w.vec2()).collect();
        let w = f.iter().map(|p| p.x).fold(f64::MIN, f64::max)
            - f.iter().map(|p| p.x).fold(f64::MAX, f64::min);
        let h = f.iter().map(|p| p.y).fold(f64::MIN, f64::max)
            - f.iter().map(|p| p.y).fold(f64::MAX, f64::min);
        assert!((w - 3.425_427_5).abs() < 1e-6, "B₀ bbox width {w}");
        assert!((h - 1.649_599_0).abs() < 1e-6, "B₀ bbox height {h}");
    }

    /// Level-1 structure of the seed: two deduplicated candidate
    /// placements, three unit tiles each (shapes {1, 1, 2}), interior
    /// disjoint, and the exact area law 2λ² in twist units.
    #[test]
    fn level1_two_choice() {
        let seed = seed_parent();
        let cands = candidates(&seed);
        assert_eq!(cands.len(), 2, "shape-1 parent must have 2 candidates");
        let lam2 = K7([1, 1, 1]); // λ² = b²+b+1
        for c in &cands {
            assert_eq!(c.tiles.len(), 3, "rule-1 fan has 3 tiles");
            let shapes: Vec<u8> = c.tiles.iter().map(|t| t.shape).collect();
            let mut sorted = shapes.clone();
            sorted.sort_unstable();
            assert_eq!(sorted, vec![1, 1, 2], "substitution row M·P₁");
            for i in 0..3 {
                for j in (i + 1)..3 {
                    assert!(
                        interior_disjoint(&c.tiles[i].v, &c.tiles[j].v),
                        "level-1 tiles overlap"
                    );
                }
            }
            // Σ twist shoelace == 2λ² exact.
            let (s, e) = twist_shoelace(&c.tiles);
            let expected = lam2.scale(2).scale(1i64 << (2 * e));
            assert_eq!(s, expected, "level-1 area law 2λ²");
            assert_even_edge_grid(&c.tiles);
            assert!(c.tiles.iter().all(|t| t.v.iter().all(|w| w.e <= 1)));
        }
    }

    /// The σ² census over the fixed canonical frame: the seed's first
    /// candidate's three children offer 4/2/2 candidates (16 combos);
    /// all-pairs interior disjointness alone keeps 9, adding
    /// eps-complementarity keeps exactly 4 — and the scratch battery
    /// (census_s1_both.log, `validate_level`) certifies those same 4 as
    /// the globally valid partitions (the two-constraint DFS predicate
    /// is sufficient; disjointness alone is not). The mirror candidate
    /// gives identical counts by the rule's mirror symmetry.
    #[test]
    fn sigma2_census() {
        let seed = seed_parent();
        let cands = candidates(&seed);
        for pick in 0..2 {
            let parents: Vec<TileR> = cands[pick].tiles.clone();
            let child_cands: Vec<Vec<Cand>> =
                parents.iter().map(candidates).collect();
            let lens: Vec<usize> = child_cands.iter().map(Vec::len).collect();
            assert_eq!(lens, vec![4, 2, 2], "fan order is (shape 2, 1, 1)");
            let mut disjoint = 0;
            let mut meshed = 0;
            for ci in 0..4 {
                for cj in 0..2 {
                    for ck in 0..2 {
                        let pick3 = [ci, cj, ck];
                        let tiles: Vec<&TileR> = (0..3)
                            .flat_map(|t| child_cands[t][pick3[t]].tiles.iter())
                            .collect();
                        let ok = (0..tiles.len()).all(|i| {
                            (i + 1..tiles.len()).all(|j| {
                                interior_disjoint(&tiles[i].v, &tiles[j].v)
                            })
                        });
                        if ok {
                            disjoint += 1;
                            if eps_compat(&parents, &pick3, &child_cands) {
                                meshed += 1;
                            }
                        }
                    }
                }
            }
            assert_eq!(disjoint, 9, "disjoint-only sigma2 census (pick {pick})");
            assert_eq!(meshed, 4, "disjoint+compat sigma2 census (pick {pick})");
        }
    }

    /// Unculled depth growth 3/10/35/126/462 (the M^d row sums from the
    /// P₁ seed), each assembly interior-disjoint with the exact area law
    /// 2·λ^(2d), unit-exponent vertices (D = 2) and the even edge grid.
    #[test]
    fn depth_counts_area_law() {
        for d in 1..=D_MAX {
            let tiles = assemble_shared(d);
            let expected = [3usize, 10, 35, 126, 462][d - 1];
            assert_eq!(tiles.len(), expected, "depth {d} count");
            // all-pairs interior disjointness (bucketed: every tile
            // registers in every 2-unit cell its bbox spans, so any
            // bbox-overlapping pair shares at least one cell — complete)
            const AREA_CELL: f64 = 2.0;
            let mut buckets: std::collections::HashMap<(i32, i32), Vec<usize>> =
                std::collections::HashMap::new();
            for (i, t) in tiles.iter().enumerate() {
                for gx in (t.bbox.0.x / AREA_CELL).floor() as i32
                    ..=(t.bbox.1.x / AREA_CELL).floor() as i32
                {
                    for gy in (t.bbox.0.y / AREA_CELL).floor() as i32
                        ..=(t.bbox.1.y / AREA_CELL).floor() as i32
                    {
                        buckets.entry((gx, gy)).or_default().push(i);
                    }
                }
            }
            for list in buckets.values() {
                for x in 0..list.len() {
                    for y in (x + 1)..list.len() {
                        let (i, j) = (list[x], list[y]);
                        if bbox_intersect(&tiles[i].bbox, &tiles[j].bbox)
                            && !interior_disjoint(&tiles[i].v, &tiles[j].v)
                        {
                            panic!("overlap at depth {d}, tiles {i}/{j}");
                        }
                    }
                }
            }
            let (s, e) = twist_shoelace(&tiles);
            assert_eq!(s, area_law_expected(d, e), "area law at depth {d}");
            assert_even_edge_grid(&tiles);
            assert!(
                tiles.iter().all(|t| t.v.iter().all(|w| w.e <= 1)),
                "depth {d} vertex denominators exceed 2"
            );
        }
    }

    /// The Perron–Frobenius eigenpair of the substitution matrix, exact
    /// in K₇: M·v = λ²·v for v = (1, b²+b−1, b+1), λ² = b²+b+1.
    #[test]
    fn pf_eigenvalue() {
        let m: [[i64; 3]; 3] = [[2, 1, 0], [1, 2, 1], [0, 1, 3]];
        let v = [K7::ONE, K7([-1, 1, 1]), K7([1, 1, 0])];
        let lam2 = K7([1, 1, 1]); // b²+b+1
        for i in 0..3 {
            let mut row = K7::ZERO;
            for j in 0..3 {
                row = row.add(v[j].scale(m[i][j]));
            }
            assert_eq!(row, v[i].mul(lam2), "PF eigenvector row {i}");
        }
    }

    /// The K₇-twist certificate at depth 4 (scratch `k7_twist.py`
    /// battery, module port): the full assembly's vertices all have
    /// dyadic denominator ≤ 2 (normalized e ≤ 1) and every edge sits on
    /// the even π/14 class grid.
    #[test]
    fn k7_twist_certificate_port() {
        let tiles = assemble_shared(4);
        assert_eq!(tiles.len(), 126);
        // Vertex census: every normalized twist vertex has e ≤ 1.
        let mut verts: Vec<Tw> = tiles.iter().flat_map(|t| t.v).collect();
        verts.sort_by_key(|w| w.key());
        verts.dedup();
        assert!(verts.iter().all(|w| w.e <= 1), "denominator census");
        assert_even_edge_grid(&tiles);
    }

    // --- standard trio ---------------------------------------------------
    // Explicit windows at the default presentation: the plane-filling
    // seed lattice covers every query window by construction (cells
    // intersecting φ⁻¹(W + C₀) cover W), so these small panes refine at
    // the D_MAX = 5 budget pick; the trio helpers pin shape, coverage
    // and determinism on them.

    const WIN: [f64; 4] = [-0.55, -0.41, 0.55, 0.41];
    const WIN_ROT: [f64; 4] = [-0.05, -0.04, 0.05, 0.04];

    #[test]
    fn glb_binary_n7_covers_region() {
        check_covers(
            &GlbBinaryN7Tiling,
            "glb-binary-n7",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }

    #[test]
    fn glb_binary_n7_rotated_scaled() {
        check_rotated(
            &GlbBinaryN7Tiling,
            "glb-binary-n7(rot25,scale0.6)",
            BBox::new(
                Vec2::new(WIN_ROT[0], WIN_ROT[1]),
                Vec2::new(WIN_ROT[2], WIN_ROT[3]),
            ),
            0.6,
            25.0,
        );
    }

    #[test]
    fn glb_binary_n7_deterministic() {
        check_det(
            &GlbBinaryN7Tiling,
            "glb-binary-n7",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }

    /// Plane-filling semantics: a far-off window still returns a covered,
    /// message-free result (the seed lattice covers φ⁻¹ of ANY window —
    /// there is no finite domain to fall outside of); a degenerate
    /// (zero-area) view still refuses with a message.
    #[test]
    fn far_view_covered() {
        let t = GlbBinaryN7Tiling;
        let p = ParamSet::default_for(t.params());
        let region = BBox::new(Vec2::new(100.0, 100.0), Vec2::new(101.0, 101.0));
        // Far view == the shared plane-battery "far" window at the
        // default scale/rotation: served by the memoized cache.
        let r = generate_cached(2.75, 0.0, region);
        assert!(r.message.is_none(), "far view must not fail");
        assert!(
            (40..=90).contains(&r.tiles.len()),
            "far view tiles {}",
            r.tiles.len()
        );
        // Culling correctness: every emitted tile reaches the window.
        for tile in &r.tiles {
            assert!(
                tile.poly.aabb().intersects(region),
                "emitted tile misses the far window"
            );
        }

        let r = t.generate(&p, &Query {
            region: BBox::new(Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0)),
        });
        assert_eq!(r.message, Some("empty view"));
        assert!(r.tiles.is_empty());
    }

    /// The area pre-guard refuses views too large for the tile budget
    /// before any refinement work (default scale, depth floored at
    /// D_MIN = 3: the guard threshold is
    /// MAX_TILES·s²·λ⁻⁴·TILE_AVG_MIN ≈ 88 950 world units² — measured
    /// boundary: the ±148 pane still emits ≈ 247 k tiles while ±150 is
    /// already refused; the ±250 pane is 250 000 units²).
    #[test]
    fn zoom_out_refuses() {
        let t = GlbBinaryN7Tiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query {
            region: BBox::new(Vec2::new(-250.0, -250.0), Vec2::new(250.0, 250.0)),
        });
        assert_eq!(r.message, Some("too many tiles for this view — zoom in"));
        assert!(r.tiles.is_empty());

        // Threshold pin (measured): just inside the pre-guard bound the
        // view passes with the post-guard backstop unhit (emitted well
        // under MAX_TILES).
        let r = t.generate(&p, &Query {
            region: BBox::new(Vec2::new(-148.0, -148.0), Vec2::new(148.0, 148.0)),
        });
        assert!(r.message.is_none(), "±148 pane is inside the pre-guard bound");
        assert!(
            (240_000..250_000).contains(&r.tiles.len()),
            "±148 pane tiles {}",
            r.tiles.len()
        );
        let r = t.generate(&p, &Query {
            region: BBox::new(Vec2::new(-150.0, -150.0), Vec2::new(150.0, 150.0)),
        });
        assert_eq!(r.message, Some("too many tiles for this view — zoom in"));
        assert!(r.tiles.is_empty());
    }

    /// Pins BOTH default presentations for the plane-filling generation:
    ///
    /// (a) The app/preview convention: a bounded ±10 probe, then repeated
    ///     preview_window(9 × mean tile side) regeneration — the depth
    ///     rule walks the budget picks 3 → 4 → 5, every round is
    ///     plane-covered and message-free, the mean tile side contracts
    ///     at each zoom, and the round-4 count equals round 3 (the flow
    ///     is count-stable even where the window is not yet a fixed
    ///     point).
    /// (b) The CLI convention: a 20/scale·(2/3) × 10/scale world-unit
    ///     origin-centered pane (4:3 aspect) — plane-covered at the
    ///     depth-3 budget pick (measured 796 tiles).
    #[test]
    fn default_view_fills_pane() {
        let t = GlbBinaryN7Tiling;
        let p = ParamSet::default_for(t.params());
        assert!(
            (p.float(SCALE_ID, 1.0) - 2.75).abs() < 1e-12,
            "scale default must stay patch-fitted (2.75)"
        );
        let s = 2.75;

        // --- (a) App/preview convention --------------------------------
        let probe = t.generate(&p, &Query {
            region: BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)),
        });
        assert!(probe.message.is_none());
        // The probe estimate busts VIEW_BUDGET at depth 4, so the rule
        // floors at D_MIN = 3 (measured 1 241 tiles).
        assert!(
            (1100..1400).contains(&probe.tiles.len()),
            "±10 probe tiles {}",
            probe.tiles.len()
        );
        let mw = tess_core::app::mean_tile_side(&probe.tiles);
        assert!((0.90..0.92).contains(&mw), "probe mean side {mw}");
        let win = tess_core::app::preview_window(probe.periodic, mw, 4.0 / 3.0);
        let r = t.generate(&p, &Query { region: win });
        assert!(r.message.is_none());
        assert!(
            (850..1100).contains(&r.tiles.len()),
            "preview round 1 tiles {}",
            r.tiles.len()
        );
        let mw2 = tess_core::app::mean_tile_side(&r.tiles);
        assert!(mw2 < mw, "round-1 mean side must contract ({mw2} vs {mw})");
        let win2 = tess_core::app::preview_window(r.periodic, mw2, 4.0 / 3.0);
        let r2 = t.generate(&p, &Query { region: win2 });
        assert!(r2.message.is_none());
        assert!(
            (880..1100).contains(&r2.tiles.len()),
            "preview round 2 tiles {}",
            r2.tiles.len()
        );
        let mw3 = tess_core::app::mean_tile_side(&r2.tiles);
        assert!(mw3 < mw2, "round-2 mean side must contract");
        let win3 = tess_core::app::preview_window(r2.periodic, mw3, 4.0 / 3.0);
        let r3 = t.generate(&p, &Query { region: win3 });
        assert!(r3.message.is_none());
        assert!(
            (240..330).contains(&r3.tiles.len()),
            "preview round 3 tiles {}",
            r3.tiles.len()
        );
        // Count-stability: one more round re-derives (near-)the same
        // window and exactly the same tile count.
        let mw4 = tess_core::app::mean_tile_side(&r3.tiles);
        let win4 = tess_core::app::preview_window(r3.periodic, mw4, 4.0 / 3.0);
        let r4 = t.generate(&p, &Query { region: win4 });
        assert!(r4.message.is_none());
        assert_eq!(r4.tiles.len(), r3.tiles.len(), "preview flow count-stable");
        assert!(
            (win4.max.x - win3.max.x).abs() < 1e-2
                && (win4.max.y - win3.max.y).abs() < 1e-2,
            "round-4 window must track round 3"
        );

        // --- (b) CLI convention ----------------------------------------
        let region = BBox::new(
            Vec2::new(-20.0 / s * 2.0 / 3.0, -10.0 / s),
            Vec2::new(20.0 / s * 2.0 / 3.0, 10.0 / s),
        );
        let rc = t.generate(&p, &Query { region });
        assert!(rc.message.is_none());
        assert!(
            (700..900).contains(&rc.tiles.len()),
            "CLI default pane tiles {}",
            rc.tiles.len()
        );
        let mwc = tess_core::app::mean_tile_side(&rc.tiles);
        assert!(
            (0.465..0.482).contains(&mwc),
            "CLI pane depth-4 mean side {mwc}"
        );
    }

    // =================================================================
    // Plane-filling battery (Route-B seed-lattice presentation)
    // =================================================================

    type VK = ([i64; 3], [i64; 3], i32);

    fn ekey(p: Tw, q: Tw) -> (VK, VK) {
        let a = p.key();
        let b = q.key();
        if a <= b {
            (a, b)
        } else {
            (b, a)
        }
    }

    #[derive(Default, Debug)]
    struct MeshAudit {
        n_tiles: usize,
        e_int: usize,
        e_bdy: usize,
        e_multi: usize,
        verts: usize,
        rim_walks: usize,
        rim_simple: bool,
        link_bad: usize,
        cls_bad: usize,
        cross_bad: usize,
        area_got: f64,
        area_exp: f64,
        area_rel: f64,
    }

    /// Structural mesh audit of a unit-tile patch (the cross-boundary
    /// battery core): edge parity (interior edges in exactly 2 tiles,
    /// boundary edges on the rim only), single simple closed rim walk,
    /// connected tile links at every interior vertex, shared-edge class
    /// agreement (mod half-turn), all-pairs interior disjointness, and
    /// the exact fan-area law Σtiles = λ²·Σparents (float shoelace,
    /// 1e-9 relative guard).
    fn audit_mesh(tiles: &[TileR], parents: &[TileR]) -> MeshAudit {
        let mut a = MeshAudit { n_tiles: tiles.len(), ..Default::default() };
        let mut edges: std::collections::HashMap<(VK, VK), Vec<(usize, usize)>> =
            std::collections::HashMap::new();
        for (ti, t) in tiles.iter().enumerate() {
            for s in 0..4 {
                let k = ekey(t.v[s], t.v[(s + 1) % 4]);
                edges.entry(k).or_default().push((ti, s));
            }
        }
        let mut bdy_v: std::collections::HashSet<VK> = std::collections::HashSet::new();
        let mut bdy_edges: Vec<(VK, VK)> = Vec::new();
        for (k, occ) in &edges {
            match occ.len() {
                1 => {
                    a.e_bdy += 1;
                    bdy_v.insert(k.0);
                    bdy_v.insert(k.1);
                    bdy_edges.push(*k);
                }
                2 => a.e_int += 1,
                _ => a.e_multi += 1,
            }
        }
        let mut vset: std::collections::HashSet<VK> = std::collections::HashSet::new();
        for t in tiles {
            for w in &t.v {
                vset.insert(w.key());
            }
        }
        a.verts = vset.len();
        // shared-edge class agreement: same class or ± half-turn
        for (_k, occ) in &edges {
            if occ.len() == 2 {
                let d = (tiles[occ[0].0].cls[occ[0].1] - tiles[occ[1].0].cls[occ[1].1])
                    .rem_euclid(TWO_N);
                if d != 0 && d != TWO_N / 2 {
                    a.cls_bad += 1;
                }
            }
        }
        // all-pairs interior disjointness (bucketed: every tile
        // registers in every 2-unit cell its bbox spans, so any
        // bbox-overlapping pair shares at least one cell — complete)
        const AUDIT_CELL: f64 = 2.0;
        let mut buckets: std::collections::HashMap<(i32, i32), Vec<usize>> =
            std::collections::HashMap::new();
        for (i, t) in tiles.iter().enumerate() {
            for gx in (t.bbox.0.x / AUDIT_CELL).floor() as i32
                ..=(t.bbox.1.x / AUDIT_CELL).floor() as i32
            {
                for gy in (t.bbox.0.y / AUDIT_CELL).floor() as i32
                    ..=(t.bbox.1.y / AUDIT_CELL).floor() as i32
                {
                    buckets.entry((gx, gy)).or_default().push(i);
                }
            }
        }
        for list in buckets.values() {
            for x in 0..list.len() {
                for y in (x + 1)..list.len() {
                    let (i, j) = (list[x], list[y]);
                    if bbox_intersect(&tiles[i].bbox, &tiles[j].bbox)
                        && !interior_disjoint(&tiles[i].v, &tiles[j].v)
                    {
                        a.cross_bad += 1;
                    }
                }
            }
        }
        // rim: every boundary vertex has exactly 2 boundary edges and the
        // boundary-edge graph is one closed simple walk
        let mut vadj: std::collections::HashMap<VK, Vec<usize>> =
            std::collections::HashMap::new();
        for (i, k) in bdy_edges.iter().enumerate() {
            vadj.entry(k.0).or_default().push(i);
            vadj.entry(k.1).or_default().push(i);
        }
        a.rim_simple = a.e_bdy > 0
            && vadj.values().all(|l| l.len() == 2)
            && {
                let mut used = vec![false; bdy_edges.len()];
                let mut walks = 0usize;
                for s0 in 0..bdy_edges.len() {
                    if used[s0] {
                        continue;
                    }
                    walks += 1;
                    used[s0] = true;
                    let start = bdy_edges[s0].0;
                    let mut head = bdy_edges[s0].1;
                    while head != start {
                        let opts = &vadj[&head];
                        match opts.iter().find(|&&i| !used[i]) {
                            Some(&i) => {
                                used[i] = true;
                                let (p, q) = bdy_edges[i];
                                head = if p == head { q } else { p };
                            }
                            None => break,
                        }
                    }
                }
                a.rim_walks = walks;
                walks == 1
            };
        // interior vertex links: tiles around each interior vertex form
        // one cycle (connected through shared count-2 edges at the vertex)
        let mut vtiles: std::collections::HashMap<VK, Vec<usize>> =
            std::collections::HashMap::new();
        for (ti, t) in tiles.iter().enumerate() {
            for w in &t.v {
                vtiles.entry(w.key()).or_default().push(ti);
            }
        }
        for (vk, ts) in &vtiles {
            if bdy_v.contains(vk) || ts.len() < 3 {
                continue;
            }
            let mut uf: Vec<usize> = (0..ts.len()).collect();
            for (x, &ta) in ts.iter().enumerate() {
                for s in 0..4 {
                    let p = tiles[ta].v[s].key();
                    let q = tiles[ta].v[(s + 1) % 4].key();
                    if p != *vk && q != *vk {
                        continue;
                    }
                    let occ = &edges[&ekey(tiles[ta].v[s], tiles[ta].v[(s + 1) % 4])];
                    if occ.len() != 2 {
                        continue;
                    }
                    let other = if occ[0].0 == ta { occ[1].0 } else { occ[0].0 };
                    if let Some(y) = ts.iter().position(|&t2| t2 == other) {
                        let (mut rx, mut ry) = (x, y);
                        while uf[rx] != rx {
                            rx = uf[rx];
                        }
                        while uf[ry] != ry {
                            ry = uf[ry];
                        }
                        uf[rx] = ry;
                    }
                }
            }
            let roots: std::collections::HashSet<usize> =
                (0..ts.len()).map(|mut x| {
                    while uf[x] != x {
                        x = uf[x];
                    }
                    x
                }).collect();
            if roots.len() > 1 {
                a.link_bad += 1;
            }
        }
        // exact fan-area law, float side
        let shoel = |ts: &[TileR]| -> f64 {
            ts.iter()
                .map(|t| {
                    let v: Vec<Vec2> = t.v.iter().map(|w| w.vec2()).collect();
                    let mut s = 0.0;
                    for i in 0..4 {
                        let p = v[i];
                        let q = v[(i + 1) % 4];
                        s += p.x * q.y - p.y * q.x;
                    }
                    s / 2.0
                })
                .sum::<f64>()
        };
        a.area_got = shoel(tiles);
        a.area_exp = LAM * LAM * shoel(parents);
        a.area_rel = (a.area_got - a.area_exp).abs() / a.area_exp.abs().max(1e-12);
        a
    }

/// Build the exact-Tw plane patch for a world window — the same
    /// presentation math as [`GlbBinaryN7Tiling::generate`] (cell-frame
    /// region, C₀ recentering, φ⁻¹ target, seed lattice) without the
    /// emission cull. Returns (level-d tiles, seeds).
    fn plane_patch(sc: f64, rot: f64, world: BBox, d: usize) -> (Vec<TileR>, Vec<TileR>) {
        let t = GlbBinaryN7Tiling;
        let mut pp = ParamSet::default_for(t.params());
        pp.set(SCALE_ID, tess_core::params::ParamValue::Float(sc));
        pp.set(ROTATION_ID, tess_core::params::ParamValue::Angle(rot));
        let map = similarity_map(&pp);
        let inv = map.inverse();
        let mut rmin = Vec2::new(f64::MAX, f64::MAX);
        let mut rmax = Vec2::new(f64::MIN, f64::MIN);
        for &corner in &world.corners() {
            let q = inv.transform(corner);
            rmin.x = rmin.x.min(q.x);
            rmin.y = rmin.y.min(q.y);
            rmax.x = rmax.x.max(q.x);
            rmax.y = rmax.y.max(q.y);
        }
        let region = BBox::new(rmin, rmax);
        // C₀ recentering: the bbox center of the seed big tile φ(V).
        let cverts: Vec<Vec2> = SEED_V
            .iter()
            .map(|&(a, b)| phi_tw(Tw::new(K7(a), K7(b), 2)).vec2())
            .collect();
        let dmin = cverts
            .iter()
            .fold(Vec2::new(f64::MAX, f64::MAX), |m, p| {
                Vec2::new(m.x.min(p.x), m.y.min(p.y))
            });
        let dmax = cverts
            .iter()
            .fold(Vec2::new(f64::MIN, f64::MIN), |m, p| {
                Vec2::new(m.x.max(p.x), m.y.max(p.y))
            });
        let center = Vec2::new((dmin.x + dmax.x) / 2.0, (dmin.y + dmax.y) / 2.0);
        let mut target = [Vec2::ZERO; 4];
        for (i, corner) in region.corners().iter().enumerate() {
            let (s, c) = ALPHA.sin_cos();
            let q = *corner + center;
            target[i] = Vec2::new(c * q.x + s * q.y, -s * q.x + c * q.y) / LAM;
        }
        let seeds = seed_lattice(&target);
        let tiles = assemble_lattice(&seeds, d).expect("plane patch refinement");
        (tiles, seeds)
    }

    /// Cross-boundary structural audit of the plane hierarchy: for every
    /// battery window the whole seed-lattice cascade is refined with the
    /// certified per-level DFS and EVERY level is audited — edge parity
    /// (interior edges in exactly 2 tiles, none above 2), one simple
    /// closed rim walk, interior vertex links, shared-edge class
    /// agreement (0 bend-bit mismatches), bucketed all-pairs interior
    /// disjointness, and the exact fan-area law Σkids = λ²·Σparents.
    /// Lattice-cell seams sit inside every patch, so a clean audit
    /// certifies the cross-boundary meshing (gap- and overlap-freedom
    /// across seeds, not just inside single-cell lineages).
    #[test]
    fn plane_mesh_audit() {
        // (tag, scale, rotation°, window, depth, patch-size pin)
        for (tag, sc, rot, world, d, lo, hi) in [
            ("covers d=4", 2.75f64, 0.0f64,
                BBox::new(Vec2::new(-8.0, -6.0), Vec2::new(8.0, 6.0)), 4usize, 3600, 4300),
            ("covers d=5", 2.75, 0.0,
                BBox::new(Vec2::new(-8.0, -6.0), Vec2::new(8.0, 6.0)), 5, 13500, 15500),
            ("rotated d=3", 0.6, 25.0,
                BBox::new(Vec2::new(-7.0, -7.0), Vec2::new(7.0, 7.0)), 3, 22000, 26000),
            ("det d=4", 2.75, 0.0,
                BBox::new(Vec2::new(-5.0, -5.0), Vec2::new(5.0, 5.0)), 4, 2400, 2900),
            ("box20 d=3", 2.75, 0.0,
                BBox::new(Vec2::new(-20.0, -20.0), Vec2::new(20.0, 20.0)), 3, 5500, 7500),
            ("far d=5", 2.75, 0.0,
                BBox::new(Vec2::new(100.0, 100.0), Vec2::new(101.0, 101.0)), 5, 1200, 1600),
        ] {
            let (tiles, seeds) = plane_patch(sc, rot, world, d);
            let mut cur = seeds.clone();
            let mut last = MeshAudit::default();
            for lvl in 1..=d {
                let kids = refine_level(&cur)
                    .unwrap_or_else(|| panic!("{tag}: level {lvl} refinement failed"));
                let au = audit_mesh(&kids, &cur);
                assert_eq!(au.cross_bad, 0, "{tag} L{lvl} interior overlap");
                assert_eq!(au.e_multi, 0, "{tag} L{lvl} edge >2 occupancy");
                assert_eq!(au.cls_bad, 0, "{tag} L{lvl} shared-edge class");
                assert_eq!(au.link_bad, 0, "{tag} L{lvl} interior vertex link");
                assert!(au.rim_simple, "{tag} L{lvl} rim not one simple cycle");
                assert!(
                    au.area_rel < 1e-9,
                    "{tag} L{lvl} area law rel {}",
                    au.area_rel
                );
                last = au;
                cur = kids;
            }
            assert_eq!(last.n_tiles, tiles.len(), "{tag} audit covers the patch");
            println!(
                "{tag}: seeds={} patch={} audit[cross=0 cls=0 link=0 e_multi=0 \
                 rim_simple={} area_rel={:.1e}]",
                seeds.len(),
                tiles.len(),
                last.rim_simple,
                last.area_rel
            );
            assert!(
                (lo..=hi).contains(&tiles.len()),
                "{tag} patch size {} outside [{lo}, {hi}]",
                tiles.len()
            );
        }
    }

    /// Exactly-once plane-coverage census over real generate() output: a
    /// dense sample grid over each battery window must hit exactly one
    /// tile interior everywhere. Samples landing on shared edges are
    /// resolved by an 8-way ±10⁻⁷ jitter battery (a sample is a hole
    /// only if every resolvable jitter lands strictly outside, an
    /// overlap as soon as one jitter sees two strict containers).
    /// Plus the clipped-area identity Σ area(tile ∩ W) = area(W) to
    /// 1e-6 relative (Sutherland–Hodgman clip, shoelace) — the
    /// continuous, whole-window counterpart of the point census.
    #[test]
    fn plane_coverage_census() {
        // (tag, scale, rotation°, window, samples/axis)
        for (tag, sc, rot, world, ns) in [
            ("covers", 2.75f64, 0.0f64,
                BBox::new(Vec2::new(-8.0, -6.0), Vec2::new(8.0, 6.0)), 101usize),
            ("rotated", 0.6, 25.0,
                BBox::new(Vec2::new(-7.0, -7.0), Vec2::new(7.0, 7.0)), 81),
            ("det", 2.75, 0.0,
                BBox::new(Vec2::new(-5.0, -5.0), Vec2::new(5.0, 5.0)), 101),
            ("box20@1", 1.0, 0.0,
                BBox::new(Vec2::new(-20.0, -20.0), Vec2::new(20.0, 20.0)), 121),
            ("clipane", 2.75, 0.0,
                BBox::new(
                    Vec2::new(-20.0 / 2.75 * 2.0 / 3.0, -10.0 / 2.75),
                    Vec2::new(20.0 / 2.75 * 2.0 / 3.0, 10.0 / 2.75),
                ), 61),
            ("far", 2.75, 0.0,
                BBox::new(Vec2::new(100.0, 100.0), Vec2::new(101.0, 101.0)), 9),
        ] {
            let r = generate_cached(sc, rot, world);
            assert!(r.message.is_none(), "{tag} must be plane-covered");
            assert!(!r.tiles.is_empty(), "{tag} must emit tiles");
            let polys: Vec<Vec<Vec2>> =
                r.tiles.iter().map(|tl| tl.poly.pts.clone()).collect();

            // 2-unit spatial buckets; every tile registers in every cell
            // its bbox spans, so any point's 3×3 neighborhood sees every
            // candidate tile.
            const CELL: f64 = 2.0;
            let mut grid: std::collections::HashMap<(i32, i32), Vec<usize>> =
                std::collections::HashMap::new();
            for (i, p) in polys.iter().enumerate() {
                let mut mn = p[0];
                let mut mx = p[0];
                for q in p.iter().skip(1) {
                    mn = Vec2::new(mn.x.min(q.x), mn.y.min(q.y));
                    mx = Vec2::new(mx.x.max(q.x), mx.y.max(q.y));
                }
                for gx in (mn.x / CELL).floor() as i32..=(mx.x / CELL).floor() as i32 {
                    for gy in (mn.y / CELL).floor() as i32..=(mx.y / CELL).floor() as i32 {
                        grid.entry((gx, gy)).or_default().push(i);
                    }
                }
            }

            // Strict-ccw containment count over the 3×3 bucket scan;
            // generation-stamped dedup (no per-sample allocation).
            fn strict_count(
                grid: &std::collections::HashMap<(i32, i32), Vec<usize>>,
                polys: &[Vec<Vec2>],
                stamp: &mut [u64],
                gen: u64,
                p: Vec2,
            ) -> i32 {
                const EPS: f64 = 1e-12;
                let gx = (p.x / CELL).floor() as i32;
                let gy = (p.y / CELL).floor() as i32;
                let mut c = 0;
                let mut bnd = false;
                for dy in -1..=1i32 {
                    for dx in -1..=1i32 {
                        if let Some(list) = grid.get(&(gx + dx, gy + dy)) {
                            for &ti in list {
                                if stamp[ti] == gen {
                                    continue;
                                }
                                stamp[ti] = gen;
                                let t = &polys[ti];
                                let mut inside = true;
                                let mut on_edge = false;
                                for k in 0..t.len() {
                                    let a = t[k];
                                    let b = t[(k + 1) % t.len()];
                                    let cr = (b.x - a.x) * (p.y - a.y)
                                        - (b.y - a.y) * (p.x - a.x);
                                    if cr <= -EPS {
                                        inside = false;
                                        break;
                                    } else if cr < EPS {
                                        on_edge = true;
                                    }
                                }
                                if inside {
                                    c += 1;
                                } else if on_edge {
                                    bnd = true;
                                }
                            }
                        }
                    }
                }
                if c == 0 && bnd {
                    return -1; // boundary hit: undecided, jitter below
                }
                c
            }

            let (x0, y0) = (world.min.x, world.min.y);
            let (wx, wy) = (world.max.x - world.min.x, world.max.y - world.min.y);
            let mut stamp = vec![0u64; polys.len()];
            let mut gen = 0u64;
            let mut holes = 0i32;
            let mut multis = 0i32;
            let mut bnd_samples = 0i32;
            for iy in 0..ns {
                for ix in 0..ns {
                    let p = Vec2::new(
                        x0 + wx * (ix as f64) / ((ns - 1) as f64),
                        y0 + wy * (iy as f64) / ((ns - 1) as f64),
                    );
                    gen += 1;
                    let c = strict_count(&grid, &polys, &mut stamp, gen, p);
                    if c < 0 {
                        // 8-way ±1e-7 jitter battery.
                        bnd_samples += 1;
                        let mut min_c = i32::MAX;
                        let mut max_c = 0;
                        for d in 0..8u32 {
                            let (jx, jy) = match d {
                                0 => (1.0, 0.0),
                                1 => (-1.0, 0.0),
                                2 => (0.0, 1.0),
                                3 => (0.0, -1.0),
                                4 => (1.0, 1.0),
                                5 => (1.0, -1.0),
                                6 => (-1.0, 1.0),
                                _ => (-1.0, -1.0),
                            };
                            let q = Vec2::new(p.x + jx * 1e-7, p.y + jy * 1e-7);
                            gen += 1;
                            let cj = strict_count(&grid, &polys, &mut stamp, gen, q);
                            if cj < 0 {
                                continue; // still on an edge
                            }
                            min_c = min_c.min(cj);
                            max_c = max_c.max(cj);
                        }
                        if max_c >= 2 {
                            multis += 1;
                        }
                        if min_c == 0 && max_c == 0 {
                            holes += 1; // every resolvable jitter outside
                        }
                    } else {
                        if c == 0 {
                            holes += 1;
                        }
                        if c > 1 {
                            multis += 1;
                        }
                    }
                }
            }
            assert_eq!(
                holes, 0,
                "{tag} coverage holes ({bnd_samples} boundary samples)"
            );
            assert_eq!(multis, 0, "{tag} coverage overlaps");

            // Clipped-area identity: Σ area(tile ∩ W) = area(W).
            let clip_area = |poly: &[Vec2]| -> f64 {
                let mut cur: Vec<Vec2> = poly.to_vec();
                for &(nx, ny, c) in &[
                    (1.0, 0.0, world.max.x),
                    (-1.0, 0.0, -world.min.x),
                    (0.0, 1.0, world.max.y),
                    (0.0, -1.0, -world.min.y),
                ] {
                    if cur.is_empty() {
                        break;
                    }
                    let mut nxt: Vec<Vec2> = Vec::with_capacity(cur.len() + 1);
                    let n = cur.len();
                    for i in 0..n {
                        let a = cur[i];
                        let b = cur[(i + 1) % n];
                        let da = nx * a.x + ny * a.y - c;
                        let db = nx * b.x + ny * b.y - c;
                        if da <= 0.0 {
                            nxt.push(a);
                            if db > 0.0 {
                                let t = da / (da - db);
                                nxt.push(Vec2::new(
                                    a.x + t * (b.x - a.x),
                                    a.y + t * (b.y - a.y),
                                ));
                            }
                        } else if db <= 0.0 {
                            let t = da / (da - db);
                            nxt.push(Vec2::new(
                                a.x + t * (b.x - a.x),
                                a.y + t * (b.y - a.y),
                            ));
                        }
                    }
                    cur = nxt;
                }
                let mut s = 0.0;
                for i in 0..cur.len() {
                    let p = cur[i];
                    let q = cur[(i + 1) % cur.len()];
                    s += p.x * q.y - p.y * q.x;
                }
                (s / 2.0).abs()
            };
            let got: f64 = polys.iter().map(|p| clip_area(p)).sum();
            let rel = (got - wx * wy).abs() / (wx * wy);
            println!(
                "{tag}: tiles={} holes=0 multi=0 bnd={bnd_samples} clip-area rel={rel:.2e}",
                r.tiles.len()
            );
            assert!(rel < 1e-6, "{tag} clipped-area identity rel {rel}");
        }
    }

    /// Measured per-view emitted-count bands (release builds at the
    /// shipped depth rule): every battery window is plane-covered and
    /// message-free, and lands in its pinned band. The ±20-unit boxes
    /// prove the D_MIN floor (their estimates bust VIEW_BUDGET at every
    /// certified depth yet still generate at the depth-3 floor), and
    /// every count stays far under MAX_TILES with culling at emission
    /// only.
    #[test]
    fn view_budget_pins() {
        for (tag, sc, rot, world, lo, hi) in [
            ("covers", 2.75f64, 0.0f64,
                BBox::new(Vec2::new(-8.0, -6.0), Vec2::new(8.0, 6.0)), 1900usize, 2300),
            ("det", 2.75, 0.0,
                BBox::new(Vec2::new(-5.0, -5.0), Vec2::new(5.0, 5.0)), 1000, 1250),
            ("rotated", 0.6, 25.0,
                BBox::new(Vec2::new(-7.0, -7.0), Vec2::new(7.0, 7.0)), 11000, 12800),
            ("box20@1", 1.0, 0.0,
                BBox::new(Vec2::new(-20.0, -20.0), Vec2::new(20.0, 20.0)), 33000, 36500),
            ("box20@2.75", 2.75, 0.0,
                BBox::new(Vec2::new(-20.0, -20.0), Vec2::new(20.0, 20.0)), 4400, 5000),
            ("clipane", 2.75, 0.0,
                BBox::new(
                    Vec2::new(-20.0 / 2.75 * 2.0 / 3.0, -10.0 / 2.75),
                    Vec2::new(20.0 / 2.75 * 2.0 / 3.0, 10.0 / 2.75),
                ), 700, 900),
            ("far", 2.75, 0.0,
                BBox::new(Vec2::new(100.0, 100.0), Vec2::new(101.0, 101.0)), 40, 90),
        ] {
            let r = generate_cached(sc, rot, world);
            assert!(r.message.is_none(), "{tag}: {:?}", r.message);
            assert!(
                (lo..=hi).contains(&r.tiles.len()),
                "{tag}: {} tiles outside [{lo}, {hi}]",
                r.tiles.len()
            );
        }
    }

    // The depth param: appended last, Int, default 0 (auto), range 0..=5.
    #[test]
    fn depth_spec_pins_int_appended_last() {
        let t = GlbBinaryN7Tiling;
        let specs = t.params();
        assert_eq!(specs.len(), 3, "scale, rotation, depth (appended last)");
        let depth = &specs[2];
        assert_eq!(depth.id, DEPTH_ID);
        assert_eq!(depth.label, "depth");
        match depth.kind {
            ParamKind::Int { min, max, step, default } => {
                assert_eq!(default, 0);
                assert_eq!(min, 0);
                assert_eq!(max, D_MAX as i64);
                assert_eq!(step, 1);
            }
            other => panic!("depth spec must be Int, got {other:?}"),
        }
    }

    // Out-of-range depth is a named refusal (never a silent snap).
    #[test]
    fn depth_refusal_names_exact_bounds() {
        let t = GlbBinaryN7Tiling;
        for d in [-1i64, 6i64] {
            let mut pp = ParamSet::default_for(t.params());
            pp.set(DEPTH_ID, ParamValue::Int(d));
            let r = t.generate(&pp, &Query { region: BBox::new(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0)) });
            assert_eq!(r.message, Some("unsupported depth — must be 0 (auto) or 1..=5"));
            assert!(r.tiles.is_empty());
        }
    }

    // Explicit depth 0 is byte-identical to the default budget rule;
    // a pinned depth is a different rule (fewer tiles at pinned depth 1).
    #[test]
    fn depth_zero_is_default_and_pin_changes_output() {
        let t = GlbBinaryN7Tiling;
        let window = BBox::new(Vec2::new(-4.0, -4.0), Vec2::new(4.0, 4.0));
        let mut explicit = ParamSet::default_for(t.params());
        explicit.set(DEPTH_ID, ParamValue::Int(0));
        let r_explicit = t.generate(&explicit, &Query { region: window });
        let r_default = t.generate(&ParamSet::default_for(t.params()), &Query { region: window });
        assert_eq!(r_explicit.message, r_default.message);
        assert_eq!(r_explicit.tiles.len(), r_default.tiles.len(), "explicit depth 0 == default");
        assert!(
            r_explicit.tiles.iter().zip(r_default.tiles.iter()).all(|(a, b)| {
                a.proto == b.proto && a.poly.aabb() == b.poly.aabb()
            }),
            "explicit depth 0 reproduces the default byte-identically"
        );
        let mut pin1 = ParamSet::default_for(t.params());
        pin1.set(DEPTH_ID, ParamValue::Int(1));
        let r_pin1 = t.generate(&pin1, &Query { region: window });
        assert!(r_pin1.message.is_none());
        assert!(r_pin1.tiles.len() < r_explicit.tiles.len(), "pinned depth 1 emits fewer tiles");
    }
}
