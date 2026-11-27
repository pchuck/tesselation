//! Tangram tiling — the classic 7-piece dissection run as a substitution
//! (inflation λ = √2).
//!
//! The classic tangram set (two large, two small, one medium right
//! isosceles triangles, one square and one parallelogram) has five
//! distinct piece *classes*: small triangle (S), medium triangle (M),
//! large triangle (L), square (Q) and parallelogram (P). The
//! substitution rule (Bielefeld "tangram" entry) inflates every parent
//! by λ = √2 and forces one dissection:
//!
//! - S → { M }
//! - M → { L }
//! - L → { M, Q, P, S, S }
//! - Q → { M, S, S }
//! - P → { M, S, S }
//!
//! All geometry sits on the 45° grid, so every coordinate is an exact
//! integer pair `(a, b) ∈ ℤ²` representing `a + b·√2` in *u-units*
//! (`u = √2/2` classical units, the natural tangram scale): the S area
//! is 1, M 2, L 4, Q 2, P 2 (u²); the pinned seed block is the √2×√2
//! square tiled by exactly two S tiles. Only the float emission ever
//! leaves the lattice.
//!
//! Chirality: the parallelogram is the up-LEFT-slant class `P+`. The
//! pinned seed contains no P tile, only L emits P, and every dissection
//! row is a pure rotation of the parent (no reflection anywhere in the
//! rule), so `P−` is unreachable — every P tile ever emitted is `P+`
//! (the marker test asserts it on every P tile of every generation).
//!
//! The construction is **non-edge-to-edge**: the forced dissections
//! place child vertices in the relative interiors of sibling edges
//! (T-junctions: 0, 0, 0, 6, 10, 18 per generation 0–5, from the
//! pinned seed). What the construction guarantees — and the tests
//! assert exactly on the lattice — is that every generation is a
//! *perfect* tiling: the children tile each parent exactly, the
//! per-proto counts follow the substitution matrix, and the total area
//! law 2^(g+2) (2× area) never changes.
//!
//! Period structure: the finite-depth approximants built from the
//! origin-anchored seed block lattice have periods growing without
//! bound (one period per block-lattice step), so the limit tiling is
//! limit-periodic — the same finite-depth shadow as the chair and
//! armchair.
//!
//! Emission follows the repo's finite-depth approximant convention (see
//! `armchair.rs`): the plane is seeded with √2^d×√2^d seed blocks —
//! each the √2×√2 square (u-units) tiled by its two S tiles (twists
//! 0 and 4, anchors (0,0) and (√2/2, √2/2)) on a lattice anchored
//! through the origin — and every tile is deflated exactly `d` times
//! (deflation steps `k = d, d−1, …, 1`), so the final tiles land on
//! unit natural scale regardless of the view. The seed scale is
//! quantized to powers of √2 measured from the region *extent*, not
//! its position, so panning and zooming within an octave only move the
//! culling window over one fixed global tiling (bit-identical tiles);
//! crossing an octave selects a finer global tiling.
//!
//! Source: the Bielefeld substitution index entry "tangram"
//! (PROPOSED.md §23d).
//!
//! Pattern coloring: five prototile classes, so `proto` is the
//! prototile index (0=S, 1=M, 2=L, 3=Q, 4=P) and `seed` is the tile's
//! twist (45° step 0..=7) + 1 (1..=8), which makes the orientation
//! classes visible in "pattern" mode.

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// √2 in u-units.
const SQRT2: f64 = 1.4142135623730951;

/// An exact lattice point: `x = x0 + x1·√2`, `y = y0 + y1·√2`
/// (u-units). Every coordinate of the construction is such a pair, so
/// refinement, area and T-junction tests are exact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pt {
    pub x0: i64,
    pub x1: i64,
    pub y0: i64,
    pub y1: i64,
}

impl Pt {
    const ZERO: Pt = Pt {
        x0: 0,
        x1: 0,
        y0: 0,
        y1: 0,
    };

    const fn new(x0: i64, x1: i64, y0: i64, y1: i64) -> Self {
        Pt { x0, x1, y0, y1 }
    }

    const fn add(self, o: Pt) -> Pt {
        Pt::new(
            self.x0 + o.x0,
            self.x1 + o.x1,
            self.y0 + o.y0,
            self.y1 + o.y1,
        )
    }

    /// Exact float rendering: `a + b·√2`.
    fn fx(self) -> f64 {
        self.x0 as f64 + self.x1 as f64 * SQRT2
    }

    fn fy(self) -> f64 {
        self.y0 as f64 + self.y1 as f64 * SQRT2
    }

    fn vec2(self) -> Vec2 {
        Vec2::new(self.fx(), self.fy())
    }

    /// `√2^m·self` (exact, m ≥ 0): even `2m` multiplies both
    /// components by 2^m; odd `2m+1` first multiplies by 2^m, then
    /// applies `√2` (`(a, b) → (2b, a)`).
    fn scale_sqrt2_m(self, m: u32) -> Pt {
        let f = 1i64 << (m / 2);
        if m % 2 == 0 {
            Pt::new(self.x0 * f, self.x1 * f, self.y0 * f, self.y1 * f)
        } else {
            Pt::new(2 * self.x1 * f, self.x0 * f, 2 * self.y1 * f, self.y0 * f)
        }
    }

    /// One 45° CCW rotation step (exact):
    /// `((ax, bx), (ay, by)) → ((bx − by, (ax − ay)/2), (bx + by,
    /// (ax + ay)/2))`. Inputs on the 45° grid keep `ax − ay` even, so
    /// the halving is exact — pinned invariant, asserted per step.
    fn rot45_step(self) -> Pt {
        debug_assert_eq!(
            (self.x0 - self.y0) % 2,
            0,
            "rot45 x-parity fail at {self:?}"
        );
        Pt::new(
            self.x1 - self.y1,
            (self.x0 - self.y0) / 2,
            self.x1 + self.y1,
            (self.x0 + self.y0) / 2,
        )
    }

    /// `k` 45° CCW steps: `k/2` 90° steps (each exact) followed by
    /// `k % 2` 45° steps — `k·45°` total.
    fn rot45k(self, k: u8) -> Pt {
        let mut p = self;
        for _ in 0..(k / 2) {
            p = Pt::new(-p.y0, -p.y1, p.x0, p.x1);
        }
        for _ in 0..(k % 2) {
            p = p.rot45_step();
        }
        p
    }
}

#[cfg(test)]
impl Pt {
    /// `self − o` (test-only: growth and the exact T-junction census).
    fn sub(self, o: Pt) -> Pt {
        Pt::new(
            self.x0 - o.x0,
            self.x1 - o.x1,
            self.y0 - o.y0,
            self.y1 - o.y1,
        )
    }

    /// `√2·self` (exact: `√2(a + b√2) = 2b + a√2`, so each pair
    /// `(a, b) → (2b, a)`). Test-only (the growth step).
    fn scale_sqrt2(self) -> Pt {
        Pt::new(2 * self.x1, self.x0, 2 * self.y1, self.y0)
    }

    /// The 2D dot product `self · o` as an exact ℤ[√2] scalar,
    /// emitted as the u-pair `(s0, s1)` of `s0 + s1·√2` — needed only
    /// by the exact T-junction census.
    fn dot2(self, o: Pt) -> (i64, i64) {
        (
            self.x0 * o.x0 + 2 * self.x1 * o.x1 + self.y0 * o.y0 + 2 * self.y1 * o.y1,
            self.x0 * o.x1 + self.x1 * o.x0 + self.y0 * o.y1 + self.y1 * o.y0,
        )
    }

    /// The 2D cross product `self × o` as an exact ℤ[√2] scalar
    /// (u-pair `(s0, s1)`); zero iff the two vectors are collinear.
    fn cross2(self, o: Pt) -> (i64, i64) {
        (
            self.x0 * o.y0 + 2 * self.x1 * o.y1 - self.y0 * o.x0 - 2 * self.y1 * o.x1,
            self.x0 * o.y1 + self.x1 * o.x0 - self.y0 * o.x1 - self.y1 * o.x0,
        )
    }
}

/// The exact sign of the ℤ[√2] scalar `a + b·√2` (−1, 0, +1).
/// Decided case by case so no float is involved: with `a > 0 > b`,
/// `a + b√2 > 0 ⟺ a² > 2b²`; with `a < 0 < b`, `a + b√2 > 0
/// ⟺ 2b² > a²`. (A pair is zero iff both components are 0.)
#[cfg(test)]
fn pair_sign(a: i64, b: i64) -> i8 {
    if a == 0 && b == 0 {
        0
    } else if a > 0 && b >= 0 {
        1
    } else if a < 0 && b <= 0 {
        -1
    } else if a > 0 {
        i8::from(a * a > 2 * b * b)
    } else {
        i8::from(2 * b * b > a * a)
    }
}

/// The five canonical prototile outlines, CCW, in u-pair form
/// (reference corner at the origin; the area-2× values are S 2, M 4,
/// L 8, Q 4, P 4). P is the `P+` class: the up-LEFT-slant parallelogram
/// (see the module docs).
const CANON: [&[Pt]; 5] = [
    &[
        Pt::new(0, 0, 0, 0),
        Pt::new(0, 1, 0, 0),
        Pt::new(0, 0, 0, 1),
    ], // S: small triangle
    &[
        Pt::new(0, 0, 0, 0),
        Pt::new(2, 0, 0, 0),
        Pt::new(0, 0, 2, 0),
    ], // M: medium triangle
    &[
        Pt::new(0, 0, 0, 0),
        Pt::new(0, 2, 0, 0),
        Pt::new(0, 0, 0, 2),
    ], // L: large triangle
    &[
        Pt::new(0, 0, 0, 0),
        Pt::new(0, 1, 0, 0),
        Pt::new(0, 1, 0, 1),
        Pt::new(0, 0, 0, 1),
    ], // Q: square
    &[
        Pt::new(0, 0, 0, 0),
        Pt::new(2, 0, 0, 0),
        Pt::new(1, 0, 1, 0),
        Pt::new(-1, 0, 1, 0),
    ], // P: parallelogram, up-left slant (P+)
];

/// One dissection row: a child prototile `cp` sitting at lattice
/// reference corner `c0` (inside the √2·canonical-parent frame, i.e.
/// the child's reference corner relative to the parent's) with twist
/// delta `d` relative to the parent's twist.
#[derive(Clone, Copy, Debug)]
struct Row {
    cp: u8,
    d: u8,
    c0: Pt,
}

/// The pinned substitution rule (Bielefeld "tangram" entry), per
/// parent prototile.
const DISSECT: [[Row; 5]; 5] = [
    // S:
    [
        Row {
            cp: 1,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
        Row {
            cp: 0,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
        Row {
            cp: 0,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
        Row {
            cp: 0,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
        Row {
            cp: 0,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
    ],
    // M:
    [
        Row {
            cp: 2,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
        Row {
            cp: 0,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
        Row {
            cp: 0,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
        Row {
            cp: 0,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
        Row {
            cp: 0,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
    ],
    // L: M, Q, P, S, S
    [
        Row {
            cp: 1,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
        Row {
            cp: 3,
            d: 1,
            c0: Pt::new(1, 0, 1, 0),
        },
        Row {
            cp: 4,
            d: 0,
            c0: Pt::new(2, 0, 0, 0),
        },
        Row {
            cp: 0,
            d: 3,
            c0: Pt::new(1, 0, 3, 0),
        },
        Row {
            cp: 0,
            d: 5,
            c0: Pt::new(2, 0, 2, 0),
        },
    ],
    // Q: M, S, S
    [
        Row {
            cp: 1,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
        Row {
            cp: 0,
            d: 7,
            c0: Pt::new(1, 0, 1, 0),
        },
        Row {
            cp: 0,
            d: 1,
            c0: Pt::new(1, 0, 1, 0),
        },
        Row {
            cp: 0,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
        Row {
            cp: 0,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
    ],
    // P: M, S, S
    [
        Row {
            cp: 1,
            d: 1,
            c0: Pt::new(0, 0, 0, 0),
        },
        Row {
            cp: 0,
            d: 0,
            c0: Pt::new(0, 1, 0, 0),
        },
        Row {
            cp: 0,
            d: 2,
            c0: Pt::new(0, 1, 0, 0),
        },
        Row {
            cp: 0,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
        Row {
            cp: 0,
            d: 0,
            c0: Pt::new(0, 0, 0, 0),
        },
    ],
];

/// The rows of parent `proto`'s dissection.
fn dissect_rows(proto: u8) -> &'static [Row] {
    debug_assert!((proto as usize) < DISSECT.len());
    let mut n = 0;
    for r in DISSECT[proto as usize] {
        if r.cp == 0 && r.d == 0 && r.c0 == Pt::ZERO {
            break;
        }
        n += 1;
    }
    &DISSECT[proto as usize][..n]
}

/// The pinned seed: the √2×√2 square (u-units) tiled by exactly two S
/// tiles — twist 0 at anchor (0,0) and twist 4 at anchor
/// ((0,1),(0,1)) = (√2/2, √2/2).
const SEED: [(u8, u8, Pt); 2] = [(0, 0, Pt::new(0, 0, 0, 0)), (0, 4, Pt::new(0, 1, 0, 1))];

/// A tangram tile at scale exponent `k`: prototile `proto`, twist
/// `twist` (45° steps 0..=7 about the reference corner), reference
/// corner `anchor` (a u-pair value = the natural c0 coordinate scaled
/// by √2^k), and scale exponent `k` (0 at unit natural scale). The
/// float vertices are `anchor + R_twist(CANON_v)·√2^k`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Tangram {
    proto: u8,
    twist: u8,
    anchor: Pt,
    k: u32,
}

impl Tangram {
    /// Polygon outline in cell space (exact: float only at emission).
    /// Every twist is emitted as the image of the canonical outline
    /// under the twist·45° CCW rotation, scaled by √2^k and re-anchored
    /// to `anchor` — this keeps the vertex correspondence i↔i across
    /// the eight orientation classes: tess-gui's per-prototile
    /// triangulation cache transfers index triples between copies of a
    /// prototile only for vertex lists aligned like this
    /// (per-orientation outline tables would start each class at a
    /// different structural corner and misalign the cached triangles).
    fn poly(&self) -> Polygon {
        let canon = &CANON[self.proto as usize];
        let pts: Vec<Vec2> = canon
            .iter()
            .map(|v| {
                v.rot45k(self.twist)
                    .scale_sqrt2_m(self.k)
                    .add(self.anchor)
                    .vec2()
            })
            .collect();
        Polygon::from_slice(&pts)
    }

    /// The tile's bounding box (float cell space).
    fn aabb(&self) -> BBox {
        let pts = self.poly().pts;
        let mut x0 = f64::INFINITY;
        let mut y0 = f64::INFINITY;
        let mut x1 = f64::NEG_INFINITY;
        let mut y1 = f64::NEG_INFINITY;
        for p in &pts {
            x0 = x0.min(p.x);
            y0 = y0.min(p.y);
            x1 = x1.max(p.x);
            y1 = y1.max(p.y);
        }
        BBox::new(Vec2::new(x0, y0), Vec2::new(x1, y1))
    }

    /// The substitution children of a tile at scale exponent `k`
    /// (k ≥ 1): each child sits at `anchor + √2^(k−1)·R_twist(c0)`
    /// with twist `(twist + d) % 8` and scale exponent `k − 1`. The
    /// children tile the parent set EXACTLY (area-conserving), so every
    /// child lies inside its parent — bounding-box culling against the
    /// region is sound.
    fn children_deflate(&self, k: u32) -> Vec<Tangram> {
        debug_assert_eq!(
            k, self.k,
            "deflation step must match the tile's scale exponent"
        );
        debug_assert!(k >= 1, "deflation needs a positive scale exponent");
        let mut kids = Vec::with_capacity(dissect_rows(self.proto).len());
        for r in dissect_rows(self.proto) {
            kids.push(Tangram {
                proto: r.cp,
                twist: (self.twist + r.d) % 8,
                anchor: self
                    .anchor
                    .add(r.c0.rot45k(self.twist).scale_sqrt2_m(k - 1)),
                k: k - 1,
            });
        }
        kids
    }
}

/// Seed tangrams: the √2^d×√2^d seed blocks (each holding the two
/// `SEED` S tiles at scale √2^d) on the block lattice anchored through
/// the origin, covering `region` in u-unit floats.
fn seed_tangrams(region: &BBox, depth: u32) -> Vec<Tangram> {
    // Block side √2^(depth+1) in u-units: even depth 2m → 2^m·√2;
    // odd depth 2m+1 → 2^(m+1) (both powers of two, exact floats).
    let m = depth / 2;
    let side = if depth % 2 == 0 {
        2f64.powi(m as i32) * SQRT2
    } else {
        2f64.powi(m as i32 + 1)
    };
    let i0 = (region.min.x / side).floor() as i64;
    let i1 = (region.max.x / side).floor() as i64;
    let j0 = (region.min.y / side).floor() as i64;
    let j1 = (region.max.y / side).floor() as i64;
    // Block lattice step √2^(depth+1) as a u-pair: even depth 2m →
    // (0, 2^m); odd depth 2m+1 → (2^(m+1), 0).
    let step = if depth % 2 == 0 {
        Pt::new(0, 1i64 << m, 0, 1i64 << m)
    } else {
        Pt::new(1i64 << (m + 1), 0, 1i64 << (m + 1), 0)
    };
    let mut v = Vec::new();
    for j in j0..=j1 {
        for i in i0..=i1 {
            // Lattice origin i·step (x) + j·step (y), u-pair form.
            let o = Pt::new(step.x0 * i, step.x1 * i, step.y0 * j, step.y1 * j);
            for &(proto, twist, off) in &SEED {
                v.push(Tangram {
                    proto,
                    twist,
                    anchor: o.add(off.scale_sqrt2_m(depth)),
                    k: depth,
                });
            }
        }
    }
    v
}

/// One deflation step: replace every tile by its substitution children
/// (scale √2^k → √2^(k−1)), keeping only those whose bounding box
/// intersects `region`. Sound: the children tile the parent exactly
/// (never leave it), so a final tile intersecting the region has its
/// full ancestor chain present — nothing needed is ever dropped.
fn deflate_step(tiles: &[Tangram], k: u32, region: &BBox) -> Vec<Tangram> {
    let mut out = Vec::with_capacity(tiles.len() * 5);
    for t in tiles {
        for kid in t.children_deflate(k) {
            if kid.aabb().intersects(*region) {
                out.push(kid);
            }
        }
    }
    out
}

/// Tangram tiling (aperiodic, limit-periodic 5-prototile substitution).
pub struct TangramTiling;

impl Tiling for TangramTiling {
    fn id(&self) -> &'static str {
        "tangram"
    }

    fn name(&self) -> &'static str {
        "Tangram"
    }

    fn description(&self) -> &'static str {
        "Aperiodic limit-periodic classic tangram substitution tiling; λ = √2; five prototiles, homochiral."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Nonperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None)
            .with_notes("aperiodic; limit-periodic; 5 prototiles; non-edge-to-edge");
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Region in the tiling's own (u-unit) coordinates: the
        // axis-aligned box around the inverse image of the four world
        // corners.
        let mut rmin = Vec2::new(f64::MAX, f64::MAX);
        let mut rmax = Vec2::new(f64::MIN, f64::MIN);
        for &corner in &query.region.corners() {
            let q = inv.transform(corner);
            rmin.x = rmin.x.min(q.x);
            rmin.y = rmin.y.min(q.y);
            rmax.x = rmax.x.max(q.x);
            rmax.y = rmax.y.max(q.y);
        }
        let region = BBox::new(rmin, rmax);
        if region.area() <= 1e-12 {
            return fail("empty view");
        }
        // Final unit-scale tiles have area at least 1 (the S triangle,
        // the smallest prototile), and the u-unit AABB of a rotated
        // world box is at most ~2× the world area, so this bounds the
        // tile count before any refinement work.
        if region.area() > MAX_TILES as f64 * 2.0 {
            return fail("too many tiles for this view — zoom in");
        }

        // View-independent patch: quantize the seed scale to powers of
        // √2 (one octave per deflation depth) measured from the region
        // *extent* (not its position — panning must not change the
        // octave), and anchor the seed blocks to the lattice through
        // the origin. The emitted tiles are the restriction of one
        // global refined tiling to the query region: panning and
        // resizing only move the culling window; the pattern changes
        // only when the extent crosses a √2 octave (the depth
        // increments and a finer global tiling is selected). Deflating
        // exactly `depth` times lands the final tiles on unit natural
        // scale.
        let extent = (rmax - rmin).length();
        if !extent.is_finite() || extent <= 1e-9 {
            return fail("empty view");
        }
        let depth = ((extent / EXTENT_UNIT).log2().ceil().max(0.0) as u32).min(40);

        // Float culling pad: the seed block lattice is exact, but
        // children are culled by float AABBs; pad against the largest
        // coordinate magnitude so no tile touching the region is ever
        // dropped by a rounding error.
        let m = region
            .corners()
            .iter()
            .map(|c| c.x.abs().max(c.y.abs()))
            .fold(0.0f64, f64::max);
        let pad = (16.0 * m * f64::EPSILON).max(1e-9);
        let padded = BBox::new(
            region.min - Vec2::new(pad, pad),
            region.max + Vec2::new(pad, pad),
        );

        let mut tiles = seed_tangrams(&padded, depth);
        for k in (1..=depth).rev() {
            tiles = deflate_step(&tiles, k, &padded);
        }
        if tiles.is_empty() {
            return fail("empty view");
        }

        // Emit the tiles. `proto` is the prototile index (0=S, 1=M,
        // 2=L, 3=Q, 4=P); `seed` is the tile's twist + 1 (1..=8) — see
        // the module docs — which makes the orientation classes visible
        // in "pattern" mode.
        let mut out = Vec::with_capacity(tiles.len());
        for t in &tiles {
            let poly = t.poly().transform(&map);
            if poly.aabb().intersects(query.region) {
                out.push(Tile {
                    proto: t.proto,
                    seed: t.twist as u32 + 1, level: depth as u8,
                    poly,
                });
            }
        }

        if out.len() > MAX_TILES {
            return fail("too many tiles for this view — zoom in");
        }

        TilingResult {
            tiles: out,
            symmetry: meta,
            periodic: None,
            message: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The 2× natural u² area of each prototile (S 2, M 4, L 8, Q 4,
    /// P 4 — matching the pinned `TWO_AREA` values).
    const TWO_AREA: [i64; 5] = [2, 4, 8, 4, 4];

    /// The pinned growth count sequence, generations 0–19 (iterated
    /// from the det=2 seed (2,0,0,0,0) through the substitution
    /// matrix).
    const PINNED: &[i64; 20] = &[
        2, 2, 2, 10, 18, 26, 66, 138, 242, 506, 1058, 2026, 4050, 8282, 16386, 32586, 65714,
        131258, 261602, 524458,
    ];

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Signed 2× shoelace area of a polygon (float).
    fn signed_area2(pts: &[Vec2]) -> f64 {
        let n = pts.len();
        let mut s = 0.0;
        for i in 0..n {
            let a = pts[i];
            let b = pts[(i + 1) % n];
            s += a.x * b.y - b.x * a.y;
        }
        s
    }

    /// One growth generation, in the natural frame (k = 0): each tile
    /// at c0 `anchor`, twist `twist` emits its children at c0
    /// `√2·anchor + R_twist(c0)`, twist `(twist + d) % 8`.
    fn grow(tiles: &[Tangram]) -> Vec<Tangram> {
        let mut out = Vec::new();
        for t in tiles {
            for r in dissect_rows(t.proto) {
                let a = t.anchor.scale_sqrt2().add(r.c0.rot45k(t.twist));
                out.push(Tangram {
                    proto: r.cp,
                    twist: (t.twist + r.d) % 8,
                    anchor: a,
                    k: 0,
                });
            }
        }
        out
    }

    /// The pinned det=2 seed: two natural-scale S tiles tiling the
    /// √2×√2 square.
    fn seed_tiles() -> Vec<Tangram> {
        SEED.iter()
            .map(|(p, t, off)| Tangram {
                proto: *p,
                twist: *t,
                anchor: *off,
                k: 0,
            })
            .collect()
    }

    /// Ray-cast point-in-polygon on floats, boundary-inclusive (a
    /// point on an edge counts as inside).
    fn point_in_poly_incl(p: &Vec2, poly: &[Vec2]) -> bool {
        let (x, y) = (p.x, p.y);
        let n = poly.len();
        let mut inside = false;
        let mut j = n - 1;
        for i in 0..n {
            let (xi, yi) = (poly[i].x, poly[i].y);
            let (xj, yj) = (poly[j].x, poly[j].y);
            if (yi > y) != (yj > y) {
                let xin = xj + (y - yj) * (xi - xj) / (yi - yj);
                if x < xin {
                    inside = !inside;
                }
            }
            j = i;
        }
        if inside {
            return true;
        }
        // Boundary: on a supporting edge, within the segment.
        for i in 0..n {
            let a = poly[i];
            let b = poly[(i + 1) % n];
            let cross = (b.x - a.x) * (y - a.y) - (b.y - a.y) * (x - a.x);
            if cross.abs() < 1e-9
                && x >= a.x.min(b.x) - 1e-9
                && x <= a.x.max(b.x) + 1e-9
                && y >= a.y.min(b.y) - 1e-9
                && y <= a.y.max(b.y) + 1e-9
            {
                return true;
            }
        }
        false
    }

    /// Is `p` on the left (interior) side of directed edge `a→b` (CCW
    /// clip polygon)?
    fn left_of(p: &Vec2, a: &Vec2, b: &Vec2) -> f64 {
        (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
    }

    /// Intersection point of segment `p1p2` with the line through
    /// `a→b`.
    fn seg_line_inter(p1: &Vec2, p2: &Vec2, a: &Vec2, b: &Vec2) -> Vec2 {
        let d = (p2.x - p1.x) * (b.y - a.y) - (p2.y - p1.y) * (b.x - a.x);
        let t = ((a.x - p1.x) * (b.y - a.y) - (a.y - p1.y) * (b.x - a.x)) / d;
        Vec2::new(p1.x + t * (p2.x - p1.x), p1.y + t * (p2.y - p1.y))
    }

    /// Area of the intersection of two CCW convex polygons
    /// (Sutherland–Hodgman). Returns 0.0 for disjoint or
    /// measure-zero overlap.
    fn poly_overlap_area(subject: &[Vec2], clip: &[Vec2]) -> f64 {
        let mut output = subject.to_vec();
        for i in 0..clip.len() {
            let a = clip[i];
            let b = clip[(i + 1) % clip.len()];
            let input = std::mem::replace(&mut output, Vec::new());
            let mut next = Vec::new();
            for j in 0..input.len() {
                let cur = input[j];
                let prev = input[(j + input.len() - 1) % input.len()];
                let cur_in = left_of(&cur, &a, &b) >= 0.0;
                let prev_in = left_of(&prev, &a, &b) >= 0.0;
                if cur_in {
                    if !prev_in {
                        next.push(seg_line_inter(&prev, &cur, &a, &b));
                    }
                    next.push(cur);
                } else if prev_in {
                    next.push(seg_line_inter(&prev, &cur, &a, &b));
                }
            }
            output = next;
            if output.len() < 3 {
                return 0.0;
            }
        }
        0.5 * signed_area2(&output).abs()
    }

    /// The children of a parent tile as float polygons, in the
    /// parent's *inflated* frame (deflation view, one step): each
    /// child sits at `anchor + √2^(k-1)·R_twist(c0)` with the child at
    /// scale √2^(k-1). For a natural-frame parent (k = 1) this is the
    /// natural dissection.
    fn children_float(parent: &Tangram) -> Vec<Vec<Vec2>> {
        let mut v = Vec::new();
        for r in dissect_rows(parent.proto) {
            let kid = Tangram {
                proto: r.cp,
                twist: (parent.twist + r.d) % 8,
                anchor: parent
                    .anchor
                    .add(r.c0.rot45k(parent.twist).scale_sqrt2_m(parent.k - 1)),
                k: parent.k - 1,
            };
            v.push(kid.poly().pts);
        }
        v
    }

    /// Every prototile × every twist emits a CCW piece of the expected
    /// 2× area and vertex count.
    #[test]
    fn emitted_outlines_are_tangram_pieces() {
        let counts = [3, 3, 3, 4, 4];
        for proto in 0..5u8 {
            for twist in 0..8u8 {
                let t = Tangram {
                    proto,
                    twist,
                    anchor: Pt::ZERO,
                    k: 0,
                };
                let pts = t.poly().pts;
                assert_eq!(
                    pts.len(),
                    counts[proto as usize],
                    "proto {proto}: vertex count"
                );
                let a2 = signed_area2(&pts);
                assert!(
                    (a2 - TWO_AREA[proto as usize] as f64).abs() < 1e-9,
                    "proto {proto} twist {twist}: signed 2×area {a2} != {}",
                    TWO_AREA[proto as usize]
                );
                assert!(a2 > 0.0, "proto {proto} twist {twist}: not CCW");
            }
        }
    }

    /// Emitted vertex lists are rotation-aligned across twists: edge i
    /// of twist `t` is the (t−u)·45° rotation of edge i of twist `u`
    /// (the tess-gui triangulation cache needs index-aligned vertex
    /// lists across orientation classes).
    #[test]
    fn poly_vertex_lists_are_rotation_aligned() {
        for proto in 0..5u8 {
            let base = Tangram {
                proto,
                twist: 0,
                anchor: Pt::ZERO,
                k: 0,
            }
            .poly()
            .pts;
            for twist in 1..8u8 {
                let p = Tangram {
                    proto,
                    twist,
                    anchor: Pt::ZERO,
                    k: 0,
                }
                .poly()
                .pts;
                for i in 0..p.len() {
                    let e0 = base[(i + 1) % p.len()] - base[i];
                    let e = p[(i + 1) % p.len()] - p[i];
                    // R_twist(e0) computed in floats: rotate e0 by
                    // twist·45° CCW.
                    let ang = twist as f64 * std::f64::consts::FRAC_PI_4;
                    let (ca, sa) = (ang.cos(), ang.sin());
                    let rx = e0.x * ca - e0.y * sa;
                    let ry = e0.x * sa + e0.y * ca;
                    assert!(
                        (e.x - rx).abs() < 1e-9 && (e.y - ry).abs() < 1e-9,
                        "proto {proto} twist {twist}: edge {i} not the twist·45° rotation"
                    );
                }
            }
        }
    }

    /// For every prototile and every twist, the children exactly tile
    /// the inflated parent (deflation view): the child areas sum to the
    /// parent's, every child vertex lies in the √2·parent float
    /// polygon, and no two children overlap with positive area.
    #[test]
    fn substitution_tiles_parent_exactly() {
        for proto in 0..5u8 {
            for twist in 0..8u8 {
                let parent = Tangram {
                    proto,
                    twist,
                    anchor: Pt::new(0, 0, 0, 0),
                    k: 1,
                };
                let kids = children_float(&parent);
                // The children tile the parent exactly (area-conserving
                // substitution at this scale): child 2× area sum equals
                // the parent's 2× area.
                let a2k: f64 = kids.iter().map(|k| signed_area2(k)).sum();
                let a2p = signed_area2(&parent.poly().pts);
                assert!(
                    (a2k - a2p).abs() < 1e-9,
                    "proto {proto} twist {twist}: child 2×area sum {a2k} != {a2p}"
                );
                // √2·parent float polygon: the parent's natural-frame
                // outline scaled by √2 about its c0 (anchor 0, k = 1
                // means the natural outline times √2).
                let scaled_parent = CANON[proto as usize]
                    .iter()
                    .map(|v| (v.rot45k(twist).scale_sqrt2_m(1)).vec2())
                    .collect::<Vec<Vec2>>();
                for k in &kids {
                    for v in k {
                        assert!(
                            point_in_poly_incl(v, &scaled_parent),
                            "proto {proto} twist {twist}: child vertex {v:?} outside √2·parent"
                        );
                    }
                }
                // No positive-area overlap between any pair of children.
                for i in 0..kids.len() {
                    for j in (i + 1)..kids.len() {
                        let ov = poly_overlap_area(&kids[i], &kids[j]);
                        assert!(
                            ov < 1e-9,
                            "proto {proto} twist {twist}: children {i}/{j} overlap {ov}"
                        );
                    }
                }
            }
        }
    }

    /// The pinned seed tiles the √2×√2 square exactly: two S triangles,
    /// area 2, no overlap, all four square corners on tile boundaries.
    #[test]
    fn seed_block_tiles_square() {
        let seed = seed_tiles();
        assert_eq!(seed.len(), 2, "seed tile count");
        let polys: Vec<Vec<Vec2>> = seed.iter().map(|t| t.poly().pts.clone()).collect();
        let a2: f64 = polys.iter().map(|p| signed_area2(p)).sum();
        assert!(
            (a2 - 4.0).abs() < 1e-9,
            "seed 2×area {a2} != 4 (√2×√2 square, area 2)"
        );
        let ov = poly_overlap_area(&polys[0], &polys[1]);
        assert!(ov < 1e-9, "seed tiles overlap {ov}");
        // The four square corners lie on tile boundaries.
        let corners = [
            v2(0.0, 0.0),
            v2(SQRT2, 0.0),
            v2(0.0, SQRT2),
            v2(SQRT2, SQRT2),
        ];
        for c in &corners {
            let on_boundary = polys.iter().any(|poly| {
                for i in 0..poly.len() {
                    let a = poly[i];
                    let b = poly[(i + 1) % poly.len()];
                    let cross = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
                    if cross.abs() < 1e-9
                        && c.x >= a.x.min(b.x) - 1e-9
                        && c.x <= a.x.max(b.x) + 1e-9
                        && c.y >= a.y.min(b.y) - 1e-9
                        && c.y <= a.y.max(b.y) + 1e-9
                    {
                        return true;
                    }
                }
                false
            });
            assert!(on_boundary, "seed: corner {c:?} not on any tile boundary");
        }
    }

    /// The P+ chirality marker: undo the twist on a P tile's P+ marker
    /// corner (CANON[4][3]) and it lands exactly on the pinned P+
    /// marker u-pair ((−1,0),(1,0)); every P tile ever emitted is P+
    /// (the P− mirror is unreachable from the pinned seed).
    #[test]
    fn chirality_is_homochiral_p_plus() {
        let marker = CANON[4][3]; // P+ marker vertex
        let mut tiles = seed_tiles();
        for g in 0..6u32 {
            for t in &tiles {
                if t.proto == 4 {
                    // World marker corner, then undo the twist about
                    // the anchor.
                    let world = t.anchor.add(marker.rot45k(t.twist));
                    let undone = world.sub(t.anchor).rot45k((8 - t.twist) % 8);
                    assert_eq!(
                        undone, marker,
                        "gen {g}: P tile marker not the pinned P+ vertex"
                    );
                }
            }
            tiles = grow(&tiles);
        }
    }

    /// Growth from the pinned seed: per-proto counts follow the
    /// substitution matrix, total tile counts follow the pinned
    /// sequence, the total 2× area obeys 2^(g+2), and all five
    /// prototiles appear by generation 5 (the first all-positive c_g).
    #[test]
    fn growth_counts_and_area_law() {
        // Generation 0 = the pinned det=2 seed: two S tiles.
        let mut counts = [2i64, 0, 0, 0, 0];
        for g in 0..20 {
            let c: [i64; 5] = counts.try_into().unwrap();
            let n: i64 = c.iter().sum();
            assert_eq!(
                n, PINNED[g as usize],
                "gen {g}: tile count {n} != {}",
                PINNED[g as usize]
            );
            let a2: i64 = (0..5).map(|p| c[p] * TWO_AREA[p as usize]).sum();
            assert_eq!(
                a2,
                2i64.pow(g as u32 + 2),
                "gen {g}: total 2×area {a2} != 2^{}",
                g + 2
            );
            if g == 5 {
                assert!(
                    (0..5).all(|p| c[p] > 0),
                    "gen 5: not all five prototiles present {c:?}"
                );
            }
            // Next generation via the substitution matrix.
            let mut next = [0i64; 5];
            // A[child][parent]: iterate over parents.
            let matrix: [[i64; 5]; 5] = [
                // child S:
                [0, 0, 2, 2, 2],
                // child M:
                [1, 0, 1, 1, 1],
                // child L:
                [0, 1, 0, 0, 0],
                // child Q:
                [0, 0, 1, 0, 0],
                // child P:
                [0, 0, 1, 0, 0],
            ];
            for parent in 0..5 {
                for child in 0..5 {
                    next[child] += c[parent] * matrix[child][parent];
                }
            }
            counts = next;
        }
    }

    /// T-junctions (tile A's corner in the relative interior of tile B's
    /// edge, A ≠ B) are inherent to the tangram substitution: 0, 0, 0,
    /// 6, 10, 18 per generation 0–5 (the pinned battery counts).
    /// Counted in exact u-pair arithmetic: for every corner of every
    /// tile and every edge of every other tile, collinearity is the
    /// exact cross product, and "strictly interior" means the corner is
    /// distinct from both edge endpoints and on the segment.
    #[test]
    fn t_junctions_are_inherent() {
        let expected = [0, 0, 0, 6, 10, 18];
        let mut tiles = seed_tiles();
        for g in 0..6u32 {
            // Exact u-pair vertices and directed edges of the
            // generation (k = 0: anchor + R_twist(CANON)).
            let polys: Vec<Vec<Pt>> = tiles
                .iter()
                .map(|t| {
                    CANON[t.proto as usize]
                        .iter()
                        .map(|v| v.rot45k(t.twist).add(t.anchor))
                        .collect()
                })
                .collect();
            let n = polys.len();
            let mut triples = 0i64;
            for i in 0..n {
                for c in &polys[i] {
                    for j in 0..n {
                        if i == j {
                            continue;
                        }
                        let m = polys[j].len();
                        for e in 0..m {
                            let a = polys[j][e];
                            let b = polys[j][(e + 1) % m];
                            let ba = b.sub(a);
                            let ca = c.sub(a);
                            let (z0, z1) = ca.cross2(ba);
                            if z0 != 0 || z1 != 0 {
                                continue; // not collinear
                            }
                            // Strictly between a and b: (c−a)·(b−a) > 0
                            // (c ≠ a, same direction) and
                            // (c−b)·(b−a) < 0 (c ≠ b).
                            let da = ca.dot2(ba);
                            let db = c.sub(b).dot2(ba);
                            if pair_sign(da.0, da.1) == 1 && pair_sign(db.0, db.1) == -1 {
                                triples += 1;
                            }
                        }
                    }
                }
            }
            assert_eq!(
                triples, expected[g as usize],
                "gen {g}: T-junctions {triples} != {}",
                expected[g as usize]
            );
            tiles = grow(&tiles);
        }
    }

    /// The patch is view-independent within a √2 octave: growing,
    /// panning, or zooming the view only moves the culling window over
    /// one fixed, origin-anchored refined tiling.
    fn tile_key(tile: &Tile) -> (u32, Vec<u64>) {
        let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
        for p in &tile.poly.pts {
            bits.push(p.x.to_bits());
            bits.push(p.y.to_bits());
        }
        (tile.seed, bits)
    }

    #[test]
    fn patch_is_stable_under_view_changes_within_an_octave() {
        let t = TangramTiling;
        let p = ParamSet::default_for(t.params());
        let base = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
            },
        );
        assert!(base.message.is_none(), "{:?}", base.message);
        let grown = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-8.4, -6.3), v2(8.4, 6.3)),
            },
        );
        let panned = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-7.5, -5.5), v2(8.5, 6.5)),
            },
        );
        let zoomed = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-8.0 / 1.3, -6.0 / 1.3), v2(8.0 / 1.3, 6.0 / 1.3)),
            },
        );
        let base_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            base.tiles.iter().map(tile_key).collect();
        let zoomed_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            zoomed.tiles.iter().map(tile_key).collect();
        let grown_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            grown.tiles.iter().map(tile_key).collect();
        for k in &base_keys {
            assert!(
                grown_keys.contains(k),
                "grow: tile changed when the view grew"
            );
        }
        for k in &zoomed_keys {
            assert!(
                base_keys.contains(k),
                "zoom: tile changed when the view changed"
            );
        }
        let overlap = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-7.45, -5.45), v2(7.95, 5.95)),
            },
        );
        let panned_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            panned.tiles.iter().map(tile_key).collect();
        for tile in &overlap.tiles {
            let k = tile_key(tile);
            assert!(
                base_keys.contains(&k),
                "pan: overlap tile missing/changed in base view"
            );
            assert!(
                panned_keys.contains(&k),
                "pan: overlap tile missing/changed in panned view"
            );
        }
        assert!(
            !overlap.tiles.is_empty(),
            "pan: overlap view produced no tiles"
        );
    }

    /// The octave-anchored deflation lands exactly on unit natural
    /// scale: every emitted tile has the natural 2× area of its
    /// prototile, all five prototiles occur (extent 120 → depth 5, the
    /// first generation where every prototile is present), and `seed`
    /// is a twist in 1..=8.
    #[test]
    fn emitted_tiles_are_unit_scale() {
        let t = TangramTiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-48.0, -36.0), v2(48.0, 36.0)),
            },
        );
        assert!(r.message.is_none(), "{:?}", r.message);
        assert!(!r.tiles.is_empty());
        let mut protos: std::collections::BTreeSet<u8> = std::collections::BTreeSet::new();
        for tile in &r.tiles {
            let a = tile.poly.area();
            let want = TWO_AREA[tile.proto as usize] as f64 * 0.5;
            assert!(
                (a - want).abs() < 1e-9,
                "proto {}: tile area {a} != natural {}",
                tile.proto,
                want
            );
            assert!(
                (1..=8).contains(&tile.seed),
                "seed {} out of range",
                tile.seed
            );
            protos.insert(tile.proto);
        }
        let want_protos: std::collections::BTreeSet<u8> = [0u8, 1, 2, 3, 4].into_iter().collect();
        assert_eq!(protos, want_protos, "want all five prototiles");
    }
}

/// View extent (u-units) at which the seed octave increments.
const EXTENT_UNIT: f64 = 6.0;
