//! Andritz 10gon-1 substitution — a genuinely quasiperiodic girih tiling
//! from Leonhard Andritz's fivefold substitution family (submitted to the
//! Tilings Encyclopedia, February 2016; entry *bowtie-hexagon-10gon-1*),
//! tiles inspired by Islamic girih patterns.
//!
//! This is the decagon-bearing sibling of
//! [`super::andritz_bowtie_hexagon`]: the same unit-edge BOW and HEX
//! girih shapes (here rotated by 18°, a factor of ζ₂₀), plus the regular
//! decagon.
//!
//! # The substitution system
//!
//! Three unit-edge girih prototiles, angles in multiples of 18°:
//!
//! * **BOW** (`TY_B`) — the concave bow-tie hexagon (area 1.31433);
//! * **HEX** (`TY_H`) — the elongated hexagon "tie" (long diagonal φ²,
//!   area 2.12663);
//! * **DEC** (`TY_D`) — the regular decagon (circumradius φ, area
//!   2.5·cot 18° = 7.69421).
//!
//! The inflation is **λ = φ²** with substitution matrix (rows = parent,
//! columns = BOW / HEX / DEC children)
//!
//! ```text
//! M = [[2, 3, 0],    BOW → 2 BOW + 3 HEX
//!      [2, 2, 1],    HEX → 2 BOW + 2 HEX + 1 DEC
//!      [5, 0, 6]]    DEC → 5 BOW + 6 DEC   (the 6-petal decagon flower)
//! ```
//!
//! whose Perron–Frobenius eigenvalue is φ⁴ = λ²; the area identities
//! 2A_B + 3A_H = φ⁴A_B, 2A_B + 2A_H + A_D = φ⁴A_H and
//! 5A_B + 6A_D = φ⁴A_D all hold exactly.
//!
//! # The rule
//!
//! As in [`super::andritz_bowtie_hexagon`], the rule is a **σ-frame
//! walk** (an overlapping-cluster / straddler substitution in the modern
//! SVT sense): a tile pose (kind, c, r) — center, and observable
//! rotation r ∈ 0..5 in 36° units (the DEC is rotation-blind, r := 0) —
//! claims children at
//!
//! ```text
//! C = μ·c + τ,   child = (kind_j, C + t^{Ψ(r)}·v_j, (r + d_j) mod 5)
//! ```
//!
//! with μ = φ²·t¹⁸ = (2, 0, −1, 0, 1, 0, −2, 0) and τ = (−13, 0, 16, 0,
//! 2, 0, 23, 0) in Z[ζ₂₀], t = e^{18°i}.  The frame rotation exponent is
//! Ψ_B(r) = 2r + 3 + 10·((r+1) mod 2) for BOW parents and Ψ_H(r) = 2r +
//! 8 + 10·(r mod 2) for HEX parents, so the slot table rotates with the
//! parent inside the ring.  The DEC row is *absolute* (no rotation
//! factor): the center DEC at C, five petal DECs at C + AP·t^{3+4k} (AP
//! = 2·apothem = (0, 2, 0, 0, 0, 1, 0, −2)) and five gap BOWs at
//! C + ½·AP·φ·t^{1+4k} with absolute rotations [3, 0, 2, 4, 1].  Unlike
//! landing-1 the state needs **no lift bookkeeping**: tiles are
//! C₂-symmetric so rotations r and r+5 emit the same tile at the same
//! center, and centers themselves are exact half-ring elements.
//!
//! Every claim lands on an exact tile of the tiling, and every tile of
//! the tiling is claimed (verified against the decoded Bielefeld
//! reference patch: all 454 exactly reconstructed tile poses are
//! reproduced by level 3, and the 54-pose inner disk matched with zero
//! discrepancy in both directions).
//!
//! # Exactness
//!
//! All positions live in ½·Z[ζ₂₀], ζ₂₀ = t = e^{2πi/20}, stored as
//! integer 8-tuples over (1, t, …, t⁷) with the Φ₂₀ reduction
//! t⁸ = t⁶ − t⁴ + t² − 1 (equivalently t¹⁰ = −1); tile *vertices* are
//! exact ring elements and centers exact half-ring elements.  The frame
//! constants, both rotation laws, all three slot tables and the
//! prototile vertices are exact; floats appear only when a point is
//! finally turned into a `Vec2` for emission.  Z[ζ₂₀] is dense in ℂ, so
//! exactness is *not* claimed from point-snapping: the tables were
//! derived from the girih **edge graph** of the decoded raster patch
//! (every girih edge is exactly a power of t; integer vertex coordinates
//! propagated by BFS with zero conflicts across 3280 shared edges) and
//! cross-instance tuple equality of every slot offset (26/26 BOW rows,
//! 10/10 HEX rows, 16/16 DEC flowers, 22/22 agreeing values of τ).  The
//! Rust structural tests re-verify: ring-exact unit edges and areas,
//! the Perron–Frobenius/area identities, the exact 54-pose inner patch
//! reproduced by the level-2 walk (sizes 14 → 86 → 604, ratio → φ⁴),
//! and an exact edge census (every interior edge shared by exactly two
//! tiles, none dangling) at radius 4.5 for level 1 and radius 8 for
//! level 2.
//!
//! # Generation for a view
//!
//! A wavefront BFS from the 14-tile seed core around the fixed point
//! o = τ/(1 − μ) grows the tiling outward; substitution level *k* covers
//! a disk of radius ≈ 4, 8, 18, 30, … (growing ×φ² per level, the
//! conservative table below).  The sweep is region-filtered through the
//! σ⁻¹ preimage box chain of the view (claims land within SLOT_REACH of
//! σ(parent), so only centers in the preimage boxes can descend into the
//! view), keeping the work linear in the view area even far from o.
//! `generate` BFS-iterates until the emitted patch provably covers the
//! view (clipped-area equality, as for [`super::girih_qc`]); tile
//! emission is unit-edge at every level.  Views whose far corner lies
//! beyond the level-6 covered radius (≈ 380) or whose area would exceed
//! [`MAX_TILES`] fail with a zoom-in message.

use std::collections::HashSet;
#[cfg(test)]
use std::collections::{BTreeMap, BTreeSet};

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

// =============================================================
// Exact arithmetic in R = Z[ζ₂₀], t = e^{2πi/20} (basis 1, t, …, t⁷;
// Φ₂₀(t) = t⁸ − t⁶ + t⁴ − t² + 1, i.e. t⁸ = t⁶ − t⁴ + t² − 1 and
// t¹⁰ = −1).  Integer 8-tuples, no denominators.
// =============================================================

/// Basis values t^k (float evaluation at the emission boundary).
const TB: [(f64, f64); 8] = [
    (1.0, 0.0),
    (0.9510565162951535, 0.3090169943749474),
    (0.8090169943749475, 0.5877852522924731),
    (0.5877852522924731, 0.8090169943749475),
    (0.30901699437494745, 0.9510565162951536),
    (0.0, 1.0),
    (-0.30901699437494734, 0.9510565162951536),
    (-0.5877852522924731, 0.8090169943749475),
];

/// Prototile codes.
const TY_B: u8 = 1; // BOW: concave bow-tie hexagon
const TY_H: u8 = 2; // HEX: elongated hexagon "tie"
const TY_D: u8 = 3; // DEC: regular decagon (rotation-blind)

type Ke = [i64; 8];

#[inline]
fn kadd(x: Ke, y: Ke) -> Ke {
    [
        x[0] + y[0],
        x[1] + y[1],
        x[2] + y[2],
        x[3] + y[3],
        x[4] + y[4],
        x[5] + y[5],
        x[6] + y[6],
        x[7] + y[7],
    ]
}

#[cfg(test)]
#[inline]
fn ksub(x: Ke, y: Ke) -> Ke {
    kadd(x, kneg(y))
}

#[cfg(test)]
#[inline]
fn kneg(x: Ke) -> Ke {
    [-x[0], -x[1], -x[2], -x[3], -x[4], -x[5], -x[6], -x[7]]
}

/// Ring product: reduce x·y modulo Φ₂₀.  Degrees 10..14 fold by
/// t^{10+j} = −t^j; then t⁸ → (−1, 0, 1, 0, −1, 0, 1, 0) and
/// t⁹ → (0, −1, 0, 1, 0, −1, 0, 1).
#[inline]
const fn kmul(x: Ke, y: Ke) -> Ke {
    let mut p = [0i64; 15];
    let mut i = 0;
    while i < 8 {
        if x[i] != 0 {
            let mut j = 0;
            while j < 8 {
                p[i + j] += x[i] * y[j];
                j += 1;
            }
        }
        i += 1;
    }
    let c8 = p[8];
    let c9 = p[9];
    [
        p[0] - p[10] - c8,
        p[1] - p[11] - c9,
        p[2] - p[12] + c8,
        p[3] - p[13] + c9,
        p[4] - p[14] - c8,
        p[5] - c9,
        p[6] + c8,
        p[7] + c9,
    ]
}

/// Complex conjugation t^k ↦ t^{−k} (= t^{20−k}), exact over the basis:
/// t^{-k} for k = 0..7 is (1, −t⁹, −t⁸, −t⁷, −t⁶, −t⁵, −t⁴, −t³)
/// reduced.
#[cfg(test)]
#[inline]
fn kconj(x: Ke) -> Ke {
    const C: [Ke; 8] = [
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, -1, 0, 1, 0, -1], // t⁻¹ = t¹⁹ = −t⁹
        [1, 0, -1, 0, 1, 0, -1, 0], // t⁻² = t¹⁸ = −t⁸
        [0, 0, 0, 0, 0, 0, 0, -1],  // t⁻³ = t¹⁷ = −t⁷
        [0, 0, 0, 0, 0, 0, -1, 0],  // t⁻⁴ = t¹⁶ = −t⁶
        [0, 0, 0, 0, 0, -1, 0, 0],  // t⁻⁵ = t¹⁵ = −t⁵
        [0, 0, 0, 0, -1, 0, 0, 0],  // t⁻⁶ = t¹⁴ = −t⁴
        [0, 0, 0, -1, 0, 0, 0, 0],  // t⁻⁷ = t¹³ = −t³
    ];
    let mut r = [0i64; 8];
    let mut i = 0;
    while i < 8 {
        if x[i] != 0 {
            let mut j = 0;
            while j < 8 {
                r[j] += x[i] * C[i][j];
                j += 1;
            }
        }
        i += 1;
    }
    r
}

/// Float value of a ring element (emission boundary only).
#[inline]
fn kv(k: Ke) -> Vec2 {
    let mut x = TB[0].0 * k[0] as f64;
    let mut y = TB[0].1 * k[0] as f64;
    for i in 1..8 {
        x += TB[i].0 * k[i] as f64;
        y += TB[i].1 * k[i] as f64;
    }
    Vec2::new(x, y)
}

/// Float value of a half-ring element (centers are stored as 2·c).
#[inline]
fn kv2(k: Ke) -> Vec2 {
    let v = kv(k);
    Vec2::new(0.5 * v.x, 0.5 * v.y)
}

/// t = e^{18°i} and its powers up to t¹⁹ (all ring elements, t¹⁰ = −1).
const TP: [Ke; 20] = {
    let mut t = [[0i64; 8]; 20];
    t[0] = [1, 0, 0, 0, 0, 0, 0, 0];
    let one = [0i64, 1, 0, 0, 0, 0, 0, 0];
    let mut i = 1;
    while i < 20 {
        t[i] = kmul(t[i - 1], one);
        i += 1;
    }
    t
};

/// μ = φ²·t¹⁸ (the frame contraction) and τ, the frame translation —
/// σ(c) = μ·c + τ, pinned by 22/22 identical DEC center-child claims in
/// the exact reference patch.  τ is stored as a ring element; the walk
/// works on doubled centers, so it uses TAU2 = 2τ.
const MU: Ke = [2, 0, -1, 0, 1, 0, -2, 0];
const TAU: Ke = [-13, 0, 16, 0, 2, 0, 23, 0];
const TAU2: Ke = [-26, 0, 32, 0, 4, 0, 46, 0];

/// φ and 2·apothem of the unit DEC (ring elements).  AP = 2φ·cos18° has
/// magnitude 3.0777; the DEC petal offset is AP·t^{3+4k} and the gap
/// BOW offset (AP·φ/2)·t^{1+4k} with GBP = AP·φ a doubled numerator.
#[cfg(test)]
const PHI: Ke = [1, 0, 0, 0, 1, 0, -1, 0];
const AP: Ke = [0, 2, 0, 0, 0, 1, 0, -2];
const GBP: Ke = [0, 4, 0, -1, 0, 2, 0, -3];

// =============================================================
// The substitution tables (derived in the exact Python prototype from
// the girih edge-graph reconstruction of the decoded Bielefeld patch:
// 26/26 BOW parents, 10/10 HEX parents and 16/16 DEC flowers give
// *identical* slot tuples after de-rotating by t^{Ψ(r)}).
//
// Entry: (child kind, slot numerator v — a doubled center offset, and
// child rotation d_j mod 5 relative to the parent rotation).
// =============================================================

/// BOW parent: 2 BOW + 3 HEX in the frame rotated by t^{Ψ_B(r)}.
const BROW: [(u8, Ke, u8); 5] = [
    (TY_B, [0, -2, 0, 1, 0, -1, 0, 1], 4),
    (TY_B, [0, 2, 0, -1, 0, 1, 0, -1], 4),
    (TY_H, [0, -4, 0, 1, 0, -2, 0, 3], 4),
    (TY_H, [0, 2, 0, 0, 0, 2, 0, -1], 2),
    (TY_H, [0, 2, 0, -1, 0, 0, 0, -2], 1),
];

/// HEX parent: 2 HEX + 2 BOW + 1 DEC in the frame rotated by t^{Ψ_H(r)}.
const HROW: [(u8, Ke, u8); 5] = [
    (TY_H, [-4, 0, 1, 0, -2, 0, 3, 0], 0),
    (TY_B, [-2, 0, 0, 0, -1, 0, 2, 0], 3),
    (TY_B, [2, 0, 0, 0, 2, 0, -1, 0], 0),
    (TY_H, [3, 0, 1, 0, 2, 0, -1, 0], 3),
    (TY_D, [1, 0, -2, 0, 1, 0, -1, 0], 0),
];

/// DEC gap-BOW absolute rotations, k = 0..4 (the DEC row carries no
/// parent rotation factor; verified on 16/16 DEC flowers).
const GAPR: [u8; 5] = [3, 0, 2, 4, 1];

/// Frame rotation exponent Ψ(r) in t-power units.
#[inline]
fn psi_m(ty: u8, r: u8) -> usize {
    let r = r as usize;
    if ty == TY_B {
        (2 * r + 3 + 10 * ((r + 1) % 2)) % 20
    } else {
        (2 * r + 8 + 10 * (r % 2)) % 20
    }
}

/// Prototile local vertices (ring-exact, anticlockwise, center at the
/// arithmetic mean; DEC at the origin).  The doubled-center difference
/// `2·L − CEN` below is exactly `2·(L − center)`.
const BOW_L: [Ke; 6] = [
    [0, 0, 0, 0, -1, 0, -1, 0],
    [1, 0, 0, 0, -1, 0, -1, 0],
    [1, 0, 0, 0, -1, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, -1, 0],
];
const HEX_L: [Ke; 6] = [
    [1, 0, -1, 0, 1, 0, -1, 0],
    [2, 0, -1, 0, 1, 0, -1, 0],
    [2, 0, 0, 0, 1, 0, -1, 0],
    [1, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
];
const DEC_L: [Ke; 10] = [
    [1, 0, 0, 0, 1, 0, -1, 0],
    [1, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 1, 0],
    [-1, 0, 1, 0, 0, 0, 1, 0],
    [-1, 0, 0, 0, 0, 0, 1, 0],
    [-1, 0, 0, 0, -1, 0, 1, 0],
    [-1, 0, 0, 0, -1, 0, 0, 0],
    [0, 0, -1, 0, 0, 0, -1, 0],
    [1, 0, -1, 0, 0, 0, -1, 0],
    [1, 0, 0, 0, 0, 0, -1, 0],
];

/// Doubled local centers, 2·(ΣL)/n = L₀ + L₃ (C₂ symmetry pairs the
/// vertices; the DEC is centered at 0).
const CENB: Ke = [1, 0, 0, 0, -1, 0, -1, 0];
const CENH: Ke = [2, 0, 0, 0, 1, 0, -1, 0];

// ===== seed core =====

/// The seed core: the 14 reference-patch tiles whose centers lie within
/// radius 4 of the fixed point o = τ/(1 − μ), as exact (kind, doubled
/// center, r mod 5) poses.
const SEED: [(u8, Ke, u8); 14] = [
    (TY_D, [22, 0, -4, 0, 10, 0, -18, 0], 0),
    (TY_B, [23, 0, -8, 0, 9, 0, -24, 0], 1),
    (TY_B, [23, 0, -5, 0, 10, 0, -21, 0], 3),
    (TY_D, [24, 0, -6, 0, 10, 0, -24, 0], 0),
    (TY_H, [24, 0, -3, 0, 13, 0, -19, 0], 2),
    (TY_H, [25, 0, -9, 0, 9, 0, -26, 0], 1),
    (TY_H, [25, 0, -5, 0, 12, 0, -22, 0], 3),
    (TY_B, [25, 0, -4, 0, 13, 0, -22, 0], 1),
    (TY_D, [26, 0, -2, 0, 14, 0, -22, 0], 0),
    (TY_D, [28, 0, -8, 0, 10, 0, -28, 0], 0),
    (TY_B, [28, 0, -7, 0, 12, 0, -27, 0], 2),
    (TY_D, [28, 0, -6, 0, 14, 0, -26, 0], 0),
    (TY_H, [30, 0, -4, 0, 15, 0, -25, 0], 0),
    (TY_B, [31, 0, -6, 0, 15, 0, -27, 0], 0),
];

/// The decoded Bielefeld reference patch: all exactly reconstructed
/// tile poses with center radius < 8 around o (54 tiles).  The walk
/// test asserts the level-2 walk reproduces this set *exactly*, in
/// both directions.
#[cfg(test)]
const PATCH_REF: [(u8, Ke, u8); 54] = [
    (TY_B, [16, 0, -5, 0, 6, 0, -17, 0], 2),
    (TY_B, [18, 0, -1, 0, 10, 0, -15, 0], 2),
    (TY_B, [19, 0, -8, 0, 6, 0, -21, 0], 4),
    (TY_B, [19, 0, -4, 0, 9, 0, -17, 0], 0),
    (TY_B, [19, 0, 0, 0, 12, 0, -13, 0], 4),
    (TY_B, [22, 0, -3, 0, 12, 0, -17, 0], 2),
    (TY_B, [23, 0, -12, 0, 7, 0, -27, 0], 0),
    (TY_B, [23, 0, -8, 0, 9, 0, -24, 0], 1),
    (TY_B, [23, 0, -5, 0, 10, 0, -21, 0], 3),
    (TY_B, [23, 0, 0, 0, 15, 0, -16, 0], 1),
    (TY_B, [25, 0, -4, 0, 13, 0, -22, 0], 1),
    (TY_B, [25, 0, -1, 0, 14, 0, -19, 0], 3),
    (TY_B, [27, 0, -13, 0, 8, 0, -31, 0], 3),
    (TY_B, [27, 0, -10, 0, 9, 0, -28, 0], 1),
    (TY_B, [28, 0, -7, 0, 12, 0, -27, 0], 2),
    (TY_B, [29, 0, -12, 0, 11, 0, -32, 0], 1),
    (TY_B, [29, 0, -2, 0, 15, 0, -23, 0], 0),
    (TY_B, [29, 0, -2, 0, 17, 0, -21, 0], 0),
    (TY_B, [31, 0, -10, 0, 12, 0, -31, 0], 4),
    (TY_B, [31, 0, -6, 0, 15, 0, -27, 0], 0),
    (TY_B, [32, 0, -1, 0, 18, 0, -23, 0], 2),
    (TY_B, [35, 0, -10, 0, 15, 0, -34, 0], 1),
    (TY_B, [35, 0, -7, 0, 16, 0, -31, 0], 3),
    (TY_B, [35, 0, -4, 0, 18, 0, -27, 0], 4),
    (TY_D, [16, 0, -6, 0, 4, 0, -18, 0], 0),
    (TY_D, [16, 0, -4, 0, 8, 0, -16, 0], 0),
    (TY_D, [20, 0, -6, 0, 8, 0, -20, 0], 0),
    (TY_D, [22, 0, -10, 0, 8, 0, -24, 0], 0),
    (TY_D, [22, 0, -4, 0, 10, 0, -18, 0], 0),
    (TY_D, [22, 0, -2, 0, 14, 0, -16, 0], 0),
    (TY_D, [24, 0, -6, 0, 10, 0, -24, 0], 0),
    (TY_D, [26, 0, -12, 0, 8, 0, -28, 0], 0),
    (TY_D, [26, 0, -2, 0, 14, 0, -22, 0], 0),
    (TY_D, [28, 0, -8, 0, 10, 0, -28, 0], 0),
    (TY_D, [28, 0, -6, 0, 14, 0, -26, 0], 0),
    (TY_D, [32, 0, -8, 0, 14, 0, -30, 0], 0),
    (TY_D, [32, 0, -2, 0, 16, 0, -24, 0], 0),
    (TY_D, [34, 0, -12, 0, 14, 0, -34, 0], 0),
    (TY_D, [34, 0, -6, 0, 16, 0, -28, 0], 0),
    (TY_D, [36, 0, -8, 0, 16, 0, -34, 0], 0),
    (TY_H, [18, 0, -9, 0, 5, 0, -22, 0], 4),
    (TY_H, [18, 0, -2, 0, 9, 0, -15, 0], 0),
    (TY_H, [18, 0, -1, 0, 11, 0, -14, 0], 4),
    (TY_H, [24, 0, -3, 0, 13, 0, -19, 0], 2),
    (TY_H, [25, 0, -9, 0, 9, 0, -26, 0], 1),
    (TY_H, [25, 0, -5, 0, 12, 0, -22, 0], 3),
    (TY_H, [25, 0, -1, 0, 15, 0, -18, 0], 1),
    (TY_H, [27, 0, -1, 0, 16, 0, -20, 0], 3),
    (TY_H, [28, 0, 0, 0, 17, 0, -19, 0], 0),
    (TY_H, [29, 0, -13, 0, 10, 0, -32, 0], 3),
    (TY_H, [30, 0, -11, 0, 11, 0, -32, 0], 4),
    (TY_H, [30, 0, -4, 0, 15, 0, -25, 0], 0),
    (TY_H, [30, 0, -1, 0, 17, 0, -21, 0], 2),
    (TY_H, [37, 0, -7, 0, 18, 0, -32, 0], 3),
];

// ===== generation bookkeeping =====

/// φ² (float; the covered-radius contraction per σ⁻¹ step; the tests
/// assert λ² = φ⁴ through it).
#[cfg(test)]
const PHI2_F: f64 = 2.618033988749895;

/// 1/μ = e^{36°i}/φ² (float): σ⁻¹ rotates the preimage box chain.
const IM_X: f64 = 0.30901699437494745; // cos 36° / φ²
const IM_Y: f64 = 0.2245139882897927; // sin 36° / φ²

/// Fixed point of σ: o = τ/(1 − μ) (float; used only for window tests
/// and the preimage box chain).
fn o_fix() -> Vec2 {
    let t = kv(TAU);
    let m = kv(MU);
    let dx = 1.0 - m.x;
    let dy = -m.y;
    let n = dx * dx + dy * dy;
    Vec2::new((t.x * dx + t.y * dy) / n, (t.y * dx - t.x * dy) / n)
}

/// Tile areas at the prototile vertices (float, emission-boundary tests
/// only; the exact identities are checked in the tests).
const AREA_BOW: f64 = 1.3143277802978344;
const AREA_HEX: f64 = 2.1266270208801;
const AREA_DEC: f64 = 7.694208842938135;

/// Mean tile area at the substitution frequencies (left PF eigenvector
/// of M, BOW/HEX/DEC → 0.42705/0.26393/0.30902): the tile-count
/// guard's area-per-tile estimate.
const FREQ_BOW: f64 = 0.42705098300936906;
const FREQ_HEX: f64 = 0.26393202250281154;
const FREQ_DEC: f64 = 0.3090169943749474;
const MEAN_AREA: f64 = FREQ_BOW * AREA_BOW + FREQ_HEX * AREA_HEX + FREQ_DEC * AREA_DEC;

/// Guaranteed-covered disk radius around o after each BFS level
/// (measured 4.24 / 8.22 / 18.5 at levels 0–2 against the decoded
/// patch; level 3 already reproduces every reconstructed tile out to
/// radius 36 — the table is rounded down, levels 3–6 follow the ×φ²
/// growth conservatively, and the area acceptance escalates a level if
/// it was optimistic).
const COVERED: [f64; MAX_LEVEL + 1] = [4.0, 8.0, 18.0, 30.0, 70.0, 160.0, 380.0];

/// Maximum BFS level (covered radius ≈ 380).
const MAX_LEVEL: usize = 6;

/// Sanity cap on total BFS claims processed (the region-filtered sweep
/// is linear in the view area; this only catches degenerate pathology).
const WORK_CAP: usize = 20_000_000;

/// A tile's center is at most φ (DEC circumradius) from any of its
/// points, so a tile reaching the view has its center within
/// TILE_REACH of the region.
const TILE_REACH: f64 = 1.7;

/// Largest slot offset |v|/2 in the claim formula (DEC petals at
/// |AP| = 3.0777 dominate): a claim lands at σ(parent) + t^Ψ·v with
/// offset ≤ SLOT_REACH.
const SLOT_REACH: f64 = 3.1;

/// Add the area of a tile clipped to the region (fast path for fully
/// contained tiles, as in [`super::girih_qc`]).
fn clip_add(ob: &Obj, region: BBox, acc: &mut f64) {
    let poly = Polygon::from_slice(&poly_pts(ob));
    let bb = poly.aabb();
    if !bb.intersects(region) {
        return;
    }
    if region.min.x <= bb.min.x
        && region.min.y <= bb.min.y
        && region.max.x >= bb.max.x
        && region.max.y >= bb.max.y
    {
        *acc += poly.area();
    } else if let Some(c) = poly.clipped_to(&region) {
        *acc += c.area();
    }
}

/// One tile of the walk: prototile, rotation r mod 5 (DEC: 0) and the
/// exact doubled center.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Obj {
    ty: u8,
    rot: u8,
    pos: Ke,
}

#[inline]
fn obj_key(o: &Obj) -> [i64; 10] {
    [
        o.ty as i64,
        o.rot as i64,
        o.pos[0],
        o.pos[1],
        o.pos[2],
        o.pos[3],
        o.pos[4],
        o.pos[5],
        o.pos[6],
        o.pos[7],
    ]
}

/// The exact vertex numerators (doubled) of a pose.
fn verts_exact(o: &Obj) -> Vec<Ke> {
    if o.ty == TY_D {
        DEC_L.iter().map(|l| kadd(o.pos, kadd(*l, *l))).collect()
    } else {
        let (locals, cen) = if o.ty == TY_B {
            (&BOW_L, CENB)
        } else {
            (&HEX_L, CENH)
        };
        let q = TP[(2 * o.rot) as usize];
        locals
            .iter()
            .map(|l| {
                let d = [
                    2 * l[0] - cen[0],
                    2 * l[1] - cen[1],
                    2 * l[2] - cen[2],
                    2 * l[3] - cen[3],
                    2 * l[4] - cen[4],
                    2 * l[5] - cen[5],
                    2 * l[6] - cen[6],
                    2 * l[7] - cen[7],
                ];
                kadd(o.pos, kmul(q, d))
            })
            .collect()
    }
}

/// The frame claims of one pose, appended to `out` (BOW/HEX rows
/// through the rotating frame; the DEC row absolute).
fn children_into(o: &Obj, out: &mut Vec<Obj>) {
    let w0 = kadd(kmul(MU, o.pos), TAU2);
    if o.ty == TY_D {
        out.push(Obj {
            ty: TY_D,
            rot: 0,
            pos: w0,
        });
        for kk in 0..5usize {
            let v = kmul(AP, TP[(3 + 4 * kk) % 20]);
            out.push(Obj {
                ty: TY_D,
                rot: 0,
                pos: kadd(w0, kadd(v, v)),
            });
        }
        for kk in 0..5usize {
            let v = kmul(GBP, TP[(1 + 4 * kk) % 20]);
            out.push(Obj {
                ty: TY_B,
                rot: GAPR[kk],
                pos: kadd(w0, v),
            });
        }
        return;
    }
    let row: &[(u8, Ke, u8)] = if o.ty == TY_B { &BROW } else { &HROW };
    let q = TP[psi_m(o.ty, o.rot)];
    for &(tyj, v, dj) in row {
        let rot = if tyj == TY_D { 0 } else { (o.rot + dj) % 5 };
        out.push(Obj {
            ty: tyj,
            rot,
            pos: kadd(w0, kmul(q, v)),
        });
    }
}

/// Float polygon of a tile: doubled exact vertices scaled by ½ at the
/// end (exact rotation, float only at emission).
fn poly_pts(o: &Obj) -> Vec<Vec2> {
    verts_exact(o).iter().map(|v| kv2(*v)).collect()
}

/// Output prototile index: BOW → 0, HEX → 1, DEC → 2.
fn proto_of(ty: u8) -> u8 {
    match ty {
        TY_B => 0,
        TY_H => 1,
        _ => 2,
    }
}

/// BFS the walk from the seed core to `max_level` (test helper).
#[cfg(test)]
fn walk(max_level: usize) -> (Vec<usize>, BTreeSet<[i64; 10]>) {
    let mut seen: BTreeSet<[i64; 10]> = BTreeSet::new();
    let mut sizes = Vec::new();
    let mut frontier: Vec<Obj> = Vec::new();
    for &(ty, pos, rot) in SEED.iter() {
        let o = Obj { ty, rot, pos };
        seen.insert(obj_key(&o));
        frontier.push(o);
    }
    sizes.push(frontier.len());
    for _ in 0..max_level {
        let mut next: Vec<Obj> = Vec::new();
        let mut children: Vec<Obj> = Vec::new();
        for o in &frontier {
            children.clear();
            children_into(o, &mut children);
            for c in children.drain(..) {
                if seen.insert(obj_key(&c)) {
                    next.push(c);
                }
            }
        }
        sizes.push(next.len());
        frontier = next;
    }
    (sizes, seen)
}

/// Andritz 10gon-1 (quasiperiodic; Andritz 2016 fivefold substitution,
/// bowtie–hexagon–decagon).
pub struct AndritzBowtieHexagon10gon1Tiling;

impl Tiling for AndritzBowtieHexagon10gon1Tiling {
    fn id(&self) -> &'static str {
        "andritz-bowtie-hexagon-10gon-1"
    }

    fn name(&self) -> &'static str {
        "Andritz 10gon-1 (quasiperiodic)"
    }

    fn description(&self) -> &'static str {
        "Quasiperiodic girih tiling from Andritz's 2016 fivefold substitution (10gon-1): bow-tie, elongated-hexagon and regular-decagon tiles, inflation φ² with matrix [[2,3,0],[2,2,1],[5,0,6]] (PF = φ⁴); an exact Z[ζ₂₀] σ-frame walk grown from a 14-tile seed core, edge-to-edge at every level."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Quasiperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn default_color_mode(&self) -> ColorMode {
        ColorMode::Proto
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "quasiperiodic; Andritz 2016 fivefold substitution (10gon-1); inflation phi^2; sigma-frame walk; edge-to-edge",
        );
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Region in the tiling's own (unit-edge) coordinates, plus the
        // farthest corner distance from the walk's fixed point.
        let o = o_fix();
        let mut rmin = Vec2::new(f64::MAX, f64::MAX);
        let mut rmax = Vec2::new(f64::MIN, f64::MIN);
        let mut rcorr = 0.0f64;
        for &corner in &query.region.corners() {
            let q = inv.transform(corner);
            rmin.x = rmin.x.min(q.x);
            rmin.y = rmin.y.min(q.y);
            rmax.x = rmax.x.max(q.x);
            rmax.y = rmax.y.max(q.y);
            rcorr = rcorr.max((q - o).length());
        }
        let region = BBox::new(rmin, rmax);
        if region.area() <= 1e-12 {
            return fail("empty view");
        }
        // Tiles are unit-edge with mean area MEAN_AREA at the
        // substitution frequencies, so this bounds the tile count
        // before any work.
        if region.area() > MAX_TILES as f64 * MEAN_AREA {
            return fail("too many tiles for this view — zoom in");
        }
        if rcorr > COVERED[MAX_LEVEL] {
            return fail("too many tiles for this view — zoom in");
        }

        let want = region.area();

        // Region-filtered walk.  A tile reaching the view has its
        // center within TILE_REACH of the region, and a claim lands at
        // σ(parent) + t^Ψ·v with |v|/2 ≤ SLOT_REACH, so the centers of
        // tiles whose j-step descendant chain first touches the book
        // box B₀ live in the preimage box chain B_j: centers march
        // toward o under σ⁻¹ (contraction φ² with a 36° rotation, so
        // each preimage box is rotated — its axis-aligned bounding box
        // is kept, conservatively).  Keeping at BFS level L exactly the
        // tiles of B₀ ∪ … ∪ B_{MAX_LEVEL−L} is a conservative
        // necessary condition for reaching the view, so the sweep stays
        // linear in the view area even for views far from the fixed
        // point.
        let mut boxes = Vec::with_capacity(MAX_LEVEL + 1);
        boxes.push(BBox::new(
            Vec2::new(region.min.x - TILE_REACH, region.min.y - TILE_REACH),
            Vec2::new(region.max.x + TILE_REACH, region.max.y + TILE_REACH),
        ));
        for _ in 0..MAX_LEVEL {
            let b = boxes[boxes.len() - 1];
            let c = Vec2::new(0.5 * (b.min.x + b.max.x), 0.5 * (b.min.y + b.max.y));
            let a = c - o;
            let cc = Vec2::new(o.x + a.x * IM_X - a.y * IM_Y, o.y + a.x * IM_Y + a.y * IM_X);
            let hx = 0.5 * (b.max.x - b.min.x) + SLOT_REACH;
            let hy = 0.5 * (b.max.y - b.min.y) + SLOT_REACH;
            let hxs = IM_X.abs() * hx + IM_Y.abs() * hy;
            let hys = IM_Y.abs() * hx + IM_X.abs() * hy;
            boxes.push(BBox::new(
                Vec2::new(cc.x - hxs, cc.y - hys),
                Vec2::new(cc.x + hxs, cc.y + hys),
            ));
        }
        let in_chain = |p: Vec2, maxj: usize| -> bool {
            boxes[..maxj.min(MAX_LEVEL) + 1]
                .iter()
                .any(|b| b.contains_point(p))
        };

        // BFS the walk outward from the seed core.  Book tiles (box B₀)
        // drive the acceptance and the emission; frontier tiles are the
        // potential ancestors of book tiles at deeper levels.  Tiles
        // are non-overlapping, so the sum of clipped areas reaches the
        // region area exactly when the view is fully covered.
        let mut seen: HashSet<[i64; 10]> = HashSet::new();
        let mut book: Vec<Obj> = Vec::new();
        let mut frontier: Vec<Obj> = Vec::new();
        let mut acc = 0.0;
        for &(ty, pos, rot) in SEED.iter() {
            let ob = Obj { ty, rot, pos };
            seen.insert(obj_key(&ob));
            frontier.push(ob);
            if in_chain(kv2(pos), 0) {
                clip_add(&ob, region, &mut acc);
                book.push(ob);
            }
        }
        let mut covered = acc >= want * (1.0 - 1e-9);
        let mut work = frontier.len();
        let mut level = 0;
        let mut children: Vec<Obj> = Vec::new();
        while !covered && level < MAX_LEVEL {
            level += 1;
            let mut next: Vec<Obj> = Vec::new();
            for ob in &frontier {
                children.clear();
                children_into(ob, &mut children);
                work += children.len();
                for c in children.drain(..) {
                    if !seen.insert(obj_key(&c)) {
                        continue;
                    }
                    let p = kv2(c.pos);
                    if in_chain(p, 0) {
                        clip_add(&c, region, &mut acc);
                        book.push(c);
                    }
                    if level < MAX_LEVEL && in_chain(p, MAX_LEVEL - level) {
                        next.push(c);
                    }
                }
            }
            frontier = next;
            if acc >= want * (1.0 - 1e-9) {
                covered = true;
                break;
            }
            if work > WORK_CAP {
                return fail("too many tiles for this view — zoom in");
            }
        }
        if !covered {
            return fail("too many tiles for this view — zoom in");
        }

        // Emit exactly the tiles intersecting the query region.
        let mut tiles = Vec::with_capacity(book.len().min(4096));
        for ob in &book {
            let pts = poly_pts(ob);
            let poly = Polygon::from_slice(&pts).transform(&map);
            if poly.aabb().intersects(query.region) {
                let proto = proto_of(ob.ty);
                tiles.push(Tile {
                    proto,
                    seed: proto as u32, level: 0,
                    poly,
                });
            }
        }
        if tiles.len() > MAX_TILES {
            return fail("too many tiles for this view — zoom in");
        }

        TilingResult {
            tiles,
            symmetry: meta,
            periodic: None,
            message: None,
        }
    }
}

// =============================================================
// Structural tests
// =============================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// All three prototiles are ring-exact unit-edge polygons (every
    /// girih edge is a power of t: e·ē = 1 over Z[ζ₂₀]) with the girih
    /// areas BOW = 1.31433…, HEX = 2.12663…, DEC = 2.5·cot18°, and each
    /// tile is C₂-symmetric about its center (L ↦ 2·cen − L permutes
    /// the vertices — this is what makes rotations r and r+5 the same
    /// pose and lets the DEC carry no rotation at all).
    #[test]
    fn prototiles_are_exact_unit_edge_polygons() {
        for (locals, cen) in [
            (&BOW_L[..], CENB),
            (&HEX_L[..], CENH),
            (&DEC_L[..], [0i64; 8]),
        ] {
            let n = locals.len();
            for i in 0..n {
                let e = ksub(locals[(i + 1) % n], locals[i]);
                assert_eq!(
                    kmul(e, kconj(e)),
                    [1, 0, 0, 0, 0, 0, 0, 0],
                    "edge not ring-exact unit: {e:?}"
                );
            }
            for l in locals {
                let f = ksub(cen, *l);
                assert!(
                    locals.iter().any(|m| m == &f),
                    "C2 flip image {f:?} not a vertex"
                );
            }
        }
        let shoelace = |locals: &[Ke]| -> f64 {
            let pts: Vec<Vec2> = locals.iter().map(|l| kv(*l)).collect();
            let mut a = 0.0;
            for i in 0..locals.len() {
                let p = pts[i];
                let q = pts[(i + 1) % locals.len()];
                a += p.x * q.y - p.y * q.x;
            }
            a / 2.0
        };
        assert!((shoelace(&BOW_L) - AREA_BOW).abs() < 1e-12, "BOW area");
        assert!((shoelace(&HEX_L) - AREA_HEX).abs() < 1e-12, "HEX area");
        assert!((shoelace(&DEC_L) - AREA_DEC).abs() < 1e-12, "DEC area");
        // The petal/gap slot constants: GBP = AP·φ exactly.
        assert_eq!(kmul(AP, PHI), GBP, "GBP = AP·phi");
    }

    /// The substitution matrix M = [[2,3,0],[2,2,1],[5,0,6]] has
    /// Perron–Frobenius eigenvalue φ⁴ = λ² for the inflation λ = φ²,
    /// and the three area identities 2A_B + 3A_H = φ⁴·A_B,
    /// 2A_B + 2A_H + A_D = φ⁴·A_H, 5A_B + 6A_D = φ⁴·A_D hold to float
    /// exactness.
    #[test]
    fn substitution_matrix_perron_eigenvalue_is_inflation_squared() {
        let cb = |t: &[(u8, Ke, u8)]| {
            (
                t.iter().filter(|e| e.0 == TY_B).count(),
                t.iter().filter(|e| e.0 == TY_H).count(),
                t.iter().filter(|e| e.0 == TY_D).count(),
            )
        };
        assert_eq!(cb(&BROW), (2, 3, 0), "BOW child census");
        assert_eq!(cb(&HROW), (2, 2, 1), "HEX child census");
        // DEC row: center + 5 petals + 5 gap BOWs.
        assert_eq!((5, 0, 6), (GAPR.len(), 0, 1 + 5), "DEC child census");
        let m = [[2.0, 3.0, 0.0], [2.0, 2.0, 1.0], [5.0, 0.0, 6.0]];
        let mut v = [1.0; 3];
        for _ in 0..200 {
            v = [
                m[0][0] * v[0] + m[1][0] * v[1] + m[2][0] * v[2],
                m[0][1] * v[0] + m[1][1] * v[1] + m[2][1] * v[2],
                m[0][2] * v[0] + m[1][2] * v[1] + m[2][2] * v[2],
            ];
            let s = v[0] + v[1] + v[2];
            v = [v[0] / s, v[1] / s, v[2] / s];
        }
        let mv = [
            m[0][0] * v[0] + m[1][0] * v[1] + m[2][0] * v[2],
            m[0][1] * v[0] + m[1][1] * v[1] + m[2][1] * v[2],
            m[0][2] * v[0] + m[1][2] * v[1] + m[2][2] * v[2],
        ];
        let pf: f64 = mv.iter().sum();
        let phi = (1.0 + 5.0f64.sqrt()) / 2.0;
        let phi4 = phi.powi(4);
        assert!((pf - phi4).abs() < 1e-9, "PF(M) = φ⁴, got {pf}");
        assert!((PHI2_F * PHI2_F - phi4).abs() < 1e-12, "λ² = φ⁴");
        assert!((2.0 * AREA_BOW + 3.0 * AREA_HEX - phi4 * AREA_BOW).abs() < 1e-12);
        assert!((2.0 * AREA_BOW + 2.0 * AREA_HEX + AREA_DEC - phi4 * AREA_HEX).abs() < 1e-12);
        assert!((5.0 * AREA_BOW + 6.0 * AREA_DEC - phi4 * AREA_DEC).abs() < 1e-12);
    }

    /// The σ-frame walk from the seed core reproduces the decoded
    /// Bielefeld reference patch exactly: level growth 14 → 86 → 604
    /// (ratio → φ⁴), and the walk tiles with center radius < 8 around o
    /// are *precisely* the 54 reference poses of the inner disk — no
    /// missing tile and no spurious one.
    #[test]
    fn walk_reproduces_the_decoded_reference_patch() {
        let (sizes, seen) = walk(2);
        assert_eq!(sizes, vec![14, 86, 604], "walk level sizes");
        let o = o_fix();
        let inner: BTreeSet<[i64; 10]> = seen
            .iter()
            .filter(|k| (kv2([k[2], k[3], k[4], k[5], k[6], k[7], k[8], k[9]]) - o).length() < 8.0)
            .copied()
            .collect();
        let want: BTreeSet<[i64; 10]> = PATCH_REF
            .iter()
            .map(|(ty, pos, rot)| {
                [
                    *ty as i64,
                    *rot as i64,
                    pos[0],
                    pos[1],
                    pos[2],
                    pos[3],
                    pos[4],
                    pos[5],
                    pos[6],
                    pos[7],
                ]
            })
            .collect();
        assert_eq!(inner.len(), 54);
        assert_eq!(inner, want, "walk vs decoded patch inner disk");
    }

    /// The walk tiles its union edge-to-edge with no overlaps: at level
    /// 1 every edge with both endpoints inside radius 4.5 of o appears
    /// exactly twice (shared by two tiles); likewise at level 2 inside
    /// radius 8.  A dangling edge would be a gap, an edge used three
    /// times an overlap — both are caught.
    #[test]
    fn walk_is_edge_to_edge_in_the_interior() {
        let mut seen: BTreeSet<[i64; 10]> = BTreeSet::new();
        let mut frontier: Vec<Obj> = Vec::new();
        for &(ty, pos, rot) in SEED.iter() {
            let ob = Obj { ty, rot, pos };
            seen.insert(obj_key(&ob));
            frontier.push(ob);
        }
        let o = o_fix();
        let mut census: Vec<BTreeMap<(Ke, Ke), u32>> = Vec::new();
        for _ in 1..=2 {
            let mut next: Vec<Obj> = Vec::new();
            let mut children: Vec<Obj> = Vec::new();
            for ob in &frontier {
                children.clear();
                children_into(ob, &mut children);
                for c in children.drain(..) {
                    if seen.insert(obj_key(&c)) {
                        next.push(c);
                    }
                }
            }
            let mut cnt: BTreeMap<(Ke, Ke), u32> = BTreeMap::new();
            for k in &seen {
                let ob = Obj {
                    ty: k[0] as u8,
                    rot: k[1] as u8,
                    pos: [k[2], k[3], k[4], k[5], k[6], k[7], k[8], k[9]],
                };
                let vs = verts_exact(&ob);
                for i in 0..vs.len() {
                    let (a, b) = (vs[i], vs[(i + 1) % vs.len()]);
                    let key = if a <= b { (a, b) } else { (b, a) };
                    *cnt.entry(key).or_insert(0) += 1;
                }
            }
            census.push(cnt);
            frontier = next;
        }
        for (idx, rad, min_checked) in [(0, 4.5, 40), (1, 8.0, 150)] {
            let mut checked = 0;
            for ((a, b), n) in &census[idx] {
                if (kv2(*a) - o).length() < rad && (kv2(*b) - o).length() < rad {
                    assert_eq!(*n, 2, "interior edge used {n} times (level {})", idx + 1);
                    checked += 1;
                }
            }
            assert!(checked > min_checked, "census window degenerate: {checked}");
        }
    }

    /// Views beyond the reachable disk fail with a message instead of
    /// panicking or running away.
    #[test]
    fn unreachable_views_fail_gracefully() {
        let t = AndritzBowtieHexagon10gon1Tiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(
            crate::lattice::SCALE_ID,
            tess_core::params::ParamValue::Float(1.0),
        );
        let far = COVERED[MAX_LEVEL] + 50.0;
        let r = t.generate(
            &p,
            &Query {
                region: BBox::new(Vec2::new(far, far), Vec2::new(far + 10.0, far + 10.0)),
            },
        );
        assert!(r.message.is_some(), "far view must fail with a message");
        assert!(r.tiles.is_empty());
    }
}
