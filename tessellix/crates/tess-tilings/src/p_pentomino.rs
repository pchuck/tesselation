//! P-pentomino tiling — the rep-4 substitution tiling (inflation λ = 2).
//!
//! The prototile is the *P-pentomino*: a 2×2 block of squares with one
//! extra square attached (5 unit cells). It is a rep-4 rep-tile: a P of
//! unit side `s` is dissected into four P's of side `s/2`. Because the
//! P is chiral, the dissection necessarily mixes chiralities: an
//! exhaustive exact enumeration of the inflated P (20-cell region) finds
//! exactly TWO dissections, class multisets {0,1,4,6} and {3,4,4,5}; the
//! lexicographically first, {0,1,4,6}, is the canonical one used here.
//! (Both dissections contain mirror classes — classes 4 and 6 are
//! reflections — so the substitution necessarily uses all eight
//! orientation classes.)
//!
//! The P has eight orientation classes: four rotations times two
//! chiralities (the chiral pair of the P-pentomino). The substitution
//! matrix is doubly stochastic (rows and columns sum to 4), so from a
//! class-uniform seed every class stays equally numerous at every
//! generation.
//!
//! The tiling is **non-edge-to-edge**: in every parent, five child
//! vertices land in the interior of a sibling's maximal edge (asserted
//! for the canonical side-2 parent by a test — the same situation as the
//! chair's and the pinwheel's T-junctions). What the construction
//! guarantees — and the tests assert, exactly, on the integer grid — is
//! that every generation is a *perfect* tiling: every grid cell is
//! covered by exactly one P, counts follow 4^g per class, and the total
//! area never changes.
//!
//! The P-pentomino substitution tiling is limit-periodic (finer and
//! finer levels repeat with ever-larger periods) and aperiodic. Emission
//! follows the repo's finite-depth approximant convention (see
//! `chair.rs`): the plane is seeded with 5×8 blocks — each holding eight
//! side-`2^g` P's, one of each orientation class (one of the 62 exact
//! tilings of the 5×8 rectangle; this one is the only family that uses
//! all eight classes) — on a lattice anchored through the origin, and
//! every P is deflated exactly `g` times, so the final P's land on unit
//! scale (area 5·scale²) regardless of the view. The seed scale is
//! quantized to powers of 2 measured from the region *extent*, not its
//! position, so panning and zooming within an octave only move the
//! culling window over one fixed global tiling (bit-identical tiles);
//! crossing an octave selects a finer global tiling with a doubled
//! period — the finite-depth shadow of limit periodicity.
//!
//! Pattern coloring: the tiling is monohedral, so `proto` is always 0;
//! `seed` is the orientation class + 1 (1..=8), which makes the eight
//! classes visible in "pattern" mode.

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// The eight re-anchored outlines (orientation class, bounding-box
/// origin at (0,0)), all CCW, in unit-side coordinates. Each class is a
/// rigid image of class 0; rotation classes 1..=3 are 90°·c images of
/// class 0 with vertex index correspondence i↔i, and mirror classes
/// 4..=7 are 90°·(c−4) images of class 4 with i↔i (class 4 itself is
/// the mirror of class 0, reversed to keep CCW). This keeps the vertex
/// lists aligned for tess-gui's per-prototile triangulation cache: the
/// four rotation classes share one vertex layout and the four mirror
/// classes share its mirror image. Signed 2× shoelace area 10 (area 5).
const OUTLINES: [[[i64; 2]; 6]; 8] = [
    [[0, 0], [3, 0], [3, 1], [2, 1], [2, 2], [0, 2]],
    [[2, 0], [2, 3], [1, 3], [1, 2], [0, 2], [0, 0]],
    [[3, 2], [0, 2], [0, 1], [1, 1], [1, 0], [3, 0]],
    [[0, 3], [0, 0], [1, 0], [1, 1], [2, 1], [2, 3]],
    [[3, 2], [1, 2], [1, 1], [0, 1], [0, 0], [3, 0]],
    [[0, 3], [0, 1], [1, 1], [1, 0], [2, 0], [2, 3]],
    [[0, 0], [2, 0], [2, 1], [3, 1], [3, 2], [0, 2]],
    [[2, 0], [2, 2], [1, 2], [1, 3], [0, 3], [0, 0]],
];

/// The five unit squares of each orientation class (unit-side
/// coordinates, bounding-box origin at (0,0)).
#[cfg(test)]
const CELL_OFFSETS: [[(i64, i64); 5]; 8] = [
    [(0, 0), (1, 0), (2, 0), (0, 1), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1), (1, 2)],
    [(0, 1), (1, 0), (1, 1), (2, 0), (2, 1)],
    [(0, 0), (0, 1), (0, 2), (1, 1), (1, 2)],
    [(0, 0), (1, 0), (2, 0), (1, 1), (2, 1)],
    [(0, 1), (0, 2), (1, 0), (1, 1), (1, 2)],
    [(0, 0), (1, 0), (0, 1), (1, 1), (2, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2)],
];

/// Substitution rule per parent class: the four children as
/// (child class, dx, dy); the child's bounding-box origin is
/// (parent.x + dx·(s/2), parent.y + dy·(s/2)). The table encodes the
/// full geometry (including the per-class bounding boxes), so
/// `Pent::children` needs no rotation math. Verified by exact cell
/// census for all eight parents at parent scale 2 and 4.
const CHILD_RULE: [[(u8, i64, i64); 4]; 8] = [
    [(0, 0, 0), (1, 2, 1), (4, 3, 0), (6, 0, 2)],
    [(1, 2, 0), (2, 0, 2), (5, 2, 3), (7, 0, 0)],
    [(2, 3, 2), (3, 2, 0), (6, 0, 2), (4, 3, 0)],
    [(3, 0, 3), (0, 1, 2), (7, 0, 0), (5, 2, 3)],
    [(4, 3, 0), (7, 2, 1), (0, 0, 0), (2, 3, 2)],
    [(5, 2, 3), (4, 0, 2), (1, 2, 0), (3, 0, 3)],
    [(6, 0, 2), (5, 2, 0), (2, 3, 2), (0, 0, 0)],
    [(7, 0, 0), (6, 1, 2), (3, 0, 3), (1, 2, 0)],
];

/// The seed block: eight unit P's (orientation class, unit origin)
/// tiling a 5×8 rectangle, one of each class. Of the 62 exact tilings
/// of the 5×8 rectangle by eight P's, this is one of the (few) that use
/// all eight orientation classes — required for class-uniform growth.
const SEED_UNITS: [(u8, i64, i64); 8] = [
    (0, 0, 0),
    (1, 3, 0),
    (2, 0, 4),
    (3, 2, 1),
    (4, 2, 6),
    (5, 3, 3),
    (6, 0, 6),
    (7, 0, 2),
];

/// Seed block size in unit-side coordinates (5 wide, 8 tall).
const SEED_W: i64 = 5;
const SEED_H: i64 = 8;

/// View extent (cell units) at which the seed octave increments
/// (one step above the 5×8 block diagonal of √89 ≈ 9.4).
const EXTENT_UNIT: f64 = 10.0;

/// A P-pentomino on the integer grid: orientation class `c` (0..=7),
/// bounding-box lower-left `(x, y)`, unit side `s` (a power of two ≥
/// 1). Even classes have a 3s×2s bounding box, odd classes 2s×3s. Every
/// coordinate in the construction is an integer, so refinement and
/// culling are exact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pent {
    c: u8,
    x: i64,
    y: i64,
    s: i64,
}

impl Pent {
    /// Polygon outline in cell space (exact: all inputs are integers
    /// well inside f64's exact-integer range). The class's re-anchored
    /// outline, scaled by `s` and translated to `(x, y)`.
    fn poly(&self) -> Polygon {
        let s = self.s;
        let pts: Vec<Vec2> = OUTLINES[self.c as usize]
            .iter()
            .map(|p| Vec2::new((self.x + p[0] * s) as f64, (self.y + p[1] * s) as f64))
            .collect();
        Polygon::from_slice(&pts)
    }

    /// The class's bounding box: 3s×2s for even classes, 2s×3s for odd
    /// classes.
    fn aabb(&self) -> BBox {
        let (w, h) = if self.c % 2 == 0 {
            (3 * self.s, 2 * self.s)
        } else {
            (2 * self.s, 3 * self.s)
        };
        BBox::new(
            Vec2::new(self.x as f64, self.y as f64),
            Vec2::new((self.x + w) as f64, (self.y + h) as f64),
        )
    }

    /// The four substitution children (unit side s/2; requires s even).
    /// `CHILD_RULE` encodes the whole dissection — child classes and
    /// bounding-box offsets — for each parent class; they tile the
    /// parent exactly (verified by an exact cell census in the tests,
    /// for all eight classes).
    fn children(&self) -> [Pent; 4] {
        let s = self.s;
        debug_assert!(s % 2 == 0, "children need an even unit side");
        let half = s / 2;
        let rule = CHILD_RULE[self.c as usize];
        [0, 1, 2, 3].map(|k| Pent {
            c: rule[k].0,
            x: self.x + rule[k].1 * half,
            y: self.y + rule[k].2 * half,
            s: half,
        })
    }

    /// The P's unit grid cells (test helper): its five squares of side
    /// `s`, cell by cell.
    #[cfg(test)]
    fn cells(&self) -> Vec<(i64, i64)> {
        let mut v = Vec::with_capacity((5 * self.s * self.s) as usize);
        for (ox, oy) in CELL_OFFSETS[self.c as usize] {
            for i in 0..self.s {
                for j in 0..self.s {
                    v.push((self.x + ox * self.s + i, self.y + oy * self.s + j));
                }
            }
        }
        v
    }
}

/// Seed P's: the 5×8 blocks (each holding the eight `SEED_UNITS` P's
/// at scale 2^g) on the block lattice anchored through the origin,
/// covering `region`.
fn seed_pents(region: &BBox, g: u32) -> Vec<Pent> {
    let u = 1i64 << g;
    let bw = (SEED_W * u) as f64;
    let bh = (SEED_H * u) as f64;
    let i0 = (region.min.x / bw).floor() as i64;
    let j0 = (region.min.y / bh).floor() as i64;
    let i1 = (region.max.x / bw).floor() as i64;
    let j1 = (region.max.y / bh).floor() as i64;
    let mut v = Vec::new();
    for j in j0..=j1 {
        for i in i0..=i1 {
            let (bx, by) = (i * SEED_W * u, j * SEED_H * u);
            for (c, ux, uy) in SEED_UNITS {
                v.push(Pent {
                    c,
                    x: bx + ux * u,
                    y: by + uy * u,
                    s: u,
                });
            }
        }
    }
    v
}

/// One deflation step: replace every P by its four children (scale 1/2),
/// keeping only those whose bounding box intersects `region`.
/// Sound: children never leave their parent, so a final tile
/// intersecting the region has its full ancestor chain present —
/// nothing needed is ever dropped.
fn deflate_step(pents: &[Pent], region: &BBox) -> Vec<Pent> {
    let mut out = Vec::with_capacity(pents.len() * 4);
    for p in pents {
        for kid in p.children() {
            if kid.aabb().intersects(*region) {
                out.push(kid);
            }
        }
    }
    out
}

/// P-pentomino tiling (aperiodic, limit-periodic rep-4 substitution).
pub struct PentPentominoTiling;

impl Tiling for PentPentominoTiling {
    fn id(&self) -> &'static str {
        "p-pentomino"
    }

    fn name(&self) -> &'static str {
        "P-pentomino (rep-4 reptile, limit-periodic)"
    }

    fn description(&self) -> &'static str {
        "Aperiodic limit-periodic P-pentomino rep-4 substitution tiling; λ = 2; tiles in 8 chiral orientation classes; non-edge-to-edge."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Aperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None)
            .with_notes("aperiodic; limit-periodic; tiles in 8 chiral orientation classes; non-edge-to-edge");
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Region in the tiling's own (cell) coordinates: the axis-aligned
        // box around the inverse image of the four world corners.
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
        // Final P's have area exactly 5 in cell space (five unit squares;
        // the octave-anchored deflation always lands on unit scale), and
        // the cell-space AABB of a rotated world box is at most ~2× the
        // world area, so this bounds the tile count before any
        // refinement work.
        if region.area() > MAX_TILES as f64 * 2.0 {
            return fail("too many tiles for this view — zoom in");
        }

        // View-independent patch: quantize the seed scale to powers of 2
        // (one octave per deflation depth) measured from the region
        // *extent* (not its position — panning must not change the
        // octave), and anchor the 5×8 seed blocks to the lattice through
        // the origin. The emitted tiles are the restriction of one
        // global refined tiling to the query region: panning and
        // resizing only move the culling window; the pattern changes
        // only when the extent crosses a power-of-two octave (the depth
        // increments and a finer global tiling is selected). Deflating
        // exactly `depth` times lands the final P's on unit scale.
        let extent = (rmax - rmin).length();
        if !extent.is_finite() || extent <= 1e-9 {
            return fail("empty view");
        }
        let depth = ((extent / EXTENT_UNIT).log2().ceil().max(0.0) as u32).min(40);

        let mut pents = seed_pents(&region, depth);
        for _ in 0..depth {
            pents = deflate_step(&pents, &region);
        }
        if pents.is_empty() {
            return fail("empty view");
        }

        // Emit the P's as tiles. The tiling is monohedral (one
        // prototile), so `proto` is always 0; `seed` is the orientation
        // class + 1 (1..=8) — see the module docs — which makes the
        // eight classes visible in "pattern" mode.
        let mut tiles = Vec::with_capacity(pents.len());
        for p in &pents {
            let poly = p.poly().transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto: 0,
                    seed: p.c as u32 + 1,
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
mod tests {
    use super::*;

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

    /// Exact cell census of a P patch.
    fn census(pents: &[Pent]) -> std::collections::BTreeMap<(i64, i64), u32> {
        let mut m = std::collections::BTreeMap::new();
        for p in pents {
            for cell in p.cells() {
                *m.entry(cell).or_insert(0) += 1;
            }
        }
        m
    }

    /// Every orientation class emits a CCW P-pentomino of the expected
    /// shape: signed 2× area +10s² (CCW and area both) and the vertex
    /// set of the class's re-anchored outline.
    #[test]
    fn emitted_outlines_are_pents() {
        for c in 0..8u8 {
            let p = Pent {
                c,
                x: 3,
                y: -2,
                s: 5,
            };
            let pts = &p.poly().pts;
            assert_eq!(pts.len(), 6, "c{c}: vertex count");
            let a2 = signed_area2(pts);
            assert!(
                (a2 - 10.0 * 25.0).abs() < 1e-9,
                "c{c}: signed 2×area {a2} != 250 (want CCW area-5 P)"
            );
            let want: std::collections::BTreeSet<(i64, i64)> = OUTLINES[c as usize]
                .iter()
                .map(|q| (3 + 5 * q[0], -2 + 5 * q[1]))
                .collect();
            let got: std::collections::BTreeSet<(i64, i64)> = pts
                .iter()
                .map(|q| (q.x.round() as i64, q.y.round() as i64))
                .collect();
            assert_eq!(got, want, "c{c}: emitted vertex set");
        }
    }

    /// The emitted vertex lists are aligned for tess-gui's
    /// per-prototile triangulation cache: rotation classes 1..=3 are
    /// 90°·c rotations of class 0 with edge index correspondence i↔i,
    /// and mirror classes 5..=7 are 90°·(c−4) rotations of class 4 with
    /// i↔i (class 4 is the mirror of class 0 with indices reversed).
    /// Each family therefore shares one vertex layout, so one cached
    /// ear-clip per family is valid.
    #[test]
    fn poly_vertex_lists_are_class_aligned() {
        let s = 3;
        let outline = |c: u8| {
            let p = Pent { c, x: 0, y: 0, s };
            let pts = p.poly().pts;
            let mut edges = Vec::with_capacity(6);
            for i in 0..6 {
                let e = pts[(i + 1) % 6] - pts[i];
                edges.push((e.x, e.y));
            }
            edges
        };
        let e0 = outline(0);
        let e4 = outline(4);
        for c in 1..=3u8 {
            let ec = outline(c);
            for i in 0..6 {
                let (x, y) = e0[i];
                let (rx, ry) = match c {
                    1 => (-y, x),
                    2 => (-x, -y),
                    _ => (y, -x),
                };
                assert!(
                    (ec[i].0 - rx).abs() < 1e-9 && (ec[i].1 - ry).abs() < 1e-9,
                    "c{c}: edge {i} is not the 90°·{c} rotation of the \
                     canonical edge — triangulation cache would misalign"
                );
            }
        }
        for c in 5..=7u8 {
            let ec = outline(c);
            let k = (c - 4) as u8;
            for i in 0..6 {
                let (x, y) = e4[i];
                let (rx, ry) = match k {
                    1 => (-y, x),
                    2 => (-x, -y),
                    _ => (y, -x),
                };
                assert!(
                    (ec[i].0 - rx).abs() < 1e-9 && (ec[i].1 - ry).abs() < 1e-9,
                    "c{c}: edge {i} is not the 90°·{k} rotation of the \
                     mirror edge — triangulation cache would misalign"
                );
            }
        }
        // Class 4 is the mirror of class 0 with the index
        // correspondence i ↔ (4−i) mod 6 (reversed to stay CCW): each
        // mirror edge is the x-axis reflection of the corresponding
        // canonical edge.
        for i in 0..6 {
            let j = ((4 - i as i32).rem_euclid(6)) as usize;
            let (x, y) = e0[j];
            assert!(
                (e4[i].0 - x).abs() < 1e-9 && (e4[i].1 + y).abs() < 1e-9,
                "c4: edge {i} is not the mirror of canonical edge {j}"
            );
        }
    }

    /// The four children of a P exactly tile the parent (deflation
    /// view), for every class: exact integer cell census — every parent
    /// cell covered exactly once, no cell outside.
    #[test]
    fn substitution_tiles_parent_exactly() {
        for c in 0..8u8 {
            let parent = Pent {
                c,
                x: 3,
                y: -2,
                s: 8,
            };
            let want: std::collections::BTreeSet<(i64, i64)> =
                parent.cells().into_iter().collect();
            assert_eq!(want.len(), 5 * 8 * 8, "c{c}: parent cell count");
            let kids = parent.children();
            let cov = census(&kids);
            assert_eq!(cov.len(), want.len(), "c{c}: covered cell count");
            for (cell, n) in &cov {
                assert_eq!(*n, 1, "c{c}: cell {cell:?} covered {n} times");
                assert!(
                    want.contains(cell),
                    "c{c}: child cell {cell:?} outside parent"
                );
            }
        }
    }

    /// The seed block: eight P's, one of each orientation class, tiling
    /// the 5×8 rectangle exactly.
    #[test]
    fn seed_block_tiles_rectangle() {
        let seed: Vec<Pent> = SEED_UNITS
            .iter()
            .map(|&(c, ux, uy)| Pent {
                c,
                x: ux,
                y: uy,
                s: 1,
            })
            .collect();
        let cov = census(&seed);
        assert_eq!(cov.len(), 40, "seed: cell count");
        assert!(cov.values().all(|&n| n == 1), "seed: overlap/gap");
        for x in 0..5i64 {
            for y in 0..8i64 {
                assert!(cov.contains_key(&(x, y)), "seed: missing cell ({x},{y})");
            }
        }
        let mut classes: Vec<u8> = seed.iter().map(|p| p.c).collect();
        classes.sort_unstable();
        assert_eq!(classes, [0, 1, 2, 3, 4, 5, 6, 7], "seed must use all eight classes");
    }

    /// Deflating the seed block: counts follow 8·4^g, the eight
    /// orientation classes stay equally numerous (4^g each — the
    /// substitution matrix is doubly stochastic), and the total 2× area
    /// never changes (the patch always tiles the same 5u×8u
    /// rectangle). Final P's land exactly on unit scale (side 1).
    #[test]
    fn refined_counts_and_area_law() {
        let g = 4u32;
        let u = 1i64 << g;
        let mut pents: Vec<Pent> = SEED_UNITS
            .iter()
            .map(|&(c, ux, uy)| Pent {
                c,
                x: ux * u,
                y: uy * u,
                s: u,
            })
            .collect();
        let area2: i64 = pents.iter().map(|p| 10 * p.s * p.s).sum();
        assert_eq!(area2, 80 * u * u, "seed block 2×area");
        for gen in 1..=g {
            let mut next = Vec::with_capacity(pents.len() * 4);
            for p in &pents {
                next.extend(p.children());
            }
            pents = next;
            assert_eq!(pents.len(), 8usize * 4usize.pow(gen), "gen {gen}: count");
            let mut counts = [0usize; 8];
            for p in &pents {
                counts[p.c as usize] += 1;
            }
            assert_eq!(
                counts,
                [4usize.pow(gen); 8],
                "gen {gen}: class counts not uniform"
            );
            let a2: i64 = pents.iter().map(|p| 10 * p.s * p.s).sum();
            assert_eq!(a2, area2, "gen {gen}: total 2×area changed");
        }
        assert!(pents.iter().all(|p| p.s == 1), "final scale must be 1");
    }

    /// The deflated seed block is a perfect tiling of its 5u×8u
    /// rectangle at every generation: exact integer cell census, no
    /// gaps, no overlaps.
    #[test]
    fn refined_patch_covers_rectangle_exactly() {
        let g = 4u32;
        let u = 1i64 << g;
        let mut pents: Vec<Pent> = SEED_UNITS
            .iter()
            .map(|&(c, ux, uy)| Pent {
                c,
                x: ux * u,
                y: uy * u,
                s: u,
            })
            .collect();
        for gen in 1..=g {
            let mut next = Vec::with_capacity(pents.len() * 4);
            for p in &pents {
                next.extend(p.children());
            }
            pents = next;
            let cov = census(&pents);
            assert_eq!(cov.len() as i64, 40 * u * u, "gen {gen}: cell count");
            assert!(cov.values().all(|&n| n == 1), "gen {gen}: overlap/gap");
        }
    }

    /// T-junctions are inherent to the P substitution: in the canonical
    /// side-2 parent (class 0), the lower-left child's reentrant vertex
    /// (3,1) lies strictly inside the center child's bottom edge
    /// (2,1)–(4,1) — five such junctions per parent at every scale. The
    /// tiling is therefore non-edge-to-edge, the same situation as the
    /// chair's T-junctions.
    #[test]
    fn t_junctions_are_inherent() {
        let parent = Pent {
            c: 0,
            x: 0,
            y: 0,
            s: 2,
        };
        let kids = parent.children();
        let ll = kids[0];
        let center = kids[1];
        assert_eq!((ll.c, ll.x, ll.y, ll.s), (0, 0, 0, 1), "lower-left child");
        assert_eq!((center.c, center.x, center.y, center.s), (1, 2, 1, 1), "center child");
        // ll's reentrant vertex (3,1) strictly inside center's bottom
        // edge (2,1)-(4,1).
        let v = v2(3.0, 1.0);
        let (a, b) = (v2(2.0, 1.0), v2(4.0, 1.0));
        let ab = b - a;
        assert!(ab.cross(v - a).abs() < 1e-12, "not collinear");
        let t = (v - a).dot(ab) / ab.dot(ab);
        assert!(t > 1e-9 && t < 1.0 - 1e-9, "vertex not strictly inside edge");
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

    /// The patch is view-independent within a power-of-two octave:
    /// growing, panning, or zooming the view only moves the culling
    /// window over one fixed, origin-anchored refined tiling. (All the
    /// extents lie in (10, 20] — the octave [EXTENT_UNIT,
    /// EXTENT_UNIT·2].)
    #[test]
    fn patch_is_stable_under_view_changes_within_an_octave() {
        let t = PentPentominoTiling;
        let p = ParamSet::default_for(t.params());
        let base = t.generate(
            &p,
            &Query { region: BBox::new(v2(-7.5, -5.625), v2(7.5, 5.625)) },
        );
        assert!(base.message.is_none(), "{:?}", base.message);
        let grown = t.generate(
            &p,
            &Query { region: BBox::new(v2(-7.9, -5.9), v2(7.9, 5.9)) },
        );
        let panned = t.generate(
            &p,
            &Query { region: BBox::new(v2(-6.5, -4.6), v2(8.5, 6.6)) },
        );
        let zoomed = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-7.5 / 1.3, -5.625 / 1.3), v2(7.5 / 1.3, 5.625 / 1.3)),
            },
        );

        let base_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            base.tiles.iter().map(tile_key).collect();
        let zoomed_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            zoomed.tiles.iter().map(tile_key).collect();
        let grown_keys: std::collections::HashSet<(u32, Vec<u64>)> =
            grown.tiles.iter().map(tile_key).collect();
        for k in &base_keys {
            assert!(grown_keys.contains(k), "grow: tile changed when the view grew");
        }
        for k in &zoomed_keys {
            assert!(base_keys.contains(k), "zoom: tile changed when the view changed");
        }
        let overlap = t.generate(
            &p,
            &Query {
                region: BBox::new(v2(-6.45, -4.55), v2(7.4, 5.55)),
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
        assert!(!overlap.tiles.is_empty(), "pan: overlap view produced no tiles");
    }

    /// The octave-anchored deflation always lands exactly on unit scale:
    /// every emitted P has area 5·scale² (5 at default params), and all
    /// eight orientation classes occur in a default view.
    #[test]
    fn emitted_tiles_are_unit_scale() {
        let t = PentPentominoTiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(
            &p,
            &Query { region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)) },
        );
        assert!(r.message.is_none(), "{:?}", r.message);
        assert!(!r.tiles.is_empty());
        let mut classes = std::collections::BTreeSet::new();
        for tile in &r.tiles {
            let a = tile.poly.area();
            assert!(
                (a - 5.0).abs() < 1e-9,
                "tile area {a} != 5 (unit P at default params)"
            );
            classes.insert(tile.seed);
        }
        let want: std::collections::BTreeSet<u32> = [1u32, 2, 3, 4, 5, 6, 7, 8].into_iter().collect();
        assert_eq!(classes, want, "want all eight orientation classes");
    }
}
