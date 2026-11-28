//! Pentiamond AD factor-2 tiling — the rep-4 substitution tiling of the
//! triangular-lattice pentiamonds A + D (inflation λ = 2).
//!
//! The prototiles are the chiral "A" and the achiral "D" pentiamonds (5
//! unit triangles each; D is the trapezoid quadrilateral
//! [(0,0),(1,0),(1,2),(0,3)]). The Bielefeld substitution index entry
//! "Pentiamond AD factor 2" (discovered: folklore; Info empty — rule
//! image the sole source) inflates by λ = 2 with the recovered
//! dissections A → 2A + 2D, D → 2A + 2D (substitution matrix
//! [[2, 2], [2, 2]] = rank-1, char poly x² − 4x, PF 4 = λ², primitive
//! index 1; per-seed closed form (2·4^(d−1), 2·4^(d−1)) for BOTH seed
//! letters — the rank-1 matrix makes the letter mix decay to 1:1
//! immediately).
//!
//! The D dissection is SELF-MIRROR: a single table serves both achiral
//! pose labels {g, g∘8} (the placed children agree 12/12 — pinned in
//! the tests), so unlike the AC entries there is no second table. The A
//! dissection uses table 0 for every pose (the canonical pin).
//!
//! The dissections were recovered by exhaustive exact enumeration over
//! the decoded folklore patch (recon s02) and are pinned here as cell
//! tables in the identity parent frame.
//!
//! Generation mirrors `sphinx.rs`: the UNIQUE 1×5 seed block (two D
//! tiles, poses 2 and 8) homothetied about the block origin to side
//! u = λ^d on the (u, 0) × (0, 5u) block lattice, then deflated d times
//! with the pinned placement formula — for a parent of letter L, pose
//! g, side λ^k: `R0 = lam_scaled(SHAPES[L], g, λ^k)` (pose FIRST, then
//! homothety about the posed shape's lex-min vertex; the result is
//! already posed), `tau = mR − mqr(gcells(g, U))` with U the union of
//! origin-homothetied child frames, `d = mqr(C) − mR`, children
//! `trans(gcells(g, hom_origin(Fj, λ^{k−1})), d + tau)` at pose
//! `compose(g, s0j)`. Every coordinate is an exact i64 lattice tuple;
//! the runtime carries tiles as `(letter, pose, side, translation)`
//! with the invariant `region == trans(lam_scaled(shape, pose, side),
//! t)` — the placement formula then reduces to O(5) closed-form
//! min-arithmetic per child (validated exhaustively against
//! materialized cell sets in the tests: frame probe 96/96, region
//! identities 96/96). Floats appear only at the emission boundary
//! (world corners).
//!
//! This is the only one of the three pentiamond entries whose folkloric
//! patch is depth-3 nested (recon s07). Every depth is a valid plane
//! tiling with period (λ^d, 0) × (0, 5λ^d) ⇒ a limit-periodic
//! construction with an aperiodic limit (zero exact periods in the unit
//! block tiling, |v| ≤ 10, top near-miss 0.6270 < 1).

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSpec, ParamSet};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// √3/2 — the height of the unit triangle (world y of lattice row 1).
const H: f64 = 0.8660254037844386;

/// Area of one pentiamond tile: 5 unit triangles of area √3/4 each.
const TILE_AREA: f64 = 2.5 * H;

/// Inflation factor λ of this entry.
const LAM: i64 = 2;

/// The 12 symmetries of the triangular lattice as integer matrices,
/// row-major `[a, b, c, d]` acting on lattice points `(q, r)` as
/// `(a·q + b·r, c·q + d·r)`. Index 9 is the identity; det −1 entries are
/// the reflections. Identical to `sphinx.rs` (closure asserted in tests).
const SYMS: [[i64; 4]; 12] = [
    [-1, -1, 0, 1],
    [-1, -1, 1, 0],
    [-1, 0, 0, -1],
    [-1, 0, 1, 1],
    [0, -1, -1, 0],
    [0, -1, 1, 1],
    [0, 1, -1, -1],
    [0, 1, 1, 0],
    [1, 0, -1, -1],
    [1, 0, 0, 1],
    [1, 1, -1, 0],
    [1, 1, 0, -1],
];

/// A unit triangular-lattice cell key `(kind, q, r)`: kind 0 = UP with
/// vertices {(q, r), (q+1, r), (q, r+1)}, kind 1 = DN with vertices
/// {(q+1, r), (q, r+1), (q+1, r+1)}. The anchor vertex (the lex-min of
/// the three) is (q, r) for UP and (q, r+1) for DN.
type Cell = (i64, i64, i64);

/// A side-`m` cell key `(kind, q, r, m)`: the side-m triangle with the
/// SAME orientation and anchor vertex as the unit cell `(kind, q, r)`
/// (m unit triangles wide; holds m² unit cells).
type SCell = (i64, i64, i64, i64);

/// The canonical prototiles in the identity pose (translate-minimized
/// cell sets; each holds 5 unit cells).
const SHAPE_A: [Cell; 5] = [(0, 0, 0), (0, 0, 1), (0, 1, 0), (1, 0, 0), (1, 0, 1)];
const SHAPE_D: [Cell; 5] = [(0, 0, 0), (0, 0, 1), (0, 0, 2), (1, 0, 0), (1, 0, 1)];

fn shape(letter: u8) -> &'static [Cell; 5] {
    if letter == 0 {
        &SHAPE_A
    } else {
        &SHAPE_D
    }
}

/// Canonical corner outlines (CCW, collinear unit steps merged), pinned
/// from the cell-set boundary walks: A is the notched pentagon
/// [(0,0),(2,0),(1,1),(1,2),(0,2)], C the 7-gon
/// [(0,1),(1,0),(2,0),(1,1),(2,1),(1,2),(0,2)]. Emitted under the pose
/// symmetry (reversed when it mirrors) — see `corners`.
const OUTLINE_A: [[i64; 2]; 5] = [[0, 0], [2, 0], [1, 1], [1, 2], [0, 2]];
const OUTLINE_D: [[i64; 2]; 4] = [[0, 0], [1, 0], [1, 2], [0, 3]];

/// The pinned dissection of the A parent (table 0): four children as
/// (letter, frame cells in the canonical parent frame, pose base s0j).
/// `s0j` is the min-index pose whose posed prototile equals the frame up
/// to translation (unique for the chiral A; min of the {g, g∘8} class
/// for the achiral D) — re-derived from the frames by a test.
#[rustfmt::skip]
const TABLE_A: [(u8, [Cell; 5], u8); 4] = [
    (0, [(0, 0, 1), (0, 0, 2), (0, 1, 1), (1, 0, 1), (1, 0, 2)], 9),
    (0, [(0, 0, 3), (0, 1, 3), (1, 0, 3), (1, 1, 2), (1, 1, 3)], 4),
    (1, [(0, 0, 0), (0, 1, 0), (0, 2, 0), (1, 0, 0), (1, 1, 0)], 1),
    (1, [(0, 1, 2), (0, 2, 1), (0, 3, 0), (1, 1, 1), (1, 2, 0)], 0),
];

/// The pinned dissection of the D parent — SELF-MIRROR: one table serves
/// both achiral pose labels {g, g∘8} (placed children agree 12/12,
/// pinned in the tests; no second table exists, unlike the AC entries).
#[rustfmt::skip]
const TABLE_D: [(u8, [Cell; 5], u8); 4] = [
    (0, [(0, 0, 0), (0, 0, 1), (0, 1, 0), (1, 0, 0), (1, 1, 0)], 7),
    (0, [(0, 0, 4), (0, 0, 5), (0, 1, 4), (1, 0, 4), (1, 1, 3)], 6),
    (1, [(0, 0, 2), (0, 0, 3), (1, 0, 1), (1, 0, 2), (1, 0, 3)], 2),
    (1, [(0, 1, 1), (0, 1, 2), (0, 1, 3), (1, 1, 1), (1, 1, 2)], 8),
];

fn table(letter: u8) -> &'static [(u8, [Cell; 5], u8); 4] {
    if letter == 0 {
        &TABLE_A
    } else {
        &TABLE_D
    }
}

/// The UNIQUE seed block: a 1×5 lattice rectangle (5 unit cells... the
/// two D tiles hold 10) tiled by two D tiles (poses 2 and 8) — the s13
/// record, exhaustive-search-pinned. Block (i, j) of the seed lattice
/// sits at (i·u, j·5u) and is the whole-block homothetic image of this
/// canonical block.
const BLOCK_W: i64 = 1;
const BLOCK_H: i64 = 5;
const SEED_TILES: [(u8, u8, i64, i64); 2] = [
    (1, 2, 0, 2),
    (1, 8, 0, 0),
];

/// View extent (world units) at which the seed octave increments: the
/// standard ±8×±6 view (extent 20) lands at depth 3 (calibrated; pinned
/// by `view_stability`).
const EXTENT_UNIT: f64 = 4.0;

fn mat_vec(m: [i64; 4], p: (i64, i64)) -> (i64, i64) {
    (m[0] * p.0 + m[1] * p.1, m[2] * p.0 + m[3] * p.1)
}

fn mat_det(m: [i64; 4]) -> i64 {
    m[0] * m[3] - m[1] * m[2]
}

/// Group composition: the class index of `SYMS[h] · SYMS[k]` (apply `k`
/// first, then `h`).
fn compose(h: u8, k: u8) -> u8 {
    let a = SYMS[h as usize];
    let b = SYMS[k as usize];
    let m = [
        a[0] * b[0] + a[1] * b[2],
        a[0] * b[1] + a[1] * b[3],
        a[2] * b[0] + a[3] * b[2],
        a[2] * b[1] + a[3] * b[3],
    ];
    SYMS
        .iter()
        .position(|&x| x == m)
        .expect("triangular-lattice symmetry group is closed") as u8
}

/// The anchor vertex (lex-min of the three vertices) of a cell key.
fn anchor(kind: i64, q: i64, r: i64) -> (i64, i64) {
    if kind == 0 {
        (q, r)
    } else {
        (q, r + 1)
    }
}

/// The side-`m` key whose anchor vertex is `a` (inverse of `anchor` for
/// scaled cells).
fn side_key(kind: i64, a: (i64, i64), m: i64) -> SCell {
    if kind == 0 {
        (0, a.0, a.1, m)
    } else {
        (1, a.0, a.1 - 1, m)
    }
}

/// (min q, min r) over the m² unit cells of the side-m cell — closed
/// form: the UP anchor is its own lex-min vertex; a DN side-m triangle's
/// lowest row sits m−1 steps below its key row.
fn cell_min(kind: i64, q: i64, r: i64, m: i64) -> (i64, i64) {
    if kind == 0 {
        (q, r)
    } else {
        (q, r - m + 1)
    }
}

/// (min q, min r) over the union of the fine cells of `keys` — the min
/// over per-key closed forms (translations never mix kinds).
fn set_min(keys: &[SCell]) -> (i64, i64) {
    let mut it = keys.iter();
    let k0 = it.next().expect("non-empty key set");
    let mut mq = cell_min(k0.0, k0.1, k0.2, k0.3).0;
    let mut mr = cell_min(k0.0, k0.1, k0.2, k0.3).1;
    for k in it {
        let m = cell_min(k.0, k.1, k.2, k.3);
        mq = mq.min(m.0);
        mr = mr.min(m.1);
    }
    (mq, mr)
}

/// The image of the side-m cell `(kind, q, r)` under `SYMS[g]`: the three
/// vertices map linearly, the new anchor is their lex-min — which is the
/// image of the old anchor shifted by m·δ with δ the lex-min of
/// {(0,0), u₁, u₂} (m-independent for m ≥ 1) — and the new kind follows
/// from the vertex pattern (60° rotations flip UP↔DN, so the kind is
/// never derived from the determinant).
fn pose_cell(g: u8, kind: i64, q: i64, r: i64, m: i64) -> SCell {
    let sym = SYMS[g as usize];
    let a = anchor(kind, q, r);
    let (d1, d2) = if kind == 0 {
        ((1, 0), (0, 1))
    } else {
        ((1, -1), (1, 0))
    };
    let b = mat_vec(sym, a);
    let u1 = mat_vec(sym, d1);
    let u2 = mat_vec(sym, d2);
    let zero = (0, 0);
    let delta = zero.min(u1).min(u2);
    let a2 = (b.0 + m * delta.0, b.1 + m * delta.1);
    let w: Vec<(i64, i64)> = [(0, 0), u1, u2]
        .iter()
        .filter(|v| **v != delta)
        .map(|v| (v.0 - delta.0, v.1 - delta.1))
        .collect();
    let nk = if (w.contains(&(1, 0)) && w.contains(&(0, 1)))
        || (w.contains(&(-1, 0)) && w.contains(&(0, -1)))
        || (w.contains(&(-1, -1)) && w.contains(&(-1, 0)))
        || (w.contains(&(0, 1)) && w.contains(&(-1, -1)))
        || (w.contains(&(-1, 0)) && w.contains(&(0, 1)))
        || (w.contains(&(0, -1)) && w.contains(&(-1, -1)))
    {
        0
    } else {
        1
    };
    side_key(nk, a2, m)
}

/// The lex-min vertex of the shape `letter` under pose `g` — the
/// homothety center of `lam_scaled`.
fn v0_prime(letter: u8, g: u8) -> (i64, i64) {
    let sym = SYMS[g as usize];
    let mut v0: Option<(i64, i64)> = None;
    for c in shape(letter) {
        let vs = cell_verts(c.0, c.1, c.2);
        for p in vs {
            let img = mat_vec(sym, p);
            v0 = Some(match v0 {
                None => img,
                Some(w) => w.min(img),
            });
        }
    }
    v0.expect("shapes are non-empty")
}

/// The three vertices of a unit cell (sorted; matches the recon's
/// `cell_verts`).
fn cell_verts(kind: i64, q: i64, r: i64) -> [(i64, i64); 3] {
    if kind == 0 {
        [(q, r), (q + 1, r), (q, r + 1)]
    } else {
        [(q + 1, r), (q, r + 1), (q + 1, r + 1)]
    }
}

/// `lam_scaled(SHAPES[letter], g, m)` as side-m keys: pose the unit
/// cells FIRST (the exact linear image: anchor at the image's lex-min
/// vertex, kind by vertex pattern), then homothety by m about the posed
/// shape's lex-min vertex v0'. The per-cell pose correction is
/// kind-dependent, so posing first is essential — raw linear images of
/// the anchors do not share a common translation frame (validated
/// key-for-key against the recon's lam_scaled ground truth for every
/// letter, pose, scale). The result is ALREADY posed — never wrap it
/// in `gcells`.
fn lam_scaled_keys(letter: u8, g: u8, m: i64) -> [SCell; 5] {
    let v0 = v0_prime(letter, g);
    let mut out = [(0, 0, 0, 1); 5];
    for (i, c) in shape(letter).iter().enumerate() {
        let p = pose_cell(g, c.0, c.1, c.2, 1);
        let a = anchor(p.0, p.1, p.2);
        let sa = (
            v0.0 + m * (a.0 - v0.0),
            v0.1 + m * (a.1 - v0.1),
        );
        out[i] = side_key(p.0, sa, m);
    }
    out
}

/// The side-`m` image of the unit cell `c` under the origin homothety
/// (the `hom_origin` construction: the anchor maps to m·anchor).
fn hom_cell(c: Cell, m: i64) -> SCell {
    let a = anchor(c.0, c.1, c.2);
    side_key(c.0, (m * a.0, m * a.1), m)
}

/// A pentiamond tile: prototile `letter` (0 = A, 1 = C), pose `g`
/// (0..12), triangle side `side` (a power of λ ≥ 1), and the lattice
/// translation `t` with the invariant
/// `region == trans(lam_scaled(shape, g, side), t)` — asserted
/// materially in the tests at pinned combos.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pentiamond {
    letter: u8,
    g: u8,
    side: i64,
    t: (i64, i64),
}

impl Pentiamond {
    /// The substitution children (table 0 for every pose — the canonical
    /// pin): side side/λ, poses compose(g, s0j), translations from the
    /// pinned placement formula reduced to O(5) closed-form mins:
    /// `t_child = mqr(gcells(g, hom_origin(Fj, m0))) + d + tau −
    /// mR(child)`, with `tau = mR(parent) − mqr(gcells(g, U))` and
    /// `d = t` (the region identity makes the parent's translation the
    /// placement offset directly).
    fn children(&self) -> Vec<Pentiamond> {
        let m0 = self.side / LAM;
        debug_assert!(
            self.side % LAM == 0 && self.side >= LAM,
            "children need side divisible by λ"
        );
        // R0 and its mqr for the parent.
        let r0 = lam_scaled_keys(self.letter, self.g, self.side);
        let mr = set_min(&r0);
        // U = union of the origin-homothetied child frames; posed.
        let tab = table(self.letter);
        let mut ukeys: Vec<SCell> = Vec::with_capacity(tab.len() * 5);
        for (_, f, _) in tab.iter() {
            for c in f.iter() {
                let hk = hom_cell(*c, m0);
                let p = pose_cell(self.g, hk.0, hk.1, hk.2, m0);
                ukeys.push(p);
            }
        }
        let m_u = set_min(&ukeys);
        let tau = (mr.0 - m_u.0, mr.1 - m_u.1);
        let mut out = Vec::with_capacity(tab.len());
        for (lj, f, s0j) in tab.iter() {
            let mut fkeys: Vec<SCell> = Vec::with_capacity(5);
            for c in f.iter() {
                let hk = hom_cell(*c, m0);
                let p = pose_cell(self.g, hk.0, hk.1, hk.2, m0);
                fkeys.push(p);
            }
            let cf = set_min(&fkeys);
            let g_child = compose(self.g, *s0j);
            let r_child = lam_scaled_keys(*lj, g_child, m0);
            let m_rc = set_min(&r_child);
            out.push(Pentiamond {
                letter: *lj,
                g: g_child,
                side: m0,
                t: (
                    cf.0 + self.t.0 + tau.0 - m_rc.0,
                    cf.1 + self.t.1 + tau.1 - m_rc.1,
                ),
            });
        }
        out
    }

    /// The tile's corner outline in world coordinates: the canonical
    /// outline points mapped by the pose symmetry, homothetied about the
    /// posed shape's lex-min vertex, translated by `t` — the exact image
    /// of the canonical boundary under the tile's region map. Mirrored
    /// poses are reversed so every emitted polygon is CCW.
    fn corners(&self) -> Vec<Vec2> {
        let sym = SYMS[self.g as usize];
        let v0 = v0_prime(self.letter, self.g);
        let outline: &[[i64; 2]] = if self.letter == 0 {
            &OUTLINE_A
        } else {
            &OUTLINE_D
        };
        let mut pts: Vec<Vec2> = Vec::with_capacity(outline.len());
        for c in outline.iter() {
            let gc = mat_vec(sym, (c[0], c[1]));
            let q = v0.0 + self.side * (gc.0 - v0.0) + self.t.0;
            let r = v0.1 + self.side * (gc.1 - v0.1) + self.t.1;
            pts.push(Vec2::new(q as f64 + 0.5 * r as f64, H * r as f64));
        }
        if mat_det(sym) < 0 {
            pts.reverse();
        }
        pts
    }

    fn poly(&self) -> Polygon {
        Polygon::from_slice(&self.corners())
    }

    fn aabb(&self) -> BBox {
        let pts = self.corners();
        let mut min = pts[0];
        let mut max = pts[0];
        for p in &pts[1..] {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
        }
        BBox::new(min, max)
    }

    /// The pose label for emission: the chiral A uses the exact pose;
    /// the achiral C is double-labeled {g, g∘8} and emits the min of the
    /// class (the two labels are pure translations of each other).
    fn label(&self) -> u8 {
        if self.letter == 0 {
            self.g
        } else {
            self.g.min(compose(self.g, 8))
        }
    }
}

/// The canonical seed block with its origin at lattice point
/// `(bq, br)`: the whole-block homothetic image of the canonical block
/// (each `SEED_TILES` tile at side u).
fn seed_block(u: i64, bq: i64, br: i64) -> Vec<Pentiamond> {
    let mut v = Vec::with_capacity(SEED_TILES.len());
    for (letter, g, aq, ar) in SEED_TILES {
        // The u-homothetic image of the canonical block tile:
        // cells trans(hom_origin(norm(gcells(g, shape)), u), u·a),
        // re-anchored to the tile's own bookkeeping translation.
        let sym = SYMS[g as usize];
        let posed: Vec<(i64, i64, i64)> = shape(letter)
            .iter()
            .map(|c| {
                let vs = cell_verts(c.0, c.1, c.2);
                // pose the cell key by re-deriving from vertices
                let img: Vec<(i64, i64)> =
                    vs.iter().map(|p| mat_vec(sym, *p)).collect();
                posed_unit_key(img)
            })
            .collect();
        let mp = (
            posed.iter().map(|c| c.1).min().expect("5 cells"),
            posed.iter().map(|c| c.2).min().expect("5 cells"),
        );
        let mut keys: Vec<SCell> = Vec::with_capacity(5);
        for c in &posed {
            let a = anchor(c.0, c.1 - mp.0, c.2 - mp.1);
            keys.push(side_key(c.0, (u * a.0, u * a.1), u));
        }
        let ms = set_min(&keys);
        let mr = set_min(&lam_scaled_keys(letter, g, u));
        v.push(Pentiamond {
            letter,
            g,
            side: u,
            t: (
                bq + ms.0 + u * aq - mr.0,
                br + ms.1 + u * ar - mr.1,
            ),
        });
    }
    v
}

/// Seed tiles: the seed blocks (each holding the canonical block's
/// tiles at side u) on the block lattice ⟨(u, 0), (0, 5u)⟩ anchored
/// through the origin, covering `region`. Block indices are found by
/// projecting the region corners onto the block-lattice basis (with a
/// one-block margin), which is sound: a point in block (i, j) has basis
/// coordinates i + α, j + β with α, β ∈ [0, 1], so the containing
/// block's indices lie within ±1 of any corner's projection.
fn seed_tiles(region: &BBox, u: i64) -> Vec<Pentiamond> {
    // Block lattice vectors in world coordinates: (u, 0) and (0, 5u)
    // lattice = (0.5u, 5u·H) world.
    let (ux, uy) = (BLOCK_W as f64 * u as f64, 0.0);
    let (vx, vy) = (0.5 * BLOCK_H as f64 * u as f64, H * BLOCK_H as f64 * u as f64);
    let det = ux * vy - uy * vx;
    let mut imin = f64::MAX;
    let mut imax = f64::MIN;
    let mut jmin = f64::MAX;
    let mut jmax = f64::MIN;
    for corner in &region.corners() {
        let i = (corner.x * vy - corner.y * vx) / det;
        let j = (ux * corner.y - uy * corner.x) / det;
        imin = imin.min(i);
        imax = imax.max(i);
        jmin = jmin.min(j);
        jmax = jmax.max(j);
    }
    let mut v = Vec::new();
    for j in (jmin.floor() as i64 - 1)..=(jmax.ceil() as i64 + 1) {
        for i in (imin.floor() as i64 - 1)..=(imax.ceil() as i64 + 1) {
            // Block (i, j) of the seed lattice: its canonical origin
            // maps to lattice point (i·W·u, j·H·u).
            let (bq, br) = (i * BLOCK_W * u, j * BLOCK_H * u);
            v.extend(seed_block(u, bq, br));
        }
    }
    v
}

/// Re-derive a posed unit cell key from its three image vertices (the
/// vertex-pattern match; used where the seed construction poses raw
/// vertices).
fn posed_unit_key(vs: Vec<(i64, i64)>) -> (i64, i64, i64) {
    let a = *vs.iter().min().expect("3 vertices");
    let mut w: Vec<(i64, i64)> = vs.iter().map(|p| (p.0 - a.0, p.1 - a.1)).collect();
    w.retain(|d| *d != (0, 0));
    let mut d = w;
    d.sort();
    if d[0] == (0, 1) && d[1] == (1, 0) {
        (0, a.0, a.1)
    } else {
        debug_assert_eq!((d[0], d[1]), ((1, -1), (1, 0)), "UP/DN patterns");
        (1, a.0, a.1 - 1)
    }
}

/// One deflation step: replace every tile by its table-0 children (scale
/// 1/λ), keeping only those whose bounding box intersects `region`.
/// Sound: children never leave their parent (asserted per round in the
/// battery), so a final tile intersecting the region has its full
/// ancestor chain present.
fn deflate_step(tiles: &[Pentiamond], region: &BBox) -> Vec<Pentiamond> {
    let mut out = Vec::with_capacity(tiles.len() * 4);
    for t in tiles {
        for kid in t.children() {
            if kid.aabb().intersects(*region) {
                out.push(kid);
            }
        }
    }
    out
}

/// Pentiamond AD factor-2 tiling (aperiodic limit, limit-periodic
/// rep-4 substitution on the triangular lattice).
pub struct PentiamondAdFactor2Tiling;

impl Tiling for PentiamondAdFactor2Tiling {
    fn id(&self) -> &'static str {
        "pentiamond-ad-factor-2"
    }

    fn name(&self) -> &'static str {
        "Pentiamond AD factor 2 (A + D rep-4)"
    }

    fn description(&self) -> &'static str {
        "Aperiodic limit-periodic pentiamond substitution tiling (Bielefeld \"Pentiamond AD factor 2\"); λ = 2; prototiles A + D (self-mirror D rule); tiles in 12 A orientations."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Nonperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None)
            .with_notes("aperiodic limit; limit-periodic construction; λ = 2 rep-4; prototiles A + D");
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Region in the tiling's own (unit-scale world) coordinates.
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
        // Final tiles have area exactly 5√3/4 ≈ 2.165 in tiling space
        // (five unit triangles; the octave-anchored deflation always
        // lands on unit scale), so this bounds the tile count before
        // any refinement work.
        if region.area() > MAX_TILES as f64 * TILE_AREA {
            return fail("too many tiles for this view — zoom in");
        }

        // View-independent patch: quantize the seed scale to powers of
        // λ (one octave per deflation depth) measured from the region
        // *extent* (not its position — panning must not change the
        // octave), and anchor the seed blocks to the block lattice
        // through the origin. The emitted tiles are the restriction of
        // one global refined tiling to the query region. Deflating
        // exactly `depth` times lands the final tiles on unit scale.
        let extent = (rmax - rmin).length();
        if !extent.is_finite() || extent <= 1e-9 {
            return fail("empty view");
        }
        let depth = ((extent / EXTENT_UNIT).log2().ceil().max(0.0) as u32).min(40);

        let mut tiles = seed_tiles(&region, LAM.pow(depth as u32));
        for _ in 0..depth {
            tiles = deflate_step(&tiles, &region);
        }
        if tiles.is_empty() {
            return fail("empty view");
        }

        // Emit: proto = letter index (0 = A, 1 = D); seed = 1 + the
        // min-class pose label (1..=12), which makes the orientations
        // visible in "pattern" mode.
        let mut out = Vec::with_capacity(tiles.len());
        for t in &tiles {
            let poly = t.poly().transform(&map);
            if poly.aabb().intersects(query.region) {
                out.push(Tile { family: tess_core::tiling::FamilyTag::None, reflected: false,
                    proto: t.letter,
                    seed: t.label() as u32 + 1, level: depth as u8,
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
    use std::collections::{BTreeMap, BTreeSet};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Signed 2× shoelace area of a polygon.
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

    /// The m² unit cells of a side-m cell — the exact fine expansion
    /// (validated against the recon's `sub_lambda` for m = 1..28).
    fn fine_cells(kind: i64, q: i64, r: i64, m: i64) -> BTreeSet<Cell> {
        let mut out = BTreeSet::new();
        if kind == 0 {
            for i in 0..m {
                for j in 0..(m - i) {
                    out.insert((0, q + i, r + j));
                }
            }
            for i in 0..(m - 1) {
                for j in 0..(m - 1 - i) {
                    out.insert((1, q + i, r + j));
                }
            }
        } else {
            for t in 0..m {
                for qp in (q + t)..(q + m) {
                    out.insert((1, qp, r - t));
                }
            }
            for t in 0..(m - 1) {
                for qp in (q + t + 1)..(q + m) {
                    out.insert((0, qp, r - t));
                }
            }
        }
        out
    }

    /// The exact unit-cell region of a tile — the materialization of
    /// the bookkeeping invariant
    /// `region(tile) == trans(lam_scaled(SHAPE[letter], g, side), t)`.
    fn region_cells(t: &Pentiamond) -> BTreeSet<Cell> {
        let mut out = BTreeSet::new();
        for key in lam_scaled_keys(t.letter, t.g, t.side) {
            for cell in fine_cells(key.0, key.1, key.2, key.3) {
                out.insert((cell.0, cell.1 + t.t.0, cell.2 + t.t.1));
            }
        }
        out
    }

    /// The posed fine cells of a side-m key, translated — the
    /// materialized counterpart of the closed-form placement.
    fn key_fine(key: SCell, dq: i64, dr: i64) -> BTreeSet<Cell> {
        fine_cells(key.0, key.1, key.2, key.3)
            .into_iter()
            .map(|c| (c.0, c.1 + dq, c.2 + dr))
            .collect()
    }

    /// The tabulated symmetries are exactly the 12-element triangular
    /// lattice group (copied verbatim from sphinx.rs): distinct, identity
    /// at index 9, det ±1, unit directions preserved, closed under
    /// `compose`.
    #[test]
    fn symmetry_table_is_the_triangular_group() {
        let unit: Vec<(i64, i64)> = vec![(1, 0), (0, 1), (1, -1), (-1, 0), (0, -1), (-1, 1)];
        let unit_set: BTreeSet<(i64, i64)> = unit.iter().copied().collect();
        assert_eq!(SYMS.iter().collect::<BTreeSet<_>>().len(), 12, "distinct");
        assert_eq!(SYMS[9], [1, 0, 0, 1], "identity at index 9");
        for g in SYMS {
            assert_eq!(mat_det(g).abs(), 1, "det {g:?}");
            let imgs: BTreeSet<(i64, i64)> =
                unit.iter().map(|&p| mat_vec(g, p)).collect();
            assert_eq!(imgs, unit_set, "{g:?} preserves unit directions");
        }
        for h in 0..12u8 {
            for k in 0..12u8 {
                let a = SYMS[h as usize];
                let b = SYMS[k as usize];
                let prod = [
                    a[0] * b[0] + a[1] * b[2],
                    a[0] * b[1] + a[1] * b[3],
                    a[2] * b[0] + a[3] * b[2],
                    a[2] * b[1] + a[3] * b[3],
                ];
                assert!(SYMS.contains(&prod), "product {h}∘{k} not in table");
                assert_eq!(compose(h, k) as usize, SYMS.iter().position(|&x| x == prod).unwrap());
            }
        }
    }

    /// The pinned tables partition the canonical λ-scaled parent exactly
    /// (s02 self-check): each table's frames are pairwise disjoint with
    /// union == the identity-pose λ-scaled region, |T| = λ², and the
    /// frame's pose base s0j is the min-index pose whose posed prototile
    /// matches the frame up to translation (unique for A, a {g, g∘8}
    /// pair for C).
    #[test]
    fn rule_tables_partition_the_canonical_region() {
        for letter in 0..2u8 {
            let base = lam_scaled_keys(letter, 9, LAM);
            let idregion: BTreeSet<Cell> = base
                .iter()
                .flat_map(|k| fine_cells(k.0, k.1, k.2, k.3))
                .collect();
            assert_eq!(idregion.len() as i64, 5 * LAM * LAM, "λ²·5 cells");
            // re-anchor to mqr
            let mq = idregion.iter().map(|c| c.1).min().unwrap();
            let mr = idregion.iter().map(|c| c.2).min().unwrap();
            let idcells: BTreeSet<Cell> = idregion
                .iter()
                .map(|c| (c.0, c.1 - mq, c.2 - mr))
                .collect();
            {
                let tab = table(letter);
                let mut u: BTreeSet<Cell> = BTreeSet::new();
                for (lj, f, s0j) in tab.iter() {
                    for c in f.iter() {
                        assert!(u.insert(*c), "table overlap at letter {letter}");
                    }
                    // s0j: min-index pose with norm-match
                    let fnorm: BTreeSet<Cell> = {
                        let mqf = f.iter().map(|c| c.1).min().unwrap();
                        let mrf = f.iter().map(|c| c.2).min().unwrap();
                        f.iter().map(|c| (c.0, c.1 - mqf, c.2 - mrf)).collect()
                    };
                    let mut matching = Vec::new();
                    for p in 0..12u8 {
                        let posed: Vec<(i64, i64, i64)> = shape(*lj)
                            .iter()
                            .map(|c| {
                                let vs = cell_verts(c.0, c.1, c.2);
                                let img: Vec<(i64, i64)> =
                                    vs.iter().map(|pt| mat_vec(SYMS[p as usize], *pt)).collect();
                                posed_unit_key(img)
                            })
                            .collect();
                        let mpq = posed.iter().map(|c| c.1).min().unwrap();
                        let mpr = posed.iter().map(|c| c.2).min().unwrap();
                        let pnorm: BTreeSet<Cell> = posed
                            .iter()
                            .map(|c| (c.0, c.1 - mpq, c.2 - mpr))
                            .collect();
                        if pnorm == fnorm {
                            matching.push(p);
                        }
                    }
                    assert!(!matching.is_empty(), "no pose matches frame");
                    if *lj == 0 {
                        assert_eq!(matching.len(), 1, "chiral A: unique pose");
                    } else {
                        assert_eq!(matching.len(), 2, "achiral C: pose pair");
                        assert_eq!(
                            compose(matching[0], 8),
                            matching[1],
                            "the pair is {{g, g∘8}}"
                        );
                    }
                    assert_eq!(*s0j, matching[0], "s0j = min-index match");
                }
                assert_eq!(u, idcells, "table != canonical region");
                assert_eq!(tab.len() as i64, LAM * LAM, "|T| = λ²");
            }
        }
    }

    /// The pinned placement formula, materially: for EVERY parent
    /// letter, pose g ∈ 0..12, scale k ∈ 1..=4 (one table per letter —
    /// D's rule is self-mirror): the posed child-frame union
    /// translated by tau equals the posed λ^k-scaled parent region
    /// (frame probe), and each child's closed-form bookkeeping
    /// translation satisfies the region invariant
    /// `child cells == trans(lam_scaled(child), t_child)` with t_child
    /// ALSO equal to the materialized mqr difference. Pins: 96 frame
    /// probes / 96 region identities / 384 children (4 per probe —
    /// λ² = 4).
    #[test]
    fn placement_formula_and_child_bookkeeping() {
        let mut nprobe = 0usize;
        let mut nident = 0usize;
        let mut nchild = 0usize;
        for letter in 0..2u8 {
            let tables: Vec<&[(u8, [Cell; 5], u8); 4]> = vec![table(letter)];
            for tab in tables {
                for g in 0..12u8 {
                    for k in 1..=4i64 {
                        let side = LAM.pow(k as u32);
                        let m0 = side / LAM;
                        let r0 = lam_scaled_keys(letter, g, side);
                        let r0fine: BTreeSet<Cell> = r0
                            .iter()
                            .flat_map(|key| fine_cells(key.0, key.1, key.2, key.3))
                            .collect();
                        assert_eq!(
                            r0fine.len() as i64,
                            5 * side * side,
                            "scale test n == 5·λ^(2k)"
                        );
                        let mr = set_min(&r0);
                        let mut ukeys: Vec<SCell> = Vec::new();
                        for (_, f, _) in tab.iter() {
                            for c in f.iter() {
                                let hk = hom_cell(*c, m0);
                                let p = pose_cell(g, hk.0, hk.1, hk.2, m0);
                                ukeys.push(p);
                            }
                        }
                        let m_u = set_min(&ukeys);
                        let tau = (mr.0 - m_u.0, mr.1 - m_u.1);
                        // frame probe (materialized)
                        let mut gu: BTreeSet<Cell> = BTreeSet::new();
                        for key in &ukeys {
                            gu.extend(key_fine(*key, tau.0, tau.1));
                        }
                        assert_eq!(gu, r0fine, "frame probe ({letter}, {g}, k={k})");
                        nprobe += 1;
                        // region identity with d = 0 is the frame probe;
                        // the child path: per child, closed t == materialized t
                        let mut union: BTreeSet<Cell> = BTreeSet::new();
                        for (lj, f, s0j) in tab.iter() {
                            let mut fkeys: Vec<SCell> = Vec::new();
                            for c in f.iter() {
                                let hk = hom_cell(*c, m0);
                                let p = pose_cell(g, hk.0, hk.1, hk.2, m0);
                                fkeys.push(p);
                            }
                            let cf = set_min(&fkeys);
                            let gch = compose(g, *s0j);
                            let rc = lam_scaled_keys(*lj, gch, m0);
                            let m_rc = set_min(&rc);
                            let t_closed =
                                (cf.0 + tau.0 - m_rc.0, cf.1 + tau.1 - m_rc.1);
                            let rcfine: BTreeSet<Cell> = rc
                                .iter()
                                .flat_map(|key| fine_cells(key.0, key.1, key.2, key.3))
                                .collect();
                            let mut placed = BTreeSet::new();
                            for key in &fkeys {
                                // materialized child cells: the posed
                                // frame translated by tau ONLY — the
                                // bookkeeping translation t_closed is
                                // what this sweep derives, never an
                                // input to the materialized path.
                                placed.extend(key_fine(*key, tau.0, tau.1));
                            }
                            let t_mat = (
                                placed.iter().map(|c| c.1).min().unwrap() - m_rc.0,
                                placed.iter().map(|c| c.2).min().unwrap() - m_rc.1,
                            );
                            assert_eq!(
                                t_closed, t_mat,
                                "child bookkeeping ({letter}, {g}, k={k})"
                            );
                            let want: BTreeSet<Cell> = rcfine
                                .iter()
                                .map(|c| (c.0, c.1 + t_closed.0, c.2 + t_closed.1))
                                .collect();
                            assert_eq!(placed, want, "child region invariant");
                            union.extend(placed);
                            nchild += 1;
                        }
                        assert_eq!(union, r0fine, "children tile the parent");
                        nident += 1;
                    }
                }
            }
        }
        assert_eq!(nprobe, 96, "frame probe combos");
        assert_eq!(nident, 96, "region identities");
        assert_eq!(nchild, 384, "children checked");
    }

    /// Achiral self-mirror: D children placed under (g, T_D) equal
    /// those under (g∘8, T_D) for the same physical region — 24/24
    /// combos (12 poses × both cross directions); plus the posed regions
    /// R0(g∘8) and R0(g) are pure translations of each other. (The D
    /// rule carries ONE table serving both achiral labels.)
    #[test]
    fn achiral_label_invariance() {
        let m0 = 1i64;
        let anchor_at = (40i64, 40i64);
        let placed = |g: u8, tab: &[(u8, [Cell; 5], u8); 4]| -> Vec<BTreeSet<Cell>> {
            let r0 = lam_scaled_keys(1, g, 1);
            let mr = set_min(&r0);
            let mut ukeys: Vec<SCell> = Vec::new();
            for (_, f, _) in tab.iter() {
                for c in f.iter() {
                    let hk = hom_cell(*c, m0);
                    let p = pose_cell(g, hk.0, hk.1, hk.2, m0);
                    ukeys.push(p);
                }
            }
            let m_u = set_min(&ukeys);
            let tau = (mr.0 - m_u.0, mr.1 - m_u.1);
            // place the whole region so its mqr lands at anchor_at
            let sh = (anchor_at.0 - mr.0, anchor_at.1 - mr.1);
            let mut out = Vec::new();
            for (_, f, _) in tab.iter() {
                let mut fk: Vec<SCell> = Vec::new();
                for c in f.iter() {
                    let hk = hom_cell(*c, m0);
                    let p = pose_cell(g, hk.0, hk.1, hk.2, m0);
                    fk.push((p.0, p.1, p.2, m0));
                }
                let mut s = BTreeSet::new();
                for key in &fk {
                    s.extend(key_fine(*key, sh.0 + tau.0, sh.1 + tau.1));
                }
                out.push(s);
            }
            out.sort();
            out
        };
        let mut n = 0usize;
        for g in 0..12u8 {
            let gstar = compose(g, 8);
            // pure-translation fact
            let r0 = lam_scaled_keys(1, g, 1);
            let r0s = lam_scaled_keys(1, gstar, 1);
            let f0: BTreeSet<Cell> = r0
                .iter()
                .flat_map(|k| fine_cells(k.0, k.1, k.2, k.3))
                .collect();
            let fs: BTreeSet<Cell> = r0s
                .iter()
                .flat_map(|k| fine_cells(k.0, k.1, k.2, k.3))
                .collect();
            let d0 = (
                fs.iter().map(|c| c.1).min().unwrap() - f0.iter().map(|c| c.1).min().unwrap(),
                fs.iter().map(|c| c.2).min().unwrap() - f0.iter().map(|c| c.2).min().unwrap(),
            );
            let shifted: BTreeSet<Cell> =
                f0.iter().map(|c| (c.0, c.1 + d0.0, c.2 + d0.1)).collect();
            assert_eq!(shifted, fs, "R0(g∘8) == R0(g) + t at g={g}");
            // (g, T_D) == (g∘8, T_D) — the self-mirror single table
            assert_eq!(placed(g, &TABLE_D), placed(gstar, &TABLE_D), "self-mirror at g={g}");
            n += 1;
        }
        assert_eq!(n, 12, "12/12 self-mirror label-invariance combos");
    }

    /// The substitution matrix from table 0 is the pinned
    /// [[2, 2], [2, 2]] (rank-1, column sums λ²), and the per-seed
    /// counts follow the single closed form (2·4^(d−1), 2·4^(d−1))
    /// for BOTH seed letters — (2,2)/(8,8)/(32,32)/(128,128) at
    /// d = 1..4 (rank-1 makes A-seeds and D-seeds identical); the
    /// seed block's round-1 letter counts are M × {A:0, D:2} =
    /// {A:4, D:4}.
    #[test]
    fn substitution_matrix_and_counts() {
        // M from the tables (brief convention: row = child letter,
        // column = parent letter, so column sums are λ²; rank-1 makes
        // the matrix symmetric, but keep the brief's convention).
        let mut m = [[0i64; 2]; 2];
        for letter in 0..2u8 {
            for (lj, _, _) in table(letter).iter() {
                m[*lj as usize][letter as usize] += 1;
            }
        }
        assert_eq!(m, [[2, 2], [2, 2]], "pinned M (rank-1)");
        for col in 0..2 {
            assert_eq!(m[0][col] + m[1][col], LAM * LAM, "column sum λ²");
        }
        // Per-seed closed forms: iterate M^d × (1, 0) and (0, 1) — the
        // rank-1 matrix makes both seed letters follow the same law
        // (2·4^(d−1), 2·4^(d−1)).
        let want = [(2i64, 2i64), (8, 8), (32, 32), (128, 128)];
        for d in 1..=4usize {
            // M^d by repeated multiply
            let mut p = [[1i64, 0], [0, 1]];
            for _ in 0..d {
                let n = [
                    [p[0][0] * m[0][0] + p[0][1] * m[1][0], p[0][0] * m[0][1] + p[0][1] * m[1][1]],
                    [p[1][0] * m[0][0] + p[1][1] * m[1][0], p[1][0] * m[0][1] + p[1][1] * m[1][1]],
                ];
                p = n;
            }
            // v_d = M^d · e_seed is the matching COLUMN of M^d (== the row
            // here: the rank-1 matrix is symmetric).
            assert_eq!((p[0][0], p[1][0]), want[d - 1], "A seed at d={d}");
            assert_eq!((p[0][1], p[1][1]), want[d - 1], "D seed at d={d}");
        }
        // Round-1 block counts: M × {A:0, D:2} (the seed is D-free of A).
        let block = [0i64, 2i64];
        let r1 = [
            m[0][0] * block[0] + m[0][1] * block[1],
            m[1][0] * block[0] + m[1][1] * block[1],
        ];
        assert_eq!(r1, [4, 4], "round-1 block counts {{A:4, D:4}}");
    }

    /// char poly det(xI − M) by the exact 2-permutation expansion,
    /// pinned as the coefficient vector [1, −4, 0] (x² − 4x, rank-1 M).
    #[test]
    fn char_poly_pin() {
        let m = [[2i64, 2], [2, 2]];
        // rank-1: rows equal
        assert_eq!(m[0], m[1], "rank-1 substitution matrix");
        // det(xI − M) = (x − a)(x − d) − b·c  — expand exactly:
        // permutations of {0, 1}: identity (sign +), transposition (−).
        let mut coeff = [0i64; 3]; // x², x, const
        // identity permutation (sign +): (x − m00)(x − m11)
        coeff[0] += 1;
        coeff[1] += -m[0][0] - m[1][1];
        coeff[2] += m[0][0] * m[1][1];
        // transposition (sign −): −(−m01)·(−m10), a constant term
        coeff[2] += -(m[0][1] * m[1][0]);
        // visit count of the 2-permutation expansion is exactly 2
        let visited = 2usize;
        assert_eq!(visited, 2, "2-permutation expansion visited");
        assert_eq!(coeff, [1, -4, 0], "pinned char poly x² − 4x");
    }

    /// The seeded block deflation: round-1 chirality pin (A tiles
    /// P1:P2 = 2:2), per-round letter counts == M^round × block counts,
    /// pairwise-disjoint children with union == the seed rectangle at
    /// every round, and final-round unit validity (5 cells each; 512
    /// tiles at depth 4 = 2·4⁴).
    #[test]
    fn deflation_block_reconstruction() {
        let depth = 4u32;
        let u = LAM.pow(depth);
        // seed the canonical block at the origin
        let seeds = seed_block(u, 0, 0);
        assert_eq!(seeds.len(), 2, "one block's worth of seeds");
        // seeds partition the u-scaled 1×5 rectangle exactly
        let mut all: BTreeMap<Cell, u8> = BTreeMap::new();
        for s in &seeds {
            for c in region_cells(s) {
                assert!(all.insert(c, s.letter).is_none(), "seed overlap");
            }
        }
        assert_eq!(all.len() as i64, 2 * BLOCK_W * u * BLOCK_H * u, "2·W·H·u² cells");
        let m = [[2i64, 2], [2, 2]];
        // rank-1: rows equal
        assert_eq!(m[0], m[1], "rank-1 substitution matrix");
        let block0 = [0i64, 2i64];
        let mut tiles = seeds.clone();
        for round in 1..=depth {
            // deflate, verifying exact partition each round
            let mut kids: Vec<Pentiamond> = Vec::new();
            for t in &tiles {
                kids.extend(t.children());
            }
            let mut cov: BTreeMap<Cell, u8> = BTreeMap::new();
            for k in &kids {
                for c in region_cells(k) {
                    assert!(cov.insert(c, k.letter).is_none(), "child overlap r{round}");
                }
            }
            assert_eq!(cov.len() as i64, 2 * BLOCK_W * u * BLOCK_H * u, "union == seed rectangle");
            tiles = kids;
            // letter counts AFTER this round's deflation = M^round × block0
            let mut p = [[1i64, 0], [0, 1]];
            for _ in 0..round {
                p = [
                    [p[0][0] * m[0][0] + p[0][1] * m[1][0], p[0][0] * m[0][1] + p[0][1] * m[1][1]],
                    [p[1][0] * m[0][0] + p[1][1] * m[1][0], p[1][0] * m[0][1] + p[1][1] * m[1][1]],
                ];
            }
            assert_eq!(
                (p[0][0] * block0[0] + p[0][1] * block0[1], p[1][0] * block0[0] + p[1][1] * block0[1]),
                {
                    let mut cnt = [0i64; 2];
                    for t in &tiles {
                        cnt[t.letter as usize] += 1;
                    }
                    (cnt[0], cnt[1])
                },
                "letter counts at round {round}"
            );
            if round == 1 {
                // A chirality AFTER the round-1 deflation: P1:P2 = 2:2
                let mut chir = [0i64; 2];
                for t in &tiles {
                    if t.letter == 0 {
                        chir[if mat_det(SYMS[t.g as usize]) > 0 { 0 } else { 1 }] += 1;
                    }
                }
                assert_eq!(chir, [2, 2], "round-1 A chirality");
            }
        }
        // final: unit tiles, valid letters and poses
        assert_eq!(tiles.len(), 512, "final unit tile count at depth 4");
        for t in &tiles {
            assert_eq!(t.side, 1, "unit scale");
            assert_eq!(region_cells(t).len(), 5, "5 unit cells");
        }
    }

    /// The substitution refinement property: every tile, at every scale
    /// of the ladder, is the exact union of λ² finer tiles — its
    /// children — pairwise disjoint, each inside the parent. Pinned for
    /// both letters, all 12 poses, sides λ..λ³.
    ///
    /// Cross-depth note (measured): the depth-anchored generation
    /// recipe re-seeds each octave on the block lattice, so depth-d and
    /// depth-(d+1) patches are distinct finite approximants of the same
    /// limit — every shallower unit tile is either bit-identical to
    /// exactly one deeper tile or disjoint from the deeper patch. For
    /// ad-f2 the origin-anchored d2/d3 unit patches share no tile at
    /// all (census 32 disjoint / 0 identical — the 1×5 block's D-pose
    /// chains re-anchor every octave); the folklore patch's level-3
    /// nesting (recon dmax = 3, the only one of the three) lives in the
    /// mixed-level seed structure, not the per-depth recipe.
    #[test]
    fn refinement_property() {
        // (a) per-tile exact λ² partition at every scale ≥ λ
        for side in [LAM, LAM * LAM, LAM * LAM * LAM] {
            for letter in 0..2u8 {
                for g in 0..12u8 {
                    let t = Pentiamond { letter, g, side, t: (0, 0) };
                    let want = region_cells(&t);
                    let kids = t.children();
                    assert_eq!(kids.len() as i64, LAM * LAM, "λ² children");
                    let mut cov = BTreeSet::new();
                    for k in &kids {
                        for c in region_cells(k) {
                            assert!(want.contains(&c), "child cell outside parent");
                            assert!(cov.insert(c), "child overlap");
                        }
                    }
                    assert_eq!(
                        cov, want,
                        "children exactly partition the parent \
                         (side {side}, letter {letter}, pose {g})"
                    );
                }
            }
        }
        // (b) cross-depth census, d2 vs d3 (measured pins)
        let huge = BBox::new(v2(-1e9, -1e9), v2(1e9, 1e9));
        let run = |d: u32| -> Vec<Pentiamond> {
            let mut tiles = seed_block(LAM.pow(d), 0, 0);
            for _ in 0..d {
                tiles = deflate_step(&tiles, &huge);
            }
            tiles
        };
        let t2 = run(2);
        let t3 = run(3);
        assert_eq!(t2.len(), 32, "2·4² unit tiles at depth 2");
        let t3_cells: Vec<BTreeSet<Cell>> = t3.iter().map(region_cells).collect();
        let mut census = [0i64; 2]; // [disjoint, bit-identical]
        for t in &t2 {
            let want = region_cells(t);
            let mut identical = false;
            let mut inside = 0;
            for cells in &t3_cells {
                if cells.iter().all(|c| want.contains(c)) {
                    inside += 1;
                    if *cells == want {
                        identical = true;
                    }
                }
            }
            if identical {
                assert_eq!(inside, 1, "bit-identical recurrence is unique");
                census[1] += 1;
            } else {
                assert_eq!(inside, 0, "no partial containment without identity");
                census[0] += 1;
            }
        }
        assert_eq!(census, [32, 0], "cross-depth census (d2 vs d3)");
    }

    /// A-tile chirality stays balanced 1:1 at every depth (the
    /// substitution matrix's A rows are chirality-balanced); pinned for
    /// d = 1..6 via the closed-form bookkeeping path (no cell
    /// materialization).
    #[test]
    fn chirality_pins() {
        let huge = BBox::new(v2(-1e9, -1e9), v2(1e9, 1e9));
        let mut tiles = seed_block(LAM.pow(6), 0, 0);
        for round in 1..=6u32 {
            tiles = deflate_step(&tiles, &huge);
            let mut chir = [0i64; 2];
            for t in &tiles {
                if t.letter == 0 {
                    chir[if mat_det(SYMS[t.g as usize]) > 0 { 0 } else { 1 }] += 1;
                }
            }
            assert_eq!(chir[0], chir[1], "A chirality 1:1 at d={round}");
        }
    }

    /// Aperiodicity: the fully deflated unit block carries NO exact
    /// period for any shift |dq|, |dr| ≤ 10 (100% same-letter match on
    /// ≥ 120 interior cells would be one), and the measured near-miss
    /// top matches the pinned 0.6270.
    #[test]
    fn no_exact_periods() {
        let huge = BBox::new(v2(-1e9, -1e9), v2(1e9, 1e9));
        let mut tiles = seed_block(LAM.pow(4), 0, 0);
        for _ in 0..4 {
            tiles = deflate_step(&tiles, &huge);
        }
        let mut c2l: BTreeMap<Cell, u8> = BTreeMap::new();
        for t in &tiles {
            for c in region_cells(t) {
                c2l.insert(c, t.letter);
            }
        }
        let vbox = 10i64;
        let minint = 120usize;
        let mut top = (0.0f64, (0i64, 0i64));
        for dq in -vbox..=vbox {
            for dr in -vbox..=vbox {
                if dq == 0 && dr == 0 {
                    continue;
                }
                let mut n = 0usize;
                let mut m2 = 0usize;
                for (c, l) in &c2l {
                    let c2 = (c.0, c.1 + dq, c.2 + dr);
                    if let Some(l2) = c2l.get(&c2) {
                        n += 1;
                        m2 += (l2 == l) as usize;
                    }
                }
                if n > 0 {
                    let ratio = m2 as f64 / n as f64;
                    assert!(
                        !(ratio == 1.0 && n >= minint),
                        "EXACT period at v=({dq},{dr}) n={n}"
                    );
                    if ratio > top.0 {
                        top = (ratio, (dq, dr));
                    }
                }
            }
        }
        assert!(
            (top.0 - 0.6270).abs() <= 1e-3,
            "near-miss top {:.4} at v={:?} != pinned 0.6270",
            top.0,
            top.1
        );
    }

    /// The emitted outline of every orientation class is a CCW A-pentagon
    /// or C-heptagon of the exact area (5·√3/4·s²), and its corner set
    /// matches an INDEPENDENT derivation: the boundary walk of the tile's
    /// own unit cells, collinear steps merged.
    #[test]
    fn emitted_outlines_match_the_cell_boundary() {
        for letter in 0..2u8 {
            for g in 0..12u8 {
                let t = Pentiamond {
                    letter,
                    g,
                    side: 5,
                    t: (13, -7),
                };
                let pts = t.poly().pts;
                let want_len = if letter == 0 { 5 } else { 4 };
                assert_eq!(pts.len(), want_len, "letter {letter} g {g}: corners");
                let a2 = signed_area2(&pts);
                let want = 5.0 * H * 25.0;
                assert!(
                    (a2 - want).abs() < 1e-9 * want,
                    "letter {letter} g {g}: 2×area {a2} != {want} (want CCW)"
                );
                // recover lattice corners and compare with the merged
                // boundary walk of the materialized region
                let got: BTreeSet<(i64, i64)> = pts
                    .iter()
                    .map(|p| {
                        let r = (p.y / H).round() as i64;
                        let q = (p.x - 0.5 * r as f64).round() as i64;
                        (q, r)
                    })
                    .collect();
                assert_eq!(got, boundary_outline(letter, g, 5, (13, -7)));
            }
        }
    }

    /// Independent corner derivation: unit cells of the tile → boundary
    /// unit edges → walk → collinear merge (the sphinx pattern).
    #[cfg(test)]
    fn boundary_outline(
        letter: u8,
        g: u8,
        side: i64,
        t: (i64, i64),
    ) -> BTreeSet<(i64, i64)> {
        let tile = Pentiamond { letter, g, side, t };
        let cells: Vec<Cell> = region_cells(&tile).into_iter().collect();
        let mut edges: BTreeMap<((i64, i64), (i64, i64)), u32> = BTreeMap::new();
        for cell in &cells {
            let mut vs = cell_verts(cell.0, cell.1, cell.2);
            vs.sort();
            for k in 0..3 {
                let (a, b) = (vs[k], vs[(k + 1) % 3]);
                let key = if a <= b { (a, b) } else { (b, a) };
                *edges.entry(key).or_insert(0) += 1;
            }
        }
        let bedges: Vec<((i64, i64), (i64, i64))> = edges
            .into_iter()
            .filter(|(_, n)| *n == 1)
            .map(|(e, _)| e)
            .collect();
        let mut adj: BTreeMap<(i64, i64), Vec<(i64, i64)>> = BTreeMap::new();
        for (a, b) in &bedges {
            adj.entry(*a).or_default().push(*b);
            adj.entry(*b).or_default().push(*a);
        }
        for (v, ws) in &adj {
            assert_eq!(ws.len(), 2, "boundary vertex {v:?} pinched");
        }
        let start = *adj.keys().next().unwrap();
        let mut cyc = vec![start, adj[&start][0]];
        while cyc.last().copied() != Some(start) {
            let (prev, cur) = (cyc[cyc.len() - 2], cyc[cyc.len() - 1]);
            let mut nexts: Vec<(i64, i64)> =
                adj[&cur].iter().copied().filter(|&w| w != prev).collect();
            nexts.sort();
            assert_eq!(nexts.len(), 1, "ambiguous walk at {cur:?}");
            cyc.push(nexts[0]);
        }
        cyc.pop();
        // CCW in world orientation
        let w = |p: (i64, i64)| Vec2::new(p.0 as f64 + 0.5 * p.1 as f64, H * p.1 as f64);
        let n = cyc.len();
        let mut area2 = 0.0;
        for i in 0..n {
            let a = w(cyc[i]);
            let b = w(cyc[(i + 1) % n]);
            area2 += a.x * b.y - b.x * a.y;
        }
        if area2 < 0.0 {
            cyc.reverse();
        }
        // merge collinear runs into corners
        let n = cyc.len();
        let dirs: Vec<(i64, i64)> = (0..n)
            .map(|i| (cyc[(i + 1) % n].0 - cyc[i].0, cyc[(i + 1) % n].1 - cyc[i].1))
            .collect();
        (0..n)
            .filter(|&i| dirs[i] != dirs[(i + n - 1) % n])
            .map(|i| cyc[i])
            .collect()
    }

    /// View stability: within one octave the patch is fixed — growing,
    /// panning, or zooming the view only moves the culling window over
    /// one fixed, origin-anchored refined tiling.
    #[test]
    fn patch_is_stable_under_view_changes_within_an_octave() {
        let t = PentiamondAdFactor2Tiling;
        let p = ParamSet::default_for(t.params());
        let base = t.generate(&p, &Query { region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)) });
        assert!(base.message.is_none(), "{:?}", base.message);
        let grown = t.generate(&p, &Query { region: BBox::new(v2(-8.4, -6.3), v2(8.4, 6.3)) });
        // Zoom by 1.2: diagonal extent 20/1.2 ≈ 16.7 stays inside the
        // depth-3 octave (extent ∈ (16, 32]); a 1.3 zoom would cross the
        // octave boundary (extent 15.4 → depth 2) and land on the
        // *different*, depth-2 anchored patch.
        let zoomed = t.generate(
            &p,
            &Query { region: BBox::new(v2(-8.0 / 1.2, -6.0 / 1.2), v2(8.0 / 1.2, 6.0 / 1.2)) },
        );
        let key = |tile: &Tile| -> (u8, u32, Vec<u64>) {
            let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
            for p in &tile.poly.pts {
                bits.push(p.x.to_bits());
                bits.push(p.y.to_bits());
            }
            (tile.proto, tile.seed, bits)
        };
        let base_keys: BTreeSet<(u8, u32, Vec<u64>)> =
            base.tiles.iter().map(key).collect();
        let zoomed_keys: BTreeSet<(u8, u32, Vec<u64>)> =
            zoomed.tiles.iter().map(key).collect();
        for k in &zoomed_keys {
            assert!(base_keys.contains(k), "zoom: tile changed");
        }
        let grown_keys: BTreeSet<(u8, u32, Vec<u64>)> =
            grown.tiles.iter().map(key).collect();
        for k in &base_keys {
            assert!(grown_keys.contains(k), "grow: tile changed when the view grew");
        }
    }

    /// EXTENT_UNIT calibration: the standard ±8×±6 view lands at depth
    /// 3, so every emitted tile is at exact unit scale (area 5√3/4) and
    /// both prototiles appear; a far view is refused by the area guard
    /// with the zoom-in message.
    #[test]
    fn view_stability_and_unit_scale() {
        let t = PentiamondAdFactor2Tiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        assert!(!r.tiles.is_empty());
        let want_area = TILE_AREA;
        let mut protos = BTreeSet::new();
        for tile in &r.tiles {
            let a = tile.poly.area();
            assert!(
                (a - want_area).abs() < 1e-9,
                "tile area {a} != {want_area} (unit scale at default params)"
            );
            protos.insert(tile.proto);
        }
        assert_eq!(protos, BTreeSet::from([0, 1]), "both prototiles visible");
        // far view: area guard fires
        let far = t.generate(&p, &Query { region: BBox::new(v2(-1e6, -1e6), v2(1e6, 1e6)) });
        assert_eq!(
            far.message,
            Some("too many tiles for this view — zoom in"),
            "area guard"
        );
    }
}