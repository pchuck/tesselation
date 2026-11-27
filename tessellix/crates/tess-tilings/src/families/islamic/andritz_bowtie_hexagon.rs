//! Andritz bowtie-and-hexagon substitution — a genuinely quasiperiodic
//! girih tiling from Leonhard Andritz's fivefold substitution family
//! (submitted to the Tilings Encyclopedia, February 2016), tiles inspired
//! by Islamic girih patterns (the entry notes it is *different* from the
//! "Tie and Navette" tiling of Lu & Steinhardt 1990/1994).
//!
//! # The substitution system
//!
//! Two unit-edge girih prototiles, angles in multiples of 36° — the very
//! same shapes as the girih bobbin and bow-tie of the Darb-i Imam system
//! ([`super::girih_qc`]):
//!
//! * **BOW** (`TY_R`) — the concave bow-tie hexagon (area 1.31433);
//! * **HEX** (`TY_T`) — the elongated hexagon "tie" (long diagonal φ²).
//!
//! The inflation is **λ = φ²** with substitution matrix
//!
//! ```text
//! M = [[2, 3],    (BOW → 2 BOW + 3 HEX,  HEX → 3 BOW + 5 HEX)
//!      [3, 5]]
//! ```
//!
//! whose Perron–Frobenius eigenvalue is φ⁴ = λ² (area identity exact:
//! 2A_BOW + 3A_HEX = φ⁴·A_BOW and 3A_BOW + 5A_HEX = φ⁴·A_HEX to 2e−15).
//!
//! The rule itself is a **σ-frame walk** (an overlapping-cluster /
//! straddler substitution in the modern SVT sense — a parent's inflated
//! frame σ(Y) is *not* a union of tiles; the tiles of the tiling are the
//! exact anchor claims of the frames): every tile pose (kind, a, r), with
//! the reflection lift folded into even rotations via the central symmetry
//! flip (a + U^r·d_K, r + 5), claims children at
//!
//! ```text
//! A = −φ²·a + τ,   R = (r + 5) mod 10,   child = (kind_j, A + U^R·v_j, (R + d_j) mod 10)
//! ```
//!
//! with U = e^{−36°i} and τ = (11, −16, −45, −35) in Z[ζ₅].  BOW parents
//! claim through **both** lifts with their full 5-slot table (2 BOW + 3
//! HEX); HEX parents claim with their full 8-slot table (3 BOW + 5 HEX)
//! through the even lift and with slots {1, 3, 4, 5} through the odd lift.
//! Every claim lands on an exact tile of the tiling, and every tile of the
//! tiling is claimed (verified against the decoded Bielefeld reference
//! patch: all 646 tiles reproduced, 253-pose inner disk matched with zero
//! discrepancy in both directions).
//!
//! # Exactness
//!
//! All positions live in the cyclotomic ring Z[ζ₅], ζ = e^{2πi/5}, stored
//! as integer 4-tuples over (1, ζ, ζ², ζ³) — no denominators anywhere:
//! the frame constants, both slot tables, the prototile vertices and the
//! seed anchors are ring elements, and every rotation is a power of
//! U = −ζ².  Floats appear only when a ring point is finally turned into
//! a `Vec2` for emission.  The tables were derived in a Python prototype
//! in exact ring arithmetic (σ fitted to the decoded reference patch, slot
//! offsets recovered by meet-in-the-middle, parity of every slot audited
//! over all interior parent poses), and the Rust structural tests
//! re-verify them end to end: ring-exact unit edges and areas, the exact
//! 253-pose inner patch, level growth 27 → 168 → 1160 tiles (ratio → φ⁴),
//! and an exact edge census (every interior edge shared by exactly two
//! tiles, none dangling) out to radius 8 at level 1 and radius 20 at
//! level 2.
//!
//! # Generation for a view
//!
//! A wavefront BFS from the 27-tile seed core around the fixed point
//! o = τ/(1 + φ²) grows the tiling outward; substitution level *k* covers
//! a disk of radius ≈ 3.25, 9.0, 23.5, 61, … (growing ×φ² per level, the
//! conservative table below).  The sweep is region-filtered through the
//! σ⁻¹ preimage box chain of the view (claims land within SLOT_REACH of
//! σ(parent), so only anchors in the preimage boxes can descend into the
//! view), keeping the work linear in the view area even far from o.
//! `generate` BFS-iterates until the emitted patch provably covers the
//! view (clipped-area equality, as for [`super::girih_qc`]); tile emission
//! is unit-edge at every level.  Views whose far corner lies beyond the
//! level-5 covered radius (≈ 406) or whose area would exceed [`MAX_TILES`]
//! fail with a zoom-in message.

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
// Exact arithmetic in R = Z[ζ₅], ζ = e^{2πi/5} (basis 1, ζ, ζ², ζ³;
// ζ⁴ = −ζ³−ζ²−ζ−1, ζ⁵ = 1).  Integer 4-tuples, no denominators.
// =============================================================

/// Basis values of ζ, ζ², ζ³ (float evaluation at the emission boundary).
const Z1: (f64, f64) = (0.30901699437494745, 0.9510565162951535);
const Z2: (f64, f64) = (-0.8090169943749473, 0.5877852522924731);
const Z3: (f64, f64) = (-0.8090169943749475, -0.587785252292473);

/// Prototile codes.
const TY_R: u8 = 1; // BOW: concave bow-tie hexagon
const TY_T: u8 = 2; // HEX: elongated hexagon "tie" (long diagonal φ²)

type Ke = [i64; 4];

#[inline]
fn kadd(x: Ke, y: Ke) -> Ke {
    [x[0] + y[0], x[1] + y[1], x[2] + y[2], x[3] + y[3]]
}

/// Ring product: reduce x·y modulo ζ⁴ + ζ³ + ζ² + ζ + 1.
#[inline]
const fn kmul(x: Ke, y: Ke) -> Ke {
    let mut p = [0i64; 7];
    let mut i = 0;
    while i < 4 {
        let mut j = 0;
        while j < 4 {
            p[i + j] += x[i] * y[j];
            j += 1;
        }
        i += 1;
    }
    // ζ⁴ = −(ζ³+ζ²+ζ+1), ζ⁵ = 1, ζ⁶ = ζ
    [
        p[0] - p[4] + p[5],
        p[1] - p[4] + p[6],
        p[2] - p[4],
        p[3] - p[4],
    ]
}

/// Float value of a ring element (emission boundary only).
#[inline]
fn kv(k: Ke) -> Vec2 {
    Vec2::new(
        k[0] as f64 + k[1] as f64 * Z1.0 + k[2] as f64 * Z2.0 + k[3] as f64 * Z3.0,
        k[1] as f64 * Z1.1 + k[2] as f64 * Z2.1 + k[3] as f64 * Z3.1,
    )
}

/// U = e^{−36°i} = −ζ², and its ten powers (all ring elements).
const U: Ke = [0, 0, -1, 0];
const UP: [Ke; 10] = {
    let mut t = [[0i64; 4]; 10];
    t[0] = [1, 0, 0, 0];
    let mut i = 1;
    while i < 10 {
        t[i] = kmul(t[i - 1], U);
        i += 1;
    }
    t
};

/// μ = −φ² (the frame contraction) and τ, the frame translation —
/// exactly the fitted σ(z) = −φ²z + τ (verified to 0.0 float deviation
/// against the fit on the decoded reference patch).
const MU: Ke = [-1, 0, 1, 1];
const TAU: Ke = [11, -16, -45, -35];

/// Central-symmetry flip diagonals (tiles are C₂-symmetric; the odd
/// rotation lift is the same tile anchored at the antipodal vertex).
const DK_R: Ke = [0, -2, -1, -1]; // 1 − 2i·sin72° (BOW diagonal)
const DK_T: Ke = [1, 0, -1, -1]; // φ² (HEX long diagonal)

#[inline]
fn dk_of(ty: u8) -> Ke {
    if ty == TY_R {
        DK_R
    } else {
        DK_T
    }
}

// =============================================================
// The substitution tables (derived in the exact Python prototype;
// every offset recovered as a ring element and audited over all
// parent poses within radius 4.5 of the fixed point — 100 % of the
// in-patch claims land on exact reference tiles, both lifts).
//
// Entry: (child kind, slot offset v in frame coordinates, child
// rotation d_j mod 10 relative to the frame rotation).
// =============================================================

/// BOW parent: 2 BOW + 3 HEX.
const BTAB5: [(u8, Ke, u8); 5] = [
    (TY_R, [0, 0, -1, 0], 0),
    (TY_R, [-1, -2, -2, -1], 0),
    (TY_T, [0, 0, 0, 0], 2),
    (TY_T, [-2, -4, -3, -1], 0),
    (TY_T, [-1, -4, -4, -2], 7), // d = −3
];

/// HEX parent: 3 BOW + 5 HEX.
const CORE8: [(u8, Ke, u8); 8] = [
    (TY_R, [1, 0, 0, 0], 9), // d = −1
    (TY_R, [1, 1, -1, -1], 0),
    (TY_R, [1, 0, -2, -3], 1),
    (TY_T, [0, 0, 0, 0], 1),
    (TY_T, [2, 0, -3, -3], 6), // d = −4
    (TY_T, [0, -1, -2, -1], 0),
    (TY_T, [1, -1, -2, -2], 8), // d = −2
    (TY_T, [1, 1, 0, -1], 2),
];

/// HEX parents claim through the odd lift only through these slots
/// (slots 1, 3, 4, 5 — the audit shows the other odd-lift slots claim
/// positions that are not tiles; the even lift carries the full table).
const SLOTS_T_ODD: [usize; 4] = [1, 3, 4, 5];

/// Prototile local vertices (ring-exact, anticlockwise, anchor at the
/// designated lift vertex).  BOW area = 1.31433 (rectangle 1 × 2cos18°
/// minus the two waist notches), HEX area = 2.12663 (φ² long diagonal).
const BOW_L: [Ke; 6] = [
    [-1, -2, -1, -1],
    [0, -2, -1, -1],
    [1, -1, 0, 0],
    [1, 0, 0, 0],
    [0, 0, 0, 0],
    [-1, -1, -1, -1],
];
const HEX_L: [Ke; 6] = [
    [0, 0, -1, 0],
    [1, 0, -1, 0],
    [1, 0, -1, -1],
    [1, 0, 0, -1],
    [0, 0, 0, -1],
    [0, 0, 0, 0],
];

// ===== seed core =====

/// The seed lineage: the 27 reference-patch tiles whose centroids lie
/// within radius 4 of the fixed point o = τ/(1 + φ²), in canonical
/// (even-rotation) lifts with ring-exact anchors.
const SEED: [(u8, Ke, u8); 27] = [
    (TY_R, [1, -6, -12, -9], 8),
    (TY_R, [2, -5, -13, -10], 0),
    (TY_R, [2, -3, -11, -9], 4),
    (TY_R, [3, -5, -13, -11], 8),
    (TY_R, [3, -4, -11, -9], 2),
    (TY_R, [3, -3, -12, -9], 0),
    (TY_R, [4, -5, -14, -11], 2),
    (TY_R, [4, -1, -10, -9], 0),
    (TY_R, [5, -3, -13, -10], 6),
    (TY_T, [1, -7, -14, -10], 0),
    (TY_T, [1, -7, -14, -10], 6),
    (TY_T, [1, -7, -14, -10], 8),
    (TY_T, [1, -6, -12, -8], 8),
    (TY_T, [1, -5, -12, -9], 0),
    (TY_T, [3, -6, -15, -12], 4),
    (TY_T, [3, -5, -13, -10], 0),
    (TY_T, [3, -5, -13, -10], 2),
    (TY_T, [3, -5, -13, -10], 8),
    (TY_T, [3, -3, -11, -9], 0),
    (TY_T, [3, -3, -11, -9], 2),
    (TY_T, [3, -3, -11, -9], 4),
    (TY_T, [3, -3, -11, -9], 6),
    (TY_T, [3, -3, -11, -9], 8),
    (TY_T, [4, -4, -14, -11], 2),
    (TY_T, [4, -4, -14, -11], 6),
    (TY_T, [5, -2, -12, -11], 4),
    (TY_T, [5, -1, -10, -9], 2),
];

/// The decoded Bielefeld reference patch: all canonical tile poses with
/// anchor radius < 12 around o (253 tiles), reproduced from the exact
/// decoding of the encyclopedia patch figure.  The walk test asserts the
/// level-2 walk reproduces this set *exactly*, in both directions.
#[cfg(test)]
const PATCH_REF: [(u8, Ke, u8); 253] = [
    (TY_R, [-2, -13, -20, -11], 0),
    (TY_T, [-2, -13, -19, -11], 0),
    (TY_T, [-2, -7, -10, -6], 0),
    (TY_R, [-1, -11, -18, -11], 0),
    (TY_T, [-1, -10, -15, -9], 0),
    (TY_R, [-1, -8, -14, -8], 0),
    (TY_T, [-1, -8, -13, -8], 0),
    (TY_T, [0, -10, -18, -12], 0),
    (TY_R, [0, -9, -17, -10], 0),
    (TY_T, [0, -9, -16, -10], 0),
    (TY_R, [0, -5, -10, -6], 0),
    (TY_T, [0, -5, -9, -6], 0),
    (TY_R, [1, -11, -22, -14], 0),
    (TY_T, [1, -11, -21, -14], 0),
    (TY_R, [1, -7, -15, -10], 0),
    (TY_T, [1, -7, -14, -10], 0),
    (TY_T, [1, -5, -12, -9], 0),
    (TY_T, [1, -4, -10, -7], 0),
    (TY_R, [2, -8, -18, -12], 0),
    (TY_T, [2, -8, -17, -12], 0),
    (TY_R, [2, -5, -13, -10], 0),
    (TY_R, [2, -2, -9, -7], 0),
    (TY_T, [2, -2, -8, -7], 0),
    (TY_T, [2, -1, -6, -5], 0),
    (TY_T, [3, -5, -13, -10], 0),
    (TY_R, [3, -3, -12, -9], 0),
    (TY_T, [3, -3, -11, -9], 0),
    (TY_T, [4, -7, -18, -14], 0),
    (TY_R, [4, -5, -17, -13], 0),
    (TY_T, [4, -5, -16, -13], 0),
    (TY_T, [4, -4, -14, -11], 0),
    (TY_R, [4, -1, -10, -9], 0),
    (TY_T, [4, 0, -7, -7], 0),
    (TY_R, [4, 2, -6, -6], 0),
    (TY_T, [4, 2, -5, -6], 0),
    (TY_T, [5, -6, -19, -15], 0),
    (TY_R, [5, -2, -13, -11], 0),
    (TY_T, [5, -2, -12, -11], 0),
    (TY_T, [5, 0, -10, -10], 0),
    (TY_R, [5, 1, -9, -8], 0),
    (TY_T, [5, 1, -8, -8], 0),
    (TY_R, [6, -4, -18, -15], 0),
    (TY_T, [6, -4, -17, -15], 0),
    (TY_T, [6, -2, -15, -14], 0),
    (TY_T, [6, -1, -13, -12], 0),
    (TY_R, [6, 3, -7, -8], 0),
    (TY_T, [6, 3, -6, -8], 0),
    (TY_R, [7, 2, -10, -10], 0),
    (TY_T, [7, 2, -9, -10], 0),
    (TY_T, [-2, -13, -19, -11], 2),
    (TY_T, [-2, -12, -17, -9], 2),
    (TY_T, [-1, -10, -15, -9], 2),
    (TY_T, [-1, -8, -13, -8], 2),
    (TY_T, [0, -11, -18, -11], 2),
    (TY_R, [0, -10, -16, -10], 2),
    (TY_T, [0, -9, -16, -10], 2),
    (TY_R, [0, -6, -9, -6], 2),
    (TY_T, [0, -5, -9, -6], 2),
    (TY_T, [0, -4, -7, -4], 2),
    (TY_T, [1, -11, -21, -14], 2),
    (TY_R, [1, -7, -12, -8], 2),
    (TY_T, [1, -6, -12, -8], 2),
    (TY_R, [2, -9, -17, -12], 2),
    (TY_T, [2, -8, -17, -12], 2),
    (TY_T, [2, -7, -15, -10], 2),
    (TY_R, [2, -2, -6, -5], 2),
    (TY_T, [2, -1, -6, -5], 2),
    (TY_R, [3, -10, -20, -14], 2),
    (TY_T, [3, -9, -20, -14], 2),
    (TY_T, [3, -5, -13, -10], 2),
    (TY_R, [3, -4, -11, -9], 2),
    (TY_T, [3, -3, -11, -9], 2),
    (TY_T, [3, -2, -9, -7], 2),
    (TY_R, [3, -1, -7, -6], 2),
    (TY_T, [4, -7, -18, -14], 2),
    (TY_T, [4, -5, -16, -13], 2),
    (TY_R, [4, -5, -14, -11], 2),
    (TY_T, [4, -4, -14, -11], 2),
    (TY_R, [4, -1, -7, -7], 2),
    (TY_T, [4, 0, -7, -7], 2),
    (TY_T, [4, 2, -5, -6], 2),
    (TY_R, [5, -7, -19, -15], 2),
    (TY_T, [5, -6, -19, -15], 2),
    (TY_R, [5, -4, -15, -12], 2),
    (TY_T, [5, -1, -10, -9], 2),
    (TY_R, [5, 0, -8, -8], 2),
    (TY_T, [5, 1, -8, -8], 2),
    (TY_R, [6, -4, -15, -13], 2),
    (TY_T, [6, -3, -15, -13], 2),
    (TY_T, [6, -2, -13, -11], 2),
    (TY_R, [6, 3, -4, -6], 2),
    (TY_T, [6, 4, -4, -6], 2),
    (TY_R, [7, 1, -9, -10], 2),
    (TY_T, [7, 2, -9, -10], 2),
    (TY_T, [7, 3, -7, -8], 2),
    (TY_R, [8, -1, -14, -14], 2),
    (TY_R, [8, 0, -12, -12], 2),
    (TY_T, [8, 1, -12, -12], 2),
    (TY_R, [-3, -13, -19, -11], 4),
    (TY_R, [-3, -9, -12, -7], 4),
    (TY_T, [-2, -13, -19, -11], 4),
    (TY_R, [-2, -12, -17, -10], 4),
    (TY_T, [-2, -12, -17, -9], 4),
    (TY_R, [-2, -10, -15, -9], 4),
    (TY_T, [-2, -9, -12, -7], 4),
    (TY_R, [-1, -12, -20, -13], 4),
    (TY_T, [-1, -11, -17, -11], 4),
    (TY_T, [-1, -10, -15, -9], 4),
    (TY_T, [-1, -8, -13, -8], 4),
    (TY_R, [-1, -5, -9, -6], 4),
    (TY_T, [0, -12, -20, -13], 4),
    (TY_R, [0, -7, -14, -10], 4),
    (TY_T, [0, -5, -9, -6], 4),
    (TY_T, [1, -11, -21, -14], 4),
    (TY_R, [1, -8, -17, -12], 4),
    (TY_T, [1, -7, -14, -10], 4),
    (TY_T, [1, -6, -12, -8], 4),
    (TY_T, [1, -3, -7, -6], 4),
    (TY_T, [2, -8, -17, -12], 4),
    (TY_R, [2, -3, -11, -9], 4),
    (TY_T, [2, -2, -8, -7], 4),
    (TY_R, [3, -9, -20, -15], 4),
    (TY_T, [3, -9, -20, -14], 4),
    (TY_R, [3, -7, -18, -14], 4),
    (TY_T, [3, -6, -15, -12], 4),
    (TY_T, [3, -3, -11, -9], 4),
    (TY_R, [3, 0, -7, -7], 4),
    (TY_T, [3, 1, -4, -5], 4),
    (TY_T, [4, -8, -20, -16], 4),
    (TY_T, [4, -7, -18, -14], 4),
    (TY_T, [4, -5, -16, -13], 4),
    (TY_R, [4, -2, -12, -11], 4),
    (TY_T, [4, -1, -9, -9], 4),
    (TY_T, [4, 0, -7, -7], 4),
    (TY_T, [4, 2, -5, -6], 4),
    (TY_R, [5, -4, -17, -15], 4),
    (TY_R, [5, -3, -15, -13], 4),
    (TY_T, [5, -2, -12, -11], 4),
    (TY_R, [5, 3, -6, -8], 4),
    (TY_T, [6, -4, -17, -15], 4),
    (TY_T, [6, -3, -15, -13], 4),
    (TY_R, [6, 2, -9, -10], 4),
    (TY_T, [6, 3, -6, -8], 4),
    (TY_R, [6, 4, -4, -7], 4),
    (TY_T, [6, 4, -4, -6], 4),
    (TY_R, [7, 0, -14, -14], 4),
    (TY_T, [7, 2, -9, -10], 4),
    (TY_R, [7, 4, -7, -10], 4),
    (TY_R, [8, 1, -12, -13], 4),
    (TY_T, [8, 1, -12, -12], 4),
    (TY_T, [8, 4, -7, -10], 4),
    (TY_T, [-2, -13, -19, -11], 6),
    (TY_T, [-2, -12, -17, -9], 6),
    (TY_T, [-2, -10, -15, -8], 6),
    (TY_R, [-2, -9, -13, -7], 6),
    (TY_T, [-2, -9, -12, -7], 6),
    (TY_R, [-1, -12, -18, -10], 6),
    (TY_T, [-1, -8, -13, -8], 6),
    (TY_R, [0, -12, -21, -13], 6),
    (TY_T, [0, -12, -20, -13], 6),
    (TY_T, [0, -9, -16, -10], 6),
    (TY_R, [0, -7, -12, -7], 6),
    (TY_T, [0, -5, -9, -6], 6),
    (TY_T, [1, -11, -21, -14], 6),
    (TY_R, [1, -8, -15, -9], 6),
    (TY_T, [1, -7, -14, -10], 6),
    (TY_T, [1, -6, -12, -8], 6),
    (TY_T, [1, -4, -10, -7], 6),
    (TY_R, [1, -4, -8, -5], 6),
    (TY_R, [2, -10, -20, -13], 6),
    (TY_T, [2, -8, -17, -12], 6),
    (TY_R, [2, -3, -9, -6], 6),
    (TY_T, [2, -1, -6, -5], 6),
    (TY_T, [3, -9, -20, -14], 6),
    (TY_T, [3, -7, -18, -13], 6),
    (TY_R, [3, -7, -16, -11], 6),
    (TY_T, [3, -3, -11, -9], 6),
    (TY_T, [3, 0, -7, -6], 6),
    (TY_R, [3, 0, -5, -4], 6),
    (TY_R, [3, 1, -5, -5], 6),
    (TY_T, [3, 1, -4, -5], 6),
    (TY_T, [4, -4, -14, -11], 6),
    (TY_R, [4, -2, -10, -8], 6),
    (TY_T, [4, 2, -5, -6], 6),
    (TY_T, [5, -6, -19, -15], 6),
    (TY_R, [5, -3, -13, -10], 6),
    (TY_T, [5, -2, -12, -11], 6),
    (TY_T, [5, 1, -8, -8], 6),
    (TY_R, [5, 3, -4, -5], 6),
    (TY_R, [6, -5, -18, -14], 6),
    (TY_T, [6, -4, -17, -15], 6),
    (TY_T, [6, -3, -15, -13], 6),
    (TY_T, [6, -1, -13, -12], 6),
    (TY_R, [6, 2, -7, -7], 6),
    (TY_T, [6, 3, -6, -8], 6),
    (TY_T, [6, 4, -4, -6], 6),
    (TY_R, [7, 0, -12, -11], 6),
    (TY_T, [7, 2, -9, -10], 6),
    (TY_R, [7, 4, -5, -7], 6),
    (TY_T, [8, 1, -12, -12], 6),
    (TY_T, [8, 3, -10, -11], 6),
    (TY_R, [8, 3, -8, -9], 6),
    (TY_R, [8, 4, -8, -10], 6),
    (TY_T, [8, 4, -7, -10], 6),
    (TY_R, [-3, -11, -14, -8], 8),
    (TY_T, [-2, -13, -19, -11], 8),
    (TY_T, [-2, -9, -12, -7], 8),
    (TY_R, [-2, -8, -10, -6], 8),
    (TY_R, [-1, -10, -15, -10], 8),
    (TY_T, [-1, -10, -15, -9], 8),
    (TY_T, [0, -12, -20, -13], 8),
    (TY_R, [0, -11, -18, -12], 8),
    (TY_T, [0, -11, -18, -11], 8),
    (TY_T, [0, -9, -16, -10], 8),
    (TY_T, [0, -7, -11, -7], 8),
    (TY_R, [0, -5, -9, -7], 8),
    (TY_T, [0, -5, -9, -6], 8),
    (TY_T, [1, -7, -14, -10], 8),
    (TY_R, [1, -6, -12, -9], 8),
    (TY_T, [1, -6, -12, -8], 8),
    (TY_R, [1, -5, -10, -7], 8),
    (TY_T, [1, -4, -10, -7], 8),
    (TY_R, [1, -2, -5, -5], 8),
    (TY_T, [2, -10, -19, -13], 8),
    (TY_R, [2, -8, -17, -13], 8),
    (TY_T, [2, -8, -17, -12], 8),
    (TY_R, [2, -1, -6, -6], 8),
    (TY_T, [2, -1, -6, -5], 8),
    (TY_R, [3, -5, -13, -11], 8),
    (TY_T, [3, -5, -13, -10], 8),
    (TY_T, [3, -3, -11, -9], 8),
    (TY_T, [3, 1, -4, -5], 8),
    (TY_R, [4, -7, -18, -15], 8),
    (TY_T, [4, -7, -18, -14], 8),
    (TY_T, [4, -4, -14, -11], 8),
    (TY_R, [4, 0, -7, -8], 8),
    (TY_T, [4, 0, -7, -7], 8),
    (TY_T, [5, -6, -19, -15], 8),
    (TY_R, [5, -2, -12, -12], 8),
    (TY_T, [5, -2, -12, -11], 8),
    (TY_R, [5, -1, -10, -10], 8),
    (TY_T, [5, -1, -10, -9], 8),
    (TY_T, [5, 1, -8, -8], 8),
    (TY_R, [6, -4, -17, -16], 8),
    (TY_T, [6, -4, -17, -15], 8),
    (TY_R, [6, -3, -15, -14], 8),
    (TY_T, [6, -3, -15, -13], 8),
    (TY_R, [6, -2, -13, -12], 8),
    (TY_T, [6, 3, -6, -8], 8),
    (TY_T, [7, 0, -11, -11], 8),
    (TY_R, [7, 2, -9, -11], 8),
    (TY_T, [7, 2, -9, -10], 8),
    (TY_T, [8, 4, -7, -10], 8),
];

// ===== generation bookkeeping =====

/// φ² (float; the covered-radius growth per level).
const PHI2_F: f64 = 2.618033988749895;

/// Fixed point of σ: o = τ/(1 + φ²) (float; used only for window tests).
fn o_fix() -> Vec2 {
    let t = kv(TAU);
    Vec2::new(t.x / (1.0 + PHI2_F), t.y / (1.0 + PHI2_F))
}

/// Tile areas at the prototile vertices (float, emission-boundary tests
/// only; the exact identities are checked in the tests).
const AREA_BOW: f64 = 1.3143277802978344;
const AREA_HEX: f64 = 2.126627020880099;

/// Mean tile area at the substitution frequencies (T/R → φ): the
/// tile-count guard's area-per-tile estimate,
/// (A_BOW + φ·A_HEX)/(1 + φ).
const MEAN_AREA: f64 = (AREA_BOW + 1.618033988749895 * AREA_HEX) / 2.618033988749895;

/// Guaranteed-covered disk radius around o after each BFS level
/// (measured 3.25 / 9.0 / 23.5 / 61.0 for levels 0–3, growth ×φ²; the
/// table is rounded down, the area acceptance escalates a level if it
/// was optimistic).
const COVERED: [f64; MAX_LEVEL + 1] = [3.0, 8.5, 23.0, 60.0, 156.0, 406.0];

/// Maximum BFS level (covered radius ≈ 406).
const MAX_LEVEL: usize = 5;

/// Sanity cap on total BFS claims processed (the region-filtered sweep
/// is linear in the view area; this only catches degenerate pathology).
const WORK_CAP: usize = 20_000_000;

/// A tile's anchor is a vertex and its diameter is φ², so a tile
/// reaching the view has its anchor within TILE_REACH of the region.
const TILE_REACH: f64 = 2.7;

/// Largest slot offset |v| in the claim formula (a claim lands at
/// σ(parent) + U^R·v, |v| ≤ SLOT_REACH ≈ 5.38).
const SLOT_REACH: f64 = 5.4;

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

/// One tile of the walk: prototile, canonical (even) rotation in 36°
/// units, and the exact ring anchor.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Obj {
    ty: u8,
    rot: u8,
    pos: Ke,
}

#[inline]
fn obj_key(o: &Obj) -> [i64; 6] {
    [
        o.ty as i64,
        o.rot as i64,
        o.pos[0],
        o.pos[1],
        o.pos[2],
        o.pos[3],
    ]
}

/// Canonicalize a raw child pose: odd rotations fold onto the even lift
/// through the central symmetry (a + U^r·d_K, r + 5).
#[inline]
fn canon(ty: u8, pos: Ke, rot: u8) -> Obj {
    if rot % 2 == 0 {
        Obj { ty, pos, rot }
    } else {
        Obj {
            ty,
            pos: kadd(pos, kmul(UP[rot as usize], dk_of(ty))),
            rot: (rot + 5) % 10,
        }
    }
}

/// The frame claims of one canonical pose: the even-lift frame (full
/// table) and the odd-lift frame (full table for BOW, slots {1,3,4,5}
/// for HEX), appended to `out` in canonical form.
fn frames_into(o: &Obj, out: &mut Vec<Obj>) {
    let tab: &[(u8, Ke, u8)] = if o.ty == TY_R { &BTAB5 } else { &CORE8 };
    // Even lift: A = μ·a + τ, frame rotation R = r + 5, full table.
    let base = kadd(kmul(MU, o.pos), TAU);
    let re = (o.rot + 5) % 10;
    for &(tyj, v, dj) in tab {
        let pos = kadd(base, kmul(UP[re as usize], v));
        out.push(canon(tyj, pos, (re + dj) % 10));
    }
    // Odd lift: anchor a + U^r·d_K, frame rotation R = r.  BOW parents
    // claim through the full table, HEX parents only through slots
    // {1, 3, 4, 5} (the other odd-lift slots claim non-tile positions).
    let ao = kadd(o.pos, kmul(UP[o.rot as usize], dk_of(o.ty)));
    let base_o = kadd(kmul(MU, ao), TAU);
    let ro = o.rot as usize;
    let odd_slots: &[usize] = if o.ty == TY_R {
        &[0, 1, 2, 3, 4]
    } else {
        &SLOTS_T_ODD
    };
    for &j in odd_slots {
        let (tyj, v, dj) = tab[j];
        let pos = kadd(base_o, kmul(UP[ro], v));
        out.push(canon(tyj, pos, (o.rot + dj) % 10));
    }
}

/// Float polygon of a tile: anchor plus U^rot applied to the ring-exact
/// local vertices (exact rotation, float only at the end).
fn poly_pts(o: &Obj) -> Vec<Vec2> {
    let locals: &[Ke] = if o.ty == TY_R { &BOW_L } else { &HEX_L };
    let r = o.rot as usize;
    locals
        .iter()
        .map(|l| kv(kadd(o.pos, kmul(UP[r], *l))))
        .collect()
}

/// Output prototile index: BOW → 0, HEX → 1.
fn proto_of(ty: u8) -> u8 {
    if ty == TY_R {
        0
    } else {
        1
    }
}

/// BFS the walk from the seed core to `max_level` (test helper).
#[cfg(test)]
fn walk(max_level: usize) -> (Vec<usize>, BTreeSet<[i64; 6]>) {
    let mut seen: BTreeSet<[i64; 6]> = BTreeSet::new();
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
            frames_into(o, &mut children);
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

/// Andritz bowtie-and-hexagon (quasiperiodic; Andritz 2016 fivefold
/// substitution).
pub struct AndritzBowtieHexagonTiling;

impl Tiling for AndritzBowtieHexagonTiling {
    fn id(&self) -> &'static str {
        "andritz-bowtie-hexagon"
    }

    fn name(&self) -> &'static str {
        "Andritz Bowtie-and-Hexagon (quasiperiodic)"
    }

    fn description(&self) -> &'static str {
        "Quasiperiodic girih tiling from Andritz's 2016 fivefold substitution: bow-tie and elongated-hexagon tiles, inflation φ² with matrix [[2,3],[3,5]] (PF = φ⁴); an exact Z[ζ₅] σ-frame walk grown from a 27-tile seed core, edge-to-edge at every level."
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
            "quasiperiodic; Andritz 2016 fivefold substitution; inflation phi^2; sigma-frame walk; edge-to-edge",
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
        // Tiles are unit-edge with mean area MEAN_AREA at the substitution
        // frequencies, so this bounds the tile count before any work.
        if region.area() > MAX_TILES as f64 * MEAN_AREA {
            return fail("too many tiles for this view — zoom in");
        }
        if rcorr > COVERED[MAX_LEVEL] {
            return fail("too many tiles for this view — zoom in");
        }

        let want = region.area();

        // Region-filtered walk.  A tile reaching the view has its anchor
        // within TILE_REACH of the region (the anchor is a vertex and the
        // tile's diameter is φ²), and a claim lands at σ(parent) + U^R·v
        // with |v| ≤ SLOT_REACH, so the anchors of tiles whose j-step
        // descendant chain first touches the book box B₀ live in the
        // preimage box chain B_j: centers march toward o under σ⁻¹
        // (contraction −φ², half-turn) and half extents shrink as
        // (h + SLOT_REACH)/φ².  Keeping at BFS level L exactly the tiles
        // of B₀ ∪ … ∪ B_{MAX_LEVEL−L} is a conservative necessary
        // condition for reaching the view, so the sweep stays linear in
        // the view area even for views far from the fixed point.
        let mut boxes = Vec::with_capacity(MAX_LEVEL + 1);
        boxes.push(BBox::new(
            Vec2::new(region.min.x - TILE_REACH, region.min.y - TILE_REACH),
            Vec2::new(region.max.x + TILE_REACH, region.max.y + TILE_REACH),
        ));
        for _ in 0..MAX_LEVEL {
            let b = boxes[boxes.len() - 1];
            let c = Vec2::new(
                o.x - (0.5 * (b.min.x + b.max.x) - o.x) / PHI2_F,
                o.y - (0.5 * (b.min.y + b.max.y) - o.y) / PHI2_F,
            );
            let h = (0.5 * (b.max.x - b.min.x).max(b.max.y - b.min.y) + SLOT_REACH) / PHI2_F;
            boxes.push(BBox::new(
                Vec2::new(c.x - h, c.y - h),
                Vec2::new(c.x + h, c.y + h),
            ));
        }
        let in_chain = |p: Vec2, maxj: usize| -> bool {
            boxes[..maxj.min(MAX_LEVEL) + 1]
                .iter()
                .any(|b| b.contains_point(p))
        };

        // BFS the walk outward from the seed core.  Book tiles (box B₀)
        // drive the acceptance and the emission; frontier tiles are the
        // potential ancestors of book tiles at deeper levels.  Tiles are
        // non-overlapping, so the sum of clipped areas reaches the region
        // area exactly when the view is fully covered.
        let mut seen: HashSet<[i64; 6]> = HashSet::new();
        let mut book: Vec<Obj> = Vec::new();
        let mut frontier: Vec<Obj> = Vec::new();
        let mut acc = 0.0;
        for &(ty, pos, rot) in SEED.iter() {
            let ob = Obj { ty, rot, pos };
            seen.insert(obj_key(&ob));
            frontier.push(ob);
            if in_chain(kv(pos), 0) {
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
                frames_into(ob, &mut children);
                work += children.len();
                for c in children.drain(..) {
                    if !seen.insert(obj_key(&c)) {
                        continue;
                    }
                    let p = kv(c.pos);
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

    /// Norm N(x) = x·x̄ ∈ Z of a ring element (x̄ = conjugate, ζ ↦ ζ⁴).
    fn conj(x: Ke) -> Ke {
        // ζ̄ = ζ⁴ = −1−ζ−ζ²−ζ³; expand x̄ = Σ x_i ζ^{4i} reduced.
        let p4: Ke = [-1, -1, -1, -1];
        let mut pw = [[0i64; 4]; 4];
        let mut cur = [1, 0, 0, 0];
        for i in 0..4 {
            pw[i] = cur;
            cur = kmul(cur, p4);
        }
        let mut r = [0i64; 4];
        for i in 0..4 {
            for j in 0..4 {
                r[j] += x[i] * pw[i][j];
            }
        }
        r
    }

    fn norm(x: Ke) -> i64 {
        let p = kmul(x, conj(x));
        assert_eq!(
            (p[1], p[2], p[3]),
            (0, 0, 0),
            "norm left the rational axis: {x:?}"
        );
        p[0]
    }

    /// Both prototiles are ring-exact unit-edge hexagons with the girih
    /// areas (BOW = 1.31433…, HEX = 2.12663… — the same shapes as the
    /// Darb-i Imam bobbin and bow-tie).
    #[test]
    fn prototiles_are_exact_unit_edge_polygons() {
        for locals in [&BOW_L, &HEX_L] {
            for i in 0..6 {
                let e = kadd(
                    locals[(i + 1) % 6],
                    [-locals[i][0], -locals[i][1], -locals[i][2], -locals[i][3]],
                );
                assert_eq!(norm(e), 1, "edge not ring-exact unit: {e:?}");
            }
        }
        let shoelace = |locals: &[Ke]| -> f64 {
            let pts: Vec<Vec2> = locals.iter().map(|l| kv(*l)).collect();
            let mut a = 0.0;
            for i in 0..6 {
                let p = pts[i];
                let q = pts[(i + 1) % 6];
                a += p.x * q.y - p.y * q.x;
            }
            a / 2.0
        };
        assert!((shoelace(&BOW_L) - AREA_BOW).abs() < 1e-12, "BOW area");
        assert!((shoelace(&HEX_L) - AREA_HEX).abs() < 1e-12, "HEX area");
        // C2 symmetry: each prototile equals its own vertex set under
        // L ↦ d_K − L (the flip is a symmetry of the tile).
        let flip_match = |locals: &[Ke], d: Ke| {
            for l in locals {
                let f = kadd(d, [-l[0], -l[1], -l[2], -l[3]]);
                assert!(
                    locals.iter().any(|m| m == &f),
                    "flip image {f:?} not a vertex (d = {d:?})"
                );
            }
        };
        flip_match(&BOW_L, DK_R);
        flip_match(&HEX_L, DK_T);
    }

    /// The substitution matrix M = [[2,3],[3,5]] has Perron–Frobenius
    /// eigenvalue φ⁴ = λ² for the inflation λ = φ², and the area
    /// identities 2A_B + 3A_H = φ⁴·A_B, 3A_B + 5A_H = φ⁴·A_H hold to
    /// float exactness.
    #[test]
    fn substitution_matrix_perron_eigenvalue_is_inflation_squared() {
        let cb = |t: &[(u8, Ke, u8)]| {
            (
                t.iter().filter(|e| e.0 == TY_R).count(),
                t.iter().filter(|e| e.0 == TY_T).count(),
            )
        };
        assert_eq!(cb(&BTAB5), (2, 3), "BOW child census 2 BOW + 3 HEX");
        assert_eq!(cb(&CORE8), (3, 5), "HEX child census 3 BOW + 5 HEX");
        let phi = (1.0 + 5.0f64.sqrt()) / 2.0;
        let phi4 = phi.powi(4);
        let pf = (7.0 + 3.0 * 5.0f64.sqrt()) / 2.0; // eigenvalue of [[2,3],[3,5]]
        assert!((pf - phi4).abs() < 1e-12, "PF(M) = φ⁴");
        assert!((PHI2_F * PHI2_F - phi4).abs() < 1e-12, "λ² = φ⁴");
        assert!((2.0 * AREA_BOW + 3.0 * AREA_HEX - phi4 * AREA_BOW).abs() < 1e-12);
        assert!((3.0 * AREA_BOW + 5.0 * AREA_HEX - phi4 * AREA_HEX).abs() < 1e-12);
    }

    /// The σ-frame walk from the seed core reproduces the decoded
    /// Bielefeld reference patch exactly: level growth 27 → 168 → 1160
    /// (ratio → φ⁴), and the walk tiles with anchor radius < 12 are
    /// *precisely* the 253 reference poses of the inner disk — no
    /// missing tile and no spurious one.
    #[test]
    fn walk_reproduces_the_decoded_reference_patch() {
        let (sizes, seen) = walk(2);
        assert_eq!(sizes, vec![27, 168, 1160], "walk level sizes");
        let o = o_fix();
        let inner: BTreeSet<[i64; 6]> = seen
            .iter()
            .filter(|k| (kv([k[2], k[3], k[4], k[5]]) - o).length() < 12.0)
            .copied()
            .collect();
        let want: BTreeSet<[i64; 6]> = PATCH_REF
            .iter()
            .map(|(ty, pos, rot)| [*ty as i64, *rot as i64, pos[0], pos[1], pos[2], pos[3]])
            .collect();
        assert_eq!(inner.len(), 253);
        assert_eq!(inner, want, "walk vs decoded patch inner disk");
    }

    /// The walk tiles its union edge-to-edge with no overlaps: at level
    /// 1 every edge with both endpoints inside radius 8 of o appears
    /// exactly twice (shared by two tiles); likewise at level 2 inside
    /// radius 20.  A dangling edge would be a gap, an edge used three
    /// times an overlap — both are caught.
    #[test]
    fn walk_is_edge_to_edge_in_the_interior() {
        let mut seen: BTreeSet<[i64; 6]> = BTreeSet::new();
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
                frames_into(ob, &mut children);
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
                    pos: [k[2], k[3], k[4], k[5]],
                };
                let vs: Vec<Ke> = (0..6)
                    .map(|i| kadd(ob.pos, kmul(UP[ob.rot as usize], tile_local(ob.ty)[i])))
                    .collect();
                for i in 0..6 {
                    let (a, b) = (vs[i], vs[(i + 1) % 6]);
                    let key = if a <= b { (a, b) } else { (b, a) };
                    *cnt.entry(key).or_insert(0) += 1;
                }
            }
            census.push(cnt);
            frontier = next;
        }
        for (idx, rad, min_checked) in [(0, 8.0, 250), (1, 20.0, 2_000)] {
            let mut checked = 0;
            for ((a, b), n) in &census[idx] {
                if (kv(*a) - o).length() < rad && (kv(*b) - o).length() < rad {
                    assert_eq!(*n, 2, "interior edge used {n} times (level {})", idx + 1);
                    checked += 1;
                }
            }
            assert!(checked > min_checked, "census window degenerate: {checked}");
        }
    }

    fn tile_local(ty: u8) -> &'static [Ke; 6] {
        if ty == TY_R {
            &BOW_L
        } else {
            &HEX_L
        }
    }

    /// Views beyond the reachable disk fail with a message instead of
    /// panicking or running away.
    #[test]
    fn unreachable_views_fail_gracefully() {
        let t = AndritzBowtieHexagonTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(
            crate::lattice::SCALE_ID,
            tess_core::params::ParamValue::Float(1.0),
        );
        let far = (COVERED[MAX_LEVEL] + 50.0) as f64;
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
