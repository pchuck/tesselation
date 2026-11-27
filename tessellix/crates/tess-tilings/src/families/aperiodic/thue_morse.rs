//! Thue–Morse — 2D product (τ⊗τ) of the 1D Thue–Morse morphism.
//!
//! The 1D rule is pinned from the Bielefeld Tilings Encyclopedia entry
//! "Thue Morse"
//! (https://tilings.math.uni-bielefeld.de/substitution/thue-morse/):
//! `0 → 01`, `1 → 10` — the classic Thue–Morse morphism, two
//! prototiles, discovered by Thue/Prouhet/Morse; reference [JS99]
//! Allouche & Shallit, *The ubiquitous Prouhet–Thue–Morse sequence*,
//! Springer 1999. The entry is 1D-only; the 2D member here is its
//! **product** τ⊗τ: unit squares of Z² labelled by pairs
//! `(x, y) ∈ {0, 1}²`, each parent inflated by λ = 2 into a 2×2 frame
//! whose child at frame position `(c, r)` receives the label
//! `2·(x ⊕ c) + (y ⊕ r)` (τ(x)_c = x ⊕ c). Four label classes
//! `00, 01, 10, 11` (class index = 2·x + y, x = column letter,
//! y = row letter) sit on ONE geometric square prototile; the
//! labels are emitted as the tile `seed` (seeds 0..3).
//!
//! **Exact arithmetic.** Tiles are unit squares on Z²: every placement
//! is an exact integer triple (class, col, row). No radical ever
//! touches the geometry; f64 appears only at the view/emission
//! boundary, where the cell squares pass through the similarity map.
//!
//! **Substitution.** A depth-g patch of class `p` is 4^g unit squares
//! tiling a 2^g × 2^g block. The substitution matrix S = S[j][i]
//! (children of class j in parent i, order 00, 01, 10, 11) is the
//! all-ones matrix — the Steinhaus property: every parent block
//! contains each child class exactly once:
//!
//! ```text
//!        00 01 10 11
//!   00  [ 1  1  1  1 ]
//!   01  [ 1  1  1  1 ]
//!   10  [ 1  1  1  1 ]
//!   11  [ 1  1  1  1 ]
//! ```
//!
//! Column sums 4 = λ², PF eigenvalue 4, characteristic polynomial
//! det(xI − S) = x⁴ − 4x³ — eigenvalues {4, 0, 0, 0} (rank 1),
//! pinned by exact 24-permutation determinants in the tests (no
//! floating-point characteristic polynomial).
//!
//! **Generation (finite covered-patch model, domino precedent).**
//! The seed is the 00 square; depth d is view-anchored: with E the
//! inverse-mapped view diagonal and the d = 5 patch block 32 × 32
//! (diagonal 32√2 = √2048), zoom = 0 for E ≥ 32√2 else
//! ceil(log2(32√2/E)), d = clamp(zoom, 5, 8) (4^8 = 65 536 ≤
//! MAX_TILES). Depth bands in E²: d = 5 for E² ≥ 2, d = 6 for
//! [0.5, 2), d = 7 for [0.125, 0.5), d = 8 below. Nested views
//! within one band emit bit-identical tiles on their intersection
//! (`nested_view_bit_identical`).
//!
//! **Emission recentering.** The depth-d patch spans the 2^d × 2^d
//! block; it is recentered on the origin by the constant shift
//! (2^(d−1), 2^(d−1)) at emission: the covered domain is
//! |x| ≤ 2^(d−1), |y| ≤ 2^(d−1) in cell units. Beyond that block the
//! view is background.
//!
//! **Aperiodicity (measured, not asserted).** The depth-g patch
//! carries **no proper translation period at any depth** — the
//! distinguishing aperiodic signature, in contrast to the
//! limit-periodic period-doubling cousin, whose depth-g patch is the
//! fundamental cell of the period-(2^g, 2^g) approximating tiling.
//! Measured over g = 1..6 (Rust battery `no_period_manifest`): every
//! proper full-block candidate shift (px, py) — (g+1)² − 1 shifts
//! per depth, px, py in {0} ∪ the proper divisors of 2^g, mixed
//! shifts included — is rejected: 3, 8, 15, 24, 35, 48 (133
//! cumulative), zero survivors. The Thue–Morse hull is aperiodic;
//! its diffraction spectrum is singular continuous, not pure point
//! (JS99).

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

// ---------------------------------------------------------------------------
// Exact Z² model: rule, refinement
// ---------------------------------------------------------------------------

/// Label class ids: the four label pairs (x, y) ∈ {0, 1}² with class
/// index = 2·x + y.
const L00: u8 = 0;
const L01: u8 = 1;
const L10: u8 = 2;
const L11: u8 = 3;

#[cfg(test)]
/// The 1D Thue–Morse morphism τ: τ(0) = "01", τ(1) = "10",
/// stored as EXP[letter][child position] → child letter (position 0
/// first). τ(w_g) = the depth-(g+1) word from the depth-g word.
/// Test-only: cross-checks RULES against the τ⊗τ derivation.
const EXP: [[u8; 2]; 2] = [
    [0, 1], // 0 → 01
    [1, 0], // 1 → 10
];

/// The pinned τ⊗τ rule: (child class, col, row) in the parent's 2×2
/// inflated frame (frame cell (c, r), row 0 at the top; the child at
/// (c, r) of parent p = 2x + y has class 2·(x ⊕ c) + (y ⊕ r) =
/// 2·EXP[x][c] + EXP[y][r]). Cross-checked against the τ⊗τ
/// derivation in the tests.
const RULES: [[(u8, i64, i64); 4]; 4] = [
    [(L00, 0, 0), (L01, 0, 1), (L10, 1, 0), (L11, 1, 1)],
    [(L01, 0, 0), (L00, 0, 1), (L11, 1, 0), (L10, 1, 1)],
    [(L10, 0, 0), (L11, 0, 1), (L00, 1, 0), (L01, 1, 1)],
    [(L11, 0, 0), (L10, 0, 1), (L01, 1, 0), (L00, 1, 1)],
];

/// Frame of the depth-g patch in unit cells: 2^g × 2^g (square —
/// same block shape as the period-doubling product).
fn patch_dims(g: u32) -> (i64, i64) {
    let a = 1i64 << g;
    (a, a)
}

/// Diagonal of the d = 5 patch block (32 × 32): 32√2 = √2048.
fn patch_diag() -> f64 {
    (2048.0_f64).sqrt()
}
/// Presentation depth floor: the depth rendered whenever the whole
/// patch fits the view (E ≥ 32√2). d = 5 frames the UI preview window
/// self-consistently (see `default_view_fills_pane`).
const D_PRESENT: u32 = 5;
/// Max refinement depth: 4^8 = 65 536 ≤ MAX_TILES < 4^9.
const MAX_DEPTH: u32 = 8;

/// One tile of a depth-g patch: class + (col, row) of the unit cell,
/// exact integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TileCell {
    ty: u8,
    c: i64,
    r: i64,
}

/// Nesting convention (verified against the pinned depth-2 00 block):
/// the child at frame cell (c, r) of the depth-1 rule is replaced by
/// its own depth-(d−1) patch at offset 2^(d−1)·(c, r).
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
    let m = 1i64 << (g - 1);
    RULES[p as usize]
        .iter()
        .flat_map(|&(ty, c, r)| patch_of_at(ty, g - 1, c0 + m * c, r0 + m * r))
        .collect()
}

/// The depth-g patch of class `p`: 4^g unit squares tiling the 2^g ×
/// 2^g block.
fn patch_of(p: u8, g: u32) -> Vec<TileCell> {
    patch_of_at(p, g, 0, 0)
}

// ---------------------------------------------------------------------------
// Tiling
// ---------------------------------------------------------------------------

/// The 2D-product (τ⊗τ) Thue–Morse non-periodic substitution tiling (aperiodic).
pub struct ThueMorseTiling;

impl Tiling for ThueMorseTiling {
    fn id(&self) -> &'static str {
        "thue-morse"
    }

    fn name(&self) -> &'static str {
        "Thue-Morse (2D product) substitution"
    }

    fn description(&self) -> &'static str {
        "2D-product (τ⊗τ) of the 1D Thue–Morse morphism 0 → 01, \
         1 → 10 (Bielefeld entry \"Thue Morse\", reference JS99 = \
         Allouche & Shallit, Springer 1999): one unit-square \
         prototile on the square lattice, four label classes (00, \
         01, 10, 11), λ = 2 substitution into 4 children (all-ones \
         substitution matrix — every parent block contains each \
         child class exactly once, column sums 4 = λ², PF \
         eigenvalue 4 = λ², char poly x⁴−4x³, eigenvalues \
         {4, 0, 0, 0}). Finite covered patch = 2^d × 2^d block \
         recentered on the origin, d = 5..8 picked by the view \
         diagonal (nested views bit-identical on their \
         intersection); exact integer cell arithmetic end to end. \
         Non-periodic — aperiodic long-range order: no proper translation period at any depth g = \
         1..6 (133 candidate shifts rejected), singular continuous \
         diffraction (JS99)."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Aperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "non-periodic substitution tiling; aperiodic long-range order (Thue-Morse 2D product τ⊗τ; Bielefeld entry \
             \"Thue Morse\", 1D rule 0 → 01, 1 → 10, reference \
             JS99 = Allouche & Shallit, Springer 1999); no proper \
             translation period at any depth; singular continuous \
             diffraction (JS99)",
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

        // View-anchored depth: each √2-shrink of the view diagonal
        // past the d = 5 patch diagonal 32√2 adds one refinement
        // level.
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

        // Emission recentering: the 2^d × 2^d block is shifted by
        // half the block, (2^(d−1), 2^(d−1)) — the covered domain is
        // |x| ≤ 2^(d−1), |y| ≤ 2^(d−1) in cell units.
        let (frame, _) = patch_dims(d);
        let sh = frame / 2;
        let patch_box = BBox::new(
            Vec2::new(-sh as f64, -sh as f64),
            Vec2::new(sh as f64, sh as f64),
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

        let cells = patch_of(L00, d);

        // Emission: exact (class, col, row) integers → the recentered
        // unit square, culled to the query region. f64 only here.
        let mut tiles = Vec::new();
        for c in &cells {
            let x0 = c.c - sh;
            let y0 = c.r - sh;
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
    use std::collections::{BTreeSet, HashSet};
    use tess_core::params::ParamValue;

    // --- rule line -------------------------------------------------------

    /// Rule line, all four parents: the four children are the four
    /// distinct 2×2 frame cells — containment + disjointness +
    /// equal-area ⇒ the rule line closes.
    #[test]
    fn rule_line() {
        for p in 0..4u8 {
            let mut cells: Vec<(i64, i64)> =
                RULES[p as usize].iter().map(|&(_, c, r)| (c, r)).collect();
            cells.sort();
            cells.dedup();
            assert_eq!(
                cells,
                vec![(0, 0), (0, 1), (1, 0), (1, 1)],
                "p={p}: children must be the four distinct frame cells"
            );
            for &(ty, c, r) in &RULES[p as usize] {
                assert!(ty < 4, "p={p}: child class out of range");
                assert!(
                    c >= 0 && c < 2 && r >= 0 && r < 2,
                    "p={p}: child outside the 2×2 frame"
                );
            }
        }
    }

    /// RULES == the τ⊗τ derivation: the child at frame cell (c, r) of
    /// parent p = 2x + y has class 2·(x ⊕ c) + (y ⊕ r) =
    /// 2·EXP[x][c] + EXP[y][r] — the full (class, col, row) tuples,
    /// in frame-cell order (k = 2·c + r).
    #[test]
    fn product_rule_pin() {
        for p in 0..4u8 {
            let x = (p / 2) as usize;
            let y = (p % 2) as usize;
            let mut want: Vec<(u8, i64, i64)> = Vec::new();
            for k in 0..4usize {
                let c = (k / 2) as i64;
                let r = (k % 2) as i64;
                let cls = 2 * EXP[x][c as usize] + EXP[y][r as usize];
                want.push((cls, c, r));
            }
            let got: Vec<(u8, i64, i64)> = RULES[p as usize].to_vec();
            assert_eq!(got, want, "parent {p} (x={x}, y={y}): pinned τ⊗τ rule");
        }
    }

    // --- substitution matrix + growth -----------------------------------

    /// S derived from RULES equals the pinned all-ones matrix; column
    /// sums = 4 = λ²; S^g·e_00 pinned at g = 1..4 (= 4^(g−1) per
    /// class, total 4^g at g = 1..8), with the refine() census
    /// agreeing at every level.
    #[test]
    fn substitution_matrix_and_counts() {
        let mut s = [[0i64; 4]; 4];
        for p in 0..4usize {
            for &(ty, _, _) in &RULES[p] {
                s[ty as usize][p] += 1;
            }
        }
        assert_eq!(
            s,
            [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]],
            "S[j][i] (child j, parent i) — all-ones (Steinhaus)"
        );
        for i in 0..4usize {
            assert_eq!(
                s[0][i] + s[1][i] + s[2][i] + s[3][i],
                4,
                "column {i} sum = λ²"
            );
        }
        let pins: [Option<[i64; 4]>; 8] = [
            Some([1, 1, 1, 1]),
            Some([4, 4, 4, 4]),
            Some([16, 16, 16, 16]),
            Some([64, 64, 64, 64]),
            None,
            None,
            None,
            None,
        ];
        let mut c = [1i64, 0, 0, 0];
        for g in 1..=8usize {
            c = [
                s[0][0] * c[0] + s[0][1] * c[1] + s[0][2] * c[2] + s[0][3] * c[3],
                s[1][0] * c[0] + s[1][1] * c[1] + s[1][2] * c[2] + s[1][3] * c[3],
                s[2][0] * c[0] + s[2][1] * c[1] + s[2][2] * c[2] + s[2][3] * c[3],
                s[3][0] * c[0] + s[3][1] * c[1] + s[3][2] * c[2] + s[3][3] * c[3],
            ];
            assert_eq!(
                c[0] + c[1] + c[2] + c[3],
                1i64 << (2 * g),
                "g={g}: total = 4^g"
            );
            if let Some(pin) = pins[g - 1] {
                assert_eq!(c, pin, "S^g·e_00 at g = {g}");
            }
            // refine() census cross-check
            let mut census = [0i64; 4];
            for t in &patch_of(L00, g as u32) {
                census[t.ty as usize] += 1;
            }
            assert_eq!(census, c, "depth-{g} refine() census");
        }
    }

    /// The pinned all-ones 4×4 matrix. (S[i][i] = 1: the xI − S
    /// diagonal is `x − S[i][i]`, not `x`.)
    const S: [[i64; 4]; 4] = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]];

    /// xI − S: diagonal x − S[i][i], off-diagonal −S[i][j].
    fn xi_minus_s(x: i64) -> [[i64; 4]; 4] {
        let mut m = [[0i64; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                m[i][j] = if i == j { x - S[i][j] } else { -S[i][j] };
            }
        }
        m
    }

    /// 24-permutation expansion of a 4×4 determinant, returning the
    /// value and the number of visited permutations (the M135 lesson:
    /// a truncated expansion silently returns a wrong det).
    fn det4(m: [[i64; 4]; 4]) -> (i64, u32) {
        let mut total = 0i64;
        let mut cnt = 0u32;
        for p0 in 0..4 {
            for p1 in 0..4 {
                if p1 == p0 {
                    continue;
                }
                for p2 in 0..4 {
                    if p2 == p0 || p2 == p1 {
                        continue;
                    }
                    let p3 = 6 - p0 - p1 - p2; // the remaining element
                    let perm = [p0, p1, p2, p3];
                    let mut inv = 0;
                    for i in 0..4 {
                        for j in (i + 1)..4 {
                            if perm[i] > perm[j] {
                                inv += 1;
                            }
                        }
                    }
                    let sign = if inv % 2 == 0 { 1 } else { -1 };
                    let mut prod = 1i64;
                    for i in 0..4 {
                        prod *= m[i][perm[i] as usize];
                    }
                    total += sign * prod;
                    cnt += 1;
                }
            }
        }
        (total, cnt)
    }

    /// Characteristic polynomial det(xI − S) = x⁴ − 4x³ — exact
    /// 24-permutation determinants at x = 0..6 (pinned 0, −3, −16,
    /// −27, 0, 125, 432): eigenvalues {4, 0, 0, 0} (rank-1 all-ones
    /// matrix), trace 4, det 0; PF eigenvalue 4 (det(4I − S) = 0).
    #[test]
    fn char_poly_and_pf() {
        let pin = [0i64, -3, -16, -27, 0, 125, 432];
        for x in 0..=6i64 {
            let (d, cnt) = det4(xi_minus_s(x));
            assert_eq!(cnt, 24, "full 24-permutation expansion at x = {x}");
            assert_eq!(d, pin[x as usize], "det(xI−S) at x = {x}");
            // Cross-check the closed form x⁴ − 4x³.
            let poly = x * x * x * x - 4 * x * x * x;
            assert_eq!(d, poly, "char poly closed form at x = {x}");
        }
        assert_eq!(pin[0], 0, "det S = 0 (rank-1 all-ones matrix)");
        assert_eq!(pin[4], 0, "det(4I−S) = 0: PF eigenvalue 4 = λ²");
        let trace: i64 = (0..4).map(|i| S[i][i]).sum();
        assert_eq!(trace, 4, "trace S = 4 + 0 + 0 + 0 = 4");
    }

    // --- depth-2 block reproduction --------------------------------------

    /// The pinned depth-2 00 patch (16 cells, row-major; row 0 at the
    /// top) — the same 16-cell readout the Python battery pinned
    /// before Rust, census 4/4/4/4.
    const BLOCK2: [(u8, i64, i64); 16] = [
        // row 0: 00 10 10 00
        (L00, 0, 0),
        (L10, 1, 0),
        (L10, 2, 0),
        (L00, 3, 0),
        // row 1: 01 11 11 01
        (L01, 0, 1),
        (L11, 1, 1),
        (L11, 2, 1),
        (L01, 3, 1),
        // row 2: 01 11 11 01
        (L01, 0, 2),
        (L11, 1, 2),
        (L11, 2, 2),
        (L01, 3, 2),
        // row 3: 00 10 10 00
        (L00, 0, 3),
        (L10, 1, 3),
        (L10, 2, 3),
        (L00, 3, 3),
    ];

    #[test]
    fn depth2_block_reproduction() {
        let mut gen: Vec<(u8, i64, i64)> =
            patch_of(L00, 2).iter().map(|t| (t.ty, t.c, t.r)).collect();
        let mut want = BLOCK2.to_vec();
        gen.sort();
        want.sort();
        assert_eq!(gen, want, "depth-2 00 block");
        assert_eq!(gen.len(), 16, "depth-2 count = 4²");
        let mut census = [0i64; 4];
        for &(ty, _, _) in &gen {
            census[ty as usize] += 1;
        }
        assert_eq!(census, [4, 4, 4, 4], "depth-2 census");
    }

    // --- word formula ------------------------------------------------------

    /// The depth-g word: w_0 = "0" (letter 0), w_{g+1} = τ(w_g).
    fn word(g: u32) -> Vec<u8> {
        let mut w = vec![0u8];
        for _ in 0..g {
            let mut nw = Vec::with_capacity(w.len() * 2);
            for &sym in &w {
                nw.push(EXP[sym as usize][0]);
                nw.push(EXP[sym as usize][1]);
            }
            w = nw;
        }
        w
    }

    /// The closed-form label grid: cell (c, r) of the depth-g block
    /// has class 2·w_g[c] + w_g[r] (equivalently w_g[c] ⊕ w_g[r]
    /// parity-of-sum — the Thue–Morse property) — the refinement tree
    /// must agree with the word formula at every cell, g = 1..5.
    #[test]
    fn word_formula_matches_refinement() {
        for g in 1..=5u32 {
            let w = word(g);
            assert_eq!(w.len(), 1usize << g, "depth-{g} word length");
            for t in &patch_of(L00, g) {
                let want = 2 * w[t.c as usize] + w[t.r as usize];
                assert_eq!(
                    t.ty, want,
                    "depth-{g} cell ({}, {}): class {} != word formula {want}",
                    t.c, t.r, t.ty
                );
            }
        }
    }

    // --- no overlap + coverage --------------------------------------------

    /// Exact single coverage of the 2^g × 2^g frame (0 overlaps, 0
    /// gaps, pinned tile counts 4/16/64/256 at g = 1..4).
    #[test]
    #[allow(non_snake_case)] // L = the frame side, the paper's notation
    fn no_overlap_and_coverage_depths() {
        let pin_counts = [4, 16, 64, 256];
        for g in 1..=4usize {
            let tiles = patch_of(L00, g as u32);
            let (L, _) = patch_dims(g as u32);
            assert_eq!(tiles.len(), pin_counts[g - 1], "depth-{g} count = 4^g");
            // Cell-level exact single coverage: every frame cell owned
            // by exactly one tile.
            let mut owner: Vec<Option<usize>> = vec![None; (L * L) as usize];
            for (i, t) in tiles.iter().enumerate() {
                assert!(
                    t.c >= 0 && t.c + 1 <= L && t.r >= 0 && t.r + 1 <= L,
                    "tile {i} outside the frame"
                );
                let k = (t.r * L + t.c) as usize;
                assert!(owner[k].is_none(), "overlap at cell ({}, {})", t.c, t.r);
                owner[k] = Some(i);
            }
            assert!(
                owner.iter().all(|o| o.is_some()),
                "g={g}: gaps in the frame"
            );
        }
    }

    // --- no-period manifest -------------------------------------------------

    /// The aperiodic signature: the class labeling of the depth-g patch
    /// carries **no proper translation period** at any depth. A shift
    /// (px, py) is a candidate period of the L × L block only if
    /// px | L and py | L (the full-block filter: the period must tile
    /// the whole block); in [0, L) coordinates the candidates are
    /// px, py ∈ {0} ∪ the proper divisors of L = 2^g — (g+1)² − 1
    /// shifts, mixed shifts included. The depth-g labeling is
    /// invariant under none of them: every candidate (px, py) ≠
    /// (0, 0) moves the class-0 (0, 0) cell to class
    /// 2·w_g[px] + w_g[py] ≠ 0, since w_g[2^k] = 1 for
    /// 0 < 2^k < L. Returns (rejected candidate count, survivors).
    #[allow(non_snake_case)] // L = the frame side, the paper's notation
    fn period_manifest(g: u32) -> (usize, Vec<(i64, i64)>) {
        let tiles = patch_of(L00, g);
        let (L, _) = patch_dims(g);
        let mut grid: Vec<u8> = vec![0; (L * L) as usize];
        for t in &tiles {
            grid[(t.r * L + t.c) as usize] = t.ty;
        }
        // Candidate coordinates: 0 plus the proper divisors of 2^g.
        let coords: Vec<i64> = std::iter::once(0i64)
            .chain((0..g).map(|k| 1i64 << k))
            .collect();
        let mut rejected = 0usize;
        let mut survivors = Vec::new();
        for &px in &coords {
            for &py in &coords {
                if px == 0 && py == 0 {
                    continue;
                }
                let mut ok = true;
                for r in 0..L {
                    if !ok {
                        break;
                    }
                    let r2 = (r + py) % L;
                    for c in 0..L {
                        if grid[(r2 * L + c) as usize] != grid[(r * L + (c + px) % L) as usize] {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    survivors.push((px, py));
                } else {
                    rejected += 1;
                }
            }
        }
        (rejected, survivors)
    }

    /// The measured no-period manifest, g = 1..6: zero survivors at
    /// every depth — the Thue–Morse structure is aperiodic at every
    /// depth (unlike the period-doubling patch, which is the
    /// fundamental cell of the period-(2^g, 2^g) approximant).
    /// Rejected candidate-shift counts pinned per depth:
    /// (g+1)² − 1 = 3, 8, 15, 24, 35, 48 (133 cumulative).
    #[test]
    fn no_period_manifest() {
        let count_pins = [4, 16, 64, 256, 1024, 4096];
        let rej_pins = [3usize, 8, 15, 24, 35, 48];
        for g in 1..=6u32 {
            let tiles = patch_of(L00, g);
            assert_eq!(tiles.len(), count_pins[g as usize - 1], "g={g} count");
            let (rejected, survivors) = period_manifest(g);
            assert!(
                survivors.is_empty(),
                "g={g}: surviving proper period {survivors:?} — a measured \
                 period at any depth flips the aperiodic classification"
            );
            assert_eq!(
                rejected,
                rej_pins[g as usize - 1],
                "g={g}: rejected candidate-shift count"
            );
        }
    }

    // --- nested views --------------------------------------------------------

    fn gen_at(region: BBox) -> TilingResult {
        let t = ThueMorseTiling;
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

    /// All five windows sit in the d = 7 band (E² ∈ [0.125, 0.5) —
    /// pinned per window) and emit the same 4^7 patch recentered on
    /// the origin: every tile of a smaller view must reappear
    /// bit-identically in each larger one containing it. True
    /// containments only: B5 ⊂ B1, B1 ⊂ B2, B4 ⊂ B2 (B4 and B3 stick
    /// out of their near-neighbors — no containment asserted for B3).
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
                let s = win.size();
                let e2 = s.x * s.x + s.y * s.y;
                assert!(
                    (0.125..0.5).contains(&e2),
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

    // --- default view --------------------------------------------------------

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Pins both default presentations against the degenerate-floor
    /// bug (a d = 5 view renders 1024 tiles, never the whole tiling
    /// as one patch):
    ///
    /// (a) The UI/preview convention: a ±10 probe (d = 5: E² = 800 ≥
    ///     2), then `preview_window(mean tile side, 4/3)` (base =
    ///     9 × mean side = 9.0). Every unit square has AABB sides
    ///     (1, 1), so the mean side is 1.0 and the derived window
    ///     (±6 × ±4.5, E² = 225) stays in the d = 5 band, non-empty
    ///     with all four label classes, and the second probe round is
    ///     a fixed point (all tiles have side (1, 1), so no culling
    ///     shifts the mix).
    /// (b) The CLI convention: a ±(40/3) × ±10 origin-centered pane —
    ///     d = 5, non-empty (the d = 5 block |x|, |y| ≤ 16 covers the
    ///     pane).
    #[test]
    fn default_view_fills_pane() {
        let t = ThueMorseTiling;
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
        let r = t.generate(&p, &Query { region: win });
        assert!(r.message.is_none());
        assert!(!r.tiles.is_empty(), "preview window must be non-empty");
        let s = win.size();
        assert!(
            s.x * s.x + s.y * s.y >= 2.0,
            "derived window E² = {} left the d = 5 band",
            s.x * s.x + s.y * s.y
        );
        assert_eq!(
            r.tiles
                .iter()
                .map(|tl| tl.seed)
                .collect::<BTreeSet<u32>>()
                .len(),
            4,
            "preview must show all four label classes"
        );

        // fixed point: the second round re-derives the same window
        let mw2 = tess_core::app::mean_tile_side(&r.tiles);
        let win2 = tess_core::app::preview_window(r.periodic, mw2, 4.0 / 3.0);
        let r2 = t.generate(&p, &Query { region: win2 });
        assert!(r2.message.is_none());
        assert!(!r2.tiles.is_empty(), "second round must be non-empty");
        let s2 = win2.size();
        assert!(
            s2.x * s2.x + s2.y * s2.y >= 2.0,
            "second-round window left the d = 5 band"
        );
        assert_eq!(
            r2.tiles
                .iter()
                .map(|tl| tl.seed)
                .collect::<BTreeSet<u32>>()
                .len(),
            4,
            "second round shows only some label classes"
        );

        // (b) CLI pane ±(40/3) × ±10
        let region = BBox::new(v2(-40.0 / 3.0, -10.0), v2(40.0 / 3.0, 10.0));
        let rc = t.generate(&p, &Query { region });
        assert!(rc.message.is_none());
        assert!(!rc.tiles.is_empty(), "CLI pane must be non-empty");
    }

    // --- background -----------------------------------------------------------

    /// A far view beyond the covered domain of its picked depth
    /// returns background (no tiles, no refusal message): E² = 0.5 →
    /// d = 6, covered domain |x|, |y| ≤ 32 — the view at (100, 100)
    /// is far outside.
    #[test]
    fn far_view_is_background() {
        let r = gen_at(BBox::new(v2(100.0, 100.0), v2(100.5, 100.5)));
        assert!(
            r.message.is_none(),
            "background must not set a refusal message"
        );
        assert!(r.tiles.is_empty(), "far view must be empty (background)");
    }
}
