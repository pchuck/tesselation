//! Golden triangle (√−τ reptile) — the two-rate right-triangle
//! substitution tiling in this registry.
//!
//! The prototile is the right triangle with legs 1 and √τ and
//! hypotenuse τ (τ = (1+√5)/2; the leg ratio √τ makes the acute angles
//! atan(√τ) ≈ 51.83° and atan(1/√τ) ≈ 38.17°). The altitude from the
//! right angle dissects it into exactly **two similar copies of
//! unequal size** — a small child at scale τ⁻¹ (the half adjacent the
//! short leg) and a mid child at scale τ^(−1/2) — with area law
//! τ⁻² + τ⁻¹ = 1. Both children are the *opposite chirality* to the
//! parent, so the tiling needs the triangle and its mirror; together
//! they are the secondary-eigenvalue triangle pair of
//! Danzer–van Ophuysen (2000), the right-triangle prototile pair whose
//! substitution inflates by the **complex** multiplier λ = i√τ: with
//! λ² = −τ one has λ·L = L ∪ S and λ·S = L (Fibonacci growth matrix
//! [[1,1],[1,0]], area laws τ|L| = |L| + |S| and τ|S| = |L|, |λ|² = τ).
//!
//! **Exact arithmetic.** All geometry lives in K = Q(λ), a degree-4
//! field: an element is the integer 4-tuple (a0, a1, a2, a3) =
//! a0 + a1·λ + a2·λ² + a3·λ³ with λ² = −τ, hence the reduction law
//! λ⁴ = 1 − λ² (τ² = τ + 1). The standard complex embedding is
//! z ↦ (a0 − a2·τ) + i·√τ·(a1 − a3·τ). Every vertex of every tile is
//! an exact K element and the rule is the single uniform formula
//! F = P + τ⁻²·(Q − P) (τ⁻² = 2 − τ is exact in K); f64 is used only
//! (i) for the embedding that orders/gaps K decisions (distinct
//! elements at these coefficient sizes separate by more than 1e-9,
//! asserted at use) and (ii) at emission, after the exact patch is
//! computed. There are no matching rules: validity comes entirely from
//! the substitution.
//!
//! **Category — the §7 nuance, declared here.** The substitution
//! system with complex inflation √(−τ) is the Danzer–van Ophuysen
//! triangle-pair family; its deflation tree turns children by
//! ±atan(√τ) and ±atan(1/√τ) at every step, so edge directions are
//! dense (no finite orientation table, no translational symmetry in
//! the limit). Declared [`TilingCategory::Quasiperiodic`] on the cited
//! family; scope honesty: the exact battery below verifies the
//! substitution mechanics (two-piece reptile cuts on both prototiles,
//! area conservation, the λ-inflation piece identities, junction
//! censuses) — not the diffraction limit itself (golden-bee
//! precedent: declare the strongest category the evidence supports,
//! and say so).
//!
//! **Anchoring (finite-patch model, Taylor–Socolar / golden-bee
//! precedent).** The seed is a single unit triangle L₀ = (O, 1, λ) and
//! the patch *is* the covered domain. Refinement is in place — each
//! triangle is replaced exactly by its two children, children ⊆
//! parent, so depth d gives exactly 2^d triangles tiling the same
//! unit triangle (deflation: 2^{d−1} of each class, d ≥ 1; sizes
//! hyp² = τ^{e+2} sweep the ladder d..2d). The cell-space frame is
//! recentered so the patch bbox center is the world origin: a fixed
//! float offset RECENTRE = (−1/2, −√τ/2) is added at emission
//! (1/2 ∉ Z[λ], so the centering is a fixed float, not a
//! lattice-exact element — documented, and view-independent because
//! constant). Depth is view-anchored at the PATCH: with E the
//! cell-space view diagonal and the patch bbox diagonal exactly
//! √(1 + τ) = τ, d = max(D_PRESENT, zoom) where zoom = 0 for E ≥ τ
//! else ceil(−ln(E/τ) / ln τ), capped at MAX_DEPTH = 17 (2^17 ≤
//! 250 000 < 2^18). The floor exists because this is a BOUNDED patch:
//! d = 0 renders the entire tiling as one triangle, and the UI preview
//! window (probe → 9× mean tile side) is anchor-self-canceling.
//! D_PRESENT = 5 makes the probe-derived default view frame the whole
//! patch (bbox 1 × √τ — the same framing geometry as the chair, ≈ 69%
//! of the pane height, all 2^5 = 32 triangles, six size classes)
//! self-consistently at any scale and aspect.
//!
//! **T-junction structure (non-edge-to-edge, intrinsic).** The cut
//! foot splits the hypotenuse at the exact fraction τ⁻² = 2 − τ from
//! the short-leg vertex, so at deeper levels feet land strictly
//! inside sibling edges: the shipped patch has T-junction counts
//! 0, 0, 4, 16, 36 at depths 1..=5 (exactly 2·shared − boundary edge
//! incidences) — pinned in `junction_census`.
//!
//! **Pattern seed = size class.** Each tile's seed is the exponent −e
//! of its hypotenuse τ^{(e+2)/2}, so the default Pattern coloring
//! paints each size class one hue; `proto` is the class parity
//! (0 = L-type at even −e, 1 = S-type at odd −e).

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, ROTATION_ID, SCALE_ID};

// ---------------------------------------------------------------------------
// Exact arithmetic in K = Q(λ), λ = i√τ, λ⁴ = 1 − λ²
// ---------------------------------------------------------------------------

/// √τ as f64 (real embedding at emission and in tests only — the
/// exact field element is `LAM` below).
const SQRT_TAU_FLT: f64 = 1.272019649514069;
/// ln τ, for the depth anchor.
const LN_TAU: f64 = 0.48121182505960347;
/// The patch bbox diagonal in cell space: bbox [0,1] × [0,√τ] has
/// diagonal √(1 + τ) = √(τ²) = τ exactly.
const PATCH_DIAG: f64 = 1.618033988749895;
/// Presentation depth floor: the depth rendered whenever the whole
/// patch fits the view (E ≥ τ). A bounded patch at d = 0 IS one tile
/// — the lone seed triangle — so whole-patch views must show the
/// substitution structure instead. 5 is fitted to the UI preview
/// window (probe → 9× mean tile side, `tess_core::app`): mean AABB
/// side at depth 5 is ≈ 0.205 cell, so the derived window spans
/// ≈ 1.85 × 2.54 cell and frames the √τ-tall patch at ≈ 69% of the
/// pane height, all 32 triangles visible, at any scale and aspect.
/// Zoomed-in views (E < τ) refine past the floor by τ-octaves.
const D_PRESENT: u32 = 5;
/// Max refinement depth: 2^17 = 131 072 ≤ MAX_TILES < 2^18.
const MAX_DEPTH: u32 = 17;

/// An element of K = Q(λ): (a0, a1, a2, a3) ⇔
/// a0 + a1·λ + a2·λ² + a3·λ³, with λ² = −τ and λ⁴ = 1 − λ².
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct K(i64, i64, i64, i64);

impl K {
    const fn add(self, o: Self) -> Self {
        K(self.0 + o.0, self.1 + o.1, self.2 + o.2, self.3 + o.3)
    }
    const fn sub(self, o: Self) -> Self {
        K(self.0 - o.0, self.1 - o.1, self.2 - o.2, self.3 - o.3)
    }
    #[cfg(test)]
    const fn neg(self) -> Self {
        K(-self.0, -self.1, -self.2, -self.3)
    }
    /// x·y in K, reducing powers ≥ λ⁴ by λ⁴ = 1 − λ² (so λ⁵ = λ³ − λ,
    /// λ⁶ = 1 − 2λ²).
    #[cfg(test)]
    const fn mul(self, o: Self) -> Self {
        let (a0, a1, a2, a3) = (self.0, self.1, self.2, self.3);
        let (b0, b1, b2, b3) = (o.0, o.1, o.2, o.3);
        let c4 = a1 * b3 + a2 * b2 + a3 * b1; // λ⁴ slot
        let c5 = a2 * b3 + a3 * b2; // λ⁵ slot
        let c6 = a3 * b3; // λ⁶ slot
        K(
            a0 * b0 + c4 - c6,
            a0 * b1 + a1 * b0 + c5,
            a0 * b2 + a1 * b1 + a2 * b0 - c4 + 2 * c6,
            a0 * b3 + a1 * b2 + a2 * b1 + a3 * b0 - c5,
        )
    }
    /// x·τ⁻², the cut fraction τ⁻² = 2 − τ = 2 + λ² — the only
    /// constant product the generator performs (= 2x + x·λ², reducing
    /// λ⁴ = 1 − λ² and λ⁵ = λ − λ³).
    const fn mul_f(self) -> Self {
        K(
            2 * self.0 + self.2,
            2 * self.1 + self.3,
            self.0 + self.2,
            self.1 + self.3,
        )
    }
    /// Complex conjugate λ ↦ −λ (the nontrivial automorphism of
    /// K/Q(τ)); on the standard embedding it is complex conjugation.
    #[cfg(test)]
    const fn conj(self) -> Self {
        K(self.0, -self.1, self.2, -self.3)
    }
    /// Real part in the standard embedding: Re(a + bλ + cλ² + dλ³)
    /// = a − c·τ (real subfield Q(λ²) = Q(τ)).
    fn re(self) -> f64 {
        self.0 as f64 - self.2 as f64 * PATCH_DIAG
    }
    /// Imaginary part divided by √τ: Im(z)/√τ = b − d·τ.
    fn im_sqrtau(self) -> f64 {
        self.1 as f64 - self.3 as f64 * PATCH_DIAG
    }
    /// Standard complex embedding.
    fn cplx(self) -> (f64, f64) {
        (self.re(), SQRT_TAU_FLT * self.im_sqrtau())
    }
}

const K0: K = K(0, 0, 0, 0);
const K1: K = K(1, 0, 0, 0);
/// λ = i√τ, the field generator (= inflation multiplier).
const LAM: K = K(0, 1, 0, 0);
/// τ = −λ² (used by the tests' exact power bookkeeping).
#[cfg(test)]
const TAU_E: K = K(0, 0, -1, 0);

// ---------------------------------------------------------------------------
// The tiles and the substitution rule
// ---------------------------------------------------------------------------

/// One triangle in the exact patch, in canonical vertex order
/// `(r, p, q)`: right-angle vertex `r`, short-leg end `p`, long-leg
/// end `q`. `e` is the hypotenuse exponent: hyp² = τ^{e+2} (seed
/// L₀: e = 0, hyp = τ). The class is the parity of −e (0 = L-type,
/// 1 = S-type); chirality alternates with the generation, which the
/// canonical order carries exactly.
#[derive(Debug, Clone, Copy)]
struct Tri {
    r: K,
    p: K,
    q: K,
    e: i32,
}

/// One substitution step: the altitude cut from the right angle. The
/// foot F = P + τ⁻²(Q − P) splits the hypotenuse at the exact
/// fraction τ⁻² from the short-leg vertex; child A = (F, P, R) is the
/// τ⁻¹-scale half at the short leg (Δe = −2, same class) and child
/// B = (F, R, Q) the τ^(−1/2)-scale half at the long leg (Δe = −1,
/// class flips). Children ⊆ parent, so the set grows by exactly ×2.
fn refine(tiles: &[Tri]) -> Vec<Tri> {
    let mut out = Vec::with_capacity(tiles.len() * 2);
    for t in tiles {
        let f = t.p.add(t.q.sub(t.p).mul_f());
        out.push(Tri {
            r: f,
            p: t.p,
            q: t.r,
            e: t.e - 2,
        });
        out.push(Tri {
            r: f,
            p: t.r,
            q: t.q,
            e: t.e - 1,
        });
    }
    out
}

/// The seed: unit golden triangle L₀ = (O, 1, λ), legs 1 and √τ.
fn seed() -> Tri {
    Tri {
        r: K0,
        p: K1,
        q: LAM,
        e: 0,
    }
}

/// The patch at depth d (d refinements of the seed).
fn patch(d: u32) -> Vec<Tri> {
    let mut tiles = vec![seed()];
    for _ in 0..d {
        tiles = refine(&tiles);
    }
    tiles
}

/// Class of a tile: parity of −e (seed e = 0 → L-type).
fn cls_of(t: &Tri) -> u8 {
    ((-t.e) & 1) as u8
}

// ---------------------------------------------------------------------------
// The tiling
// ---------------------------------------------------------------------------

/// Golden triangle (√−τ reptile) — see the module docs.
///
/// Generator parameters: the shared similarity map, with the scale
/// default fitted to the finite patch. The patch spans only ~1.3 cell
/// units (bbox 1 × √τ — the chair's framing geometry), so scale 3.5
/// frames the whole patch in the standard 20/scale-tall pane at ~78%
/// height; the depth floor D_PRESENT = 5 then shows the 32-triangle
/// substitution structure at every scale. The scale slider (0.1–50)
/// still zooms; zoomed-in views refine past the floor by τ-octaves.
const PARAMS: [ParamSpec; 2] = [
    ParamSpec::float(SCALE_ID, "scale", 0.1, 50.0, 0.05, 3.5),
    ParamSpec::angle(ROTATION_ID, "rotation (deg)", 0.0, 360.0, 5.0, 0.0),
];

pub struct GoldenTriangleTiling;

impl Tiling for GoldenTriangleTiling {
    fn id(&self) -> &'static str {
        "golden-triangle"
    }

    fn name(&self) -> &'static str {
        "Golden triangle (√−τ reptile)"
    }

    fn description(&self) -> &'static str {
        "Golden triangle: the right triangle (legs 1, √τ; hypotenuse τ) \
         dissecting by its altitude into two similar children (scales τ^−1 and \
         τ^−1/2, area law τ^−2 + τ^−1 = 1); √(−τ)-inflation substitution \
         (λ = i√τ, λ² = −τ, λL = L ∪ S, λS = L) in exact Q(λ), both mirror \
         chiralities, dense rotations, T-junctions at the τ^−2 hypotenuse \
         ratio; finite covered patch — the seed triangle itself \
         (Danzer–van Ophuysen 2000)"
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
            "quasiperiodic (Danzer–van Ophuysen √(−τ) triangle family); \
             finite covered patch — the seed triangle is the domain",
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

        // The patch bbox in cell space after the fixed recentering:
        // the seed triangle [0,1] × [0,√τ] shifted by (−1/2, −√τ/2).
        let s = SQRT_TAU_FLT;
        let patch_bb = BBox::new(Vec2::new(-0.5, -s * 0.5), Vec2::new(0.5, s * 0.5));
        if !patch_bb.intersects(region) {
            // Finite patch: outside the covered domain is background.
            return TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: None,
                message: None,
            };
        }

        // View-anchored depth, anchored at the PATCH: zoom octaves are
        // measured relative to the patch diagonal τ (each τ-shrink of
        // the cell-space view diagonal adds one level), and
        // whole-patch views floor at D_PRESENT (see the module docs).
        let ext = (region.max - region.min).length();
        let zoom = if ext < PATCH_DIAG {
            ((-(ext / PATCH_DIAG).ln()) / LN_TAU).ceil() as u32
        } else {
            0
        };
        let d = zoom.clamp(D_PRESENT, MAX_DEPTH);

        let tiles = patch(d);

        // Emission: exact K → complex embedding + the fixed
        // recentering, culled to the query region.
        let rx = -0.5f64;
        let ry = -s * 0.5;
        let mut out = Vec::new();
        for t in &tiles {
            let pts: Vec<Vec2> = [t.r, t.p, t.q]
                .iter()
                .map(|v| {
                    let (x, y) = v.cplx();
                    Vec2::new(x + rx, y + ry)
                })
                .collect();
            let poly = Polygon::new(pts).transform(&map);
            if poly.aabb().intersects(query.region) {
                out.push(Tile {
                    proto: cls_of(t),
                    seed: (-t.e) as u32, level: (-t.e).min(255) as u8,
                    poly,
                });
            }
        }
        TilingResult {
            tiles: out,
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

    /// τ⁻² = 2 − τ = 2 + λ², the exact hypotenuse cut fraction (the
    /// constant baked into `K::mul_f`, cross-checked below).
    const F2I: K = K(2, 0, 1, 0);

    // --- exact-K sign helpers (real embedding, gap-asserted) ---------
    // K has no intrinsic order; distinct elements at the coefficient
    // sizes used here separate by > 1e-9 in ℝ or ℂ, so the float
    // decision is safe once the exact-zero case is handled by
    // coefficient equality.

    /// Sign of a REAL K-element (imaginary coefficients exactly 0):
    /// value = a0 − a2·τ.
    fn sgn_re(z: K) -> i32 {
        assert_eq!((z.1, z.3), (0, 0), "sgn_re on non-real {z:?}");
        if z.0 == 0 && z.2 == 0 {
            return 0;
        }
        let v = z.re();
        assert!(v.abs() > 1e-9, "ambiguous real K order: {z:?} ({v})");
        if v > 0.0 {
            1
        } else {
            -1
        }
    }

    /// Sign of Im(z)/√τ (= b − d·τ; the imaginary part up to the
    /// positive factor √τ).
    fn sgn_im(z: K) -> i32 {
        if z.1 == 0 && z.3 == 0 {
            return 0;
        }
        let v = z.im_sqrtau();
        assert!(v.abs() > 1e-9, "ambiguous imag K order: {z:?} ({v})");
        if v > 0.0 {
            1
        } else {
            -1
        }
    }

    /// Orientation sign of the triple (a → b → c): sign of
    /// Im(conj(b−a)·(c−a)).
    fn cross_s(a: K, b: K, c: K) -> i32 {
        sgn_im(K::conj(b.sub(a)).mul(c.sub(a)))
    }

    /// Squared modulus (real K-element, ∈ Q(τ)).
    fn norm2(z: K) -> K {
        K::conj(z).mul(z)
    }

    /// τ⁻¹ = −1 − λ² exact: τ·(−1 − λ²) = λ² + λ⁴ = λ² + 1 − λ² = 1
    /// (pinned in `field_law_and_arithmetic`).
    const TAU_INV: K = K(-1, 0, -1, 0);

    /// τ^k for small k as an exact K-element.
    fn tau_pow(k: i32) -> K {
        let mut p = K1;
        for _ in 0..k.abs() {
            p = p.mul(if k >= 0 { TAU_E } else { TAU_INV });
        }
        p
    }

    /// Sign of a t-basis pair (u, v) ⇔ u + v·λ², value u − v·τ.
    fn sgnq(p: (i64, i64)) -> i32 {
        if p == (0, 0) {
            return 0;
        }
        let v = p.0 as f64 - p.1 as f64 * PATCH_DIAG;
        assert!(v.abs() > 1e-9, "ambiguous Q(τ) order: {p:?} ({v})");
        if v > 0.0 {
            1
        } else {
            -1
        }
    }

    /// Twice the signed area of the exact triangle, divided by √τ:
    /// with vertices z_i = (x_i, √τ·y_i), x = a0 + a2·λ² and
    /// y = a1 + a3·λ², the shoelace sum is √τ·Σ with Σ in the
    /// t = λ²-basis (t² = 1 − t); the seed value is the pair (1, 0).
    fn area2(t: &Tri) -> (i64, i64) {
        let pts = [t.r, t.p, t.q];
        let mut s = (0i64, 0i64);
        for i in 0..3 {
            let z = pts[i];
            let w = pts[(i + 1) % 3];
            // term x_z·y_w − x_w·y_z in the t-basis: products
            // (u0 + u1 t)(v0 + v1 t) = u0v0 + u1v1 t² + (u0v1 + u1v0)t
            // with t² = 1 − t.
            let (x0, x1) = (z.0, z.2); // x = a0 + a2·t
            let (y0, y1) = (w.1, w.3); // y = b + d·t
            let (u0, u1) = (w.0, w.2);
            let (v0, v1) = (z.1, z.3);
            let p0 = x0 * y0 + x1 * y1;
            let p1 = x0 * y1 + x1 * y0 - x1 * y1;
            let q0 = u0 * v0 + u1 * v1;
            let q1 = u0 * v1 + u1 * v0 - u1 * v1;
            s = (s.0 + p0 - q0, s.1 + p1 - q1);
        }
        s
    }

    // --- field laws --------------------------------------------------

    #[test]
    fn field_law_and_arithmetic() {
        // λ⁴ + λ² = 1 and λ⁻¹ = λ³ + λ (the defining quartic).
        let l2 = LAM.mul(LAM);
        let l4 = l2.mul(l2);
        assert_eq!(l4.add(l2), K1, "λ⁴ + λ² = 1");
        assert_eq!(LAM.mul(K(0, 1, 0, 1)), K1, "λ⁻¹ = λ³ + λ");
        // τ² = τ + 1 with τ = −λ².
        assert_eq!(TAU_E.mul(TAU_E), TAU_E.add(K1), "τ² = τ + 1");
        // τ⁻¹ and τ⁻² exact.
        assert_eq!(TAU_E.mul(TAU_INV), K1, "τ·τ⁻¹ = 1");
        assert_eq!(F2I.mul(TAU_E).mul(TAU_E), K1, "F = τ⁻²");
        // mul_f agrees with the general product; conjugation is a
        // field automorphism; associativity on generator tuples.
        let x = K(3, -2, 4, 1);
        assert_eq!(x.mul_f(), x.mul(F2I));
        assert_eq!(K::conj(x.mul(LAM)), K::conj(x).mul(K::conj(LAM)));
        let y = K(-1, 2, 0, 3);
        let z = K(2, 0, -3, 1);
        assert_eq!(x.mul(y.mul(z)), x.mul(y).mul(z));
        assert_eq!(x.mul(y.add(z)), x.mul(y).add(x.mul(z)));
        // The complex embedding is a ring homomorphism on samples.
        let (xr, xi) = x.cplx();
        let (yr, yi) = y.cplx();
        let p = x.mul(y);
        let (pr, pi) = p.cplx();
        let tol = 1e-9;
        assert!((pr - (xr * yr - xi * yi)).abs() < tol);
        assert!((pi - (xr * yi + xi * yr)).abs() < tol);
    }

    // --- prototile identities ---------------------------------------

    /// S₀ = (O, λ/τ, 1) in canonical order: legs τ^−1/2 (short, to
    /// λ/τ) and 1 (long, to 1), hyp² = τ = τ^{e+2} at e = −1.
    fn s_seed() -> Tri {
        Tri {
            r: K0,
            p: LAM.mul(TAU_INV),
            q: K1,
            e: -1,
        }
    }

    #[test]
    fn prototile_identities() {
        let l0 = seed(); // (O, 1, λ): legs 1, √τ
        // Right angle at r: Re(conj(p−r)·(q−r)) = 0 exactly, and the
        // product is purely imaginary (real K coefficients zero).
        let w = K::conj(l0.p.sub(l0.r)).mul(l0.q.sub(l0.r));
        assert_eq!((w.0, w.2), (0, 0));
        assert_eq!(norm2(l0.p.sub(l0.r)), K1, "short leg 1");
        assert_eq!(norm2(l0.q.sub(l0.r)), TAU_E, "long leg √τ");
        assert_eq!(norm2(l0.q.sub(l0.p)), tau_pow(2), "hyp τ");
        assert_eq!(area2(&l0), (1, 0), "seed 2A/√τ = 1");
        // S₀: right, legs τ^−1/2 and 1, hyp² = τ; the two classes are
        // similar at linear ratio √τ (hyp² ratio = τ exactly).
        let s0 = s_seed();
        let w = K::conj(s0.p.sub(s0.r)).mul(s0.q.sub(s0.r));
        assert_eq!((w.0, w.2), (0, 0), "S₀ right angle");
        assert_eq!(norm2(s0.p.sub(s0.r)), TAU_INV, "S₀ short leg τ^−1/2");
        assert_eq!(norm2(s0.q.sub(s0.r)), K1, "S₀ long leg 1");
        assert_eq!(norm2(s0.q.sub(s0.p)), TAU_E, "S₀ hyp √τ");
        assert_eq!(
            norm2(s0.q.sub(s0.p)).mul(TAU_E),
            norm2(l0.q.sub(l0.p)),
            "class similarity at ratio √τ"
        );
        // hyp² ladder: τ^{e+2} for both seeds (e = 0 → τ², e = −1 → τ).
        assert_eq!(norm2(l0.q.sub(l0.p)), tau_pow(2));
        assert_eq!(norm2(s0.q.sub(s0.p)), tau_pow(1));
    }

    #[test]
    fn altitude_cut_children() {
        // Run the uniform rule on both prototiles (L₀ e = 0 and the
        // canonical S₀ e = −1).
        for par in [seed(), s_seed()] {
            let kids = refine(&[par]);
            assert_eq!(kids.len(), 2);
            for k in &kids {
                // right angle at the foot F: Re(conj(p−r)·(q−r)) = 0
                // exactly (slots zero), purely imaginary product
                let w = K::conj(k.p.sub(k.r)).mul(k.q.sub(k.r));
                assert_eq!((w.0, w.2), (0, 0), "child right angle {k:?}");
                assert_ne!((w.1, w.3), (0, 0), "degenerate child {k:?}");
                // child foot lies strictly inside the parent hypotenuse
                assert_eq!(cross_s(par.p, par.q, k.r), 0, "foot collinear");
                assert!(on_open(k.r, par.p, par.q), "foot interior");
            }
            let a = &kids[0];
            let b = &kids[1];
            assert_eq!(a.e, par.e - 2);
            assert_eq!(b.e, par.e - 1);
            assert_eq!(cls_of(a), cls_of(&par));
            assert_eq!(cls_of(b), cls_of(&par) ^ 1);
            // hypotenuses: A's = parent short leg, B's = parent long leg
            assert_eq!(norm2(a.q.sub(a.p)), norm2(par.p.sub(par.r)), "A hyp = short leg");
            assert_eq!(norm2(b.q.sub(b.p)), norm2(par.q.sub(par.r)), "B hyp = long leg");
            // hyp ladder consistency with the exponent bookkeeping
            assert_eq!(norm2(a.q.sub(a.p)), tau_pow(a.e + 2), "A ladder");
            assert_eq!(norm2(b.q.sub(b.p)), tau_pow(b.e + 2), "B ladder");
            // exact area split: area(A) + area(B) = area(parent)
            let mut sa = area2(a);
            let mut sb = area2(b);
            let mut sp = area2(&par);
            if sgnq(sa) < 0 {
                sa = (-sa.0, -sa.1);
            }
            if sgnq(sb) < 0 {
                sb = (-sb.0, -sb.1);
            }
            if sgnq(sp) < 0 {
                sp = (-sp.0, -sp.1);
            }
            assert_eq!((sa.0 + sb.0, sa.1 + sb.1), sp, "area split");
        }
    }

    // --- patch triangulation ----------------------------------------

    #[test]
    fn patch_is_triangulation() {
        for d in 0..=6u32 {
            let tiles = patch(d);
            let n = tiles.len();
            assert_eq!(n, if d == 0 { 1 } else { 1 << d }, "count at d={d}");
            if d >= 1 {
                let cl = tiles.iter().filter(|t| cls_of(t) == 0).count();
                assert_eq!(cl, 1 << (d - 1), "L census at d={d}");
                assert_eq!(n - cl, 1 << (d - 1), "S census at d={d}");
            }
            // exact area conservation (Σ|2A|/√τ = 1, the seed value)
            let mut tot = (0i64, 0i64);
            for t in &tiles {
                let s = area2(t);
                assert_ne!(sgnq(s), 0, "degenerate tile at d={d}");
                let g = if sgnq(s) > 0 { s } else { (-s.0, -s.1) };
                tot = (tot.0 + g.0, tot.1 + g.1);
            }
            assert_eq!(tot, (1, 0), "total area drift at d={d}");

            // pairwise: no proper crossings; collinear overlaps only
            // as shared-boundary contact (third vertices strictly on
            // opposite sides); no vertex strictly inside another tile.
            for i in 0..n {
                for j in (i + 1)..n {
                    let a = [tiles[i].r, tiles[i].p, tiles[i].q];
                    let b = [tiles[j].r, tiles[j].p, tiles[j].q];
                    for ei in 0..3 {
                        for ej in 0..3 {
                            let (a1, a2) = (a[ei], a[(ei + 1) % 3]);
                            let (b1, b2) = (b[ej], b[(ej + 1) % 3]);
                            let s1 = cross_s(a1, a2, b1);
                            let s2 = cross_s(a1, a2, b2);
                            let s3 = cross_s(b1, b2, a1);
                            let s4 = cross_s(b1, b2, a2);
                            if s1 * s2 < 0 && s3 * s4 < 0 {
                                panic!("proper crossing at d={d} tiles {i},{j}");
                            }
                            if s1 == 0 && s2 == 0 && s3 == 0 && s4 == 0 {
                                let overlap = [b1, b2, a1, a2].iter().any(|&pt| {
                                    let (u, w) = if pt == b1 || pt == b2 {
                                        (a1, a2)
                                    } else {
                                        (b1, b2)
                                    };
                                    on_open(pt, u, w)
                                });
                                if overlap {
                                    let ta = a[(ei + 2) % 3];
                                    let tb = b[(ej + 2) % 3];
                                    assert!(
                                        cross_s(a1, a2, ta) * cross_s(a1, a2, tb) < 0,
                                        "same-side collinear overlap at d={d} tiles {i},{j}"
                                    );
                                }
                            }
                        }
                    }
                    for &v in a.iter() {
                        assert!(!pip(&tiles[j], v), "vertex of {i} strictly inside {j} at d={d}");
                    }
                    for &v in b.iter() {
                        assert!(!pip(&tiles[i], v), "vertex of {j} strictly inside {i} at d={d}");
                    }
                }
            }
        }
    }

    /// Strict interior of segment (u, w) for collinear p.
    fn on_open(p: K, u: K, w: K) -> bool {
        if cross_s(u, w, p) != 0 {
            return false;
        }
        let len = norm2(w.sub(u));
        let v = K::conj(w.sub(u)).mul(p.sub(u)); // real when collinear
        assert_eq!((v.1, v.3), (0, 0));
        sgn_re(v) > 0 && sgn_re(len.sub(v)) > 0
    }

    /// Strict point-in-triangle (exact orientation signs).
    fn pip(t: &Tri, pt: K) -> bool {
        let s1 = cross_s(t.r, t.p, pt);
        let s2 = cross_s(t.p, t.q, pt);
        let s3 = cross_s(t.q, t.r, pt);
        (s1 > 0 && s2 > 0 && s3 > 0) || (s1 < 0 && s2 < 0 && s3 < 0)
    }

    // --- deflation census (Python battery §8 table, pinned) ----------

    #[test]
    fn deflation_census_matches_battery() {
        // At depth d the hypotenuse exponents sweep −e = d + k,
        // k = 0..=d, with count exactly C(d, k): each depth-d path
        // chooses k of the d cuts to take the Δe = −2 branch. Rows
        // d = 0..=8 are the battery's printed e-histograms
        // (e.g. d=4: 1, 4, 6, 4, 1 at −e = 4..8), totalling 2^d with
        // 2^(d−1) tiles of each class for d ≥ 1.
        let mut bin = [[0u64; 11]; 11];
        for n in 0..11 {
            bin[n][0] = 1;
            for k in 1..=n {
                bin[n][k] = bin[n - 1][k - 1] + bin[n - 1][k];
            }
        }
        for d in 0..=10u32 {
            let tiles = patch(d);
            let mut counts = vec![0u64; d as usize + 1];
            let mut lc = 0usize;
            for t in &tiles {
                let k = (-t.e - d as i32) as usize;
                counts[k] += 1;
                lc += (cls_of(t) == 0) as usize;
            }
            assert_eq!(
                counts,
                (0..=d as usize).map(|k| bin[d as usize][k]).collect::<Vec<_>>(),
                "e-histogram at d={d}"
            );
            if d >= 1 {
                assert_eq!(lc, 1 << (d - 1), "class census at d={d}");
            }
        }
    }

    // --- junction census ---------------------------------------------

    #[test]
    fn junction_census() {
        // Shared (reversed-pair) and boundary edge incidences and the
        // T-junction count (vertices strictly inside a non-partner
        // edge), pinned from the Python battery for depths 1..=5.
        const PINS: [(usize, usize, usize); 5] =
            [(2, 4, 0), (6, 6, 0), (12, 12, 4), (24, 24, 16), (50, 46, 36)];
        for (di, &(shared, boundary, tjs)) in PINS.iter().enumerate() {
            let d = di as u32 + 1;
            let tiles = patch(d);
            let mut edges = Vec::new();
            for (ti, t) in tiles.iter().enumerate() {
                let v = [t.r, t.p, t.q];
                for k in 0..3 {
                    edges.push((v[k], v[(k + 1) % 3], ti));
                }
            }
            let mut sh = 0;
            let mut bd = 0;
            let mut tj = 0;
            for (ui, vi, ti) in &edges {
                let match_cnt = edges
                    .iter()
                    .filter(|&&(u2, v2, t2)| t2 != *ti && u2 == *vi && v2 == *ui)
                    .count();
                assert!(match_cnt <= 1, "edge paired twice at d={d}");
                if match_cnt == 1 {
                    sh += 1;
                } else {
                    bd += 1;
                }
                for (u2, v2, t2) in &edges {
                    if t2 != ti && !(*u2 == *vi && *v2 == *ui) && on_open(*ui, *u2, *v2) {
                        tj += 1;
                    }
                }
            }
            assert_eq!((sh, bd, tj), (shared, boundary, tjs), "census at d={d}");
        }
    }

    // --- chirality and rotation bookkeeping ---------------------------

    #[test]
    fn chirality_flips_each_generation() {
        // orientation sign of (r, p, q)
        fn orient(t: &Tri) -> i32 {
            cross_s(t.r, t.p, t.q)
        }
        let mut tiles = vec![seed()];
        let mut o = orient(&tiles[0]);
        assert_eq!(o, 1, "seed orientation CCW");
        for d in 1..=4u32 {
            tiles = refine(&tiles);
            for t in &tiles {
                assert_eq!(orient(t), -o, "chirality must flip at every cut, d={d}");
            }
            o = -o;
        }
    }

    #[test]
    fn child_rotation_multipliers() {
        // In the parent frame (long leg O->lambda), the children's long
        // legs are exact K-multiples of the parent's, pinning the turn
        // and scale of each half of the cut:
        //   child A (scale tau^-1, turn +128.17 deg):  uA*lam = -F
        //   child B (scale tau^-1/2, turn +38.17 deg): uB*lam = lam - F
        // with the foot F = (-1, 2, -1, 1). Both u's are exact in
        // Z[lam] — pinned by multiplication, no division.
        let par = seed();
        let kids = refine(&[par]);
        let (a, b) = (&kids[0], &kids[1]);
        assert_eq!(a.r, K(-1, 2, -1, 1), "foot F of the seed cut");
        let ua = K(-2, 2, -1, 1);
        let ub = K(-1, 2, -1, 1);
        assert_eq!(ua.mul(LAM), a.r.neg(), "uA*lam = R_A - F = -F");
        assert_eq!(ub.mul(LAM), par.q.sub(b.r), "uB*lam = Q - F");
        assert_eq!(norm2(ua), TAU_INV.mul(TAU_INV), "|uA|^2 = tau^-2");
        assert_eq!(norm2(ub), TAU_INV, "|uB|^2 = tau^-1");
        // Embedded angles/scales (presentation bookkeeping): the two
        // turns are the acute angles of the tile (and 180° minus
        // them), so deflation directions are dense — no finite
        // orientation table exists.
        let (ar, ai) = ua.cplx();
        let (br, bi) = ub.cplx();
        let aa = ai.atan2(ar);
        let ab = bi.atan2(br);
        assert!((aa - 2.237_033_958_6).abs() < 1e-4, "uA angle {aa} rad");
        assert!((ab - 0.666_256_418_6).abs() < 1e-4, "uB angle {ab} rad");
        assert!((hypot2(ar, ai) - 1.0 / PATCH_DIAG).abs() < 1e-9, "|uA| = tau^-1");
        assert!(
            (hypot2(br, bi) - SQRT_TAU_FLT / PATCH_DIAG).abs() < 1e-9,
            "|uB| = tau^-1/2"
        );
    }

    fn hypot2(x: f64, y: f64) -> f64 {
        (x * x + y * y).sqrt()
    }

    // --- inflation law ------------------------------------------------

    #[test]
    fn inflation_pieces() {
        // λ·L₀ = (O, V=λ, H=λ²=−τ). The cut foot of the inflated
        // triangle is X = λ·F (the rule is equivariant under the
        // similarity λ), splitting V..H at the same τ⁻² fraction; the
        // two pieces are an S-unit and an L-unit of the tiling — the
        // inflation law λL = L ∪ S.
        let f0 = K1.add(LAM.sub(K1).mul_f()); // foot in L₀
        assert_eq!(f0, K(-1, 2, -1, 1), "seed foot");
        let x = LAM.mul(f0); // inflated foot
        let v = LAM;
        let h = LAM.mul(LAM);
        // X strictly on segment V..H, at the same τ⁻² ratio:
        assert_eq!(cross_s(v, h, x), 0, "inflated foot collinear");
        assert!(on_open(x, v, h), "inflated foot interior");
        assert_eq!(
            x.sub(v).mul(K1.sub(F2I)),
            h.sub(x).mul(F2I),
            "foot ratio τ⁻² equivariant under λ"
        );
        // piece hypotenuses: |OV|² = τ (S-unit), |OH|² = τ² (L-unit)
        assert_eq!(norm2(v), TAU_E);
        assert_eq!(norm2(h), tau_pow(2));
        // altitude: |OF|² = τ⁻¹ in the seed, |OX|² = 1 inflated
        assert_eq!(norm2(f0), TAU_INV, "|OF|² = τ⁻¹");
        assert_eq!(norm2(x), K1, "|OX|² = 1");
        // λ·S₀ = (O, λ·(λ/τ) = −1, λ): hyp² = |λ − (−1)|² = τ² — an
        // L-unit up to rotation (λS = L).
        let qs = LAM.mul(TAU_INV);
        let sv = LAM.mul(qs);
        let sw = LAM;
        assert_eq!(sv, K(-1, 0, 0, 0), "λ·(λ/τ) = −1");
        assert_eq!(norm2(sw.sub(sv)), tau_pow(2), "λ·S₀ hyp² = τ²");
    }

    // --- standard trio + framing --------------------------------------

    // Windows strictly INSIDE the recentered seed triangle: unlike
    // the chair's origin-symmetric bbox frame, the recentered
    // hypotenuse passes exactly through the origin, so an
    // origin-centered window would be half outside the patch. The
    // incircle (center ≈ (−0.173, −0.309), radius (1 + √τ − τ)/2 ≈
    // 0.327) frames the windows below.
    const WIN: [f64; 4] = [-0.31, -0.45, -0.04, -0.17];
    const WIN_ROT: [f64; 4] = [-0.135_722, -0.331_909, 0.104_278, -0.091_909];

    #[test]
    fn golden_triangle_covers_region() {
        check_covers(
            &GoldenTriangleTiling,
            "golden-triangle",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }

    #[test]
    fn golden_triangle_rotated_scaled() {
        // Square around the world image of the cell-space incenter
        // (half side 0.12 world): the inverse cell image is a −25°
        // rotated square of half-diagonal 0.283 around the incenter —
        // strictly inside the incircle (0.327), hence fully covered.
        check_rotated(
            &GoldenTriangleTiling,
            "golden-triangle(rot25,scale0.6)",
            BBox::new(Vec2::new(WIN_ROT[0], WIN_ROT[1]), Vec2::new(WIN_ROT[2], WIN_ROT[3])),
            0.6,
            25.0,
        );
    }

    #[test]
    fn golden_triangle_deterministic() {
        check_det(
            &GoldenTriangleTiling,
            "golden-triangle",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }

    /// (proto, seed, vertex bit pattern) — bit-identity key.
    fn tile_key(tile: &Tile) -> (u8, u32, Vec<u64>) {
        let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
        for p in &tile.poly.pts {
            bits.push(p.x.to_bits());
            bits.push(p.y.to_bits());
        }
        (tile.proto, tile.seed, bits)
    }

    fn gen_keys(region: BBox) -> Vec<(u8, u32, Vec<u64>)> {
        let t = GoldenTriangleTiling;
        // Mechanics pinned at scale 1.0, where the window geometry
        // below was derived; the DEFAULT scale is 3.5 (patch-fitted —
        // see default_view_fills_pane).
        let mut p = ParamSet::default_for(t.params());
        p.set(SCALE_ID, tess_core::params::ParamValue::Float(1.0));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "unexpected message");
        assert!(!r.tiles.is_empty(), "empty view");
        r.tiles.iter().map(tile_key).collect()
    }

    #[test]
    fn patch_nested_view_bit_identical() {
        // All five windows sit in the τ-octave [φ⁻⁴, φ⁻³) of
        // patch-relative view diagonals (E/τ ≈ 0.19 / 0.19 / 0.19 /
        // 0.16 / 0.16 — the patch diagonal τ is the chair's φ), so all
        // pick the same depth (d = 5 = D_PRESENT, pinned by the size
        // classes −e ∈ 5..=10 below) and refine the same patch: every
        // tile of a smaller view must reappear bit-identically in each
        // larger one containing it.
        let base = gen_keys(BBox::new(Vec2::new(-0.09, -0.12), Vec2::new(0.09, 0.12)));
        let grown = gen_keys(BBox::new(Vec2::new(-0.0945, -0.126), Vec2::new(0.0945, 0.126)));
        let panned = gen_keys(BBox::new(Vec2::new(-0.07, -0.09), Vec2::new(0.11, 0.15)));
        let overlap = gen_keys(BBox::new(Vec2::new(-0.07, -0.09), Vec2::new(0.09, 0.12)));
        let zoomed = gen_keys(BBox::new(Vec2::new(-0.0765, -0.102), Vec2::new(0.0765, 0.102)));
        for k in base
            .iter()
            .chain(grown.iter())
            .chain(panned.iter())
            .chain(overlap.iter())
            .chain(zoomed.iter())
        {
            assert!(
                (5..=10).contains(&k.1),
                "view picked depth != 5: size class {}",
                k.1
            );
        }
        for k in &base {
            assert!(grown.contains(k), "base tile not bit-identical in grown view");
        }
        for k in &zoomed {
            assert!(base.contains(k), "zoomed tile not bit-identical in base view");
        }
        for k in &overlap {
            assert!(
                base.contains(k) && panned.contains(k),
                "overlap tile not bit-identical in base ∩ panned"
            );
        }
    }

    #[test]
    fn default_view_fills_pane() {
        // Pins BOTH default presentations against the lone-seed bug
        // (the patch at d = 0 IS the whole tiling — one gray
        // triangle):
        //
        // (a) The UI/preview convention (TUI render pane, GUI canvas,
        //     card thumbnails): tess_core::app derives the window from
        //     the tiling itself — a bounded ±10 probe, then
        //     preview_window(9 × mean tile side) — an anchor-self-
        //     canceling loop that must land on the D_PRESENT floor and
        //     frame the whole patch (~69% of the pane height, 32
        //     triangles) self-consistently at the default scale.
        // (b) The CLI convention: a ~20/scale-unit-tall origin-centered
        //     pane; the patch-fitted scale default (3.5) frames the
        //     whole patch at ~78% height (the chair's framing
        //     geometry — same 1 × √τ bbox).
        let t = GoldenTriangleTiling;
        let p = ParamSet::default_for(t.params());
        assert!(
            (p.float(SCALE_ID, 1.0) - 3.5).abs() < 1e-12,
            "scale default must stay patch-fitted (3.5)"
        );
        let s = SQRT_TAU_FLT;
        let patch_h = s * p.float(SCALE_ID, 1.0); // patch bbox height, world
        let patch_w = p.float(SCALE_ID, 1.0);

        // --- (a) App/preview convention --------------------------------
        let probe = t.generate(
            &p,
            &Query {
                region: BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)),
            },
        );
        assert!(probe.message.is_none());
        let mw = tess_core::app::mean_tile_side(&probe.tiles);
        assert!(mw > 0.0, "probe must return tiles");
        let win = tess_core::app::preview_window(probe.periodic, mw, 4.0 / 3.0);
        let r = t.generate(&p, &Query { region: win });
        assert!(r.message.is_none());
        // The floor renders the whole patch: exactly 2^5 triangles
        // (the view ⊇ patch at every aspect for the derived window).
        assert_eq!(r.tiles.len(), 32, "preview must show the depth-5 patch");
        let classes: std::collections::BTreeSet<u32> = r.tiles.iter().map(|tl| tl.seed).collect();
        assert!(
            classes.len() >= 5,
            "preview must show the size hierarchy, got classes {classes:?}"
        );
        let span_h = win.max.y - win.min.y;
        let fill = patch_h / span_h;
        assert!(
            (0.55..=0.95).contains(&fill),
            "preview frames the patch at {fill:.0}% of the pane height"
        );
        assert!(
            win.max.x - win.min.x >= patch_w && span_h >= patch_h,
            "derived window must contain the whole patch"
        );
        // Self-consistency: regenerating over a window re-derived from
        // THIS result lands on the same depth (no d = 0 regression).
        let mw2 = tess_core::app::mean_tile_side(&r.tiles);
        let win2 = tess_core::app::preview_window(r.periodic, mw2, 4.0 / 3.0);
        let r2 = t.generate(&p, &Query { region: win2 });
        assert_eq!(r2.tiles.len(), 32, "second probe round must be a fixed point");

        // --- (b) CLI convention ----------------------------------------
        let region = BBox::new(
            Vec2::new(-20.0 / 3.5 * 2.0 / 3.0, -10.0 / 3.5),
            Vec2::new(20.0 / 3.5 * 2.0 / 3.0, 10.0 / 3.5),
        );
        let rc = t.generate(&p, &Query { region });
        assert!(rc.message.is_none());
        assert_eq!(rc.tiles.len(), 32, "CLI default pane must show the whole patch");
        let fill_c = patch_h / (region.max.y - region.min.y);
        assert!(
            (0.7..=0.85).contains(&fill_c),
            "CLI default pane frames the patch at {fill_c:.0}% of its height"
        );
    }
}
