//! Domino (table) — Lançon's non-periodic substitution tiling (limit-periodic).
//!
//! Lançon's domino substitution (Bielefeld Tilings Encyclopedia entry
//! "domino"; the entry says verbatim "Also known as 'table tiling.'",
//! and no independent "table" entry exists in the substitution index —
//! "table" is an alias of this one tiling). The two prototiles are
//! dominoes on the integer square lattice: A = vertical 1×2 (amber in
//! the entry's figure) and B = horizontal 2×1 (blue). Each parent is
//! inflated by λ = 2, orientation-preserving (no rotations or
//! reflections in the rule — the entry's "Finite Rotations" tag is the
//! identity class {id}) and subdivided into **four children**: the
//! A-frame is 2×4, the B-frame 4×2 (the pinned rule below, read off
//! the entry's figure; its depth-2 blocks are reproduced exactly in
//! the tests). Entry tags: Finite Rotations {id}, Polyomio Tiling,
//! Rep-Tiles, Self-Similar Substitution; discovered by Frédéric Lançon.
//!
//! **Exact arithmetic.** Tiles are unit-cell rectangles on Z²: every
//! placement is an exact integer triple (type, col, row) with
//! footprint (1, 2) or (2, 1). No radical ever touches the geometry;
//! f64 appears only at the view/emission boundary, where the cell
//! rectangles pass through the similarity map.
//!
//! **Substitution.** A depth-g patch of A (resp. B) is 4^g tiles
//! tiling a 2^g × 2^(g+1) (resp. 2^(g+1) × 2^g) unit-cell block. The
//! substitution matrix S = [[2, 2], [2, 2]] (S[j][i] = children of
//! type j in parent i) has column sums 4 = λ², PF eigenvalue 4 = λ²
//! (λ = 2) and characteristic polynomial det(xI − S) = x² − 4x =
//! x(x − 4) — eigenvalues {4, 0} — asserted directly by exact 2×2
//! determinants in the tests (no permutation expansion needed at 2×2).
//!
//! **Generation (finite covered-patch model, half-hex precedent).**
//! The seed is the unit A-domino; depth d is view-anchored: with E the
//! inverse-mapped view diagonal and the d = 5 patch block 32 × 64
//! (diagonal 32√5 = √5120), zoom = 0 for E ≥ 32√5 else
//! ceil(log2(32√5/E)), d = clamp(zoom, 5, 8) (4^8 = 65 536 ≤
//! MAX_TILES). Depth bands in E²: d = 5 for E² ≥ 5, d = 6 for
//! [1.25, 5), d = 7 for [0.3125, 1.25), d = 8 below. Nested views
//! within one band emit bit-identical tiles on their intersection
//! (`nested_view_bit_identical`).
//!
//! **Emission recentering.** The depth-d A-patch spans the 2^d ×
//! 2^(d+1) block; it is recentered on the origin by the constant shift
//! (2^(d−1), 2^d) at emission: the covered domain is |x| ≤ 2^(d−1),
//! |y| ≤ 2^d in cell units. Beyond that block the view is background.
//!
//! **Limit-periodicity (measured, not asserted).** The depth-g patch
//! carries **no proper translation period at any depth** — it is
//! itself the fundamental cell of the period-(2^g, 2^(g+1))
//! *approximating* periodic tiling: measured over g = 1..6 (Rust
//! battery: g = 2..6), the minimal patch period is (2^g, 2^(g+1)) —
//! (2, 4), (4, 8), (8, 16), (16, 32), (32, 64), (64, 128) — the full
//! block, growing 2× per level (`period_hierarchy`). This is
//! limit-periodicity via **approximating periodic tilings** — a
//! different kind from half-hex, whose depth-g patch is itself
//! (3/2)·2^g-periodic. It is **not** a cut-and-project or model-set
//! structure: the entry states that the dynamical spectrum of the
//! tiling has a continuous component, "Thus it cannot be a cut and
//! project tiling" — Solomyak 1997, ETDS 17(3), 695–738 ("Dynamics of
//! self-similar tilings"); the same structure for the table and chair
//! tilings was shown in Robinson 1999, Indag. Math. 10(4), 581–599
//! ("On the table and the chair").

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

// ---------------------------------------------------------------------------
// Exact Z² model: rule, refinement
// ---------------------------------------------------------------------------

/// Prototile ids: A = vertical domino (1 wide × 2 tall), B =
/// horizontal domino (2 wide × 1 tall), in unit cells.
const A: u8 = 0;
const B: u8 = 1;

/// The pinned substitution rule (child, col, row, w, h) in unit cells
/// of the parent's λ-inflated frame (frame 2×4 for A, 4×2 for B; row 0
/// at the top, the figure's convention). Cross-checked against the
/// entry figure's depth-2 blocks and the exact growth battery in the
/// tests.
const RULES: [[(u8, i64, i64, i64, i64); 4]; 2] = [
    [
        (B, 0, 0, 2, 1),
        (A, 0, 1, 1, 2),
        (A, 1, 1, 1, 2),
        (B, 0, 3, 2, 1),
    ],
    [
        (A, 0, 0, 1, 2),
        (B, 1, 0, 2, 1),
        (B, 1, 1, 2, 1),
        (A, 3, 0, 1, 2),
    ],
];

/// Footprint (w, h) in unit cells of prototile `ty`.
fn footprint(ty: u8) -> (i64, i64) {
    match ty {
        A => (1, 2),
        _ => (2, 1),
    }
}

/// Frame of the depth-g patch of prototile `p` in unit cells:
/// 2^g × 2^(g+1) for A, 2^(g+1) × 2^g for B.
fn patch_dims(p: u8, g: u32) -> (i64, i64) {
    let a = 1i64 << g;
    match p {
        A => (a, 2 * a),
        _ => (2 * a, a),
    }
}

/// Diagonal of the d = 5 patch block (32 × 64): 32√5 = √5120.
fn patch_diag() -> f64 {
    (5120.0_f64).sqrt()
}
/// Presentation depth floor: the depth rendered whenever the whole
/// patch fits the view (E ≥ 32√5). d = 5 frames the UI preview window
/// self-consistently (see `default_view_fills_pane`).
const D_PRESENT: u32 = 5;
/// Max refinement depth: 4^8 = 65 536 ≤ MAX_TILES < 4^9.
const MAX_DEPTH: u32 = 8;

/// One tile of a depth-g patch: type + (col, row) of the tile's
/// top-left unit cell, exact integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TileCell {
    ty: u8,
    c: i64,
    r: i64,
}

/// Nesting convention (verified against the figure's depth-2 blocks):
/// the child at frame cell (c, r) of the depth-1 rule is replaced by
/// its own depth-(d−1) patch at offset 2^(d−1)·(c, r).
fn patch_of_at(p: u8, g: u32, c0: i64, r0: i64) -> Vec<TileCell> {
    if g == 1 {
        return RULES[p as usize]
            .iter()
            .map(|&(ty, c, r, _, _)| TileCell {
                ty,
                c: c0 + c,
                r: r0 + r,
            })
            .collect();
    }
    let m = 1i64 << (g - 1);
    RULES[p as usize]
        .iter()
        .flat_map(|&(ty, c, r, _, _)| patch_of_at(ty, g - 1, c0 + m * c, r0 + m * r))
        .collect()
}

/// The depth-g patch of prototile `p`: 4^g tiles tiling the 2^g ×
/// 2^(g+1) (A) / 2^(g+1) × 2^g (B) block.
fn patch_of(p: u8, g: u32) -> Vec<TileCell> {
    patch_of_at(p, g, 0, 0)
}

// ---------------------------------------------------------------------------
// Tiling
// ---------------------------------------------------------------------------

/// Lançon's domino (table) non-periodic substitution tiling (limit-periodic).
pub struct DominoTiling;

impl Tiling for DominoTiling {
    fn id(&self) -> &'static str {
        "domino"
    }

    fn name(&self) -> &'static str {
        "Domino (table) substitution"
    }

    fn description(&self) -> &'static str {
        "Lançon's domino (table) non-periodic substitution tiling (Bielefeld entry \
         \"domino\", also known as the table tiling): A (1×2 vertical) \
         and B (2×1 horizontal) dominoes on the square lattice, λ = 2 \
         substitution into 4 children (substitution matrix [[2,2],[2,2]], \
         PF eigenvalue 4 = λ², char poly x²−4x). Finite covered patch = \
         2^d × 2^(d+1) block recentered on the origin, d = 5..8 picked \
         by the view diagonal (nested views bit-identical on their \
         intersection); exact integer cell arithmetic end to end. \
         Limit-periodic (long-range order) via approximating periodic tilings: the depth-g \
         patch is the fundamental cell of the period-(2^g, 2^(g+1)) \
         approximant — measured, no proper within-patch period \
         (g = 1..6); not cut-and-project (Solomyak 1997 continuous \
         spectral component; Robinson 1999)."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Aperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "non-periodic substitution tiling; limit-periodic (Lançon domino / table; Bielefeld entry \
             \"domino\", also known as the table tiling); approximating \
             periods (2^d, 2^(d+1)); NOT cut-and-project / model set — \
             Solomyak ETDS 17(3) 1997, Robinson Indag. Math. 10(4) 1999",
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

        // View-anchored depth: each √2-shrink of the view diagonal past
        // the d = 5 patch diagonal 32√5 adds one refinement level.
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

        // Emission recentering: the 2^d × 2^(d+1) block is shifted by
        // half the frame, (2^(d−1), 2^d) — the covered domain is
        // |x| ≤ 2^(d−1), |y| ≤ 2^d in cell units.
        let (frame_w, frame_h) = patch_dims(A, d);
        let sh_x = frame_w / 2;
        let sh_y = frame_h / 2;
        let patch_box = BBox::new(
            Vec2::new(-sh_x as f64, -sh_y as f64),
            Vec2::new(sh_x as f64, sh_y as f64),
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

        // Emission: exact (type, col, row) integers → the recentered
        // cell rectangle, culled to the query region. f64 only here.
        let mut tiles = Vec::new();
        for c in &cells {
            let (w, h) = footprint(c.ty);
            let x0 = c.c - sh_x;
            let y0 = c.r - sh_y;
            let poly = Polygon::new(vec![
                Vec2::new(x0 as f64, y0 as f64),
                Vec2::new((x0 + w) as f64, y0 as f64),
                Vec2::new((x0 + w) as f64, (y0 + h) as f64),
                Vec2::new(x0 as f64, (y0 + h) as f64),
            ])
            .transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto: 0,
                    seed: c.ty as u32,
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

    /// Rule line, both orientations: the four children are unit-cell
    /// rectangles contained in the 2×4 / 4×2 inflated frame, pairwise
    /// interior-disjoint, areas summing to the frame (8 = λ² · 2) —
    /// containment + disjointness + equal-area ⇒ the rule line closes.
    #[test]
    fn rule_line() {
        for p in [A, B] {
            let (frame_w, frame_h) = patch_dims(p, 1);
            let rects: Vec<(i64, i64, i64, i64)> = RULES[p as usize]
                .iter()
                .map(|&(_, c, r, w, h)| (c, r, c + w, r + h))
                .collect();
            for (x0, y0, x1, y1) in &rects {
                assert!(
                    *x0 >= 0 && *x1 <= frame_w && *y0 >= 0 && *y1 <= frame_h,
                    "p={p}: child outside the frame"
                );
                assert_eq!((*x1 - *x0) * (*y1 - *y0), 2, "child area");
            }
            for i in 0..4 {
                for j in (i + 1)..4 {
                    let a = rects[i];
                    let b = rects[j];
                    let ox = a.2.min(b.2) - a.0.max(b.0);
                    let oy = a.3.min(b.3) - a.1.max(b.1);
                    assert!(ox <= 0 || oy <= 0, "p={p}: children {i} and {j} overlap");
                }
            }
            let area: i64 = rects
                .iter()
                .map(|&(x0, y0, x1, y1)| (x1 - x0) * (y1 - y0))
                .sum();
            assert_eq!(
                area,
                frame_w * frame_h,
                "p={p}: children areas must fill the frame"
            );
        }
    }

    // --- substitution matrix + growth -----------------------------------

    /// S derived from RULES equals the pinned [[2,2],[2,2]]; column
    /// sums = 4 = λ²; S^g·e_A = (2^(2g−1), 2^(2g−1)), total 4^g, and
    /// the refine() census agrees at every level g = 1..8.
    #[test]
    fn substitution_matrix_and_counts() {
        let mut s = [[0i64; 2]; 2];
        for p in 0..2usize {
            for &(ty, _, _, _, _) in &RULES[p] {
                s[ty as usize][p] += 1;
            }
        }
        assert_eq!(s, [[2, 2], [2, 2]], "S[j][i] (child j, parent i)");
        for i in 0..2usize {
            assert_eq!(s[0][i] + s[1][i], 4, "column {i} sum = λ²");
        }
        let mut c = [1i64, 0];
        for g in 1..=8usize {
            c = [
                s[0][0] * c[0] + s[0][1] * c[1],
                s[1][0] * c[0] + s[1][1] * c[1],
            ];
            assert_eq!(c, [1i64 << (2 * g - 1), 1i64 << (2 * g - 1)], "S^g·e_A");
            assert_eq!(c[0] + c[1], 1i64 << (2 * g), "total = 4^g");
            // refine() census cross-check
            let mut census = [0i64; 2];
            for t in &patch_of(A, g as u32) {
                census[t.ty as usize] += 1;
            }
            assert_eq!(census, c, "depth-{g} refine() census");
        }
    }

    /// Characteristic polynomial det(xI − S) = x² − 4x — exact 2×2
    /// determinants at sample x (no permutation expansion needed at
    /// 2×2): eigenvalues {4, 0} (trace 4, det 0), PF eigenvalue 4 =
    /// λ² with λ = 2.
    #[test]
    fn char_poly_and_pf() {
        let det = |m: [[i64; 2]; 2]| m[0][0] * m[1][1] - m[0][1] * m[1][0];
        assert_eq!(det([[2, 2], [2, 2]]), 0, "det S = 0");
        assert_eq!(2 + 2, 4, "trace = PF eigenvalue = λ² = 4");
        for x in [0i64, 3, 5, 7] {
            let m = x - 2;
            assert_eq!(
                det([[m, -2], [-2, m]]),
                x * x - 4 * x,
                "det(xI−S) = x²−4x at x = {x}"
            );
        }
    }

    // --- depth-2 figure reproduction --------------------------------------

    /// The entry figure's depth-2 blocks (bottom-left 4×8 = A-patch,
    /// bottom-right 8×4 = B-patch), transcribed cell by cell from
    /// domino_rule.gif — the same 16-tile readouts the Python battery
    /// pinned before Rust.
    const FIG_A: [(u8, i64, i64); 16] = [
        (A, 0, 0),
        (B, 1, 0),
        (A, 3, 0),
        (B, 1, 1),
        (B, 0, 2),
        (B, 2, 2),
        (A, 0, 3),
        (A, 1, 3),
        (A, 2, 3),
        (A, 3, 3),
        (B, 0, 5),
        (B, 2, 5),
        (A, 0, 6),
        (B, 1, 6),
        (A, 3, 6),
        (B, 1, 7),
    ];
    const FIG_B: [(u8, i64, i64); 16] = [
        (B, 0, 0),
        (A, 2, 0),
        (B, 3, 0),
        (A, 5, 0),
        (B, 6, 0),
        (A, 0, 1),
        (A, 1, 1),
        (B, 3, 1),
        (A, 6, 1),
        (A, 7, 1),
        (A, 2, 2),
        (B, 3, 2),
        (A, 5, 2),
        (B, 0, 3),
        (B, 3, 3),
        (B, 6, 3),
    ];

    #[test]
    fn depth2_figure_reproduction() {
        let mut gen_a: Vec<(u8, i64, i64)> =
            patch_of(A, 2).iter().map(|t| (t.ty, t.c, t.r)).collect();
        let mut want_a = FIG_A.to_vec();
        gen_a.sort();
        want_a.sort();
        assert_eq!(gen_a, want_a, "depth-2 A-patch = figure bottom-left block");
        let mut gen_b: Vec<(u8, i64, i64)> =
            patch_of(B, 2).iter().map(|t| (t.ty, t.c, t.r)).collect();
        let mut want_b = FIG_B.to_vec();
        gen_b.sort();
        want_b.sort();
        assert_eq!(gen_b, want_b, "depth-2 B-patch = figure bottom-right block");
        assert_eq!(gen_a.len(), 16, "depth-2 A-patch count");
        assert_eq!(gen_b.len(), 16, "depth-2 B-patch count");
    }

    // --- no overlap + coverage -------------------------------------------

    /// Exact single coverage of the 2^g × 2^(g+1) frame (0 overlaps, 0
    /// gaps, pinned tile counts 4/16/64/256 at g = 1..4) plus the fine
    /// 1/4-cell grid: every in-frame grid point is covered by exactly
    /// one tile (pinned 128/512/2048/8192 in-frame points).
    #[test]
    fn no_overlap_and_coverage_depths() {
        let pin_counts = [4, 16, 64, 256];
        let pin_grids = [128, 512, 2048, 8192];
        for g in 1..=4usize {
            let tiles = patch_of(A, g as u32);
            let (frame_w, frame_h) = patch_dims(A, g as u32);
            assert_eq!(tiles.len(), pin_counts[g - 1], "depth-{g} count = 4^g");
            // Cell-level exact single coverage: every frame cell owned
            // by exactly one tile.
            let mut owner: Vec<Option<usize>> = vec![None; (frame_w * frame_h) as usize];
            for (i, t) in tiles.iter().enumerate() {
                let (w, h) = footprint(t.ty);
                assert!(
                    t.c >= 0 && t.c + w <= frame_w && t.r >= 0 && t.r + h <= frame_h,
                    "tile {i} outside the frame"
                );
                for cc in t.c..(t.c + w) {
                    for rr in t.r..(t.r + h) {
                        let k = (rr * frame_w + cc) as usize;
                        assert!(owner[k].is_none(), "overlap at cell ({cc}, {rr})");
                        owner[k] = Some(i);
                    }
                }
            }
            assert!(
                owner.iter().all(|o| o.is_some()),
                "g={g}: gaps in the frame"
            );
            // Fine 1/4-cell grid: the point (ii/4 + 1/8, jj/4 + 1/8)
            // is strictly interior to its unit cell, whose owner is
            // the only tile that can contain it — verified in exact
            // eighths.
            let mut in_region = 0usize;
            for ii in 0..(4 * frame_w) {
                for jj in 0..(4 * frame_h) {
                    in_region += 1;
                    let (cc, rr) = (ii / 4, jj / 4);
                    let i = owner[(rr * frame_w + cc) as usize].unwrap();
                    let t = &tiles[i];
                    let (w, h) = footprint(t.ty);
                    let ox8 = 2 * ii + 1 - 8 * t.c; // in-tile x, in eighths
                    let oy8 = 2 * jj + 1 - 8 * t.r; // in-tile y, in eighths
                    assert!(
                        ox8 > 0 && ox8 < 8 * w && oy8 > 0 && oy8 < 8 * h,
                        "g={g}: grid point ({ii}, {jj}) not covered by its owner"
                    );
                }
            }
            assert_eq!(in_region, pin_grids[g - 1], "g={g}: in-frame grid points");
        }
    }

    // --- period hierarchy --------------------------------------------------

    /// The minimal translation period of the (type, in-tile offset)
    /// labeling of the depth-g A-patch: a shift (p, q) leaves the
    /// labeling invariant iff every cell (c, r) and (c+p, r) (resp.
    /// (c, r+q)) agree on type and on in-tile offset shifted by the
    /// translation — the same measurement as the Python battery. The
    /// depth-g patch has no proper period: the minimal period is the
    /// full block (2^g, 2^(g+1)), the fundamental cell of the
    /// period-(2^g, 2^(g+1)) approximating periodic tiling, strictly
    /// 2× growing per level.
    fn patch_period(g: u32) -> (i64, i64) {
        let tiles = patch_of(A, g);
        let (frame_w, frame_h) = patch_dims(A, g);
        let mut grid: Vec<[i64; 3]> = vec![[0, 0, 0]; (frame_w * frame_h) as usize];
        for t in &tiles {
            let (w, h) = footprint(t.ty);
            for cc in t.c..(t.c + w) {
                for rr in t.r..(t.r + h) {
                    grid[(rr * frame_w + cc) as usize] = [i64::from(t.ty), cc - t.c, rr - t.r];
                }
            }
        }
        let mut px = 0i64;
        for p in 1..=frame_w {
            let mut ok = true;
            'x: for c in 0..(frame_w - p) {
                for r in 0..frame_h {
                    let a = &grid[(r * frame_w + c) as usize];
                    let b = &grid[(r * frame_w + c + p) as usize];
                    if a[0] != b[0] || a[1] + p != b[1] || a[2] != b[2] {
                        ok = false;
                        break 'x;
                    }
                }
            }
            if ok {
                px = p;
                break;
            }
        }
        let mut py = 0i64;
        for q in 1..=frame_h {
            let mut ok = true;
            'y: for r in 0..(frame_h - q) {
                for c in 0..frame_w {
                    let a = &grid[(r * frame_w + c) as usize];
                    let b = &grid[((r + q) * frame_w + c) as usize];
                    if a[0] != b[0] || a[1] != b[1] || a[2] + q != b[2] {
                        ok = false;
                        break 'y;
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

    #[test]
    fn period_hierarchy() {
        let count_pins = [16, 64, 256, 1024, 4096];
        let mut prev: (i64, i64) = (0, 0);
        for g in 2..=6usize {
            let tiles = patch_of(A, g as u32);
            assert_eq!(tiles.len(), count_pins[g - 2], "g={g} count");
            let (px, py) = patch_period(g as u32);
            // Full block: no proper period at any depth — the patch is
            // the fundamental cell of the approximant.
            assert_eq!(px, 1i64 << g, "g={g}: minimal x-period {px}");
            assert_eq!(py, 2i64 << g, "g={g}: minimal y-period {py}");
            if prev != (0, 0) {
                assert_eq!(
                    (px, py),
                    (2 * prev.0, 2 * prev.1),
                    "g={g}: not strictly 2× growing"
                );
            }
            prev = (px, py);
        }
    }

    // --- nested views ------------------------------------------------------

    fn gen_at(region: BBox) -> TilingResult {
        let t = DominoTiling;
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

    /// All five windows sit in the d = 7 band (E² ∈ [5/16, 5/4) —
    /// pinned per window) and emit the same 4^7 patch recentered on
    /// the origin: every tile of a smaller view must reappear
    /// bit-identically in each larger one containing it. True
    /// containments only: B5 ⊂ B1, B1 ⊂ B2, B4 ⊂ B2 (B4 and B3 stick
    /// out of their near-neighbors — no containment asserted for B3).
    #[test]
    fn nested_view_bit_identical() {
        let wins: [(&str, BBox); 5] = [
            // E² = 0.72
            ("B1", BBox::new(v2(-0.25, -0.25), v2(0.35, 0.35))),
            // E² = 1.1113
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
                    (5.0 / 16.0..5.0 / 4.0).contains(&e2),
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

    // --- default view ------------------------------------------------------

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Pins both default presentations against the degenerate-floor
    /// bug (a d = 5 view renders 1024 tiles, never the whole tiling as
    /// one patch):
    ///
    /// (a) The UI/preview convention: a ±10 probe (d = 5: E² = 800 ≥
    ///     5), then `preview_window(9 × mean tile side, 4/3)`. Every
    ///     domino has AABB sides (1, 2), so the mean side is 1.5 and
    ///     the derived window stays in the d = 5 band (E² = 506.25),
    ///     non-empty with both domino types, and the second probe
    ///     round is a fixed point (all tiles have side (1, 2), so no
    ///     culling shifts the mix).
    /// (b) The CLI convention: a ±(40/3) × ±10 origin-centered pane —
    ///     d = 5, non-empty (the d = 5 block |x| ≤ 16, |y| ≤ 32
    ///     covers the pane).
    #[test]
    fn default_view_fills_pane() {
        let t = DominoTiling;
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
            (1.5 - 1e-9..=1.5 + 1e-9).contains(&mw),
            "mean tile side {mw} — every domino AABB has sides (1, 2), mean 1.5"
        );
        let win = tess_core::app::preview_window(probe.periodic, mw, 4.0 / 3.0);
        let r = t.generate(&p, &Query { region: win });
        assert!(r.message.is_none());
        assert!(!r.tiles.is_empty(), "preview window must be non-empty");
        let s = win.size();
        assert!(
            s.x * s.x + s.y * s.y >= 5.0,
            "derived window E² = {} left the d = 5 band",
            s.x * s.x + s.y * s.y
        );
        assert_eq!(
            r.tiles
                .iter()
                .map(|tl| tl.seed)
                .collect::<BTreeSet<u32>>()
                .len(),
            2,
            "preview must show both domino types"
        );

        // fixed point: the second round re-derives the same window
        let mw2 = tess_core::app::mean_tile_side(&r.tiles);
        let win2 = tess_core::app::preview_window(r.periodic, mw2, 4.0 / 3.0);
        let r2 = t.generate(&p, &Query { region: win2 });
        assert!(r2.message.is_none());
        assert!(!r2.tiles.is_empty(), "second round must be non-empty");
        let s2 = win2.size();
        assert!(
            s2.x * s2.x + s2.y * s2.y >= 5.0,
            "second-round window left the d = 5 band"
        );
        assert_eq!(
            r2.tiles
                .iter()
                .map(|tl| tl.seed)
                .collect::<BTreeSet<u32>>()
                .len(),
            2,
            "second round shows only one domino type"
        );

        // (b) CLI pane ±(40/3) × ±10
        let region = BBox::new(v2(-40.0 / 3.0, -10.0), v2(40.0 / 3.0, 10.0));
        let rc = t.generate(&p, &Query { region });
        assert!(rc.message.is_none());
        assert!(!rc.tiles.is_empty(), "CLI pane must be non-empty");
    }
}
