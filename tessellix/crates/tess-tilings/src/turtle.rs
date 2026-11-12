//! Turtle tiling — "Hats in Turtles", a member of the einstein family of
//! Smith, Myers, Kaplan and Goodman-Strauss
//! (arXiv:2305.17743, "A chiral aperiodic monotile").
//!
//! # Exact geometry
//!
//! Everything lives on Kaplan's hexagonal lattice: a point `(x, y)` in
//! Z² stands for the world position `(x + y/2, −(√3/2)·y)` (the
//! conjugate of the `hat.rs` Eisenstein basis). In this basis both
//! prototiles have integer vertices: the **turtle** (14 sides, 8 of
//! length √3 and 6 of length 1, area exactly 10√3) is the dominant
//! tile, and the **hat** (14 sides — the `hat.rs` hat with its length-2
//! side split into two collinear unit sides, area exactly 8√3) is the
//! minor tile. Every placement is therefore an exact D6 transform:
//! rotation by 60°·k, an optional y-axis reflection, and an integer
//! translation. The tiling is aperiodic, homochiral (no reflected
//! tiles), and edge-to-edge; the turtle-to-hat ratio is 4+√15 ≈ 7.873.
//!
//! # Substitution (the construction actually generated)
//!
//! Tiles carry one of nine matching-rule labels (Gamma..Psi). A
//! supertile is an 8-ring of children glued by a key-point chain
//! (`T_RULES`), after which the whole ring is reflected (the per-level
//! chirality flip of the paper); the label table `SUPER_RULES` is
//! transcribed verbatim from Kaplan's reference implementation
//! (cs.uwaterloo.ca/~csk/spectre/spectre.js). Gamma is the "Mystic"
//! compound: one turtle at the identity plus one hat rotated 60° CCW
//! and translated to lattice point (4, −5). Even-depth patches are
//! entirely unreflected; odd-depth patches are entirely reflected, so
//! the generator mirrors odd-depth patches as a whole (the mirror of a
//! homochiral patch is equally valid) before placing them.
//!
//! # Per-view generation
//!
//! Unlike `hat.rs`'s H lineage, a supertile never contains a copy of
//! itself, so there is no nested refinement. Each view is covered by
//! one rigid depth-`d` patch of the Delta supertile, translated so its
//! inscribed disk (center = the patch's boundary-vertex centroid,
//! radius `DEPTH_INFO[d − 1].2`) contains the query box: a box whose
//! half-diagonal fits in the disk is fully covered. The translation is
//! snapped to the `ANCHOR_GRID` lattice grid, so the emitted patch is a
//! deterministic function of (view center, depth) — the tiling
//! re-seeds only when a view crosses an anchor cell or a depth
//! boundary, as with the hat's 96-unit grid. Full-patch counts are
//! 9/71/559/4401/34649 at depths 1..5 (all under `MAX_TILES`), so the
//! zoom-out limit is depth 5's inscribed radius (≈130 world units).

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// √3/2 — world y of a unit hex-lattice step.
const H: f64 = 0.8660254037844386;

/// The turtle's 14 vertices in hex lattice coordinates `(x, y)`,
/// counterclockwise in world, verbatim from Kaplan's spectre.js.
const TURTLE: [[i64; 2]; 14] = [
    [0, 0], [2, -1], [3, 0], [4, -1], [4, -2], [6, -3], [7, -5], [6, -5],
    [5, -4], [4, -5], [2, -4], [0, -3], [-1, -1], [0, -1],
];

/// The hat's 14 vertices — the same polygon as `hat::HAT`, with its
/// length-2 side split at the lattice point (1, −2).
const HAT: [[i64; 2]; 14] = [
    [-1, 2], [0, 2], [0, 3], [2, 2], [3, 0], [4, 0], [5, -1], [4, -2],
    [2, -1], [2, -2], [1, -2], [0, -2], [-1, -1], [0, 0],
];

/// Exact prototile areas: 10√3 (turtle) and 8√3 (hat).
const HAT_AREA: f64 = 16.0 * H;

/// Depth cap: the full depth-5 Delta patch has 34649 tiles
/// (depth 6 would have 272791, still under `MAX_TILES`, but its
/// inscribed radius would rarely be needed — the depth-5 disk of
/// radius ≈129.86 already bounds every admissible view).
const MAX_DEPTH: u32 = 5;

/// Nine matching-rule labels: Gamma=0 .. Psi=8 (spectre.js order).
/// Delta is the seeded supertile.
const DELTA: u8 = 1;
/// The "no child" marker (Gamma's Mystic center slot).
const NULL: u8 = 99;

/// `super_rules`, verbatim from spectre.js: row = parent label, eight
/// columns = the ring's children (slot 2 of Gamma is the Mystic
/// center, which is the parent itself and carries no child).
const SUPER_RULES: [[u8; 8]; 9] = [
    [5, 1, NULL, 2, 6, 4, 7, 0], // Gamma
    [4, 1, 4, 7, 6, 5, 7, 0], // Delta
    [8, 1, 5, 7, 6, 5, 7, 0], // Theta
    [8, 1, 4, 7, 6, 5, 7, 0], // Lambda
    [8, 1, 5, 7, 6, 8, 7, 0], // Xi
    [8, 1, 4, 7, 6, 8, 7, 0], // Pi
    [4, 1, 4, 7, 6, 5, 3, 0], // Sigma
    [8, 1, 8, 7, 6, 5, 7, 0], // Phi
    [8, 1, 8, 7, 6, 5, 7, 0], // Psi
];

/// The key-point gluing chain: `(cumulative angle increment, from
/// quad point, to quad point)`, verbatim from spectre.js.
const T_RULES: [(i32, u8, u8); 7] =
    [(60, 3, 1), (0, 2, 0), (60, 3, 1), (60, 3, 1), (0, 2, 0), (60, 3, 1), (-120, 3, 3)];

/// The hat inside the Mystic Gamma compound: rotated 60° CCW and
/// translated by the hex lattice point (4, −5) — world turtle[9].
const MYSTIC_HAT: Pl = Pl { k: 1, refl: false, t: [4, -5] };

/// Exact D6 placement: rotate by 60°·k (CCW), optionally reflect in
/// the y-axis first, then translate. All arithmetic is integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pl {
    k: u8,
    refl: bool,
    t: [i64; 2],
}

const ID: Pl = Pl { k: 0, refl: false, t: [0, 0] };
/// Reflection in the world y-axis — the per-level chirality flip.
const FLIP: Pl = Pl { k: 0, refl: true, t: [0, 0] };

/// 60° CCW rotation in the hex lattice: world (1,0) -> (1/2, √3/2).
fn rot(p: [i64; 2]) -> [i64; 2] {
    [p[0] + p[1], -p[0]]
}

/// Reflection in the world y-axis.
fn refl(p: [i64; 2]) -> [i64; 2] {
    [-p[0] - p[1], p[1]]
}

fn rotk(p: [i64; 2], k: u8) -> [i64; 2] {
    let mut q = p;
    for _ in 0..(k % 6) {
        q = rot(q);
    }
    q
}

fn apply_lin(p: [i64; 2], k: u8, r: bool) -> [i64; 2] {
    let q = if r { refl(p) } else { p };
    rotk(q, k)
}

fn place(p: [i64; 2], pl: &Pl) -> [i64; 2] {
    let q = apply_lin(p, pl.k, pl.refl);
    [q[0] + pl.t[0], q[1] + pl.t[1]]
}

/// Compose placements: `g2 ∘ g1` (apply `g1` first).
fn compose(g2: &Pl, g1: &Pl) -> Pl {
    let k = if g1.refl && g2.refl {
        (g2.k as i32 - g1.k as i32).rem_euclid(6) as u8
    } else if g1.refl {
        (g1.k + g2.k) % 6
    } else if g2.refl {
        (g2.k as i32 - g1.k as i32).rem_euclid(6) as u8
    } else {
        (g1.k + g2.k) % 6
    };
    let t = apply_lin(g1.t, g2.k, g2.refl);
    Pl {
        k,
        refl: g1.refl ^ g2.refl,
        t: [t[0] + g2.t[0], t[1] + g2.t[1]],
    }
}

/// World position of a hex lattice point.
fn world(p: [i64; 2]) -> Vec2 {
    Vec2::new(p[0] as f64 + 0.5 * p[1] as f64, -H * p[1] as f64)
}

/// The shape's 14 vertices under `pl`, counterclockwise in world
/// (a reflected placement reverses the walk, keeping vertex 0 first).
fn shape_vertices(shape: u8, pl: &Pl) -> [[i64; 2]; 14] {
    let base = if shape == 0 { TURTLE } else { HAT };
    let mut pts = [[0i64; 2]; 14];
    for i in 0..14 {
        pts[i] = place(base[i], pl);
    }
    if pl.refl {
        let first = pts[0];
        for i in 0..13 {
            pts[1 + i] = place(base[13 - i], pl);
        }
        pts[0] = first;
    }
    pts
}

/// One substitution level: the eight ring placements of a child
/// supertile in the parent's frame.
type Level = [Pl; 8];

/// Build levels 0..max_depth: `levels[d]` is the ring used when
/// expanding a depth-(d+1) supertile. Level 0 starts from the turtle's
/// key points (vertices 3, 5, 7, 11), the same as spectre.js.
fn build_levels(max_depth: u32) -> Vec<Level> {
    let mut levels = Vec::with_capacity(max_depth as usize);
    let mut quad = [TURTLE[3], TURTLE[5], TURTLE[7], TURTLE[11]];
    for _ in 0..max_depth {
        let mut ang = 0;
        let mut ts: Vec<Pl> = vec![ID];
        for &(dang, frm, to) in T_RULES.iter() {
            ang += dang;
            let k = (ang / 60) as u8 % 6;
            let prev = *ts.last().unwrap();
            let target = place(quad[frm as usize], &prev);
            let anchor = apply_lin(quad[to as usize], k, false);
            ts.push(Pl {
                k,
                refl: false,
                t: [target[0] - anchor[0], target[1] - anchor[1]],
            });
        }
        // the per-level chirality flip
        let ts: Vec<Pl> = ts.iter().map(|t| compose(&FLIP, t)).collect();
        let mut lvl = [ID; 8];
        lvl.copy_from_slice(&ts);
        levels.push(lvl);
        // the supertile's own key points (spectre.js `super_quad`)
        quad = [
            place(quad[2], &ts[6]),
            place(quad[1], &ts[5]),
            place(quad[2], &ts[3]),
            place(quad[1], &ts[0]),
        ];
    }
    levels
}

/// A base-level tile: prototile `shape` (0 = turtle, 1 = hat), exact
/// placement, and the matching-rule label it carries (1..=9 when
/// emitted as a `seed`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Leaf {
    shape: u8,
    pl: Pl,
    label: u8,
}

/// Flatten a depth-`depth` supertile of label `label` rooted at
/// placement `root` into base tiles.
fn expand(label: u8, root: &Pl, depth: u32, levels: &[Level], out: &mut Vec<Leaf>) {
    if depth == 0 {
        out.push(Leaf {
            shape: 0,
            pl: *root,
            label,
        });
        if label == 0 {
            // Mystic Gamma: the companion hat
            out.push(Leaf {
                shape: 1,
                pl: compose(root, &MYSTIC_HAT),
                label,
            });
        }
        return;
    }
    let ts = &levels[(depth - 1) as usize];
    for (i, &child) in SUPER_RULES[label as usize].iter().enumerate() {
        if child == NULL {
            continue;
        }
        expand(child, &compose(root, &ts[i]), depth - 1, levels, out);
    }
}

/// Coverage data for the depth-`d` Delta patch (odd `d` mirrored, so
/// every tile is unreflected): (centroid x, centroid y, inscribed
/// radius) in world units. The centroid is the average of the patch's
/// boundary vertices; the inscribed disk around it lies inside the
/// patch, so a query box whose half-diagonal fits inside it — measured
/// from the view center, plus the `ANCHOR_MARGIN` snap slack — is
/// covered. Verified by `depth_tables_and_inscribed_circles`.
const DEPTH_INFO: [(f64, f64, f64); 5] = [
    (-1.184_782_608_695_652, -2.579_249_572_140_611, 4.808_477_469_694_492), // d=1
    (17.079_670_329_670_33, -2.041_345_594_634_748, 7.215_126_789_099_404), // d=2
    (-13.734168865435356, -45.171_565_157_553_19, 17.015825764644948), // d=3
    (135.84443402126328, -37.758761765439445, 46.38353660992194),   // d=4
    (-109.54459139943845, -379.591_527_488_876_2, 129.86170309261936), // d=5
];

/// Anchor snapping grid, in hex lattice units.
const ANCHOR_GRID: f64 = 4.0;

/// Worst-case offset of the snapped patch centroid from the view
/// center (grid/2 per hex coordinate → |Δ| ≤ (√3/2)·grid).
const ANCHOR_MARGIN: f64 = H * ANCHOR_GRID;

/// Inverse of [`world`]: world point → (possibly fractional) hex coords.
fn hex_of(p: Vec2) -> (f64, f64) {
    let yh = -p.y / H;
    (p.x - 0.5 * yh, yh)
}

/// Turtle tiling: "Hats in Turtles", the two-prototile einstein tiling.
pub struct TurtleTiling;

impl Tiling for TurtleTiling {
    fn id(&self) -> &'static str {
        "turtle"
    }

    fn name(&self) -> &'static str {
        "Turtle (hats in turtles, einstein family)"
    }

    fn description(&self) -> &'static str {
        "Aperiodic homochiral tiling by turtle + hat prototiles (4+√15 ratio); nine-label substitution; exact hex-lattice geometry."
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "aperiodic; einstein family; homochiral (no reflections); edge-to-edge",
        );
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: None,
            message: Some(message),
        };

        let map = similarity_map(params);
        let inv = map.inverse();

        // Region in the tiling's own (unit-scale) coordinates: the
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
        // Final tiles have area at least 8√3 (hat), bounding the count
        // before any substitution work.
        if region.area() > MAX_TILES as f64 * HAT_AREA {
            return fail("too many tiles for this view — zoom in");
        }

        let c = region.center();
        let mut radius = 0.0f64;
        for &corner in &region.corners() {
            radius = radius.max((corner - c).length());
        }
        let Some(depth) = (1..=MAX_DEPTH)
            .find(|&d| radius <= DEPTH_INFO[(d - 1) as usize].2 - ANCHOR_MARGIN)
        else {
            return fail("too many tiles for this view — zoom in");
        };

        // Snap the patch's centroid onto the hex lattice near the view
        // center: the translation, in hex lattice units.
        let (cx, cy) = hex_of(c);
        let (ox, oy) = hex_of(Vec2::new(
            DEPTH_INFO[depth as usize - 1].0,
            DEPTH_INFO[depth as usize - 1].1,
        ));
        let tx = (((cx - ox) / ANCHOR_GRID).round() * ANCHOR_GRID) as i64;
        let ty = (((cy - oy) / ANCHOR_GRID).round() * ANCHOR_GRID) as i64;

        let levels = build_levels(depth);
        // Odd-depth patches come out fully reflected; compose the
        // global y-axis flip into the root so every emitted tile is
        // unreflected (CCW).
        let root = Pl {
            k: 0,
            refl: depth % 2 == 1,
            t: [tx, ty],
        };
        let mut leaves = Vec::new();
        expand(DELTA, &root, depth, &levels, &mut leaves);

        let mut tiles = Vec::with_capacity(leaves.len());
        for lf in &leaves {
            let wpts: Vec<Vec2> = shape_vertices(lf.shape, &lf.pl)
                .iter()
                .map(|&p| world(p))
                .collect();
            let aabb = Polygon::from_slice(&wpts).aabb();
            if !aabb.intersects(region) {
                continue;
            }
            let poly = Polygon::from_slice(&wpts).transform(&map);
            if !poly.aabb().intersects(query.region) {
                continue;
            }
            tiles.push(Tile {
                proto: lf.shape,
                seed: lf.label as u32 + 1,
                poly,
            });
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
    use std::collections::{BTreeMap, BTreeSet};

    /// Exact turtle area (10√3) — test scope only.
    const TURTLE_AREA: f64 = 20.0 * H;

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// World squared length of a hex-lattice step.
    fn l2(d: (i64, i64)) -> i64 {
        d.0 * d.0 + d.0 * d.1 + d.1 * d.1
    }

    /// Both prototiles: 14 CCW sides, exact areas 10√3 and 8√3, and the
    /// exact side-length sequence (lattice squared lengths 1 and 3),
    /// for every D6 orientation.
    #[test]
    fn turtle_and_hat_outline_geometry() {
        let turtle_len2: [i64; 14] = [3, 3, 1, 1, 3, 3, 1, 1, 3, 3, 3, 3, 1, 1];
        let hat_len2: [i64; 14] = [1, 1, 3, 3, 1, 1, 3, 3, 1, 1, 1, 1, 3, 3];
        for &shape in &[0u8, 1] {
            let sqrt3 = 3.0f64.sqrt();
            let want_2a = if shape == 0 { 20.0 * sqrt3 } else { 16.0 * sqrt3 };
            let want_len = if shape == 0 { &turtle_len2 } else { &hat_len2 };
            for k in 0..6u8 {
                for &reflected in &[false, true] {
                    let pl = Pl { k, refl: reflected, t: [3, -5] };
                    let pts = shape_vertices(shape, &pl);
                    let wpts: Vec<Vec2> = pts.iter().map(|&p| world(p)).collect();
                    let mut a2 = 0.0;
                    for i in 0..14 {
                        let p = wpts[i];
                        let q = wpts[(i + 1) % 14];
                        a2 += p.x * q.y - q.x * p.y;
                    }
                    assert!(a2 > 0.0, "shape {shape} k={k} refl={reflected}: not CCW");
                    assert!(
                        (a2 - want_2a).abs() < 1e-9,
                        "shape {shape} k={k} refl={reflected}: 2×area {a2} != {want_2a}"
                    );
                    let lens: Vec<i64> = (0..14)
                        .map(|i| l2((pts[(i + 1) % 14][0] - pts[i][0], pts[(i + 1) % 14][1] - pts[i][1])))
                        .collect();
                    let want: Vec<i64> = if reflected {
                        want_len.iter().rev().copied().collect()
                    } else {
                        want_len.to_vec()
                    };
                    assert_eq!(lens, want, "shape {shape} k={k} refl={reflected}: side lengths");
                }
            }
        }
    }

    /// The leaf counts of the substitution match the verified reference
    /// (spectre.js, arXiv:2305.17743), every supertile contains exactly
    /// one Mystic-Γ child (hence the hat count at depth d equals the
    /// turtle count of the same seed at depth d−1), the chirality is
    /// uniform per depth, and no two leaves share a placement. (The
    /// patch area ratio converges to 4+√15 per depth but is never
    /// exactly it — the Γ child has a different area — so no exact
    /// area-growth assertion is made here.)
    #[test]
    fn substitution_counts_match_reference() {
        const DELTA_COUNTS: [(usize, usize); 5] = [(1, 0), (8, 1), (63, 8), (496, 63), (3905, 496)];
        const GAMMA_COUNTS: [(usize, usize); 5] = [(1, 1), (7, 1), (55, 7), (433, 55), (3409, 433)];
        let levels = build_levels(4);
        for (seed, counts) in [(DELTA, DELTA_COUNTS), (0u8, GAMMA_COUNTS)] {
            for d in 0..5u32 {
                let mut leaves = Vec::new();
                expand(seed, &ID, d, &levels, &mut leaves);
                let (t, h) = counts[d as usize];
                let nt = leaves.iter().filter(|l| l.shape == 0).count();
                let nh = leaves.iter().filter(|l| l.shape == 1).count();
                assert_eq!((nt, nh), (t, h), "seed {seed} depth {d}: turtle/hat counts");
                if d > 0 {
                    assert_eq!(nh, counts[(d - 1) as usize].0, "seed {seed} depth {d}: hats = previous turtles");
                }
                let want_refl = d % 2 == 1;
                assert!(
                    leaves.iter().all(|l| l.pl.refl == want_refl),
                    "seed {seed} depth {d}: chirality must be uniform (refl = {want_refl})"
                );
                let set: BTreeSet<_> = leaves.iter().map(|l| (l.shape, l.pl)).collect();
                assert_eq!(set.len(), leaves.len(), "seed {seed} depth {d}: duplicate placements");
            }
        }
    }

    /// The depth-3 Delta patch (559 tiles) is a perfect tiling of a
    /// topological disk: no edge owned by more than two tiles, exactly
    /// 3534 shared edges and 758 boundary edges, Euler characteristic 1.
    #[test]
    fn patch_gluing_census() {
        let levels = build_levels(3);
        let mut leaves = Vec::new();
        expand(DELTA, &ID, 3, &levels, &mut leaves);
        assert_eq!(leaves.len(), 559);
        let mut edges: BTreeMap<([i64; 2], [i64; 2]), u32> = BTreeMap::new();
        let mut verts = BTreeSet::new();
        for lf in &leaves {
            let pts = shape_vertices(lf.shape, &lf.pl);
            for &p in &pts {
                verts.insert(p);
            }
            for i in 0..14 {
                let (a, b) = (pts[i], pts[(i + 1) % 14]);
                let key = if a <= b { (a, b) } else { (b, a) };
                *edges.entry(key).or_insert(0) += 1;
            }
        }
        let bad: Vec<_> = edges.iter().filter(|&(_, &n)| n > 2).collect();
        assert!(bad.is_empty(), "edges with >2 owners: {bad:?}");
        let shared = edges.values().filter(|&&n| n == 2).count();
        let boundary = edges.values().filter(|&&n| n == 1).count();
        assert_eq!((shared, boundary), (3534, 758), "shared/boundary edges");
        let (v, e, f) = (verts.len() as i64, edges.len() as i64, leaves.len() as i64);
        assert_eq!((v, e, f), (3734, 4292, 559), "V, E, F");
        assert_eq!(v - e + f, 1, "Euler characteristic of the patch disk");
    }

    /// `DEPTH_INFO` matches the exact expansions: for every depth 1..5
    /// the recomputed boundary-vertex centroid and inscribed radius of
    /// the (odd-mirrored) Delta patch agree with the table, and the
    /// full-patch counts are 9/71/559/4401/34649.
    #[test]
    fn depth_tables_and_inscribed_circles() {
        const FULL: [usize; 5] = [9, 71, 559, 4401, 34649];
        let levels = build_levels(MAX_DEPTH);
        for d in 1..=MAX_DEPTH {
            let mut leaves = Vec::new();
            expand(DELTA, &ID, d, &levels, &mut leaves);
            assert_eq!(leaves.len(), FULL[d as usize - 1], "depth {d}: full patch count");
            if d % 2 == 1 {
                for lf in &mut leaves {
                    lf.pl = compose(&FLIP, &lf.pl);
                }
                assert!(leaves.iter().all(|l| !l.pl.refl), "depth {d}: normalization failed");
            }
            // boundary edges: exactly one owner
            let mut edges: BTreeMap<([i64; 2], [i64; 2]), u32> = BTreeMap::new();
            let mut bverts = BTreeSet::new();
            for lf in &leaves {
                let pts = shape_vertices(lf.shape, &lf.pl);
                for i in 0..14 {
                    let (a, b) = (pts[i], pts[(i + 1) % 14]);
                    let key = if a <= b { (a, b) } else { (b, a) };
                    *edges.entry(key).or_insert(0) += 1;
                }
            }
            for (&(a, b), &n) in edges.iter() {
                if n == 1 {
                    bverts.insert(a);
                    bverts.insert(b);
                }
            }
            let (mut sx, mut sy, mut n) = (0.0f64, 0.0f64, 0usize);
            for &p in &bverts {
                let w = world(p);
                sx += w.x;
                sy += w.y;
                n += 1;
            }
            let (cx, cy) = (sx / n as f64, sy / n as f64);
            let (ox, oy, rin) = DEPTH_INFO[d as usize - 1];
            assert!(
                (cx - ox).abs() < 1e-9 && (cy - oy).abs() < 1e-9,
                "depth {d}: centroid ({cx}, {cy}) != table ({ox}, {oy})"
            );
            // the centroid lies in the patch, and the min distance to a
            // boundary segment reproduces the table's inscribed radius
            let c = Vec2::new(cx, cy);
            let mut hit = false;
            for lf in &leaves {
                let poly: Vec<Vec2> =
                    shape_vertices(lf.shape, &lf.pl).iter().map(|&p| world(p)).collect();
                if Polygon::from_slice(&poly).contains(c) {
                    hit = true;
                }
            }
            let mut rin_actual = f64::MAX;
            for (&(a, b), &n) in edges.iter() {
                if n == 1 {
                    rin_actual = rin_actual.min(seg_dist(c, world(a), world(b)));
                }
            }
            assert!(hit, "depth {d}: centroid not inside patch");
            assert!(
                (rin_actual - rin).abs() < 1e-9,
                "depth {d}: r_in {rin_actual} != table {rin}"
            );
        }
    }

    /// Point-to-segment distance in world coordinates.
    fn seg_dist(p: Vec2, a: Vec2, b: Vec2) -> f64 {
        let (dx, dy) = (b.x - a.x, b.y - a.y);
        let t = ((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy);
        let t = t.clamp(0.0, 1.0);
        ((p.x - (a.x + t * dx)).powi(2) + (p.y - (a.y + t * dy)).powi(2)).sqrt()
    }

    /// Every emitted tile is a unit prototile: 14 CCW vertices, exact
    /// area 10√3 (turtle) or 8√3 (hat), proto 0/1, seed 1..=9.
    #[test]
    fn emitted_tiles_are_valid() {
        let t = TurtleTiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(
            &p,
            &Query { region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)) },
        );
        assert!(r.message.is_none(), "{:?}", r.message);
        assert!(!r.tiles.is_empty());
        for tile in &r.tiles {
            assert_eq!(tile.poly.pts.len(), 14, "vertex count");
            assert!(tile.proto <= 1, "proto {}", tile.proto);
            assert!((1..=9).contains(&tile.seed), "seed {}", tile.seed);
            let want = if tile.proto == 0 { TURTLE_AREA } else { HAT_AREA };
            assert!(
                (tile.poly.area() - want).abs() < 1e-9,
                "tile area {} != {want}",
                tile.poly.area()
            );
            // CCW
            let mut a2 = 0.0;
            let n = tile.poly.pts.len();
            for i in 0..n {
                let p1 = tile.poly.pts[i];
                let p2 = tile.poly.pts[(i + 1) % n];
                a2 += p1.x * p2.y - p2.x * p1.y;
            }
            assert!(a2 > 0.0, "tile not CCW");
        }
    }

    /// Bit-exact tile identity: same seed and identical vertex bits.
    fn tile_key(tile: &Tile) -> (u8, u32, Vec<u64>) {
        let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
        for p in &tile.poly.pts {
            bits.push(p.x.to_bits());
            bits.push(p.y.to_bits());
        }
        (tile.proto, tile.seed, bits)
    }

    /// Views sharing the same anchor cell and depth bucket see one
    /// fixed rigid patch: growing, panning within the cell, or zooming
    /// keeps every visible tile bit-identical (the patch is rigid, so
    /// deeper acceptance never happens — the sets just shrink or grow
    /// at the boundary). The four views below all quantize to anchor
    /// (40, −52) at depth 3.
    #[test]
    fn patch_is_stable_under_view_changes() {
        let t = TurtleTiling;
        let p = ParamSet::default_for(t.params());
        let gen = |min: Vec2, max: Vec2| t.generate(&p, &Query { region: BBox::new(min, max) });
        let base = gen(v2(-8.0, -6.0), v2(8.0, 6.0));
        assert!(base.message.is_none(), "{:?}", base.message);
        let grown = gen(v2(-8.4, -6.3), v2(8.4, 6.3));
        let panned = gen(v2(-6.5, -7.5), v2(10.5, 6.5));
        let zoomed = gen(v2(-8.0 / 1.3, -6.0 / 1.3), v2(8.0 / 1.3, 6.0 / 1.3));
        for r in [&grown, &panned, &zoomed] {
            assert!(r.message.is_none(), "{:?}", r.message);
        }

        let keys = |r: &TilingResult| r.tiles.iter().map(tile_key).collect::<std::collections::HashSet<_>>();
        let base_keys = keys(&base);
        let grown_keys = keys(&grown);
        let panned_keys = keys(&panned);
        let zoomed_keys = keys(&zoomed);
        for k in &base_keys {
            assert!(grown_keys.contains(k), "grow: tile changed when the view grew");
        }
        for k in &zoomed_keys {
            assert!(base_keys.contains(k), "zoom: tile changed when the view changed");
        }
        // the overlap window: its tiles must appear unchanged in both
        let overlap = gen(v2(-6.5, -6.5), v2(8.0, 6.0));
        assert!(!overlap.tiles.is_empty(), "overlap view produced no tiles");
        for tile in &overlap.tiles {
            let k = tile_key(tile);
            assert!(base_keys.contains(&k), "pan: overlap tile missing/changed in base view");
            assert!(panned_keys.contains(&k), "pan: overlap tile missing/changed in panned view");
        }
    }
}
