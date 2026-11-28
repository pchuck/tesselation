//! Pregnant chairs — two-prototile √3-substitution tiling (reptiles
//! family).
//!
//! The prototiles are the *large* and *small* "pregnant chair" on the
//! cell grid of √3 (cells √3 wide × 1 tall): the large chair is a
//! 6×2√3 L-block missing its lower-right quadrant (area 9√3), the
//! small chair a 2√3×2 L-block missing its lower-right quadrant
//! (area 3√3). The substitution inflates by λ = √3: the √3-inflated
//! large chair — a two-rectangle frame [0,3√3]×[0,6] ∪
//! [3√3,6√3]×[3,6] — is tiled by ONE large chair plus SIX small
//! chairs, and the √3-inflated small chair is exactly a large chair
//! (3√3·√3 = 9√3). The rule is pinned to the Bielefeld substitution
//! index entry "Pregnant chairs" (two prototiles, E. Amezcua —
//! <https://tilings.math.uni-bielefeld.de/substitution/pregnant-chairs/>);
//! this orientation pin (C1) is the x-mirror of the published
//! Bielefeld figure, and the exact √3 rule above is verified by the
//! spike battery (`pc_rule.py` / `pc_battery.py` / `pc_seed.py`) in
//! exact integer arithmetic over Z[√3].
//!
//! Every coordinate and area in the construction is exact in
//! Z[√3] = i + j·√3, kept as a pair `(i, j)`; floats appear only at
//! render time (`i + j·1.7320508075688772`).
//!
//! The tiling is **non-edge-to-edge**: the dissections land a vertex
//! in the interior of a sibling's edge at every scale — 11
//! T-junctions in the 14-tile depth-1 seed block. The orientation
//! census is chiral: of the eight dihedral classes, only m ∈ {0, 2,
//! 6, 7} occur at any depth ≥ 1; m ∈ {1, 3, 4, 5} never do.
//!
//! Growth law: the per-generation (L, S) leaf-count pair obeys
//! (L, S) → (L + S, 6L) — a single seed gives 7 → 55 → 133 leaves at
//! depths 1, 2, 4 with total area 9·3^d (√3 units); the two-chair
//! seed block (a 9×2√3 rectangle on the period lattice (9, 0),
//! (0, 2√3)) gives 14 → 26 → 266. The depth-g block period is
//! √3^g·(9, 0), √3^g·(0, 2√3) and grows without bound, so the
//! tiling is limit-periodic (aperiodic in the limit).
//!
//! Emission follows the repo's finite-depth approximant convention
//! (cf. `armchair.rs`): the plane is seeded with two-chair seed
//! blocks on the base period lattice (9, 0), (0, 2√3) through the
//! origin and refined exactly g times — each refined block fills its
//! √3^g-scaled region exactly. Unlike the armchair — whose deflation
//! shrinks tiles to unit scale — refinement here *grows* the covered
//! region: the leaves are unit-scale chairs at every depth (large
//! area 9√3, small area 3√3), and the depth only changes the L/S
//! mix (the limit-periodic shadow). The seed depth is quantized to
//! powers of √3 measured from the region *extent* (one octave per
//! refinement, depth capped at 10), so panning and zooming within an
//! octave only move the culling window over one fixed global tiling;
//! crossing an octave selects the coarser L/S mix.
//!
//! Source: the Bielefeld substitution index entry "Pregnant chairs"
//! (two prototiles, E. Amezcua; the entry carries no public paper),
//! <https://tilings.math.uni-bielefeld.de/substitution/pregnant-chairs/>.
//!
//! Pattern coloring: `proto` is the prototile class (0 = large
//! chair, 1 = small chair); `seed` is the chair's dihedral
//! orientation + 1 (1..=8) — see the module docs — which makes the
//! orientation classes visible in "pattern" mode.

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// The exact ring Z[√3] as a pair `(i, j)` meaning `i + j*√3`.
type Z = (i64, i64);

/// A point in Z[√3]² : `(x, y)`, each coordinate a Z[√3] pair.
type ZPt = (Z, Z);

/// The √3 constant used only at render time.
const S3: f64 = 1.7320508075688772;

/// Render a Z[√3] value as a float (exact for all pairs that occur).
fn zf(v: Z) -> f64 {
    v.0 as f64 + v.1 as f64 * S3
}

fn zadd(a: Z, b: Z) -> Z {
    (a.0 + b.0, a.1 + b.1)
}

fn zneg(a: Z) -> Z {
    (-a.0, -a.1)
}

/// Sign of the Z[√3] value `a - b` (exact, no floats): the sign of
/// `p + q√3` from `p = a.0 - b.0`, `q = a.1 - b.1`.
#[cfg(test)]
fn zcmp(a: Z, b: Z) -> i32 {
    let p = a.0 - b.0;
    let q = a.1 - b.1;
    if p == 0 && q == 0 {
        return 0;
    }
    if p >= 0 {
        if q >= 0 {
            return 1;
        }
        let d = p * p - 3 * q * q;
        if d > 0 {
            1
        } else if d < 0 {
            -1
        } else {
            0
        }
    } else {
        if q <= 0 {
            return -1;
        }
        let d = p * p - 3 * q * q;
        if d > 0 {
            -1
        } else if d < 0 {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
fn zmin(a: Z, b: Z) -> Z {
    if zcmp(b, a) < 0 {
        b
    } else {
        a
    }
}

#[cfg(test)]
fn zmax(a: Z, b: Z) -> Z {
    if zcmp(b, a) > 0 {
        b
    } else {
        a
    }
}

/// Multiplication by √3 in Z[√3]: `√3·(i + j√3) = 3j + i√3`.
fn zs3(v: Z) -> Z {
    (3 * v.1, v.0)
}

/// The dihedral image of a Z[√3] point (index m in 0..8 about the
/// origin: rotations m ∈ {0,1,2,3}, the two axis reflections m ∈
/// {4,5}, and the two diagonal reflections m ∈ {6,7}).
fn rm(m: u8, p: ZPt) -> ZPt {
    let (x, y) = p;
    match m {
        0 => (x, y),
        1 => (zneg(y), x),
        2 => (zneg(x), zneg(y)),
        3 => (y, zneg(x)),
        4 => (x, zneg(y)),
        5 => (zneg(x), y),
        6 => (y, x),
        _ => (zneg(y), zneg(x)),
    }
}

/// `COMPOSE[m][n]` = the k with R_k = R_m ∘ R_n (precomputed from
/// the probe point (1, √3): the table is the dihedral closure of
/// R1 with R4 = x-mirror).
const COMPOSE: [[u8; 8]; 8] = [
    [0, 1, 2, 3, 4, 5, 6, 7],
    [1, 2, 3, 0, 6, 7, 5, 4],
    [2, 3, 0, 1, 5, 4, 7, 6],
    [3, 0, 1, 2, 7, 6, 4, 5],
    [4, 7, 5, 6, 0, 2, 3, 1],
    [5, 6, 4, 7, 2, 0, 1, 3],
    [6, 4, 7, 5, 1, 3, 0, 2],
    [7, 5, 6, 4, 3, 1, 2, 0],
];

/// An axis-aligned rectangle in Z[√3]² : `[x0,x1] × [y0,y1]` —
/// test helper (exact-shape battery).
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq)]
struct ZRect {
    x0: Z,
    y0: Z,
    x1: Z,
    y1: Z,
}

/// The large chair K0_L (landscape, BR quadrant missing):
/// [0,3]×[0,2√3] ∪ [3√3,6]×[√3,2√3], CCW 6-vertex outline.
const L_OUTLINE: [ZPt; 6] = [
    ((0, 0), (0, 0)),
    ((3, 0), (0, 0)),
    ((3, 0), (0, 1)),
    ((6, 0), (0, 1)),
    ((6, 0), (0, 2)),
    ((0, 0), (0, 2)),
];

/// The small chair K0_S (landscape, BR quadrant missing):
/// [0,√3]×[0,2] ∪ [√3,2√3]×[1,2], CCW 6-vertex outline.
const S_OUTLINE: [ZPt; 6] = [
    ((0, 0), (0, 0)),
    ((0, 1), (0, 0)),
    ((0, 1), (1, 0)),
    ((0, 2), (1, 0)),
    ((0, 2), (2, 0)),
    ((0, 0), (2, 0)),
];

/// K0_L as its two-rectangle decomposition.
#[cfg(test)]
const L_BASE: [ZRect; 2] = [
    ZRect {
        x0: (0, 0),
        y0: (0, 0),
        x1: (3, 0),
        y1: (0, 2),
    },
    ZRect {
        x0: (3, 0),
        y0: (0, 1),
        x1: (6, 0),
        y1: (0, 2),
    },
];

/// K0_S as its two-rectangle decomposition (test-only: the exact
/// shape rectangles are used by the battery; the frame culling and
/// emission never need the small chair's shape boxes).
#[cfg(test)]
const S_BASE: [ZRect; 2] = [
    ZRect {
        x0: (0, 0),
        y0: (0, 0),
        x1: (0, 1),
        y1: (2, 0),
    },
    ZRect {
        x0: (0, 1),
        y0: (1, 0),
        x1: (0, 2),
        y1: (2, 0),
    },
];

/// √3·K0_L — the region a large chair's children tile:
/// [0,3√3]×[0,6] ∪ [3√3,6√3]×[3,6]. (A small chair's frame is
/// L_BASE, since √3·K0_S = K0_L exactly.)
#[cfg(test)]
const L_FRAME: [ZRect; 2] = [
    ZRect {
        x0: (0, 0),
        y0: (0, 0),
        x1: (0, 3),
        y1: (6, 0),
    },
    ZRect {
        x0: (0, 3),
        y0: (3, 0),
        x1: (0, 6),
        y1: (6, 0),
    },
];

/// The canonical large-rule children of the m=0 large chair at the
/// origin: `(kind, m, croot_x, croot_y)` with kind 0 = L, 1 = S and
/// croot the child's bounding-box lower-left in the canonical
/// parent's frame (Z[√3] pairs).
const CANON_CHILDREN: [(u8, u8, Z, Z); 7] = [
    (0, 7, (0, 4), (6, 0)),
    (1, 2, (0, 2), (2, 0)),
    (1, 0, (0, 0), (1, 0)),
    (1, 2, (0, 2), (5, 0)),
    (1, 0, (0, 0), (4, 0)),
    (1, 2, (0, 6), (5, 0)),
    (1, 0, (0, 4), (4, 0)),
];

/// The seed block: two large chairs tiling the 9×2√3 rectangle
/// (the block lattice is (9, 0), (0, 2√3) through the origin).
const SEED2: [Chair; 2] = [
    Chair {
        kind: 0,
        m: 0,
        tx: (0, 0),
        ty: (0, 0),
    },
    Chair {
        kind: 0,
        m: 2,
        tx: (9, 0),
        ty: (0, 2),
    },
];

/// View extent (cell units) at which the seed octave increments
/// (one octave = a factor of √3 in extent; the base period's x-span).
const EXTENT_UNIT: f64 = 9.0;

/// log2(√3) — the depth quantizes extent/EXTENT_UNIT to powers of √3.
const LOG2_SQRT3: f64 = 0.7924812503605781;

/// The depth cap (i64 pair components stay far from overflow at 10;
/// zs3^10 = ·243 on the pair components).
const DEPTH_CAP: u32 = 10;

/// A chair on the exact lattice: prototile class `kind` (0 = large
/// chair, 1 = small chair), dihedral orientation `m` (0..8),
/// bounding-box translation `T` (Z[√3] pair per coordinate). The
/// shape is `R(m)(K0_kind) + T` — every emitted chair is unit scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Chair {
    kind: u8,
    m: u8,
    tx: Z,
    ty: Z,
}

/// The `base` two rectangles under R(m), shifted by (tx, ty) — the
/// bounding box of the dihedral image of each rectangle (rectangles
/// map to rectangles under the dihedral group).
#[cfg(test)]
fn map_rects(m: u8, base: &[ZRect; 2], tx: Z, ty: Z) -> [ZRect; 2] {
    [0, 1].map(|k| {
        let r = &base[k];
        let cs = [
            rm(m, (r.x0, r.y0)),
            rm(m, (r.x0, r.y1)),
            rm(m, (r.x1, r.y0)),
            rm(m, (r.x1, r.y1)),
        ];
        let x0 = zmin(zmin(cs[0].0, cs[1].0), zmin(cs[2].0, cs[3].0));
        let x1 = zmax(zmax(cs[0].0, cs[1].0), zmax(cs[2].0, cs[3].0));
        let y0 = zmin(zmin(cs[0].1, cs[1].1), zmin(cs[2].1, cs[3].1));
        let y1 = zmax(zmax(cs[0].1, cs[1].1), zmax(cs[2].1, cs[3].1));
        ZRect {
            x0: zadd(x0, tx),
            y0: zadd(y0, ty),
            x1: zadd(x1, tx),
            y1: zadd(y1, ty),
        }
    })
}

impl Chair {
    /// The chair's shape as its two-rectangle decomposition (exact
    /// Z[√3] rectangles) — test helper.
    #[cfg(test)]
    fn rects(&self) -> [ZRect; 2] {
        let base = if self.kind == 0 { &L_BASE } else { &S_BASE };
        map_rects(self.m, base, self.tx, self.ty)
    }

    /// The region this chair's children exactly tile:
    /// `R(m)(√3·K0_kind) + √3·T` (a large chair's two-rectangle
    /// frame; a small chair's frame is the large chair's, since
    /// √3·K0_S = K0_L exactly).
    #[cfg(test)]
    fn frame_rects(&self) -> [ZRect; 2] {
        let base = if self.kind == 0 { &L_FRAME } else { &L_BASE };
        let t3 = (zs3(self.tx), zs3(self.ty));
        map_rects(self.m, base, t3.0, t3.1)
    }

    /// The substitution children: seven for a large chair (one large
    /// at `compose(m, cm)` + six small, from the canonical C1 list),
    /// one for a small chair (a large chair of the same orientation,
    /// since √3·K0_S = K0_L exactly).
    fn children(&self) -> Vec<Chair> {
        let t3 = (zs3(self.tx), zs3(self.ty));
        if self.kind == 0 {
            CANON_CHILDREN
                .iter()
                .map(|&(ck, cm, cx, cy)| {
                    let (rx, ry) = rm(self.m, (cx, cy));
                    Chair {
                        kind: ck,
                        m: COMPOSE[self.m as usize][cm as usize],
                        tx: zadd(rx, t3.0),
                        ty: zadd(ry, t3.1),
                    }
                })
                .collect()
        } else {
            vec![Chair {
                kind: 0,
                m: self.m,
                tx: t3.0,
                ty: t3.1,
            }]
        }
    }

    /// The polygon outline in cell space: the canonical 6-vertex
    /// outline under the 90°/reflection `R(m)`, translated by `T`,
    /// rendered with `i + j·√3` (exact for all pairs that occur).
    /// Every orientation class emits the dihedral image of the SAME
    /// outline, so the vertex correspondence i↔i transfers across
    /// orientation classes (tess-gui's per-prototile triangulation
    /// cache relies on that alignment).
    fn poly(&self) -> Polygon {
        let outline = if self.kind == 0 {
            &L_OUTLINE
        } else {
            &S_OUTLINE
        };
        let pts: Vec<Vec2> = outline
            .iter()
            .map(|&(vx, vy)| {
                let (rx, ry) = rm(self.m, (vx, vy));
                Vec2::new(zf(zadd(rx, self.tx)), zf(zadd(ry, self.ty)))
            })
            .collect();
        Polygon::from_slice(&pts)
    }
}

/// `n`-fold ring scaling in Z[√3]: `√3^n · v`.
fn zs3n(v: Z, n: u32) -> Z {
    let mut w = v;
    for _ in 0..n {
        w = zs3(w);
    }
    w
}

/// Seed chairs: the two-chair blocks on the BASE period lattice
/// (9, 0), (0, 2√3) through the origin, covering `region` (with a
/// ±1 block margin). Block (i, j) holds the depth-0 SEED2 chairs
/// at the base-lattice origin (9i, 0), (0, 2j) — real 9i, 2j√3;
/// refining that block exactly `depth` times fills the nominal
/// block region [i·sw, (i+1)·sw] × [j·sh, (j+1)·sh] EXACTLY, where
/// sw = 9√3^depth and sh = 2√3^(depth+1) are the real `zf` of the
/// depth-`depth` ring-scaled period vectors (parity-agnostic: the
/// pair's real value is the block's dimension at every depth). The
/// refined leaves are a restriction of one global limit-periodic
/// tiling, so panning only moves which blocks are seeded.
fn seed_chairs(region: &BBox, depth: u32) -> Vec<Chair> {
    let sw = zf(zs3n((9, 0), depth));
    let sh = zf(zs3n((0, 2), depth));
    // √3 coordinates round in f64, so a block index exactly on the
    // region border can be off by one; the ±1 block margin covers
    // it (the per-step culling and the final emission cull
    // re-verify).
    let i0 = (region.min.x / sw).floor() as i64 - 1;
    let i1 = (region.max.x / sw).floor() as i64 + 1;
    let j0 = (region.min.y / sh).floor() as i64 - 1;
    let j1 = (region.max.y / sh).floor() as i64 + 1;
    let mut v = Vec::new();
    for j in j0..=j1 {
        for i in i0..=i1 {
            // Block origin in Z[√3]²: (9i, 0), (0, 2j) — the
            // SEED2 offsets stay depth-0.
            let ox: Z = (9 * i, 0);
            let oy: Z = (0, 2 * j);
            for c in &SEED2 {
                v.push(Chair {
                    kind: c.kind,
                    m: c.m,
                    tx: zadd(ox, c.tx),
                    ty: zadd(oy, c.ty),
                });
            }
        }
    }
    v
}

/// The culling box for a chair's descendants after `r` more
/// refinement steps: a proven orientation-agnostic superset. The
/// rule tiles each frame exactly, so `r` further refinements of
/// this chair fill exactly √3^`r`·shape(chair), and the shape is
/// `R(m)(K0) + T` with K0_L spanning 6 × 2√3 and K0_S spanning
/// 2√3 × 2, every dihedral image inside T + [−6, 6]² (large) or
/// T + [−2√3, 2√3]² (small: the portrait classes m ∈ {2, 6} put the
/// span-2√3 axis on x). Hence half-extents 6√3^r and 2√3^(r+1).
/// Review A8: the small branch used 2√3^r, which under-covered
/// portrait small chairs by √3 — the cull then dropped the leaves
/// that reach left of their anchor, and the app showed exactly that
/// strip of holes at region left edges. Scaling that loose shape box
/// about the ring-scaled translation anchors the descendants' union
/// exactly.
fn cover_box(c: &Chair, r: u32) -> BBox {
    let half = if c.kind == 0 {
        zf(zs3n((6, 0), r)) // 6√3^r
    } else {
        zf(zs3n((0, 2), r)) // 2√3^(r+1)
    };
    let ax = zf(zs3n(c.tx, r));
    let ay = zf(zs3n(c.ty, r));
    BBox::new(
        Vec2::new(ax - half, ay - half),
        Vec2::new(ax + half, ay + half),
    )
}

/// One refinement step: replace every chair by its substitution
/// children, keeping only those whose descendants (after `r` more
/// refinement steps) can reach `region`. Sound: the cover box is a
/// proven superset of the descendants' union, so a final leaf
/// intersecting the region keeps its full ancestor chain — nothing
/// needed is ever dropped; the final emission cull trims the rest.
fn refine_step(chairs: &[Chair], region: &BBox, r: u32) -> Vec<Chair> {
    let mut out = Vec::with_capacity(chairs.len() * 7);
    for c in chairs {
        for kid in c.children() {
            if cover_box(&kid, r).intersects(*region) {
                out.push(kid);
            }
        }
    }
    out
}

/// Pregnant-chairs tiling (aperiodic, limit-periodic two-prototile
/// √3-substitution).
pub struct PregnantChairTiling;

impl Tiling for PregnantChairTiling {
    fn id(&self) -> &'static str {
        "pregnant-chair"
    }

    fn name(&self) -> &'static str {
        "Pregnant Chairs"
    }

    fn description(&self) -> &'static str {
        "Aperiodic limit-periodic √3-substitution tiling of two chair prototiles (large and small); λ = √3; non-edge-to-edge (T-junctions at every scale)."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Nonperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "aperiodic; limit-periodic; two prototiles (large and small chair); \
             chiral orientations; non-edge-to-edge",
        );
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Region in the tiling's own (cell) coordinates: the
        // axis-aligned box around the inverse image of the four
        // world corners.
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
        // Final chairs have cell-space areas 9√3 (large) and 3√3
        // (small), and the cell-space AABB of a rotated world box is
        // at most ~2× the world area, so this bounds the emitted tile
        // count below MAX_TILES before any refinement work; the
        // final `tiles.len()` check enforces the bound regardless.
        if region.area() > MAX_TILES as f64 * 6.0 {
            return fail("too many tiles for this view — zoom in");
        }

        // View-independent patch: quantize the substitution depth to
        // powers of √3 (one octave per refinement) measured from the
        // region *extent* (not its position — panning must not change
        // the depth), and anchor the seed blocks to the base period
        // lattice (9, 0), (0, 2√3) through the origin — refining a
        // block `depth` times fills its √3^depth-scaled region
        // exactly. The emitted tiles are the restriction of one
        // global refined tiling to the query region: panning and
        // resizing only move the culling window; the L/S mix (the
        // limit-periodic shadow) changes only when the extent
        // crosses a √3 octave.
        let extent = (rmax - rmin).length();
        if !extent.is_finite() || extent <= 1e-9 {
            return fail("empty view");
        }
        let depth = ((extent / EXTENT_UNIT).log2() / LOG2_SQRT3).ceil().max(0.0) as u32;
        let depth = depth.min(DEPTH_CAP);

        let mut chairs = seed_chairs(&region, depth);
        for s in 0..depth {
            // `r` refinement steps remain after this one: cull each
            // child by the proven superset of its descendants' union.
            let r = depth - s - 1;
            chairs = refine_step(&chairs, &region, r);
        }
        if chairs.is_empty() {
            return fail("empty view");
        }

        // Emit the chairs as tiles: `proto` is the prototile class
        // (0 = large, 1 = small); `seed` is the chair's orientation
        // + 1 (1..=8) — see the module docs.
        let mut tiles = Vec::with_capacity(chairs.len());
        for c in &chairs {
            let poly = c.poly().transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile { family: tess_core::tiling::FamilyTag::None, reflected: false,
                    proto: c.kind,
                    seed: c.m as u32 + 1, level: depth as u8,
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

#[cfg(test)]
mod pregnant_chair_tests {
    use super::*;
    use crate::lattice::SCALE_ID;
    use crate::tests::{check_coverage, check_det, check_rotated};
    use std::collections::BTreeSet;
    use tess_core::params::ParamValue;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn zsub(a: Z, b: Z) -> Z {
        (a.0 - b.0, a.1 - b.1)
    }

    fn zsum(vals: impl IntoIterator<Item = Z>) -> Z {
        let mut t = (0, 0);
        for v in vals {
            t = zadd(t, v);
        }
        t
    }

    /// Exact ordering on Z[√3] values as a total `Ordering` (the
    /// exact sign test — no floats).
    fn zord(a: Z, b: Z) -> std::cmp::Ordering {
        match zcmp(a, b) {
            -1 => std::cmp::Ordering::Less,
            1 => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    }

    fn grow1(tiles: &[Chair]) -> Vec<Chair> {
        let mut out = Vec::new();
        for t in tiles {
            out.extend(t.children());
        }
        out
    }

    fn rect_overlap(a: &ZRect, b: &ZRect) -> bool {
        zcmp(a.x0, b.x1) < 0 && zcmp(b.x0, a.x1) < 0 && zcmp(a.y0, b.y1) < 0 && zcmp(b.y0, a.y1) < 0
    }

    /// Ring product in Z[√3] pairs: the exact area of an
    /// axis-aligned rectangle.
    fn zarea(r: &ZRect) -> Z {
        let dx = zsub(r.x1, r.x0);
        let dy = zsub(r.y1, r.y0);
        (dx.0 * dy.0 + 3 * dx.1 * dy.1, dx.0 * dy.1 + dx.1 * dy.0)
    }

    /// The exact √3-multiple area (√3 coefficient) of an
    /// axis-aligned rectangle; asserts the area is a pure √3
    /// multiple (as in the spike's `area_rect`).
    fn area_coeff(r: &ZRect) -> i64 {
        let dx = zsub(r.x1, r.x0);
        let dy = zsub(r.y1, r.y0);
        let a0 = dx.0 * dy.0 + 3 * dx.1 * dy.1;
        let a1 = dx.0 * dy.1 + dx.1 * dy.0;
        assert_eq!(a0, 0, "area not a pure √3 multiple: {r:?}");
        a1
    }

    fn isect_area(a: &ZRect, b: &ZRect) -> Z {
        let x0 = zmax(a.x0, b.x0);
        let x1 = zmin(a.x1, b.x1);
        let y0 = zmax(a.y0, b.y0);
        let y1 = zmin(a.y1, b.y1);
        if zcmp(x0, x1) >= 0 || zcmp(y0, y1) >= 0 {
            return (0, 0);
        }
        zarea(&ZRect { x0, y0, x1, y1 })
    }

    /// `r` is contained in the union of the disjoint frame
    /// rectangles `frames` (exact: a rect is in the union iff its
    /// area equals its sum of intersection areas with the frames).
    fn inside_union(r: &ZRect, frames: &[ZRect]) -> bool {
        let u = zsum(frames.iter().map(|f| isect_area(r, f)));
        zcmp(u, zarea(r)) == 0
    }

    /// Merge Z[√3] open intervals (a, b) — exact, numeric order.
    fn merge_ivs(mut ivs: Vec<(Z, Z)>) -> Vec<(Z, Z)> {
        if ivs.is_empty() {
            return ivs;
        }
        ivs.sort_by(|&(a, _), &(b, _)| zord(a, b));
        let mut out = vec![ivs[0]];
        for (a, b) in ivs.iter().skip(1) {
            let (a, b) = (*a, *b);
            let last_end = out.last().unwrap().1;
            if zcmp(a, last_end) <= 0 {
                if zcmp(b, last_end) > 0 {
                    out.last_mut().unwrap().1 = b;
                }
            } else {
                out.push((a, b));
            }
        }
        out
    }

    /// Exact slab sweep (the spike's `slab_check`): the child
    /// rectangles tile the frame region exactly — equal total area,
    /// no overlap (sum of lengths = union length on every slab), no
    /// gap.
    fn slab_check(child_rects: &[ZRect], frame_rects: &[ZRect]) -> bool {
        let fa: i64 = frame_rects.iter().map(area_coeff).sum();
        let ua: i64 = child_rects.iter().map(area_coeff).sum();
        if ua != fa {
            return false;
        }
        let mut xs: Vec<Z> = child_rects
            .iter()
            .chain(frame_rects.iter())
            .flat_map(|r| [r.x0, r.x1])
            .collect();
        xs.sort_by(|a, b| zord(*a, *b));
        xs.dedup_by(|a, b| zcmp(*a, *b) == 0);
        for w in xs.windows(2) {
            let (a, b) = (w[0], w[1]);
            if zcmp(a, b) == 0 {
                continue;
            }
            let fiv = merge_ivs(
                frame_rects
                    .iter()
                    .filter(|fr| zcmp(fr.x0, b) < 0 && zcmp(a, fr.x1) < 0)
                    .map(|fr| (fr.y0, fr.y1))
                    .collect(),
            );
            if fiv.is_empty() {
                return false;
            }
            let civ: Vec<(Z, Z)> = child_rects
                .iter()
                .filter(|r| zcmp(r.x0, b) < 0 && zcmp(a, r.x1) < 0)
                .map(|r| (r.y0, r.y1))
                .collect();
            let cm = merge_ivs(civ.clone());
            let ov = zsum(civ.iter().map(|&(aa, bb)| zsub(bb, aa)));
            let un = zsum(cm.iter().map(|&(aa, bb)| zsub(bb, aa)));
            if zcmp(ov, un) != 0 {
                return false;
            }
            let fu = zsum(fiv.iter().map(|&(aa, bb)| zsub(bb, aa)));
            if zcmp(un, fu) != 0 {
                return false;
            }
        }
        true
    }

    /// All edges of the two-rectangle outline as (p, q) segment
    /// pairs (8 per tile, no dedup; interior seam edges are kept — a
    /// junction on one counts once per owning tile via the vertex
    /// test below).
    fn edges_of(rects: &[ZRect]) -> Vec<((Z, Z), (Z, Z))> {
        let mut out = Vec::with_capacity(4 * rects.len());
        for r in rects {
            let a = (r.x0, r.y0);
            let b = (r.x1, r.y0);
            let c = (r.x1, r.y1);
            let d = (r.x0, r.y1);
            out.push((a, b));
            out.push((b, c));
            out.push((c, d));
            out.push((d, a));
        }
        out
    }

    /// `p` is strictly interior to the axis-aligned segment
    /// `a`–`b` (exact Z[√3] comparisons).
    fn on_segment(p: (Z, Z), a: (Z, Z), b: (Z, Z)) -> bool {
        if zcmp(a.0, b.0) == 0 {
            zcmp(p.1, a.1) > 0 && zcmp(p.1, b.1) < 0 && zcmp(p.0, a.0) == 0
        } else {
            zcmp(p.0, a.0) > 0 && zcmp(p.0, b.0) < 0 && zcmp(p.1, a.1) == 0
        }
    }

    /// The seed block: two large chairs tiling the 9×2√3 rectangle
    /// exactly (area 18√3 = 2 × 9, no overlap, no gap).
    #[test]
    fn seed_block_tiles_rectangle() {
        let leaves: Vec<ZRect> = SEED2.iter().flat_map(|c| c.rects()).collect();
        let block = [ZRect {
            x0: (0, 0),
            y0: (0, 0),
            x1: (9, 0),
            y1: (0, 2),
        }];
        assert!(slab_check(&leaves, &block), "seed block exact cover");
        assert_eq!(area_coeff(&block[0]), 18, "block area = 2 × 9 (√3 units)");
    }

    /// Single-seed battery: the per-depth (L, S) counts follow
    /// (L, S) → (L + S, 6L); depth 4 = 133 leaves, total area
    /// 729 = 3⁴·9 (√3 units).
    #[test]
    fn single_seed_counts_and_area_law() {
        let mut tiles = vec![Chair {
            kind: 0,
            m: 0,
            tx: (0, 0),
            ty: (0, 0),
        }];
        let mut seq = Vec::new();
        for _ in 0..4 {
            tiles = grow1(&tiles);
            let n_l = tiles.iter().filter(|c| c.kind == 0).count();
            seq.push((n_l, tiles.len() - n_l));
        }
        assert_eq!(
            seq,
            [(1, 6), (7, 6), (13, 42), (55, 78)],
            "per-depth (L,S) counts"
        );
        assert_eq!(tiles.len(), 133, "depth-4 leaf count");
        let area: i64 = tiles.iter().map(|c| if c.kind == 0 { 9 } else { 3 }).sum();
        assert_eq!(area, 729, "total area = 3⁴·9 (√3 units)");
    }

    /// The depth-4 single-seed battery: the 133 leaves are pairwise
    /// disjoint (exact rect intersection), each lies inside its
    /// parent's conjugated frame, and they exactly tile the √3⁴-
    /// inflated seed region — a pinned region, enumerated
    /// independently, via the exact slab sweep.
    #[test]
    fn single_seed_depth4_battery() {
        let mut parents = vec![Chair {
            kind: 0,
            m: 0,
            tx: (0, 0),
            ty: (0, 0),
        }];
        for _ in 0..3 {
            parents = grow1(&parents);
        }
        assert_eq!(parents.len(), 55, "depth-3 count");
        let frames: Vec<[ZRect; 2]> = parents.iter().map(|p| p.frame_rects()).collect();

        let mut leaves = Vec::new();
        let mut pidx = Vec::new();
        for (i, p) in parents.iter().enumerate() {
            for ch in p.children() {
                leaves.push(ch);
                pidx.push(i);
            }
        }
        assert_eq!(leaves.len(), 133, "depth-4 leaf count");
        let lrects: Vec<ZRect> = leaves.iter().flat_map(|c| c.rects()).collect();

        // (a) pairwise disjointness of all 133 leaves (exact)
        for i in 0..leaves.len() {
            for j in (i + 1)..leaves.len() {
                for r in &lrects[2 * i..2 * i + 2] {
                    for s in &lrects[2 * j..2 * j + 2] {
                        assert!(!rect_overlap(r, s), "leaf overlap ({i},{j})");
                    }
                }
            }
        }

        // (b) each leaf inside its parent's conjugated frame
        for (k, _) in leaves.iter().enumerate() {
            for r in &lrects[2 * k..2 * k + 2] {
                assert!(
                    inside_union(r, &frames[pidx[k]]),
                    "leaf {k} outside its parent's frame"
                );
            }
        }

        // (c) slab sweep against the pinned 9× seed region
        let seed4 = [
            ZRect {
                x0: (0, 0),
                y0: (0, 0),
                x1: (27, 0),
                y1: (0, 18),
            },
            ZRect {
                x0: (27, 0),
                y0: (0, 9),
                x1: (54, 0),
                y1: (0, 18),
            },
        ];
        assert!(
            slab_check(&lrects, &seed4),
            "133 leaves must exactly tile the 9× seed region"
        );
    }

    /// Per-block battery: the two-seed block's per-depth (L, S)
    /// counts are 2× the single-seed sequence; the depth-4 block
    /// (266 leaves) exactly tiles the √3⁴-inflated 9×2√3 block.
    #[test]
    fn seed_block_depth_counts_and_exact_cover() {
        let mut tiles = SEED2.to_vec();
        let mut seq = Vec::new();
        for d in 0..=4 {
            let n_l = tiles.iter().filter(|c| c.kind == 0).count();
            seq.push((n_l, tiles.len() - n_l));
            if d < 4 {
                tiles = grow1(&tiles);
            }
        }
        assert_eq!(
            seq,
            [(2, 0), (2, 12), (14, 12), (26, 84), (110, 156)],
            "per-block (L,S) counts d0..d4"
        );
        let totals = [2, 14, 26, 110, 266];
        for (d, &(n_l, n_s)) in seq.iter().enumerate() {
            assert_eq!(n_l + n_s, totals[d], "d{d}: total leaves");
            assert_eq!(
                9 * n_l as i64 + 3 * n_s as i64,
                18 * 3i64.pow(d as u32),
                "d{d}: area law (2·9·3^d, √3 units)"
            );
        }
        assert_eq!(tiles.len(), 266, "depth-4 block leaf count");
        let lrects: Vec<ZRect> = tiles.iter().flat_map(|c| c.rects()).collect();
        let block = [ZRect {
            x0: (0, 0),
            y0: (0, 0),
            x1: (81, 0),
            y1: (0, 18),
        }];
        assert!(
            slab_check(&lrects, &block),
            "266 leaves must exactly tile the 9×-inflated block"
        );
    }

    /// T-junction census on the 14-tile depth-1 block: exactly 11
    /// (each: a vertex of one tile strictly interior to an edge of a
    /// sibling, undirected) — the tiling is non-edge-to-edge at
    /// every scale.
    #[test]
    fn depth1_block_has_11_t_junctions() {
        let t1 = grow1(&SEED2);
        assert_eq!(t1.len(), 14, "depth-1 block leaf count");
        let teds: Vec<Vec<((Z, Z), (Z, Z))>> = t1.iter().map(|c| edges_of(&c.rects())).collect();
        let mut seen: BTreeSet<(usize, ((Z, Z), (Z, Z)), (Z, Z))> = BTreeSet::new();
        for i in 0..teds.len() {
            let verts: BTreeSet<(Z, Z)> = teds[i].iter().flat_map(|&(a, b)| [a, b]).collect();
            for j in 0..teds.len() {
                if j == i {
                    continue;
                }
                for &v in &verts {
                    for &(a, b) in &teds[j] {
                        if on_segment(v, a, b) {
                            // Undirected identity: (min tile index,
                            // the undirected edge, the vertex).
                            let edge = if (b, a) < (a, b) { (a, b) } else { (b, a) };
                            seen.insert((i.min(j), edge, v));
                        }
                    }
                }
            }
        }
        assert_eq!(seen.len(), 11, "depth-1 T-junction census");
    }

    /// Orientation census: the seed block carries m ∈ {0, 2}; every
    /// depth ≥ 1 carries exactly m ∈ {0, 2, 6, 7} — the four other
    /// dihedral classes never occur (the tiling is chiral).
    #[test]
    fn orientation_census_is_chiral() {
        let mut tiles = SEED2.to_vec();
        let mut sets: Vec<BTreeSet<u8>> = Vec::new();
        sets.push(tiles.iter().map(|c| c.m).collect());
        for _ in 0..4 {
            tiles = grow1(&tiles);
            sets.push(tiles.iter().map(|c| c.m).collect());
        }
        assert_eq!(
            sets[0],
            [0u8, 2].into_iter().collect(),
            "d0 orientation set"
        );
        let all4: BTreeSet<u8> = [0u8, 2, 6, 7].into_iter().collect();
        for d in 1..=4 {
            assert_eq!(&sets[d], &all4, "d{d} orientation set");
        }
        for s in &sets {
            assert!(
                !s.contains(&1) && !s.contains(&3) && !s.contains(&4) && !s.contains(&5),
                "the m ∈ {{1, 3, 4, 5}} classes must never occur"
            );
        }
    }

    /// Period growth: the depth-g block period √3^g·(9, 0),
    /// √3^g·(0, 2√3) (exact Z[√3] vectors) grows without bound —
    /// the limit-periodic signature.
    #[test]
    fn block_periods_grow_exactly() {
        let mut vx = (9, 0);
        let mut vy = (0, 2);
        for g in 0..5 {
            vx = zs3(vx);
            vy = zs3(vy);
            assert!(zcmp(vx, (9, 0)) > 0, "g{g}: x-period must grow");
            assert!(zcmp(vy, (0, 2)) > 0, "g{g}: y-period must grow");
        }
        assert_eq!((vx, vy), ((0, 81), (54, 0)), "g5 periods (= 81√3, 54)");
    }

    /// Emitted chairs are always unit scale: large chairs have area
    /// 9√3, small chairs 3√3 (default params), and both prototiles
    /// plus all four active orientation classes occur in a default
    /// view.
    #[test]
    fn emitted_chairs_are_unit_scale() {
        let t = PregnantChairTiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
            },
        );
        assert!(r.message.is_none(), "{:?}", r.message);
        assert!(!r.tiles.is_empty());
        let mut protos = BTreeSet::new();
        let mut seeds = BTreeSet::new();
        for tile in &r.tiles {
            let a = tile.poly.area();
            if tile.proto == 0 {
                assert!((a - 9.0 * S3).abs() < 1e-9, "large chair area {a} != 9√3");
            } else {
                assert!((a - 3.0 * S3).abs() < 1e-9, "small chair area {a} != 3√3");
            }
            protos.insert(tile.proto);
            seeds.insert(tile.seed);
        }
        assert_eq!(
            protos,
            BTreeSet::from([0u8, 1]),
            "both prototile classes must occur"
        );
        let want: BTreeSet<u32> = [1u32, 3, 7, 8].into_iter().collect();
        assert_eq!(seeds, want, "orientation classes m ∈ {{0,2,6,7}} (+1)");
    }

    /// Bit-exact tile identity: same seed and identical vertex bits.
    fn tile_key(tile: &Tile) -> (u32, Vec<u64>) {
        let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
        for p in &tile.poly.pts {
            bits.push(p.x.to_bits());
            bits.push(p.y.to_bits());
        }
        (tile.seed, bits)
    }

    /// The patch is view-independent within a √3 octave: growing,
    /// panning, or zooming the view only moves the culling window
    /// over one fixed, origin-anchored refined tiling. (All five
    /// extents lie in the g = 2 octave [9√3, 27] of
    /// extent/EXTENT_UNIT.)
    #[test]
    fn patch_is_stable_under_view_changes_within_an_octave() {
        let t = PregnantChairTiling;
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
        assert!(grown.message.is_none(), "{:?}", grown.message);
        let panned = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-7.5, -5.5), v2(8.5, 6.5)),
            },
        );
        assert!(panned.message.is_none(), "{:?}", panned.message);
        let zoomed = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-8.0 / 1.2, -6.0 / 1.2), v2(8.0 / 1.2, 6.0 / 1.2)),
            },
        );
        assert!(zoomed.message.is_none(), "{:?}", zoomed.message);

        let base_keys: BTreeSet<(u32, Vec<u64>)> = base.tiles.iter().map(tile_key).collect();
        let zoomed_keys: BTreeSet<(u32, Vec<u64>)> = zoomed.tiles.iter().map(tile_key).collect();
        let grown_keys: BTreeSet<(u32, Vec<u64>)> = grown.tiles.iter().map(tile_key).collect();
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
        assert!(overlap.message.is_none(), "{:?}", overlap.message);
        let panned_keys: BTreeSet<(u32, Vec<u64>)> = panned.tiles.iter().map(tile_key).collect();
        for tile in &overlap.tiles {
            let k = tile_key(tile);
            assert!(
                base_keys.contains(&k),
                "pan: overlap tile missing/changed in the base view"
            );
            assert!(
                panned_keys.contains(&k),
                "pan: overlap tile missing/changed in the panned view"
            );
        }
        assert!(
            !overlap.tiles.is_empty(),
            "pan: overlap view produced no tiles"
        );
    }

    /// The standard `covers_region` check at scale 1.0.
    #[test]
    fn pregnant_chair_covers_region() {
        let t = PregnantChairTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(SCALE_ID, ParamValue::Float(1.0));
        check_coverage(
            &t,
            "pregnant-chair",
            &p,
            BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)),
        );
    }

    /// Review A8 regression: the descendant cull box for SMALL chairs
    /// once used a half-extent of 2√3^`r`, but the portrait dihedral
    /// classes (m ∈ {2, 6}) span 2√3 — √3 wider — so the cull dropped
    /// the leaves reaching left of their anchor, and the app showed a
    /// strip of background through at region LEFT edges. Pin coverage
    /// on the regions that caught it (the same offsets and jitter the
    /// diagnosis sweep used): a jittered 0.25 grid, every sample
    /// inside some emitted chair.
    #[test]
    fn pregnant_chair_left_edge_covers() {
        let t = PregnantChairTiling;
        let mut p = ParamSet::default_for(t.params());
        p.set(SCALE_ID, ParamValue::Float(1.0));
        for &(w, xo) in &[
            (12.0, -17.0),
            (12.0, 13.0),
            (20.0, -17.0),
            (20.0, -4.0),
            (40.0, -4.0),
            (40.0, 13.0),
        ] {
            let region = BBox::new(v2(xo, -w / 2.0), v2(xo + w, w / 2.0));
            let r = t.generate(&p, &Query { region });
            assert!(r.message.is_none(), "w={w} xo={xo}: {:?}", r.message);
            let step = 0.25;
            let gx = (region.size().x / step).ceil() as i64;
            let gy = (region.size().y / step).ceil() as i64;
            for j in 0..gy {
                for i in 0..gx {
                    let pt = v2(
                        region.min.x + (i as f64 + 0.5 + 0.618_034_0 * 0.37) * step,
                        region.min.y + (j as f64 + 0.5 + 0.754_877_7 * 0.53) * step,
                    );
                    assert!(
                        r.tiles.iter().any(|tl| tl.poly.contains(pt)),
                        "w={w} xo={xo}: gap at ({}, {})",
                        pt.x,
                        pt.y
                    );
                }
            }
        }
    }

    /// The standard rotated + scaled coverage check.
    #[test]
    fn pregnant_chair_rotated_scaled() {
        check_rotated(
            &PregnantChairTiling,
            "pregnant-chair(rot25,scale0.6)",
            BBox::new(v2(-7.0, -7.0), v2(7.0, 7.0)),
            0.6,
            25.0,
        );
    }

    /// The standard determinism check.
    #[test]
    fn pregnant_chair_deterministic() {
        check_det(
            &PregnantChairTiling,
            "pregnant-chair",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
        );
    }
}
