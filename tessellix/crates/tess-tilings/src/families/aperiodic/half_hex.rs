//! Half-hex — Socolar's non-periodic substitution tiling (limit-periodic).
//!
//! Socolar's "half-hex" substitution (Bielefeld Tilings Encyclopedia entry
//! "half-hex"; limit-periodic, 1 prototile): the prototile is a
//! half-hexagon P1, an **achiral** quadrilateral whose six rotations
//! M^k(P1), k = 0..=5, are the complete orientation model (its mirror
//! image is a free translation of one of the rotations — there is no
//! reflection class). Each parent is inflated by λ = 2 and subdivided
//! into **four children**: relative to an upright parent, the children
//! are M^off(P1) + t_off for off ∈ {0, 2, 3, 4} with pinned offsets
//! (T_bot = M⁰(P1)+(1,0), T_R = M²(P1)+(4,0), T_top = M³(P1)+(3,1),
//! T_L = M⁴(P1)+(1,1)) — rotation classes 1 and 5 are absent.
//!
//! **Exact arithmetic.** All geometry lives in the √3 field: a cell
//! point (x, y) has real height √3·y, and the map
//! (u, v) → (x, y) = ((u+v)/2, (u−v)/2) sends integer pairs onto every
//! tile vertex. Every tile placement is therefore an exact integer pair
//! (u, v), the 60° rotation acts on (u, v) by R(u, v) = (v, −u+v)
//! (R⁶ = I, R³ = −I), and distances satisfy Δd² = Δu² − ΔuΔv + Δv².
//! f64 appears only at the view/emission boundary, where the real
//! height √3·y is materialized (√3 via `f64::sqrt` — no decimal
//! literal ever touches the exact geometry).
//!
//! **Substitution.** A depth-g patch is 4^g unit (u, v) tiles tiling
//! 2^g·B0 in the (u, v) frame. The substitution matrix S, S[j][i] = 1
//! iff (j−i) mod 6 ∈ {0, 2, 3, 4} (diagonal 1: every parent has one
//! same-orientation child), has
//! PF eigenvalue 4 = λ², characteristic polynomial
//! (x−4)(x−2)(x−1)²(x+1)² — eigenvalues {4, 2, 1, 1, −1, −1} —
//! pinned by a 720-permutation i128 expansion in the tests.
//!
//! **Generation (finite covered-patch model, Taylor–Socolar /
//! Jeandel–Rao precedent).** The seed is the unit half-hexagon at the
//! origin; depth d is view-anchored: with E the inverse-mapped view
//! diagonal and the d = 5 patch bbox diagonal exactly √4864,
//! zoom = 0 for E ≥ √4864 else ceil(log2(√4864/E)), d = clamp(zoom,
//! 5, 8) (4^8 = 65 536 ≤ MAX_TILES). Depth bands in E²: d = 5 for
//! E² ≥ 4.75, d = 6 for [1.1875, 4.75), d = 7 for [0.296875, 1.1875),
//! d = 8 below. Nested views within one band emit bit-identical tiles
//! on their intersection (`nested_view_bit_identical`).
//!
//! **Emission recentering.** The depth-d patch spans x ∈ [0, 2^{d+1}],
//! y ∈ [0, 2^{d−1}] in cell units; it is recentered on the origin by
//! the constant shift (Δx, Δy) = (2^d, 2^{d−2}) at emission, i.e.
//! (Δu, Δv) = (5·2^{d−2}, 3·2^{d−2}) in (u, v) — the anchor
//! (5/4, 3/4)·2^d is not a (u, v)-lattice point, so this is a
//! presentation recentering, not a lattice-exact one: the geometry,
//! the covered domain and the depth rule are all shift-invariant.
//! Beyond the recentered 2^d·half-hexagon the view is background.
//!
//! **Limit-periodicity (cited, not re-proven here).** The Bielefeld
//! entry cites the limit-periodic structure (Frettlöh's cut-and-project
//! construction — Fre02; Grünbaum & Shephard 1987, Tiling Theory,
//! Ex. 10.1.3 — GS87). Measured, not claimed: the minimal x-period of
//! the depth-g patch is (3/2)·2^g — 6, 12, 24, 48, 96 at g = 2..6,
//! strictly growing by 2× per level — pinned in `period_hierarchy`.
//! (The depth-0 atomic hexagonal tiling already carries the pure-x
//! period 3/2, which the naive 2^g growth law would miss.)

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

// ---------------------------------------------------------------------------
// Exact (u, v) model: bases, rule, refinement
// ---------------------------------------------------------------------------

/// √3 — the view/emission boundary constant (real height = √3·y).
/// Computed, never written as a decimal literal: exact geometry stays
/// in the √3 field, and only this boundary materializes it.
fn sq3() -> f64 {
    (3.0_f64).sqrt()
}

/// Diagonal of the depth-5 patch bbox in the cell frame: the d = 5
/// patch spans x ∈ [−32, 32], Y ∈ [−√3·8, √3·8] after recentering,
/// diagonal² = 64² + (16√3)² = 4864 (in general E²(d) = 19·4^{d−1}).
fn patch_diag() -> f64 {
    (4864.0_f64).sqrt()
}
/// Presentation depth floor: the depth rendered whenever the whole
/// patch fits the view (E ≥ √4864). d = 5 frames the UI preview
/// window self-consistently (see `default_view_fills_pane`).
const D_PRESENT: u32 = 5;
/// Max refinement depth: 4^8 = 65 536 ≤ MAX_TILES < 4^9.
const MAX_DEPTH: u32 = 8;

/// The six rotation bases B_k of the unit half-hexagon in (u, v):
/// B_k = R^k(B0), CCW in the (u, v) plane, shoelace2 = +3 each
/// (the (u, v) frame is orientation-reversing w.r.t. the cell frame,
/// so the bases are pinned CCW in (u, v) and all exact convex tests
/// run directly there — a linear bijection, no correction needed).
///
/// P2 (the depth-0 parent, 2·B_k) in (u, v) is
/// [(0,0),(4,0),(3,1),(1,1)] for k = 0 → vertex set
/// {(0,0),(4,4),(4,2),(2,0)} = 2·B0 (shoelace2 = +12).
const BASES: [[(i64, i64); 4]; 6] = [
    [(0, 0), (1, 0), (2, 1), (2, 2)],
    [(0, 0), (0, -1), (1, -1), (2, 0)],
    [(0, 0), (-1, -1), (-1, -2), (0, -2)],
    [(0, 0), (-1, 0), (-2, -1), (-2, -2)],
    [(0, 0), (0, 1), (-1, 1), (-2, 0)],
    [(0, 0), (1, 1), (1, 2), (0, 2)],
];

/// The pinned substitution rule (pre-conjugated): for parent class k,
/// each entry is (child_class, du, dv) with child position
/// (2·u_parent + du, 2·v_parent + dv). Child classes are
/// (k + off) mod 6 for off ∈ {0, 2, 3, 4} (classes 1 and 5 absent
/// relative to the parent); the offsets are the upright-parent child
/// offsets conjugated to class k. Cross-checked against the exact
/// growth battery and the substitution matrix in the tests.
const RULES: [[(u8, i64, i64); 4]; 6] = [
    [(0, 1, 1), (2, 4, 4), (3, 4, 2), (4, 2, 0)],
    [(1, 1, 0), (3, 4, 0), (4, 2, -2), (5, 0, -2)],
    [(2, 0, -1), (4, 0, -4), (5, -2, -4), (0, -2, -2)],
    [(3, -1, -1), (5, -4, -4), (0, -4, -2), (1, -2, 0)],
    [(4, -1, 0), (0, -4, 0), (1, -2, 2), (2, 0, 2)],
    [(5, 0, 1), (1, 0, 4), (2, 2, 4), (3, 2, 2)],
];

/// One tile of the depth-g patch: orientation class + (u, v) position
/// of the class's reference vertex, in exact integer (u, v) coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TileCell {
    cls: u8,
    u: i64,
    v: i64,
}

/// In-place refinement: each tile is replaced exactly by its four
/// children (children ⊆ parent), so refinement never culls.
fn refine(tiles: &[TileCell]) -> Vec<TileCell> {
    let mut out = Vec::with_capacity(tiles.len() * 4);
    for t in tiles {
        for &(cls, du, dv) in &RULES[t.cls as usize] {
            out.push(TileCell {
                cls,
                u: 2 * t.u + du,
                v: 2 * t.v + dv,
            });
        }
    }
    out
}

/// The depth-g patch: 4^g tiles tiling 2^g·B0 in the (u, v) frame.
fn patch(g: u32) -> Vec<TileCell> {
    let mut tiles = vec![TileCell { cls: 0, u: 0, v: 0 }];
    for _ in 0..g {
        tiles = refine(&tiles);
    }
    tiles
}

// ---------------------------------------------------------------------------
// Tiling
// ---------------------------------------------------------------------------

/// Socolar's half-hex non-periodic substitution tiling (limit-periodic).
pub struct HalfHexTiling;

impl Tiling for HalfHexTiling {
    fn id(&self) -> &'static str {
        "half-hex"
    }

    fn name(&self) -> &'static str {
        "Half-hex (Socolar, limit-periodic)"
    }

    fn description(&self) -> &'static str {
        "Socolar's half-hex non-periodic substitution tiling (Bielefeld entry \
         \"half-hex\"): one achiral half-hexagon prototile, six rotation \
         classes, λ = 2 substitution into 4 children (substitution \
         matrix PF eigenvalue 4 = λ², char poly (x−4)(x−2)(x−1)²(x+1)²). \
         Finite covered patch = 2^d-scaled half-hexagon (d = 5..8 by \
         view diagonal, recentered on the origin); exact √3-field \
         integer-pair (u,v) arithmetic end to end. Limit-periodic (long-range order): \
         minimal x-period of the depth-g patch is (3/2)·2^g — 6, 12, \
         24, 48, 96 at g = 2..6."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Aperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "non-periodic substitution tiling; limit-periodic (Socolar half-hex; Bielefeld entry \
             \"half-hex\"); Fre02 cut-and-project; GS87 Ex. 10.1.3; \
             finite covered patch = 2^d-scaled half-hexagon, \
             minimal x-period (3/2)·2^d",
        );
        let map = similarity_map(params);
        let inv = map.inverse();

        // Query box in the cell frame (x, Y = √3·y_cell): axis-aligned
        // hull of the inverse image of the four world corners.
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

        // View-anchored depth: each √2-shrink of the view diagonal past
        // the d = 5 patch diagonal √4864 adds one refinement level.
        let ext = (region.max - region.min).length();
        let pd = patch_diag();
        let zoom = if ext < pd {
            ((pd / ext).log2()).ceil() as u32
        } else {
            0
        };
        let d = zoom.clamp(D_PRESENT, MAX_DEPTH);
        assert!(
            (4u64).pow(d) as usize <= MAX_TILES,
            "depth {d} exceeds MAX_TILES"
        );

        // The covered domain after the constant recentering:
        // x ∈ [−2^d, 2^d], Y ∈ [−√3·2^{d−2}, √3·2^{d−2}].
        let px = (1u64 << d) as f64;
        let py = sq3() * (1u64 << (d - 2)) as f64;
        let patch_box = BBox::new(Vec2::new(-px, -py), Vec2::new(px, py));
        if !patch_box.intersects(region) {
            // Finite patch: outside the covered domain is background.
            return TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: None,
                message: None,
            };
        }

        let cells = patch(d);

        // Emission: exact (u, v) integers → (X, Y) = ((u+v)/2,
        // √3·(u−v)/2) in the recentered frame (Δx, Δy) =
        // (2^d, 2^{d−2}) ⟺ (Δu, Δv) = (5·2^{d−2}, 3·2^{d−2}),
        // culled to the query region.
        let sh_u = 5i64 << (d - 2);
        let sh_v = 3i64 << (d - 2);
        let mut tiles = Vec::new();
        for c in &cells {
            let b = &BASES[c.cls as usize];
            let pts: Vec<Vec2> = b
                .iter()
                .map(|&(bu, bv)| {
                    let uu = c.u + bu - sh_u;
                    let vv = c.v + bv - sh_v;
                    Vec2::new((uu + vv) as f64 * 0.5, (uu - vv) as f64 * 0.5 * sq3())
                })
                .collect();
            let poly = Polygon::new(pts).transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto: 0,
                    seed: c.cls as u32, level: d as u8,
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
    use crate::lattice::SCALE_ID;
    use std::collections::{BTreeSet, HashMap, HashSet};
    use tess_core::params::ParamValue;

    // --- exact (u, v) convex helpers ----------------------------------

    /// Doubled signed area (shoelace) of a CCW (u, v) polygon.
    fn shoelace2(p: &[(i64, i64)]) -> i64 {
        let mut s = 0i64;
        let n = p.len();
        for i in 0..n {
            let (u1, v1) = p[i];
            let (u2, v2) = p[(i + 1) % n];
            s += u1 * v2 - u2 * v1;
        }
        s
    }

    /// Closed half-plane containment: `p` inside the CCW polygon.
    fn ccw_contains(poly: &[(i64, i64)], u: i64, v: i64) -> bool {
        for i in 0..poly.len() {
            let (au, av) = poly[i];
            let (bu, bv) = poly[(i + 1) % poly.len()];
            if (bu - au) * (v - av) - (bv - av) * (u - au) < 0 {
                return false;
            }
        }
        true
    }

    /// A point (U, V) of the depth-g patch region R_g^uv =
    /// {V ≥ 0, U ≥ V, U−V ≤ 2^g, U ≤ 2^{g+1}} (in the (u, v) frame).
    fn in_rg_uv(u: i64, v: i64, g: u32) -> bool {
        let two_g = 1i64 << g;
        v >= 0 && u >= v && u - v <= two_g && u <= 2 * two_g
    }

    // --- exact Sutherland–Hodgman clip in (u, v), i128 rationals ------
    // The depth ≤ 8 overlap/rule-line tests clip integer convex
    // polygons whose coordinates stay within a few dozen units: every
    // rational below is gcd-reduced at each construction step, and the
    // magnitudes never approach i128.

    /// A reduced rational n/d (d > 0).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Rat {
        n: i128,
        d: i128,
    }

    fn rat_gcd(a: i128, b: i128) -> i128 {
        let (mut a, mut b) = (a.abs(), b.abs());
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a
    }

    impl Rat {
        fn new(n: i128, d: i128) -> Self {
            debug_assert!(d != 0, "zero rational denominator");
            let (n, d) = if d < 0 { (-n, -d) } else { (n, d) };
            let g = rat_gcd(n, d).max(1);
            Self { n: n / g, d: d / g }
        }
        fn add(self, o: &Rat) -> Self {
            Self::new(self.n * o.d + o.n * self.d, self.d * o.d)
        }
        fn sub(self, o: &Rat) -> Self {
            self.add(&Rat { n: -o.n, d: o.d })
        }
    }

    /// Half-plane value w·(q.v − av) − h·(q.u − au) for the CCW clipper
    /// edge a → b with (w, h) = b − a and (au, av) = a.
    fn edge_val(w: i128, h: i128, au: i128, av: i128, q: &(Rat, Rat)) -> Rat {
        let tv = q.1.sub(&Rat::new(av, 1));
        let tu = q.0.sub(&Rat::new(au, 1));
        Rat::new(w * tv.n, tv.d).sub(&Rat::new(h * tu.n, tu.d))
    }

    /// Clip `subject` (CCW integer polygon) against all half-planes of
    /// `clipper` (CCW integer polygon) — Sutherland–Hodgman with
    /// exact rational intersections.
    fn sh_clip(subject: &[(i64, i64)], clipper: &[(i64, i64)]) -> Vec<(Rat, Rat)> {
        let mut poly: Vec<(Rat, Rat)> = subject
            .iter()
            .map(|&(u, v)| (Rat::new(u as i128, 1), Rat::new(v as i128, 1)))
            .collect();
        for i in 0..clipper.len() {
            if poly.is_empty() {
                break;
            }
            let (au, av) = clipper[i];
            let (bu, bv) = clipper[(i + 1) % clipper.len()];
            let w = i128::from(bu) - i128::from(au);
            let h = i128::from(bv) - i128::from(av);
            let au = i128::from(au);
            let av = i128::from(av);
            let mut out: Vec<(Rat, Rat)> = Vec::with_capacity(poly.len() + 1);
            for j in 0..poly.len() {
                let cur = poly[j];
                let prev = poly[(j + poly.len() - 1) % poly.len()];
                let hc = edge_val(w, h, au, av, &cur);
                let hp = edge_val(w, h, au, av, &prev);
                let ci = hc.n >= 0;
                let pi = hp.n >= 0;
                if ci {
                    out.push(cur);
                    if !pi {
                        out.push(clip_x(&prev, &cur, &hc, &hp));
                    }
                } else if pi {
                    out.push(clip_x(&prev, &cur, &hc, &hp));
                }
            }
            poly = out;
        }
        poly
    }

    /// Intersection of the segment prev → cur with the clip line; the
    /// half-plane values hc, hp differ in sign (t = −hp/(hc−hp) ∈
    /// [0, 1]).
    fn clip_x(prev: &(Rat, Rat), cur: &(Rat, Rat), hc: &Rat, hp: &Rat) -> (Rat, Rat) {
        let den = hc.sub(hp);
        let t = Rat::new(-hp.n, den.n);
        let dx = cur.0.sub(&prev.0);
        let dy = cur.1.sub(&prev.1);
        let ix = prev.0.add(&Rat::new(t.n * dx.n, t.d * dx.d));
        let iy = prev.1.add(&Rat::new(t.n * dy.n, t.d * dy.d));
        (ix, iy)
    }

    /// True iff the two CCW (u, v) polygons have positive-area overlap.
    fn inter_area_pos(a: &[(i64, i64)], b: &[(i64, i64)]) -> bool {
        let poly = sh_clip(a, b);
        if poly.len() < 3 {
            return false;
        }
        let mut acc = Rat::new(0, 1);
        let n = poly.len();
        for i in 0..n {
            let p = &poly[i];
            let q = &poly[(i + 1) % n];
            let p1 = Rat::new(p.0.n * q.1.n, p.0.d * q.1.d);
            let p2 = Rat::new(p.1.n * q.0.n, p.1.d * q.0.d);
            acc = acc.add(&p1.sub(&p2));
        }
        acc.n > 0
    }

    // --- rule line ------------------------------------------------------

    /// Rule line, all six orientations: the inflated parent 2·B_k is
    /// exactly tiled by its four children B_{cls} + offset — each child
    /// a unit half-hexagon (shoelace2 = 3), contained in the parent,
    /// pairwise zero-area overlap, areas summing to the parent (12).
    #[test]
    fn rule_line_all_six_orientations() {
        for k in 0..6usize {
            let parent: [(i64, i64); 4] = BASES[k].map(|p| (2 * p.0, 2 * p.1));
            let kids: Vec<[(i64, i64); 4]> = RULES[k]
                .iter()
                .map(|&(cls, du, dv)| BASES[cls as usize].map(|p| (p.0 + du, p.1 + dv)))
                .collect();
            for kid in &kids {
                assert_eq!(shoelace2(kid), 3, "k={k}: child not a unit tile");
            }
            assert_eq!(shoelace2(&parent), 12, "k={k}: parent area");
            for kid in &kids {
                for (u, v) in *kid {
                    assert!(
                        ccw_contains(&parent, u, v),
                        "k={k}: child vertex ({u}, {v}) outside the parent"
                    );
                }
            }
            for i in 0..4 {
                for j in (i + 1)..4 {
                    assert!(
                        !inter_area_pos(&kids[i], &kids[j]),
                        "k={k}: children {i} and {j} overlap"
                    );
                }
            }
            // area sum: 4 children × 3 = 12 = parent (the rule line
            // closes — containment + disjointness + equal-area ⇒ tiling)
        }
    }

    // --- substitution matrix + growth ----------------------------------

    /// S derived from RULES equals the pinned 6×6 matrix and its
    /// structure; S^g·e0 matches the battery growth vectors, and the
    /// refine() census agrees at every level.
    #[test]
    fn substitution_matrix_and_counts() {
        let mut s = [[0i64; 6]; 6];
        for k in 0..6usize {
            for &(cls, _, _) in &RULES[k] {
                s[cls as usize][k] += 1;
            }
        }
        assert_eq!(
            s,
            [
                [1, 0, 1, 1, 1, 0],
                [0, 1, 0, 1, 1, 1],
                [1, 0, 1, 0, 1, 1],
                [1, 1, 0, 1, 0, 1],
                [1, 1, 1, 0, 1, 0],
                [0, 1, 1, 1, 0, 1],
            ],
            "S[j][i] (child j, parent i)"
        );
        for i in 0..6usize {
            for j in 0..6usize {
                let m = (j + 6 - i) % 6;
                assert_eq!(
                    s[j][i],
                    matches!(m, 0 | 2 | 3 | 4) as i64,
                    "S[{j}][{i}] off the {{0,2,3,4}} diagonal band"
                );
            }
        }
        let mut c = [1i64, 0, 0, 0, 0, 0];
        let pins = [
            [1, 0, 1, 1, 1, 0],
            [4, 2, 3, 2, 3, 2],
            [12, 9, 12, 10, 12, 9],
            [46, 40, 45, 40, 45, 40],
        ];
        for g in 1..=4usize {
            c = std::array::from_fn(|j| s[j].iter().zip(c.iter()).map(|(&a, &b)| a * b).sum());
            assert_eq!(c, pins[g - 1], "S^g·e0");
            assert_eq!(c.iter().sum::<i64>(), 4i64.pow(g as u32), "total = 4^g");
            // refine() census cross-check
            let mut census = [0i64; 6];
            for t in &patch(g as u32) {
                census[t.cls as usize] += 1;
            }
            assert_eq!(census, pins[g - 1], "depth-{g} refine() census");
        }
    }

    /// PF eigenvalue 4 = λ² (60-iteration power iteration, battery-
    /// faithful) and the characteristic polynomial
    /// x⁶ − 6x⁵ + 6x⁴ + 12x³ − 15x² − 6x + 8 =
    /// (x−4)(x−2)(x−1)²(x+1)² — the exact 720-permutation i128
    /// expansion of det(xI − S) (every diagonal S[i][i] = 1, so the
    /// diagonal factor is (x − 1)).
    #[test]
    fn pf_eigenvalue_exact() {
        let s: [[i64; 6]; 6] = [
            [1, 0, 1, 1, 1, 0],
            [0, 1, 0, 1, 1, 1],
            [1, 0, 1, 0, 1, 1],
            [1, 1, 0, 1, 0, 1],
            [1, 1, 1, 0, 1, 0],
            [0, 1, 1, 1, 0, 1],
        ];
        // power iteration, normalized to sum 1 (battery: 60 iterations)
        let matvec = |vec: &[f64; 6]| -> [f64; 6] {
            std::array::from_fn(|j| {
                s[j].iter()
                    .zip(vec.iter())
                    .map(|(&a, &b)| a as f64 * b)
                    .sum()
            })
        };
        let mut v = [1.0f64; 6];
        for _ in 0..60 {
            let w = matvec(&v);
            let tot = w.iter().sum::<f64>();
            for i in 0..6 {
                v[i] = w[i] / tot;
            }
        }
        let w = matvec(&v);
        let pf = w.iter().sum::<f64>() / v.iter().sum::<f64>();
        assert!((pf - 4.0).abs() < 1e-5, "PF eigenvalue {pf} != 4");

        // exact char poly, all 720 permutations, i128 (diagonal
        // (x−1)^fixed, off-diagonal factors ±1, vanishing on 0)
        let binom: [[i128; 7]; 7] = [
            [1, 0, 0, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0],
            [1, 2, 1, 0, 0, 0, 0],
            [1, 3, 3, 1, 0, 0, 0],
            [1, 4, 6, 4, 1, 0, 0],
            [1, 5, 10, 10, 5, 1, 0],
            [1, 6, 15, 20, 15, 6, 1],
        ];
        let mut coeffs = [0i128; 7];
        let mut perm = [0u8, 1, 2, 3, 4, 5];
        let mut visited = 0usize;
        loop {
            visited += 1;
            let mut fixed = 0usize;
            let mut offprod: i128 = 1;
            let mut alive = true;
            for i in 0..6usize {
                let j = perm[i] as usize;
                if j == i {
                    fixed += 1;
                } else if s[i][j] == 0 {
                    alive = false;
                    break;
                } else {
                    offprod *= -s[i][j] as i128; // −S[i][π(i)] = −1
                }
            }
            if alive {
                let inv: u32 = (0..6)
                    .map(|i| (0..i).filter(|&j| perm[j] > perm[i]).count() as u32)
                    .sum();
                let sign: i128 = if inv & 1 == 0 { 1 } else { -1 };
                for k in 0..=fixed {
                    // (x−1)^fixed = Σ_k C(fixed,k)·x^k·(−1)^{fixed−k}
                    let neg: i128 = if (fixed - k) & 1 == 1 { -1 } else { 1 };
                    coeffs[k] += sign * binom[fixed][k] * neg * offprod;
                }
            }
            if !next_perm(&mut perm) {
                break;
            }
        }
        assert_eq!(visited, 720, "all 720 permutations visited");
        assert_eq!(
            coeffs,
            [8i128, -6, -15, 12, 6, -6, 1],
            "char poly det(xI−S) = x⁶−6x⁵+6x⁴+12x³−15x²−6x+8"
        );
    }

    /// Lexicographic next permutation of a sorted [u8; 6]; false at
    /// the descending (last) arrangement.
    fn next_perm(p: &mut [u8; 6]) -> bool {
        let mut i = 5;
        while i > 0 && p[i - 1] > p[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = 5;
        while p[j] <= p[i - 1] {
            j -= 1;
        }
        p.swap(i - 1, j);
        p[i..].reverse();
        true
    }

    // --- no overlap + grid coverage at every growth step ---------------

    /// Battery verbatim (depth g = 1..4): coarse-cell bucketing tests
    /// only nearby tile pairs — pinned tested-pair counts
    /// 12/116/653/3017 with **zero** positive-area overlaps (exact
    /// i128-rational SH clip) — and the fine 1/4-cell grid covers R_g
    /// exactly (pinned 52/200/784/3104 in-region points).
    #[test]
    fn no_overlap_and_coverage_depths() {
        let tested_pins = [12, 116, 653, 3017];
        let cov_pins = [52, 200, 784, 3104];
        for g in 1..=4usize {
            let tiles = patch(g as u32);
            assert_eq!(tiles.len(), 4usize.pow(g as u32), "depth-{g} count");

            // Bucket tiles by coarse cell: the battery's cell() =
            // (floor min x, floor min y, floor max x, floor max y) in
            // cell units; x = (u+v)/2, y = (u−v)/2 → the bucket cell
            // is (u+v)/2, (u−v)/2 floored, computed on the doubled
            // integers x2 = u+v, y2 = u−v with floor division.
            let mut buckets: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
            for (idx, t) in tiles.iter().enumerate() {
                let b = &BASES[t.cls as usize];
                let mut min_x2 = i64::MAX;
                let mut max_x2 = i64::MIN;
                let mut min_y2 = i64::MAX;
                let mut max_y2 = i64::MIN;
                for &(bu, bv) in b {
                    let x2 = t.u + bu + t.v + bv;
                    let y2 = t.u + bu - t.v - bv;
                    min_x2 = min_x2.min(x2);
                    max_x2 = max_x2.max(x2);
                    min_y2 = min_y2.min(y2);
                    max_y2 = max_y2.max(y2);
                }
                for cx in min_x2.div_euclid(2)..=max_x2.div_euclid(2) {
                    for cy in min_y2.div_euclid(2)..=max_y2.div_euclid(2) {
                        buckets.entry((cx, cy)).or_default().push(idx);
                    }
                }
            }
            let mut tested = 0;
            let mut overlaps = 0;
            for (_, idxs) in &buckets {
                let mut seen = HashSet::new();
                for a in 0..idxs.len() {
                    for b in (a + 1)..idxs.len() {
                        let (ia, ib) = if idxs[a] > idxs[b] {
                            (idxs[b], idxs[a])
                        } else {
                            (idxs[a], idxs[b])
                        };
                        if !seen.insert((ia, ib)) {
                            continue;
                        }
                        let ta = &tiles[ia];
                        let tb = &tiles[ib];
                        let pts_a: [(i64, i64); 4] =
                            BASES[ta.cls as usize].map(|p| (p.0 + ta.u, p.1 + ta.v));
                        let pts_b: [(i64, i64); 4] =
                            BASES[tb.cls as usize].map(|p| (p.0 + tb.u, p.1 + tb.v));
                        if inter_area_pos(&pts_a, &pts_b) {
                            overlaps += 1;
                        }
                        tested += 1;
                    }
                }
            }
            assert_eq!(tested, tested_pins[g - 1], "g={g}: tested pairs");
            assert_eq!(overlaps, 0, "g={g}: positive-area overlaps");

            // Grid coverage of R_g: sample every 1/4-cell box at its
            // center (cell point (ii/4+1/8, jj/4+1/8)); (4u, 4v) =
            // (ii+jj+1, ii−jj). In R_g iff ii ≥ jj ∧ ii+jj+1 ≤
            // 4·2^{g+1} ∧ 2jj+1 ≤ 4·2^g (x ≥ y trivial).
            let mut in_region = 0;
            for ii in 0..(1u32 << (g + 3)) {
                for jj in 0..(1u32 << (g + 1)) {
                    let ii = i64::from(ii);
                    let jj = i64::from(jj);
                    if jj > ii || ii + jj + 1 > 4i64 << (g + 1) || 2 * jj + 1 > 4i64 << g {
                        continue;
                    }
                    in_region += 1;
                    let u4 = ii + jj + 1;
                    let v4 = ii - jj;
                    // point (u4, v4) inside the 4-scaled tile 4·(B_cls
                    // + (u, v)): exact closed half-plane test.
                    let mut covered = false;
                    for t in &tiles {
                        let b = &BASES[t.cls as usize];
                        let mut ok = true;
                        for i in 0..4usize {
                            let (au, av) = b[i];
                            let (bu, bv) = b[(i + 1) % 4];
                            let a = (4 * au + 4 * t.u, 4 * av + 4 * t.v);
                            let bb = (4 * bu + 4 * t.u, 4 * bv + 4 * t.v);
                            if (bb.0 - a.0) * (v4 - a.1) - (bb.1 - a.1) * (u4 - a.0) < 0 {
                                ok = false;
                                break;
                            }
                        }
                        if ok {
                            covered = true;
                            break;
                        }
                    }
                    assert!(covered, "g={g}: grid point ({ii}, {jj}) not covered");
                }
            }
            assert_eq!(in_region, cov_pins[g - 1], "g={g}: in-region points");
        }
    }

    // --- period hierarchy ----------------------------------------------

    /// The minimal x-period of the depth-g patch is (3/2)·2^g: pinned
    /// m_found = 2p = 12, 24, 48, 96, 192 for g = 2..6 (p = 6, 12,
    /// 24, 48, 96), strictly 2× growing, on tile counts
    /// 16/64/256/1024/4096. In (u, v), an x-shift p is the uv-shift
    /// (u+p, v+p); doubled units m = 2p keep p half-integers integer.
    /// The O(1) shifted-containment pre-check (mins/maxs) is
    /// cross-checked against the exact 4-vertex test for p ∈ {1, 2, 5}.
    #[test]
    fn period_hierarchy() {
        let m_pins = [12, 24, 48, 96, 192];
        let count_pins = [16, 64, 256, 1024, 4096];
        let mut prev_m = 0i64;
        for g in 2..=6usize {
            let tiles = patch(g as u32);
            assert_eq!(tiles.len(), count_pins[g - 2], "g={g} count");
            let w = 1i64 << (g + 1); // x-width 2^{g+1} of R_g
            let two_w = 2 * w;
            let min_v: [i64; 6] = BASES
                .iter()
                .map(|b| b.iter().map(|&(_, bv)| bv).min().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let max_u: [i64; 6] = BASES
                .iter()
                .map(|b| b.iter().map(|&(bu, _)| bu).max().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            // x − y = v, x + y = u exactly: per-tile mins = v + min_v,
            // maxs = u + max_u.
            let mins: Vec<i64> = tiles.iter().map(|t| t.v + min_v[t.cls as usize]).collect();
            let maxs: Vec<i64> = tiles.iter().map(|t| t.u + max_u[t.cls as usize]).collect();
            // cross-check the O(1) pre-check against the exact vertex
            // test (integer p keeps the uv vertices integral):
            // t ± p in R_g ⟺ [2·mins ≥ ∓m] ∧ [2·maxs ≤ 2W ∓ m],
            // and −p keeps the min clause only.
            for (i, t) in tiles.iter().enumerate() {
                for &p in &[1i64, 2, 5] {
                    let m = 2 * p;
                    for sgn in [1i64, -1] {
                        let exact = BASES[t.cls as usize].iter().all(|&(bu, bv)| {
                            in_rg_uv(bu + t.u + sgn * p, bv + t.v + sgn * p, g as u32)
                        });
                        let fast = 2 * mins[i] >= -sgn * m && 2 * maxs[i] <= two_w - sgn * m;
                        assert_eq!(
                            exact, fast,
                            "g={g} tile {i} p={p} sgn={sgn}: pre-check mismatch"
                        );
                    }
                }
            }
            let tset: HashSet<(u8, i64, i64)> =
                tiles.iter().map(|t| (t.cls, 2 * t.u, 2 * t.v)).collect();
            let mut m_found = 0;
            for m in 1..=(1i64 << (g + 2)) {
                let mut ok = true;
                for i in 0..tiles.len() {
                    let t = &tiles[i];
                    // t + p stays in R_g (2·maxs ≤ 2W − m): must also
                    // be a tile (x-shift p = m/2 = uv-shift (m, m))
                    if 2 * maxs[i] <= two_w - m
                        && !tset.contains(&(t.cls, 2 * t.u + m, 2 * t.v + m))
                    {
                        ok = false;
                        break;
                    }
                    // t − p stays in R_g (2·mins ≥ m): must also be a tile
                    if 2 * mins[i] >= m && !tset.contains(&(t.cls, 2 * t.u - m, 2 * t.v - m)) {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    m_found = m;
                    break;
                }
            }
            assert_ne!(m_found, 0, "g={g}: no period found");
            assert_eq!(m_found, m_pins[g - 2], "g={g}: minimal 2p");
            if prev_m != 0 {
                assert_eq!(m_found, 2 * prev_m, "g={g}: not strictly 2× growing");
            }
            prev_m = m_found;
        }
    }

    // --- nested views ----------------------------------------------------

    fn gen_at(region: BBox) -> TilingResult {
        let t = HalfHexTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(SCALE_ID, ParamValue::Float(1.0));
        t.generate(&p, &Query { region })
    }

    /// Bit-identity key: (proto, seed, per-vertex f64 bits).
    fn tile_key(tile: &Tile) -> (u8, u32, Vec<u64>) {
        let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
        for p in &tile.poly.pts {
            bits.push(p.x.to_bits());
            bits.push(p.y.to_bits());
        }
        (tile.proto, tile.seed, bits)
    }

    /// All five windows sit in the d = 7 band (E² ∈ [19/64, 19/16) —
    /// pinned per window, since all tiles are unit-sized at every depth
    /// and size alone cannot pin the depth) and refine the same 4^7
    /// patch: every tile of a smaller view must reappear bit-identically
    /// in each larger one containing it. True containments only:
    /// B5 ⊂ B1, B1 ⊂ B2, B4 ⊂ B2 (B4 and B3 stick out of their
    /// near-neighbors — no containment asserted for B3).
    #[test]
    fn nested_view_bit_identical() {
        let wins: [(&str, BBox); 5] = [
            // E² = 0.72
            ("B1", BBox::new(v2(-0.25, -0.25), v2(0.35, 0.35))),
            // E² = 648/625 = 1.0368
            ("B2", BBox::new(v2(-0.31, -0.36), v2(0.41, 0.41))),
            // E² = 0.72 — overlaps B1/B2, no containment pair
            ("B3", BBox::new(v2(0.05, -0.40), v2(0.65, 0.20))),
            // E² = 0.3204
            ("B4", BBox::new(v2(0.05, -0.265), v2(0.35, 0.215))),
            // E² = 0.32
            ("B5", BBox::new(v2(-0.15, -0.20), v2(0.25, 0.20))),
        ];
        let keys: Vec<HashSet<(u8, u32, Vec<u64>)>> = wins
            .iter()
            .map(|(name, win)| {
                let r = gen_at(*win);
                assert!(r.message.is_none(), "{name}: message");
                assert!(!r.tiles.is_empty(), "{name}: empty view");
                let s = win.size();
                let e2 = s.x * s.x + s.y * s.y;
                assert!(
                    (19.0 / 64.0..19.0 / 16.0).contains(&e2),
                    "{name}: E² = {e2} not in the d = 7 band"
                );
                r.tiles.iter().map(tile_key).collect()
            })
            .collect();
        for k in &keys[4] {
            assert!(keys[0].contains(k), "B5 tile not bit-identical in B1");
        }
        for k in &keys[0] {
            assert!(keys[1].contains(k), "B1 tile not bit-identical in B2");
        }
        for k in &keys[3] {
            assert!(keys[1].contains(k), "B4 tile not bit-identical in B2");
        }
        // B3 shares the depth-7 band and is non-empty, but overlaps B1
        // / B2 without being contained in either — no key inclusion.
    }

    // --- default view ----------------------------------------------------

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Pins both default presentations against the degenerate-floor
    /// bug (a d = 5 view renders 1024 tiles, never the whole tiling as
    /// one patch):
    ///
    /// (a) The UI/preview convention: a ±10 probe (d = 5: E² = 800 ≥
    ///     4.75), then `preview_window(9 × mean tile side, 4/3)`. Mean
    ///     AABB side per class: 1 + √3/4 ≈ 1.4330 for even classes,
    ///     (1.5 + √3)/2 ≈ 1.616 for odd — the depth-5 mix (528 even /
    ///     496 odd before culling, uncropped by the ±10 probe) bounds
    ///     the mean into [1.43, 1.62]. The derived window stays in the
    ///     d = 5 band (E² ≥ 4.75, non-empty, ≥ 5 orientation classes),
    ///     and the second probe round is a fixed point (R_5 covers
    ///     both windows, so no culling shifts the mix).
    /// (b) The CLI convention: a ±(40/3) × ±10 origin-centered pane —
    ///     d = 5, non-empty (R_5 spans x ±32, Y ±13.86 ⊃ the pane).
    #[test]
    fn default_view_fills_pane() {
        let t = HalfHexTiling;
        let p = ParamSet::default_for(t.params());
        assert_eq!(p.float(SCALE_ID, 1.0), 1.0, "scale default");

        // (a) probe ±10 → derived preview window
        let probe = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-10.0, -10.0), v2(10.0, 10.0)),
            },
        );
        assert!(probe.message.is_none());
        assert!(!probe.tiles.is_empty(), "probe must return tiles");
        let mw = tess_core::app::mean_tile_side(&probe.tiles);
        assert!(
            (1.43..=1.62).contains(&mw),
            "mean tile side {mw} outside the per-class AABB bounds"
        );
        let win = tess_core::app::preview_window(probe.periodic, mw, 4.0 / 3.0);
        let r = t.generate(&p, &Query { region: win });
        assert!(r.message.is_none());
        assert!(!r.tiles.is_empty(), "preview window must be non-empty");
        let s = win.size();
        assert!(
            s.x * s.x + s.y * s.y >= 19.0 / 4.0,
            "derived window E² left the d = 5 band"
        );
        let classes: BTreeSet<u32> = r.tiles.iter().map(|tl| tl.seed).collect();
        assert!(classes.len() >= 5, "preview shows only {classes:?} classes");

        // fixed point: the second round re-derives the same window
        let mw2 = tess_core::app::mean_tile_side(&r.tiles);
        let win2 = tess_core::app::preview_window(r.periodic, mw2, 4.0 / 3.0);
        let r2 = t.generate(&p, &Query { region: win2 });
        assert!(r2.message.is_none());
        assert!(!r2.tiles.is_empty(), "second round must be non-empty");
        let s2 = win2.size();
        assert!(
            s2.x * s2.x + s2.y * s2.y >= 19.0 / 4.0,
            "second-round window left the d = 5 band"
        );
        assert!(
            r2.tiles
                .iter()
                .map(|tl| tl.seed)
                .collect::<BTreeSet<u32>>()
                .len()
                >= 5,
            "second round shows too few classes"
        );

        // (b) CLI pane ±(40/3) × ±10
        let region = BBox::new(v2(-40.0 / 3.0, -10.0), v2(40.0 / 3.0, 10.0));
        let rc = t.generate(&p, &Query { region });
        assert!(rc.message.is_none());
        assert!(!rc.tiles.is_empty(), "CLI pane must be non-empty");
    }
}
