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
//! the canonical seed parent V (the unit shape-1 rhombus with edge
//! classes (0, 2, 14, 16), whose φ-image is the seed big tile
//! B₀ = big_tile(0,1) — the λ-rhombus spanned by λU(1), λU(3), rooted
//! at O). Depth d inflates d times (tiles stay unit-edge; the covered
//! region grows by λ per level); the cell presentation scales the
//! depth-d assembly by λ^−(d−1) and un-rotates by (d−1)·π/14, so the
//! covered cell domain is *fixed*: B₀ itself (bbox ≈ 3.425 × 1.650 cell
//! units, presented bbox-centered on the origin — the probe/preview
//! convention), tiled at depth d by λ^−(d−1)-edge tiles. Depth follows
//! the view: d = clamp(ceil(log_λ(9/ext)), 3, 5); the floor shows the
//! whole 35-tile patch, the cap D_MAX = 5 is the deepest level the
//! refinement constructs *and* certifies (at depth 6 the candidate
//! search exhausts: a legal patch need not admit a legal refinement —
//! see [`D_MAX`]). An area pre-guard fails extreme
//! zoom-ins before any work. Every level is refined in full: the
//! eps-complementarity that meshes adjacent parents is a global
//! constraint cascade (a parent's valid candidate set depends on its
//! edge-neighbors' choices through arbitrarily long adjacency chains),
//! so spatial culling of the refinement frontier is unsound — the
//! unculled parity battery pins this. The finite domain makes full
//! refinement affordable at every depth, and the view window is
//! applied only at emission. (Geometrically, each big-tile side still
//! bends off its straight λ-edge by a 2-step U(h−1)+U(h+1) walk of the
//! exact λ-chord identity λU(h) = U(h−1)+U(h+1), deviating by at most
//! sin(π/14), and adjacent parents' bends interlock — complementary
//! eps bits — so the union of staircase regions equals the union of
//! λ-rhombi exactly.)
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
use tess_core::params::{ParamSet, ParamSpec};
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
/// sin(3π/7) — the shape-3 rhombus area factor (max prototile area).
const SIN_3PI_7: f64 = 0.974_927_912_181_823_6;
/// π/14 — one orientation class.
const ALPHA: f64 = std::f64::consts::PI / 14.0;

/// Presentation depth floor: whole-patch views show the full depth-3
/// assembly (35 tiles — the seed's two-level substitution structure).
const D_MIN: usize = 3;
/// Presentation depth cap: the deepest level the refinement constructs
/// *and* certifies. Depth 5 (462 tiles) passes the full staircase
/// boundary certificate; at depth 6 the per-parent candidate search
/// exhausts — a legal depth-5 patch need not admit any legal depth-6
/// refinement, because eps-complementarity cascades through arbitrarily
/// long adjacency chains (the unculled-parity-battery lesson) and no
/// cheap local rule selects an extendable branch. Tiles are unit-edge
/// at every level, so depth only sets cell-frame fineness over the
/// fixed B₀ domain: depth 5 resolves ≈ 13 tiles per cell unit — finer
/// than any view the area pre-guard admits.
const D_MAX: usize = 5;

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
    /// tile has it (either direction).
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

/// Bucket size for the parent φ-image spatial hash (φ-images of unit
/// tiles span ≤ ~4 assembly units; interacting parents have overlapping
/// φ-images, so adjacent buckets amply cover every interacting pair).
const HASH_CELL: f64 = 16.0;

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

    // Spatial hash over φ-image bbox centers (3×3 neighborhood scans).
    let mut hash: std::collections::HashMap<(i64, i64), Vec<usize>> =
        std::collections::HashMap::new();
    let centers: Vec<(i64, i64)> = parents
        .iter()
        .enumerate()
        .map(|(t, p)| {
            let bb = phi_image_bbox(p);
            let cx = ((bb.0.x + bb.1.x) / 2.0 / HASH_CELL).floor() as i64;
            let cy = ((bb.0.y + bb.1.y) / 2.0 / HASH_CELL).floor() as i64;
            hash.entry((cx, cy)).or_default().push(t);
            (cx, cy)
        })
        .collect();
    let mut assign: Vec<Option<usize>> = vec![None; parents.len()];

    // Conflict test: candidate tiles vs the assigned neighbors' tiles
    // (interior disjointness) + eps-complementarity on shared parent
    // edges — the certified σ²/σ³-sufficient pair of constraints.
    fn conflicts(
        parents: &[TileR],
        cands: &[Vec<Cand>],
        assign: &[Option<usize>],
        t: usize,
        ci: usize,
        nbs: &[usize],
    ) -> bool {
        let cand = &cands[t][ci];
        let pt = &parents[t];
        for &j in nbs {
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
            // compat: bit sum must be exactly 1).
            let pj = &parents[j];
            for ia in 0..4 {
                if let Some(sj) = pj.side_of(pt.v[ia], pt.v[(ia + 1) % 4]) {
                    if cand.eps[ia] as u16 + cj.eps[sj] as u16 != 1 {
                        return true;
                    }
                }
            }
        }
        false
    }

    // DFS over parents in canonical order.
    fn dfs(
        parents: &[TileR],
        cands: &[Vec<Cand>],
        hash: &std::collections::HashMap<(i64, i64), Vec<usize>>,
        centers: &[(i64, i64)],
        assign: &mut Vec<Option<usize>>,
        t: usize,
    ) -> bool {
        if t == parents.len() {
            return true;
        }
        // Candidate order = generation order (deterministic).
        for ci in 0..cands[t].len() {
            // neighbors via the spatial hash
            let (cx, cy) = centers[t];
            let mut nbs = Vec::new();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if let Some(list) = hash.get(&(cx + dx, cy + dy)) {
                        for &j in list {
                            if j != t && assign[j].is_some() {
                                nbs.push(j);
                            }
                        }
                    }
                }
            }
            if conflicts(parents, cands, assign, t, ci, &nbs) {
                continue;
            }
            assign[t] = Some(ci);
            if dfs(parents, cands, hash, centers, assign, t + 1) {
                return true;
            }
            assign[t] = None;
        }
        false
    }

    if !dfs(parents, &cands, &hash, &centers, &mut assign, 0) {
        return None; // no consistent refinement — must never happen
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
// The tiling proper
// ---------------------------------------------------------------------

const PARAMS: [ParamSpec; 2] = [
    // Patch-fitted default: the covered cell domain is the seed big tile
    // B₀ = φ(V) (bbox ≈ 3.425 × 1.650 cell units, presented bbox-centered
    // on the origin); at scale 2.75 the standard 20/scale-tall CLI pane
    // (7.27 world units) frames the patch height 1.650·2.75 ≈ 4.54 at
    // ~62% — a comfortable margin under the pane on every aspect.
    ParamSpec::float(SCALE_ID, "scale", 0.1, 50.0, 0.05, 2.75),
    ParamSpec::angle(ROTATION_ID, "rotation (deg)", 0.0, 360.0, 5.0, 0.0),
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
         throughout; fixed cell domain = the seed big tile B₀ = φ(V) \
         (bbox ≈ 3.43 × 1.65, presented bbox-centered), depth = \
         clamp(ceil(log_λ(9/ext)), 3, 5) with certified depth-5 \
         refinement at 462 tiles"
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

        // Domain bbox in the cell frame, fixed at every depth: the seed
        // big tile B₀ = φ(V) (spanned by λU(1), λU(3), rooted at O),
        // presented bbox-centered on the origin — the probe/preview
        // convention. C₀ is its bbox center, computed at runtime.
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

        // Finite domain semantics: outside the covered big tile is
        // background (golden-bee precedent).
        let patch = BBox::new(dmin - center, dmax - center);
        if !patch.intersects(region) {
            return empty(None);
        }

        // Depth from the view: cell-frame tile edges are λ^−(d−1); aim
        // for ≥ 9 tiles across the view diagonal, floored at the
        // full-patch depth and capped at the certified construction
        // depth (D_MAX).
        let ext = (region.max - region.min).length();
        let d = if ext < 1e-9 {
            D_MIN
        } else {
            let want = (9.0 / ext).ln() / LAM.ln();
            (want.ceil() as usize).clamp(D_MIN, D_MAX)
        };

        // Area pre-guard (mirrors lattice.rs): every emitted tile has
        // area ≤ s²·λ^(−2(d−1))·sin(3π/7) (world units) — the level-d
        // tiles are unit-edge rhombi, scaled by λ^−(d−1) into the cell
        // frame — so the region area lower-bounds the emitted count
        // before any refinement work.
        let scl = params.float(SCALE_ID, 1.0);
        let lamd = LAM.powi(-((d - 1) as i32));
        if query.region.area() / (scl * scl * lamd * lamd * SIN_3PI_7)
            > MAX_TILES as f64
        {
            return fail("too many tiles for this view — zoom in");
        }

        // Assembly-coordinate transform: cell = rot(−(d−1)α)·(asm·
        // λ^−(d−1)) − C₀ (the depth-d patch spans φ^d(V) =
        // λ^(d−1)·rot((d−1)α)·B₀, so this is depth-invariant: the
        // covered cell domain is always B₀, bbox-centered).
        let rot_fwd = (d - 1) as f64 * ALPHA;
        let lam_d = LAM.powi((d - 1) as i32);

        // Build the full assembly (always total — see refine_level).
        let tiles = match assemble(d) {
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
            let tiles = assemble(d)
                .unwrap_or_else(|| panic!("depth {d} refinement failed"));
            let expected = [3usize, 10, 35, 126, 462][d - 1];
            assert_eq!(tiles.len(), expected, "depth {d} count");
            for i in 0..tiles.len() {
                for j in (i + 1)..tiles.len() {
                    assert!(
                        interior_disjoint(&tiles[i].v, &tiles[j].v),
                        "overlap at depth {d}, tiles {i}/{j}"
                    );
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
        let tiles = assemble(4).expect("depth-4 refinement");
        assert_eq!(tiles.len(), 126);
        // Vertex census: every normalized twist vertex has e ≤ 1.
        let mut verts: Vec<Tw> = tiles.iter().flat_map(|t| t.v).collect();
        verts.sort_by_key(|w| w.key());
        verts.dedup();
        assert!(verts.iter().all(|w| w.e <= 1), "denominator census");
        assert_even_edge_grid(&tiles);
    }

    // --- standard trio ---------------------------------------------------
    // Explicit windows well inside the presented patch (the cell-domain
    // rhombus B₀ bbox-centered at the default scale 2.75 spans ±(1.71,
    // 0.82) cell units, with the covered boundary's staircase notch
    // reaching ~0.22 cell units inside the flat edges near the short-edge
    // middles — the windows below clear it with margin).

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

    /// Finite-domain semantics (golden-bee precedent): a view wholly
    /// outside the patch is an empty success (message `None`); a
    /// degenerate view refuses with a message.
    #[test]
    fn empty_view_outside_patch() {
        let t = GlbBinaryN7Tiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query {
            region: BBox::new(Vec2::new(100.0, 100.0), Vec2::new(101.0, 101.0)),
        });
        assert!(r.message.is_none(), "outside view must not fail");
        assert!(r.tiles.is_empty());

        let r = t.generate(&p, &Query {
            region: BBox::new(Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0)),
        });
        assert_eq!(r.message, Some("empty view"));
        assert!(r.tiles.is_empty());
    }

    /// The area pre-guard refuses views too large for the tile budget
    /// before any refinement work (default scale, depth floored at
    /// D_MIN = 3: the guard threshold is MAX_TILES·s²·λ⁻⁴·sin(3π/7) ≈
    /// 127 500 world units², and a ±250 pane is 250 000).
    #[test]
    fn zoom_out_refuses() {
        let t = GlbBinaryN7Tiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query {
            region: BBox::new(Vec2::new(-250.0, -250.0), Vec2::new(250.0, 250.0)),
        });
        assert_eq!(r.message, Some("too many tiles for this view — zoom in"));
        assert!(r.tiles.is_empty());
    }

    /// Pins BOTH default presentations (golden-bee precedent):
    ///
    /// (a) The app/preview convention: a bounded ±10 probe, then
    ///     preview_window(9 × mean tile side) — the loop must land on
    ///     the D_MIN = 3 floor and frame the whole 35-tile patch
    ///     self-consistently at the default scale 2.75.
    /// (b) The CLI convention: a 20/scale-unit-tall origin-centered
    ///     pane (2:3 aspect); the patch-fitted scale default frames the
    ///     whole patch at ~62% of the pane height.
    #[test]
    fn default_view_fills_pane() {
        let t = GlbBinaryN7Tiling;
        let p = ParamSet::default_for(t.params());
        assert!(
            (p.float(SCALE_ID, 1.0) - 2.75).abs() < 1e-12,
            "scale default must stay patch-fitted (2.75)"
        );
        let s = 2.75;
        let patch_w = 3.425_428 * s; // B₀ bbox, world
        let patch_h = 1.649_599 * s;

        // --- (a) App/preview convention --------------------------------
        let probe = t.generate(&p, &Query {
            region: BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)),
        });
        assert!(probe.message.is_none());
        // The probe views the whole patch: exactly the depth-3 assembly.
        assert_eq!(probe.tiles.len(), 35, "±10 probe must show the full patch");
        let mw = tess_core::app::mean_tile_side(&probe.tiles);
        assert!(mw > 0.0, "probe must return tiles");
        let win = tess_core::app::preview_window(probe.periodic, mw, 4.0 / 3.0);
        let r = t.generate(&p, &Query { region: win });
        assert!(r.message.is_none());
        assert_eq!(r.tiles.len(), 35, "preview must show the depth-3 patch");
        assert!(
            win.max.x - win.min.x >= patch_w && win.max.y - win.min.y >= patch_h,
            "derived window must contain the whole patch"
        );
        let span_h = win.max.y - win.min.y;
        let fill = patch_h / span_h;
        assert!(
            (0.537..=0.577).contains(&fill),
            "preview frames the patch at {fill:.0}% of the pane height"
        );
        // Self-consistency: regenerating over a window re-derived from
        // THIS result lands on the same depth (no drift).
        let mw2 = tess_core::app::mean_tile_side(&r.tiles);
        let win2 = tess_core::app::preview_window(r.periodic, mw2, 4.0 / 3.0);
        let r2 = t.generate(&p, &Query { region: win2 });
        assert_eq!(r2.tiles.len(), 35, "second probe round must be a fixed point");

        // --- (b) CLI convention ----------------------------------------
        let region = BBox::new(
            Vec2::new(-20.0 / s * 2.0 / 3.0, -10.0 / s),
            Vec2::new(20.0 / s * 2.0 / 3.0, 10.0 / s),
        );
        let rc = t.generate(&p, &Query { region });
        assert!(rc.message.is_none());
        assert_eq!(rc.tiles.len(), 35, "CLI default pane must show the whole patch");
        let fill_c = patch_h / (region.max.y - region.min.y);
        assert!(
            (0.604..=0.644).contains(&fill_c),
            "CLI default pane frames the patch at {fill_c:.0}% of its height"
        );
    }
}