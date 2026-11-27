//! Golden bee (Ammann A2 "chair") — the first irreptile substitution
//! tiling in this registry.
//!
//! The prototile is Ammann's A2 "chair": a concave hexagon with five
//! right angles and one reflex angle, whose six edge lengths are
//! {1, s, s⁻¹, s⁻², s⁻³, s⁻⁴} (s = √φ, φ the golden ratio) and whose
//! area is s − s⁻⁷. It dissects into exactly **two smaller similar
//! copies of unequal size** — a large child at scale s⁻¹ = φ^(-1/2)
//! (area fraction 1/φ) and a small child at scale s⁻² = φ⁻¹ (area
//! fraction 1/φ²), with the area law s⁻² + s⁻⁴ = 1. Discovered by
//! Robert Ammann (Tilings Encyclopedia "Ammann Chair", aperiodic set
//! A2), independently rediscovered by Karl Scherer; the only polygon
//! other than non-isosceles right triangles that admits such a
//! non-congruent similar-pair dissection (Vince, "Self-Similar
//! Polygonal Tilings", Amer. Math. Monthly 121 (2014)).
//!
//! **Category — the §7 nuance, declared here (per PROPOSED.md §7).**
//! Vince (2014) proves the *canonical* golden-bee substitution
//! sequence quasiperiodic; the structure of A2 tilings is classified
//! in Durand–Shen–Vereshchagin, Discrete Comput. Geom. 63 (2020).
//! This module implements exactly that canonical rule, so it is
//! declared [`TilingCategory::Quasiperiodic`]. Scope honesty: the
//! exact Q(√φ) battery below verifies the substitution mechanics
//! (two-piece dissections, area law, size hierarchy, junction ratios)
//! — not the diffraction limit itself. The quasiperiodicity claim
//! rests on the cited theorem, not on re-proof here (chair precedent:
//! declare the strongest category the evidence supports, and say so).
//!
//! **Exact arithmetic.** All geometry lives in K = Q(√φ), a degree-4
//! field: an element is the integer 4-tuple (a0, a1, a2, a3) =
//! a0 + a1·s + a2·s² + a3·s³ with s² = φ, hence the reduction law
//! s⁴ = s² + 1 (the same quartic-field fixed-point trick as
//! `girih_qc`). The 8 chair shapes, every substitution anchor, and
//! every test assertion are exact K equalities; f64 is used only
//! (i) for the real embedding that *orders* K elements (safe here —
//! distinct elements separate by more than 1e-9, asserted at use)
//! and (ii) at emission, after the exact patch is computed.
//!
//! **Anchoring (finite-patch model, Taylor–Socolar / Jeandel–Rao
//! precedent).** We ship a Taylor–Socolar/JR-style finite patch: the
//! seed is a single unit A0 chair and the patch *is* the covered
//! domain. Refinement is in place — each chair is replaced exactly by
//! its two children, children ⊆ parent, so refinement never culls and
//! depth d gives exactly 2^d chairs tiling the same unit chair. The
//! cell-space frame is recentered so the patch bbox center is the
//! world origin: a fixed float offset RECENTRE = (−1/2, −s/2) is
//! added at emission (1/2 ∉ Z[√φ], so the centering is a fixed float,
//! not a lattice-exact element — documented, and view-independent
//! because constant). Depth is view-anchored with the anchor at the
//! PATCH, not at an absolute cell size: with E the cell-space view
//! diagonal and the patch bbox diagonal exactly φ (= √(1 + s²)),
//! d = max(D_PRESENT, zoom) where zoom = 0 for E ≥ φ else
//! ceil(−ln(E/φ) / ln φ), capped at MAX_DEPTH = 17 (2^17 ≤ 250 000 <
//! 2^18). The zoom term is the pinwheel convention re-based on the
//! patch: each φ-shrink of the view adds one refinement level, so
//! nested views within one octave emit bit-identical tiles on their
//! intersection (`patch_nested_view_bit_identical`). The floor exists
//! because this is a BOUNDED patch — d = 0 renders the entire tiling
//! as one tile (the lone seed chair reported as a broken preview), and
//! the UI preview window is derived from the generated tiles
//! themselves (`tess_core::app::preview_region`: probe → 9× mean tile
//! side), an anchor-self-canceling feedback loop that pins an absolute
//! rule at d = 0 for every scale. D_PRESENT = 5 makes the probe-derived
//! default view frame the whole patch (≈ 74% of the pane height, all
//! 2^5 = 32 chairs, six size classes) self-consistently at any scale
//! and aspect.
//!
//! **T-junction structure (non-edge-to-edge, intrinsic).** A child's
//! reflex (notch) corner lands in the relative interior of its
//! sibling's long edge at exactly the s⁻⁴ fraction of that edge from
//! one end (s⁻² from the other, since 1 − s⁻⁴ = s⁻²). These
//! T-junctions are intrinsic to the A2 tiling, not an artifact of the
//! shipped patch: in the reference tiling, the junction events sit at
//! ratio 0.381966… = s⁻⁴ of the host edge. The shipped patch has
//! exactly 13 at depth 3 and 64 at depth 5 — pinned in
//! `junctions_at_exact_s_ratio`.
//!
//! **Pattern seed = size class.** Each tile's seed is the exponent e
//! of its width s^−e, so the default Pattern coloring paints each
//! size class one hue (seed % 8).

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, ROTATION_ID, SCALE_ID};

// ---------------------------------------------------------------------------
// Exact arithmetic in K = Q(√φ)
// ---------------------------------------------------------------------------

/// s = √φ, the field generator as f64 (real embedding at emission and
/// in tests only — the exact field element is `S` below).
const S_FLT: f64 = 1.272019649514069;
/// ln φ = ln s², for the depth anchor.
const LN_PHI: f64 = 0.48121182505960347;
/// The patch bbox diagonal in cell space: bbox [−0.5, 0.5] ×
/// [−s/2, s/2] has diagonal √(1 + s²) = √(1 + φ) = φ exactly.
const PATCH_DIAG: f64 = 1.618033988749895;
/// Presentation depth floor: the depth rendered whenever the whole
/// patch fits the view (E ≥ φ). A bounded patch at d = 0 IS one tile —
/// the lone seed chair — so whole-patch views must show the
/// substitution structure instead. 5 is fitted to the UI preview
/// window (probe → 9× mean tile side, `tess_core::app`): mean AABB
/// side at depth 5 is ≈ 0.19 cell, so the derived window spans
/// ≈ 1.72 cell and frames the 1.27-cell patch at ≈ 74% of the pane
/// height, all 32 chairs visible, at any scale and aspect. Zoomed-in
/// views (E < φ) refine past the floor by φ-octaves as before.
const D_PRESENT: u32 = 5;
/// Max refinement depth: 2^17 = 131 072 ≤ MAX_TILES < 2^18.
const MAX_DEPTH: u32 = 17;

/// An element of K = Q(√φ): (a0, a1, a2, a3) ⇔
/// a0 + a1·s + a2·s² + a3·s³, with s² = φ and s⁴ = s² + 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct K(i64, i64, i64, i64);

impl K {
    const fn add(self, o: Self) -> Self {
        K(self.0 + o.0, self.1 + o.1, self.2 + o.2, self.3 + o.3)
    }
    #[cfg(test)]
    const fn sub(self, o: Self) -> Self {
        K(self.0 - o.0, self.1 - o.1, self.2 - o.2, self.3 - o.3)
    }
    #[cfg(test)]
    const fn neg(self) -> Self {
        K(-self.0, -self.1, -self.2, -self.3)
    }
    /// x·s, reducing s⁴ = s² + 1.
    #[cfg(test)]
    const fn mul_s(self) -> Self {
        K(self.3, self.0, self.1 + self.3, self.2)
    }
    /// x/s, using s⁻¹ = s³ − s.
    const fn div_s(self) -> Self {
        K(self.1, self.2 - self.0, self.3, self.0)
    }
    /// x·y in K, reducing powers ≥ s⁴ by s⁴ = s² + 1 (so s⁵ = s³ + s,
    /// s⁶ = 1 + 2s²).
    const fn mul(self, o: Self) -> Self {
        let (a0, a1, a2, a3) = (self.0, self.1, self.2, self.3);
        let (b0, b1, b2, b3) = (o.0, o.1, o.2, o.3);
        let c4 = a1 * b3 + a2 * b2 + a3 * b1; // s⁴ slot
        let c5 = a2 * b3 + a3 * b2; // s⁵ slot
        let c6 = a3 * b3; // s⁶ slot
        K(
            a0 * b0 + c4 + c6,
            a0 * b1 + a1 * b0 + c5,
            a0 * b2 + a1 * b1 + a2 * b0 + c4 + 2 * c6,
            a0 * b3 + a1 * b2 + a2 * b1 + a3 * b0 + c5,
        )
    }
    /// Real embedding in the standard ordering of K ⊂ ℝ.
    fn val(self) -> f64 {
        let s = S_FLT;
        self.0 as f64 + self.1 as f64 * s + self.2 as f64 * s * s + self.3 as f64 * s * s * s
    }
}

const K0: K = K(0, 0, 0, 0);
const K1: K = K(1, 0, 0, 0);
const S: K = K(0, 1, 0, 0); // s
const S1I: K = K(0, -1, 0, 1); // s⁻¹ = s³ − s
const S2I: K = S1I.div_s(); // s⁻²
const S3I: K = S2I.div_s(); // s⁻³
const S4I: K = S3I.div_s(); // s⁻⁴

/// SINV_POWS[k] = s^−k for k = 0..=39 — every width power (e ≤
/// 2·MAX_DEPTH = 34) and area power the generator and tests need.
const SINV_POWS: [K; 40] = {
    let mut p = [K1; 40];
    let mut i = 1;
    while i < 40 {
        p[i] = p[i - 1].div_s();
        i += 1;
    }
    p
};

/// SPOWS[k] = s^(k−12) for k = 0..=16 (exponents −12..=+4): cross-check
/// of the div_s/mul_s chains against SINV_POWS and the reduction law.
#[cfg(test)]
const SPOWS: [K; 17] = {
    let mut p = [K0; 17];
    p[12] = K1;
    let mut i = 12;
    while i > 0 {
        p[i - 1] = p[i].div_s();
        i -= 1;
    }
    let mut i = 13;
    while i < 17 {
        p[i] = p[i - 1].mul_s();
        i += 1;
    }
    p
};

// ---------------------------------------------------------------------------
// The 8 chair classes and the substitution rule
// ---------------------------------------------------------------------------

/// The 8 orientation classes of the unit chair — CCW, bbox lower-left
/// at the origin, all vertices exact K. A family: rotations of A0 by
/// 90°·k; B family: the mirror of A0, then the same rotations. A0/B0/B2
/// are portrait (bbox 1 × s), A1/A3/B1/B3 landscape (s × 1).
const SHAPES: [[(K, K); 6]; 8] = [
    // A0: (0,0) (s⁻²,0) (s⁻²,s⁻³) (1,s⁻³) (1,s) (0,s)
    [(K0, K0), (S2I, K0), (S2I, S3I), (K1, S3I), (K1, S), (K0, S)],
    // A1: (s⁻³,0) (s,0) (s,1) (0,1) (0,s⁻⁴) (s⁻³,s⁻⁴)
    [(S3I, K0), (S, K0), (S, K1), (K0, K1), (K0, S4I), (S3I, S4I)],
    // A2: (0,0) (1,0) (1,s) (s⁻⁴,s) (s⁻⁴,s⁻¹) (0,s⁻¹)
    [(K0, K0), (K1, K0), (K1, S), (S4I, S), (S4I, S1I), (K0, S1I)],
    // A3: (0,0) (s,0) (s,s⁻²) (s⁻¹,s⁻²) (s⁻¹,1) (0,1)
    [(K0, K0), (S, K0), (S, S2I), (S1I, S2I), (S1I, K1), (K0, K1)],
    // B0: (s⁻⁴,0) (1,0) (1,s) (0,s) (0,s⁻³) (s⁻⁴,s⁻³)
    [(S4I, K0), (K1, K0), (K1, S), (K0, S), (K0, S3I), (S4I, S3I)],
    // B1: (0,0) (s,0) (s,1) (s⁻³,1) (s⁻³,s⁻²) (0,s⁻²)
    [(K0, K0), (S, K0), (S, K1), (S3I, K1), (S3I, S2I), (K0, S2I)],
    // B2: (0,0) (1,0) (1,s⁻¹) (s⁻²,s⁻¹) (s⁻²,s) (0,s)
    [(K0, K0), (K1, K0), (K1, S1I), (S2I, S1I), (S2I, S), (K0, S)],
    // B3: (0,0) (s⁻¹,0) (s⁻¹,s⁻⁴) (s,s⁻⁴) (s,1) (0,1)
    [(K0, K0), (S1I, K0), (S1I, S4I), (S, S4I), (S, K1), (K0, K1)],
];

/// The substitution (Vince Fig. 4), one entry per parent class:
/// (L child class, L offset in the parent unit frame, S child class,
/// S offset). The L child is scaled by s⁻¹ (Δe = +1), the S child by
/// s⁻² (Δe = +2); offsets are the child bbox lower-left, exact K.
const RULES: [(u8, K, K, u8, K, K); 8] = [
    // A0 -> A1 (s⁻¹) @ (0, s⁻³)   + B2 (s⁻²) @ (0, 0)
    (1, K0, S3I, 6, K0, K0),
    // A1 -> A2 (s⁻¹) @ (s⁻³, 0)  + B3 (s⁻²) @ (0, s⁻⁴)
    (2, S3I, K0, 7, K0, S4I),
    // A2 -> A3 (s⁻¹) @ (0, 0)    + B0 (s⁻²) @ (s⁻⁴, s⁻³)
    (3, K0, K0, 4, S4I, S3I),
    // A3 -> A0 (s⁻¹) @ (0, 0)    + B1 (s⁻²) @ (s⁻³, 0)
    (0, K0, K0, 5, S3I, K0),
    // B0 -> B3 (s⁻¹) @ (0, s⁻³)  + A2 (s⁻²) @ (s⁻⁴, 0)
    (7, K0, S3I, 2, S4I, K0),
    // B1 -> B0 (s⁻¹) @ (s⁻³, 0)  + A3 (s⁻²) @ (0, 0)
    (4, S3I, K0, 3, K0, K0),
    // B2 -> B1 (s⁻¹) @ (0, 0)    + A0 (s⁻²) @ (0, s⁻³)
    (5, K0, K0, 0, K0, S3I),
    // B3 -> B2 (s⁻¹) @ (0, 0)    + A1 (s⁻²) @ (s⁻³, s⁻⁴)
    (6, K0, K0, 1, S3I, S4I),
];

/// One chair in the exact patch: orientation class `cls`, width
/// s^−`e`, bbox lower-left `(ox, oy)` in the seed unit frame (exact K).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Chair {
    cls: u8,
    e: u8,
    ox: K,
    oy: K,
}

/// One substitution step: each chair → L child (scale s⁻¹, Δe = +1)
/// and S child (scale s⁻², Δe = +2) at the exact K anchors. Children
/// lie inside their parent, so the set grows by exactly ×2.
fn refine(tiles: &[Chair]) -> Vec<Chair> {
    let mut out = Vec::with_capacity(tiles.len() * 2);
    for c in tiles {
        let (l_cls, lox, loy, s_cls, sox, soy) = RULES[c.cls as usize];
        let w = SINV_POWS[c.e as usize];
        out.push(Chair {
            cls: l_cls,
            e: c.e + 1,
            ox: c.ox.add(w.mul(lox)),
            oy: c.oy.add(w.mul(loy)),
        });
        out.push(Chair {
            cls: s_cls,
            e: c.e + 2,
            ox: c.ox.add(w.mul(sox)),
            oy: c.oy.add(w.mul(soy)),
        });
    }
    out
}

// ---------------------------------------------------------------------------
// The tiling
// ---------------------------------------------------------------------------

/// Golden bee (Ammann A2 chair) — see the module docs.
/// Generator parameters: the shared similarity map, but with the scale
/// default fitted to the finite patch. The patch spans only ~1.3 cell
/// units (bbox [−0.5, 0.5] × [−s/2, s/2]); under the shared 1.0 default
/// every UI view derived it as a tiny centered chair (and the
/// probe-derived preview window pins the depth at 0 regardless of
/// scale — see D_PRESENT). Scale 3.5 frames the whole patch in the
/// standard 20/scale-tall pane at ~78% height (patch 4.45 of 5.71
/// world units); the depth floor D_PRESENT = 5 then shows the 32-chair
/// substitution structure at every scale. The scale slider (0.1–50)
/// still zooms; zoomed-in views refine past the floor by φ-octaves.
const PARAMS: [ParamSpec; 2] = [
ParamSpec::float(SCALE_ID, "scale", 0.1, 50.0, 0.05, 3.5),
ParamSpec::angle(ROTATION_ID, "rotation (deg)", 0.0, 360.0, 5.0, 0.0),
];

pub struct GoldenBeeTiling;

impl Tiling for GoldenBeeTiling {
    fn id(&self) -> &'static str {
        "golden-bee"
    }

    fn name(&self) -> &'static str {
        "Golden bee (Ammann A2 chair)"
    }

    fn description(&self) -> &'static str {
        "Golden bee / Ammann chair (A2): the irreptile — a concave hexagon \
         (five right angles, one reflex) dissecting into two similar copies \
         of unequal size (scales φ^−1/2 and φ^−1, s = √φ); √φ-inflation \
         substitution in exact Q(√φ), 8 orientation classes, T-junctions \
         intrinsic at the s^−4 edge ratio; finite covered patch, the seed \
         chair itself (Vince 2014; Durand–Shen–Vereshchagin 2020)"
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
            "quasiperiodic (Vince 2014; Durand–Shen–Vereshchagin 2020); \
             finite covered patch — the seed chair is the domain",
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
        // the seed chair [0,1]×[0,s] shifted by (−1/2, −s/2).
        let s = S_FLT;
        let patch = BBox::new(
            Vec2::new(-0.5, -s * 0.5),
            Vec2::new(0.5, s * 0.5),
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

        // View-anchored depth, anchored at the PATCH: zoom octaves are
        // measured relative to the patch diagonal φ (each φ-shrink of
        // the cell-space view diagonal adds one level), and whole-patch
        // views floor at D_PRESENT — d = 0 would render the entire
        // bounded tiling as the single seed chair, and the UI preview
        // window (probe → 9× mean tile side) is anchor-self-canceling,
        // so an absolute E ≥ 1 rule pins every UI view at d = 0.
        let ext = (region.max - region.min).length();
        let zoom = if ext < PATCH_DIAG {
            ((-(ext / PATCH_DIAG).ln()) / LN_PHI).ceil() as u32
        } else {
            0
        };
        let d = zoom.clamp(D_PRESENT, MAX_DEPTH);

        let mut chairs: Vec<Chair> = vec![Chair {
            cls: 0,
            e: 0,
            ox: K0,
            oy: K0,
        }];
        for _ in 0..d {
            chairs = refine(&chairs);
        }

        // Emission: exact K → f64 embedding + the fixed recentering,
        // culled to the query region.
        let rx = -0.5f64;
        let ry = -s * 0.5;
        let mut tiles = Vec::new();
        for c in &chairs {
            let w = SINV_POWS[c.e as usize];
            let pts: Vec<Vec2> = SHAPES[c.cls as usize]
                .iter()
                .map(|&(vx, vy)| {
                    Vec2::new(
                        w.mul(vx).add(c.ox).val() + rx,
                        w.mul(vy).add(c.oy).val() + ry,
                    )
                })
                .collect();
            let poly = Polygon::new(pts).transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto: c.cls,
                    seed: c.e as u32, level: c.e,
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

    // --- exact-K ordering helpers (real embedding, gap-asserted) -------
    // K has no intrinsic order; distinct elements at the coefficient
    // sizes used here separate by > 1e-9 in ℝ, so the float decision is
    // safe once equality is handled exactly.

    fn kf(a: K) -> f64 {
        a.val()
    }

    fn klo(a: K, b: K) -> bool {
        assert_ne!(a, b, "klo called on equal elements");
        let d = kf(a) - kf(b);
        assert!(d.abs() > 1e-9, "ambiguous K order: {a:?} vs {b:?} (Δ={d})");
        d < 0.0
    }

    /// Strict less-than that returns false (instead of panicking) when
    /// a == b — for strict-containment checks where two tiles may share
    /// an endpoint on the same line.
    fn klt(a: K, b: K) -> bool {
        if a == b {
            return false;
        }
        let d = kf(a) - kf(b);
        assert!(
            d.abs() > 1e-9,
            "ambiguous K order: {a:?} vs {b:?} (Δ={d})"
        );
        d < 0.0
    }

    fn kle(a: K, b: K) -> bool {
        a == b || klo(a, b)
    }

    fn kmin(a: K, b: K) -> K {
        if a == b || kf(a) <= kf(b) {
            a
        } else {
            b
        }
    }

    fn kmax(a: K, b: K) -> K {
        if a == b || kf(a) >= kf(b) {
            a
        } else {
            b
        }
    }

    /// Independent f64 value of s (cross-checks for the K tables).
    fn s_float() -> f64 {
        ((1.0 + 5.0_f64.sqrt()) / 2.0).sqrt()
    }

    /// Vertical cross-section of an axis-aligned polygon at the open
    /// slab (x0, x1): even-odd rule over the horizontal edges spanning
    /// the slab (x0, x1) contains no vertex x, so "spanning" =
    /// lo ≤ x0 and hi ≥ x1. Returns the sorted interval list.
    fn cross_sections(pts: &[(K, K)], slab: (K, K)) -> Vec<(K, K)> {
        let mut ys: Vec<K> = Vec::new();
        let n = pts.len();
        for i in 0..n {
            let (x1, y1) = pts[i];
            let (x2, y2) = pts[(i + 1) % n];
            if y1 == y2 {
                let (lo, hi) = if kle(x1, x2) { (x1, x2) } else { (x2, x1) };
                if kle(lo, slab.0) && kle(slab.1, hi) {
                    ys.push(y1);
                }
            }
        }
        assert_eq!(ys.len() % 2, 0, "odd y-slice count: {ys:?}");
        ys.sort_by(|a, b| a.val().total_cmp(&b.val()));
        (0..ys.len())
            .step_by(2)
            .map(|i| (ys[i], ys[i + 1]))
            .collect()
    }

    /// Merge touching/overlapping intervals (input sorted by lo).
    fn merge_ivs(lst: &[(K, K)]) -> Vec<(K, K)> {
        let mut out: Vec<(K, K)> = Vec::new();
        for &(lo, hi) in lst {
            if let Some(last) = out.last_mut() {
                if kle(lo, last.1) {
                    last.1 = kmax(last.1, hi);
                }
            } else {
                out.push((lo, hi));
            }
        }
        out
    }

    /// Axis-aligned boundary segments (axis 0 = H: line = y, axis 1 = V:
    /// line = x), as (axis, line, lo, hi).
    fn edges_of(pts: &[(K, K)]) -> Vec<(u8, K, K, K)> {
        let n = pts.len();
        (0..n)
            .map(|i| {
                let (x1, y1) = pts[i];
                let (x2, y2) = pts[(i + 1) % n];
                if y1 == y2 {
                    (0u8, y1, kmin(x1, x2), kmax(x1, x2))
                } else {
                    (1u8, x1, kmin(y1, y2), kmax(y1, y2))
                }
            })
            .collect()
    }

    /// (a) Per slab, the tile-edge coverage depth on each line is 0
    /// (interior), 1 (exactly on the parent boundary), or 2 (internal
    /// shared edge) — never 3+; and (b) every tile vertex sitting
    /// strictly inside another tile's segment on the same line does so
    /// at exactly the s⁻⁴ (or s⁻²) fraction of that segment. Returns
    /// the T-junction count.
    fn line_structure(parent: &[(K, K)], polys: &[Vec<(K, K)>]) -> usize {
        let pseg = edges_of(parent);
        // One slab line: (axis, line coordinate, segments (lo, hi, tile index)).
        type SlabLine = (u8, K, Vec<(K, K, usize)>);
        let mut by_line: Vec<SlabLine> = Vec::new();
        for (ti, poly) in polys.iter().enumerate() {
            for &(axis, line, lo, hi) in edges_of(poly).iter() {
                match by_line.iter_mut().find(|(a, l, _)| *a == axis && *l == line) {
                    Some(e) => e.2.push((lo, hi, ti)),
                    None => by_line.push((axis, line, vec![(lo, hi, ti)])),
                }
            }
        }
        let mut n_junctions = 0usize;
        for (axis, line, lst) in by_line {
            // (a) coverage depth per slab
            let mut ev: Vec<K> = Vec::new();
            for &(lo, hi, _) in &lst {
                ev.push(lo);
                ev.push(hi);
            }
            ev.sort_by(|a, b| a.val().total_cmp(&b.val()));
            ev.dedup();
            let pseg_here: Vec<(K, K)> = pseg
                .iter()
                .filter(|&&(a, l, _, _)| a == axis && l == line)
                .map(|&(_, _, lo, hi)| (lo, hi))
                .collect();
            for i in 0..ev.len() - 1 {
                let (l, r) = (ev[i], ev[i + 1]);
                let cover = lst.iter().filter(|&&(lo, hi, _)| kle(lo, l) && kle(r, hi)).count();
                let on_parent = pseg_here.iter().any(|&(plo, phi)| kle(plo, l) && kle(r, phi));
                assert!(
                    cover <= 2,
                    "line (axis {axis}, {line:?}): slab coverage {cover}"
                );
                if on_parent {
                    assert_eq!(cover, 1, "line (axis {axis}, {line:?}): boundary slab coverage {cover}");
                } else {
                    assert!(cover == 0 || cover == 2, "line (axis {axis}, {line:?}): interior slab coverage {cover}");
                }
            }
            // (b) junction positions
            for &(lo, hi, ti) in &lst {
                for &v in [lo, hi].iter() {
                    for &(olo, ohi, oti) in &lst {
                        if oti != ti && klt(olo, v) && klt(v, ohi) {
                            n_junctions += 1;
                            let l = ohi.sub(olo);
                            let dv = v.sub(olo);
                            assert!(
                                dv == S4I.mul(l) || dv == S2I.mul(l),
                                "line (axis {axis}, {line:?}): junction at {dv:?} of {l:?}"
                            );
                        }
                    }
                }
            }
        }
        n_junctions
    }

    #[test]
    fn eight_chair_shapes_are_exact() {
        // The f64 embedding and the const power tables cross-check
        // against an independent float computation.
        let s = s_float();
        assert!((S_FLT - s).abs() < 1e-15, "S_FLT embeds s");
        for (k, &inv) in SINV_POWS.iter().enumerate() {
            assert!(
                (kf(inv) - s.powi(-(k as i32))).abs() < 1e-12,
                "SINV_POWS[{k}] off: {} vs {}",
                kf(inv),
                s.powi(-(k as i32))
            );
        }
        // SPOWS cross-checks the mul_s chain against the div_s chain.
        for k in 0..=12u32 {
            assert_eq!(
                SPOWS[k as usize],
                SINV_POWS[(12 - k) as usize],
                "SPOWS[{k}] != SINV_POWS[{}]",
                12 - k
            );
        }
        assert_eq!(SPOWS[13], S, "SPOWS[13] = s");
        assert_eq!(SPOWS[14], K(0, 0, 1, 0), "SPOWS[14] = s²");
        assert_eq!(SPOWS[15], K(0, 0, 0, 1), "SPOWS[15] = s³");
        assert_eq!(SPOWS[16], K(1, 0, 1, 0), "SPOWS[16] = s⁴ = s² + 1");
        for cls in 0..8u8 {
            let pts = SHAPES[cls as usize];
            // 6 distinct vertices.
            for i in 0..6 {
                for j in (i + 1)..6 {
                    assert_ne!(pts[i], pts[j], "shape {cls}: repeated vertex");
                }
            }
            // Axis-aligned edges; exact edge-length multiset
            // {s⁻⁴, s⁻³, s⁻², s⁻¹, 1, s}.
            let mut lens: Vec<K> = Vec::new();
            for i in 0..6 {
                let (ax, ay) = pts[i];
                let (bx, by) = pts[(i + 1) % 6];
                assert!(ax == bx || ay == by, "shape {cls}: non-axis-aligned edge");
                let raw = if ax != bx {
                    bx.sub(ax)
                } else {
                    by.sub(ay)
                };
                assert_ne!(raw, K0, "shape {cls}: zero-length edge");
                lens.push(if raw.val() < 0.0 { raw.neg() } else { raw });
            }
            let mut want: Vec<K> = vec![S4I, S3I, S2I, S1I, K1, S];
            lens.sort_by(|a, b| a.val().total_cmp(&b.val()));
            want.sort_by(|a, b| a.val().total_cmp(&b.val()));
            assert_eq!(lens, want, "shape {cls}: edge multiset");
            // CCW with exact area: shoelace sum = 2·(s − s⁻⁷).
            let mut a2 = K0;
            for i in 0..6 {
                let (x1, y1) = pts[i];
                let (x2, y2) = pts[(i + 1) % 6];
                a2 = a2.add(x1.mul(y2)).sub(x2.mul(y1));
            }
            assert_eq!(a2, K(0, -8, 0, 6), "shape {cls}: 2·area = (0,−8,0,6)");
            // Exactly 5 right turns + 1 reflex turn (CCW).
            let mut right = 0usize;
            let mut reflex = 0usize;
            for i in 0..6 {
                let (px, py) = pts[(i + 5) % 6];
                let (vx, vy) = pts[i];
                let (nx, ny) = pts[(i + 1) % 6];
                let turn = (vx.sub(px)).mul(ny.sub(vy)).sub((vy.sub(py)).mul(nx.sub(vx)));
                assert!(turn.val().abs() > 1e-9, "shape {cls}: zero turn");
                if turn.val() > 0.0 {
                    right += 1;
                } else {
                    reflex += 1;
                }
            }
            assert_eq!((right, reflex), (5, 1), "shape {cls}: turn census");
            // Bbox lower-left at the origin; size 1×s or s×1.
            let mut mnx = pts[0].0;
            let mut mny = pts[0].1;
            for &p in pts.iter() {
                mnx = kmin(mnx, p.0);
                mny = kmin(mny, p.1);
            }
            let mut mxx = pts[0].0;
            let mut mxy = pts[0].1;
            for &p in pts.iter() {
                mxx = kmax(mxx, p.0);
                mxy = kmax(mxy, p.1);
            }
            assert_eq!(mnx, K0, "shape {cls}: bbox x0");
            assert_eq!(mny, K0, "shape {cls}: bbox y0");
            assert!(mxx == K1 || mxx == S, "shape {cls}: bbox width");
            assert!(mxy == K1 || mxy == S, "shape {cls}: bbox height");
        }
    }

    #[test]
    fn all_eight_dissections_exact_two_piece() {
        // Exact area law: the two child area fractions sum to 1.
        assert_eq!(S2I.add(S4I), K1, "area law s⁻² + s⁻⁴ = 1");
        // Field identities used by the junction-ratio checks.
        assert_eq!(K1.sub(S4I), S2I, "1 − s⁻⁴ = s⁻²");
        assert_eq!(K1.sub(S2I), S4I, "1 − s⁻² = s⁻⁴");
        for cls in 0..8u8 {
            let parent = SHAPES[cls as usize];
            let (l_cls, lox, loy, s_cls, sox, soy) = RULES[cls as usize];
            let lpts: Vec<(K, K)> = SHAPES[l_cls as usize]
                .iter()
                .map(|&(x, y)| (S1I.mul(x).add(lox), S1I.mul(y).add(loy)))
                .collect();
            let spts: Vec<(K, K)> = SHAPES[s_cls as usize]
                .iter()
                .map(|&(x, y)| (S2I.mul(x).add(sox), S2I.mul(y).add(soy)))
                .collect();
            // Slab sweep: the children's cross-sections merge exactly
            // to the parent's and never overlap.
            let mut xs: Vec<K> = Vec::new();
            for &(x, _) in parent.iter().chain(lpts.iter()).chain(spts.iter()) {
                xs.push(x);
            }
            xs.sort_by(|a, b| a.val().total_cmp(&b.val()));
            xs.dedup();
            for i in 0..xs.len() - 1 {
                let slab = (xs[i], xs[i + 1]);
                let cp = cross_sections(&parent, slab);
                let mut flat: Vec<(K, K)> = Vec::new();
                flat.extend(cross_sections(&lpts, slab));
                flat.extend(cross_sections(&spts, slab));
                flat.sort_by(|a, b| a.0.val().total_cmp(&b.0.val()));
                for w in flat.windows(2) {
                    assert!(
                        kle(w[0].1, w[1].0),
                        "parent {cls}: child overlap in slab {slab:?}"
                    );
                }
                assert_eq!(
                    merge_ivs(&flat),
                    cp,
                    "parent {cls}: children union != parent in slab {slab:?}"
                );
            }
            // Line structure: depth 0/1/2 per slab (1 iff on the
            // parent boundary), junctions at the exact s-ratio.
            line_structure(&parent, &[lpts.clone(), spts.clone()]);
        }
    }

    #[test]
    fn growth_counts_size_histogram_area() {
        // (depth: tile count, class census, size-class census) pinned from the
        // exact spike; the class parity is even depths {A0,A2,B1,B3},
        // odd {A1,A3,B0,B2} (depth 1: A1, B2), each at 2^(d−2).
        type DepthCensus<'a> = (usize, &'a [(usize, usize)], &'a [(usize, usize)]);
        let expected: &[DepthCensus] = &[
            (1, &[(0, 1)], &[(0, 1)]),
            (2, &[(1, 1), (6, 1)], &[(1, 1), (2, 1)]),
            (
                4,
                &[(0, 1), (2, 1), (5, 1), (7, 1)],
                &[(2, 1), (3, 2), (4, 1)],
            ),
            (
                8,
                &[(1, 2), (3, 2), (4, 2), (6, 2)],
                &[(3, 1), (4, 3), (5, 3), (6, 1)],
            ),
            (
                16,
                &[(0, 4), (2, 4), (5, 4), (7, 4)],
                &[(4, 1), (5, 4), (6, 6), (7, 4), (8, 1)],
            ),
            (
                32,
                &[(1, 8), (3, 8), (4, 8), (6, 8)],
                &[(5, 1), (6, 5), (7, 10), (8, 10), (9, 5), (10, 1)],
            ),
        ];
        let mut tiles: Vec<Chair> = vec![Chair {
            cls: 0,
            e: 0,
            ox: K0,
            oy: K0,
        }];
        for (d, &(n_exp, cls_exp, e_exp)) in expected.iter().enumerate() {
            assert_eq!(tiles.len(), n_exp, "depth {d}: tile count");
            let mut cnt = [0usize; 8];
            for c in &tiles {
                cnt[c.cls as usize] += 1;
            }
            let mut want_cls = [0usize; 8];
            for &(k, v) in cls_exp {
                want_cls[k] = v;
            }
            assert_eq!(cnt, want_cls, "depth {d}: class census");
            let mut want_e: Vec<usize> = vec![0; 33];
            for &(k, v) in e_exp {
                want_e[k] = v;
            }
            let got_e: Vec<usize> = (0..33)
                .map(|e| tiles.iter().filter(|c| c.e as usize == e).count())
                .collect();
            assert_eq!(got_e, want_e, "depth {d}: size histogram");
            // Exact area law: Σ s^−2e = 1 (unit-chair area factor).
            let mut area = K0;
            for c in &tiles {
                area = area.add(SINV_POWS[2 * c.e as usize]);
            }
            assert_eq!(area, K1, "depth {d}: area law");
            tiles = refine(&tiles);
        }
        assert_eq!(tiles.len(), 64, "depth 6: tile count");
    }

    #[test]
    fn junctions_at_exact_s_ratio() {
        // The global patch structure at depth 3 and 5: slab coverage
        // depth ∈ {0,1,2} (1 iff on the patch boundary), every
        // T-junction at the exact s⁻⁴ / s⁻² fraction of its host edge,
        // and the pinned junction censuses 13 (depth 3) / 64 (depth 5).
        for (d, n_expected) in [(3u32, 13usize), (5, 64)] {
            let mut tiles: Vec<Chair> = vec![Chair {
                cls: 0,
                e: 0,
                ox: K0,
                oy: K0,
            }];
            for _ in 0..d {
                tiles = refine(&tiles);
            }
            let polys: Vec<Vec<(K, K)>> = tiles
                .iter()
                .map(|c| {
                    let w = SINV_POWS[c.e as usize];
                    SHAPES[c.cls as usize]
                        .iter()
                        .map(|&(x, y)| (w.mul(x).add(c.ox), w.mul(y).add(c.oy)))
                        .collect()
                })
                .collect();
            let n = line_structure(&SHAPES[0], &polys);
            assert_eq!(n, n_expected, "depth {d}: T-junction census");
        }
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
        let t = GoldenBeeTiling;
        // Mechanics pinned at scale 1.0, where the window geometry below
        // was derived; the DEFAULT scale is 3.5 (patch-fitted — see
        // default_view_fills_pane).
        let mut p = ParamSet::default_for(t.params());
        p.set(SCALE_ID, tess_core::params::ParamValue::Float(1.0));
        let r = t.generate(&p, &Query { region });
        assert!(r.message.is_none(), "unexpected message");
        assert!(!r.tiles.is_empty(), "empty view");
        r.tiles.iter().map(tile_key).collect()
    }

    #[test]
    fn patch_nested_view_bit_identical() {
        // All five windows sit in the φ-octave [φ⁻⁴, φ⁻³) of
        // patch-relative view diagonals (E/φ ≈ 0.19 / 0.19 / 0.19 /
        // 0.16 / 0.16), so all pick the same depth (d = 5 = D_PRESENT,
        // pinned by the size classes e ∈ 5..=10 below) and refine the
        // same patch: every tile of a smaller view must reappear
        // bit-identically in each larger one containing it.
        let base = gen_keys(BBox::new(Vec2::new(-0.09, -0.12), Vec2::new(0.09, 0.12)));
        let grown =
            gen_keys(BBox::new(Vec2::new(-0.0945, -0.126), Vec2::new(0.0945, 0.126)));
        let panned =
            gen_keys(BBox::new(Vec2::new(-0.07, -0.09), Vec2::new(0.11, 0.15)));
        let overlap =
            gen_keys(BBox::new(Vec2::new(-0.07, -0.09), Vec2::new(0.09, 0.12)));
        let zoomed =
            gen_keys(BBox::new(Vec2::new(-0.0765, -0.102), Vec2::new(0.0765, 0.102)));
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

    // --- standard trio (explicit scale-1.0-style windows; check_covers /
    // check_det build default params, whose scale default is now the
    // patch-fitted 3.5 — the windows below still land inside the chair
    // domain at that scale, just deeper (the depth rule follows the
    // view). The default presentation itself is pinned by
    // default_view_fills_pane.)

    const WIN: [f64; 4] = [-0.25, -0.1, 0.05, 0.15];
    const WIN_ROT: [f64; 4] = [-0.04, -0.04, 0.04, 0.04];

    #[test]
    fn golden_bee_covers_region() {
        check_covers(
            &GoldenBeeTiling,
            "golden-bee",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }

    #[test]
    fn golden_bee_rotated_scaled() {
        // Inverse cell disk radius ≈ 0.0943 < 0.1909 (min distance from
        // the recentered origin to the chair boundary).
        check_rotated(
            &GoldenBeeTiling,
            "golden-bee(rot25,scale0.6)",
            BBox::new(Vec2::new(WIN_ROT[0], WIN_ROT[1]), Vec2::new(WIN_ROT[2], WIN_ROT[3])),
            0.6,
            25.0,
        );
    }

    #[test]
    fn golden_bee_deterministic() {
        check_det(
            &GoldenBeeTiling,
            "golden-bee",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }

    #[test]
    fn default_view_fills_pane() {
        // Pins BOTH default presentations against the lone-seed-chair
        // bug (the patch at d = 0 IS the whole tiling — one gray chair):
        //
        // (a) The UI/preview convention (TUI render pane, GUI canvas,
        //     card thumbnails): tess_core::app derives the window from
        //     the tiling itself — a bounded ±10 probe, then
        //     preview_window(9 × mean tile side) — an anchor-self-
        //     canceling loop that must land on the D_PRESENT floor and
        //     frame the whole patch (~74% of the pane height, 32
        //     chairs) self-consistently at the default scale.
        // (b) The CLI convention: a ~20/scale-unit-tall origin-centered
        //     pane; the patch-fitted scale default (3.5) frames the
        //     whole patch at ~78% height.
        let t = GoldenBeeTiling;
        let p = ParamSet::default_for(t.params());
        assert!(
            (p.float(SCALE_ID, 1.0) - 3.5).abs() < 1e-12,
            "scale default must stay patch-fitted (3.5)"
        );
        let s = S_FLT;
        let patch_h = s * p.float(SCALE_ID, 1.0); // patch bbox height, world
        let patch_w = p.float(SCALE_ID, 1.0);

        // --- (a) App/preview convention --------------------------------
        let probe = t.generate(&p, &Query {
            region: BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)),
        });
        assert!(probe.message.is_none());
        let mw = tess_core::app::mean_tile_side(&probe.tiles);
        assert!(mw > 0.0, "probe must return tiles");
        let win = tess_core::app::preview_window(probe.periodic, mw, 4.0 / 3.0);
        let r = t.generate(&p, &Query { region: win });
        assert!(r.message.is_none());
        // The floor renders the whole patch: exactly 2^5 chairs (the
        // view ⊇ patch at every aspect for the derived window).
        assert_eq!(r.tiles.len(), 32, "preview must show the depth-5 patch");
        let classes: std::collections::BTreeSet<u32> =
            r.tiles.iter().map(|tl| tl.seed).collect();
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
