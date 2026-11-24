//! Watanabe–Ito–Soma 12-fold — the Bielefeld entry "WIS 12-fold".
//!
//! Three prototiles — a 60° equilateral triangle, a square, and a 30°/150°
//! rhombus — with dodecagonal symmetry and inflation **λ = 2+√3** (λ² =
//! 7+4√3 ≈ 13.928). [WSI95] Y. Watanabe, T. Soma, M. Ito, "A new
//! quasiperiodic tiling with dodecagonal symmetry", *Acta Cryst.* A
//! **51**(6) (1995) 936–942, DOI 10.1107/s0108767395009160.
//!
//! **Mandatory provenance caveat (READ FIRST).** The rule tables below are
//! the ENTRY FIGURE's published rules: the Bielefeld `rule.png` panels are
//! [WSI95] Fig. 2(iii) — the very ruleset the entry page says "are not
//! unique". They are a valid rule set for the tile set (every row is
//! certified: closed simple boundary, all interior edges paired, exact
//! ℤ[√3] area identity, exact λ² area law) and they generate a valid
//! same-family (WSI95) tiling. They are **NOT** the Bielefeld patch's
//! undocumented rule set: the figure's clusters have zero occurrences in the
//! 792 k-node patch (tri 0/150 certified leaves, rho topologically
//! distinct, sq 0 in the crop), and the patch specimen itself is
//! non-generic, so it cannot serve as a census oracle. The "correct set" the
//! entry attributes to Musesti–Paolini (2023) has no machine-readable record
//! anywhere (source hunt: negative). Provenance is therefore pinned as
//! **WSI95 Fig. 2(iii), the entry figure's non-unique ruleset**.
//!
//! **Non-conformal, same-scale rules.** Each rule is NOT a λ×-parent frame
//! (collinear-edge proof; 0 of 143 892 frame candidates). The children are
//! prototiles at edge 2H — the SAME size as the parent — arranged in a
//! jagged cluster whose union has area λ²·parent and CONTAINS the parent
//! tile (pinned by the exact parent-containment battery). The rule's `m` is
//! expressed in the parent's root-corner frame.
//!
//! **The rho rule ships as a 3-piece disconnected cluster** (blob / island /
//! pendant, 9 + 2 + 2 tiles; per-piece simple boundaries 10 / 5 / 5 edges).
//! The pieces are free up to translation; the offsets are FITTED lattice
//! snaps: island (−2,−1,−3,−2), pendant (2,1,1,0). The per-piece free
//! translation is not measured by the fit — the figure's ~1.9 H inter-piece
//! gaps are artist-chosen; the fitted offsets sit in the exact lattice
//! frame. The full 13-tile cluster never occurs in the patch (0 occurrences
//! of all 12 rotations); island alone 1147, pendant alone 1105.
//!
//! **Pinch geometry.** The three rho pieces share NO edges; they touch at
//! exactly two point-contacts (pinches), each a 4-tile vertex whose
//! tile-corner sum is exactly 8 (30° units — a 120° gap): blob–island at
//! (−1,0,−2,−1), blob–pendant at (2,1,1,0). Island and pendant do not
//! touch.
//!
//! **The rules do not compose at depth ≥ 2.** Growing a single parent to
//! depth 2 by the same-scale rule application produces real positive-area
//! overlaps between tiles from different D1 parents (the drift-guard pins,
//! exact ℤ[√3], pre-dedup: tri seed 155 tiles / 770 overlap pairs / 0
//! same-parent; sq seed 358 / 3660 / 0; rho seed 179 / 1023 / 0). A
//! criterion-unsalvageable pre-wipe pin of 151/336/123 is superseded by
//! these; the decisive fact (non-composition, same-parent always clean) is
//! unchanged and stronger.
//!
//! **Deliverable shape.** Because the rules do not compose, the module ships
//! the figure's certified level-1 dissections and `generate` emits the
//! depth-1 single-parent square supertile (26 tiles, simply connected,
//! parent square ⊆ union) as a finite patch; each tile is filtered by BBox
//! intersection (contract-legal for finite patches).
//!
//! **Exact arithmetic.** A point is an integer 4-tuple (P,Q,R,S) ⇔
//! x = (s/2)·(P + Q√3), y = (s/2)·(R + S√3) with H = s/2; the vertex module
//! is the full ℤ⁴ and tile edge vectors lie in the index-4 sublattice
//! {P≡S (mod 2), Q≡R (mod 2)}. Areas are exact 2·area pairs in ℤ[√3],
//! (a,b) = a + b√3. 30° rotation: rot4(P,Q,R,S) = ((3Q−R)/2, (P−S)/2,
//! (P+3S)/2, (Q+R)/2), k mod 12. f64 appears only at emission and in the
//! standard-coverage windows.
//!
//! **Patch cross-check (report only).** Bielefeld `patch.pdf` fill census:
//! 18,302 tri / 11,644 sq / 7,689 rho (37,635 total; 28,783 clean-index).
//!
//! **Scope.** The substitution mechanics are fully verified in exact
//! arithmetic (battery in `mod wis_12_fold_tests`); the quasiperiodic-
//! diffraction claim follows the golden-family convention (see
//! golden_rhomboid's scope note).

use tess_core::geometry::{Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

// ---------------------------------------------------------------------------
// Exact arithmetic: points of the ℤ[√3] lattice as integer 4-tuples
// ---------------------------------------------------------------------------

/// Shape codes, in seed order (main/largest first): triangle, square, rho.
const TRI: u8 = 0;
const SQ: u8 = 1;
const RHO: u8 = 2;

/// A lattice point: (P, Q, R, S) ⇔ x = H·(P + Q√3), y = H·(R + S√3).
pub(crate) type P4 = (i64, i64, i64, i64);

/// H = s/2, the half-edge. World embedding only (emission + test floats).
const H: f64 = 0.5;
/// √3 as f64 — emission and test floats only.
const SQRT3_FLT: f64 = 1.7320508075688772;

/// Base prototile corners (CCW, y-up), (P,Q,R,S), at k=0 / anchor 0.
pub(crate) const TRI_CORN: [P4; 3] = [(0, 0, 0, 0), (2, 0, 0, 0), (1, 0, 0, 1)];
pub(crate) const SQ_CORN: [P4; 4] =
    [(0, 0, 0, 0), (2, 0, 0, 0), (2, 0, 2, 0), (0, 0, 2, 0)];
pub(crate) const RHO_CORN: [P4; 4] =
    [(0, 0, 0, 0), (2, 0, 0, 0), (2, 1, 1, 0), (0, 1, 1, 0)];

pub(crate) fn base_corners(kind: u8) -> &'static [P4] {
    match kind {
        TRI => &TRI_CORN,
        SQ => &SQ_CORN,
        _ => &RHO_CORN,
    }
}

/// 2·area of each prototile as a ℤ[√3] pair (a,b) = a + b√3.
#[cfg(test)]
const AREA2: [[i64; 2]; 3] = [[0, 2], [8, 0], [4, 0]];

/// Tile-corner angles in 30° units, in corner order.
#[cfg(test)]
const ANGLE: [[i32; 4]; 3] = [[2, 2, 2, 0], [3, 3, 3, 3], [1, 5, 1, 5]];

/// The 12 directed primitive edge directions of the dodecagonal lattice
/// (M-generators), keyed by their world angle in degrees.
#[cfg(test)]
const EVEC12: [(i32, P4); 12] = [
    (0, (2, 0, 0, 0)),
    (180, (-2, 0, 0, 0)),
    (90, (0, 0, 2, 0)),
    (270, (0, 0, -2, 0)),
    (30, (0, 1, 1, 0)),
    (210, (0, -1, -1, 0)),
    (330, (0, 1, -1, 0)),
    (150, (0, -1, 1, 0)),
    (60, (1, 0, 0, 1)),
    (240, (-1, 0, 0, -1)),
    (300, (1, 0, 0, -1)),
    (120, (-1, 0, 0, 1)),
];

#[inline]
pub(crate) fn add4(a: P4, b: P4) -> P4 {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
}

#[cfg(test)]
#[inline]
pub(crate) fn sub4(a: P4, b: P4) -> P4 {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3)
}

#[cfg(test)]
#[inline]
fn mul2(a: P4) -> P4 {
    (2 * a.0, 2 * a.1, 2 * a.2, 2 * a.3)
}

/// A (P,Q,R,S) tuple as an array, for variable-index arithmetic.
#[cfg(test)]
#[inline]
fn p4a(p: P4) -> [i64; 4] {
    [p.0, p.1, p.2, p.3]
}

/// Exact R^k, 30° steps, on a (P,Q,R,S) point. The four numerators are
/// always even on lattice points; the debug assert pins that.
pub(crate) fn rot4(t: P4, k: i8) -> P4 {
    let (mut p, mut q, mut r, mut s) = t;
    for _ in 0..(k.rem_euclid(12)) {
        let (p2, q2, r2, s2) = (3 * q - r, p - s, p + 3 * s, q + r);
        debug_assert!(
            p2 % 2 == 0 && q2 % 2 == 0 && r2 % 2 == 0 && s2 % 2 == 0,
            "rot4 on a non-lattice point: {:?}",
            (p2, q2, r2, s2)
        );
        (p, q, r, s) = (p2 / 2, q2 / 2, r2 / 2, s2 / 2);
    }
    (p, q, r, s)
}

/// World embedding (emission + test floats only).
pub(crate) fn val(p: P4) -> Vec2 {
    Vec2::new(
        H * (p.0 as f64 + p.1 as f64 * SQRT3_FLT),
        H * (p.2 as f64 + p.3 as f64 * SQRT3_FLT),
    )
}

/// An element of ℤ[√3]: a + b·√3.
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Z3 {
    a: i64,
    b: i64,
}

#[cfg(test)]
impl Z3 {
    #[inline]
    const fn new(a: i64, b: i64) -> Self {
        Z3 { a, b }
    }
    const ZERO: Z3 = Z3 { a: 0, b: 0 };

    #[inline]
    const fn add(self, o: Self) -> Self {
        Z3::new(self.a + o.a, self.b + o.b)
    }
    #[inline]
    const fn sub(self, o: Self) -> Self {
        Z3::new(self.a - o.a, self.b - o.b)
    }
    /// (a + b√3)(c + d√3) = (ac + 3bd) + (ad + bc)√3.
    #[inline]
    const fn mul(self, o: Self) -> Self {
        Z3::new(
            self.a * o.a + 3 * self.b * o.b,
            self.a * o.b + self.b * o.a,
        )
    }
    /// Exact sign of a + b√3: −1 / 0 / +1. a² > 3b² ⇒ sign(a);
    /// a² < 3b² ⇒ sign(b); equality over ℤ only at (0,0).
    fn sign(self) -> i8 {
        let (a, b) = (self.a, self.b);
        if a == 0 {
            return b.signum() as i8;
        }
        if b == 0 {
            return a.signum() as i8;
        }
        if a * a > 3 * b * b {
            return a.signum() as i8;
        }
        b.signum() as i8
    }
    fn abs(self) -> Self {
        if self.sign() < 0 {
            Z3::new(-self.a, -self.b)
        } else {
            self
        }
    }
}

/// (q − p) × (r − p) as a ℤ[√3] pair (2-D cross / oriented-area 2×).
#[cfg(test)]
pub(crate) fn cross_z3(p: P4, q: P4, r: P4) -> Z3 {
    let ax = Z3::new(q.0 - p.0, q.1 - p.1);
    let ay = Z3::new(q.2 - p.2, q.3 - p.3);
    let bx = Z3::new(r.0 - p.0, r.1 - p.1);
    let by = Z3::new(r.2 - p.2, r.3 - p.3);
    ax.mul(by).sub(ay.mul(bx))
}

/// Free-vector u × v as a ℤ[√3] pair.
#[cfg(test)]
pub(crate) fn cross_z3v(u: P4, v: P4) -> Z3 {
    let ux = Z3::new(u.0, u.1);
    let uy = Z3::new(u.2, u.3);
    let vx = Z3::new(v.0, v.1);
    let vy = Z3::new(v.2, v.3);
    ux.mul(vy).sub(uy.mul(vx))
}

/// Euclidean dot of two ℤ⁴ vectors as a ℤ[√3] pair.
#[cfg(test)]
pub(crate) fn dot_z3(u: P4, v: P4) -> Z3 {
    let t1 = Z3::new(u.0, u.1).mul(Z3::new(v.0, v.1));
    let t2 = Z3::new(u.2, u.3).mul(Z3::new(v.2, v.3));
    t1.add(t2)
}

/// 2× signed area of a (CCW or CW) polygon corner chain, as a ℤ[√3] pair.
#[cfg(test)]
pub(crate) fn shoelace_signed(poly: &[P4]) -> Z3 {
    let mut tot = Z3::ZERO;
    let n = poly.len();
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        tot = tot.add(Z3::new(
            a.0 * b.2 + 3 * a.1 * b.3 - b.0 * a.2 - 3 * b.1 * a.3,
            a.0 * b.3 + a.1 * b.2 - b.0 * a.3 - b.1 * a.2,
        ));
    }
    tot
}

/// 2× unsigned area of a polygon corner chain.
#[cfg(test)]
pub(crate) fn area2_of(poly: &[P4]) -> Z3 {
    shoelace_signed(poly).abs()
}

// ---------------------------------------------------------------------------
// Certified figure rule tables (WSI95 Fig. 2(iii)), exact child placements.
// Row = (child kind, child krot mod 12, m, piece). For tri/sq the piece is
// always 0 and m is relative to the parent root corner. For rho, m is
// relative to the owning piece's frame origin and `piece` names the piece;
// rows_global() adds the fitted piece offset.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub(crate) struct Row {
    pub kind: u8,
    pub krot: i8,
    pub m: P4,
    pub piece: u8,
}

/// Rho piece fitted offsets (piece 0 = blob, 1 = island, 2 = pendant), in
// the parent root-corner frame. Inter-piece translations are free (see
// module docs): these are the exact-lattice snaps chosen for the shipped
// cluster.
pub(crate) const RHO_OFF: [P4; 3] = [
    (0, 0, 0, 0),
    (-2, -1, -3, -2),
    (2, 1, 1, 0),
];

/// tri supertile: 11 children (7 tri + 2 sq + 2 rho), one closed 15-edge
/// boundary chain, 2·area = (24,14) = (7,4)·(0,2).
pub(crate) const TRI_SUP: [Row; 11] = [
    Row { kind: RHO, krot: 1,  m: (-1, -1, -1, -1), piece: 0 },
    Row { kind: RHO, krot: 0,  m: (-1, -1, -1, -1), piece: 0 },
    Row { kind: SQ,  krot: 0,  m: (0, 0, 0, 0),      piece: 0 },
    Row { kind: SQ,  krot: 2,  m: (1, 1, -1, -1),    piece: 0 },
    Row { kind: TRI, krot: 3,  m: (0, 0, 0, 0),      piece: 0 },
    Row { kind: TRI, krot: 0,  m: (-1, 0, 0, -1),    piece: 0 },
    Row { kind: TRI, krot: 3,  m: (1, 0, -2, -1),    piece: 0 },
    Row { kind: TRI, krot: 10, m: (0, 0, 0, 0),      piece: 0 },
    Row { kind: TRI, krot: 0,  m: (0, 0, 2, 0),      piece: 0 },
    Row { kind: TRI, krot: 3,  m: (2, 1, -1, 0),     piece: 0 },
    Row { kind: TRI, krot: 0,  m: (1, 1, -1, -1),    piece: 0 },
];

/// sq supertile: 26 children (16 tri + 4 sq + 6 rho), one closed 20-edge
/// boundary chain, 2·area = (56,32) = (7,4)·(8,0).
pub(crate) const SQ_SUP: [Row; 26] = [
    Row { kind: RHO, krot: 2,  m: (-1, 0, -2, -1), piece: 0 },
    Row { kind: RHO, krot: 1,  m: (-1, 0, -2, -1), piece: 0 },
    Row { kind: RHO, krot: 0,  m: (-1, 0, -2, -1), piece: 0 },
    Row { kind: RHO, krot: 5,  m: (1, 1, 1, 1),    piece: 0 },
    Row { kind: RHO, krot: 4,  m: (2, 2, 0, 0),    piece: 0 },
    Row { kind: RHO, krot: 3,  m: (3, 2, -2, -1),  piece: 0 },
    Row { kind: SQ,  krot: 1,  m: (0, 0, 0, 0),    piece: 0 },
    Row { kind: SQ,  krot: 0,  m: (0, 1, -1, 0),   piece: 0 },
    Row { kind: SQ,  krot: 2,  m: (1, 2, -2, -1),  piece: 0 },
    Row { kind: SQ,  krot: 0,  m: (1, 2, 0, 1),    piece: 0 },
    Row { kind: TRI, krot: 0,  m: (-2, 0, 0, 0),   piece: 0 },
    Row { kind: TRI, krot: 1,  m: (-1, 0, 0, 1),   piece: 0 },
    Row { kind: TRI, krot: 1,  m: (0, 0, -2, 0),   piece: 0 },
    Row { kind: TRI, krot: 3,  m: (0, 1, -1, 0),   piece: 0 },
    Row { kind: TRI, krot: 0,  m: (-1, 1, -1, -1), piece: 0 },
    Row { kind: TRI, krot: 2,  m: (0, 1, 1, 0),    piece: 0 },
    Row { kind: TRI, krot: 3,  m: (1, 1, -3, -1),  piece: 0 },
    Row { kind: TRI, krot: 2,  m: (1, 1, -1, -1),  piece: 0 },
    Row { kind: TRI, krot: 0,  m: (0, 1, 1, 0),    piece: 0 },
    Row { kind: TRI, krot: 1,  m: (1, 1, 1, 1),    piece: 0 },
    Row { kind: TRI, krot: 3,  m: (1, 2, 0, 1),    piece: 0 },
    Row { kind: TRI, krot: 1,  m: (2, 1, -1, 0),   piece: 0 },
    Row { kind: TRI, krot: 3,  m: (2, 2, -2, 0),   piece: 0 },
    Row { kind: TRI, krot: 0,  m: (1, 2, -2, -1),  piece: 0 },
    Row { kind: TRI, krot: 2,  m: (2, 2, 0, 0),    piece: 0 },
    Row { kind: TRI, krot: 2,  m: (3, 2, 0, -1),   piece: 0 },
];

/// rho supertile: 13 children (8 tri + 2 sq + 3 rho) in 3 pieces. `m` is
/// piece-relative; `piece` = 0 blob (9 tiles), 1 island (2), 2 pendant (2).
/// Per-piece 2·area: blob (20,12), island (4,2), pendant (4,2); total
/// (28,16) = (7,4)·(4,0).
pub(crate) const RHO_SUP: [Row; 13] = [
    Row { kind: RHO, krot: 1,  m: (0, 0, 0, 0),    piece: 1 },
    Row { kind: RHO, krot: 10, m: (0, 0, 0, 0),    piece: 0 },
    Row { kind: RHO, krot: 1,  m: (0, 0, 0, 0),    piece: 2 },
    Row { kind: SQ,  krot: 0,  m: (-1, 0, -2, -1), piece: 0 },
    Row { kind: SQ,  krot: 0,  m: (0, 1, -1, 0),   piece: 0 },
    Row { kind: TRI, krot: 3,  m: (-1, 0, -2, -1), piece: 0 },
    Row { kind: TRI, krot: 4,  m: (2, 1, 1, 0),    piece: 1 },
    Row { kind: TRI, krot: 0,  m: (-1, 0, 0, -1),  piece: 0 },
    Row { kind: TRI, krot: 3,  m: (0, 1, -1, 0),   piece: 0 },
    Row { kind: TRI, krot: 1,  m: (1, 0, -2, -1),  piece: 0 },
    Row { kind: TRI, krot: 2,  m: (1, 1, -1, -1),  piece: 0 },
    Row { kind: TRI, krot: 2,  m: (0, 0, 0, 0),    piece: 2 },
    Row { kind: TRI, krot: 1,  m: (2, 1, -1, 0),   piece: 0 },
];

pub(crate) fn rows_table(kind: u8) -> &'static [Row] {
    match kind {
        TRI => &TRI_SUP,
        SQ => &SQ_SUP,
        _ => &RHO_SUP,
    }
}

/// D1 child spec of a kind-`kind` parent at k=0 / anchor 0: (child kind,
/// child krot, anchor). The anchor is the piece-relative m plus the fitted
/// piece offset (a no-op for tri/sq, whose piece is always 0).
fn rows_global(kind: u8) -> Vec<(u8, i8, P4)> {
    rows_table(kind)
        .iter()
        .map(|r| (r.kind, r.krot, add4(r.m, RHO_OFF[r.piece as usize])))
        .collect()
}

// ---------------------------------------------------------------------------
// Exact predicates (port of the research spike s09_cert; all Z[√3]-exact).
// Test-only: used by the certification battery below.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod preds {
    use super::*;

    /// Strict interior of a CCW convex polygon.
    pub(crate) fn inside(p: P4, poly: &[P4]) -> bool {
        let n = poly.len();
        for i in 0..n {
            if cross_z3(poly[i], poly[(i + 1) % n], p).sign() <= 0 {
                return false;
            }
        }
        true
    }

    pub(crate) fn on_closed_seg(p: P4, a: P4, b: P4) -> bool {
        cross_z3(a, b, p).sign() == 0 && dot_z3(sub4(p, a), sub4(p, b)).sign() <= 0
    }

    /// p strictly inside the OPEN segment a->b.
    pub(crate) fn on_open_seg(p: P4, a: P4, b: P4) -> bool {
        cross_z3(a, b, p).sign() == 0 && dot_z3(sub4(p, a), sub4(p, b)).sign() < 0
    }

    /// PROPER (interior) crossing of segments ab, cd: all four orientations
    /// strictly nonzero and opposite on each segment (shared endpoints /
    /// T-junctions give a zero sign and must NOT count).
    pub(crate) fn edges_cross(a: P4, b: P4, c: P4, d: P4) -> bool {
        let d1 = cross_z3(c, d, a).sign();
        let d2 = cross_z3(c, d, b).sign();
        let d3 = cross_z3(a, b, c).sign();
        let d4 = cross_z3(a, b, d).sign();
        d1 != 0 && d2 != 0 && d1 != d2 && d3 != 0 && d4 != 0 && d3 != d4
    }

    /// Collinear edge pairs with POSITIVE shared length whose two polygons'
    /// off-line vertices sit on the SAME side of the shared line (positive
    /// area overlap invisible to vertex / proper-crossing tests).
    pub(crate) fn same_side_line_overlap(si: &[P4], sj: &[P4]) -> bool {
        for a_i in 0..si.len() {
            let (a1, a2) = (si[a_i], si[(a_i + 1) % si.len()]);
            for c_i in 0..sj.len() {
                let (c1, c3) = (sj[c_i], sj[(c_i + 1) % sj.len()]);
                if cross_z3(a1, a2, c1).sign() != 0 || cross_z3(a1, a2, c3).sign() != 0 {
                    continue; // not the same line
                }
                let overlap = on_open_seg(c1, a1, a2)
                    || on_open_seg(a1, c1, c3)
                    || (a1 == c1 && dot_z3(sub4(a2, a1), sub4(c3, a1)).sign() > 0)
                    || (a1 == c3 && dot_z3(sub4(a2, a1), sub4(c1, a1)).sign() > 0)
                    || (a2 == c1 && dot_z3(sub4(c3, c1), sub4(a1, c1)).sign() > 0)
                    || (a2 == c3 && dot_z3(sub4(c3, c1), sub4(a1, c1)).sign() > 0);
                if !overlap {
                    continue; // point touch or no overlap
                }
                let wi = si.iter().find(|v| cross_z3(a1, a2, **v).sign() != 0);
                let wj = sj.iter().find(|v| cross_z3(a1, a2, **v).sign() != 0);
                if let (Some(wi), Some(wj)) = (wi, wj) {
                    if cross_z3(a1, a2, *wi).sign() == cross_z3(a1, a2, *wj).sign() {
                        return true; // interiors same side
                    }
                }
            }
        }
        false
    }

    /// Positive-area overlap of two convex tile corners (interior
    /// intersection: vertex-in-strict-interior + proper crossing +
    /// collinear same-side overlap).
    pub(crate) fn overlap_pair(si: &[P4], sj: &[P4]) -> bool {
        for v in si {
            if inside(*v, sj) {
                return true;
            }
        }
        for v in sj {
            if inside(*v, si) {
                return true;
            }
        }
        for a in 0..si.len() {
            for c in 0..sj.len() {
                if edges_cross(si[a], si[(a + 1) % si.len()], sj[c], sj[(c + 1) % sj.len()]) {
                    return true;
                }
            }
        }
        same_side_line_overlap(si, sj)
    }

    /// Exact winding number of p about the closed corner chain `poly`
    /// (horizontal ray, up/down rule). None iff p lies on the chain.
    pub(crate) fn winding(p: P4, poly: &[P4]) -> Option<i32> {
        let n = poly.len();
        for i in 0..n {
            if on_closed_seg(p, poly[i], poly[(i + 1) % n]) {
                return None;
            }
        }
        let mut w = 0i32;
        for i in 0..n {
            let (a, b) = (poly[i], poly[(i + 1) % n]);
            let s = cross_z3(a, b, p).sign();
            let ya = Z3::new(a.2 - p.2, a.3 - p.3);
            let yb = Z3::new(b.2 - p.2, b.3 - p.3);
            if ya.sign() <= 0 {
                if yb.sign() > 0 && s > 0 {
                    w += 1;
                }
            } else if yb.sign() <= 0 && s < 0 {
                w -= 1;
            }
        }
        Some(w)
    }

    /// p in the closed interior of a SIMPLE polygonal chain: winding ±1 or
    /// on the chain.
    pub(crate) fn inside_or_on(p: P4, poly: &[P4]) -> bool {
        match winding(p, poly) {
            None => true,
            Some(w) => w.abs() == 1,
        }
    }

    /// q2 = 2*midpoint of a child edge (Z4); True iff that midpoint lies in
    /// the strict interior of, or on the boundary of, CCW convex polygon
    /// `poly`. (Factor-2 doubling is sign-stable for every comparison.)
    fn mid_covered(q2: P4, poly: &[P4]) -> bool {
        let n = poly.len();
        let mut interior = true;
        for i in 0..n {
            let (c, d) = (poly[i], poly[(i + 1) % n]);
            if cross_z3v(sub4(d, c), sub4(q2, mul2(c))).sign() <= 0 {
                interior = false;
                break;
            }
        }
        if interior {
            return true;
        }
        for i in 0..n {
            let (c, d) = (poly[i], poly[(i + 1) % n]);
            if cross_z3v(sub4(d, c), sub4(q2, mul2(c))).sign() == 0
                && dot_z3(sub4(q2, mul2(c)), sub4(q2, mul2(d))).sign() <= 0
            {
                return true;
            }
        }
        false
    }

    /// Placed children from a D1 spec: (kind, krot, anchor, corners).
    pub(crate) fn children_cors(spec: &[(u8, i8, P4)]) -> Vec<(u8, i8, P4, Vec<P4>)> {
        spec.iter()
            .map(|&(sh, kr, m4)| {
                let cs = base_corners(sh)
                    .iter()
                    .map(|c| add4(rot4(*c, kr), m4))
                    .collect();
                (sh, kr, m4, cs)
            })
            .collect()
    }

    /// Union-boundary of the children by exact midpoint extraction: a
    /// child's directed edge is on the union boundary iff its midpoint lies
    /// in no other child (interior or boundary). Robust to T-junctions.
    /// -> list of directed boundary edge chains.
    pub(crate) fn boundary(spec: &[(u8, i8, P4)]) -> Vec<Vec<(P4, P4)>> {
        let cl = children_cors(spec);
        let mut kept: Vec<(P4, P4)> = Vec::new();
        for (i, c) in cl.iter().enumerate() {
            let si = &c.3;
            for a in 0..si.len() {
                let e = (si[a], si[(a + 1) % si.len()]);
                let q2 = add4(e.0, e.1); // 2 * midpoint
                let covered = cl.iter().enumerate().any(|(j, cj)| {
                    j != i && mid_covered(q2, &cj.3)
                });
                if !covered {
                    kept.push(e);
                }
            }
        }
        let mut rem = kept;
        let mut chains: Vec<Vec<(P4, P4)>> = Vec::new();
        while let Some(e) = rem.first().copied() {
            rem.remove(0);
            let mut chain = vec![e];
            loop {
                let start = chain.last().unwrap().1;
                let pos = rem.iter().position(|x| x.0 == start);
                match pos {
                    Some(p) => chain.push(rem.remove(p)),
                    None => break,
                }
            }
            chains.push(chain);
        }
        chains
    }

    pub(crate) fn chain_closed(ch: &[(P4, P4)]) -> bool {
        ch.first().map(|e| e.0) == ch.last().map(|e| e.1)
    }

    /// True iff closed edge chain `ch` is a SIMPLE polygon: no repeated
    /// corner at a non-adjacent position, no proper self-crossing, no
    /// positive-length collinear self-overlap, no vertex in the interior of
    /// a non-adjacent edge.
    pub(crate) fn chain_simple(ch: &[(P4, P4)]) -> bool {
        let l = ch.len();
        if l < 4 {
            return false;
        }
        for i in 0..l {
            for j in (i + 1)..l {
                if j == i + 1 || (i == 0 && j == l - 1) {
                    continue; // adjacent edges share a corner
                }
                let (a, b) = (ch[i].0, ch[i].1);
                let (c, d) = (ch[j].0, ch[j].1);
                if a == c || a == d || b == c || b == d {
                    continue; // handled by the repeat test
                }
                if on_open_seg(c, a, b) || on_open_seg(d, a, b) {
                    return false; // vertex in a non-adjacent edge
                }
                if edges_cross(a, b, c, d) {
                    return false; // proper self-crossing
                }
                if cross_z3(a, b, c).sign() == 0 {
                    // collinear 4-tuple: positive shared length?
                    let l2 = dot_z3(sub4(b, a), sub4(b, a));
                    let tc = dot_z3(sub4(c, a), sub4(b, a));
                    let td = dot_z3(sub4(d, a), sub4(b, a));
                    let (mn, mx) = if tc.sub(td).sign() <= 0 { (tc, td) } else { (td, tc) };
                    let lo = if mn.sign() > 0 { mn } else { Z3::ZERO };
                    let hi = if mx.sub(l2).sign() < 0 { mx } else { l2 };
                    if hi.sub(lo).sign() > 0 {
                        return false; // positive-length self-overlap
                    }
                }
            }
        }
        let cns: Vec<P4> = ch.iter().map(|e| e.0).collect();
        let mut seen: std::collections::HashMap<P4, usize> = std::collections::HashMap::new();
        for (i, c) in cns.iter().enumerate() {
            if let Some(&j) = seen.get(c) {
                let d = (i as i64 - j as i64).abs();
                if d.min(l as i64 - d) > 1 {
                    return false; // repeated corner, non-adjacent
                }
            } else {
                seen.insert(*c, i);
            }
        }
        true
    }

    /// v = c0 * (primitive direction), c0 > 0 -> (world angle, c0); else
    /// None.
    pub(crate) fn edge_dir(v: P4) -> Option<(i32, i64)> {
        let v = p4a(v);
        for &(ang, ev) in EVEC12.iter() {
            let ev = p4a(ev);
            let mut c0: Option<i64> = None;
            let mut ok = true;
            for i in 0..4 {
                if ev[i] != 0 {
                    if v[i] % ev[i] != 0 {
                        ok = false;
                        break;
                    }
                    let q = v[i] / ev[i];
                    match c0 {
                        None => c0 = Some(q),
                        Some(c) if c != q => {
                            ok = false;
                            break;
                        }
                        Some(_) => {}
                    }
                }
            }
            if ok {
                if let Some(c0) = c0 {
                    if c0 > 0 && (0..4).all(|i| v[i] == c0 * ev[i]) {
                        return Some((ang, c0));
                    }
                }
            }
        }
        None
    }
}

// ---------------------------------------------------------------------------
// The tiling
// ---------------------------------------------------------------------------

/// Watanabe–Ito–Soma 12-fold — see the module docs.
///
/// Parameters: the shared similarity map. The emission is a finite
/// depth-1 patch (the 26-tile square supertile, with the parent unit
/// square [0,1]² inside the union), so the identity view frames the
/// patch; views disjoint from the patch hull are background.
pub struct Wis12FoldTiling;

/// The emitted depth-1 patch: the 26 children of one k=0 / anchor-0
/// square parent (the square supertile of the figure's certified
/// rules), as (kind, krot, corners in the parent frame).
fn patch_tiles() -> Vec<(u8, Vec<P4>)> {
    rows_global(SQ)
        .iter()
        .map(|&(sh, kr, m4)| {
            let cs = base_corners(sh)
                .iter()
                .map(|c| add4(rot4(*c, kr), m4))
                .collect();
            (sh, cs)
        })
        .collect()
}

impl Tiling for Wis12FoldTiling {
    fn id(&self) -> &'static str {
        "wis-12-fold"
    }

    fn name(&self) -> &'static str {
        "Watanabe–Ito–Soma (12-fold)"
    }

    fn description(&self) -> &'static str {
        "Bielefeld WIS 12-fold (Watanabe–Ito–Soma, Acta Cryst. A 51:936, \
         1995): equilateral triangle, square, 30°/150° rhombus; \
         dodecagonal symmetry, inflation λ = 2+√3 (λ² = 7+4√3); the \
         entry figure's certified level-1 rules (WSI95 Fig. 2(iii), \
         the entry's non-unique ruleset); non-conformal same-scale \
         dissections in exact Z[√3] 4-tuple points; the rules do not \
         compose at depth ≥ 2, so the emission is the depth-1 \
         single-parent square supertile (26 tiles) as a finite patch, \
         views outside it are background"
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
            "quasiperiodic (Watanabe–Ito–Soma 12-fold, λ = 2+√3); finite \
             depth-1 patch emission (26-tile square supertile, the entry \
             figure's certified level-1 rules); views outside the patch \
             are background",
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
        if (rmax.x - rmin.x).abs() < 1e-12 && (rmax.y - rmin.y).abs() < 1e-12 {
            return TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: None,
                message: Some("empty view"),
            };
        }

        let tiles = patch_tiles();
        debug_assert!(tiles.len() <= MAX_TILES);

        // Cell-space patch bbox (exact points -> float hull).
        let mut pmin = Vec2::new(f64::MAX, f64::MAX);
        let mut pmax = Vec2::new(f64::MIN, f64::MIN);
        for t in &tiles {
            for c in &t.1 {
                let p = val(*c);
                pmin.x = pmin.x.min(p.x);
                pmin.y = pmin.y.min(p.y);
                pmax.x = pmax.x.max(p.x);
                pmax.y = pmax.y.max(p.y);
            }
        }
        if rmax.x < pmin.x || rmax.y < pmin.y || rmin.x > pmax.x || rmin.y > pmax.y {
            return TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: None,
                message: None,
            };
        }

        // Emission: exact points -> f64 embedding, culled to the view
        // (finite patch: contract-legal to emit only the intersecting
        // tiles). proto = seed = shape code (tri 0, sq 1, rho 2).
        let mut out = Vec::new();
        for t in &tiles {
            let pts: Vec<Vec2> = t.1.iter().map(|c| val(*c)).collect();
            let poly = Polygon::new(pts).transform(&map);
            if poly.aabb().intersects(query.region) {
                out.push(Tile {
                    proto: t.0,
                    seed: t.0 as u32,
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
// Standard trio
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use tess_core::geometry::BBox;
    use crate::tests::{check_covers, check_det, check_rotated};

    // Window strictly inside the 26-tile square supertile union: inset of
    // the parent unit square [0,1]² (proved ⊆ union, exact Q⁴); min
    // boundary margin 0.0707, min sample margin 0.1344 (spike w11).
    const WIN: [f64; 4] = [0.05, 0.05, 0.95, 0.95];

    #[test]
    fn wis_12_fold_covers_region() {
        check_covers(
            &Wis12FoldTiling,
            "wis-12-fold",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }

    #[test]
    fn wis_12_fold_rotated_scaled() {
        // Cell window [1.116, 1.616] × [−0.25, 0.25] (center (1.366, 0),
        // half-extent 0.25) at scale 0.6, rotation 25°: verified strictly
        // inside the supertile union (min margin 0.9392, spike w11);
        // axis-aligned world hull ≈ [0.5435, 0.9422] × [0.1470, 0.5457].
        let th = 25.0f64.to_radians();
        let (st, ct) = th.sin_cos();
        let cell = [(1.116, -0.25), (1.616, -0.25), (1.616, 0.25), (1.116, 0.25)];
        let mut x0 = f64::MAX;
        let mut x1 = f64::MIN;
        let mut y0 = f64::MAX;
        let mut y1 = f64::MIN;
        for &(cx, cy) in cell.iter() {
            // world = scale * Rot(theta) * cell (the shared similarity map)
            let wx = 0.6 * (ct * cx - st * cy);
            let wy = 0.6 * (st * cx + ct * cy);
            x0 = x0.min(wx);
            x1 = x1.max(wx);
            y0 = y0.min(wy);
            y1 = y1.max(wy);
        }
        check_rotated(
            &Wis12FoldTiling,
            "wis-12-fold(rot25,scale0.6)",
            BBox::new(Vec2::new(x0, y0), Vec2::new(x1, y1)),
            0.6,
            25.0,
        );
    }

    #[test]
    fn wis_12_fold_deterministic() {
        check_det(
            &Wis12FoldTiling,
            "wis-12-fold",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }
}

// ---------------------------------------------------------------------------
// Exact-arithmetic battery (ports of the research spikes w10 / w10b / w11 /
// w12). Every predicate is integer-exact over Z[sqrt3]; floats appear only
// in the pinned float checks (PF, clipped emission area).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod wis_12_fold_tests {
    use super::preds::*;
    use super::*;
    use std::cmp::Ordering;
    use std::collections::{BTreeMap, BTreeSet};

    // ---------- exact rationals (w10b parent-containment port) ----------

    /// Exact reduced rational (d > 0), i128-wide.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Q {
        n: i128,
        d: i128,
    }

    impl Q {
        fn new(n: i128, d: i128) -> Self {
            assert!(d != 0, "zero rational denominator");
            let (n, d) = if d < 0 { (-n, -d) } else { (n, d) };
            let g = gcd128(if n < 0 { -n } else { n }, d);
            Q { n: n / g, d: d / g }
        }
        fn from_i(i: i64) -> Self {
            Q { n: i as i128, d: 1 }
        }
        fn add(self, o: Self) -> Self {
            Q::new(self.n * o.d + o.n * self.d, self.d * o.d)
        }
        fn sub(self, o: Self) -> Self {
            Q::new(self.n * o.d - o.n * self.d, self.d * o.d)
        }
        fn mul(self, o: Self) -> Self {
            Q::new(self.n * o.n, self.d * o.d)
        }
        fn cmp_num(&self, o: &Self) -> Ordering {
            (self.n * o.d).cmp(&(o.n * self.d))
        }
    }

    impl PartialOrd for Q {
        fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
            Some(self.cmp_num(o))
        }
    }
    impl Ord for Q {
        fn cmp(&self, o: &Self) -> Ordering {
            self.cmp_num(o)
        }
    }

    fn gcd128(mut a: i128, mut b: i128) -> i128 {
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a.max(1)
    }

    fn q4(a: P4) -> [Q; 4] {
        [Q::from_i(a.0), Q::from_i(a.1), Q::from_i(a.2), Q::from_i(a.3)]
    }

    /// (a + b√3)(c + d√3) with a,b,c,d ∈ Q.
    fn z3q_mul(a: (Q, Q), b: (Q, Q)) -> (Q, Q) {
        (
            a.0.mul(b.0).add(a.1.mul(b.1).mul(Q::from_i(3))),
            a.0.mul(b.1).add(a.1.mul(b.0)),
        )
    }

    /// Exact sign of a + b√3 (a,b ∈ Q, not both 0).
    fn z3q_sign(a: Q, b: Q) -> i8 {
        if a.n == 0 {
            return b.n.signum() as i8;
        }
        if b.n == 0 {
            return a.n.signum() as i8;
        }
        let l = a.n * b.d;
        let r = b.n * a.d;
        match (l * l).cmp(&(3 * r * r)) {
            Ordering::Greater => a.n.signum() as i8,
            Ordering::Less => b.n.signum() as i8,
            Ordering::Equal => unreachable!("a² = 3b² over Q only at 0"),
        }
    }

    /// (q − p) × (r − p) sign for Q⁴ points (2-D cross in Q + Q√3).
    fn cross_q4(p: &[Q; 4], q: &[Q; 4], r: &[Q; 4]) -> i8 {
        let ax = (q[0].sub(p[0]), q[1].sub(p[1]));
        let ay = (q[2].sub(p[2]), q[3].sub(p[3]));
        let bx = (r[0].sub(p[0]), r[1].sub(p[1]));
        let by = (r[2].sub(p[2]), r[3].sub(p[3]));
        let t1 = z3q_mul(ax, by);
        let t2 = z3q_mul(ay, bx);
        z3q_sign(t1.0.sub(t2.0), t1.1.sub(t2.1))
    }

    fn dot_q4(u: &[Q; 4], v: &[Q; 4]) -> (Q, Q) {
        let t1 = z3q_mul((u[0], u[1]), (v[0], v[1]));
        let t2 = z3q_mul((u[2], u[3]), (v[2], v[3]));
        (t1.0.add(t2.0), t1.1.add(t2.1))
    }

    fn on_closed_seg_q(p: &[Q; 4], a: P4, b: P4) -> bool {
        let ap = q4(a);
        let bp = q4(b);
        if cross_q4(&ap, &bp, p) != 0 {
            return false;
        }
        let pa = [
            p[0].sub(ap[0]),
            p[1].sub(ap[1]),
            p[2].sub(ap[2]),
            p[3].sub(ap[3]),
        ];
        let pb = [
            p[0].sub(bp[0]),
            p[1].sub(bp[1]),
            p[2].sub(bp[2]),
            p[3].sub(bp[3]),
        ];
        let d = dot_q4(&pa, &pb);
        z3q_sign(d.0, d.1) <= 0
    }

    fn strict_inside_q(p: &[Q; 4], poly: &[P4]) -> bool {
        for i in 0..poly.len() {
            if cross_q4(&q4(poly[i]), &q4(poly[(i + 1) % poly.len()]), p) <= 0 {
                return false;
            }
        }
        true
    }

    /// a + t·(b − a) as Q⁴.
    fn seg_pt(a: P4, b: P4, t: Q) -> [Q; 4] {
        [
            Q::from_i(a.0).add(t.mul(Q::from_i(b.0 - a.0))),
            Q::from_i(a.1).add(t.mul(Q::from_i(b.1 - a.1))),
            Q::from_i(a.2).add(t.mul(Q::from_i(b.2 - a.2))),
            Q::from_i(a.3).add(t.mul(Q::from_i(b.3 - a.3))),
        ]
    }

    fn t_param(p: P4, a: P4, b: P4) -> Q {
        let (p, a, b) = (p4a(p), p4a(a), p4a(b));
        for i in 0..4 {
            if b[i] != a[i] {
                return Q::new((p[i] - a[i]) as i128, (b[i] - a[i]) as i128);
            }
        }
        Q::from_i(0) // degenerate: a == b (never for our edges)
    }

    /// Exact contact t-values (parameter on segment e) where e and f meet:
    /// proper crossings, endpoint touches, collinear-overlap endpoints.
    /// Empty when the segments do not meet. (Port of spike w10b; on a
    /// non-parallel coordinate pair the solution is checked against ALL
    /// four coordinates and the bounds, and no further pair is tried.)
    fn seg_hits(e: (P4, P4), f: (P4, P4)) -> Vec<Q> {
        let (a, b) = (p4a(e.0), p4a(e.1));
        let (c, d) = (p4a(f.0), p4a(f.1));
        for i in 0..4 {
            for j in (i + 1)..4 {
                let di = b[i] - a[i];
                let dj = b[j] - a[j];
                let fi = d[i] - c[i];
                let fj = d[j] - c[j];
                let det = di * fj - dj * fi;
                if det != 0 {
                    let ri = c[i] - a[i];
                    let rj = c[j] - a[j];
                    let t = Q::new((ri * fj - rj * fi) as i128, det as i128);
                    let u = Q::new((ri * dj - rj * di) as i128, det as i128);
                    let ok = (0..4).all(|k| {
                        if k == i || k == j {
                            true
                        } else {
                            Q::from_i(a[k]).add(t.mul(Q::from_i(b[k] - a[k])))
                                == Q::from_i(c[k]).add(u.mul(Q::from_i(d[k] - c[k])))
                        }
                    });
                    if ok
                        && t >= Q::from_i(0)
                        && t <= Q::from_i(1)
                        && u >= Q::from_i(0)
                        && u <= Q::from_i(1)
                    {
                        return vec![t];
                    }
                    return Vec::new();
                }
            }
        }
        // Parallel for every coordinate pair -> collinear or disjoint.
        if cross_z3(e.0, e.1, f.0).sign() != 0 {
            return Vec::new();
        }
        let tc = t_param(f.0, e.0, e.1);
        let td = t_param(f.1, e.0, e.1);
        let (lo0, hi0) = if tc <= td { (tc, td) } else { (td, tc) };
        let zero = Q::from_i(0);
        let one = Q::from_i(1);
        let lo = if lo0 < zero { zero } else { lo0 };
        let hi = if hi0 > one { one } else { hi0 };
        if lo < hi {
            vec![lo, hi]
        } else if lo == hi {
            vec![lo]
        } else {
            Vec::new()
        }
    }

    fn point_in_union(p: &[Q; 4], polys: &[Vec<P4>]) -> bool {
        for poly in polys {
            if strict_inside_q(p, poly) {
                return true;
            }
            let n = poly.len();
            for i in 0..n {
                if on_closed_seg_q(p, poly[i], poly[(i + 1) % n]) {
                    return true;
                }
            }
        }
        false
    }

    /// Exact: closed segment e is a subset of the closed union of polys.
    fn edge_covered(e: (P4, P4), polys: &[Vec<P4>]) -> bool {
        let (a, b) = e;
        let mut brk: BTreeSet<Q> = BTreeSet::new();
        brk.insert(Q::from_i(0));
        brk.insert(Q::from_i(1));
        for poly in polys {
            let n = poly.len();
            for i in 0..n {
                for &t in seg_hits(e, (poly[i], poly[(i + 1) % n])).iter() {
                    brk.insert(t);
                }
            }
            for &v in poly.iter() {
                if on_closed_seg(v, a, b) {
                    brk.insert(t_param(v, a, b));
                }
            }
        }
        let pts: Vec<Q> = brk.into_iter().collect();
        for w in 0..pts.len() - 1 {
            let (x, y) = (pts[w], pts[w + 1]);
            if x == y {
                continue;
            }
            let m = seg_pt(a, b, x.add(y).mul(Q::new(1, 2)));
            if !point_in_union(&m, polys) {
                return false;
            }
        }
        true
    }

    /// True iff open segment c meets the strict interior of parent.
    fn chain_edge_interior_hit(c: (P4, P4), parent: &[P4]) -> bool {
        let (u, v) = c;
        let mut brk: BTreeSet<Q> = BTreeSet::new();
        brk.insert(Q::from_i(0));
        brk.insert(Q::from_i(1));
        for i in 0..parent.len() {
            let pe = (parent[i], parent[(i + 1) % parent.len()]);
            for &t in seg_hits(c, pe).iter() {
                brk.insert(t);
            }
            for &w in [pe.0, pe.1].iter() {
                if on_closed_seg(w, u, v) {
                    brk.insert(t_param(w, u, v));
                }
            }
        }
        let pts: Vec<Q> = brk.into_iter().collect();
        for w in 0..pts.len() - 1 {
            let (x, y) = (pts[w], pts[w + 1]);
            if x == y {
                continue;
            }
            let m = seg_pt(u, v, x.add(y).mul(Q::new(1, 2)));
            if strict_inside_q(&m, parent) {
                return true;
            }
        }
        false
    }

    /// P ⊆ U  <=>  ∂P ⊆ closed union AND no union-boundary chain edge has
    /// an open sub-segment in the strict interior of P.
    fn parent_contained(parent: &[P4], polys: &[Vec<P4>], chain_edges: &[(P4, P4)]) -> bool {
        for i in 0..parent.len() {
            let e = (parent[i], parent[(i + 1) % parent.len()]);
            if !edge_covered(e, polys) {
                return false;
            }
        }
        for &ce in chain_edges {
            if chain_edge_interior_hit(ce, parent) {
                return false;
            }
        }
        true
    }

    // ---------- D1 angle census (w11 port) ----------

    struct Census {
        n_vertices: usize,
        n_interior: usize,
        n_corner: usize,
        n_pinch: usize,
        n_t: usize,
        sum_s: i32,
        bad: Vec<(P4, i32, i32)>,
        bad_t: usize,
        krot: Vec<(i8, u32)>,
        dirs: [u32; 12],
        n_chain_edges: usize,
        n_chain_vertices: usize,
        pinch: Vec<P4>,
    }

    /// Reorient a directed boundary chain to CCW world sense.
    fn orient_ccw(chain: &[(P4, P4)]) -> Vec<(P4, P4)> {
        let corn: Vec<P4> = chain.iter().map(|e| e.0).collect();
        if shoelace_signed(&corn).sign() < 0 {
            chain.iter().rev().map(|e| (e.1, e.0)).collect()
        } else {
            chain.to_vec()
        }
    }

    fn census_of(spec: &[(u8, i8, P4)], piece_specs: &[Vec<(u8, i8, P4)>]) -> Census {
        let cl = children_cors(spec);
        // corner-angle sum at each vertex (30° units)
        let mut corner_sum: BTreeMap<P4, i32> = BTreeMap::new();
        for (sh, _kr, _m4, cs) in &cl {
            for (j, v) in cs.iter().enumerate() {
                *corner_sum.entry(*v).or_insert(0) += ANGLE[*sh as usize][j];
            }
        }
        // T-points: tile corner strictly in the open interior of another edge
        let mut t_edges: BTreeMap<P4, i32> = BTreeMap::new();
        for (i, (_sh, _kr, _m4, cs)) in cl.iter().enumerate() {
            let n = cs.len();
            for k in 0..n {
                let (a, b) = (cs[k], cs[(k + 1) % n]);
                for (j, (_sh2, _kr2, _m42, cs2)) in cl.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    for &w in cs2 {
                        if w != a && w != b && on_open_seg(w, a, b) {
                            *t_edges.entry(w).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
        // boundary chains (per piece for rho, whole for tri/sq), CCW
        let mut chains: Vec<Vec<(P4, P4)>> = Vec::new();
        for ps in piece_specs {
            let b = boundary(ps);
            assert_eq!(b.len(), 1, "expected one boundary chain per piece");
            chains.push(orient_ccw(&b[0]));
        }
        // pass data at chain vertices: (d_in + 6 − d_out) mod 12 per pass
        let mut passes: BTreeMap<P4, Vec<i32>> = BTreeMap::new();
        let mut chain_vertices: BTreeSet<P4> = BTreeSet::new();
        let mut dirs = [0u32; 12];
        for ch in &chains {
            for (u, w) in ch.iter() {
                let (ang, _c0) =
                    edge_dir(sub4(*w, *u)).expect("chain edge is not a lattice direction");
                chain_vertices.insert(*u);
                chain_vertices.insert(*w);
                dirs[(ang / 30) as usize] += 1;
            }
        }
        for ch in &chains {
            let l = ch.len();
            for k in 0..l {
                let (u, w) = (ch[k].0, ch[k].1);
                let prev = ch[(k + l - 1) % l];
                let din = edge_dir(sub4(u, prev.0))
                    .expect("chain edge is not a lattice direction")
                    .0 / 30;
                let dout = edge_dir(sub4(w, u))
                    .expect("chain edge is not a lattice direction")
                    .0 / 30;
                passes.entry(u).or_default().push((din + 6 - dout).rem_euclid(12));
            }
        }
        // classify every vertex and check S(v) == want(v)
        let mut s: BTreeMap<P4, i32> = BTreeMap::new();
        for (v, csum) in &corner_sum {
            s.insert(*v, csum + 6 * t_edges.get(v).copied().unwrap_or(0));
        }
        let mut bad = Vec::new();
        let mut n_interior = 0usize;
        let mut n_corner = 0usize;
        let mut n_t = 0usize;
        let mut pinch: Vec<P4> = Vec::new();
        for (v, sv) in &s {
            let (want, is_corner) = if chain_vertices.contains(v) {
                let ps = passes.get(v).cloned().unwrap_or_default();
                if ps.len() == 2 {
                    pinch.push(*v);
                }
                (ps.iter().sum::<i32>().rem_euclid(12), true)
            } else {
                if t_edges.contains_key(v) {
                    n_t += 1;
                }
                (12, false)
            };
            if is_corner {
                n_corner += 1;
            } else {
                n_interior += 1;
            }
            if *sv != want {
                bad.push((*v, *sv, want));
            }
        }
        let chain_edges: Vec<(P4, P4)> = chains.iter().flatten().copied().collect();
        let bad_t = t_edges
            .keys()
            .filter(|v| chain_edges.iter().any(|e| on_open_seg(**v, e.0, e.1)))
            .count();
        let sum_s: i32 = s.values().sum();
        let mut krot: BTreeMap<i8, u32> = BTreeMap::new();
        for &(_sh, kr, _m4) in spec.iter() {
            *krot.entry(kr).or_insert(0) += 1;
        }
        Census {
            n_vertices: s.len(),
            n_interior,
            n_corner,
            n_pinch: pinch.len(),
            n_t,
            sum_s,
            bad,
            bad_t,
            krot: krot.into_iter().collect(),
            dirs,
            n_chain_edges: chain_edges.len(),
            n_chain_vertices: chain_vertices.len(),
            pinch,
        }
    }

    // ---------- D2 drift-guard assembly (w12 port) ----------

    fn d2_assembly(seed: u8) -> Vec<(Vec<P4>, usize)> {
        let mut out: Vec<(Vec<P4>, usize)> = Vec::new();
        for (pi, &(k1, kr1, a1)) in rows_global(seed).iter().enumerate() {
            for &(k2, kr2, m2) in rows_global(k1).iter() {
                let a2 = add4(a1, rot4(m2, kr1));
                let kr = (kr1 + kr2) % 12;
                let cs: Vec<P4> = base_corners(k2)
                    .iter()
                    .map(|c| add4(rot4(*c, kr), a2))
                    .collect();
                out.push((cs, pi));
            }
        }
        out
    }

    fn poly_area_f64(pts: &[Vec2]) -> f64 {
        let n = pts.len();
        let mut s = 0.0f64;
        for i in 0..n {
            let (a, b) = (pts[i], pts[(i + 1) % n]);
            s += a.x * b.y - b.x * a.y;
        }
        s.abs() / 2.0
    }

    fn shape_of_area2(a: Z3) -> u8 {
        match a {
            _ if a == Z3::new(0, 2) => TRI,
            _ if a == Z3::new(8, 0) => SQ,
            _ if a == Z3::new(4, 0) => RHO,
            other => panic!("unexpected 2-area {:?}", other),
        }
    }

    // ---------- tests ----------

    #[test]
    fn level1_tri_certified() {
        let spec = rows_global(TRI);
        assert_eq!(spec.len(), 11);
        let cl = children_cors(&spec);
        let mut cnt = [0u32; 3];
        for (sh, _kr, _m4, _cs) in &cl {
            cnt[*sh as usize] += 1;
        }
        assert_eq!((cnt[0], cnt[1], cnt[2]), (7, 2, 2), "tri-sup count vector");
        for i in 0..cl.len() {
            for j in (i + 1)..cl.len() {
                assert!(
                    !overlap_pair(&cl[i].3, &cl[j].3),
                    "tri-sup children {} / {} overlap",
                    i,
                    j
                );
            }
        }
        let b = boundary(&spec);
        assert_eq!(b.len(), 1, "tri-sup boundary: one closed chain");
        assert!(chain_closed(&b[0]));
        assert!(chain_simple(&b[0]), "tri-sup boundary: simple");
        assert_eq!(b[0].len(), 15, "tri-sup boundary edge count");
        let l2 = Z3::new(7, 4);
        let mut tot = Z3::ZERO;
        for (_sh, _kr, _m4, cs) in &cl {
            tot = tot.add(area2_of(cs));
        }
        assert_eq!(tot, l2.mul(Z3::new(0, 2)), "tri-sup 2-area = λ²·parent");
        let corn: Vec<P4> = b[0].iter().map(|e| e.0).collect();
        assert_eq!(area2_of(&corn), tot, "union area = child sum (exact partition)");
        for (_sh, _kr, _m4, cs) in &cl {
            for v in cs {
                assert!(inside_or_on(*v, &corn), "child corner outside the union chain");
            }
        }
        let polys: Vec<Vec<P4>> = cl.iter().map(|c| c.3.clone()).collect();
        assert!(parent_contained(&TRI_CORN, &polys, &b[0]), "tri parent ⊆ union");
    }

    #[test]
    fn level1_sq_certified() {
        let spec = rows_global(SQ);
        assert_eq!(spec.len(), 26);
        let cl = children_cors(&spec);
        let mut cnt = [0u32; 3];
        for (sh, _kr, _m4, _cs) in &cl {
            cnt[*sh as usize] += 1;
        }
        assert_eq!((cnt[0], cnt[1], cnt[2]), (16, 4, 6), "sq-sup count vector");
        for i in 0..cl.len() {
            for j in (i + 1)..cl.len() {
                assert!(
                    !overlap_pair(&cl[i].3, &cl[j].3),
                    "sq-sup children {} / {} overlap",
                    i,
                    j
                );
            }
        }
        let b = boundary(&spec);
        assert_eq!(b.len(), 1, "sq-sup boundary: one closed chain");
        assert!(chain_closed(&b[0]));
        assert!(chain_simple(&b[0]), "sq-sup boundary: simple");
        assert_eq!(b[0].len(), 20, "sq-sup boundary edge count");
        let l2 = Z3::new(7, 4);
        let mut tot = Z3::ZERO;
        for (_sh, _kr, _m4, cs) in &cl {
            tot = tot.add(area2_of(cs));
        }
        assert_eq!(tot, l2.mul(Z3::new(8, 0)), "sq-sup 2-area = λ²·parent");
        let corn: Vec<P4> = b[0].iter().map(|e| e.0).collect();
        assert_eq!(area2_of(&corn), tot, "union area = child sum (exact partition)");
        for (_sh, _kr, _m4, cs) in &cl {
            for v in cs {
                assert!(inside_or_on(*v, &corn), "child corner outside the union chain");
            }
        }
        let polys: Vec<Vec<P4>> = cl.iter().map(|c| c.3.clone()).collect();
        assert!(parent_contained(&SQ_CORN, &polys, &b[0]), "sq parent ⊆ union");
    }

    #[test]
    fn level1_rho_cluster() {
        let spec = rows_global(RHO);
        assert_eq!(spec.len(), 13);
        let cl = children_cors(&spec);
        let mut cnt = [0u32; 3];
        for (sh, _kr, _m4, _cs) in &cl {
            cnt[*sh as usize] += 1;
        }
        assert_eq!((cnt[0], cnt[1], cnt[2]), (8, 2, 3), "rho-sup count vector");

        // Per-piece: 9 / 2 / 2 tiles, simple closed boundaries of 10 / 5 / 5
        // edges, 2-areas (20,12) / (4,2) / (4,2).
        let piece_specs: Vec<Vec<(u8, i8, P4)>> = (0..3)
            .map(|p| {
                rows_table(RHO)
                    .iter()
                    .filter(|r| r.piece == p)
                    .map(|r| (r.kind, r.krot, add4(r.m, RHO_OFF[r.piece as usize])))
                    .collect()
            })
            .collect();
        let n_tiles = [9usize, 2, 2];
        let n_edges = [10usize, 5, 5];
        let areas = [Z3::new(20, 12), Z3::new(4, 2), Z3::new(4, 2)];
        for p in 0..3 {
            let ps = &piece_specs[p];
            assert_eq!(ps.len(), n_tiles[p], "piece {} tile count", p);
            let clp = children_cors(ps);
            for i in 0..clp.len() {
                for j in (i + 1)..clp.len() {
                    assert!(
                        !overlap_pair(&clp[i].3, &clp[j].3),
                        "piece {} tiles {} / {} overlap",
                        p,
                        i,
                        j
                    );
                }
            }
            let b = boundary(ps);
            assert_eq!(b.len(), 1, "piece {}: one closed chain", p);
            assert!(
                chain_closed(&b[0]) && chain_simple(&b[0]),
                "piece {}: simple boundary",
                p
            );
            assert_eq!(b[0].len(), n_edges[p], "piece {} boundary edge count", p);
            let mut tot = Z3::ZERO;
            for c in &clp {
                tot = tot.add(area2_of(&c.3));
            }
            assert_eq!(tot, areas[p], "piece {} 2-area", p);
        }

        let l2 = Z3::new(7, 4);
        let mut tot = Z3::ZERO;
        for c in &cl {
            tot = tot.add(area2_of(&c.3));
        }
        assert_eq!(tot, l2.mul(Z3::new(4, 0)), "rho-sup 2-area = λ²·parent");

        // Cross-piece: NO shared edges — the closed vertex sets meet only at
        // the two pinned pinches.
        let p1: P4 = (-1, 0, -2, -1);
        let p2: P4 = (2, 1, 1, 0);
        let piece_cors: Vec<Vec<P4>> = piece_specs
            .iter()
            .map(|ps| {
                let clp = children_cors(ps);
                clp.iter().flat_map(|c| c.3.clone()).collect()
            })
            .collect();
        let shared_1: BTreeSet<P4> = piece_cors[1]
            .iter()
            .filter(|v| piece_cors[0].contains(v))
            .copied()
            .collect();
        assert_eq!(shared_1, BTreeSet::from([p1]), "island–blob meet only at the pinned pinch");
        let shared_2: BTreeSet<P4> = piece_cors[2]
            .iter()
            .filter(|v| piece_cors[0].contains(v))
            .copied()
            .collect();
        assert_eq!(
            shared_2,
            BTreeSet::from([p2]),
            "pendant–blob meet only at the pinned pinch"
        );
        let shared_3: BTreeSet<P4> = piece_cors[1]
            .iter()
            .filter(|v| piece_cors[2].contains(v))
            .copied()
            .collect();
        assert!(shared_3.is_empty(), "island–pendant do not touch");

        // Pinch vertices: 4 tile corners each, corner-angle sum exactly 8.
        for (pv, label) in [(p1, "blob–island"), (p2, "blob–pendant")] {
            let mut nc = 0u32;
            let mut asum = 0i32;
            for (sh, _kr, _m4, cs) in &cl {
                for (j, v) in cs.iter().enumerate() {
                    if *v == pv {
                        nc += 1;
                        asum += ANGLE[*sh as usize][j];
                    }
                }
            }
            assert_eq!(nc, 4, "{} pinch: 4 tile corners", label);
            assert_eq!(asum, 8, "{} pinch: corner sum 8 (30° units)", label);
        }

        // Whole cluster: 3 chains, 20 directed boundary edges, 18 distinct
        // vertices, in-degree == out-degree at every vertex.
        // NOTE: the whole-cluster directed-edge walk may merge two piece
        // chains at a pinch (point contact: 4 directed edges through one
        // vertex), so no chain-count pin here; the pinned whole-cluster
        // invariants are 20 directed edges / 18 distinct vertices /
        // in == out (spike w10), asserted below. Per-piece one-simple-chain
        // pins are above.
        let b = boundary(&spec);
        let n_edges_tot: usize = b.iter().map(|ch| ch.len()).sum();
        assert_eq!(n_edges_tot, 20, "whole cluster: 20 directed boundary edges");
        let mut vs = BTreeSet::new();
        let mut din: BTreeMap<P4, i32> = BTreeMap::new();
        let mut dout: BTreeMap<P4, i32> = BTreeMap::new();
        for ch in &b {
            for e in ch {
                vs.insert(e.0);
                vs.insert(e.1);
                *din.entry(e.1).or_insert(0) += 1;
                *dout.entry(e.0).or_insert(0) += 1;
            }
        }
        assert_eq!(vs.len(), 18, "whole cluster: 18 distinct boundary vertices");
        for v in &vs {
            assert_eq!(
                din.get(v).copied().unwrap_or(0),
                dout.get(v).copied().unwrap_or(0),
                "in/out balance at {:?}",
                v
            );
        }

        // Parent rhombus ⊆ fitted 3-piece union (exact Q⁴).
        let polys: Vec<Vec<P4>> = cl.iter().map(|c| c.3.clone()).collect();
        let chain_edges: Vec<(P4, P4)> = b.iter().flatten().copied().collect();
        assert!(
            parent_contained(&RHO_CORN, &polys, &chain_edges),
            "rho parent ⊆ 3-piece union"
        );
    }

    #[test]
    fn inflation_area_law() {
        let l2 = Z3::new(7, 4);
        for kind in [TRI, SQ, RHO] {
            let cl = children_cors(&rows_global(kind));
            let mut tot = Z3::ZERO;
            for (_sh, _kr, _m4, cs) in &cl {
                tot = tot.add(area2_of(cs));
            }
            let a = Z3::new(AREA2[kind as usize][0], AREA2[kind as usize][1]);
            assert_eq!(tot, l2.mul(a), "2-area law for {:?}", kind);
        }
        // Float PF: child sum / parent area = 7 + 4√3 per shape.
        let pf = 7.0 + 4.0 * SQRT3_FLT;
        for kind in [TRI, SQ, RHO] {
            let cl = children_cors(&rows_global(kind));
            let mut u = 0.0f64;
            for (_sh, _kr, _m4, cs) in &cl {
                let pts: Vec<Vec2> = cs.iter().map(|v| val(*v)).collect();
                u += poly_area_f64(&pts);
            }
            let pts: Vec<Vec2> = base_corners(kind).iter().map(|v| val(*v)).collect();
            let parea = poly_area_f64(&pts);
            assert!((u / parea - pf).abs() < 1e-12, "float PF for {:?}", kind);
        }
    }

    #[test]
    fn orientation_census_pinned() {
        // tri-sup: 16 vertices (1 interior, 15 corner, 0 pinch), sum_S 90.
        {
            let spec = rows_global(TRI);
            let r = census_of(&spec, &[spec.clone()]);
            assert_eq!(r.n_vertices, 16);
            assert_eq!(r.n_interior, 1);
            assert_eq!(r.n_corner, 15);
            assert_eq!(r.n_pinch, 0);
            assert_eq!(r.n_t, 0);
            assert_eq!(r.sum_s, 90);
            assert!(r.bad.is_empty(), "tri vertex mismatches: {:?}", r.bad);
            assert_eq!(r.bad_t, 0);
            assert_eq!(r.krot, vec![(0, 5), (1, 1), (2, 1), (3, 3), (10, 1)]);
            assert_eq!(r.dirs, [2, 0, 0, 3, 2, 0, 0, 3, 2, 0, 0, 3]);
            assert_eq!(r.n_chain_edges, 15);
            assert_eq!(r.n_chain_vertices, 15);
        }
        // sq-sup: 29 vertices (9 interior, 20 corner), sum_S 216.
        {
            let spec = rows_global(SQ);
            let r = census_of(&spec, &[spec.clone()]);
            assert_eq!(r.n_vertices, 29);
            assert_eq!(r.n_interior, 9);
            assert_eq!(r.n_corner, 20);
            assert_eq!(r.n_pinch, 0);
            assert_eq!(r.n_t, 0);
            assert_eq!(r.sum_s, 216);
            assert!(r.bad.is_empty(), "sq vertex mismatches: {:?}", r.bad);
            assert_eq!(r.bad_t, 0);
            assert_eq!(r.krot, vec![(0, 7), (1, 6), (2, 6), (3, 5), (4, 1), (5, 1)]);
            assert_eq!(r.dirs, [3, 0, 2, 3, 0, 2, 3, 0, 2, 3, 0, 2]);
            assert_eq!(r.n_chain_edges, 20);
            assert_eq!(r.n_chain_vertices, 20);
        }
        // rho-sup: 20 vertices (2 interior, 18 corner, 2 pinch), sum_S 108,
        // pinches at (−1,0,−2,−1) and (2,1,1,0).
        {
            let spec = rows_global(RHO);
            let piece_specs: Vec<Vec<(u8, i8, P4)>> = (0..3)
                .map(|p| {
                    rows_table(RHO)
                        .iter()
                        .filter(|r| r.piece == p)
                        .map(|r| (r.kind, r.krot, add4(r.m, RHO_OFF[r.piece as usize])))
                        .collect()
                })
                .collect();
            let r = census_of(&spec, &piece_specs);
            assert_eq!(r.n_vertices, 20);
            assert_eq!(r.n_interior, 2);
            assert_eq!(r.n_corner, 18);
            assert_eq!(r.n_pinch, 2);
            assert_eq!(r.n_t, 0);
            assert_eq!(r.sum_s, 108);
            assert!(r.bad.is_empty(), "rho vertex mismatches: {:?}", r.bad);
            assert_eq!(r.bad_t, 0);
            assert_eq!(r.krot, vec![(0, 3), (1, 4), (2, 2), (3, 2), (4, 1), (10, 1)]);
            assert_eq!(r.dirs, [2, 4, 2, 0, 1, 1, 2, 4, 2, 0, 1, 1]);
            assert_eq!(r.n_chain_edges, 20);
            assert_eq!(r.n_chain_vertices, 18);
            let p1: P4 = (-1, 0, -2, -1);
            let p2: P4 = (2, 1, 1, 0);
            let set: BTreeSet<P4> = r.pinch.iter().copied().collect();
            assert_eq!(set, BTreeSet::from([p1, p2]), "pinned pinch vertices");
        }
    }

    #[test]
    fn d2_non_composition_drift_guard() {
        // Pinned exact evidence that the figure's rules do NOT compose at
        // depth 2 (spike w12): growing a single k=0 / anchor-0 parent to
        // depth 2 yields positive-area overlaps between tiles from DIFFERENT
        // D1 parents (pre-dedup, exact Z[√3] predicates), while same-parent
        // overlaps stay 0 (level-1 dissections are exact partitions).
        // Pins: (seed, n_tiles, (tri, sq, rho), overlap_pairs, same_parent).
        for (seed, n, counts, ov) in [
            (TRI, 155u32, (97u32, 26u32, 32u32), 770u32),
            (SQ, 358u32, (224u32, 60u32, 74u32), 3660u32),
            (RHO, 179u32, (112u32, 30u32, 37u32), 1023u32),
        ] {
            let tiles = d2_assembly(seed);
            assert_eq!(tiles.len() as u32, n, "d2 tile count {:?}", seed);
            let mut cnt = [0u32; 3];
            for (cs, _pi) in &tiles {
                cnt[shape_of_area2(area2_of(cs)) as usize] += 1;
            }
            assert_eq!((cnt[0], cnt[1], cnt[2]), counts, "d2 shape counts {:?}", seed);
            let mut ov2 = 0u32;
            let mut same = 0u32;
            for i in 0..tiles.len() {
                for j in (i + 1)..tiles.len() {
                    if overlap_pair(&tiles[i].0, &tiles[j].0) {
                        ov2 += 1;
                        if tiles[i].1 == tiles[j].1 {
                            same += 1;
                        }
                    }
                }
            }
            assert_eq!(ov2, ov, "d2 overlap pairs {:?}", seed);
            assert_eq!(same, 0, "d2 same-parent overlaps (must be 0) {:?}", seed);
        }
    }

    #[test]
    fn emission_patch_pinned() {
        let tiles = patch_tiles();
        assert_eq!(tiles.len(), 26);
        let cs: Vec<Vec<P4>> = tiles.iter().map(|t| t.1.clone()).collect();
        for i in 0..cs.len() {
            for j in (i + 1)..cs.len() {
                assert!(
                    !overlap_pair(&cs[i], &cs[j]),
                    "emitted tiles {} / {} overlap",
                    i,
                    j
                );
            }
        }
        let mut tot = Z3::ZERO;
        for c in &cs {
            tot = tot.add(area2_of(c));
        }
        assert_eq!(tot, Z3::new(56, 32), "emitted 2-area = sq-sup 2-area");
        let mut u = 0.0f64;
        for c in &cs {
            let pts: Vec<Vec2> = c.iter().map(|v| val(*v)).collect();
            u += poly_area_f64(&pts);
        }
        let want = (56.0 + 32.0 * SQRT3_FLT) * H * H / 2.0;
        assert!((u - want).abs() < 1e-9 * want, "emission float area");
    }
}

