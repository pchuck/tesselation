//! Period-tripling (2D) — marginal supersubstitution (λ = 3, 2 label
//! classes).
//!
//! Pinned from the Bielefeld Tilings Encyclopedia entry "Period
//! Tripling 2D" (https://tilings.math.uni-bielefeld.de/substitution/
//! period-tripling-2D/, slug `period-tripling-2d`). The entry header
//! says **2 prototiles**; its Info section is **EMPTY** (no tags, no
//! references — verified by parsing the page HTML), so the rule image
//! is the only source and there is no citable reference (classification
//! arm: `source: None`). The pin rests on an 81-offset window match of
//! the entry's patch image against the simulated depth-4 tiling: the
//! match offsets are exactly the predicted period lattice {Δx ≡ Δy
//! (mod 81)}, and the color-swapped reading matches at 0 offsets
//! (recon, 2026-11-22).
//!
//! **Rule (array frame, row 0 = top of the parent's inflated 3×3
//! block, k = 3·r + c order; A = class 0, B = class 1 — the two label
//! classes the entry header calls "2 prototiles", on ONE geometric
//! unit-square prototile):**
//!
//! ```text
//!   parent A (class 0):          parent B (class 1):
//!     B A A                       A A A
//!     A B A                       A A A
//!     A A B                       A A A
//! ```
//!
//! i.e. A → 3×3 with B on the main diagonal (6 A + 3 B); B → 3×3 all
//! A.
//!
//! **Exact arithmetic.** Tiles are unit squares on Z²: every placement
//! is an exact integer triple (class, col, row). No radical ever
//! touches the geometry; f64 appears only at the view/emission
//! boundary, where the cell squares pass through the similarity map.
//!
//! **Substitution.** A depth-g patch of class `p` is 9^g unit squares
//! tiling a 3^g × 3^g block. The substitution matrix S (child ×
//! parent, order A, B):
//!
//! ```text
//!        A  B
//!   A   [ 6  9 ]
//!   B   [ 3  0 ]
//! ```
//!
//! Column sums 9 = λ² (λ = 3); PF eigenvalue 9 = λ²; characteristic
//! polynomial det(xI − S) = x² − 6x − 27 = (x − 9)(x + 3) — second
//! eigenvalue −3 = −λ, the **marginal** case (|λ₂| = λ), pinned by
//! exact 2-permutation determinants in the tests (no floating-point
//! characteristic polynomial).
//!
//! **Closed form.** In array coordinates (c, r), row 0 at the top: the
//! depth-g block is B iff v₃((c − r) mod 3^g) is odd, with v₃(0) := g
//! (v₃ = the 3-adic valuation); everything else is A. In world
//! coordinates after recentering (below), the cell (x, y) is B iff
//! v₃((x + y) mod 3^d) is odd, with v₃(0) := d. Verified on every
//! cell, g = 1..5 (9 … 59049 cells).
//!
//! **Generation (finite covered-patch model, M141 precedent).** The
//! seed is the A square (the A-root patch is the one the Bielefeld
//! patch image matches); depth d is view-anchored: with E the
//! inverse-mapped view diagonal and the d = 5 patch diagonal 243√2 =
//! √118098, zoom = 0 for E ≥ 243√2 else ceil(log3(243√2 / E)),
//! d = clamp(zoom, 5, 5). The rule DEGENERATES TO A SINGLE BAND:
//! 9^5 = 59049 ≤ MAX_TILES < 9^6 = 531441, so d = 5 at EVERY view
//! size (the standard zoom + clamp form with log3 is kept to match
//! M141).
//!
//! **Emission recentering.** Array cell (c, r) of the 3^d × 3^d block
//! (row 0 at the top) with sh = (3^d − 1)/2 = 121 at d = 5 emits the
//! unit square [x0, x0+1) × [y0, y0+1) with x0 = c − sh, y0 = sh − r
//! (row 0 at the TOP edge; covered domain [−121, 122)² at d = 5).
//! Beyond that block the view is background.
//!
//! **Limit-periodicity (measured, not asserted).** The depth-g patch
//! is the fundamental cell of an approximating periodic tiling whose
//! period lattice (exhaustive 2D torus census, g = 1..4; 1D reduction
//! at g = 5) is exactly
//! `{(a, b) mod 3^g : a + b ≡ 0 (mod 3^g)}` = ⟨(1, −1), (3^g, 0)⟩,
//! size 3^g, no other periods. Consequences:
//! - the (1, −1) direction is **period-1 at ALL depths** — a true
//!   period of the limit tiling (the signature of the marginal
//!   second eigenvalue −λ);
//! - the transverse 1D period is exactly 3^g (no proper one at any
//!   g = 1..5);
//! - the axis-aligned minimal period (full-block filter) is (3^g,
//!   3^g) = (3, 3), (9, 9), (27, 27), (81, 81), (243, 243) —
//!   strictly 3× per level: the "period tripling" manifest (vs
//!   period-doubling's ratio-1 (2^g, 2^g) and domino's ratio-2
//!   (2^g, 2^(g+1))).
//!
//! The marginal second eigenvalue leaves exactly one ambiguous line:
//! x + y = 0 (the array main diagonal) carries B at odd g, A at even
//! g (`defect_line_parity`) — exactly **two limit tilings up to
//! translation**.
//!
//! **1D shadow.** The label depends only on s = x + y, and the
//! transverse sequence is the fixed point of the centered morphism
//! **σ: A → ABA, B → AAA** (matrix [[2,3],[1,0]]; char poly
//! x² − 2x − 3 = (x−3)(x+1)) — the 3-adic cousin of period-doubling's
//! σ: a → ab, b → aa. The 2D rule is NOT a σ⊗σ product (that needs 4
//! label classes; there are only 2) — it is the 1D sequence pulled
//! back along x + y (1-periodic in the (1, −1) direction).

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

// ---------------------------------------------------------------------------
// Exact Z² model: rule, refinement
// ---------------------------------------------------------------------------

/// Label class ids: A (gray in the Bielefeld image) = 0, B (blue) = 1.
/// The two classes the entry header calls "2 prototiles", on ONE
/// geometric unit-square prototile (emitted as seeds 0..1).
const A: u8 = 0;
const B: u8 = 1;

/// The pinned rule: (child class, col, row) in the parent's inflated
/// 3×3 frame (array frame, row 0 at the top, k = 3·r + c order).
/// A → 3×3 with B on the main diagonal (6 A + 3 B); B → 3×3 all A.
/// Pinned by the 81-offset window match of the Bielefeld patch image
/// against the simulated depth-4 tiling (recon, 2026-11-22);
/// cross-checked against the closed form in `closed_form_pin`.
const RULES: [[(u8, i64, i64); 9]; 2] = [
    [
        (B, 0, 0),
        (A, 1, 0),
        (A, 2, 0),
        (A, 0, 1),
        (B, 1, 1),
        (A, 2, 1),
        (A, 0, 2),
        (A, 1, 2),
        (B, 2, 2),
    ],
    [
        (A, 0, 0),
        (A, 1, 0),
        (A, 2, 0),
        (A, 0, 1),
        (A, 1, 1),
        (A, 2, 1),
        (A, 0, 2),
        (A, 1, 2),
        (A, 2, 2),
    ],
];

/// Frame of the depth-g patch in unit cells: 3^g × 3^g (square block).
fn patch_dims(g: u32) -> (i64, i64) {
    let a = 3i64.pow(g);
    (a, a)
}

/// Diagonal of the d = 5 patch block (243 × 243): 243√2 = √118098.
fn patch_diag() -> f64 {
    (118098.0_f64).sqrt()
}

/// Presentation depth floor — which is also the max depth: 9^5 = 59049
/// ≤ MAX_TILES < 9^6 = 531441, so the zoom rule degenerates to the
/// single band d = 5 at every view size. The zoom computation in
/// `generate` still uses the standard clamp form (M141 pattern).
const D_PRESENT: u32 = 5;
/// Max refinement depth: 9^5 = 59049 ≤ MAX_TILES < 9^6 = 531441.
const MAX_DEPTH: u32 = 5;

/// One tile of a depth-g patch: class + (col, row) of the unit cell,
/// exact integers (array frame, row 0 at the top).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TileCell {
    ty: u8,
    c: i64,
    r: i64,
}

/// Nesting convention (verified in `closed_form_pin`): the child at
/// frame cell (c, r) of the depth-1 rule is replaced by its own
/// depth-(d−1) patch at offset 3^(d−1)·(c, r).
fn patch_of_at(p: u8, g: u32, c0: i64, r0: i64) -> Vec<TileCell> {
    if g == 1 {
        return RULES[p as usize]
            .iter()
            .map(|&(ty, c, r)| TileCell {
                ty,
                c: c0 + c,
                r: r0 + r,
            })
            .collect();
    }
    let m = 3i64.pow(g - 1);
    RULES[p as usize]
        .iter()
        .flat_map(|&(ty, c, r)| patch_of_at(ty, g - 1, c0 + m * c, r0 + m * r))
        .collect()
}

/// The depth-g patch of class `p`: 9^g unit squares tiling the 3^g ×
/// 3^g block.
fn patch_of(p: u8, g: u32) -> Vec<TileCell> {
    patch_of_at(p, g, 0, 0)
}

// ---------------------------------------------------------------------------
// Tiling
// ---------------------------------------------------------------------------

/// The 2D period-tripling marginal supersubstitution tiling (non-periodic; limit-periodic).
pub struct PeriodTriplingTiling;

impl Tiling for PeriodTriplingTiling {
    fn id(&self) -> &'static str {
        "period-tripling"
    }

    fn name(&self) -> &'static str {
        "Period-tripling (2D) substitution"
    }

    fn description(&self) -> &'static str {
        "2D period-tripling substitution (Bielefeld entry \"Period \
         Tripling 2D\" — Info section empty; rule pinned by an \
         81-offset window match of the patch image): one unit-square \
         prototile, 2 label classes (A, B), λ = 3 substitution into 9 \
         children (A → 3×3 with B on the main diagonal, B → 3×3 all A; \
         substitution matrix S = [[6,9],[3,0]] (child × parent), column \
         sums 9 = λ², PF eigenvalue 9 = λ², char poly x²−6x−27 = \
         (x−9)(x+3) — marginal second eigenvalue −3 = −λ). Finite \
         covered patch = 3^d × 3^d block recentered on the origin, \
         d = 5 (9^5 = 59049 ≤ MAX_TILES < 9^6 — single depth band); \
         exact integer cell arithmetic end to end. Limit-periodic (long-range order) via \
         approximating periodic tilings: the depth-g period lattice is \
         exactly <(1,−1),(3^g,0)> (size 3^g — period-1 in the (1,−1) \
         direction at all depths), axis-aligned period (3^g, 3^g) = \
         (3,3), (9,9), (27,27), (81,81), (243,243), strictly 3× \
         growing per level (\"period tripling\"); two limit tilings \
         up \
         to translation (defect line x+y = 0: B at odd g, A at even g)."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Aperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "non-periodic substitution tiling; limit-periodic (period-tripling 2D; Bielefeld entry \
             \"Period Tripling 2D\" — no references, rule pinned by \
             the patch-image window match); approximating period \
             lattice <(1,-1),(3^d,0)>, size 3^d — period-1 in the \
             (1,-1) direction at all depths; marginal second \
             eigenvalue -3 = -lambda; two limit tilings up to \
             translation",
        );
        let map = similarity_map(params);
        let inv = map.inverse();

        // Query box in the cell frame: axis-aligned hull of the
        // inverse image of the four world corners.
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

        // View-anchored depth (the standard M141 zoom form with log3;
        // it degenerates to the single band d = 5 — 9^5 = 59049 ≤
        // MAX_TILES < 9^6 = 531441).
        let ext = (region.max - region.min).length();
        let pd = patch_diag();
        let zoom = if ext < pd {
            (pd / ext).log(3.0).ceil() as u32
        } else {
            0
        };
        let d = zoom.clamp(D_PRESENT, MAX_DEPTH);
        assert!(
            (9u64).pow(d) as usize <= MAX_TILES,
            "depth {d} exceeds MAX_TILES"
        );

        // Emission recentering: array cell (c, r) (row 0 at the top)
        // of the 3^d × 3^d block emits the unit square [x0, x0+1) ×
        // [y0, y0+1) with x0 = c − sh, y0 = sh − r — the covered
        // domain is [−sh, sh+1)², [−121, 122)² at d = 5.
        let (frame, _) = patch_dims(d);
        let sh = (frame - 1) / 2;
        let patch_box = BBox::new(
            Vec2::new(-sh as f64, -sh as f64),
            Vec2::new((sh + 1) as f64, (sh + 1) as f64),
        );
        if !patch_box.intersects(region) {
            // Finite patch: outside the covered domain is background.
            return TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: None,
                message: None,
            };
        }

        let cells = patch_of(A, d);

        // Emission: exact (class, col, row) integers → the recentered
        // unit square, culled to the query region. f64 only here.
        let mut tiles = Vec::new();
        for c in &cells {
            let x0 = c.c - sh;
            let y0 = sh - c.r;
            let poly = Polygon::new(vec![
                Vec2::new(x0 as f64, y0 as f64),
                Vec2::new((x0 + 1) as f64, y0 as f64),
                Vec2::new((x0 + 1) as f64, (y0 + 1) as f64),
                Vec2::new(x0 as f64, (y0 + 1) as f64),
            ])
            .transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto: 0,
                    seed: c.ty as u32, level: d as u8,
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
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use tess_core::params::ParamValue;

    // --- rule line -------------------------------------------------------

    /// Rule line, both parents: the nine children are the nine
    /// distinct 3×3 frame cells — containment + disjointness +
    /// equal area ⇒ the rule line closes. The table order is the
    /// pinned k = 3·r + c.
    #[test]
    fn rule_line() {
        for p in 0..2u8 {
            let mut cells: Vec<(i64, i64)> =
                RULES[p as usize].iter().map(|&(_, c, r)| (c, r)).collect();
            cells.sort();
            cells.dedup();
            assert_eq!(
                cells.len(),
                9,
                "p={p}: children must be nine distinct frame cells"
            );
            for (k, &(ty, c, r)) in RULES[p as usize].iter().enumerate() {
                assert!(ty < 2, "p={p}: child class out of range");
                assert!(
                    c >= 0 && c < 3 && r >= 0 && r < 3,
                    "p={p}: child outside the 3×3 frame"
                );
                assert_eq!(
                    (k % 3, k / 3),
                    (c as usize, r as usize),
                    "p={p}: table order is k = 3·r + c"
                );
            }
        }
    }

    // --- closed form -----------------------------------------------------

    /// 3-adic valuation of the residue n (0 ≤ n < 3^cap), with the
    /// convention v₃(0) := cap — the defect line carries the full
    /// depth's valuation.
    fn v3(n: i64, cap: u32) -> i64 {
        if n == 0 {
            return cap as i64;
        }
        let mut v = 0i64;
        let mut m = n;
        while m % 3 == 0 {
            m /= 3;
            v += 1;
        }
        v
    }

    /// RULES is the fixed point of the closed form (the rule is not a
    /// σ⊗σ product — this IS the pin):
    ///
    /// (a) for g = 1..4, the 3×3 sub-block of the depth-(g+1) block
    ///     over depth-g cell (c, r) equals RULES[its depth-g label]
    ///     translated by (3c, 3r), every cell;
    /// (b) on the full d = 5 patch (59049 cells), world cell (x, y) is
    ///     B iff v₃((x + y) mod 243) is odd, with v₃(0) := 5.
    #[test]
    fn closed_form_pin() {
        // (a) fixed point: depth-(g+1) block == RULES applied to every
        // depth-g cell, g = 1..4.
        for g in 1..=4u32 {
            let base: Vec<(u8, i64, i64)> =
                patch_of(A, g).into_iter().map(|t| (t.ty, t.c, t.r)).collect();
            let next_map: BTreeMap<(i64, i64), u8> = patch_of(A, g + 1)
                .into_iter()
                .map(|t| ((t.c, t.r), t.ty))
                .collect();
            let g2 = g + 1;
            for &(ty, c, r) in &base {
                for &(kty, kc, kr) in &RULES[ty as usize] {
                    assert_eq!(
                        next_map.get(&(3 * c + kc, 3 * r + kr)),
                        Some(&kty),
                        "g={g}→{g2}: sub-block over cell ({c}, {r}) of \
                         label {ty} disagrees with RULES"
                    );
                }
            }
        }
        // (b) world closed form on the full d = 5 patch.
        let d = 5u32;
        let side = 3i64.pow(d);
        let sh = (side - 1) / 2;
        let mut n_b = 0i64;
        for t in &patch_of(A, d) {
            let x = t.c - sh;
            let y = sh - t.r;
            let s = ((x + y) % side + side) % side;
            let want = if v3(s, d) % 2 == 1 { B } else { A };
            assert_eq!(
                t.ty, want,
                "cell ({x}, {y}): closed form and the depth-5 patch disagree"
            );
            if want == B {
                n_b += 1;
            }
        }
        // Census cross-pin (the S^5 A-root count, from (3)).
        assert_eq!(n_b, 14823, "depth-5 B census = S^5·e_A[1]");
    }

    // --- substitution matrix + growth -----------------------------------

    /// S derived from RULES equals the pinned matrix (child × parent,
    /// order A, B); column sums = 9 = λ²; S^g·e pinned at g = 1..5 for
    /// BOTH roots, total 9^g, with the patch census agreeing at every
    /// level.
    #[test]
    fn substitution_matrix_and_counts() {
        let mut s = [[0i64; 2]; 2];
        for p in 0..2usize {
            for &(ty, _, _) in &RULES[p] {
                s[ty as usize][p] += 1;
            }
        }
        assert_eq!(s, [[6, 9], [3, 0]], "S[j][i] (child j, parent i)");
        for i in 0..2usize {
            assert_eq!(s[0][i] + s[1][i], 9, "column {i} sum = λ²");
        }
        let pin_a: [[i64; 2]; 5] = [
            [6, 3],
            [63, 18],
            [540, 189],
            [4941, 1620],
            [44226, 14823],
        ];
        let pin_b: [[i64; 2]; 5] = [
            [9, 0],
            [54, 27],
            [567, 162],
            [4860, 1701],
            [44469, 14580],
        ];
        let mut ca = [1i64, 0];
        let mut cb = [0i64, 1];
        for g in 1..=5usize {
            ca = [
                s[0][0] * ca[0] + s[0][1] * ca[1],
                s[1][0] * ca[0] + s[1][1] * ca[1],
            ];
            cb = [
                s[0][0] * cb[0] + s[0][1] * cb[1],
                s[1][0] * cb[0] + s[1][1] * cb[1],
            ];
            assert_eq!(ca, pin_a[g - 1], "S^g·e_A at g = {g}");
            assert_eq!(cb, pin_b[g - 1], "S^g·e_B at g = {g}");
            assert_eq!(
                ca[0] + ca[1],
                9i64.pow(g as u32),
                "g={g}: total from A = 9^g"
            );
            assert_eq!(
                cb[0] + cb[1],
                9i64.pow(g as u32),
                "g={g}: total from B = 9^g"
            );
            // patch census cross-check, both roots.
            let mut cens_a = [0i64; 2];
            for t in &patch_of(A, g as u32) {
                cens_a[t.ty as usize] += 1;
            }
            assert_eq!(cens_a, ca, "depth-{g} A-root census");
            let mut cens_b = [0i64; 2];
            for t in &patch_of(B, g as u32) {
                cens_b[t.ty as usize] += 1;
            }
            assert_eq!(cens_b, cb, "depth-{g} B-root census");
        }
    }

    // --- characteristic polynomial ----------------------------------------

    /// The pinned 2×2 matrix (child × parent, order A, B).
    const S: [[i64; 2]; 2] = [[6, 9], [3, 0]];

    /// xI − S: diagonal x − S[i][i], off-diagonal −S[i][j].
    fn xi_minus_s(x: i64) -> [[i64; 2]; 2] {
        [[x - S[0][0], -S[0][1]], [-S[1][0], x - S[1][1]]]
    }

    /// 2-permutation expansion of a 2×2 determinant, returning the
    /// value and the number of visited permutations (the M135 lesson:
    /// a truncated expansion silently returns a wrong det).
    fn det2(m: [[i64; 2]; 2]) -> (i64, u32) {
        let mut total = 0i64;
        let mut cnt = 0u32;
        for p0 in 0..2 {
            for p1 in 0..2 {
                if p1 == p0 {
                    continue;
                }
                let sign = if p0 > p1 { -1 } else { 1 };
                total += sign * m[0][p0] * m[1][p1];
                cnt += 1;
            }
        }
        (total, cnt)
    }

    /// Characteristic polynomial det(xI − S) = x² − 6x − 27 = (x − 9)(x
    /// + 3) — exact 2-permutation determinants at x = 0..9 (pinned
    /// −27, −32, −35, −36, −35, −32, −27, −20, −11, 0): PF eigenvalue
    /// 9 = λ²; second eigenvalue −3 = −λ — the **marginal** case
    /// (|λ₂| = λ).
    #[test]
    fn char_poly() {
        let pin = [-27i64, -32, -35, -36, -35, -32, -27, -20, -11, 0];
        for (x, want) in pin.iter().enumerate().map(|(i, w)| (i as i64, *w)) {
            let (d, cnt) = det2(xi_minus_s(x));
            assert_eq!(cnt, 2, "full 2-permutation expansion at x = {x}");
            assert_eq!(d, want, "det(xI−S) at x = {x}");
            // Cross-check the closed form x² − 6x − 27.
            assert_eq!(d, x * x - 6 * x - 27, "char poly closed form at x = {x}");
        }
        assert_eq!(pin[0], -27, "det S = −27 = 9 × (−3)");
        assert_eq!(pin[9], 0, "det(9I−S) = 0: PF eigenvalue 9 = λ²");
        let (dm3, _) = det2(xi_minus_s(-3));
        assert_eq!(dm3, 0, "det(−3I−S) = 0: second eigenvalue −3 = −λ (marginal)");
        let trace: i64 = S[0][0] + S[1][1];
        assert_eq!(trace, 6, "trace S = 9 + (−3) = 6");
        let det = S[0][0] * S[1][1] - S[0][1] * S[1][0];
        assert_eq!(det, -27, "det S = 9 × (−3) = −27");
    }

    // --- no overlap + coverage --------------------------------------------

    /// Exact single coverage of the 3^g × 3^g frame (0 overlaps, 0
    /// gaps, pinned tile counts 9/81/729/6561/59049 at g = 1..5).
    #[test]
    fn no_overlap_and_coverage_depths() {
        let pin_counts = [9, 81, 729, 6561, 59049];
        for g in 1..=5usize {
            let tiles = patch_of(A, g as u32);
            let (side, _) = patch_dims(g as u32);
            assert_eq!(tiles.len(), pin_counts[g - 1], "depth-{g} count = 9^g");
            // Cell-level exact single coverage: every frame cell owned
            // by exactly one tile.
            let mut owner: Vec<Option<usize>> = vec![None; (side * side) as usize];
            for (i, t) in tiles.iter().enumerate() {
                assert!(
                    t.c >= 0 && t.c + 1 <= side && t.r >= 0 && t.r + 1 <= side,
                    "tile {i} outside the frame"
                );
                let k = (t.r * side + t.c) as usize;
                assert!(owner[k].is_none(), "overlap at cell ({}, {})", t.c, t.r);
                owner[k] = Some(i);
            }
            assert!(
                owner.iter().all(|o| o.is_some()),
                "g={g}: gaps in the frame"
            );
        }
    }

    // --- period hierarchy ---------------------------------------------------

    /// The minimal translation period of the class labeling of the
    /// depth-g patch: a shift (p, q) leaves the labeling invariant iff
    /// every cell (c, r) and (c+p, r) (resp. (c, r+q)) agree on class
    /// over the overlap, AND p (resp. q) divides the block side 3^g —
    /// the period must tile the whole block to be a period of
    /// the approximating periodic tiling (the full-block filter).
    ///
    /// Unit-square degeneracy (M141 precedent): every tile is exactly
    /// one cell, so the domino-style in-tile-offset term degenerates
    /// here — all offsets are 0 and `0 + p == 0` is false for every
    /// p ≥ 1, killing every candidate. The overlap-only notion
    /// (without the p | side filter) gives degenerate border-coincidence
    /// values and must not be used.
    fn patch_period(g: u32) -> (i64, i64) {
        let tiles = patch_of(A, g);
        let (side, _) = patch_dims(g);
        let mut grid: Vec<u8> = vec![0; (side * side) as usize];
        for t in &tiles {
            grid[(t.r * side + t.c) as usize] = t.ty;
        }
        let mut px = 0i64;
        for p in 1..=side {
            if side % p != 0 {
                continue; // full-block filter: the period tiles the block
            }
            let mut ok = true;
            for r in 0..side {
                if !ok {
                    break;
                }
                for c in 0..(side - p) {
                    if grid[(r * side + c) as usize] != grid[(r * side + c + p) as usize] {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                px = p;
                break;
            }
        }
        let mut py = 0i64;
        for q in 1..=side {
            if side % q != 0 {
                continue;
            }
            let mut ok = true;
            for r in 0..(side - q) {
                if !ok {
                    break;
                }
                for c in 0..side {
                    if grid[(r * side + c) as usize] != grid[((r + q) * side + c) as usize] {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                py = q;
                break;
            }
        }
        (px, py)
    }

    /// World-indexed grid of the depth-g block: `grid[(j*side + i)]` is
    /// the class at world cell (i − sh, j − sh), i, j in 0..side. (x + sh
    /// = (c − sh) + sh = c; y + sh = (sh − r) + sh = side − 1 − r.)
    fn world_grid(g: u32) -> (Vec<u8>, i64) {
        let (side, _) = patch_dims(g);
        let mut grid = vec![0u8; (side * side) as usize];
        for t in &patch_of(A, g) {
            let i = t.c;
            let j = side - 1 - t.r;
            grid[(j * side + i) as usize] = t.ty;
        }
        (grid, side)
    }

    /// The period manifest (measured, never asserted):
    ///
    /// 1. Axis-aligned minimal period (full-block filter), g = 1..5:
    ///    (3^g, 3^g) = (3,3), (9,9), (27,27), (81,81), (243,243) —
    ///    strictly 3× growing per level: the "period tripling"
    ///    signature (vs period-doubling's ratio-1 (2^g, 2^g) and
    ///    domino's ratio-2 (2^g, 2^(g+1))).
    /// 2. Full period-lattice census, g = 1..4 (exhaustive 2D torus
    ///    sweep over WORLD shift coordinates (a, b): (x, y) → (x+a,
    ///    y+b), wrap mod 3^g): exactly {(a, b): a + b ≡ 0 (mod 3^g)},
    ///    size 3^g, no other periods — generated by (1, −1) and
    ///    (3^g, 0). The (1, −1) direction is period-1 at ALL depths
    ///    (a true period of the limit tiling). (The array-frame twin
    ///    of this law is {(a, b): a − b ≡ 0 (mod 3^g)} — a world
    ///    shift (a, b) is an array shift (a, −b).)
    /// 3. 1D reduction at g = 5: the label depends only on s = x + y
    ///    (verified on all 59049 cells), and the transverse 1D period
    ///    is exactly 3^5 = 243 — no proper one (full-block filter).
    #[test]
    fn period_hierarchy() {
        // 1. axis-aligned minimal period, g = 1..5.
        let mut prev: (i64, i64) = (0, 0);
        for g in 1..=5usize {
            let (px, py) = patch_period(g as u32);
            assert_eq!(
                px,
                3i64.pow(g as u32),
                "g={g}: minimal x-period {px} (want 3^g)"
            );
            assert_eq!(
                py,
                3i64.pow(g as u32),
                "g={g}: minimal y-period {py} (want 3^g)"
            );
            if prev != (0, 0) {
                assert_eq!(
                    (px, py),
                    (3 * prev.0, 3 * prev.1),
                    "g={g}: not strictly 3× growing"
                );
            }
            prev = (px, py);
        }

        // 2. full period-lattice census, g = 1..4 (world torus shifts).
        for g in 1..=4usize {
            let (grid, side) = world_grid(g as u32);
            let mut periods: Vec<(i64, i64)> = Vec::new();
            for a in 0..side {
                for b in 0..side {
                    let mut ok = true;
                    'sweep: for i in 0..side {
                        for j in 0..side {
                            if grid[(((i + a) % side) * side + ((j + b) % side)) as usize]
                                != grid[(j * side + i) as usize]
                            {
                                ok = false;
                                break 'sweep;
                            }
                        }
                    }
                    if ok {
                        periods.push((a, b));
                    }
                }
            }
            assert_eq!(
                periods.len(),
                side as usize,
                "g={g}: lattice size (want 3^g = {side})"
            );
            for a in 0..side {
                let b = (side - a % side) % side; // a + b ≡ 0 (mod side), b in 0..side
                assert!(
                    periods.contains(&(a, b)),
                    "g={g}: (a, b) = ({a}, {b}) of a + b ≡ 0 (mod {side}) \
                     missing from the measured lattice"
                );
            }
        }

        // 3. 1D reduction at g = 5: the label depends only on
        //    s = x + y (anti-diagonal constancy, all 59049 cells),
        //    and the transverse 1D period is exactly 3^5 = 243 — no
        //    proper one divides 243 and acts as a period.
        {
            let g = 5usize;
            let (grid, side) = world_grid(g as u32);
            let span = (2 * side - 1) as usize; // s = x + y ranges over [−(side−1), side−1]
            let mut diag = vec![0u8; span];
            for s_i in 0..span {
                // Anti-diagonal i + j = s_i (s = s_i − (side − 1));
                // verify the whole anti-diagonal is class-constant.
                let mut first = None;
                for j in 0..side {
                    let i = s_i as i64 - j;
                    if (0..side).contains(&i) {
                        let k = grid[(j * side + i) as usize];
                        match first {
                            None => first = Some(k),
                            Some(f) => assert_eq!(
                                k, f,
                                "s = {} (index {}): the class varies along \
                                 the anti-diagonal — the label does not \
                                 depend only on x + y",
                                s_i as i64 - (side - 1),
                                s_i
                            ),
                        }
                    }
                }
                diag[s_i] = first.expect("anti-diagonal has a cell");
            }
            for t in 1..side {
                if side % t != 0 {
                    continue; // full-block filter: t tiles the window
                }
                let mut ok = true;
                for s_i in 0..(span - t as usize) {
                    if diag[s_i + t as usize] != diag[s_i] {
                        ok = false;
                        break;
                    }
                }
                assert!(
                    !ok,
                    "g=5: proper transverse period t = {t} found (the \
                     transverse period must be exactly 3^5 = {side})"
                );
            }
        }
    }

    // --- defect line --------------------------------------------------------

    /// The marginal ambiguity: the main diagonal (array c = r; world
    /// x + y = 0) is all B at odd g, all A at even g, g = 1..5 — the
    /// limit leaves exactly two limit tilings up to translation.
    #[test]
    fn defect_line_parity() {
        for g in 1..=5u32 {
            let side = 3i64.pow(g);
            let want = if g % 2 == 1 { B } else { A };
            let tiles = patch_of(A, g);
            let mut n = 0i64;
            for t in &tiles {
                if t.c == t.r {
                    n += 1;
                    assert_eq!(
                        t.ty,
                        want,
                        "g={g}: cell ({}, {}) on the main diagonal (world \
                         x + y = 0)",
                        t.c,
                        t.r
                    );
                }
            }
            assert_eq!(n, side, "g={g}: the diagonal has exactly 3^g cells");
        }
    }

    // --- nested views --------------------------------------------------------

    fn gen_at(region: BBox) -> TilingResult {
        let t = PeriodTriplingTiling;
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

    /// All five windows (M141's verbatim set; E² = 0.18…0.405) sit in
    /// the single d = 5 band — d = 5 at EVERY view size, since 9^5 ≤
    /// MAX_TILES < 9^6 leaves the zoom rule no extent to span — and
    /// emit the same 3^5 × 3^5 patch recentered on the origin: every
    /// tile of a smaller view must reappear bit-identically (proto,
    /// seed, per-vertex f64 bits) in each larger one containing it.
    /// True containments only: B5 ⊂ B1, B1 ⊂ B2, B4 ⊂ B2 (B4 and B3
    /// stick out of their near-neighbors — no containment asserted
    /// for B3).
    #[test]
    fn nested_view_bit_identical() {
        let wins: [(&str, BBox); 5] = [
            // E² = 0.32
            ("B1", BBox::new(v2(-0.10, -0.10), v2(0.30, 0.30))),
            // E² = 0.405
            ("B2", BBox::new(v2(-0.12, -0.12), v2(0.33, 0.33))),
            // E² = 0.25 — overlaps B1/B2, no containment pair
            ("B3", BBox::new(v2(0.05, -0.20), v2(0.45, 0.10))),
            // E² = 0.25
            ("B4", BBox::new(v2(0.00, -0.10), v2(0.30, 0.30))),
            // E² = 0.18
            ("B5", BBox::new(v2(-0.05, -0.05), v2(0.25, 0.25))),
        ];
        let keys: Vec<HashSet<(u8, u32, Vec<u64>)>> = wins
            .iter()
            .map(|(name, win)| {
                let r = gen_at(*win);
                assert!(r.message.is_none(), "{name}: message");
                assert!(!r.tiles.is_empty(), "{name}: empty view");
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
    }

    // --- default view --------------------------------------------------------

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Pins the default presentation:
    ///
    /// (a) The UI/preview convention: a ±10 probe (E² = 800 — in band,
    ///     d = 5 at every size), then `preview_window(mean tile side,
    ///     4/3)` (base = 9 × mean side = 9.0): every unit square has
    ///     AABB sides (1, 1), so the mean side is 1.0 and the derived
    ///     window is exactly ±6 × ±4.5 (E² = 225), non-empty with
    ///     BOTH label classes. Census pin (recon C9, re-verified):
    ///     the 120 cells whose bottom-left corners (x0, y0) satisfy
    ///     x0 ∈ [−6, 6) ∩ Z, y0 ∈ [−5, 5) ∩ Z carry A = 84, B = 36.
    ///     The second probe round is a fixed point (all tiles have
    ///     side (1, 1), so no culling shifts the mix).
    /// (b) The CLI convention: a ±(40/3) × ±10 origin-centered pane
    ///     (E² ≈ 1111, in band) — non-empty (the d = 5 block
    ///     [−121, 122)² covers the pane).
    #[test]
    fn default_view_fills_pane() {
        let t = PeriodTriplingTiling;
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
            (1.0 - 1e-9..=1.0 + 1e-9).contains(&mw),
            "mean tile side {mw} — every unit square has AABB sides (1, 1), mean 1.0"
        );
        let win = tess_core::app::preview_window(probe.periodic, mw, 4.0 / 3.0);
        assert!(
            (win.min.x + 6.0).abs() < 1e-9
                && (win.min.y + 4.5).abs() < 1e-9
                && (win.max.x - 6.0).abs() < 1e-9
                && (win.max.y - 4.5).abs() < 1e-9,
            "derived preview window {:?} != ±6 × ±4.5",
            win
        );
        let r = t.generate(&p, &Query { region: win });
        assert!(r.message.is_none());
        assert!(!r.tiles.is_empty(), "preview window must be non-empty");
        let classes: BTreeSet<u32> = r.tiles.iter().map(|tl| tl.seed).collect();
        assert_eq!(
            classes,
            BTreeSet::from([0u32, 1]),
            "preview must show BOTH label classes (seeds 0..1)"
        );
        // Census pin (recon C9): the 120 cells with bottom-left
        // corners (x0, y0) in x0 ∈ [−6, 6) ∩ Z, y0 ∈ [−5, 5) ∩ Z carry
        // A = 84, B = 36.
        let mut full = 0usize;
        let mut cens = [0usize; 2];
        for tl in &r.tiles {
            let bb = tl.poly.aabb();
            if bb.min.x >= -6.0 - 1e-9
                && bb.min.x < 6.0 - 1e-9
                && bb.min.y >= -5.0 - 1e-9
                && bb.min.y < 5.0 - 1e-9
            {
                full += 1;
                cens[tl.seed as usize] += 1;
            }
        }
        assert_eq!(full, 120, "cells with bottom-left corners in [−6,6)×[−5,5)");
        assert_eq!(
            cens,
            [84, 36],
            "window census (A = seed 0, B = seed 1)"
        );

        // fixed point: the second round re-derives the same window
        let mw2 = tess_core::app::mean_tile_side(&r.tiles);
        let win2 = tess_core::app::preview_window(r.periodic, mw2, 4.0 / 3.0);
        assert!(
            (win2.min.x - win.min.x).abs() < 1e-9
                && (win2.min.y - win.min.y).abs() < 1e-9
                && (win2.max.x - win.max.x).abs() < 1e-9
                && (win2.max.y - win.max.y).abs() < 1e-9,
            "second-round window {:?} is not a fixed point of {:?}:",
            win2,
            win
        );
        let r2 = t.generate(&p, &Query { region: win2 });
        assert!(r2.message.is_none());
        assert!(!r2.tiles.is_empty(), "second round must be non-empty");
        let classes2: BTreeSet<u32> = r2.tiles.iter().map(|tl| tl.seed).collect();
        assert_eq!(
            classes2,
            BTreeSet::from([0u32, 1]),
            "second round shows only some label classes"
        );

        // (b) CLI pane ±(40/3) × ±10
        let region = BBox::new(v2(-40.0 / 3.0, -10.0), v2(40.0 / 3.0, 10.0));
        let rc = t.generate(&p, &Query { region });
        assert!(rc.message.is_none());
        assert!(!rc.tiles.is_empty(), "CLI pane must be non-empty");
    }

    // --- background -----------------------------------------------------------

    /// A far view beyond the covered domain of the single band
    /// returns background (no tiles, no refusal message): the d = 5
    /// covered domain is [−121, 122)², and 150 ≥ 122 puts the view at
    /// (150, 150)–(150.5, 150.5) outside it. (M141's far-view
    /// coordinate (100, 100) is INSIDE this domain — a far-view trap
    /// for this tiling.)
    #[test]
    fn far_view_is_background() {
        let r = gen_at(BBox::new(v2(150.0, 150.0), v2(150.5, 150.5)));
        assert!(
            r.message.is_none(),
            "background must not set a refusal message"
        );
        assert!(r.tiles.is_empty(), "far view must be empty (background)");
    }
}
