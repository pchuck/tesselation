//! Girih decagon patch: the classic two-level girih figure — the
//! Darb-i Imam large-scale decagon-and-bowtie network with every large
//! tile infilled by its small girih tiles — shipped as a finite
//! documentary patch table (the LOOK of the Lu & Steinhardt, Science
//! 315, 1106 (2007) two-level girih figures; the geometry authority is
//! `girih_qc.rs`'s certified substitution tables).
//!
//! The region is minimal for "one full large decagon plus its adjacent
//! large bow-ties": the seed decagon at the origin and its **10**
//! edge-adjacent large bow-ties (empirically all ten edge-neighbours of
//! the seed decagon in the certified generation-1 lineage are bow-ties,
//! a perfectly 10-fold ring).  Each of the 11 large tiles is replaced
//! by its **closed cluster** — the real substitution children plus the
//! straddling edge trio (`EDGE_RULE`) on every unique large-tile edge —
//! so the 611-tile union is a duplicate-free, edge-to-edge, T-junction-
//! free topological disk covering every large-tile outline exactly once
//! (all certified in the in-module battery against the Python
//! derivation; 270 bobbin + 220 bow-tie + 121 decagon tiles).
//!
//! The small-tile data (prototile vertices, rotation matrices,
//! `REAL_BOW`/`REAL_DEC` child tables, the edge rule, the exact
//! `K = Q(2 cos 18°)` arithmetic) is copied verbatim from
//! `girih_qc.rs` — this module assembles and presents that certified
//! content; it does not re-derive the substitution.  Floats appear only
//! at the emission boundary.
//!
//! Strapwork bands (the stretch goal) are NOT shipped: the strap chords
//! cross inside the decagon's {10/3}-star in a way whose planarization
//! exceeds the exact-partition contract (see the report); the patch is
//! tiles-only, which is contract-legal by the closed-cluster cover law.
//!
//! As a finite patch the domain IS the 611-tile union: views beyond the
//! rim return no tiles (the `jeandel-rao` precedent), and the default
//! view shows the whole 611-tile patch.

use std::collections::{BTreeSet, HashSet};
use std::sync::OnceLock;

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, SIMILARITY_PARAMS};

// =============================================================
// Exact tables — copied verbatim from `girih_qc.rs` (the certified
// Darb-i Imam substitution: BFS through exact edge-contact equations,
// every child centre cross-checked against reference doubles to
// <= 1.1e-14; EDGE markers == interior edges of the closed clusters,
// verified bijectively for all three parent types).
//
// K = Q(a), a = 2cos(pi/10); elements are integer 4-tuples over
// the basis (1, a, a^2, a^3) at the global denominator D = 64.
// =============================================================

/// Basis values for float evaluation of K elements.
const A: f64 = 1.902113032590307;
const A2: f64 = 3.6180339887498945;
const A3: f64 = 6.881909602355867;

/// Inflation factor IF = 4*a^2 - 6 = 2*(2+sqrt5) = 2*phi^3, exact and float.
const IFK: Ke = [-384, 0, 256, 0];
#[allow(dead_code)]
const IF_F: f64 = 8.472135954999578;

/// Prototile vertices (unit edges), exact K points, anticlockwise.
/// Type codes: 0 EDGE (marker), 1 HEX (bobbin), 2 BOW (bow-tie), 3 DEC.
#[allow(dead_code)]
const HEX_V: [Kp; 6] = [
    ([-32, 0, 32, 0], [0, 0, 0, 0]),
    ([32, 0, 0, 0], [0, -96, 0, 32]),
    ([-32, 0, 0, 0], [0, -96, 0, 32]),
    ([32, 0, -32, 0], [0, 0, 0, 0]),
    ([-32, 0, 0, 0], [0, 96, 0, -32]),
    ([32, 0, 0, 0], [0, 96, 0, -32]),
];
#[allow(dead_code)]
const BOW_V: [Kp; 6] = [
    ([0, 32, 0, 0], [32, 0, 0, 0]),
    ([0, 0, 0, 0], [128, 0, -32, 0]),
    ([0, -32, 0, 0], [32, 0, 0, 0]),
    ([0, -32, 0, 0], [-32, 0, 0, 0]),
    ([0, 0, 0, 0], [-128, 0, 32, 0]),
    ([0, 32, 0, 0], [-32, 0, 0, 0]),
];
#[allow(dead_code)]
const DEC_V: [Kp; 10] = [
    ([-128, 0, 64, 0], [0, 0, 0, 0]),
    ([-32, 0, 32, 0], [0, 32, 0, 0]),
    ([32, 0, 0, 0], [0, -64, 0, 32]),
    ([-32, 0, 0, 0], [0, -64, 0, 32]),
    ([32, 0, -32, 0], [0, 32, 0, 0]),
    ([128, 0, -64, 0], [0, 0, 0, 0]),
    ([32, 0, -32, 0], [0, -32, 0, 0]),
    ([-32, 0, 0, 0], [0, 64, 0, -32]),
    ([32, 0, 0, 0], [0, 64, 0, -32]),
    ([-32, 0, 32, 0], [0, -32, 0, 0]),
];

/// Prototile vertices as float (evaluated from the exact tables).
const HEX_F: [(f64, f64); 6] = [
    (1.3090169943749472, 0.0),
    (0.5, 0.5877852522924729),
    (-0.5, 0.5877852522924729),
    (-1.3090169943749472, 0.0),
    (-0.5, -0.5877852522924729),
    (0.5, -0.5877852522924729),
];
const BOW_F: [(f64, f64); 6] = [
    (0.9510565162951535, 0.5),
    (0.0, 0.19098300562505277),
    (-0.9510565162951535, 0.5),
    (-0.9510565162951535, -0.5),
    (0.0, -0.19098300562505277),
    (0.9510565162951535, -0.5),
];
const DEC_F: [(f64, f64); 10] = [
    (1.6180339887498945, 0.0),
    (1.3090169943749472, 0.9510565162951535),
    (0.5, 1.5388417685876263),
    (-0.5, 1.5388417685876263),
    (-1.3090169943749472, 0.9510565162951535),
    (-1.6180339887498945, 0.0),
    (-1.3090169943749472, -0.9510565162951535),
    (-0.5, -1.5388417685876263),
    (0.5, -1.5388417685876263),
    (1.3090169943749472, -0.9510565162951535),
];

/// The 20 exact rotation matrices R_r (rotation by 18r degrees),
/// each ((m00, m01), (m10, m11)) acting on column points.
const ROTM: [[[Ke; 2]; 2]; 20] = [
    [[[64, 0, 0, 0], [0, 0, 0, 0]], [[0, 0, 0, 0], [64, 0, 0, 0]]],
    [
        [[0, 32, 0, 0], [96, 0, -32, 0]],
        [[-96, 0, 32, 0], [0, 32, 0, 0]],
    ],
    [
        [[-64, 0, 32, 0], [0, 96, 0, -32]],
        [[0, -96, 0, 32], [-64, 0, 32, 0]],
    ],
    [
        [[0, -96, 0, 32], [64, 0, -32, 0]],
        [[-64, 0, 32, 0], [0, -96, 0, 32]],
    ],
    [
        [[-96, 0, 32, 0], [0, -32, 0, 0]],
        [[0, 32, 0, 0], [-96, 0, 32, 0]],
    ],
    [
        [[0, 0, 0, 0], [-64, 0, 0, 0]],
        [[64, 0, 0, 0], [0, 0, 0, 0]],
    ],
    [
        [[96, 0, -32, 0], [0, -32, 0, 0]],
        [[0, 32, 0, 0], [96, 0, -32, 0]],
    ],
    [
        [[0, 96, 0, -32], [64, 0, -32, 0]],
        [[-64, 0, 32, 0], [0, 96, 0, -32]],
    ],
    [
        [[64, 0, -32, 0], [0, 96, 0, -32]],
        [[0, -96, 0, 32], [64, 0, -32, 0]],
    ],
    [
        [[0, -32, 0, 0], [96, 0, -32, 0]],
        [[-96, 0, 32, 0], [0, -32, 0, 0]],
    ],
    [
        [[-64, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [-64, 0, 0, 0]],
    ],
    [
        [[0, -32, 0, 0], [-96, 0, 32, 0]],
        [[96, 0, -32, 0], [0, -32, 0, 0]],
    ],
    [
        [[64, 0, -32, 0], [0, -96, 0, 32]],
        [[0, 96, 0, -32], [64, 0, -32, 0]],
    ],
    [
        [[0, 96, 0, -32], [-64, 0, 32, 0]],
        [[64, 0, -32, 0], [0, 96, 0, -32]],
    ],
    [
        [[96, 0, -32, 0], [0, 32, 0, 0]],
        [[0, -32, 0, 0], [96, 0, -32, 0]],
    ],
    [
        [[0, 0, 0, 0], [64, 0, 0, 0]],
        [[-64, 0, 0, 0], [0, 0, 0, 0]],
    ],
    [
        [[-96, 0, 32, 0], [0, 32, 0, 0]],
        [[0, -32, 0, 0], [-96, 0, 32, 0]],
    ],
    [
        [[0, -96, 0, 32], [-64, 0, 32, 0]],
        [[64, 0, -32, 0], [0, -96, 0, 32]],
    ],
    [
        [[-64, 0, 32, 0], [0, -96, 0, 32]],
        [[0, 96, 0, -32], [-64, 0, 32, 0]],
    ],
    [
        [[0, 32, 0, 0], [-96, 0, 32, 0]],
        [[96, 0, -32, 0], [0, 32, 0, 0]],
    ],
];

/// The 20 rotation matrices as float rows (m00, m01, m10, m11).
const ROT_F: [[f64; 4]; 20] = [
    [1.0, 0.0, 0.0, 1.0],
    [
        0.9510565162951535,
        -0.30901699437494723,
        0.30901699437494723,
        0.9510565162951535,
    ],
    [
        0.8090169943749472,
        -0.5877852522924729,
        0.5877852522924729,
        0.8090169943749472,
    ],
    [
        0.5877852522924729,
        -0.8090169943749472,
        0.8090169943749472,
        0.5877852522924729,
    ],
    [
        0.30901699437494723,
        -0.9510565162951535,
        0.9510565162951535,
        0.30901699437494723,
    ],
    [0.0, -1.0, 1.0, 0.0],
    [
        -0.30901699437494723,
        -0.9510565162951535,
        0.9510565162951535,
        -0.30901699437494723,
    ],
    [
        -0.5877852522924729,
        -0.8090169943749472,
        0.8090169943749472,
        -0.5877852522924729,
    ],
    [
        -0.8090169943749472,
        -0.5877852522924729,
        0.5877852522924729,
        -0.8090169943749472,
    ],
    [
        -0.9510565162951535,
        -0.30901699437494723,
        0.30901699437494723,
        -0.9510565162951535,
    ],
    [-1.0, 0.0, 0.0, -1.0],
    [
        -0.9510565162951535,
        0.30901699437494723,
        -0.30901699437494723,
        -0.9510565162951535,
    ],
    [
        -0.8090169943749472,
        0.5877852522924729,
        -0.5877852522924729,
        -0.8090169943749472,
    ],
    [
        -0.5877852522924729,
        0.8090169943749472,
        -0.8090169943749472,
        -0.5877852522924729,
    ],
    [
        -0.30901699437494723,
        0.9510565162951535,
        -0.9510565162951535,
        -0.30901699437494723,
    ],
    [0.0, 1.0, -1.0, 0.0],
    [
        0.30901699437494723,
        0.9510565162951535,
        -0.9510565162951535,
        0.30901699437494723,
    ],
    [
        0.5877852522924729,
        0.8090169943749472,
        -0.8090169943749472,
        0.5877852522924729,
    ],
    [
        0.8090169943749472,
        0.5877852522924729,
        -0.5877852522924729,
        0.8090169943749472,
    ],
    [
        0.9510565162951535,
        0.30901699437494723,
        -0.30901699437494723,
        0.9510565162951535,
    ],
];

/// The edge rule: the trio straddling a marker segment, in the
/// segment's frame (x along the edge, origin at its midpoint):
/// HEX, DEC, HEX at offsets -(3phi+1)/2, 0, +(3phi+1)/2.
const EDGE_RULE_TY: [u8; 3] = [1, 3, 1];
const EDGE_RULE_ROT: [u8; 3] = [0, 0, 0];
const EDGE_RULE_OFF: [Kp; 3] = [
    ([160, 0, -96, 0], [0, 0, 0, 0]),
    ([0, 0, 0, 0], [0, 0, 0, 0]),
    ([-160, 0, 96, 0], [0, 0, 0, 0]),
];

/// Substitution children (real tiles) per parent type:
/// `(child type, child rotation, offset x, offset y)` in the parent's
/// frame scaled by IF (children are unit tiles around the inflated
/// parent).  Determined by BFS through the exact edge-contact
/// equations; cycle-consistent and cross-checked against doubles.
const REAL_BOW: [(u8, u8, [i64; 4], [i64; 4]); 25] = [
    (2, 10, [0, 128, 0, -96], [160, 0, -96, 0]),
    (2, 10, [0, 128, 0, -96], [-160, 0, 96, 0]),
    (2, 8, [0, 144, 0, -96], [80, 0, -48, 0]),
    (2, 12, [0, 144, 0, -96], [-80, 0, 48, 0]),
    (2, 10, [0, 160, 0, -96], [0, 0, 0, 0]),
    (2, 12, [0, 48, 0, -32], [80, 0, -48, 0]),
    (2, 8, [0, 48, 0, -32], [-80, 0, 48, 0]),
    (2, 18, [0, -48, 0, 32], [80, 0, -48, 0]),
    (2, 2, [0, -48, 0, 32], [-80, 0, 48, 0]),
    (2, 0, [0, -160, 0, 96], [0, 0, 0, 0]),
    (2, 2, [0, -144, 0, 96], [80, 0, -48, 0]),
    (2, 18, [0, -144, 0, 96], [-80, 0, 48, 0]),
    (2, 0, [0, -128, 0, 96], [160, 0, -96, 0]),
    (2, 0, [0, -128, 0, 96], [-160, 0, 96, 0]),
    (3, 5, [0, 128, 0, -64], [0, 0, 0, 0]),
    (3, 5, [0, 0, 0, 0], [0, 0, 0, 0]),
    (3, 15, [0, -128, 0, 64], [0, 0, 0, 0]),
    (1, 9, [0, 176, 0, -112], [160, 0, -80, 0]),
    (1, 11, [0, 176, 0, -112], [-160, 0, 80, 0]),
    (1, 9, [0, 112, 0, -80], [0, 0, -16, 0]),
    (1, 11, [0, 112, 0, -80], [0, 0, 16, 0]),
    (1, 1, [0, -112, 0, 80], [0, 0, -16, 0]),
    (1, 19, [0, -112, 0, 80], [0, 0, 16, 0]),
    (1, 1, [0, -176, 0, 112], [160, 0, -80, 0]),
    (1, 19, [0, -176, 0, 112], [-160, 0, 80, 0]),
];

const REAL_DEC: [(u8, u8, [i64; 4], [i64; 4]); 181] = [
    (2, 15, [480, 0, -352, 0], [0, -64, 0, 32]),
    (2, 5, [480, 0, -352, 0], [0, 64, 0, -32]),
    (2, 3, [400, 0, -304, 0], [0, -144, 0, 96]),
    (2, 13, [400, 0, -304, 0], [0, 48, 0, -32]),
    (2, 7, [400, 0, -304, 0], [0, -48, 0, 32]),
    (2, 17, [400, 0, -304, 0], [0, 144, 0, -96]),
    (2, 9, [400, 0, -288, 0], [0, 128, 0, -80]),
    (2, 9, [400, 0, -288, 0], [0, -64, 0, 48]),
    (2, 11, [400, 0, -288, 0], [0, -128, 0, 80]),
    (2, 11, [400, 0, -288, 0], [0, 64, 0, -48]),
    (2, 13, [400, 0, -272, 0], [0, -176, 0, 128]),
    (2, 7, [400, 0, -272, 0], [0, 176, 0, -128]),
    (2, 15, [320, 0, -224, 0], [0, 192, 0, -128]),
    (2, 5, [320, 0, -224, 0], [0, -192, 0, 128]),
    (2, 9, [240, 0, -192, 0], [0, 32, 0, -16]),
    (2, 11, [240, 0, -192, 0], [0, -32, 0, 16]),
    (2, 7, [240, 0, -176, 0], [0, -176, 0, 128]),
    (2, 13, [240, 0, -176, 0], [0, 176, 0, -128]),
    (2, 13, [240, 0, -176, 0], [0, 112, 0, -64]),
    (2, 7, [240, 0, -176, 0], [0, -112, 0, 64]),
    (2, 1, [240, 0, -160, 0], [0, -256, 0, 176]),
    (2, 11, [240, 0, -160, 0], [0, 192, 0, -144]),
    (2, 9, [240, 0, -160, 0], [0, -128, 0, 80]),
    (2, 9, [240, 0, -160, 0], [0, -192, 0, 144]),
    (2, 19, [240, 0, -160, 0], [0, 256, 0, -176]),
    (2, 11, [240, 0, -160, 0], [0, 128, 0, -80]),
    (2, 5, [160, 0, -96, 0], [0, -128, 0, 96]),
    (2, 15, [160, 0, -96, 0], [0, 128, 0, -96]),
    (2, 9, [80, 0, -64, 0], [0, 224, 0, -176]),
    (2, 11, [80, 0, -64, 0], [0, 32, 0, -16]),
    (2, 9, [80, 0, -64, 0], [0, -32, 0, 16]),
    (2, 11, [80, 0, -64, 0], [0, -224, 0, 176]),
    (2, 13, [80, 0, -48, 0], [0, 112, 0, -96]),
    (2, 7, [80, 0, -48, 0], [0, -48, 0, 32]),
    (2, 17, [80, 0, -48, 0], [0, 208, 0, -160]),
    (2, 13, [80, 0, -48, 0], [0, 48, 0, -32]),
    (2, 7, [80, 0, -48, 0], [0, -112, 0, 96]),
    (2, 3, [80, 0, -48, 0], [0, -208, 0, 160]),
    (2, 15, [0, 0, 0, 0], [0, 224, 0, -160]),
    (2, 15, [0, 0, 0, 0], [0, 32, 0, -32]),
    (2, 5, [0, 0, 0, 0], [0, -32, 0, 32]),
    (2, 5, [0, 0, 0, 0], [0, -224, 0, 160]),
    (2, 13, [-80, 0, 48, 0], [0, 208, 0, -160]),
    (2, 3, [-80, 0, 48, 0], [0, -48, 0, 32]),
    (2, 17, [-80, 0, 48, 0], [0, 112, 0, -96]),
    (2, 17, [-80, 0, 48, 0], [0, 48, 0, -32]),
    (2, 3, [-80, 0, 48, 0], [0, -112, 0, 96]),
    (2, 7, [-80, 0, 48, 0], [0, -208, 0, 160]),
    (2, 19, [-80, 0, 64, 0], [0, 32, 0, -16]),
    (2, 1, [-80, 0, 64, 0], [0, -32, 0, 16]),
    (2, 1, [-80, 0, 64, 0], [0, 224, 0, -176]),
    (2, 19, [-80, 0, 64, 0], [0, -224, 0, 176]),
    (2, 5, [-160, 0, 96, 0], [0, -128, 0, 96]),
    (2, 15, [-160, 0, 96, 0], [0, 128, 0, -96]),
    (2, 9, [-240, 0, 160, 0], [0, -256, 0, 176]),
    (2, 19, [-240, 0, 160, 0], [0, 192, 0, -144]),
    (2, 11, [-240, 0, 160, 0], [0, 256, 0, -176]),
    (2, 19, [-240, 0, 160, 0], [0, 128, 0, -80]),
    (2, 1, [-240, 0, 160, 0], [0, -128, 0, 80]),
    (2, 1, [-240, 0, 160, 0], [0, -192, 0, 144]),
    (2, 3, [-240, 0, 176, 0], [0, -176, 0, 128]),
    (2, 17, [-240, 0, 176, 0], [0, 176, 0, -128]),
    (2, 17, [-240, 0, 176, 0], [0, 112, 0, -64]),
    (2, 3, [-240, 0, 176, 0], [0, -112, 0, 64]),
    (2, 1, [-240, 0, 192, 0], [0, 32, 0, -16]),
    (2, 19, [-240, 0, 192, 0], [0, -32, 0, 16]),
    (2, 5, [-320, 0, 224, 0], [0, -192, 0, 128]),
    (2, 15, [-320, 0, 224, 0], [0, 192, 0, -128]),
    (2, 17, [-400, 0, 272, 0], [0, -176, 0, 128]),
    (2, 3, [-400, 0, 272, 0], [0, 176, 0, -128]),
    (2, 19, [-400, 0, 288, 0], [0, -128, 0, 80]),
    (2, 19, [-400, 0, 288, 0], [0, 64, 0, -48]),
    (2, 1, [-400, 0, 288, 0], [0, 128, 0, -80]),
    (2, 1, [-400, 0, 288, 0], [0, -64, 0, 48]),
    (2, 13, [-400, 0, 304, 0], [0, 144, 0, -96]),
    (2, 17, [-400, 0, 304, 0], [0, 48, 0, -32]),
    (2, 3, [-400, 0, 304, 0], [0, -48, 0, 32]),
    (2, 7, [-400, 0, 304, 0], [0, -144, 0, 96]),
    (2, 5, [-480, 0, 352, 0], [0, 64, 0, -32]),
    (2, 15, [-480, 0, 352, 0], [0, -64, 0, 32]),
    (3, 10, [320, 0, -256, 0], [0, 0, 0, 0]),
    (3, 12, [320, 0, -224, 0], [0, 160, 0, -96]),
    (3, 14, [320, 0, -224, 0], [0, -32, 0, 32]),
    (3, 8, [320, 0, -224, 0], [0, -160, 0, 96]),
    (3, 16, [320, 0, -224, 0], [0, 32, 0, -32]),
    (3, 12, [160, 0, -128, 0], [0, -128, 0, 96]),
    (3, 18, [160, 0, -128, 0], [0, 128, 0, -96]),
    (3, 14, [160, 0, -128, 0], [0, -64, 0, 32]),
    (3, 16, [160, 0, -128, 0], [0, 64, 0, -32]),
    (3, 14, [160, 0, -96, 0], [0, 160, 0, -128]),
    (3, 18, [160, 0, -96, 0], [0, 96, 0, -64]),
    (3, 12, [160, 0, -96, 0], [0, -96, 0, 64]),
    (3, 6, [160, 0, -96, 0], [0, -160, 0, 128]),
    (3, 0, [0, 0, 0, 0], [0, 192, 0, -128]),
    (3, 0, [0, 0, 0, 0], [0, 64, 0, -64]),
    (3, 0, [0, 0, 0, 0], [0, 0, 0, 0]),
    (3, 10, [0, 0, 0, 0], [0, -64, 0, 64]),
    (3, 10, [0, 0, 0, 0], [0, -192, 0, 128]),
    (3, 16, [-160, 0, 96, 0], [0, 160, 0, -128]),
    (3, 8, [-160, 0, 96, 0], [0, -96, 0, 64]),
    (3, 2, [-160, 0, 96, 0], [0, 96, 0, -64]),
    (3, 4, [-160, 0, 96, 0], [0, -160, 0, 128]),
    (3, 8, [-160, 0, 128, 0], [0, -128, 0, 96]),
    (3, 2, [-160, 0, 128, 0], [0, 128, 0, -96]),
    (3, 4, [-160, 0, 128, 0], [0, 64, 0, -32]),
    (3, 6, [-160, 0, 128, 0], [0, -64, 0, 32]),
    (3, 2, [-320, 0, 224, 0], [0, -160, 0, 96]),
    (3, 4, [-320, 0, 224, 0], [0, 32, 0, -32]),
    (3, 18, [-320, 0, 224, 0], [0, 160, 0, -96]),
    (3, 6, [-320, 0, 224, 0], [0, -32, 0, 32]),
    (3, 0, [-320, 0, 256, 0], [0, 0, 0, 0]),
    (1, 10, [480, 0, -352, 0], [0, 0, 0, 0]),
    (1, 16, [480, 0, -336, 0], [0, -16, 0, 16]),
    (1, 4, [480, 0, -336, 0], [0, 16, 0, -16]),
    (1, 8, [400, 0, -288, 0], [0, -160, 0, 112]),
    (1, 12, [400, 0, -288, 0], [0, 160, 0, -112]),
    (1, 18, [400, 0, -288, 0], [0, 96, 0, -80]),
    (1, 2, [400, 0, -288, 0], [0, -96, 0, 80]),
    (1, 12, [400, 0, -288, 0], [0, 96, 0, -48]),
    (1, 8, [400, 0, -288, 0], [0, -96, 0, 48]),
    (1, 10, [320, 0, -256, 0], [0, 96, 0, -64]),
    (1, 10, [320, 0, -256, 0], [0, -96, 0, 64]),
    (1, 14, [320, 0, -240, 0], [0, -144, 0, 112]),
    (1, 6, [320, 0, -240, 0], [0, 144, 0, -112]),
    (1, 14, [320, 0, -208, 0], [0, 144, 0, -112]),
    (1, 6, [320, 0, -208, 0], [0, -144, 0, 112]),
    (1, 10, [160, 0, -160, 0], [0, 0, 0, 0]),
    (1, 8, [240, 0, -160, 0], [0, -224, 0, 144]),
    (1, 12, [240, 0, -160, 0], [0, 224, 0, -144]),
    (1, 12, [240, 0, -160, 0], [0, 160, 0, -80]),
    (1, 8, [240, 0, -160, 0], [0, -160, 0, 80]),
    (1, 0, [160, 0, -128, 0], [0, -224, 0, 160]),
    (1, 0, [160, 0, -128, 0], [0, 224, 0, -160]),
    (1, 6, [160, 0, -112, 0], [0, -240, 0, 176]),
    (1, 14, [160, 0, -112, 0], [0, 240, 0, -176]),
    (1, 0, [160, 0, -96, 0], [0, 0, 0, 0]),
    (1, 8, [80, 0, -64, 0], [0, 256, 0, -176]),
    (1, 2, [80, 0, -64, 0], [0, 0, 0, -16]),
    (1, 18, [80, 0, -64, 0], [0, 0, 0, 16]),
    (1, 12, [80, 0, -64, 0], [0, -256, 0, 176]),
    (1, 6, [160, 0, -80, 0], [0, -80, 0, 80]),
    (1, 14, [160, 0, -80, 0], [0, 80, 0, -80]),
    (1, 16, [0, 0, -16, 0], [0, 176, 0, -144]),
    (1, 16, [0, 0, -16, 0], [0, -80, 0, 48]),
    (1, 4, [0, 0, -16, 0], [0, 80, 0, -48]),
    (1, 4, [0, 0, -16, 0], [0, -176, 0, 144]),
    (1, 14, [0, 0, 16, 0], [0, 176, 0, -144]),
    (1, 14, [0, 0, 16, 0], [0, -80, 0, 48]),
    (1, 6, [0, 0, 16, 0], [0, 80, 0, -48]),
    (1, 6, [0, 0, 16, 0], [0, -176, 0, 144]),
    (1, 16, [-160, 0, 80, 0], [0, 80, 0, -80]),
    (1, 4, [-160, 0, 80, 0], [0, -80, 0, 80]),
    (1, 8, [-80, 0, 64, 0], [0, 0, 0, -16]),
    (1, 12, [-80, 0, 64, 0], [0, 0, 0, 16]),
    (1, 2, [-80, 0, 64, 0], [0, 256, 0, -176]),
    (1, 18, [-80, 0, 64, 0], [0, -256, 0, 176]),
    (1, 10, [-160, 0, 96, 0], [0, 0, 0, 0]),
    (1, 16, [-160, 0, 112, 0], [0, 240, 0, -176]),
    (1, 4, [-160, 0, 112, 0], [0, -240, 0, 176]),
    (1, 10, [-160, 0, 128, 0], [0, -224, 0, 160]),
    (1, 10, [-160, 0, 128, 0], [0, 224, 0, -160]),
    (1, 18, [-240, 0, 160, 0], [0, 224, 0, -144]),
    (1, 18, [-240, 0, 160, 0], [0, 160, 0, -80]),
    (1, 2, [-240, 0, 160, 0], [0, -160, 0, 80]),
    (1, 2, [-240, 0, 160, 0], [0, -224, 0, 144]),
    (1, 0, [-160, 0, 160, 0], [0, 0, 0, 0]),
    (1, 4, [-320, 0, 208, 0], [0, -144, 0, 112]),
    (1, 16, [-320, 0, 208, 0], [0, 144, 0, -112]),
    (1, 16, [-320, 0, 240, 0], [0, -144, 0, 112]),
    (1, 4, [-320, 0, 240, 0], [0, 144, 0, -112]),
    (1, 0, [-320, 0, 256, 0], [0, -96, 0, 64]),
    (1, 0, [-320, 0, 256, 0], [0, 96, 0, -64]),
    (1, 2, [-400, 0, 288, 0], [0, -160, 0, 112]),
    (1, 18, [-400, 0, 288, 0], [0, 160, 0, -112]),
    (1, 12, [-400, 0, 288, 0], [0, 96, 0, -80]),
    (1, 18, [-400, 0, 288, 0], [0, 96, 0, -48]),
    (1, 2, [-400, 0, 288, 0], [0, -96, 0, 48]),
    (1, 8, [-400, 0, 288, 0], [0, -96, 0, 80]),
    (1, 6, [-480, 0, 336, 0], [0, 16, 0, -16]),
    (1, 14, [-480, 0, 336, 0], [0, -16, 0, 16]),
    (1, 0, [-480, 0, 352, 0], [0, 0, 0, 0]),
];

/// Exact tile areas (unit edges): DEC, HEX, BOW.
#[allow(dead_code)]
const AREA_DEC: f64 = 7.694208842938131;
#[allow(dead_code)]
const AREA_HEX: f64 = 2.126627020880099;
#[allow(dead_code)]
const AREA_BOW: f64 = 1.3143277802978344;

// ===== exact arithmetic in K = Q(a), a = 2cos(pi/10) =====

/// A K element: c0 + c1·a + c2·a² + c3·a³, divided by the global
/// denominator 64.  All construction quantities have true denominators
/// dividing 8, so products have denominators dividing 64 and the fixed
/// point representation is closed under the operations used here.
type Ke = [i64; 4];

/// A point with K coordinates.
type Kp = (Ke, Ke);

const ZERO: Ke = [0, 0, 0, 0];

#[inline]
fn kadd(x: Ke, y: Ke) -> Ke {
    [x[0] + y[0], x[1] + y[1], x[2] + y[2], x[3] + y[3]]
}

#[inline]
fn kmul(x: Ke, y: Ke) -> Ke {
    let p0 = x[0] * y[0];
    let p1 = x[0] * y[1] + x[1] * y[0];
    let p2 = x[0] * y[2] + x[1] * y[1] + x[2] * y[0];
    let p3 = x[0] * y[3] + x[1] * y[2] + x[2] * y[1] + x[3] * y[0];
    let p4 = x[1] * y[3] + x[2] * y[2] + x[3] * y[1];
    let p5 = x[2] * y[3] + x[3] * y[2];
    let p6 = x[3] * y[3];
    // reductions: a^4 = 5a^2 - 5, a^5 = 5a^3 - 5a, a^6 = 20a^2 - 25
    let c0 = p0 - 5 * p4 - 25 * p6;
    let c1 = p1 - 5 * p5;
    let c2 = p2 + 5 * p4 + 20 * p6;
    let c3 = p3 + 5 * p5;
    debug_assert!(
        c0 % 64 == 0 && c1 % 64 == 0 && c2 % 64 == 0 && c3 % 64 == 0,
        "K product left the fixed denominator"
    );
    [c0 / 64, c1 / 64, c2 / 64, c3 / 64]
}

/// Float value of a K element (used for window tests and emission; all
/// comparisons that matter for correctness are exact tuple tests).
#[inline]
fn kval(x: Ke) -> f64 {
    (x[0] as f64 + x[1] as f64 * A + x[2] as f64 * A2 + x[3] as f64 * A3) / 64.0
}

#[inline]
fn pval(p: Kp) -> (f64, f64) {
    (kval(p.0), kval(p.1))
}

/// Apply the exact rotation matrix `ROTM[r]` to a point.
#[inline]
fn rot_kp(r: usize, p: Kp) -> Kp {
    let m = &ROTM[r];
    (
        kadd(kmul(m[0][0], p.0), kmul(m[0][1], p.1)),
        kadd(kmul(m[1][0], p.0), kmul(m[1][1], p.1)),
    )
}

// ===== substitution objects =====

/// One object of the recursion: a tile (type 1 HEX, 2 BOW, 3 DEC) or an
/// edge marker (type 0), its rotation in 18° units, and its exact centre.
#[derive(Clone, Copy)]
struct Obj {
    ty: u8,
    rot: u8,
    pos: Kp,
}

// ===== the patch region =====

/// Half — the unit-edge midpoint offset used on canonical edges.
const HALF: Ke = [32, 0, 0, 0];

#[inline]
fn padd(p: Kp, q: Kp) -> Kp {
    (kadd(p.0, q.0), kadd(p.1, q.1))
}

#[inline]
fn psub(p: Kp, q: Kp) -> Kp {
    (
        [
            p.0[0] - q.0[0],
            p.0[1] - q.0[1],
            p.0[2] - q.0[2],
            p.0[3] - q.0[3],
        ],
        [
            p.1[0] - q.1[0],
            p.1[1] - q.1[1],
            p.1[2] - q.1[2],
            p.1[3] - q.1[3],
        ],
    )
}

/// Canonical unit direction of class `r` (the first column of ROTM).
fn canon(r: usize) -> Kp {
    (ROTM[r][0][0], ROTM[r][1][0])
}

/// Exact direction class of a K vector, or -1 if not canonical.
fn dir_class(v: Kp) -> i32 {
    (0..20).find(|&r| v == canon(r)).map_or(-1, |r| r as i32)
}

/// Exact local vertices of a prototile (1 HEX, 2 BOW, 3 DEC).
fn verts_of(ty: u8) -> &'static [Kp] {
    match ty {
        1 => &HEX_V,
        2 => &BOW_V,
        3 => &DEC_V,
        _ => unreachable!("no vertices for type {ty}"),
    }
}

fn real_table(ty: u8) -> &'static [(u8, u8, [i64; 4], [i64; 4])] {
    match ty {
        2 => &REAL_BOW,
        3 => &REAL_DEC,
        _ => unreachable!("large tiles are DEC or BOW"),
    }
}

/// Exact identity key of a tile (type, rotation, exact position).
fn obj_key(o: &Obj) -> [i64; 10] {
    [
        o.ty as i64,
        o.rot as i64,
        o.pos.0[0],
        o.pos.0[1],
        o.pos.0[2],
        o.pos.0[3],
        o.pos.1[0],
        o.pos.1[1],
        o.pos.1[2],
        o.pos.1[3],
    ]
}

/// The 11 large (level-1) tiles of the documentary region: the seed
/// decagon at the origin (rotation 0) and its 10 edge-adjacent large
/// bow-ties (rotations 1, 3, ..., 19).  Positions are generation-1
/// (unit-edge large-tile) coordinates; the table was extracted from the
/// certified generation-1 lineage of `girih_qc` (each bow-tie shares
/// exactly one unit edge with the central decagon — re-verified
/// exactly in the battery, and the 11-tile cluster is 10-fold
/// symmetric).
const LARGE: [(u8, u8, Ke, Ke); 11] = [
    (3, 0, [0, 0, 0, 0], [0, 0, 0, 0]),
    (2, 1, [-80, 0, 64, 0], [0, -32, 0, 16]),
    (2, 3, [-80, 0, 48, 0], [0, -48, 0, 32]),
    (2, 5, [0, 0, 0, 0], [0, -32, 0, 32]),
    (2, 7, [80, 0, -48, 0], [0, -48, 0, 32]),
    (2, 9, [80, 0, -64, 0], [0, -32, 0, 16]),
    (2, 11, [80, 0, -64, 0], [0, 32, 0, -16]),
    (2, 13, [80, 0, -48, 0], [0, 48, 0, -32]),
    (2, 15, [0, 0, 0, 0], [0, 32, 0, -32]),
    (2, 17, [-80, 0, 48, 0], [0, 48, 0, -32]),
    (2, 19, [-80, 0, 64, 0], [0, 32, 0, -16]),
];

/// The boundary trio straddling a large-tile edge `a -> b` (canonical
/// direction): the same rule as a girih_qc edge marker's substitution,
/// in the frame of the inflated large tile (children units).
fn boundary_trio(a: Kp, b: Kp) -> [Obj; 3] {
    let cls = dir_class(psub(b, a));
    assert!(cls >= 0, "large-tile edge not canonical");
    let mid = padd(a, rot_kp(cls as usize, (HALF, ZERO)));
    let base = (kmul(IFK, mid.0), kmul(IFK, mid.1));
    std::array::from_fn(|i| {
        let off = rot_kp(cls as usize, EDGE_RULE_OFF[i]);
        Obj {
            ty: EDGE_RULE_TY[i],
            rot: (cls as u8 + EDGE_RULE_ROT[i]) % 20,
            pos: (kadd(base.0, off.0), kadd(base.1, off.1)),
        }
    })
}

/// Exact vertices of a large tile (generation-1 frame).
fn large_verts(t: &(u8, u8, Ke, Ke)) -> Vec<Kp> {
    let (ty, rot, px, py) = *t;
    verts_of(ty)
        .iter()
        .map(|&v| padd((px, py), rot_kp(rot as usize, v)))
        .collect()
}

/// The real substitution children of one large tile (one inflation
/// step: `IF * pos + rot_kp(rot, off)`, girih_qc semantics).
fn children(t: &(u8, u8, Ke, Ke)) -> Vec<Obj> {
    let (ty, rot, px, py) = *t;
    let base = (kmul(IFK, px), kmul(IFK, py));
    real_table(ty)
        .iter()
        .map(|&(ct, cr, ox, oy)| {
            let off = rot_kp(rot as usize, (ox, oy));
            Obj {
                ty: ct,
                rot: (rot + cr) % 20,
                pos: (kadd(base.0, off.0), kadd(base.1, off.1)),
            }
        })
        .collect()
}

/// The raw patch assembly: the closed clusters of all 11 large tiles —
/// real children plus the straddling trio on every **unique** large-tile
/// edge (each decagon/bow-tie shared edge contributes its trio once).
fn patch_raw() -> Vec<Obj> {
    let mut out = Vec::with_capacity(640);
    let mut edges: BTreeSet<(Kp, Kp)> = BTreeSet::new();
    for t in LARGE.iter() {
        out.extend(children(t));
        let tv = large_verts(t);
        let n = tv.len();
        for i in 0..n {
            let (a, b) = (tv[i], tv[(i + 1) % n]);
            let e = if a <= b { (a, b) } else { (b, a) };
            if edges.insert(e) {
                out.extend(boundary_trio(e.0, e.1));
            }
        }
    }
    out
}

/// The patch tiles: `patch_raw` deduplicated by exact key.  The
/// deduplication is a safeguard — the raw assembly is duplicate-free
/// (asserted in the battery).
fn patch_tiles() -> Vec<Obj> {
    let mut seen: HashSet<[i64; 10]> = HashSet::with_capacity(640);
    patch_raw()
        .into_iter()
        .filter(|o| seen.insert(obj_key(o)))
        .collect()
}

/// Float polygon of a tile (unit-edge child frame).
fn poly_pts(o: &Obj) -> Vec<Vec2> {
    let (px, py) = pval(o.pos);
    let m = &ROT_F[o.rot as usize];
    locals_f(o.ty)
        .iter()
        .map(|&(lx, ly)| Vec2::new(px + m[0] * lx + m[1] * ly, py + m[2] * lx + m[3] * ly))
        .collect()
}

/// Float local vertices of a prototile (1 HEX, 2 BOW, 3 DEC).
fn locals_f(ty: u8) -> &'static [(f64, f64)] {
    match ty {
        1 => &HEX_F,
        2 => &BOW_F,
        3 => &DEC_F,
        _ => unreachable!("no vertices for type {ty}"),
    }
}

/// Output prototile index of a tile type (main/largest tile first):
/// DEC -> 0, HEX -> 1, BOW -> 2.
fn proto_of(ty: u8) -> u8 {
    match ty {
        3 => 0,
        1 => 1,
        2 => 2,
        _ => unreachable!("cannot emit a marker"),
    }
}

/// The patch polygons in tiling (unit-edge child) coordinates, built
/// once: `(proto index, vertices)` for all 611 tiles.
fn patch_polys() -> &'static Vec<(u8, Vec<Vec2>)> {
    static PATCH: OnceLock<Vec<(u8, Vec<Vec2>)>> = OnceLock::new();
    PATCH.get_or_init(|| {
        patch_tiles()
            .iter()
            .map(|o| (proto_of(o.ty), poly_pts(o)))
            .collect()
    })
}

/// Girih Decagon Patch (Darb-i Imam, two-level) — a finite documentary
/// patch of the quasiperiodic girih network.
pub struct GirihDecagonPatchTiling;

impl Tiling for GirihDecagonPatchTiling {
    fn id(&self) -> &'static str {
        "girih-decagon-patch"
    }

    fn name(&self) -> &'static str {
        "Girih Decagon Patch (Darb-i Imam, two-level)"
    }

    fn description(&self) -> &'static str {
        "Finite two-level girih patch: the Darb-i Imam closed clusters of the seed decagon and its ten edge-adjacent bow-ties (611 small girih tiles: 121 decagons, 270 bobbins, 220 bow-ties), assembled from the certified girih-quasiperiodic substitution tables. Exact Q(2cos 18 degrees) geometry, edge-to-edge and T-junction-free, exactly 10-fold symmetric; the documentary two-level decagon-and-bowtie figure of Lu & Steinhardt (Science 2007)."
    }

    fn category(&self) -> TilingCategory {
        // Classified Nonperiodic, not Islamic (review E17/A12): this is a
        // FINITE patch, and the browse card renders the wallpaper-group
        // line ("p1 · none") for every periodic category. As a
        // Nonperiodic primary the card omits the meaningless group and
        // states the declared translational character instead
        // ("finite patch (non-periodic) · 10-fold"), consistent with the
        // other non-repeating cards; the Hirschhorn pentagon is the
        // established precedent for a non-repeating structure carried by
        // a family register (Islamic here) with a Nonperiodic primary
        // tab. The islamic family docs and registration are unchanged —
        // the family docs guard keys on the register function, not the
        // category.
        TilingCategory::Nonperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn default_color_mode(&self) -> ColorMode {
        ColorMode::Proto
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "finite patch; two-level girih (Darb-i Imam substitution); 611 tiles; \
             exact 10-fold symmetry about the seed decagon",
        );
        let map = similarity_map(params);
        let inv = map.inverse();

        // Degenerate-view guard (patch-coordinates hull of the query).
        let mut rmin = Vec2::new(f64::MAX, f64::MAX);
        let mut rmax = Vec2::new(f64::MIN, f64::MIN);
        for &corner in query.region.corners().iter() {
            let q = inv.transform(corner);
            rmin.x = rmin.x.min(q.x);
            rmin.y = rmin.y.min(q.y);
            rmax.x = rmax.x.max(q.x);
            rmax.y = rmax.y.max(q.y);
        }
        if BBox::new(rmin, rmax).area() <= 1e-12 {
            return TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: None,
                message: Some("empty view"),
            };
        }

        // Finite patch: emit exactly the tiles reaching the query box.
        let mut tiles = Vec::with_capacity(611);
        for (proto, pts) in patch_polys() {
            let poly = Polygon::from_slice(pts).transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto: *proto,
                    seed: u32::from(*proto), level: 0,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{check_covers, check_det, check_rotated};
    use std::collections::{BTreeMap, HashMap};

    const ONE: Ke = [64, 0, 0, 0];

    fn ksub(x: Ke, y: Ke) -> Ke {
        [x[0] - y[0], x[1] - y[1], x[2] - y[2], x[3] - y[3]]
    }

    fn pdot(p: Kp, q: Kp) -> Ke {
        kadd(kmul(p.0, q.0), kmul(p.1, q.1))
    }

    fn pcross(p: Kp, q: Kp) -> Ke {
        ksub(kmul(p.0, q.1), kmul(p.1, q.0))
    }

    fn is_zero(x: Ke) -> bool {
        x == ZERO
    }

    fn tile_verts(o: &Obj) -> Vec<Kp> {
        verts_of(o.ty)
            .iter()
            .map(|&v| padd(o.pos, rot_kp(o.rot as usize, v)))
            .collect()
    }

    fn canon_edge(a: Kp, b: Kp) -> (Kp, Kp) {
        if a <= b {
            (a, b)
        } else {
            (b, a)
        }
    }

    fn fpoly_area(pts: &[(f64, f64)]) -> f64 {
        let n = pts.len();
        let mut s = 0.0;
        for i in 0..n {
            let (x1, y1) = pts[i];
            let (x2, y2) = pts[(i + 1) % n];
            s += x1 * y2 - x2 * y1;
        }
        s.abs() / 2.0
    }

    /// Ray-cast point-in-simple-polygon (mirror of the Python battery).
    fn fpoly_inside(polys: &[(f64, f64)], pt: (f64, f64)) -> bool {
        let (x, y) = pt;
        let n = polys.len();
        let mut c = 0;
        for i in 0..n {
            let (x1, y1) = polys[i];
            let (x2, y2) = polys[(i + 1) % n];
            if (y1 > y) != (y2 > y) {
                if x < x1 + (y - y1) * (x2 - x1) / (y2 - y1) {
                    c += 1;
                }
            }
        }
        c % 2 == 1
    }

    /// Cell-indexed float sampler over the patch tiles.
    struct Sampler {
        polys: Vec<Vec<(f64, f64)>>,
        boxes: Vec<(f64, f64, f64, f64)>,
        index: HashMap<(i32, i32), Vec<usize>>,
    }

    impl Sampler {
        fn new() -> Self {
            let mut polys: Vec<Vec<(f64, f64)>> = Vec::new();
            let mut boxes: Vec<(f64, f64, f64, f64)> = Vec::new();
            let mut index: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
            for o in &patch_tiles() {
                let fp: Vec<(f64, f64)> = tile_verts(o).iter().map(|&v| pval(v)).collect();
                let mut bx0 = f64::MAX;
                let mut bx1 = f64::MIN;
                let mut by0 = f64::MAX;
                let mut by1 = f64::MIN;
                for &(x, y) in fp.iter() {
                    bx0 = bx0.min(x);
                    bx1 = bx1.max(x);
                    by0 = by0.min(y);
                    by1 = by1.max(y);
                }
                let i = polys.len();
                for gx in (bx0 / 2.0).floor() as i32..=(bx1 / 2.0).floor() as i32 {
                    for gy in (by0 / 2.0).floor() as i32..=(by1 / 2.0).floor() as i32 {
                        index.entry((gx, gy)).or_default().push(i);
                    }
                }
                polys.push(fp);
                boxes.push((bx0, bx1, by0, by1));
            }
            Sampler {
                polys,
                boxes,
                index,
            }
        }

        fn coverage(&self, pt: (f64, f64)) -> usize {
            let gx = (pt.0 / 2.0).floor() as i32;
            let gy = (pt.1 / 2.0).floor() as i32;
            let mut c = 0;
            if let Some(list) = self.index.get(&(gx, gy)) {
                for &i in list {
                    let b = self.boxes[i];
                    if b.0 <= pt.0
                        && pt.0 <= b.1
                        && b.2 <= pt.1
                        && pt.1 <= b.3
                        && fpoly_inside(&self.polys[i], pt)
                    {
                        c += 1;
                    }
                }
            }
            c
        }
    }

    /// The large-tile outline inflated by IF into child units.
    fn outline(t: &(u8, u8, Ke, Ke)) -> Vec<(f64, f64)> {
        let (ty, rot, px, py) = *t;
        verts_of(ty)
            .iter()
            .map(|&v| {
                let (rx, ry) = rot_kp(rot as usize, v);
                pval((
                    kadd(kmul(IFK, rx), kmul(IFK, px)),
                    kadd(kmul(IFK, ry), kmul(IFK, py)),
                ))
            })
            .collect()
    }

    fn sample_region(
        s: &Sampler,
        poly: &[(f64, f64)],
        step: f64,
        ox: f64,
        oy: f64,
    ) -> (usize, usize) {
        let mut x0 = f64::MAX;
        let mut x1 = f64::MIN;
        let mut y0 = f64::MAX;
        let mut y1 = f64::MIN;
        for &(x, y) in poly {
            x0 = x0.min(x);
            x1 = x1.max(x);
            y0 = y0.min(y);
            y1 = y1.max(y);
        }
        let mut n = 0;
        let mut bad = 0;
        let mut x = x0 + ox;
        while x < x1 {
            let mut y = y0 + oy;
            while y < y1 {
                if fpoly_inside(poly, (x, y)) {
                    n += 1;
                    if s.coverage((x, y)) != 1 {
                        bad += 1;
                    }
                }
                y += step;
            }
            x += step;
        }
        (n, bad)
    }

    // ===== standard trio + patch-view tests =====

    #[test]
    fn girih_decagon_patch_covers_region() {
        check_covers(
            &GirihDecagonPatchTiling,
            "girih-decagon-patch",
            BBox::new(Vec2::new(-9.0, -9.0), Vec2::new(9.0, 9.0)),
        );
    }

    #[test]
    fn girih_decagon_patch_rotated_scaled() {
        check_rotated(
            &GirihDecagonPatchTiling,
            "girih-decagon-patch(rot22.5,scale0.7)",
            BBox::new(Vec2::new(-6.0, -6.0), Vec2::new(6.0, 6.0)),
            0.7,
            22.5,
        );
    }

    #[test]
    fn girih_decagon_patch_deterministic() {
        check_det(
            &GirihDecagonPatchTiling,
            "girih-decagon-patch",
            BBox::new(Vec2::new(-9.0, -9.0), Vec2::new(9.0, 9.0)),
        );
    }

    #[test]
    fn girih_decagon_patch_empty_beyond_patch() {
        let region = BBox::new(Vec2::new(60.0, 60.0), Vec2::new(70.0, 70.0));
        let p = ParamSet::default_for(GirihDecagonPatchTiling.params());
        let r = GirihDecagonPatchTiling.generate(&p, &Query { region });
        assert!(r.tiles.is_empty(), "tiles generated outside the patch");
        assert!(r.message.is_none(), "{:?}", r.message);
    }

    #[test]
    fn default_view_shows_the_whole_patch() {
        let region = BBox::new(Vec2::new(-30.0, -31.0), Vec2::new(30.0, 31.0));
        let p = ParamSet::default_for(GirihDecagonPatchTiling.params());
        let r = GirihDecagonPatchTiling.generate(&p, &Query { region });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(
            r.tiles.len(),
            611,
            "a view around the rim must show the whole patch"
        );
    }

    // ===== battery (mirror of the certified Python derivation) =====

    #[test]
    fn patch_census_and_closeness() {
        let raw = patch_raw();
        let patch = patch_tiles();
        assert_eq!(
            raw.len(),
            patch.len(),
            "the closed-cluster assembly has duplicates"
        );
        assert_eq!(patch.len(), 611);
        let hex = patch.iter().filter(|o| o.ty == 1).count();
        let bow = patch.iter().filter(|o| o.ty == 2).count();
        let dec = patch.iter().filter(|o| o.ty == 3).count();
        assert_eq!((hex, bow, dec), (270, 220, 121));
        let n_children: usize = LARGE.iter().map(|t| children(t).len()).sum();
        assert_eq!(n_children, 431);
        let mut uedges: BTreeSet<(Kp, Kp)> = BTreeSet::new();
        for t in LARGE.iter() {
            let tv = large_verts(t);
            for i in 0..tv.len() {
                uedges.insert(canon_edge(tv[i], tv[(i + 1) % tv.len()]));
            }
        }
        assert_eq!(uedges.len(), 60);
        assert_eq!(n_children + 3 * uedges.len(), 611);
    }

    #[test]
    fn large_ring_shares_edges_with_the_seed_decagon() {
        let dec = LARGE[0];
        assert_eq!(dec.0, 3);
        let dec_edges: BTreeSet<(Kp, Kp)> = {
            let tv = large_verts(&dec);
            (0..tv.len())
                .map(|i| canon_edge(tv[i], tv[(i + 1) % tv.len()]))
                .collect()
        };
        assert_eq!(dec_edges.len(), 10);
        for t in LARGE[1..].iter() {
            assert_eq!(t.0, 2, "large neighbours are bow-ties");
            let tv = large_verts(t);
            let shared: Vec<(Kp, Kp)> = (0..tv.len())
                .map(|i| canon_edge(tv[i], tv[(i + 1) % tv.len()]))
                .filter(|e| dec_edges.contains(e))
                .collect();
            assert_eq!(shared.len(), 1, "each bow-tie shares one edge");
        }
        // each DEC edge has exactly one bow-tie on the other side
        for e in dec_edges.iter() {
            let n = LARGE[1..]
                .iter()
                .filter(|t| {
                    let tv = large_verts(t);
                    (0..tv.len())
                        .map(|i| canon_edge(tv[i], tv[(i + 1) % tv.len()]))
                        .any(|f| &f == e)
                })
                .count();
            assert_eq!(n, 1, "decagon edge {e:?} not covered by exactly one bow-tie");
        }
    }

    #[test]
    fn patch_edge_census_exact() {
        let patch = patch_tiles();
        let mut edges: HashSet<(Kp, Kp)> = HashSet::new();
        for o in &patch {
            let tv = tile_verts(o);
            let n = tv.len();
            for i in 0..n {
                let (a, b) = (tv[i], tv[(i + 1) % n]);
                let e = psub(b, a);
                assert_eq!(pdot(e, e), ONE, "edge not unit");
                assert!(edges.insert((a, b)), "same-direction edge duplicated");
            }
        }
        let matched = edges.iter().filter(|(a, b)| edges.contains(&(*b, *a))).count();
        let boundary = edges.len() - matched;
        assert_eq!(edges.len(), 4150);
        assert_eq!(matched / 2, 1810);
        assert_eq!(boundary, 530);
    }

    #[test]
    fn patch_no_t_junctions() {
        let patch = patch_tiles();
        let mut verts: HashSet<Kp> = HashSet::new();
        let mut dir_edges: Vec<(Kp, Kp)> = Vec::new();
        for o in &patch {
            let tv = tile_verts(o);
            for i in 0..tv.len() {
                let (a, b) = (tv[i], tv[(i + 1) % tv.len()]);
                verts.insert(a);
                verts.insert(b);
                dir_edges.push((a, b));
            }
        }
        let mut grid: HashMap<(i32, i32), Vec<Kp>> = HashMap::new();
        for &v in &verts {
            let (x, y) = pval(v);
            grid.entry((x.div_euclid(4.0) as i32, y.div_euclid(4.0) as i32))
                .or_default()
                .push(v);
        }
        let mut tj = 0;
        for &(a, b) in &dir_edges {
            let (ax, ay) = pval(a);
            let (bx, by) = pval(b);
            let gxa = ax.min(bx).div_euclid(4.0) as i32 - 1;
            let gxb = ax.max(bx).div_euclid(4.0) as i32 + 1;
            let gya = ay.min(by).div_euclid(4.0) as i32 - 1;
            let gyb = ay.max(by).div_euclid(4.0) as i32 + 1;
            for gx in gxa..=gxb {
                for gy in gya..=gyb {
                    if let Some(cell) = grid.get(&(gx, gy)) {
                        for &v in cell {
                            if v == a || v == b {
                                continue;
                            }
                            if !is_zero(pcross(psub(b, a), psub(v, a))) {
                                continue;
                            }
                            let d = kval(pdot(psub(v, a), psub(b, a)));
                            if 1e-11 < d && d < 1.0 - 1e-11 {
                                tj += 1;
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(tj, 0, "T-junctions found");
    }

    #[test]
    fn patch_vertex_fans_legal() {
        let patch = patch_tiles();
        let mut corners: HashMap<Kp, Vec<(i32, i32)>> = HashMap::new();
        for o in &patch {
            let tv = tile_verts(o);
            let n = tv.len();
            for i in 0..n {
                let cur = tv[i];
                let oc = dir_class(psub(tv[(i + 1) % n], cur));
                let ic = dir_class(psub(tv[(i + n - 1) % n], cur));
                assert!(oc >= 0 && ic >= 0, "non-canonical edge direction");
                corners.entry(cur).or_default().push((oc, ic));
            }
        }
        let mut n_int = 0;
        for secs in corners.values() {
            let mut by_out: HashMap<i32, i32> = HashMap::with_capacity(secs.len());
            for &(oc, ic) in secs {
                assert!(by_out.insert(oc, ic).is_none(), "overlapping corners");
            }
            let mut heads: Vec<i32> = by_out
                .keys()
                .copied()
                .filter(|o| !by_out.values().any(|i| i == o))
                .collect();
            heads.sort_unstable();
            let starts: Vec<i32> = if heads.is_empty() {
                vec![by_out.keys().copied().min().unwrap()]
            } else {
                heads.clone()
            };
            let mut visited: HashSet<i32> = HashSet::new();
            let mut total = 0i32;
            let mut cycles = 0;
            let mut ok = true;
            for h in starts {
                let mut cur = h;
                while let Some(&nx) = by_out.get(&cur) {
                    if visited.contains(&cur) {
                        ok = false;
                        break;
                    }
                    visited.insert(cur);
                    total += (nx - cur).rem_euclid(20);
                    if nx == h {
                        if visited.len() < secs.len() {
                            ok = false;
                        }
                        cycles += 1;
                        break;
                    }
                    cur = nx;
                }
                if !ok {
                    break;
                }
            }
            assert!(ok && visited.len() == secs.len(), "bad fan at vertex");
            let interior = cycles == 1 && heads.is_empty() && total == 20;
            assert!(interior || cycles == 0, "neither interior nor boundary");
            if interior {
                n_int += 1;
            }
        }
        assert_eq!(corners.len(), 1730);
        assert_eq!(n_int, 1200);
    }

    #[test]
    fn patch_euler_chi() {
        let patch = patch_tiles();
        let mut und: BTreeSet<(Kp, Kp)> = BTreeSet::new();
        let mut verts: HashSet<Kp> = HashSet::new();
        for o in &patch {
            let tv = tile_verts(o);
            for i in 0..tv.len() {
                let (a, b) = (tv[i], tv[(i + 1) % tv.len()]);
                und.insert(canon_edge(a, b));
                verts.insert(a);
                verts.insert(b);
            }
        }
        assert_eq!(verts.len(), 1730);
        assert_eq!(und.len(), 2340);
        // V - E + F + 1 (outer face) == 2: a topological disk.
        assert_eq!(verts.len() as i64 - und.len() as i64 + patch.len() as i64 + 1, 2);
    }

    #[test]
    fn patch_symmetry_10fold() {
        let patch = patch_tiles();
        let sym = |o: &Obj| {
            let s: u8 = if o.ty == 3 { 2 } else { 10 };
            (o.ty, o.pos, o.rot % s)
        };
        let base: HashSet<(u8, Kp, u8)> = patch.iter().map(sym).collect();
        assert_eq!(base.len(), patch.len(), "symmetry keys collide");
        for steps in 1..10u32 {
            let rr: u8 = ((2 * steps) % 20) as u8;
            let got: HashSet<(u8, Kp, u8)> = patch
                .iter()
                .map(|o| {
                    let npos = rot_kp(rr as usize, o.pos);
                    let s: u8 = if o.ty == 3 { 2 } else { 10 };
                    (o.ty, npos, (o.rot + rr) % 20 % s)
                })
                .collect();
            assert_eq!(got, base, "symmetry broken at {steps}");
        }
    }

    #[test]
    fn patch_areas_and_boundary_cycle() {
        let patch = patch_tiles();
        let area_of = |ty: u8| match ty {
            1 => AREA_HEX,
            2 => AREA_BOW,
            _ => AREA_DEC,
        };
        let mut area_sum = 0.0;
        for o in &patch {
            let tv = tile_verts(o);
            let mut acc = ZERO;
            for i in 0..tv.len() {
                acc = kadd(acc, pcross(tv[i], tv[(i + 1) % tv.len()]));
            }
            let a = (kval(acc) / 2.0).abs();
            assert!((a - area_of(o.ty)).abs() < 1e-12, "tile area drift");
            area_sum += a;
        }
        assert!(
            (area_sum - 1794.340677298660).abs() < 1e-9,
            "total tile area {area_sum}"
        );

        // Boundary cycle (edges without a reverse partner) -> area identity.
        let mut edgeset: HashSet<(Kp, Kp)> = HashSet::new();
        for o in &patch {
            let tv = tile_verts(o);
            for i in 0..tv.len() {
                edgeset.insert((tv[i], tv[(i + 1) % tv.len()]));
            }
        }
        let mut und: BTreeMap<Kp, Kp> = BTreeMap::new();
        for &(a, b) in &edgeset {
            if !edgeset.contains(&(b, a)) {
                und.insert(a, b);
            }
        }
        assert_eq!(und.len(), 530);
        let start = *und.keys().next().unwrap();
        let mut cyc: Vec<Kp> = Vec::new();
        let mut cur = start;
        loop {
            cyc.push(cur);
            let nxt = und.remove(&cur).expect("boundary cycle broken");
            cur = nxt;
            if cur == start {
                break;
            }
        }
        assert_eq!(cyc.len(), 530);
        let barea = fpoly_area(&cyc.iter().map(|&v| pval(v)).collect::<Vec<_>>());
        assert!(
            (barea - area_sum).abs() < 1e-9,
            "boundary cycle area {barea} vs tile sum {area_sum}"
        );
    }

    #[test]
    fn patch_region_coverage_exactly_once() {
        let s = Sampler::new();
        let dec_outline = outline(&LARGE[0]);
        assert!(
            (fpoly_area(&dec_outline) - IF_F * IF_F * AREA_DEC).abs() < 1e-9,
            "large DEC outline area"
        );
        let bows: Vec<Vec<(f64, f64)>> = LARGE[1..].iter().map(outline).collect();
        for po in bows.iter() {
            assert!(
                (fpoly_area(po) - IF_F * IF_F * AREA_BOW).abs() < 1e-9,
                "large BOW outline area"
            );
        }
        let (n_dec, bad_dec) = sample_region(&s, &dec_outline, 0.21, 0.0733, 0.1321);
        assert_eq!((n_dec, bad_dec), (12516, 0), "DEC region sampling");
        let mut tot = n_dec;
        for po in bows.iter() {
            let (n, bad) = sample_region(&s, po, 0.21, 0.1477, 0.0619);
            assert_eq!(bad, 0, "BOW region: gap or overlap found");
            tot += n;
        }
        assert_eq!(tot, 33976);
    }

    #[test]
    fn patch_bbox_no_overlaps_no_gaps() {
        let s = Sampler::new();
        let mut x0 = f64::MAX;
        let mut x1 = f64::MIN;
        let mut y0 = f64::MAX;
        let mut y1 = f64::MIN;
        for fp in s.polys.iter() {
            for &(x, y) in fp.iter() {
                x0 = x0.min(x);
                x1 = x1.max(x);
                y0 = y0.min(y);
                y1 = y1.max(y);
            }
        }
        let outlines: Vec<Vec<(f64, f64)>> = LARGE.iter().map(outline).collect();
        let mut n_cov = 0;
        let mut over = 0;
        let mut gap = 0;
        let mut x = x0 + 0.0911;
        let mut j: i32 = 0;
        while x < x1 {
            let mut y = y0 + 0.2237 + 0.013 * j as f64;
            while y < y1 {
                let c = s.coverage((x, y));
                if c != 0 {
                    n_cov += 1;
                    if c > 1 {
                        over += 1;
                    }
                } else if outlines.iter().any(|rp| fpoly_inside(rp, (x, y))) {
                    gap += 1;
                }
                y += 0.17;
            }
            x += 0.17;
            j += 1;
        }
        assert_eq!((n_cov, over, gap), (61625, 0, 0));
    }
}
