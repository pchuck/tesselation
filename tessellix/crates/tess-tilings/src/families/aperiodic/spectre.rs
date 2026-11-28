//! Spectre tiling — the chiral einstein Tile(1,1) of Smith, Myers,
//! Kaplan and Goodman-Strauss (arXiv:2305.17743, "A chiral aperiodic
//! monotile").
//!
//! # Exact geometry
//!
//! Unlike the hat and the turtle, the spectre's vertices are not
//! rational-lattice points: they live in Q(√3). A point is stored as
//! the exact surd `((a + b√3) / 2ⁿ, (c + d√3) / 2ⁿ)` with integer
//! `a, b, c, d` and a small exponent `n` (≤ ~41 at the deepest
//! generated patch, so i64 is more than enough). The 14 spectre
//! vertices all have unit-length sides (the "equal-edge" property) and
//! the area is exactly 3 + 3√3. Every substitution frame is a D12
//! transform — rotation by 30°·k, an optional y-axis reflection, and a
//! surd translation — closed under this representation because 30°
//! rotation maps Q(√3) to itself. The tiling is aperiodic and *chiral*:
//! it covers the plane with the spectre alone, no reflected copies.
//!
//! # Substitution
//!
//! The same nine-label machinery as `turtle.rs` (transcribed verbatim
//! from Kaplan's spectre.js): an 8-ring of children glued by a key-point
//! chain (`T_RULES`) from the spectre's quad points (vertices 3, 5, 7,
//! 11), one y-axis reflection per level (the per-level chirality flip),
//! and the verbatim `SUPER_RULES` label table (buildSupertiles is
//! shape-generic). Gamma is the "Mystic" compound: one spectre at the
//! identity plus one rotated 30° CCW about the origin and translated by
//! spectre[8]. Because the Mystic child sits at 30°, ring children occur
//! at 30°-multiple orientations, so placements are D12 (k mod 12)
//! rather than D6. Even-depth patches are entirely unreflected; odd-depth
//! patches are entirely reflected, so the generator composes the global
//! y-axis flip into the root placement before emitting (every emitted
//! tile is unreflected / CCW).
//!
//! # Per-view generation
//!
//! Each view is covered by one rigid depth-`d` patch of the Delta
//! supertile whose inscribed disk (center = the patch's boundary-vertex
//! centroid, radius `DEPTH_INFO[d − 1].2`) contains the query box. The
//! patch centroid is snapped onto the 4-world-unit grid near the view
//! center, so the emitted patch is a deterministic function of (view
//! center, depth). Full-patch counts are 9/71/559/4401/34649 at depths
//! 1..5 (all under `MAX_TILES`), so the zoom-out limit is depth 5's
//! inscribed radius (≈90.4 world units).

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// √3/2.
const H: f64 = 0.8660254037844386;

/// Exact spectre area: 3 + 3√3 = 3 + 6·(√3/2).
const SPECTRE_AREA: f64 = 3.0 + 6.0 * H;

/// Depth cap: the full depth-5 Delta patch has 34649 tiles
/// (depth 6 would have ~272k — right at the `MAX_TILES` budget and its
/// inscribed radius ≈ 281 rarely helps, so 5 is the practical cap).
const MAX_DEPTH: u32 = 5;

/// Nine matching-rule labels: Gamma=0 .. Psi=8 (spectre.js order).
/// Delta is the seeded supertile.
const DELTA: u8 = 1;
/// The "no child" marker (Gamma's Mystic center slot).
const NULL: u8 = 99;

/// `super_rules`, verbatim from spectre.js: row = parent label, eight
/// columns = the ring's children (slot 2 of Gamma is the Mystic
/// center, which is the parent itself and carries no child). The table
/// is shape-generic — hat, turtle, and spectre supertiles all use it.
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

/// An exact surd point: `((a + b√3) / 2ⁿ, (c + d√3) / 2ⁿ)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SurdPt {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    n: u32,
}

const ZERO: SurdPt = SurdPt { a: 0, b: 0, c: 0, d: 0, n: 0 };

/// The 14 spectre vertices, exact surds at n = 1 (i.e. halves of
/// integers plus integer multiples of √3/2), counterclockwise in
/// world, verbatim from Kaplan's spectre.js `buildSpectreBase`. All 14
/// sides are unit length; the area is exactly 3 + 3√3.
const SPECTRE: [SurdPt; 14] = [
    SurdPt { a: 0, b: 0, c: 0, d: 0, n: 1 },
    SurdPt { a: 2, b: 0, c: 0, d: 0, n: 1 },
    SurdPt { a: 3, b: 0, c: 0, d: -1, n: 1 },
    SurdPt { a: 3, b: 1, c: 1, d: -1, n: 1 },
    SurdPt { a: 3, b: 1, c: 3, d: -1, n: 1 },
    SurdPt { a: 5, b: 1, c: 3, d: -1, n: 1 },
    SurdPt { a: 6, b: 1, c: 3, d: 0, n: 1 },
    SurdPt { a: 6, b: 0, c: 4, d: 0, n: 1 },
    SurdPt { a: 6, b: -1, c: 3, d: 0, n: 1 },
    SurdPt { a: 5, b: -1, c: 3, d: 1, n: 1 },
    SurdPt { a: 3, b: -1, c: 3, d: 1, n: 1 },
    SurdPt { a: 1, b: -1, c: 3, d: 1, n: 1 },
    SurdPt { a: 0, b: -1, c: 3, d: 0, n: 1 },
    SurdPt { a: 0, b: 0, c: 2, d: 0, n: 1 },
];

/// Exact D12 placement: rotate by 30°·k (CCW), optionally reflect in
/// the y-axis first, then translate. All arithmetic is exact.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SPl {
    k: u8,
    refl: bool,
    t: SurdPt,
}

const ID: SPl = SPl { k: 0, refl: false, t: ZERO };
/// Reflection in the world y-axis — the per-level chirality flip.
const FLIP: SPl = SPl { k: 0, refl: true, t: ZERO };
/// The second spectre of the Mystic Gamma compound: rotated 30° CCW
/// about the origin and translated by world spectre[8] =
/// ((6−√3)/2, 3/2) (spectre.js: mul(ttrans(spectre[8]), trot(PI/6))).
const MYSTIC_G2: SPl = SPl { k: 1, refl: false, t: SPECTRE[8] };

/// Sum of two surd points (aligns to the larger exponent).
fn add_pt(p: SurdPt, q: SurdPt) -> SurdPt {
    let m = p.n.max(q.n);
    let sp = 1i64 << (m - p.n);
    let sq = 1i64 << (m - q.n);
    SurdPt {
        a: p.a * sp + q.a * sq,
        b: p.b * sp + q.b * sq,
        c: p.c * sp + q.c * sq,
        d: p.d * sp + q.d * sq,
        n: m,
    }
}

fn sub_pt(p: SurdPt, q: SurdPt) -> SurdPt {
    add_pt(
        p,
        SurdPt {
            a: -q.a,
            b: -q.b,
            c: -q.c,
            d: -q.d,
            n: q.n,
        },
    )
}

/// Reflection in the world y-axis: (x, y) -> (−x, y).
fn refl(p: SurdPt) -> SurdPt {
    SurdPt {
        a: -p.a,
        b: -p.b,
        c: p.c,
        d: p.d,
        n: p.n,
    }
}

/// 30° CCW rotation: (x, y) -> ((√3·x − y)/2, (x + √3·y)/2).
fn rot30(p: SurdPt) -> SurdPt {
    SurdPt {
        a: 3 * p.b - p.c,
        b: p.a - p.d,
        c: p.a + 3 * p.d,
        d: p.b + p.c,
        n: p.n + 1,
    }
}

fn rotk(p: SurdPt, k: u8) -> SurdPt {
    let mut q = p;
    for _ in 0..(k % 12) {
        q = rot30(q);
    }
    q
}

fn apply_lin(p: SurdPt, k: u8, r: bool) -> SurdPt {
    let q = if r { refl(p) } else { p };
    rotk(q, k)
}

fn place(p: SurdPt, pl: &SPl) -> SurdPt {
    let q = apply_lin(p, pl.k, pl.refl);
    add_pt(q, pl.t)
}

/// Compose placements: `g2 ∘ g1` (apply `g1` first).
fn compose(g2: &SPl, g1: &SPl) -> SPl {
    let k = if g1.refl && g2.refl {
        (g2.k as i32 - g1.k as i32).rem_euclid(12) as u8
    } else if g1.refl {
        (g1.k + g2.k) % 12
    } else if g2.refl {
        (g2.k as i32 - g1.k as i32).rem_euclid(12) as u8
    } else {
        (g1.k + g2.k) % 12
    };
    let t = add_pt(apply_lin(g1.t, g2.k, g2.refl), g2.t);
    SPl {
        k,
        refl: g1.refl ^ g2.refl,
        t,
    }
}

/// World position of a surd point.
fn world(p: SurdPt) -> Vec2 {
    let s = 2.0f64.powi(-(p.n as i32));
    Vec2::new((p.a as f64 + p.b as f64 * (2.0 * H)) * s, (p.c as f64 + p.d as f64 * (2.0 * H)) * s)
}

/// Canonical representative of a surd point: halve all four
/// components while they are even. Unique per point value, so canon
/// keys may be compared with `==`/`Ord` for edge and vertex censuses.
#[cfg(test)]
fn canon(p: SurdPt) -> SurdPt {
    let (mut a, mut b, mut c, mut d, mut n) = (p.a, p.b, p.c, p.d, p.n);
    while n > 0 && a % 2 == 0 && b % 2 == 0 && c % 2 == 0 && d % 2 == 0 {
        a /= 2;
        b /= 2;
        c /= 2;
        d /= 2;
        n -= 1;
    }
    SurdPt { a, b, c, d, n }
}

/// The spectre's 14 vertices under `pl`, counterclockwise in world
/// (a reflected placement reverses the walk, keeping vertex 0 first).
fn shape_vertices(pl: &SPl) -> [SurdPt; 14] {
    let mut pts = [ZERO; 14];
    for i in 0..14 {
        pts[i] = place(SPECTRE[i], pl);
    }
    if pl.refl {
        let first = pts[0];
        for i in 0..13 {
            pts[1 + i] = place(SPECTRE[13 - i], pl);
        }
        pts[0] = first;
    }
    pts
}

/// One substitution level: the eight ring placements of a child
/// supertile in the parent's frame.
type Level = [SPl; 8];

/// Build levels 0..max_depth: `levels[d]` is the ring used when
/// expanding a depth-(d+1) supertile. Level 0 starts from the spectre's
/// key points (vertices 3, 5, 7, 11), the same as spectre.js.
fn build_levels(max_depth: u32) -> Vec<Level> {
    let mut levels = Vec::with_capacity(max_depth as usize);
    let mut quad = [SPECTRE[3], SPECTRE[5], SPECTRE[7], SPECTRE[11]];
    for _ in 0..max_depth {
        let mut ang = 0;
        let mut ts: Vec<SPl> = vec![ID];
        for &(dang, frm, to) in T_RULES.iter() {
            ang += dang;
            let k = ((ang / 30) as u8) % 12;
            let prev = *ts.last().unwrap();
            let target = place(quad[frm as usize], &prev);
            let anchor = apply_lin(quad[to as usize], k, false);
            ts.push(SPl {
                k,
                refl: false,
                t: sub_pt(target, anchor),
            });
        }
        // the per-level chirality flip
        let ts: Vec<SPl> = ts.iter().map(|t| compose(&FLIP, t)).collect();
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

/// A base-level tile: the matching-rule label it carries (emitted as
/// `seed` = label + 1) and its exact placement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Leaf {
    label: u8,
    pl: SPl,
}

/// Flatten a depth-`depth` supertile of label `label` rooted at
/// placement `root` into base spectres.
fn expand(label: u8, root: &SPl, depth: u32, levels: &[Level], out: &mut Vec<Leaf>) {
    if depth == 0 {
        out.push(Leaf {
            label,
            pl: *root,
        });
        if label == 0 {
            // Mystic Gamma: the companion spectre at 30° + spectre[8]
            out.push(Leaf {
                label,
                pl: compose(root, &MYSTIC_G2),
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
    (-1.7427672955001385, -1.2002785417630875, 3.5520506284562514), // d=1
    (10.100445915634971, -5.768_852_204_017_162, 5.220_287_283_230_152), // d=2
    (-21.29431528373707, -25.121678882145314, 11.803657667119884), // d=3
    (76.635_463_185_963_93, -60.72250375055981, 32.44981721847531), // d=4
    (-172.97064804705036, -213.74979539966307, 90.380_683_296_047_02), // d=5
];

/// Anchor snapping grid, in world units (the spectre has no underlying
/// lattice — the grid is a plain square grid used for determinism).
const ANCHOR_GRID: f64 = 4.0;

/// Worst-case offset of the snapped patch centroid from the view
/// center: half a grid cell in each axis (4-unit cell circumradius,
/// 2√2).
const ANCHOR_MARGIN: f64 = 2.8284271247461903;

/// Spectre tiling: the chiral einstein Tile(1,1).
pub struct SpectreTiling;

impl Tiling for SpectreTiling {
    fn id(&self) -> &'static str {
        "spectre"
    }

    fn name(&self) -> &'static str {
        "Spectre (chiral einstein, Tile(1,1))"
    }

    fn description(&self) -> &'static str {
        "Aperiodic homochiral tiling by a single equal-edge prototile (aperiodic without reflections); nine-label substitution; exact Q(√3) geometry."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Aperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &SIMILARITY_PARAMS
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None).with_notes(
            "aperiodic; einstein family; chiral (aperiodic without reflections); homochiral; edge-to-edge",
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
        // Every tile has area 3 + 3√3, bounding the count before any
        // substitution work.
        if region.area() > MAX_TILES as f64 * SPECTRE_AREA {
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

        // Snap the patch's centroid onto the world grid near the view
        // center: the translation is an exact multiple of 4.0.
        let (ox, oy, _rin) = DEPTH_INFO[depth as usize - 1];
        let tx = (((c.x - ox) / ANCHOR_GRID).round() * ANCHOR_GRID) as i64;
        let ty = (((c.y - oy) / ANCHOR_GRID).round() * ANCHOR_GRID) as i64;

        let levels = build_levels(depth);
        // Odd-depth patches come out fully reflected; compose the
        // global y-axis flip into the root so every emitted tile is
        // unreflected (CCW).
        let root = SPl {
            k: 0,
            refl: depth % 2 == 1,
            t: SurdPt { a: tx, b: 0, c: ty, d: 0, n: 0 },
        };
        let mut leaves = Vec::new();
        expand(DELTA, &root, depth, &levels, &mut leaves);

        let mut tiles = Vec::with_capacity(leaves.len());
        for lf in &leaves {
            let wpts: Vec<Vec2> = shape_vertices(&lf.pl)
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
            tiles.push(Tile { family: tess_core::tiling::FamilyTag::None, reflected: false,
                proto: 0,
                seed: lf.label as u32 + 1, level: depth as u8,
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

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// The spectre outline: 14 CCW unit sides and exact area 3 + 3√3
    /// in every D12 orientation and both chiralities.
    #[test]
    fn spectre_outline_geometry() {
        for k in 0..12u8 {
            for &reflected in &[false, true] {
                let pl = SPl { k, refl: reflected, t: ZERO };
                let pts = shape_vertices(&pl);
                let wpts: Vec<Vec2> = pts.iter().map(|&p| world(p)).collect();
                let mut a2 = 0.0;
                for i in 0..14 {
                    let p = wpts[i];
                    let q = wpts[(i + 1) % 14];
                    a2 += p.x * q.y - q.x * p.y;
                }
                assert!(a2 > 0.0, "k={k} refl={reflected}: not CCW");
                assert!(
                    (a2 - 2.0 * SPECTRE_AREA).abs() < 1e-9,
                    "k={k} refl={reflected}: 2×area {a2} != {SPECTRE_AREA}"
                );
                for i in 0..14 {
                    let p = wpts[i];
                    let q = wpts[(i + 1) % 14];
                    let d2 = (p.x - q.x).powi(2) + (p.y - q.y).powi(2);
                    assert!(
                        (d2 - 1.0).abs() < 1e-12,
                        "k={k} refl={reflected}: side {i} not unit (len2 = {d2})"
                    );
                }
            }
        }
    }

    /// The flattened leaf counts match the verified reference
    /// (spectre.js, arXiv:2305.17743), and the Mystic structure forces
    /// Γleaves(d+1) + Γleaves(d) = 2·total(d): every supertile spawns
    /// exactly one Mystic-Γ child (which flattens to a Γ-leaf pair), so
    /// the number of supertiles at depth d is total(d) − Γleaves(d)/2
    /// and equals Γleaves(d+1)/2. Chirality is uniform per depth; no
    /// two leaves share a placement.
    #[test]
    fn substitution_counts_match_reference() {
        const DELTA_COUNTS: [usize; 5] = [1, 9, 71, 559, 4401];
        const GAMMA_COUNTS: [usize; 5] = [2, 8, 62, 488, 3842];
        let levels = build_levels(4);
        for (seed, counts) in [(DELTA, DELTA_COUNTS), (0u8, GAMMA_COUNTS)] {
            let mut prev_gamma = 0usize;
            for d in 0..5u32 {
                let mut leaves = Vec::new();
                expand(seed, &ID, d, &levels, &mut leaves);
                assert_eq!(
                    leaves.len(),
                    counts[d as usize],
                    "seed {seed} depth {d}: leaf count"
                );
                let g = leaves.iter().filter(|l| l.label == 0).count();
                if d > 0 {
                    assert_eq!(
                        g + prev_gamma,
                        2 * counts[(d - 1) as usize],
                        "seed {seed} depth {d}: Γ(d)+Γ(d−1) = 2·total(d−1)"
                    );
                }
                prev_gamma = g;
                let want_refl = d % 2 == 1;
                assert!(
                    leaves.iter().all(|l| l.pl.refl == want_refl),
                    "seed {seed} depth {d}: chirality must be uniform (refl = {want_refl})"
                );
                let set: BTreeSet<_> = leaves
                    .iter()
                    .map(|l| (l.label, l.pl.k, l.pl.refl, canon(l.pl.t)))
                    .collect();
                assert_eq!(
                    set.len(),
                    leaves.len(),
                    "seed {seed} depth {d}: duplicate placements"
                );
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
        let mut edges: BTreeMap<(SurdPt, SurdPt), u32> = BTreeMap::new();
        let mut verts = BTreeSet::new();
        for lf in &leaves {
            let pts = shape_vertices(&lf.pl);
            for &p in &pts {
                verts.insert(canon(p));
            }
            for i in 0..14 {
                let (a, b) = (canon(pts[i]), canon(pts[(i + 1) % 14]));
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
                assert!(
                    leaves.iter().all(|l| !l.pl.refl),
                    "depth {d}: normalization failed"
                );
            }
            // boundary edges: exactly one owner
            let mut edges: BTreeMap<(SurdPt, SurdPt), u32> = BTreeMap::new();
            for lf in &leaves {
                let pts = shape_vertices(&lf.pl);
                for i in 0..14 {
                    let (a, b) = (canon(pts[i]), canon(pts[(i + 1) % 14]));
                    let key = if a <= b { (a, b) } else { (b, a) };
                    *edges.entry(key).or_insert(0) += 1;
                }
            }
            let mut bverts = BTreeSet::new();
            for ((a, b), &n) in edges.iter() {
                if n == 1 {
                    bverts.insert(*a);
                    bverts.insert(*b);
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
                    shape_vertices(&lf.pl).iter().map(|&p| world(p)).collect();
                if Polygon::from_slice(&poly).contains(c) {
                    hit = true;
                }
            }
            let mut rin_actual = f64::MAX;
            for ((a, b), &n) in edges.iter() {
                if n == 1 {
                    rin_actual = rin_actual.min(seg_dist(c, world(*a), world(*b)));
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
    /// area 3 + 3√3, proto 0, seed 1..=9.
    #[test]
    fn emitted_tiles_are_valid() {
        let t = SpectreTiling;
        let p = ParamSet::default_for(t.params());
        let r = t.generate(
            &p,
            &Query { region: BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0)) },
        );
        assert!(r.message.is_none(), "{:?}", r.message);
        assert!(!r.tiles.is_empty());
        for tile in &r.tiles {
            assert_eq!(tile.poly.pts.len(), 14, "vertex count");
            assert_eq!(tile.proto, 0, "proto {}", tile.proto);
            assert!((1..=9).contains(&tile.seed), "seed {}", tile.seed);
            assert!(
                (tile.poly.area() - SPECTRE_AREA).abs() < 1e-9,
                "tile area {} != {SPECTRE_AREA}",
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
    /// keeps every visible tile bit-identical. All four views below
    /// quantize to the same anchor cell at depth 4.
    #[test]
    fn patch_is_stable_under_view_changes() {
        let t = SpectreTiling;
        let p = ParamSet::default_for(t.params());
        let gen = |min: Vec2, max: Vec2| t.generate(&p, &Query { region: BBox::new(min, max) });
        let base = gen(v2(-7.5, -6.5), v2(8.5, 5.5));
        assert!(base.message.is_none(), "{:?}", base.message);
        let grown = gen(v2(-7.875, -6.825), v2(8.925, 5.825));
        let panned = gen(v2(-6.0, -6.0), v2(10.0, 6.0));
        let zoomed = gen(v2(-7.5 / 1.05, -6.5 / 1.05), v2(8.5 / 1.05, 5.5 / 1.05));
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
        let overlap = gen(v2(-5.9, -5.9), v2(8.4, 5.4));
        assert!(!overlap.tiles.is_empty(), "overlap view produced no tiles");
        for tile in &overlap.tiles {
            let k = tile_key(tile);
            assert!(base_keys.contains(&k), "pan: overlap tile missing/changed in base view");
            assert!(panned_keys.contains(&k), "pan: overlap tile missing/changed in panned view");
        }
    }
}
