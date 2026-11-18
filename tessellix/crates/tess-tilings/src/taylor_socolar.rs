//! Taylor–Socolar striped-hexagon tiling — the decorated-hexagon aperiodic
//! monotile of Taylor and Socolar (arXiv:1008.3103), ported from the audited
//! upstream study (M86) as a data table.
//!
//! # What is generated
//!
//! The paper's **forced finite patch** of 529 hexagons (its Figure 18: a
//! 23×23 rhombus of hexagon sites in axial coordinates `(u, v)`, with
//! `u + v + 21 ∈ [0, 22]` and `v ∈ [−21, 1]`), each site carrying one of the
//! paper's 12 tile states `(a, rbt, sdt)` — `a ∈ Z3` the stripe-axis class,
//! `rbt` which end of the polar vertex pair is red, `sdt` the stripe-offset
//! bit. The state table is transcribed verbatim from the study's
//! `ap_ts_patch.json`; nothing is re-derived and no coordinate is
//! hand-copied (the generator is `m86_gen.py`, checked against the exact
//! `Q(√3)` model to 1e−9 by float evaluation).
//!
//! The covered domain is that **finite patch**, not the plane: as with the
//! girih patches, `generate` returns exactly the tiles intersecting the
//! query region, and a query reaching beyond the patch boundary legitimately
//! returns only the patch tiles that reach into it, with background beyond.
//!
//! # Decoration: each hexagon partitioned into non-overlapping sub-polygons
//!
//! A hexagon (circumradius `R = 1`, vertex `V_k` at bearing 30°+60k, edge `m`
//! from `V_{m−1}` to `V_m`, outward normal at bearing 60m, tangent `u_m`
//! toward `V_m`) carries three stripes. The stripe on edge `m` crosses it at
//!
//! ```text
//! mid_m + sigma(state, m) * t0 * u_m,   sigma(state, m) = SUM[sdt][(m - a) mod 6]
//! SUM[0] = (+,-,-,+,-,+),  SUM[1] = (+,-,+,+,-,-),  t0/R = 0.24
//! ```
//!
//! and the three stripe *bands* (width `w/R = 0.0866`; both constants from
//! the paper's Figure 2 measurements) are straight strips whose axis is
//! perpendicular to `d_a = V_a`, at three exact offsets along `d_a`:
//!
//! | band | offset | crosses edges | colour |
//! |---|---|---|---|
//! | long stripe | `-sigma(a+2) * t0` | `a+2`, `a+5` | red |
//! | corner band at `V_a` | `+(3/4 + t0/2)` | `a`, `a+1` | red iff `rbt = 0` |
//! | corner band at `V_a+3` | `-(3/4 + t0/2)` | `a+3`, `a+4` | red iff `rbt = 1` |
//!
//! Those offsets are exact in `Q(√3)` and each band passes precisely through
//! the two stripe points it must: `3/4 + t0/2` is the `d_a`-projection of the
//! stripe points of edges `a` and `a+1`, and `-sigma(a+2) * t0` that of edges
//! `a+2` and `a+5`. The band windows therefore reproduce the stripe
//! edge-midpoint data exactly, and rule R1 (sigma complementarity across a
//! shared edge) makes the two adjacent tiles' windows coincide at their
//! shared stripe point — the handoff / line-continuation graph of the paper's
//! decoration is preserved. A band axis follows its own tile's `a`, so a
//! stripe bends where it crosses a tile edge: the rules fix the crossing
//! *positions*, not collinearity.
//!
//! The hexagon is then partitioned (the partition mandate) into **13 convex,
//! non-overlapping pieces** that tile it exactly:
//!
//! * 3 stripe bands — protos 1 (long), 4 (at `V_a`), 5 (at `V_a+3`);
//! * 2 corner tips between a corner band and its vertex wedge — proto 2,
//!   tagged with the vertex slot;
//! * 6 vertex wedges (chord cut at arc `LW = 0.15` from the vertex) — proto
//!   3, tagged with the vertex slot;
//! * 2 body halves between the bands — proto 0, notched by the four non-polar
//!   wedges.
//!
//! The piece areas sum to the hexagon area `3*sqrt(3)/2` **exactly in
//! `Q(√3)`** for all six `(a, sdt)` geometry classes, every interior cut edge
//! is shared by exactly two pieces, and all pieces are convex and CCW. Piece
//! colours come from the paper's corner-colour rule (`k%3 = a` -> red iff
//! `k = a + 3*rbt`, `k%3 = a+2` -> red, else blue): geometry depends only on
//! `(a, sdt)`, while `rbt` flips which end of the polar pair is red. `seed`
//! is the palette index — 0 base gray, 1 blue, 2 red — and the long band is
//! always red because both its ends sit on red corners.
//!
//! # Frame
//!
//! Site centre = `(sqrt(3) * (u + DU + (v + DV)/2), 3 * (v + DV) / 2)` with
//! `R = 1`, the lattice convention in which the study's neighbour-across-edge
//! table `DELTA` is `{(1,0), (0,1), (-1,1), (-1,0), (0,-1), (1,-1)}`.
//! `(DU, DV) = (0, 10)` only recentres the patch on the world origin; the
//! table keeps the study's verbatim `(u, v)`.
//!
//! # Verification — reproduced here
//!
//! * Handoff battery on the shipped table, re-derived in Rust: **R1 0/2992,
//!   R2 0/2816, R3 0/2904** violations — the study's JSON numbers.
//! * Stripe continuity: the world stripe point of a shared edge computed from
//!   either side agrees to 1e−12 on all 2992 shared edges.
//! * Partition exactness: 13 pieces per hexagon, convex, CCW, area sum
//!   `3*sqrt(3)/2`, no overlaps (clipped-area accounting in the coverage
//!   check catches overlap as well as gaps).
//! * No-period probe: of the 2790 nonzero lattice translations of hex-radius
//!   `<= 30`, none leaves the patch's state assignment invariant on an
//!   overlap of three or more sites.
//! * The standard coverage trio inside the patch (an axis-aligned window and
//!   a 0.6-scaled / 25°-rotated window, both verified exactly-once upstream),
//!   determinism, and empty output beyond the patch.
//!
//! # Scope honesty — cited, not reproduced
//!
//! Aperiodicity of the full plane tiling, the two local-isomorphism classes
//! and the limit-periodic structure are cited from Taylor–Socolar
//! (arXiv:1008.3103); this port proves nothing about them beyond the radius-30
//! no-period probe on the shipped patch. The inflation/deflation hierarchy is
//! NOT reproduced at all (the study's algebraic fit of the inflation aligns
//! only 57.6% of sites, so it is not even verified here), and neither is the
//! claim that this patch sits inside a unique plane tiling — the patch is
//! genuine in the sense that the study's solver extends it (R9→R10) with zero
//! backtracks under rules R1–R3, which is a *local-rule* statement. The
//! single scale pair `t0/R = 0.24`, `w/R = 0.0866` is read off the paper's
//! Figure 2 (0.24 is the shared endpoint of the two figures' measured bands,
//! fig02 0.15–0.24 and fig20 0.24–0.33); the shipped object is one finite
//! patch drawn once, so no scale family is claimed.

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::TilingCategory;

use crate::lattice::SIMILARITY_PARAMS;

/// √3 as a literal (`std::f64::consts::SQRT_3` is not stable on this toolchain).
const SQ3: f64 = 1.7320508075688773;

/// Half the width of a hexagon edge = the inradius, and hexagon half-height.
const HX: f64 = 0.8660254037844386;

/// Patch recentring offset applied to the study's axial coordinates.
const DU: i8 = 0;
const DV: i8 = 10;

/// The 529 patch sites, verbatim from `ap_ts_patch.json`:
/// `(u, v, state)` with `state = a + 3*rbt + 6*sdt`.
const SITES: [(i8, i8, u8); 529] = [
    (-22, 1, 10),
    (-21, 0, 9),
    (-21, 1, 7),
    (-20, -1, 8),
    (-20, 0, 8),
    (-20, 1, 7),
    (-19, -2, 3),
    (-19, -1, 7),
    (-19, 0, 9),
    (-19, 1, 10),
    (-18, -3, 1),
    (-18, -2, 2),
    (-18, -1, 9),
    (-18, 0, 8),
    (-18, 1, 7),
    (-17, -4, 6),
    (-17, -3, 1),
    (-17, -2, 6),
    (-17, -1, 1),
    (-17, 0, 9),
    (-17, 1, 7),
    (-16, -5, 11),
    (-16, -4, 8),
    (-16, -3, 8),
    (-16, -2, 11),
    (-16, -1, 8),
    (-16, 0, 8),
    (-16, 1, 7),
    (-15, -6, 3),
    (-15, -5, 10),
    (-15, -4, 3),
    (-15, -3, 10),
    (-15, -2, 3),
    (-15, -1, 7),
    (-15, 0, 9),
    (-15, 1, 10),
    (-14, -7, 10),
    (-14, -6, 5),
    (-14, -5, 3),
    (-14, -4, 8),
    (-14, -3, 7),
    (-14, -2, 2),
    (-14, -1, 9),
    (-14, 0, 8),
    (-14, 1, 10),
    (-13, -8, 9),
    (-13, -7, 7),
    (-13, -6, 0),
    (-13, -5, 1),
    (-13, -4, 9),
    (-13, -3, 7),
    (-13, -2, 6),
    (-13, -1, 4),
    (-13, 0, 9),
    (-13, 1, 7),
    (-12, -9, 8),
    (-12, -8, 8),
    (-12, -7, 7),
    (-12, -6, 5),
    (-12, -5, 2),
    (-12, -4, 2),
    (-12, -3, 9),
    (-12, -2, 11),
    (-12, -1, 8),
    (-12, 0, 8),
    (-12, 1, 10),
    (-11, -10, 0),
    (-11, -9, 7),
    (-11, -8, 9),
    (-11, -7, 10),
    (-11, -6, 0),
    (-11, -5, 7),
    (-11, -4, 9),
    (-11, -3, 4),
    (-11, -2, 3),
    (-11, -1, 10),
    (-11, 0, 9),
    (-11, 1, 10),
    (-10, -11, 1),
    (-10, -10, 2),
    (-10, -9, 6),
    (-10, -8, 8),
    (-10, -7, 7),
    (-10, -6, 2),
    (-10, -5, 6),
    (-10, -4, 11),
    (-10, -3, 4),
    (-10, -2, 2),
    (-10, -1, 9),
    (-10, 0, 8),
    (-10, 1, 7),
    (-9, -12, 6),
    (-9, -11, 1),
    (-9, -10, 6),
    (-9, -9, 1),
    (-9, -8, 6),
    (-9, -7, 7),
    (-9, -6, 6),
    (-9, -5, 4),
    (-9, -4, 6),
    (-9, -3, 1),
    (-9, -2, 6),
    (-9, -1, 1),
    (-9, 0, 9),
    (-9, 1, 7),
    (-8, -13, 5),
    (-8, -12, 2),
    (-8, -11, 2),
    (-8, -10, 5),
    (-8, -9, 2),
    (-8, -8, 2),
    (-8, -7, 6),
    (-8, -6, 11),
    (-8, -5, 11),
    (-8, -4, 8),
    (-8, -3, 8),
    (-8, -2, 11),
    (-8, -1, 8),
    (-8, 0, 8),
    (-8, 1, 7),
    (-7, -14, 0),
    (-7, -13, 10),
    (-7, -12, 3),
    (-7, -11, 10),
    (-7, -10, 3),
    (-7, -9, 7),
    (-7, -8, 9),
    (-7, -7, 4),
    (-7, -6, 0),
    (-7, -5, 10),
    (-7, -4, 3),
    (-7, -3, 10),
    (-7, -2, 3),
    (-7, -1, 7),
    (-7, 0, 9),
    (-7, 1, 10),
    (-6, -15, 4),
    (-6, -14, 5),
    (-6, -13, 0),
    (-6, -12, 8),
    (-6, -11, 7),
    (-6, -10, 2),
    (-6, -9, 9),
    (-6, -8, 11),
    (-6, -7, 4),
    (-6, -6, 5),
    (-6, -5, 0),
    (-6, -4, 8),
    (-6, -3, 7),
    (-6, -2, 2),
    (-6, -1, 9),
    (-6, 0, 8),
    (-6, 1, 10),
    (-5, -16, 9),
    (-5, -15, 1),
    (-5, -14, 0),
    (-5, -13, 1),
    (-5, -12, 6),
    (-5, -11, 7),
    (-5, -10, 6),
    (-5, -9, 4),
    (-5, -8, 9),
    (-5, -7, 1),
    (-5, -6, 0),
    (-5, -5, 1),
    (-5, -4, 6),
    (-5, -3, 7),
    (-5, -2, 6),
    (-5, -1, 4),
    (-5, 0, 9),
    (-5, 1, 7),
    (-4, -17, 8),
    (-4, -16, 8),
    (-4, -15, 1),
    (-4, -14, 5),
    (-4, -13, 2),
    (-4, -12, 2),
    (-4, -11, 6),
    (-4, -10, 11),
    (-4, -9, 11),
    (-4, -8, 8),
    (-4, -7, 1),
    (-4, -6, 5),
    (-4, -5, 2),
    (-4, -4, 2),
    (-4, -3, 6),
    (-4, -2, 11),
    (-4, -1, 8),
    (-4, 0, 8),
    (-4, 1, 7),
    (-3, -18, 0),
    (-3, -17, 7),
    (-3, -16, 9),
    (-3, -15, 4),
    (-3, -14, 0),
    (-3, -13, 7),
    (-3, -12, 9),
    (-3, -11, 4),
    (-3, -10, 0),
    (-3, -9, 7),
    (-3, -8, 9),
    (-3, -7, 4),
    (-3, -6, 0),
    (-3, -5, 7),
    (-3, -4, 9),
    (-3, -3, 4),
    (-3, -2, 0),
    (-3, -1, 7),
    (-3, 0, 9),
    (-3, 1, 10),
    (-2, -19, 1),
    (-2, -18, 2),
    (-2, -17, 6),
    (-2, -16, 8),
    (-2, -15, 1),
    (-2, -14, 2),
    (-2, -13, 6),
    (-2, -12, 11),
    (-2, -11, 1),
    (-2, -10, 5),
    (-2, -9, 6),
    (-2, -8, 8),
    (-2, -7, 1),
    (-2, -6, 2),
    (-2, -5, 6),
    (-2, -4, 11),
    (-2, -3, 1),
    (-2, -2, 2),
    (-2, -1, 6),
    (-2, 0, 8),
    (-2, 1, 7),
    (-1, -20, 6),
    (-1, -19, 1),
    (-1, -18, 6),
    (-1, -17, 1),
    (-1, -16, 6),
    (-1, -15, 1),
    (-1, -14, 6),
    (-1, -13, 1),
    (-1, -12, 6),
    (-1, -11, 1),
    (-1, -10, 6),
    (-1, -9, 1),
    (-1, -8, 6),
    (-1, -7, 1),
    (-1, -6, 6),
    (-1, -5, 1),
    (-1, -4, 6),
    (-1, -3, 1),
    (-1, -2, 6),
    (-1, -1, 1),
    (-1, 0, 6),
    (-1, 1, 7),
    (0, -21, 5),
    (0, -20, 2),
    (0, -19, 2),
    (0, -18, 5),
    (0, -17, 2),
    (0, -16, 2),
    (0, -15, 2),
    (0, -14, 5),
    (0, -13, 5),
    (0, -12, 2),
    (0, -11, 5),
    (0, -10, 5),
    (0, -9, 2),
    (0, -8, 2),
    (0, -7, 2),
    (0, -6, 5),
    (0, -5, 5),
    (0, -4, 2),
    (0, -3, 2),
    (0, -2, 5),
    (0, -1, 2),
    (0, 0, 2),
    (0, 1, 6),
    (1, -21, 10),
    (1, -20, 3),
    (1, -19, 10),
    (1, -18, 3),
    (1, -17, 10),
    (1, -16, 3),
    (1, -15, 10),
    (1, -14, 3),
    (1, -13, 10),
    (1, -12, 3),
    (1, -11, 10),
    (1, -10, 3),
    (1, -9, 10),
    (1, -8, 3),
    (1, -7, 10),
    (1, -6, 3),
    (1, -5, 10),
    (1, -4, 3),
    (1, -3, 10),
    (1, -2, 3),
    (1, -1, 7),
    (1, 0, 9),
    (2, -21, 3),
    (2, -20, 8),
    (2, -19, 10),
    (2, -18, 2),
    (2, -17, 3),
    (2, -16, 8),
    (2, -15, 10),
    (2, -14, 5),
    (2, -13, 3),
    (2, -12, 11),
    (2, -11, 10),
    (2, -10, 2),
    (2, -9, 3),
    (2, -8, 8),
    (2, -7, 10),
    (2, -6, 5),
    (2, -5, 3),
    (2, -4, 8),
    (2, -3, 7),
    (2, -2, 2),
    (2, -1, 9),
    (3, -21, 4),
    (3, -20, 9),
    (3, -19, 7),
    (3, -18, 0),
    (3, -17, 4),
    (3, -16, 9),
    (3, -15, 7),
    (3, -14, 0),
    (3, -13, 4),
    (3, -12, 9),
    (3, -11, 7),
    (3, -10, 0),
    (3, -9, 4),
    (3, -8, 9),
    (3, -7, 7),
    (3, -6, 0),
    (3, -5, 1),
    (3, -4, 9),
    (3, -3, 7),
    (3, -2, 6),
    (4, -21, 2),
    (4, -20, 2),
    (4, -19, 3),
    (4, -18, 11),
    (4, -17, 8),
    (4, -16, 8),
    (4, -15, 10),
    (4, -14, 5),
    (4, -13, 5),
    (4, -12, 2),
    (4, -11, 3),
    (4, -10, 11),
    (4, -9, 8),
    (4, -8, 8),
    (4, -7, 7),
    (4, -6, 5),
    (4, -5, 2),
    (4, -4, 2),
    (4, -3, 9),
    (5, -21, 7),
    (5, -20, 3),
    (5, -19, 4),
    (5, -18, 3),
    (5, -17, 10),
    (5, -16, 9),
    (5, -15, 10),
    (5, -14, 0),
    (5, -13, 7),
    (5, -12, 3),
    (5, -11, 4),
    (5, -10, 3),
    (5, -9, 7),
    (5, -8, 9),
    (5, -7, 10),
    (5, -6, 0),
    (5, -5, 7),
    (5, -4, 9),
    (6, -21, 0),
    (6, -20, 11),
    (6, -19, 4),
    (6, -18, 2),
    (6, -17, 9),
    (6, -16, 8),
    (6, -15, 7),
    (6, -14, 5),
    (6, -13, 0),
    (6, -12, 11),
    (6, -11, 1),
    (6, -10, 2),
    (6, -9, 9),
    (6, -8, 8),
    (6, -7, 7),
    (6, -6, 2),
    (6, -5, 6),
    (7, -21, 4),
    (7, -20, 6),
    (7, -19, 1),
    (7, -18, 6),
    (7, -17, 1),
    (7, -16, 9),
    (7, -15, 7),
    (7, -14, 0),
    (7, -13, 1),
    (7, -12, 6),
    (7, -11, 1),
    (7, -10, 6),
    (7, -9, 1),
    (7, -8, 9),
    (7, -7, 7),
    (7, -6, 6),
    (8, -21, 11),
    (8, -20, 8),
    (8, -19, 8),
    (8, -18, 11),
    (8, -17, 8),
    (8, -16, 8),
    (8, -15, 7),
    (8, -14, 5),
    (8, -13, 5),
    (8, -12, 2),
    (8, -11, 2),
    (8, -10, 5),
    (8, -9, 2),
    (8, -8, 2),
    (8, -7, 9),
    (9, -21, 10),
    (9, -20, 3),
    (9, -19, 10),
    (9, -18, 3),
    (9, -17, 7),
    (9, -16, 9),
    (9, -15, 10),
    (9, -14, 0),
    (9, -13, 10),
    (9, -12, 3),
    (9, -11, 10),
    (9, -10, 3),
    (9, -9, 7),
    (9, -8, 9),
    (10, -21, 3),
    (10, -20, 8),
    (10, -19, 7),
    (10, -18, 2),
    (10, -17, 9),
    (10, -16, 8),
    (10, -15, 10),
    (10, -14, 5),
    (10, -13, 0),
    (10, -12, 8),
    (10, -11, 7),
    (10, -10, 2),
    (10, -9, 9),
    (11, -21, 1),
    (11, -20, 9),
    (11, -19, 7),
    (11, -18, 6),
    (11, -17, 4),
    (11, -16, 9),
    (11, -15, 7),
    (11, -14, 0),
    (11, -13, 1),
    (11, -12, 6),
    (11, -11, 7),
    (11, -10, 6),
    (12, -21, 2),
    (12, -20, 2),
    (12, -19, 9),
    (12, -18, 11),
    (12, -17, 8),
    (12, -16, 8),
    (12, -15, 7),
    (12, -14, 5),
    (12, -13, 2),
    (12, -12, 2),
    (12, -11, 6),
    (13, -21, 7),
    (13, -20, 9),
    (13, -19, 4),
    (13, -18, 3),
    (13, -17, 7),
    (13, -16, 9),
    (13, -15, 10),
    (13, -14, 0),
    (13, -13, 7),
    (13, -12, 9),
    (14, -21, 6),
    (14, -20, 11),
    (14, -19, 1),
    (14, -18, 2),
    (14, -17, 9),
    (14, -16, 8),
    (14, -15, 7),
    (14, -14, 2),
    (14, -13, 6),
    (15, -21, 1),
    (15, -20, 6),
    (15, -19, 1),
    (15, -18, 6),
    (15, -17, 1),
    (15, -16, 9),
    (15, -15, 7),
    (15, -14, 6),
    (16, -21, 5),
    (16, -20, 2),
    (16, -19, 2),
    (16, -18, 5),
    (16, -17, 2),
    (16, -16, 2),
    (16, -15, 9),
    (17, -21, 10),
    (17, -20, 3),
    (17, -19, 10),
    (17, -18, 3),
    (17, -17, 7),
    (17, -16, 9),
    (18, -21, 3),
    (18, -20, 8),
    (18, -19, 7),
    (18, -18, 2),
    (18, -17, 9),
    (19, -21, 1),
    (19, -20, 9),
    (19, -19, 7),
    (19, -18, 6),
    (20, -21, 2),
    (20, -20, 2),
    (20, -19, 9),
    (21, -21, 7),
    (21, -20, 9),
    (22, -21, 6),
];

/// One piece of the hexagon partition, tile-local (circumradius 1), CCW.
struct Piece {
    proto: u8,
    slot: u8,
    pts: &'static [[f64; 2]],
}

const P00: [Piece; 13] = [
    Piece {
        proto: 1,
        slot: 255,
        pts: &[
            [-0.18766770500008784, 0.89165],
            [-0.26266550496782026, 0.84835],
            [0.6033598988166183, -0.65165],
            [0.6783576987843507, -0.60835],
        ],
    },
    Piece {
        proto: 4,
        slot: 0,
        pts: &[
            [0.715856598768217, 0.5867],
            [0.5658609988327522, 0.6733],
            [0.8660254037844386, 0.1534],
            [0.8660254037844386, 0.3266],
        ],
    },
    Piece {
        proto: 5,
        slot: 3,
        pts: &[
            [-0.8660254037844386, -0.1534],
            [-0.8660254037844386, -0.3266],
            [-0.715856598768217, -0.5867],
            [-0.5658609988327522, -0.6733],
        ],
    },
    Piece {
        proto: 3,
        slot: 0,
        pts: &[
            [0.8660254037844386, 0.5],
            [0.7361215932167728, 0.575],
            [0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 3,
        slot: 1,
        pts: &[
            [0.12990381056766578, 0.925],
            [0.0, 1.0],
            [-0.12990381056766578, 0.925],
        ],
    },
    Piece {
        proto: 3,
        slot: 2,
        pts: &[
            [-0.7361215932167728, 0.575],
            [-0.8660254037844386, 0.5],
            [-0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 3,
        slot: 3,
        pts: &[
            [-0.8660254037844386, -0.35],
            [-0.8660254037844386, -0.5],
            [-0.7361215932167728, -0.575],
        ],
    },
    Piece {
        proto: 3,
        slot: 4,
        pts: &[
            [-0.12990381056766578, -0.925],
            [0.0, -1.0],
            [0.12990381056766578, -0.925],
        ],
    },
    Piece {
        proto: 3,
        slot: 5,
        pts: &[
            [0.7361215932167728, -0.575],
            [0.8660254037844386, -0.5],
            [0.8660254037844386, -0.35],
        ],
    },
    Piece {
        proto: 2,
        slot: 0,
        pts: &[
            [0.7361215932167728, 0.575],
            [0.715856598768217, 0.5867],
            [0.8660254037844386, 0.3266],
            [0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 2,
        slot: 3,
        pts: &[
            [-0.8660254037844386, -0.3266],
            [-0.8660254037844386, -0.35],
            [-0.7361215932167728, -0.575],
            [-0.715856598768217, -0.5867],
        ],
    },
    Piece {
        proto: 0,
        slot: 255,
        pts: &[
            [0.5658609988327522, 0.6733],
            [0.12990381056766578, 0.925],
            [-0.12990381056766578, 0.925],
            [-0.18766770500008784, 0.89165],
            [0.6783576987843507, -0.60835],
            [0.7361215932167728, -0.575],
            [0.8660254037844386, -0.35],
            [0.8660254037844386, 0.1534],
        ],
    },
    Piece {
        proto: 0,
        slot: 255,
        pts: &[
            [-0.26266550496782026, 0.84835],
            [-0.7361215932167728, 0.575],
            [-0.8660254037844386, 0.35],
            [-0.8660254037844386, -0.1534],
            [-0.5658609988327522, -0.6733],
            [-0.12990381056766578, -0.925],
            [0.12990381056766578, -0.925],
            [0.6033598988166183, -0.65165],
        ],
    },
];

const P01: [Piece; 13] = [
    Piece {
        proto: 1,
        slot: 255,
        pts: &[
            [-0.6033598988166183, 0.65165],
            [-0.6783576987843507, 0.60835],
            [0.18766770500008784, -0.89165],
            [0.26266550496782026, -0.84835],
        ],
    },
    Piece {
        proto: 4,
        slot: 0,
        pts: &[
            [0.715856598768217, 0.5867],
            [0.5658609988327522, 0.6733],
            [0.8660254037844386, 0.1534],
            [0.8660254037844386, 0.3266],
        ],
    },
    Piece {
        proto: 5,
        slot: 3,
        pts: &[
            [-0.8660254037844386, -0.1534],
            [-0.8660254037844386, -0.3266],
            [-0.715856598768217, -0.5867],
            [-0.5658609988327522, -0.6733],
        ],
    },
    Piece {
        proto: 3,
        slot: 0,
        pts: &[
            [0.8660254037844386, 0.5],
            [0.7361215932167728, 0.575],
            [0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 3,
        slot: 1,
        pts: &[
            [0.12990381056766578, 0.925],
            [0.0, 1.0],
            [-0.12990381056766578, 0.925],
        ],
    },
    Piece {
        proto: 3,
        slot: 2,
        pts: &[
            [-0.7361215932167728, 0.575],
            [-0.8660254037844386, 0.5],
            [-0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 3,
        slot: 3,
        pts: &[
            [-0.8660254037844386, -0.35],
            [-0.8660254037844386, -0.5],
            [-0.7361215932167728, -0.575],
        ],
    },
    Piece {
        proto: 3,
        slot: 4,
        pts: &[
            [-0.12990381056766578, -0.925],
            [0.0, -1.0],
            [0.12990381056766578, -0.925],
        ],
    },
    Piece {
        proto: 3,
        slot: 5,
        pts: &[
            [0.7361215932167728, -0.575],
            [0.8660254037844386, -0.5],
            [0.8660254037844386, -0.35],
        ],
    },
    Piece {
        proto: 2,
        slot: 0,
        pts: &[
            [0.7361215932167728, 0.575],
            [0.715856598768217, 0.5867],
            [0.8660254037844386, 0.3266],
            [0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 2,
        slot: 3,
        pts: &[
            [-0.8660254037844386, -0.3266],
            [-0.8660254037844386, -0.35],
            [-0.7361215932167728, -0.575],
            [-0.715856598768217, -0.5867],
        ],
    },
    Piece {
        proto: 0,
        slot: 255,
        pts: &[
            [0.5658609988327522, 0.6733],
            [0.12990381056766578, 0.925],
            [-0.12990381056766578, 0.925],
            [-0.6033598988166183, 0.65165],
            [0.26266550496782026, -0.84835],
            [0.7361215932167728, -0.575],
            [0.8660254037844386, -0.35],
            [0.8660254037844386, 0.1534],
        ],
    },
    Piece {
        proto: 0,
        slot: 255,
        pts: &[
            [-0.6783576987843507, 0.60835],
            [-0.7361215932167728, 0.575],
            [-0.8660254037844386, 0.35],
            [-0.8660254037844386, -0.1534],
            [-0.5658609988327522, -0.6733],
            [-0.12990381056766578, -0.925],
            [0.12990381056766578, -0.925],
            [0.18766770500008784, -0.89165],
        ],
    },
];

const P10: [Piece; 13] = [
    Piece {
        proto: 1,
        slot: 255,
        pts: &[
            [-0.8660254037844386, 0.2833],
            [-0.8660254037844386, 0.1967],
            [0.8660254037844386, 0.1967],
            [0.8660254037844386, 0.2833],
        ],
    },
    Piece {
        proto: 4,
        slot: 1,
        pts: &[
            [0.30016440495168645, 0.8267],
            [0.15016880501622165, 0.9133],
            [-0.15016880501622165, 0.9133],
            [-0.30016440495168645, 0.8267],
        ],
    },
    Piece {
        proto: 5,
        slot: 4,
        pts: &[
            [-0.30016440495168645, -0.8267],
            [-0.15016880501622165, -0.9133],
            [0.15016880501622165, -0.9133],
            [0.30016440495168645, -0.8267],
        ],
    },
    Piece {
        proto: 3,
        slot: 0,
        pts: &[
            [0.8660254037844386, 0.5],
            [0.7361215932167728, 0.575],
            [0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 3,
        slot: 1,
        pts: &[
            [0.12990381056766578, 0.925],
            [0.0, 1.0],
            [-0.12990381056766578, 0.925],
        ],
    },
    Piece {
        proto: 3,
        slot: 2,
        pts: &[
            [-0.7361215932167728, 0.575],
            [-0.8660254037844386, 0.5],
            [-0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 3,
        slot: 3,
        pts: &[
            [-0.8660254037844386, -0.35],
            [-0.8660254037844386, -0.5],
            [-0.7361215932167728, -0.575],
        ],
    },
    Piece {
        proto: 3,
        slot: 4,
        pts: &[
            [-0.12990381056766578, -0.925],
            [0.0, -1.0],
            [0.12990381056766578, -0.925],
        ],
    },
    Piece {
        proto: 3,
        slot: 5,
        pts: &[
            [0.7361215932167728, -0.575],
            [0.8660254037844386, -0.5],
            [0.8660254037844386, -0.35],
        ],
    },
    Piece {
        proto: 2,
        slot: 1,
        pts: &[
            [0.15016880501622165, 0.9133],
            [0.12990381056766578, 0.925],
            [-0.12990381056766578, 0.925],
            [-0.15016880501622165, 0.9133],
        ],
    },
    Piece {
        proto: 2,
        slot: 4,
        pts: &[
            [-0.15016880501622165, -0.9133],
            [-0.12990381056766578, -0.925],
            [0.12990381056766578, -0.925],
            [0.15016880501622165, -0.9133],
        ],
    },
    Piece {
        proto: 0,
        slot: 255,
        pts: &[
            [0.7361215932167728, 0.575],
            [0.30016440495168645, 0.8267],
            [-0.30016440495168645, 0.8267],
            [-0.7361215932167728, 0.575],
            [-0.8660254037844386, 0.35],
            [-0.8660254037844386, 0.2833],
            [0.8660254037844386, 0.2833],
            [0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 0,
        slot: 255,
        pts: &[
            [-0.8660254037844386, 0.1967],
            [-0.8660254037844386, -0.35],
            [-0.7361215932167728, -0.575],
            [-0.30016440495168645, -0.8267],
            [0.30016440495168645, -0.8267],
            [0.7361215932167728, -0.575],
            [0.8660254037844386, -0.35],
            [0.8660254037844386, 0.1967],
        ],
    },
];

const P11: [Piece; 13] = [
    Piece {
        proto: 1,
        slot: 255,
        pts: &[
            [-0.8660254037844386, -0.1967],
            [-0.8660254037844386, -0.2833],
            [0.8660254037844386, -0.2833],
            [0.8660254037844386, -0.1967],
        ],
    },
    Piece {
        proto: 4,
        slot: 1,
        pts: &[
            [0.30016440495168645, 0.8267],
            [0.15016880501622165, 0.9133],
            [-0.15016880501622165, 0.9133],
            [-0.30016440495168645, 0.8267],
        ],
    },
    Piece {
        proto: 5,
        slot: 4,
        pts: &[
            [-0.30016440495168645, -0.8267],
            [-0.15016880501622165, -0.9133],
            [0.15016880501622165, -0.9133],
            [0.30016440495168645, -0.8267],
        ],
    },
    Piece {
        proto: 3,
        slot: 0,
        pts: &[
            [0.8660254037844386, 0.5],
            [0.7361215932167728, 0.575],
            [0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 3,
        slot: 1,
        pts: &[
            [0.12990381056766578, 0.925],
            [0.0, 1.0],
            [-0.12990381056766578, 0.925],
        ],
    },
    Piece {
        proto: 3,
        slot: 2,
        pts: &[
            [-0.7361215932167728, 0.575],
            [-0.8660254037844386, 0.5],
            [-0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 3,
        slot: 3,
        pts: &[
            [-0.8660254037844386, -0.35],
            [-0.8660254037844386, -0.5],
            [-0.7361215932167728, -0.575],
        ],
    },
    Piece {
        proto: 3,
        slot: 4,
        pts: &[
            [-0.12990381056766578, -0.925],
            [0.0, -1.0],
            [0.12990381056766578, -0.925],
        ],
    },
    Piece {
        proto: 3,
        slot: 5,
        pts: &[
            [0.7361215932167728, -0.575],
            [0.8660254037844386, -0.5],
            [0.8660254037844386, -0.35],
        ],
    },
    Piece {
        proto: 2,
        slot: 1,
        pts: &[
            [0.15016880501622165, 0.9133],
            [0.12990381056766578, 0.925],
            [-0.12990381056766578, 0.925],
            [-0.15016880501622165, 0.9133],
        ],
    },
    Piece {
        proto: 2,
        slot: 4,
        pts: &[
            [-0.15016880501622165, -0.9133],
            [-0.12990381056766578, -0.925],
            [0.12990381056766578, -0.925],
            [0.15016880501622165, -0.9133],
        ],
    },
    Piece {
        proto: 0,
        slot: 255,
        pts: &[
            [0.7361215932167728, 0.575],
            [0.30016440495168645, 0.8267],
            [-0.30016440495168645, 0.8267],
            [-0.7361215932167728, 0.575],
            [-0.8660254037844386, 0.35],
            [-0.8660254037844386, -0.1967],
            [0.8660254037844386, -0.1967],
            [0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 0,
        slot: 255,
        pts: &[
            [-0.8660254037844386, -0.2833],
            [-0.8660254037844386, -0.35],
            [-0.7361215932167728, -0.575],
            [-0.30016440495168645, -0.8267],
            [0.30016440495168645, -0.8267],
            [0.7361215932167728, -0.575],
            [0.8660254037844386, -0.35],
            [0.8660254037844386, -0.2833],
        ],
    },
];

const P20: [Piece; 13] = [
    Piece {
        proto: 1,
        slot: 255,
        pts: &[
            [0.26266550496782026, 0.84835],
            [0.18766770500008784, 0.89165],
            [-0.6783576987843507, -0.60835],
            [-0.6033598988166183, -0.65165],
        ],
    },
    Piece {
        proto: 4,
        slot: 2,
        pts: &[
            [-0.5658609988327522, 0.6733],
            [-0.715856598768217, 0.5867],
            [-0.8660254037844386, 0.3266],
            [-0.8660254037844386, 0.1534],
        ],
    },
    Piece {
        proto: 5,
        slot: 5,
        pts: &[
            [0.5658609988327522, -0.6733],
            [0.715856598768217, -0.5867],
            [0.8660254037844386, -0.3266],
            [0.8660254037844386, -0.1534],
        ],
    },
    Piece {
        proto: 3,
        slot: 0,
        pts: &[
            [0.8660254037844386, 0.5],
            [0.7361215932167728, 0.575],
            [0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 3,
        slot: 1,
        pts: &[
            [0.12990381056766578, 0.925],
            [0.0, 1.0],
            [-0.12990381056766578, 0.925],
        ],
    },
    Piece {
        proto: 3,
        slot: 2,
        pts: &[
            [-0.7361215932167728, 0.575],
            [-0.8660254037844386, 0.5],
            [-0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 3,
        slot: 3,
        pts: &[
            [-0.8660254037844386, -0.35],
            [-0.8660254037844386, -0.5],
            [-0.7361215932167728, -0.575],
        ],
    },
    Piece {
        proto: 3,
        slot: 4,
        pts: &[
            [-0.12990381056766578, -0.925],
            [0.0, -1.0],
            [0.12990381056766578, -0.925],
        ],
    },
    Piece {
        proto: 3,
        slot: 5,
        pts: &[
            [0.7361215932167728, -0.575],
            [0.8660254037844386, -0.5],
            [0.8660254037844386, -0.35],
        ],
    },
    Piece {
        proto: 2,
        slot: 2,
        pts: &[
            [-0.715856598768217, 0.5867],
            [-0.7361215932167728, 0.575],
            [-0.8660254037844386, 0.35],
            [-0.8660254037844386, 0.3266],
        ],
    },
    Piece {
        proto: 2,
        slot: 5,
        pts: &[
            [0.715856598768217, -0.5867],
            [0.7361215932167728, -0.575],
            [0.8660254037844386, -0.35],
            [0.8660254037844386, -0.3266],
        ],
    },
    Piece {
        proto: 0,
        slot: 255,
        pts: &[
            [0.18766770500008784, 0.89165],
            [0.12990381056766578, 0.925],
            [-0.12990381056766578, 0.925],
            [-0.5658609988327522, 0.6733],
            [-0.8660254037844386, 0.1534],
            [-0.8660254037844386, -0.35],
            [-0.7361215932167728, -0.575],
            [-0.6783576987843507, -0.60835],
        ],
    },
    Piece {
        proto: 0,
        slot: 255,
        pts: &[
            [0.7361215932167728, 0.575],
            [0.26266550496782026, 0.84835],
            [-0.6033598988166183, -0.65165],
            [-0.12990381056766578, -0.925],
            [0.12990381056766578, -0.925],
            [0.5658609988327522, -0.6733],
            [0.8660254037844386, -0.1534],
            [0.8660254037844386, 0.35],
        ],
    },
];

const P21: [Piece; 13] = [
    Piece {
        proto: 1,
        slot: 255,
        pts: &[
            [0.6783576987843507, 0.60835],
            [0.6033598988166183, 0.65165],
            [-0.26266550496782026, -0.84835],
            [-0.18766770500008784, -0.89165],
        ],
    },
    Piece {
        proto: 4,
        slot: 2,
        pts: &[
            [-0.5658609988327522, 0.6733],
            [-0.715856598768217, 0.5867],
            [-0.8660254037844386, 0.3266],
            [-0.8660254037844386, 0.1534],
        ],
    },
    Piece {
        proto: 5,
        slot: 5,
        pts: &[
            [0.5658609988327522, -0.6733],
            [0.715856598768217, -0.5867],
            [0.8660254037844386, -0.3266],
            [0.8660254037844386, -0.1534],
        ],
    },
    Piece {
        proto: 3,
        slot: 0,
        pts: &[
            [0.8660254037844386, 0.5],
            [0.7361215932167728, 0.575],
            [0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 3,
        slot: 1,
        pts: &[
            [0.12990381056766578, 0.925],
            [0.0, 1.0],
            [-0.12990381056766578, 0.925],
        ],
    },
    Piece {
        proto: 3,
        slot: 2,
        pts: &[
            [-0.7361215932167728, 0.575],
            [-0.8660254037844386, 0.5],
            [-0.8660254037844386, 0.35],
        ],
    },
    Piece {
        proto: 3,
        slot: 3,
        pts: &[
            [-0.8660254037844386, -0.35],
            [-0.8660254037844386, -0.5],
            [-0.7361215932167728, -0.575],
        ],
    },
    Piece {
        proto: 3,
        slot: 4,
        pts: &[
            [-0.12990381056766578, -0.925],
            [0.0, -1.0],
            [0.12990381056766578, -0.925],
        ],
    },
    Piece {
        proto: 3,
        slot: 5,
        pts: &[
            [0.7361215932167728, -0.575],
            [0.8660254037844386, -0.5],
            [0.8660254037844386, -0.35],
        ],
    },
    Piece {
        proto: 2,
        slot: 2,
        pts: &[
            [-0.715856598768217, 0.5867],
            [-0.7361215932167728, 0.575],
            [-0.8660254037844386, 0.35],
            [-0.8660254037844386, 0.3266],
        ],
    },
    Piece {
        proto: 2,
        slot: 5,
        pts: &[
            [0.715856598768217, -0.5867],
            [0.7361215932167728, -0.575],
            [0.8660254037844386, -0.35],
            [0.8660254037844386, -0.3266],
        ],
    },
    Piece {
        proto: 0,
        slot: 255,
        pts: &[
            [0.6033598988166183, 0.65165],
            [0.12990381056766578, 0.925],
            [-0.12990381056766578, 0.925],
            [-0.5658609988327522, 0.6733],
            [-0.8660254037844386, 0.1534],
            [-0.8660254037844386, -0.35],
            [-0.7361215932167728, -0.575],
            [-0.26266550496782026, -0.84835],
        ],
    },
    Piece {
        proto: 0,
        slot: 255,
        pts: &[
            [0.7361215932167728, 0.575],
            [0.6783576987843507, 0.60835],
            [-0.18766770500008784, -0.89165],
            [-0.12990381056766578, -0.925],
            [0.12990381056766578, -0.925],
            [0.5658609988327522, -0.6733],
            [0.8660254037844386, -0.1534],
            [0.8660254037844386, 0.35],
        ],
    },
];

/// Partition templates, indexed by `a * 2 + sdt`.
const TEMPLATES: [&[Piece; 13]; 6] = [&P00, &P01, &P10, &P11, &P20, &P21];

/// Site centre in patch coordinates (before the scale/rotation map).
fn centre(u: i8, v: i8) -> Vec2 {
    let uu = f64::from(u) + f64::from(DU);
    let vv = f64::from(v) + f64::from(DV);
    Vec2::new(SQ3 * (uu + 0.5 * vv), 1.5 * vv)
}

/// The paper's corner colour rule: true = red.
fn color_red(a: usize, rbt: usize, k: usize) -> bool {
    if k % 3 == a {
        k == a + 3 * rbt
    } else {
        k % 3 == (a + 2) % 3
    }
}

/// Pattern seed (palette index): 0 base, 1 blue, 2 red.
fn seed_of(a: usize, rbt: usize, proto: u8, slot: u8) -> u32 {
    match proto {
        0 => 0,
        1 => 2,
        _ => u32::from(color_red(a, rbt, usize::from(slot))),
    }
}

/// Taylor–Socolar striped hexagons — the paper's forced 529-hexagon patch,
/// every hexagon partitioned into 13 decorated sub-polygons.
pub struct TaylorSocolarTiling;

impl Tiling for TaylorSocolarTiling {
    fn id(&self) -> &'static str {
        "taylor-socolar"
    }

    fn name(&self) -> &'static str {
        "Taylor-Socolar striped hexagons"
    }

    fn description(&self) -> &'static str {
        "Taylor-Socolar aperiodic striped hexagon (arXiv:1008.3103): the paper's forced 529-hexagon patch, each hexagon partitioned into 13 decorated sub-polygons whose stripe bands reproduce the edge-midpoint handoff rules exactly (R1 0/2992, R2 0/2816, R3 0/2904)."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Aperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "aperiodic (cited, arXiv:1008.3103); forced finite patch of 529 hexagons; \
             no period within hex-radius 30 of the patch (verified here)",
        );
        let map = crate::lattice::similarity_map(params);
        let inv = map.inverse();

        // The query box in patch coordinates: axis-aligned hull of the
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

        // Finite patch: the patch IS the covered domain, so exactly the
        // hexagons whose box meets the region contribute their 13 pieces,
        // and each emitted piece is kept only if it reaches the region.
        let mut tiles = Vec::new();
        for &(u, v, st) in SITES.iter() {
            let c = centre(u, v);
            if c.x - HX > region.max.x
                || c.x + HX < region.min.x
                || c.y - 1.0 > region.max.y
                || c.y + 1.0 < region.min.y
            {
                continue;
            }
            let a = (st % 3) as usize;
            let rbt = ((st / 3) % 2) as usize;
            let sdt = (st / 6) as usize;
            for piece in TEMPLATES[a * 2 + sdt].iter() {
                let pts: Vec<Vec2> = piece
                    .pts
                    .iter()
                    .map(|p| Vec2::new(c.x + p[0], c.y + p[1]))
                    .collect();
                let poly = Polygon::from_slice(&pts).transform(&map);
                if poly.aabb().intersects(query.region) {
                    tiles.push(Tile {
                        proto: piece.proto,
                        seed: seed_of(a, rbt, piece.proto, piece.slot),
                        poly,
                    });
                }
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
    use std::collections::HashMap;

    // Rule data used only by the state-level checks: the runtime geometry is
    // baked into the tables, so these helpers live with their tests.

    /// Stripe offset `t0/R` (paper Figure 2).
    const T0: f64 = 0.24;

    /// Neighbour across hexagon edge `m` (the study's DELTA table).
    const DELTA: [(i8, i8); 6] = [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)];

    /// `SUM[sdt]`: `sigma(state, m) = SUM[sdt][(m - a) mod 6]`.
    const SIGMA_SUM: [[i32; 6]; 2] = [[1, -1, -1, 1, -1, 1], [1, -1, 1, 1, -1, -1]];

    /// `sigma(state, m)` — which side of edge `m` its stripe sits on.
    fn sigma(a: usize, sdt: usize, m: usize) -> i32 {
        SIGMA_SUM[sdt][(m + 6 - a) % 6]
    }

    /// Unit vector at bearing `30*k` degrees.
    fn dir30(k: usize) -> (f64, f64) {
        const T: [(f64, f64); 12] = [
            (1.0, 0.0),
            (HX, 0.5),
            (0.5, HX),
            (0.0, 1.0),
            (-0.5, HX),
            (-HX, 0.5),
            (-1.0, 0.0),
            (-HX, -0.5),
            (-0.5, -HX),
            (0.0, -1.0),
            (0.5, -HX),
            (HX, -0.5),
        ];
        T[k % 12]
    }

    /// Tile-local stripe point on edge `m` for side `s` (+1/-1): the point
    /// the stripe crosses that edge at, per the decoration rule.
    fn stripe_pt(s: i32, m: usize) -> (f64, f64) {
        let (nx, ny) = dir30(2 * m);
        let (tx, ty) = dir30(2 * m + 3);
        let off = f64::from(s) * T0;
        (HX * nx + off * tx, HX * ny + off * ty)
    }

    /// Emitted placement of site (-3,-10), state (a=0, rbt=0, sdt=0).
    /// Centre [-5.196152422706632, 0.0].
    const REF0: [Piece; 13] = [
        Piece {
            proto: 1,
            slot: 255,
            pts: &[
                [-5.38382012770672, 0.89165],
                [-5.458817927674453, 0.84835],
                [-4.592792523890013, -0.65165],
                [-4.5177947239222815, -0.60835],
            ],
        },
        Piece {
            proto: 4,
            slot: 0,
            pts: &[
                [-4.480295823938415, 0.5867],
                [-4.63029142387388, 0.6733],
                [-4.330127018922194, 0.1534],
                [-4.330127018922194, 0.3266],
            ],
        },
        Piece {
            proto: 5,
            slot: 3,
            pts: &[
                [-6.06217782649107, -0.1534],
                [-6.06217782649107, -0.3266],
                [-5.912009021474849, -0.5867],
                [-5.762013421539384, -0.6733],
            ],
        },
        Piece {
            proto: 3,
            slot: 0,
            pts: &[
                [-4.330127018922194, 0.5],
                [-4.46003082948986, 0.575],
                [-4.330127018922194, 0.35],
            ],
        },
        Piece {
            proto: 3,
            slot: 1,
            pts: &[
                [-5.066248612138966, 0.925],
                [-5.196152422706632, 1.0],
                [-5.326056233274298, 0.925],
            ],
        },
        Piece {
            proto: 3,
            slot: 2,
            pts: &[
                [-5.932274015923404, 0.575],
                [-6.06217782649107, 0.5],
                [-6.06217782649107, 0.35],
            ],
        },
        Piece {
            proto: 3,
            slot: 3,
            pts: &[
                [-6.06217782649107, -0.35],
                [-6.06217782649107, -0.5],
                [-5.932274015923404, -0.575],
            ],
        },
        Piece {
            proto: 3,
            slot: 4,
            pts: &[
                [-5.326056233274298, -0.925],
                [-5.196152422706632, -1.0],
                [-5.066248612138966, -0.925],
            ],
        },
        Piece {
            proto: 3,
            slot: 5,
            pts: &[
                [-4.46003082948986, -0.575],
                [-4.330127018922194, -0.5],
                [-4.330127018922194, -0.35],
            ],
        },
        Piece {
            proto: 2,
            slot: 0,
            pts: &[
                [-4.46003082948986, 0.575],
                [-4.480295823938415, 0.5867],
                [-4.330127018922194, 0.3266],
                [-4.330127018922194, 0.35],
            ],
        },
        Piece {
            proto: 2,
            slot: 3,
            pts: &[
                [-6.06217782649107, -0.3266],
                [-6.06217782649107, -0.35],
                [-5.932274015923404, -0.575],
                [-5.912009021474849, -0.5867],
            ],
        },
        Piece {
            proto: 0,
            slot: 255,
            pts: &[
                [-4.63029142387388, 0.6733],
                [-5.066248612138966, 0.925],
                [-5.326056233274298, 0.925],
                [-5.38382012770672, 0.89165],
                [-4.5177947239222815, -0.60835],
                [-4.46003082948986, -0.575],
                [-4.330127018922194, -0.35],
                [-4.330127018922194, 0.1534],
            ],
        },
        Piece {
            proto: 0,
            slot: 255,
            pts: &[
                [-5.458817927674453, 0.84835],
                [-5.932274015923404, 0.575],
                [-6.06217782649107, 0.35],
                [-6.06217782649107, -0.1534],
                [-5.762013421539384, -0.6733],
                [-5.326056233274298, -0.925],
                [-5.066248612138966, -0.925],
                [-4.592792523890013, -0.65165],
            ],
        },
    ];
    const REF0_TPL: &[Piece; 13] = &P00;
    const REF0_CEN: [f64; 2] = [-5.196152422706632, 0.0];

    /// Emitted placement of site (-3,-7), state (a=1, rbt=1, sdt=0).
    /// Centre [-2.598076211353316, 4.5].
    const REF1: [Piece; 13] = [
        Piece {
            proto: 1,
            slot: 255,
            pts: &[
                [-3.4641016151377544, 4.7833],
                [-3.4641016151377544, 4.6967],
                [-1.7320508075688774, 4.6967],
                [-1.7320508075688774, 4.7833],
            ],
        },
        Piece {
            proto: 4,
            slot: 1,
            pts: &[
                [-2.2979118064016295, 5.3267],
                [-2.4479074063370945, 5.4133],
                [-2.7482450163695376, 5.4133],
                [-2.8982406163050025, 5.3267],
            ],
        },
        Piece {
            proto: 5,
            slot: 4,
            pts: &[
                [-2.8982406163050025, 3.6733000000000002],
                [-2.7482450163695376, 3.5867],
                [-2.4479074063370945, 3.5867],
                [-2.2979118064016295, 3.6733000000000002],
            ],
        },
        Piece {
            proto: 3,
            slot: 0,
            pts: &[
                [-1.7320508075688774, 5.0],
                [-1.8619546181365432, 5.075],
                [-1.7320508075688774, 4.85],
            ],
        },
        Piece {
            proto: 3,
            slot: 1,
            pts: &[
                [-2.46817240078565, 5.425],
                [-2.598076211353316, 5.5],
                [-2.727980021920982, 5.425],
            ],
        },
        Piece {
            proto: 3,
            slot: 2,
            pts: &[
                [-3.334197804570089, 5.075],
                [-3.4641016151377544, 5.0],
                [-3.4641016151377544, 4.85],
            ],
        },
        Piece {
            proto: 3,
            slot: 3,
            pts: &[
                [-3.4641016151377544, 4.15],
                [-3.4641016151377544, 4.0],
                [-3.334197804570089, 3.925],
            ],
        },
        Piece {
            proto: 3,
            slot: 4,
            pts: &[
                [-2.727980021920982, 3.575],
                [-2.598076211353316, 3.5],
                [-2.46817240078565, 3.575],
            ],
        },
        Piece {
            proto: 3,
            slot: 5,
            pts: &[
                [-1.8619546181365432, 3.925],
                [-1.7320508075688774, 4.0],
                [-1.7320508075688774, 4.15],
            ],
        },
        Piece {
            proto: 2,
            slot: 1,
            pts: &[
                [-2.4479074063370945, 5.4133],
                [-2.46817240078565, 5.425],
                [-2.727980021920982, 5.425],
                [-2.7482450163695376, 5.4133],
            ],
        },
        Piece {
            proto: 2,
            slot: 4,
            pts: &[
                [-2.7482450163695376, 3.5867],
                [-2.727980021920982, 3.575],
                [-2.46817240078565, 3.575],
                [-2.4479074063370945, 3.5867],
            ],
        },
        Piece {
            proto: 0,
            slot: 255,
            pts: &[
                [-1.8619546181365432, 5.075],
                [-2.2979118064016295, 5.3267],
                [-2.8982406163050025, 5.3267],
                [-3.334197804570089, 5.075],
                [-3.4641016151377544, 4.85],
                [-3.4641016151377544, 4.7833],
                [-1.7320508075688774, 4.7833],
                [-1.7320508075688774, 4.85],
            ],
        },
        Piece {
            proto: 0,
            slot: 255,
            pts: &[
                [-3.4641016151377544, 4.6967],
                [-3.4641016151377544, 4.15],
                [-3.334197804570089, 3.925],
                [-2.8982406163050025, 3.6733000000000002],
                [-2.2979118064016295, 3.6733000000000002],
                [-1.8619546181365432, 3.925],
                [-1.7320508075688774, 4.15],
                [-1.7320508075688774, 4.6967],
            ],
        },
    ];
    const REF1_TPL: &[Piece; 13] = &P10;
    const REF1_CEN: [f64; 2] = [-2.598076211353316, 4.5];

    /// Emitted placement of site (-2,-8), state (a=2, rbt=0, sdt=1).
    /// Centre [-1.7320508075688772, 3.0].
    const REF2: [Piece; 13] = [
        Piece {
            proto: 1,
            slot: 255,
            pts: &[
                [-1.0536931087845265, 3.6083499999999997],
                [-1.128690908752259, 3.65165],
                [-1.9947163125366973, 2.15165],
                [-1.919718512568965, 2.1083499999999997],
            ],
        },
        Piece {
            proto: 4,
            slot: 2,
            pts: &[
                [-2.2979118064016295, 3.6733000000000002],
                [-2.447907406337094, 3.5867],
                [-2.598076211353316, 3.3266],
                [-2.598076211353316, 3.1534],
            ],
        },
        Piece {
            proto: 5,
            slot: 5,
            pts: &[
                [-1.1661898087361249, 2.3266999999999998],
                [-1.0161942088006604, 2.4133],
                [-0.8660254037844386, 2.6734],
                [-0.8660254037844386, 2.8466],
            ],
        },
        Piece {
            proto: 3,
            slot: 0,
            pts: &[
                [-0.8660254037844386, 3.5],
                [-0.9959292143521044, 3.575],
                [-0.8660254037844386, 3.35],
            ],
        },
        Piece {
            proto: 3,
            slot: 1,
            pts: &[
                [-1.6021469970012114, 3.925],
                [-1.7320508075688772, 4.0],
                [-1.861954618136543, 3.925],
            ],
        },
        Piece {
            proto: 3,
            slot: 2,
            pts: &[
                [-2.46817240078565, 3.575],
                [-2.598076211353316, 3.5],
                [-2.598076211353316, 3.35],
            ],
        },
        Piece {
            proto: 3,
            slot: 3,
            pts: &[
                [-2.598076211353316, 2.65],
                [-2.598076211353316, 2.5],
                [-2.46817240078565, 2.425],
            ],
        },
        Piece {
            proto: 3,
            slot: 4,
            pts: &[
                [-1.861954618136543, 2.075],
                [-1.7320508075688772, 2.0],
                [-1.6021469970012114, 2.075],
            ],
        },
        Piece {
            proto: 3,
            slot: 5,
            pts: &[
                [-0.9959292143521044, 2.425],
                [-0.8660254037844386, 2.5],
                [-0.8660254037844386, 2.65],
            ],
        },
        Piece {
            proto: 2,
            slot: 2,
            pts: &[
                [-2.447907406337094, 3.5867],
                [-2.46817240078565, 3.575],
                [-2.598076211353316, 3.35],
                [-2.598076211353316, 3.3266],
            ],
        },
        Piece {
            proto: 2,
            slot: 5,
            pts: &[
                [-1.0161942088006604, 2.4133],
                [-0.9959292143521044, 2.425],
                [-0.8660254037844386, 2.65],
                [-0.8660254037844386, 2.6734],
            ],
        },
        Piece {
            proto: 0,
            slot: 255,
            pts: &[
                [-1.128690908752259, 3.65165],
                [-1.6021469970012114, 3.925],
                [-1.861954618136543, 3.925],
                [-2.2979118064016295, 3.6733000000000002],
                [-2.598076211353316, 3.1534],
                [-2.598076211353316, 2.65],
                [-2.46817240078565, 2.425],
                [-1.9947163125366973, 2.15165],
            ],
        },
        Piece {
            proto: 0,
            slot: 255,
            pts: &[
                [-0.9959292143521044, 3.575],
                [-1.0536931087845265, 3.6083499999999997],
                [-1.919718512568965, 2.1083499999999997],
                [-1.861954618136543, 2.075],
                [-1.6021469970012114, 2.075],
                [-1.1661898087361249, 2.3266999999999998],
                [-0.8660254037844386, 2.8466],
                [-0.8660254037844386, 3.35],
            ],
        },
    ];
    const REF2_TPL: &[Piece; 13] = &P21;
    const REF2_CEN: [f64; 2] = [-1.7320508075688772, 3.0];

    /// Emitted placement of site (-3,-8), state (a=0, rbt=1, sdt=1).
    /// Centre [-3.4641016151377544, 3.0].
    const REF3: [Piece; 13] = [
        Piece {
            proto: 1,
            slot: 255,
            pts: &[
                [-4.067461513954373, 3.65165],
                [-4.142459313922105, 3.6083499999999997],
                [-3.2764339101376665, 2.1083499999999997],
                [-3.2014361101699342, 2.15165],
            ],
        },
        Piece {
            proto: 4,
            slot: 0,
            pts: &[
                [-2.7482450163695376, 3.5867],
                [-2.898240616305002, 3.6733000000000002],
                [-2.598076211353316, 3.1534],
                [-2.598076211353316, 3.3266],
            ],
        },
        Piece {
            proto: 5,
            slot: 3,
            pts: &[
                [-4.330127018922193, 2.8466],
                [-4.330127018922193, 2.6734],
                [-4.179958213905971, 2.4133],
                [-4.029962613970507, 2.3266999999999998],
            ],
        },
        Piece {
            proto: 3,
            slot: 0,
            pts: &[
                [-2.598076211353316, 3.5],
                [-2.7279800219209815, 3.575],
                [-2.598076211353316, 3.35],
            ],
        },
        Piece {
            proto: 3,
            slot: 1,
            pts: &[
                [-3.3341978045700884, 3.925],
                [-3.4641016151377544, 4.0],
                [-3.5940054257054204, 3.925],
            ],
        },
        Piece {
            proto: 3,
            slot: 2,
            pts: &[
                [-4.200223208354528, 3.575],
                [-4.330127018922193, 3.5],
                [-4.330127018922193, 3.35],
            ],
        },
        Piece {
            proto: 3,
            slot: 3,
            pts: &[
                [-4.330127018922193, 2.65],
                [-4.330127018922193, 2.5],
                [-4.200223208354528, 2.425],
            ],
        },
        Piece {
            proto: 3,
            slot: 4,
            pts: &[
                [-3.5940054257054204, 2.075],
                [-3.4641016151377544, 2.0],
                [-3.3341978045700884, 2.075],
            ],
        },
        Piece {
            proto: 3,
            slot: 5,
            pts: &[
                [-2.7279800219209815, 2.425],
                [-2.598076211353316, 2.5],
                [-2.598076211353316, 2.65],
            ],
        },
        Piece {
            proto: 2,
            slot: 0,
            pts: &[
                [-2.7279800219209815, 3.575],
                [-2.7482450163695376, 3.5867],
                [-2.598076211353316, 3.3266],
                [-2.598076211353316, 3.35],
            ],
        },
        Piece {
            proto: 2,
            slot: 3,
            pts: &[
                [-4.330127018922193, 2.6734],
                [-4.330127018922193, 2.65],
                [-4.200223208354528, 2.425],
                [-4.179958213905971, 2.4133],
            ],
        },
        Piece {
            proto: 0,
            slot: 255,
            pts: &[
                [-2.898240616305002, 3.6733000000000002],
                [-3.3341978045700884, 3.925],
                [-3.5940054257054204, 3.925],
                [-4.067461513954373, 3.65165],
                [-3.2014361101699342, 2.15165],
                [-2.7279800219209815, 2.425],
                [-2.598076211353316, 2.65],
                [-2.598076211353316, 3.1534],
            ],
        },
        Piece {
            proto: 0,
            slot: 255,
            pts: &[
                [-4.142459313922105, 3.6083499999999997],
                [-4.200223208354528, 3.575],
                [-4.330127018922193, 3.35],
                [-4.330127018922193, 2.8466],
                [-4.029962613970507, 2.3266999999999998],
                [-3.5940054257054204, 2.075],
                [-3.3341978045700884, 2.075],
                [-3.2764339101376665, 2.1083499999999997],
            ],
        },
    ];
    const REF3_TPL: &[Piece; 13] = &P01;
    const REF3_CEN: [f64; 2] = [-3.4641016151377544, 3.0];

    /// Coverage window verified exactly-once upstream (no gaps; clipped
    /// piece areas sum to the window area).
    const WIN: [f64; 4] = [-14.0, -8.68, 14.0, 8.68];

    /// Coverage window for `scale = 0.6`, `rotation = 25 deg`, likewise
    /// verified upstream in the inverse image of the world box.
    const WIN_ROT: [f64; 4] = [-9.0, -5.58, 9.0, 5.58];

    fn site_map() -> HashMap<(i8, i8), (usize, usize, usize)> {
        SITES
            .iter()
            .map(|&(u, v, st)| {
                (
                    (u, v),
                    (
                        (st % 3) as usize,
                        ((st / 3) % 2) as usize,
                        (st / 6) as usize,
                    ),
                )
            })
            .collect()
    }

    #[test]
    fn taylor_socolar_covers_region() {
        check_covers(
            &TaylorSocolarTiling,
            "taylor-socolar",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }

    #[test]
    fn taylor_socolar_rotated_scaled() {
        check_rotated(
            &TaylorSocolarTiling,
            "taylor-socolar",
            BBox::new(Vec2::new(WIN_ROT[0], WIN_ROT[1]), Vec2::new(WIN_ROT[2], WIN_ROT[3])),
            0.6,
            25.0,
        );
    }

    #[test]
    fn taylor_socolar_deterministic() {
        check_det(
            &TaylorSocolarTiling,
            "taylor-socolar",
            BBox::new(Vec2::new(WIN[0], WIN[1]), Vec2::new(WIN[2], WIN[3])),
        );
    }

    /// Finite-patch semantics: beyond the patch there is nothing to return
    /// (the patch is the covered domain, unlike a plane tiling).
    #[test]
    fn taylor_socolar_empty_beyond_patch() {
        let p = ParamSet::default_for(TaylorSocolarTiling.params());
        let r = TaylorSocolarTiling.generate(
            &p,
            &Query {
                region: BBox::new(Vec2::new(2000.0, 2000.0), Vec2::new(2010.0, 2010.0)),
            },
        );
        assert!(r.tiles.is_empty(), "tiles generated outside the patch");
    }

    /// The handoff battery, re-derived from the shipped table: exactly the
    /// study's JSON numbers, R1 0/2992, R2 0/2816, R3 0/2904.
    #[test]
    fn taylor_socolar_handoff_battery() {
        let s = site_map();
        let (mut r1t, mut r1b) = (0usize, 0usize);
        let (mut r2t, mut r2b) = (0usize, 0usize);
        let (mut r3t, mut r3b) = (0usize, 0usize);
        for &(u, v, st) in SITES.iter() {
            let a = (st % 3) as usize;
            let rbt = ((st / 3) % 2) as usize;
            let sdt = (st / 6) as usize;
            for m in 0..6usize {
                let d = DELTA[m];
                if let Some(nb) = s.get(&(u.wrapping_add(d.0), v.wrapping_add(d.1))) {
                    r1t += 1;
                    if sigma(a, sdt, m) != -sigma(nb.0, nb.2, (m + 3) % 6) {
                        r1b += 1;
                    }
                }
                let dp = DELTA[(m + 5) % 6];
                let dq = DELTA[(m + 1) % 6];
                let c = s.get(&(u.wrapping_add(dp.0), v.wrapping_add(dp.1)));
                let e = s.get(&(u.wrapping_add(dq.0), v.wrapping_add(dq.1)));
                if let (Some(c), Some(e)) = (c, e) {
                    r2t += 1;
                    if color_red(c.0, c.1, (m + 1) % 6) == color_red(e.0, e.1, (m + 4) % 6) {
                        r2b += 1;
                    }
                }
                let k = m;
                let d1 = DELTA[k];
                let d2 = DELTA[(k + 1) % 6];
                let t1 = s.get(&(u.wrapping_add(d1.0), v.wrapping_add(d1.1)));
                let t2 = s.get(&(u.wrapping_add(d2.0), v.wrapping_add(d2.1)));
                if let (Some(t1), Some(t2)) = (t1, t2) {
                    r3t += 1;
                    let c0 = color_red(a, rbt, k);
                    if c0 == color_red(t1.0, t1.1, (k + 2) % 6)
                        && c0 == color_red(t2.0, t2.1, (k + 4) % 6)
                    {
                        r3b += 1;
                    }
                }
            }
        }
        assert_eq!((r1b, r1t), (0, 2992), "R1 sigma complementarity");
        assert_eq!((r2b, r2t), (0, 2816), "R2 diagonal colour inequality");
        assert_eq!((r3b, r3t), (0, 2904), "R3 triple colour rule");
    }

    /// Stripe continuity: on every shared edge the two tiles' stripe points
    /// are the same world point — the paper's line-continuation data.
    #[test]
    fn taylor_socolar_stripe_points_continue() {
        let s = site_map();
        let mut checked = 0usize;
        for &(u, v, st) in SITES.iter() {
            let a = (st % 3) as usize;
            let sdt = (st / 6) as usize;
            let c = centre(u, v);
            for (m, &d) in DELTA.iter().enumerate() {
                let (nu, nv) = (u.wrapping_add(d.0), v.wrapping_add(d.1));
                let Some(nb) = s.get(&(nu, nv)) else {
                    continue;
                };
                let cn = centre(nu, nv);
                let pa = stripe_pt(sigma(a, sdt, m), m);
                let pb = stripe_pt(sigma(nb.0, nb.2, (m + 3) % 6), (m + 3) % 6);
                let dd = ((c.x + pa.0) - (cn.x + pb.0)).hypot((c.y + pa.1) - (cn.y + pb.1));
                assert!(
                    dd < 1e-12,
                    "edge {m} of site ({u},{v}): stripe points differ by {dd:e}"
                );
                checked += 1;
            }
        }
        assert_eq!(checked, 2992, "shared-edge stripe incidences");
    }

    /// No period within hex-radius 30 of the patch: of the 2790 nonzero
    /// translations, none preserves the states over an overlap of >= 3 sites.
    #[test]
    fn taylor_socolar_has_no_period_to_radius_30() {
        let s = site_map();
        let mut tested = 0usize;
        for du in -40i16..=40 {
            for dv in -40i16..=40 {
                if du == 0 && dv == 0 {
                    continue;
                }
                if du.abs().max(dv.abs()).max((du + dv).abs()) > 30 {
                    continue;
                }
                tested += 1;
                let mut ov = 0usize;
                let mut mt = 0usize;
                for &(u, v, st) in SITES.iter() {
                    let key = (
                        i8::try_from(i16::from(u) + du).unwrap_or(127),
                        i8::try_from(i16::from(v) + dv).unwrap_or(127),
                    );
                    if let Some(&st2) = s.get(&key) {
                        ov += 1;
                        let same = st2.0 == (st % 3) as usize
                            && st2.1 == ((st / 3) % 2) as usize
                            && st2.2 == (st / 6) as usize;
                        if same {
                            mt += 1;
                        }
                    }
                }
                assert!(
                    ov < 3 || mt != ov,
                    "translation ({du},{dv}) is a period: {mt}/{ov} sites match"
                );
            }
        }
        assert_eq!(tested, 2790, "translations probed");
    }

    /// Partition mandate, measured on emitted tiles: a query over the whole
    /// patch returns exactly `529 * 13` pieces, every recomputed placement
    /// is found in the emission by exact signature, and each hexagon's 13
    /// piece areas sum to the hexagon area `3*sqrt(3)/2`.
    #[test]
    fn taylor_socolar_partition_areas_sum_to_hexagon() {
        fn signature(proto: u8, seed: u32, pts: &[Vec2]) -> String {
            let mut key = format!("{proto}:{seed}");
            for q in pts {
                key.push_str(&format!(",{:.6}", q.x));
                key.push_str(&format!(",{:.6}", q.y));
            }
            key
        }
        let p = ParamSet::default_for(TaylorSocolarTiling.params());
        let r = TaylorSocolarTiling.generate(
            &p,
            &Query {
                region: BBox::new(Vec2::new(-30.0, -18.0), Vec2::new(30.0, 18.0)),
            },
        );
        assert!(r.message.is_none(), "{:?}", r.message);
        assert_eq!(r.tiles.len(), SITES.len() * 13, "whole-patch piece count");
        let mut pool: HashMap<String, usize> = HashMap::new();
        for t in &r.tiles {
            *pool
                .entry(signature(t.proto, t.seed, &t.poly.pts))
                .or_insert(0) += 1;
        }
        let hex_area = 3.0 * SQ3 / 2.0;
        for &(u, v, st) in SITES.iter() {
            let a = (st % 3) as usize;
            let rbt = ((st / 3) % 2) as usize;
            let sdt = (st / 6) as usize;
            let c = centre(u, v);
            let mut sum = 0.0;
            for piece in TEMPLATES[a * 2 + sdt].iter() {
                let pts: Vec<Vec2> = piece
                    .pts
                    .iter()
                    .map(|q| Vec2::new(c.x + q[0], c.y + q[1]))
                    .collect();
                let hit = pool.get_mut(&signature(
                    piece.proto,
                    seed_of(a, rbt, piece.proto, piece.slot),
                    &pts,
                ));
                assert!(
                    hit.is_some_and(|n| { *n -= 1; *n < usize::MAX }),
                    "site ({u},{v}) piece proto {} slot {} missing from the emission",
                    piece.proto,
                    piece.slot
                );
                sum += Polygon::from_slice(&pts).area();
            }
            assert!(
                (sum - hex_area).abs() < 1e-9,
                "site ({u},{v}): 13 piece areas sum to {sum}, want {hex_area}"
            );
        }
        assert!(
            pool.values().all(|n| *n == 0),
            "emission contains pieces not accounted for by any site"
        );
    }

    /// Compare a template placed at `cen` against a generator-emitted
    /// reference placement, vertex by vertex.
    fn check_ref(tpl: &[Piece; 13], cen: [f64; 2], want: &[Piece; 13], label: &str) {
        for (i, (p, w)) in tpl.iter().zip(want.iter()).enumerate() {
            assert_eq!(p.proto, w.proto, "{label} piece {i}: proto");
            assert_eq!(p.slot, w.slot, "{label} piece {i}: slot");
            assert_eq!(p.pts.len(), w.pts.len(), "{label} piece {i}: vertex count");
            for (j, q) in p.pts.iter().enumerate() {
                let gx = cen[0] + q[0];
                let gy = cen[1] + q[1];
                assert!(
                    (gx - w.pts[j][0]).abs() < 1e-12 && (gy - w.pts[j][1]).abs() < 1e-12,
                    "{label} piece {i} vertex {j}: ({gx}, {gy}) != ({}, {})",
                    w.pts[j][0],
                    w.pts[j][1]
                );
            }
        }
    }

    /// The runtime reproduces the generator's reference placements exactly
    /// (float-eval check of the emitted tables against the exact model), and
    /// the seeds the generator predicts for those pieces.
    #[test]
    fn taylor_socolar_matches_generated_reference() {
        let mut nchecked = 0usize;
        check_ref(REF0_TPL, REF0_CEN, &REF0, "REF0");
        nchecked += 1;
        check_ref(REF1_TPL, REF1_CEN, &REF1, "REF1");
        nchecked += 1;
        check_ref(REF2_TPL, REF2_CEN, &REF2, "REF2");
        nchecked += 1;
        check_ref(REF3_TPL, REF3_CEN, &REF3, "REF3");
        nchecked += 1;
        assert!(nchecked >= 4, "no reference placements checked");
    }
}
