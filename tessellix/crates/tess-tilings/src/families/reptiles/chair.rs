//! Chair tiling — the L-tromino substitution tiling (inflation λ = 2).
//!
//! The prototile is the *chair* (L-tromino): a 2×2 block of squares with
//! one quadrant removed. It is a rep-4 rep-tile: a chair of square side
//! `s` is dissected into four chairs of side `s/2`. In the parent's own
//! frame the children sit at bounding-box origins (0,0), (s/2,s/2),
//! (s,0), (0,s) with orientation deltas 0, 0, 1, 3; a parent of
//! orientation `o` rotates that arrangement by 90°·`o` CCW and re-aligns
//! it to the parent's bounding box. An exhaustive enumeration of all
//! L-tromino tilings of the inflated chair (exact integer arithmetic,
//! 12-cell region) shows this dissection is the UNIQUE one, so the rule
//! is forced — there are no choices to get wrong.
//!
//! The tiling is **non-edge-to-edge**: the forced dissection lands a
//! child's vertex in the interior of a sibling's edge at every scale
//! (e.g. in the canonical side-2 parent, the lower-left child's
//! reentrant vertex (2,1) lies in the middle of the center child's
//! bottom edge — asserted by a test). This is inherent to the chair
//! substitution, the same situation as the pinwheel's long-leg
//! T-junctions. What the construction guarantees — and the tests assert,
//! exactly, on the integer grid — is that every generation is a
//! *perfect* tiling: every grid cell is covered by exactly one chair,
//! chair counts follow 4^(g+1), the four orientations stay equally
//! numerous, and the total area never changes.
//!
//! The chair substitution tiling is limit-periodic (finer and finer
//! levels repeat with ever-larger periods) and aperiodic. Emission
//! follows the repo's finite-depth approximant convention (see
//! `pinwheel.rs`): the plane is seeded with 4×3 blocks — each holding
//! four side-`2^g` chairs, one per orientation, in the one arrangement
//! (of the four possible 4×3 L-tromino tilings that use all four
//! orientations — verified against the exhaustive enumeration) — on a
//! lattice anchored through the origin, and every chair is deflated
//! exactly `g` times, so the final chairs land on unit scale (area
//! 3·scale²) regardless of the view. The seed scale is quantized to
//! powers of 2 measured from the region *extent*, not its position, so
//! panning and zooming within an octave only move the culling window
//! over one fixed global tiling (bit-identical tiles); crossing an
//! octave selects a finer global tiling with a doubled period — the
//! finite-depth shadow of limit periodicity.
//!
//! Pattern coloring: the tiling is monohedral, so `proto` is always 0;
//! `seed` is the chair's orientation + 1 (1..=4), which makes the four
//! orientation classes visible in "pattern" mode.

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// The canonical chair outline (orientation o=0: missing upper-right
/// quadrant), CCW, in square-side units (bounding box [0,2]²). 2×
/// shoelace area 6 (area 3), perimeter 8. Every orientation is emitted
/// as the image of this outline under the 90°·o CCW rotation, re-anchored
/// to the tile's bounding box — see `Chair::poly`.
const CANONICAL_OUTLINE: [[i64; 2]; 6] =
    [[0, 0], [2, 0], [2, 1], [1, 1], [1, 2], [0, 2]];

/// The three unit squares of each orientation (square-side units, bbox
/// origin at (0,0)).
#[cfg(test)]
const SQUARE_OFFSETS: [[[i64; 2]; 3]; 4] = [
    [[0, 0], [1, 0], [0, 1]], // o=0: LL LR UL
    [[1, 0], [1, 1], [0, 0]], // o=1: LR UR LL
    [[1, 1], [0, 1], [1, 0]], // o=2: UR UL LR
    [[0, 0], [0, 1], [1, 1]], // o=3: LL UL UR
];

/// The seed block: four side-u chairs (orientation, unit origin) tiling
/// a 4u×3u rectangle, one per orientation.
const SEED_UNITS: [(u8, i64, i64); 4] = [(1, 0, 0), (0, 2, 0), (2, 2, 1), (3, 0, 1)];

/// View extent (cell units) at which the seed octave increments.
const EXTENT_UNIT: f64 = 6.0;

/// A chair on the integer grid: orientation `o` (which quadrant of its
/// 2s×2s bounding box is missing), bounding-box lower-left `(x, y)`,
/// square side `s` (a power of two ≥ 1). Every coordinate in the
/// construction is an integer, so refinement and culling are exact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Chair {
    o: u8,
    x: i64,
    y: i64,
    s: i64,
}

impl Chair {
    /// Polygon outline in cell space (exact: all inputs are integers
    /// well inside f64's exact-integer range). Every orientation is
    /// emitted as the image of `CANONICAL_OUTLINE` under the 90°·o CCW
    /// rotation, re-anchored so the bounding box sits at (x, y). This
    /// keeps the vertex correspondence i↔i across the four orientation
    /// classes — tess-gui's per-prototile triangulation cache transfers
    /// index triples between copies of a prototile only for vertex
    /// lists aligned like this (the four outlines start at different
    /// structural corners, so per-orientation outline tables would
    /// misalign the cached triangles and bleed fill across tile walls;
    /// caught by the gui cache tests).
    fn poly(&self) -> Polygon {
        let s = self.s;
        // Translation mapping R_o(bbox [0,2s]²) back onto [0,2s]².
        let (sx, sy) = match self.o {
            0 => (0, 0),
            1 => (2 * s, 0),
            2 => (2 * s, 2 * s),
            _ => (0, 2 * s),
        };
        let pts: Vec<Vec2> = CANONICAL_OUTLINE
            .iter()
            .map(|p| {
                let (rx, ry) = match self.o {
                    0 => (p[0], p[1]),
                    1 => (-p[1], p[0]),
                    2 => (-p[0], -p[1]),
                    _ => (p[1], -p[0]),
                };
                Vec2::new(
                    (self.x + sx + rx * s) as f64,
                    (self.y + sy + ry * s) as f64,
                )
            })
            .collect();
        Polygon::from_slice(&pts)
    }

    /// The chair's 2s×2s bounding box.
    fn aabb(&self) -> BBox {
        BBox::new(
            Vec2::new(self.x as f64, self.y as f64),
            Vec2::new(
                (self.x + 2 * self.s) as f64,
                (self.y + 2 * self.s) as f64,
            ),
        )
    }

    /// The four substitution children (square side s/2; requires s
    /// even). Rule: in the parent's own frame the children sit at bbox
    /// origins (0,0), (s/2,s/2), (s,0), (0,s) with orientation deltas
    /// 0,0,1,3; the whole arrangement is rotated by 90°·o CCW and
    /// re-aligned to the parent's bounding box. They tile the parent
    /// exactly (verified by an exact cell census in the tests, for all
    /// four orientations).
    fn children(&self) -> [Chair; 4] {
        let s = self.s;
        debug_assert!(s % 2 == 0, "children need an even square side");
        let half = s / 2;
        // Translation mapping R_o(parent bbox [0,2s]²) back onto [0,2s]².
        let (sx, sy) = match self.o {
            0 => (0, 0),
            1 => (2 * s, 0),
            2 => (2 * s, 2 * s),
            _ => (0, 2 * s),
        };
        let locals = [(0i64, 0i64), (half, half), (s, 0), (0, s)];
        let deltas = [0u8, 0, 1, 3];
        [0, 1, 2, 3].map(|k| {
            let (lx, ly) = locals[k];
            // Lower-left of the 90°·o-rotated child bounding box (the
            // child's bbox is s wide — the parent's square side).
            let (rx, ry) = match self.o {
                0 => (lx, ly),
                1 => (-ly - s, lx),
                2 => (-lx - s, -ly - s),
                _ => (ly, -lx - s),
            };
            Chair {
                o: (self.o + deltas[k]) % 4,
                x: self.x + sx + rx,
                y: self.y + sy + ry,
                s: half,
            }
        })
    }

    /// The chair's unit grid cells (test helper): its three squares of
    /// side `s`, cell by cell.
    #[cfg(test)]
    fn cells(&self) -> Vec<(i64, i64)> {
        let mut v = Vec::with_capacity((3 * self.s * self.s) as usize);
        for sq in SQUARE_OFFSETS[self.o as usize] {
            for i in 0..self.s {
                for j in 0..self.s {
                    v.push((self.x + sq[0] * self.s + i, self.y + sq[1] * self.s + j));
                }
            }
        }
        v
    }
}

/// Seed chairs: the 4×3 blocks (each holding the four `SEED_UNITS`
/// chairs at scale 2^g) on the block lattice anchored through the
/// origin, covering `region`.
fn seed_chairs(region: &BBox, g: u32) -> Vec<Chair> {
    let u = 1i64 << g;
    let bw = (4 * u) as f64;
    let bh = (3 * u) as f64;
    let i0 = (region.min.x / bw).floor() as i64;
    let j0 = (region.min.y / bh).floor() as i64;
    let i1 = (region.max.x / bw).floor() as i64;
    let j1 = (region.max.y / bh).floor() as i64;
    let mut v = Vec::new();
    for j in j0..=j1 {
        for i in i0..=i1 {
            let (bx, by) = (i * 4 * u, j * 3 * u);
            for (o, ux, uy) in SEED_UNITS {
                v.push(Chair {
                    o,
                    x: bx + ux * u,
                    y: by + uy * u,
                    s: u,
                });
            }
        }
    }
    v
}

/// One deflation step: replace every chair by its four children (scale
/// 1/2), keeping only those whose bounding box intersects `region`.
/// Sound: children never leave their parent, so a final tile
/// intersecting the region has its full ancestor chain present —
/// nothing needed is ever dropped.
fn deflate_step(chairs: &[Chair], region: &BBox) -> Vec<Chair> {
    let mut out = Vec::with_capacity(chairs.len() * 4);
    for c in chairs {
        for kid in c.children() {
            if kid.aabb().intersects(*region) {
                out.push(kid);
            }
        }
    }
    out
}

/// Chair tiling (aperiodic, limit-periodic L-tromino substitution).
pub struct ChairTiling;

impl Tiling for ChairTiling {
    fn id(&self) -> &'static str {
        "chair"
    }

    fn name(&self) -> &'static str {
        "Chair (L-tromino)"
    }

    fn description(&self) -> &'static str {
        "Aperiodic limit-periodic L-tromino (chair) substitution tiling; λ = 2; tiles in 4 orientations."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Nonperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None)
            .with_notes("aperiodic; limit-periodic; tiles in 4 orientations; non-edge-to-edge");
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
        // Final chairs have area exactly 3 in cell space (three unit
        // squares; the octave-anchored deflation always lands on unit
        // scale), and the cell-space AABB of a rotated world box is at
        // most ~2× the world area, so this bounds the tile count before
        // any refinement work.
        if region.area() > MAX_TILES as f64 * 2.0 {
            return fail("too many tiles for this view — zoom in");
        }

        // View-independent patch: quantize the seed scale to powers of 2
        // (one octave per deflation depth) measured from the region
        // *extent* (not its position — panning must not change the
        // octave), and anchor the 4×3 seed blocks to the lattice through
        // the origin. The emitted tiles are the restriction of one
        // global refined tiling to the query region: panning and
        // resizing only move the culling window; the pattern changes
        // only when the extent crosses a power-of-two octave (the depth
        // increments and a finer global tiling is selected). Deflating
        // exactly `depth` times lands the final chairs on unit scale.
        let extent = (rmax - rmin).length();
        if !extent.is_finite() || extent <= 1e-9 {
            return fail("empty view");
        }
        let depth = ((extent / EXTENT_UNIT).log2().ceil().max(0.0) as u32).min(40);

        let mut chairs = seed_chairs(&region, depth);
        for _ in 0..depth {
            chairs = deflate_step(&chairs, &region);
        }
        if chairs.is_empty() {
            return fail("empty view");
        }

        // Emit the chairs as tiles. The tiling is monohedral (one
        // prototile), so `proto` is always 0; `seed` is the chair's
        // orientation + 1 (1..=4) — see the module docs — which makes
        // the four orientation classes visible in "pattern" mode.
        let mut tiles = Vec::with_capacity(chairs.len());
        for c in &chairs {
            let poly = c.poly().transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto: 0,
                    seed: c.o as u32 + 1,
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

    /// Expected chair outlines per orientation (missing quadrant), as
    /// vertex SETS in square-side units — the emitted order is the
    /// canonical rotation, so sets are compared, not sequences.
    const EXPECTED_SETS: [[[i64; 2]; 6]; 4] = [
        [[0, 0], [2, 0], [2, 1], [1, 1], [1, 2], [0, 2]], // o=0 missing UR
        [[0, 0], [2, 0], [2, 2], [1, 2], [1, 1], [0, 1]], // o=1 missing UL
        [[0, 1], [1, 1], [1, 0], [2, 0], [2, 2], [0, 2]], // o=2 missing LL
        [[0, 0], [1, 0], [1, 1], [2, 1], [2, 2], [0, 2]], // o=3 missing LR
    ];

    /// Exact cell census of a chair patch.
    fn census(chairs: &[Chair]) -> std::collections::BTreeMap<(i64, i64), u32> {
        let mut m = std::collections::BTreeMap::new();
        for c in chairs {
            for cell in c.cells() {
                *m.entry(cell).or_insert(0) += 1;
            }
        }
        m
    }

    /// Every orientation emits a CCW L-tromino of the expected shape:
    /// signed 2× area +6s² (CCW and area both), and the vertex set of
    /// the orientation's outline.
    #[test]
    fn emitted_outlines_are_chairs() {
        for o in 0..4u8 {
            let c = Chair { o, x: 3, y: -2, s: 5 };
            let pts = &c.poly().pts;
            assert_eq!(pts.len(), 6, "o{o}: vertex count");
            let a2 = signed_area2(pts);
            assert!(
                (a2 - 6.0 * 25.0).abs() < 1e-9,
                "o{o}: signed 2×area {a2} != 150 (want CCW area-3 chair)"
            );
            let want: std::collections::BTreeSet<(i64, i64)> = EXPECTED_SETS
                [o as usize]
                .iter()
                .map(|p| (3 + 5 * p[0], -2 + 5 * p[1]))
                .collect();
            let got: std::collections::BTreeSet<(i64, i64)> = pts
                .iter()
                .map(|p| (p.x.round() as i64, p.y.round() as i64))
                .collect();
            assert_eq!(got, want, "o{o}: emitted vertex set");
        }
    }

    /// The four orientations' emitted vertex lists are rotations of one
    /// another with the index correspondence i↔i: edge i of orientation
    /// o is the 90°·o rotation of edge i of orientation 0. This is the
    /// alignment tess-gui's per-prototile triangulation cache relies on
    /// (a per-orientation outline table would start each class at a
    /// different structural corner and misalign the cached triangles).
    #[test]
    fn poly_vertex_lists_are_rotation_aligned() {
        let base = Chair { o: 0, x: 0, y: 0, s: 3 }.poly();
        for o in 1..4u8 {
            let p = Chair { o, x: 0, y: 0, s: 3 }.poly();
            for i in 0..6 {
                let e0 = base.pts[(i + 1) % 6] - base.pts[i];
                let e = p.pts[(i + 1) % 6] - p.pts[i];
                let (rx, ry) = match o {
                    1 => (-e0.y, e0.x),
                    2 => (-e0.x, -e0.y),
                    _ => (e0.y, -e0.x),
                };
                assert!(
                    (e.x - rx).abs() < 1e-9 && (e.y - ry).abs() < 1e-9,
                    "o{o}: edge {i} is not the 90°·{o} rotation of the \
                     canonical edge — triangulation cache would misalign"
                );
            }
        }
    }

    /// The four children of a chair exactly tile the chair (deflation
    /// view), for every orientation: exact integer cell census — every
    /// parent cell covered exactly once, no cell outside.
    #[test]
    fn substitution_tiles_parent_exactly() {
        for o in 0..4u8 {
            let parent = Chair {
                o,
                x: 3,
                y: -2,
                s: 8,
            };
            let want: std::collections::BTreeSet<(i64, i64)> =
                parent.cells().into_iter().collect();
            assert_eq!(want.len(), 3 * 8 * 8, "o{o}: parent cell count");
            let kids = parent.children();
            let cov = census(&kids);
            assert_eq!(cov.len(), want.len(), "o{o}: covered cell count");
            for (cell, n) in &cov {
                assert_eq!(*n, 1, "o{o}: cell {cell:?} covered {n} times");
                assert!(
                    want.contains(cell),
                    "o{o}: child cell {cell:?} outside parent"
                );
            }
        }
    }

    /// The seed block: four chairs, one per orientation, tiling the 4×3
    /// rectangle exactly.
    #[test]
    fn seed_block_tiles_rectangle() {
        let seed: Vec<Chair> = SEED_UNITS
            .iter()
            .map(|(o, ux, uy)| Chair {
                o: *o,
                x: *ux,
                y: *uy,
                s: 1,
            })
            .collect();
        let cov = census(&seed);
        assert_eq!(cov.len(), 12, "seed: cell count");
        assert!(cov.values().all(|&n| n == 1), "seed: overlap/gap");
        for x in 0..4i64 {
            for y in 0..3i64 {
                assert!(cov.contains_key(&(x, y)), "seed: missing cell ({x},{y})");
            }
        }
        let mut oris: Vec<u8> = seed.iter().map(|c| c.o).collect();
        oris.sort_unstable();
        assert_eq!(oris, [0, 1, 2, 3], "seed must use all four orientations");
    }

    /// Deflating the seed block: chair counts follow 4^(g+1), the four
    /// orientations stay equally numerous, and the total 2× area never
    /// changes (the patch always tiles the same 4u×3u rectangle).
    /// Final chairs land exactly on unit scale (side 1).
    #[test]
    fn refined_counts_and_area_law() {
        let g = 4u32;
        let u = 1i64 << g;
        let mut chairs: Vec<Chair> = SEED_UNITS
            .iter()
            .map(|(o, ux, uy)| Chair {
                o: *o,
                x: ux * u,
                y: uy * u,
                s: u,
            })
            .collect();
        let area2: i64 = chairs.iter().map(|c| 6 * c.s * c.s).sum();
        assert_eq!(area2, 24 * u * u, "seed block 2×area");
        for gen in 1..=g {
            let mut next = Vec::with_capacity(chairs.len() * 4);
            for c in &chairs {
                next.extend(c.children());
            }
            chairs = next;
            assert_eq!(chairs.len(), 4usize.pow(gen) * 4, "gen {gen}: count");
            let mut counts = [0usize; 4];
            for c in &chairs {
                counts[c.o as usize] += 1;
            }
            assert_eq!(
                counts,
                [4usize.pow(gen); 4],
                "gen {gen}: orientation counts not uniform"
            );
            let a2: i64 = chairs.iter().map(|c| 6 * c.s * c.s).sum();
            assert_eq!(a2, area2, "gen {gen}: total 2×area changed");
        }
        assert!(chairs.iter().all(|c| c.s == 1), "final scale must be 1");
    }

    /// The deflated seed block is a perfect tiling of its 4u×3u
    /// rectangle at every generation: exact integer cell census, no
    /// gaps, no overlaps.
    #[test]
    fn refined_patch_covers_rectangle_exactly() {
        let g = 4u32;
        let u = 1i64 << g;
        let mut chairs: Vec<Chair> = SEED_UNITS
            .iter()
            .map(|(o, ux, uy)| Chair {
                o: *o,
                x: ux * u,
                y: uy * u,
                s: u,
            })
            .collect();
        for gen in 1..=g {
            let mut next = Vec::with_capacity(chairs.len() * 4);
            for c in &chairs {
                next.extend(c.children());
            }
            chairs = next;
            let cov = census(&chairs);
            assert_eq!(cov.len() as i64, 12 * u * u, "gen {gen}: cell count");
            assert!(cov.values().all(|&n| n == 1), "gen {gen}: overlap/gap");
        }
    }

    /// T-junctions are inherent to the chair substitution (which is the
    /// unique L-tromino dissection of the inflated chair): in the
    /// canonical side-2 parent, the lower-left child's reentrant vertex
    /// (2,1) lies strictly inside the center child's bottom edge. The
    /// tiling is therefore non-edge-to-edge at every scale — the same
    /// situation as the pinwheel's long-leg T-junctions.
    #[test]
    fn t_junctions_are_inherent() {
        let parent = Chair {
            o: 0,
            x: 0,
            y: 0,
            s: 2,
        };
        let kids = parent.children();
        let ll = kids[0];
        let center = kids[1];
        assert_eq!((ll.o, ll.x, ll.y, ll.s), (0, 0, 0, 1), "lower-left child");
        assert_eq!(
            (center.o, center.x, center.y, center.s),
            (0, 1, 1, 1),
            "center child"
        );
        // ll's reentrant vertex (2,1) strictly inside center's bottom
        // edge (1,1)-(3,1).
        let v = v2(2.0, 1.0);
        let (a, b) = (v2(1.0, 1.0), v2(3.0, 1.0));
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
    /// window over one fixed, origin-anchored refined tiling. (All four
    /// extents lie in (12, 24] — the octave [EXTENT_UNIT·2,
    /// EXTENT_UNIT·4).)
    #[test]
    fn patch_is_stable_under_view_changes_within_an_octave() {
        let t = ChairTiling;
        let p = ParamSet::default_for(t.params());
        let base = t.generate(
            &p,
            &Query { region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)) },
        );
        assert!(base.message.is_none(), "{:?}", base.message);
        let grown = t.generate(
            &p,
            &Query { region: BBox::new(v2(-8.4, -6.3), v2(8.4, 6.3)) },
        );
        let panned = t.generate(
            &p,
            &Query { region: BBox::new(v2(-7.5, -5.5), v2(8.5, 6.5)) },
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
            assert!(grown_keys.contains(k), "grow: tile changed when the view grew");
        }
        for k in &zoomed_keys {
            assert!(base_keys.contains(k), "zoom: tile changed when the view changed");
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
        assert!(!overlap.tiles.is_empty(), "pan: overlap view produced no tiles");
    }

    /// The octave-anchored deflation always lands exactly on unit scale:
    /// every emitted chair has area 3·scale² (3 at default params), and
    /// all four orientation classes occur in a default view.
    #[test]
    fn emitted_tiles_are_unit_scale() {
        let t = ChairTiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(
            &p,
            &Query { region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)) },
        );
        assert!(r.message.is_none(), "{:?}", r.message);
        assert!(!r.tiles.is_empty());
        let mut oris = std::collections::BTreeSet::new();
        for tile in &r.tiles {
            let a = tile.poly.area();
            assert!(
                (a - 3.0).abs() < 1e-9,
                "tile area {a} != 3 (unit chair at default params)"
            );
            oris.insert(tile.seed);
        }
        let want: std::collections::BTreeSet<u32> = [1u32, 2, 3, 4].into_iter().collect();
        assert_eq!(oris, want, "want all four orientation classes");
    }
}