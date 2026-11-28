//! Penrose P1 tiling (pentagon, star, boat, diamond) by φ² deflation.
//!
//! Four prototiles (edge 1, CCW): the pentagon `P` (area 1.72048), the
//! 10-point star `S` (2.12663), the boat heptagon `B` (1.35721) and the
//! 36° diamond `D` (0.58779). Deflation replaces every tile by children
//! at 1/φ² edge that tile it exactly:
//!
//! ```text
//! P -> 6P + 5 gnomons       (gnomons are consumed by the merges below,
//!                            or discarded at the patch boundary)
//! S -> S + 5P + 5B
//! B -> 3P + 3B + 1S
//! D -> S + P + B
//! ```
//!
//! The children are assembled from short-edge (1/φ) pieces glued *across*
//! parent boundaries by four merge rules: `G + G -> D`, `CORE + 2G -> B`,
//! `S2C + 2G -> S`, `S1C + G -> S`, where G = pentagon-flower gnomon,
//! CORE = boat core, S1C/S2C = star with 1/2 tips removed. An unmatched
//! short edge (no partner across a boundary) is discarded.
//!
//! # Generation model: a single-seed gasket
//!
//! The patch is generated from **one** pentagon seed (edge φ^(2m))
//! deflated m times, with m chosen so the view fits inside the patch's
//! guaranteed-clean interior. The result is an exact, aperiodic,
//! φ²-self-similar tiling of a pentagonal *gasket*, not of a clean disk:
//! every interior point is covered exactly once (no overlaps — no edge
//! has more than two tile owners — and no enclosed gaps), but five
//! 5-fold-symmetric notch channels (the discarded first-level gnomons)
//! open at the boundary and reach to radius R/φ², with smaller
//! self-similar fjords inside, so the uncovered fraction of a disk crop
//! grows with radius (≈ 2–16%). The open disk of radius R/φ² about the
//! seed center (R = seed circumradius) is tiled exactly, and the
//! generator scales the seed so that disk always covers the query
//! region.
//!
//! This single-seed construction is a different *window* on the P1
//! tiling family from classical full-plane reference patches: bulk
//! tile ratios here are ≈ 23.0 : 1 : 1.90 : 4.51 (P : S : B : D)
//! against ≈ 17.4 : 1 : 2.2 : 3.5 in the standard pkl reference, while
//! the deep interior (0.4 R) agrees closely (17.9 : 1 : 2.23 : 3.7) —
//! the local P/S macro substitution rules reproduce the reference
//! wherever the patch is intact.

use std::collections::HashMap;
use std::f64::consts::PI;

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, MAX_TILES, SCALE_ID, SIMILARITY_PARAMS};

// ---- exact constants -------------------------------------------------------

const D18: f64 = PI / 10.0;
const D36: f64 = PI / 5.0;
const D72: f64 = 2.0 * PI / 5.0;
const D90: f64 = PI / 2.0;
const D108: f64 = 3.0 * PI / 5.0;
const D126: f64 = 7.0 * PI / 10.0;
const D144: f64 = 4.0 * PI / 5.0;
const D162: f64 = 9.0 * PI / 10.0;
const D180: f64 = PI;
const D216: f64 = PI + D36;

const C36: f64 = 0.809_016_994_374_947_5; // cos 36°
const S36: f64 = 0.587_785_252_292_473_1; // sin 36°
const PHI: f64 = 1.618_033_988_749_895; // (1+√5)/2
const IP: f64 = 0.618_033_988_749_894_8; // 1/φ
const IP2: f64 = 0.381_966_011_250_105_1; // 1/φ²
const R_P: f64 = 0.5 / S36; // pentagon circumradius, edge 1 (0.8506508)
const R_C: f64 = R_P * PHI; // star tip radius (1.3763819)
const R_N: f64 = R_P * IP; // star notch radius (0.5257311)
const AP: f64 = PHI * PHI * R_P * C36; // gnomon base offset = pentagon apothem (1.8017086)
const BIG: f64 = R_C + R_P; // φ²·R_P = star-center anchor (2.2270325)

/// Area of the unit-edge pentagon (largest prototile): √(25+10√5)/4.
const A_P: f64 = 1.720_477_400_588_967;

/// Deepest deflation used: m = 6 gives ≈ 72k tiles; m = 7 would exceed
/// [`MAX_TILES`], so views that do not fit a level-6 patch are rejected.
const MAX_M: i32 = 6;

// tile kinds (also the prototile indices: P is the main tile)
const TK_P: u8 = 0;
const TK_S: u8 = 1;
const TK_B: u8 = 2;
const TK_D: u8 = 3;

// atom kinds (substitution pieces)
const AK_P: u8 = 0;
const AK_S0: u8 = 1;
const AK_PA: u8 = 2;
const AK_G: u8 = 3;
const AK_CORE: u8 = 4;
const AK_S1C: u8 = 5;
const AK_S2C: u8 = 6;

/// One tile of the patch, in tiling-native (cell) coordinates: a kind,
/// pose `(c, th)` and edge length `e` (all powers of φ apart).
#[derive(Clone, Copy)]
struct P1Tile {
    kind: u8,
    c: Vec2,
    th: f64,
    e: f64,
}

/// One substitution atom (a child piece, possibly crossing a parent
/// boundary). `pts` holds the CCW vertex list in its first entries
/// (length depends on the atom kind; the rest is padding), `c`/`th`
/// the tile pose it carries to output (for P/S0/S1C/S2C), `rho`/`apex`
/// the boat data a CORE carries (slot angle and boat apex).
#[derive(Clone, Copy)]
struct Atom {
    ak: u8,
    pts: [Vec2; 10],
    c: Vec2,
    th: f64,
    rho: f64,
    apex: Vec2,
}

fn ud(a: f64) -> Vec2 {
    Vec2::new(a.cos(), a.sin())
}

// ---- shape constructors (all CCW, e = piece edge) --------------------------

/// Pentagon vertices, v0 at `c + e·R_P·ud(th)`.
fn pentagon_pts(c: Vec2, th: f64, e: f64) -> [Vec2; 5] {
    let mut v = [c; 5];
    for (k, slot) in v.iter_mut().enumerate() {
        *slot = c + ud(th + D72 * k as f64) * (e * R_P);
    }
    v
}

/// 10-point star: tips at `th + 72k` (radius e·R_C), notches at
/// `th + 36 + 72k` (radius e·R_N).
fn star10_pts(c: Vec2, th: f64, e: f64) -> [Vec2; 10] {
    let mut v = [c; 10];
    for k in 0..5 {
        v[2 * k] = c + ud(th + D72 * k as f64) * (e * R_C);
        v[2 * k + 1] = c + ud(th + D36 + D72 * k as f64) * (e * R_N);
    }
    v
}

/// Star with the listed tips removed (tip k sits at vertex index 2k; only
/// the tip vertex is dropped, its flanking notches stay and form the chord).
fn star_minus_pts(c: Vec2, th: f64, e: f64, drop: &[u8]) -> Vec<Vec2> {
    let all = star10_pts(c, th, e);
    all.iter()
        .enumerate()
        .filter(|(i, _)| !drop.iter().any(|&m| *i == (2 * m) as usize))
        .map(|(_, p)| *p)
        .collect()
}

/// Star-anchored pentagon (PA) at notch `rho`: v0/v2 at the neighboring
/// tips, v1 at the notch, then the two outer edges.
fn pa_pts(c: Vec2, rho: f64, e: f64) -> [Vec2; 5] {
    let v1 = c + ud(rho) * (e * R_N);
    let v0 = c + ud(rho + D36) * (e * R_C);
    let v2 = c + ud(rho - D36) * (e * R_C);
    let a = v2 + ud(rho + D18) * e;
    let bv = a + ud(rho + D90) * e;
    [v0, v1, v2, a, bv]
}

/// Boat core (5-gon [apex, M2, Bv, A, M1]) under notch `rho`.
fn core_pts(c: Vec2, rho: f64, e: f64) -> [Vec2; 5] {
    let pa = pa_pts(c, rho, e);
    let a = pa[3];
    let apex = a + ud(rho + D18) * (e * PHI);
    let m1 = apex + ud(rho - D162) * e;
    let m2 = apex + ud(rho + D162) * e;
    [apex, m2, pa[4], a, m1]
}

/// Pentagon-flower gnomon at apex `c + e·R_P·ud(th + 72j)`:
/// triangle [apex, base-minus, base-plus], base length e/φ.
fn gnomon_pts(c: Vec2, th: f64, e: f64, j: u8) -> [Vec2; 3] {
    let dj = th + D72 * j as f64;
    let v = c + ud(dj) * (e * R_P);
    let mid = c + ud(dj) * (e * AP);
    let off = ud(dj + D90) * (e * 0.5 * IP);
    [v, mid - off, mid + off]
}

/// Complete boat heptagon from macro center `c_m` and slot `rho`
/// (apex = c_m + e·φ³·R_P·ud(rho)).
fn b_heptagon_pts(c_m: Vec2, rho: f64, e: f64) -> [Vec2; 7] {
    let apex = c_m + ud(rho) * (e * PHI * PHI * PHI * R_P);
    let m1 = apex + ud(rho - D162) * e;
    let m2 = apex + ud(rho + D162) * e;
    let t1 = apex + ud(rho - D126) * (e * PHI);
    let t2 = apex + ud(rho + D126) * (e * PHI);
    let a = apex + ud(rho - D162) * (e * PHI);
    let bv = apex + ud(rho + D162) * (e * PHI);
    [apex, m2, t2, bv, a, t1, m1]
}

/// 36° diamond (edge φ·er) with anchor `c` at angle `th`.
fn d_rhombus_pts(c: Vec2, th: f64, er: f64) -> [Vec2; 4] {
    let t0 = c + ud(th) * (er * R_C);
    let t2 = c + ud(th + D144) * (er * R_C);
    let t3 = c + ud(th + D216) * (er * R_C);
    let apex = c + ud(th + D180) * (er * PHI * PHI * PHI * R_P);
    [t0, t2, apex, t3]
}

/// Pose of a PA piece as an output pentagon: centroid + v0 direction.
fn pa_pose(pts: &[Vec2]) -> (Vec2, f64) {
    let mut c = Vec2::ZERO;
    for &p in &pts[..5] {
        c += p * 0.2;
    }
    let d = pts[0] - c;
    (c, d.y.atan2(d.x))
}

/// The final tile polygon for a patch tile.
fn tile_poly(t: &P1Tile) -> Polygon {
    match t.kind {
        TK_P => Polygon::from_slice(&pentagon_pts(t.c, t.th, t.e)),
        TK_S => Polygon::from_slice(&star10_pts(t.c, t.th, t.e)),
        TK_B => Polygon::from_slice(&b_heptagon_pts(t.c, t.th + D180, t.e)),
        TK_D => Polygon::from_slice(&d_rhombus_pts(t.c, t.th, t.e * IP2)),
        _ => unreachable!(),
    }
}

// ---- substitution ------------------------------------------------------------

fn push(
    atoms: &mut Vec<Atom>,
    ak: u8,
    pts: &[Vec2],
    c: Vec2,
    th: f64,
    rho: f64,
    apex: Vec2,
) {
    let n = pts.len() as u8;
    debug_assert!(n <= 10);
    let mut arr = [Vec2::ZERO; 10];
    arr[..n as usize].copy_from_slice(pts);
    atoms.push(Atom { ak, pts: arr, c, th, rho, apex });
}

/// The short-edge (length e/φ) vertex pairs an atom claims across
/// parent boundaries; only these may be merged.
fn edge_pairs(ak: u8) -> &'static [(u8, u8)] {
    match ak {
        AK_G => &[(1, 2)],
        AK_CORE => &[(1, 2), (3, 4)],
        AK_S2C => &[(1, 2), (6, 7)],
        AK_S1C => &[(8, 0)],
        _ => &[],
    }
}

/// Exact-geometry segment key: rounded, order-sorted endpoint pair.
type SegKey = [(i64, i64); 2];

fn seg_key(a: Vec2, b: Vec2, e: f64) -> SegKey {
    let k = |p: Vec2| {
        ((p.x / e * 1e7).round() as i64, (p.y / e * 1e7).round() as i64)
    };
    let (mut p, mut q) = (k(a), k(b));
    if q < p {
        std::mem::swap(&mut p, &mut q);
    }
    [p, q]
}

/// Emit the substitution atoms of tile `t` at child edge `e`
/// (= t.e / φ²). Frame offsets (cell coordinates): P, S: `(c, th−36°)`;
/// B: star center `c + t.e·BIG·ud(th+180°)` at angle `th` (the anchor
/// scales with the *parent* edge); D: `(c, th)`.
fn emit(t: &P1Tile, e: f64, atoms: &mut Vec<Atom>) {
    let (cf, tf) = match t.kind {
        TK_P | TK_S => (t.c, t.th - D36),
        TK_B => (t.c + ud(t.th + D180) * (BIG * t.e), t.th),
        TK_D => (t.c, t.th),
        _ => unreachable!(),
    };
    match t.kind {
        TK_P => {
            push(atoms, AK_P, &pentagon_pts(cf, tf, e), cf, tf, 0.0, Vec2::ZERO);
            for i in 0..5 {
                let ang = tf + D36 + D72 * i as f64;
                let cs = cf + ud(ang) * (e * R_C);
                push(atoms, AK_P, &pentagon_pts(cs, ang, e), cs, ang, 0.0, Vec2::ZERO);
            }
            for j in 0..5 {
                push(atoms, AK_G, &gnomon_pts(cf, tf, e, j), cf, tf, 0.0, Vec2::ZERO);
            }
        }
        TK_S => {
            push(atoms, AK_S0, &star10_pts(cf, tf, e), cf, tf, 0.0, Vec2::ZERO);
            for i in 0..5 {
                let rho = tf + D36 + D72 * i as f64;
                let pa = pa_pts(cf, rho, e);
                let (pc, pth) = pa_pose(&pa);
                push(atoms, AK_PA, &pa, pc, pth, 0.0, Vec2::ZERO);
                let co = core_pts(cf, rho, e);
                push(atoms, AK_CORE, &co, cf, tf, rho, co[0]);
            }
        }
        TK_B => {
            for i in 0..3 {
                let rho = tf + D108 + D72 * i as f64;
                let pa = pa_pts(cf, rho, e);
                let (pc, pth) = pa_pose(&pa);
                push(atoms, AK_PA, &pa, pc, pth, 0.0, Vec2::ZERO);
                let co = core_pts(cf, rho, e);
                push(atoms, AK_CORE, &co, cf, tf, rho, co[0]);
            }
            // S1C = star minus tip 0 (the tip pointing from the star
            // center back toward the boat center); its chord sits on the
            // boat's flat edge, completed by the neighbor's gnomon.
            push(atoms, AK_S1C, &star_minus_pts(cf, tf, e, &[0]), cf, tf, 0.0, Vec2::ZERO);
        }
        TK_D => {
            push(
                atoms,
                AK_S2C,
                &star_minus_pts(cf, tf, e, &[1, 4]),
                cf,
                tf,
                0.0,
                Vec2::ZERO,
            );
            let rho = tf + D180;
            let pa = pa_pts(cf, rho, e);
            let (pc, pth) = pa_pose(&pa);
            push(atoms, AK_PA, &pa, pc, pth, 0.0, Vec2::ZERO);
            let co = core_pts(cf, rho, e);
            push(atoms, AK_CORE, &co, cf, tf, rho, co[0]);
        }
        _ => unreachable!(),
    }
}

/// Insertion-ordered segment map.
///
/// The merge rules consume claims in first-claim order, and the `G+G->D`
/// pose is anchored on the first-claiming gnomon's apex: both gnomons draw
/// the same rhombus, but only the first gnomon's anchor places the S2C
/// star (emitted at the next level from the D pose) where the neighbor
/// gnomons complete it. The reference generator iterates its segment
/// dict in insertion order, so this map preserves first-claim order to
/// keep the port bit-faithful to the verified Python tiling.
struct SegMap {
    order: Vec<SegKey>,
    index: HashMap<SegKey, usize>,
    claims: Vec<Vec<usize>>,
}

impl SegMap {
    fn push(&mut self, k: SegKey, ai: usize) {
        match self.index.get(&k) {
            Some(&i) => self.claims[i].push(ai),
            None => {
                self.index.insert(k, self.order.len());
                self.order.push(k);
                self.claims.push(vec![ai]);
            }
        }
    }

    fn get(&self, k: &SegKey) -> Option<&[usize]> {
        self.index.get(k).map(|&i| &self.claims[i][..])
    }

    /// Claim lists in first-claim (insertion) order.
    fn iter_claims(&self) -> impl Iterator<Item = &[usize]> + '_ {
        self.order.iter().map(|k| &self.claims[self.index[k]][..])
    }
}

/// Assemble the tiles of one deflation level from all atoms at edge `e`.
///
/// Pairs each segment claimed by exactly two atoms by the four merge
/// rules (in first-claim order, see [`SegMap`]), then emits the
/// standalone pieces (P, S0, PA) in atom order. Unmatched G/CORE/S2C/S1C
/// atoms are counted as discarded (the gasket's notch channels).
///
/// Returns (tiles, discarded counts [G, CORE, S2C, S1C]).
fn merge(atoms: &[Atom], e: f64) -> (Vec<P1Tile>, [u64; 4]) {
    let mut segmap = SegMap {
        order: Vec::new(),
        index: HashMap::new(),
        claims: Vec::new(),
    };
    for (ai, a) in atoms.iter().enumerate() {
        for &(i, j) in edge_pairs(a.ak) {
            segmap.push(seg_key(a.pts[i as usize], a.pts[j as usize], e), ai);
        }
    }

    let n = atoms.len();
    let mut used_g = vec![false; n];
    let mut used_c = vec![false; n];
    let mut used_s = vec![false; n];
    let mut used_s1 = vec![false; n];
    let mut tiles = Vec::new();

    for claims in segmap.iter_claims() {
        if claims.len() != 2 {
            continue;
        }
        let (i1, i2) = (claims[0], claims[1]);
        let (a1, a2) = (&atoms[i1], &atoms[i2]);
        match (a1.ak, a2.ak) {
            (AK_G, AK_G) => {
                let v1 = a1.pts[0];
                let bm = a1.pts[1];
                let bp = a1.pts[2];
                let cm = (bm + bp) * 0.5;
                let d = v1 - cm;
                let thd = d.y.atan2(d.x);
                let off = cm + ud(thd) * (e * R_P * 0.5);
                tiles.push(P1Tile { kind: TK_D, c: off, th: thd, e });
                used_g[i1] = true;
                used_g[i2] = true;
            }
            (AK_CORE, AK_G) | (AK_G, AK_CORE) => {
                let ci = if a1.ak == AK_CORE { i1 } else { i2 };
                if used_c[ci] {
                    continue;
                }
                let a = &atoms[ci];
                let mut partners = [0usize; 2];
                let mut np = 0usize;
                for &(i, j) in &[(1u8, 2u8), (3, 4)] {
                    let cl = segmap
                        .get(&seg_key(a.pts[i as usize], a.pts[j as usize], e))
                        .expect("claimed segment present");
                    if cl.len() != 2 {
                        np = 0;
                        break;
                    }
                    let other = if cl[0] == ci { cl[1] } else { cl[0] };
                    if atoms[other].ak != AK_G || used_g[other] {
                        np = 0;
                        break;
                    }
                    partners[np] = other;
                    np += 1;
                }
                if np != 2 {
                    continue;
                }
                used_c[ci] = true;
                used_g[partners[0]] = true;
                used_g[partners[1]] = true;
                // Boat macro center: apex pulled back by e·φ³·R_P.
                let cm = a.apex - ud(a.rho) * (e * PHI * PHI * PHI * R_P);
                tiles.push(P1Tile { kind: TK_B, c: cm, th: a.rho - D180, e });
            }
            (AK_S2C, AK_G) | (AK_G, AK_S2C) => {
                let si = if a1.ak == AK_S2C { i1 } else { i2 };
                if used_s[si] {
                    continue;
                }
                let a = &atoms[si];
                let mut partners = [0usize; 2];
                let mut np = 0usize;
                for &(i, j) in &[(1u8, 2u8), (6, 7)] {
                    let cl = segmap
                        .get(&seg_key(a.pts[i as usize], a.pts[j as usize], e))
                        .expect("claimed segment present");
                    if cl.len() != 2 {
                        np = 0;
                        break;
                    }
                    let other = if cl[0] == si { cl[1] } else { cl[0] };
                    if atoms[other].ak != AK_G || used_g[other] {
                        np = 0;
                        break;
                    }
                    partners[np] = other;
                    np += 1;
                }
                if np != 2 {
                    continue;
                }
                used_s[si] = true;
                used_g[partners[0]] = true;
                used_g[partners[1]] = true;
                tiles.push(P1Tile { kind: TK_S, c: a.c, th: a.th, e });
            }
            (AK_S1C, AK_G) | (AK_G, AK_S1C) => {
                let si = if a1.ak == AK_S1C { i1 } else { i2 };
                if used_s1[si] {
                    continue;
                }
                let a = &atoms[si];
                let cl = segmap
                    .get(&seg_key(a.pts[8], a.pts[0], e))
                    .expect("claimed segment present");
                if cl.len() != 2 {
                    continue;
                }
                let other = if cl[0] == si { cl[1] } else { cl[0] };
                if atoms[other].ak != AK_G || used_g[other] {
                    continue;
                }
                used_s1[si] = true;
                used_g[other] = true;
                tiles.push(P1Tile { kind: TK_S, c: a.c, th: a.th, e });
            }
            _ => {}
        }
    }

    let mut disc = [0u64; 4];
    for (ai, a) in atoms.iter().enumerate() {
        match a.ak {
            AK_P => tiles.push(P1Tile { kind: TK_P, c: a.c, th: a.th, e }),
            AK_S0 => tiles.push(P1Tile { kind: TK_S, c: a.c, th: a.th, e }),
            AK_PA => {
                let (c, th) = pa_pose(&a.pts);
                tiles.push(P1Tile { kind: TK_P, c, th, e });
            }
            AK_G if !used_g[ai] => disc[0] += 1,
            AK_CORE if !used_c[ai] => disc[1] += 1,
            AK_S2C if !used_s[ai] => disc[2] += 1,
            AK_S1C if !used_s1[ai] => disc[3] += 1,
            _ => {}
        }
    }
    (tiles, disc)
}

/// Radius of the guaranteed-clean disk (cell units) after `m`
/// deflations of a φ^(2m)-edge seed pentagon: R_P·φ^(2m−2).
///
/// Every uncovered point of the gasket lies at radius ≥ R/φ² from the
/// seed center: the five first-level gnomon fjords reach exactly to
/// R/φ² at their (zero-area) apexes, and every deeper discarded gnomon
/// is a self-similar copy pushed further out.
fn clean_radius(m: i32) -> f64 {
    R_P * PHI.powi(2 * m - 2)
}

// ---- Tiling impl -------------------------------------------------------------

/// Penrose P1 tiling (aperiodic, local 5-fold symmetry).
pub struct PenroseP1Tiling;

impl Tiling for PenroseP1Tiling {
    fn id(&self) -> &'static str {
        "penrose-p1"
    }

    fn name(&self) -> &'static str {
        "Penrose P1 (pentagon, star, boat, diamond)"
    }

    fn description(&self) -> &'static str {
        "Quasiperiodic P1 tiling by pentagon, 10-point star, boat and diamond tiles; \
         exact φ²-deflation of a single pentagon seed (gasket patch)."
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
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None)
            .with_notes("aperiodic; local 5-fold symmetry");
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Region in the tiling's own (cell) coordinates: the aabb around
        // the inverse images of the four world corners.
        let mut rmin = Vec2::new(f64::MAX, f64::MAX);
        let mut rmax = Vec2::new(f64::MIN, f64::MIN);
        for corner in query.region.corners() {
            let q = inv.transform(corner);
            rmin.x = rmin.x.min(q.x);
            rmin.y = rmin.y.min(q.y);
            rmax.x = rmax.x.max(q.x);
            rmax.y = rmax.y.max(q.y);
        }
        let region = BBox::new(rmin, rmax);
        let center = region.center();
        let mut half: f64 = 0.0;
        for corner in region.corners() {
            half = half.max((corner - center).length());
        }
        if half < 1e-9 {
            return fail("empty view");
        }

        // Pre-generation budget (mirrors penrose_pentagon): final tiles
        // have edge 1, each covers at most A_P cell units², and the
        // tiling must cover the query region — area/A_P is a lower bound
        // on the tile count, checked before any refinement work.
        let s = params.float(SCALE_ID, 1.0);
        if query.region.area() / (A_P * s * s) > MAX_TILES as f64 {
            return fail("too many tiles for this view — zoom in");
        }

        // Smallest deflation depth whose clean disk covers the region
        // (a hair of margin keeps the view off the fjord apexes).
        let mut m: i32 = 1;
        while clean_radius(m) < half * (1.0 + 1e-9) {
            m += 1;
            if m > MAX_M {
                return fail("view too far from the seed — pan back or zoom in");
            }
        }

        // Deflate the single seed m times; the children of every tile
        // tile it exactly, so no overlaps can ever occur. No culling is
        // done mid-run (a merge tile needs both parent atoms, and the
        // full patch is at most ≈ 72k tiles), only at the end.
        let e0 = PHI.powi(2 * m);
        let mut tiles = vec![P1Tile { kind: TK_P, c: center, th: 0.0, e: e0 }];
        for k in 1..=m {
            let e = e0 * IP2.powi(k);
            let mut atoms = Vec::with_capacity(tiles.len() * 11);
            for t in &tiles {
                emit(t, e, &mut atoms);
            }
            let (next, _discarded) = merge(&atoms, e);
            tiles = next;
        }

        let mut out = Vec::with_capacity(tiles.len());
        for t in &tiles {
            let poly = tile_poly(t).transform(&map);
            if poly.aabb().intersects(query.region) {
                out.push(Tile { family: tess_core::tiling::FamilyTag::Penrose, reflected: false,
                    proto: t.kind,
                    seed: t.kind as u32, level: 0,
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

    // Areas of the remaining prototiles / discarded pieces (edge 1).
    const A_D: f64 = S36; // diamond
    const A_G: f64 = A_D / 2.0; // gnomon
    const A_CORE: f64 = 0.769_420_884_293_813_4; // √(φ²−1/4)/2, boat core
    const A_S: f64 = 5.0 * R_P * R_P * S36; // star
    const A_S1C: f64 = A_S - A_G; // star minus one tip
    const A_S2C: f64 = A_S - 2.0 * A_G; // star minus two tips
    const A_B: f64 = A_CORE + 2.0 * A_G; // boat

    fn run_deflations(m: i32) -> Vec<(Vec<P1Tile>, [u64; 4])> {
        let e0 = PHI.powi(2 * m);
        let mut tiles = vec![P1Tile { kind: TK_P, c: Vec2::ZERO, th: 0.0, e: e0 }];
        let mut levels = Vec::new();
        for k in 1..=m {
            let e = e0 * IP2.powi(k);
            let mut atoms = Vec::new();
            for t in &tiles {
                emit(t, e, &mut atoms);
            }
            let (next, disc) = merge(&atoms, e);
            let entry = (next.clone(), disc);
            tiles = next;
            levels.push(entry);
        }
        levels
    }

    /// The four shape constructors reproduce the exact prototile areas.
    #[test]
    fn prototile_areas() {
        let tiles = [
            P1Tile { kind: TK_P, c: Vec2::ZERO, th: 0.0, e: 1.0 },
            P1Tile { kind: TK_S, c: Vec2::ZERO, th: 0.0, e: 1.0 },
            P1Tile { kind: TK_B, c: Vec2::ZERO, th: 0.0, e: 1.0 },
            P1Tile { kind: TK_D, c: Vec2::ZERO, th: 0.0, e: 1.0 },
        ];
        let exp = [A_P, A_S, A_B, A_D];
        for (t, &want) in tiles.iter().zip(&exp) {
            let a = tile_poly(t).area();
            assert!(
                (a - want).abs() <= 1e-9 * want,
                "kind {}: area {a} != {want}",
                t.kind
            );
        }
    }

    /// Deflating a single seed pentagon: at every level the children
    /// plus the discarded gnomons tile the parents exactly (area
    /// balance), with the exact tile-kind counts of the verified
    /// generator, and only G atoms are ever discarded.
    #[test]
    fn deflation_counts_and_conservation() {
        let m = 5;
        let e0 = PHI.powi(2 * m);
        let seed = P1Tile { kind: TK_P, c: Vec2::ZERO, th: 0.0, e: e0 };
        let levels = run_deflations(m);
        // [P, S, B, D] per level, and discarded [G, CORE, S2C, S1C].
        let want_counts = [
            [6usize, 0, 0, 0],
            [36, 0, 0, 5],
            [221, 5, 5, 40],
            [1406, 50, 80, 270],
            [9196, 400, 760, 1805],
        ];
        let want_disc = [
            [5u64, 0, 0, 0],
            [20, 0, 0, 0],
            [80, 0, 0, 0],
            [320, 0, 0, 0],
            [1280, 0, 0, 0],
        ];
        let mut prev = tile_poly(&seed).area();
        for (k, (tiles, disc)) in levels.iter().enumerate() {
            let e = e0 * IP2.powi((k + 1) as i32);
            assert!(
                (tiles[0].e - e).abs() <= 1e-12 * e,
                "L{k}: child edge wrong"
            );
            let mut cnt = [0usize; 4];
            for t in tiles {
                cnt[t.kind as usize] += 1;
            }
            assert_eq!(cnt, want_counts[k], "L{k}: tile counts");
            assert_eq!(*disc, want_disc[k], "L{k}: discarded counts");
            let a_k: f64 = tiles.iter().map(|t| tile_poly(t).area()).sum();
            let d_k = (disc[0] as f64 * A_G
                + disc[1] as f64 * A_CORE
                + disc[2] as f64 * A_S2C
                + disc[3] as f64 * A_S1C)
                * e * e;
            let balance = prev - a_k - d_k;
            assert!(
                balance.abs() <= 1e-9 * prev,
                "L{k}: area balance {balance} (parents {prev}, children {a_k}, \
                 discarded {d_k})"
            );
            prev = a_k;
        }
        // Final edge is 1: the seed is a φ^10 pentagon, 5 deflations.
        assert!((levels[4].0[0].e - 1.0).abs() <= 1e-12);
    }

    /// Gluing invariants of the full level-4 patch (1806 tiles): no
    /// edge is owned by more than two tiles (no overlaps), there are
    /// many shared edges, and no tile appears twice.
    #[test]
    fn gluing_census_and_no_dups() {
        let (tiles, _) = &run_deflations(4)[3];
        let mut census: HashMap<SegKey, u32> = HashMap::new();
        // The key must include the orientation: five B boats legally share
        // one macro center (one per star notch, around an S star), at 72°
        // apart. (kind, c, th) is the tile's full pose.
        let mut seen: HashMap<(u8, i64, i64, i64), u32> = HashMap::new();
        for t in tiles {
            let poly = tile_poly(t);
            let n = poly.pts.len();
            for i in 0..n {
                let p = poly.pts[i];
                let q = poly.pts[(i + 1) % n];
                *census.entry(seg_key(p, q, t.e)).or_insert(0) += 1;
            }
            let ck = (
                t.kind,
                (t.c.x * 1e5).round() as i64,
                (t.c.y * 1e5).round() as i64,
                (t.th * 1e6).round() as i64,
            );
            *seen.entry(ck).or_insert(0) += 1;
        }
        let max_owners = census.values().max().copied().unwrap();
        assert!(
            max_owners <= 2,
            "an edge is owned by {max_owners} tiles (overlap)"
        );
        let shared: u32 = census.values().filter(|&&c| c == 2).count() as u32;
        assert!(shared > 1000, "expected many shared edges, got {shared}");
        let dup = seen.values().filter(|&&c| c > 1).count();
        assert_eq!(dup, 0, "duplicate tiles found");
    }

    /// Deflation nesting: the centroid anchor of every level-4 tile lies
    /// in exactly one level-3 tile (children tile their parents).
    #[test]
    fn refinement_nested() {
        let levels = run_deflations(4);
        let (parents, _) = &levels[2];
        let (children, _) = &levels[3];
        let parent_polys: Vec<Polygon> = parents.iter().map(tile_poly).collect();
        for t in children {
            let n = parent_polys.iter().filter(|p| p.contains(t.c)).count();
            assert_eq!(n, 1, "child (kind {}) anchor in {n} parents", t.kind);
        }
    }
}
