//! Hexagram ({6/2} six-pointed star) and hexagon pattern.
//!
//! * `{6/2}` star and hexagon — hexagram stars on a hexagonal lattice
//!   (basis a1 = (1, 0), a2 = (1/2, √3/2)), wallpaper group p6m.
//!   One-parameter family: the star tip radius `T` (lattice constant 1)
//!   ranges over [0.05, 1/2]. At T = 1/2 the tips kiss — the classical
//!   hexagram-and-hexagon tiling, in which each cell's interstice splits
//!   into two regular hexagons (area √3/8 each, touching at the cell
//!   center). For T < 1/2 a flat gap opens and each cell carries one
//!   16-vertex interstice polygon. (The interlaced regime T > 1/2 is
//!   out of range; there the stars overlap.)
//!
//! Symmetry (p6m): the 6-fold centers are the lattice points (the star
//! centers); the 3-fold centers C1, C2 sit at P + (a1+a2)/3 and
//! P + 2(a1+a2)/3 — the hexagon centers; mirrors run along the star tip
//! lines (reflections y -> -y and in the a2 line).
//!
//! Per-cell emission (zellige-style): the full star 12-gon at the cell
//! corner P, plus the interstice: one 16-gon for T < 1/2, the two
//! hexagons H(C1), H(C2) at the kissing radius. The 16-gon combinatorics
//! are invariant across the whole range (the only critical value,
//! T = 1/3, where the tips land on the Voronoi-vertex ring, does not
//! change them); at T = 1/2 the 16-gon degenerates — four pairs of
//! consecutive vertices coincide, leaving a pinched cut-vertex polygon
//! that would break triangulation — so the explicit hexagon branch
//! replaces it.
//!
//! Exact per-cell area identities (lattice constant 1, cell area √3/2):
//! star = √3·T², interstice = √3/2 − √3·T² (16-gon), and at T = 1/2
//! √3/4 + √3/8 + √3/8 = √3/2.

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamId, ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// √3 (a local const: `std::f64::consts::SQRT_3` is unstable in this
/// toolchain — the `more_float_constants` gate).
const SQRT_3: f64 = 1.7320508075688773;

/// Star tip radius as a fraction of the lattice constant. Range
/// [0.05, 1/2]; 1/2 is the kissing (hexagram-and-hexagon) case, the
/// classical default.
pub const STAR_RADIUS_ID: ParamId = ParamId::new("star_radius");

/// Parameters: the standard scale/rotation, plus the star tip radius.
pub const HEXAGRAM_PARAMS: [ParamSpec; 3] = [
    SIMILARITY_PARAMS[0],
    SIMILARITY_PARAMS[1],
    ParamSpec::float(STAR_RADIUS_ID, "star tip radius", 0.05, 0.5, 0.01, 0.5),
];

/// {6/2} hexagram and hexagon tiling — one-parameter family (p6m,
/// hexagonal lattice).
pub struct HexagramTiling;

impl Tiling for HexagramTiling {
    fn id(&self) -> &'static str {
        "hexagram"
    }

    fn name(&self) -> &'static str {
        "Hexagram and Hexagon ({6/2})"
    }

    fn description(&self) -> &'static str {
        "Hexagram ({6/2}) six-pointed stars on a hexagonal lattice with hexagonal interstices — one-parameter family from wide gap to the kissing hexagram-and-hexagon tiling (p6m). Vertex figures 60.120.60.120 (kiss points) and 60.60.120.120 (cell centers) at the kissing radius."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Wallpaper
    }

    fn params(&self) -> &'static [ParamSpec] {
        &HEXAGRAM_PARAMS
    }

    fn default_color_mode(&self) -> ColorMode {
        ColorMode::Pattern
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let map = similarity_map(params);
        let l1 = map.transform(Vec2::new(1.0, 0.0));
        let l2 = map.transform(Vec2::new(0.5, 0.5 * SQRT_3));
        let meta = SymmetryMeta::new(WallpaperGroup::P6m, LatticeKind::Hexagonal)
            .with_notes("one-parameter family; star tip radius 0.5 = kissing hexagram-and-hexagon case");
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: Some([l1, l2]),
            message: Some(message),
        };

        let inv = map.inverse();

        // Cell coordinates (i, j) in the tiling's own frame: a point
        // (qx, qy) sits in cell (qx - qy/sqrt3, 2qy/sqrt3), since
        // lattice point P(i,j) = (i + j/2, j*sqrt3/2), so j = 2qy/sqrt3
        // and i = qx - j/2 = qx - qy/sqrt3. The axis-aligned box around
        // the inverse images of the four world corners (min/max of a
        // linear function over the inverted region happens at a vertex)
        // culls the (i, j) loop.
        let mut i_min = f64::MAX;
        let mut i_max = f64::MIN;
        let mut j_min = f64::MAX;
        let mut j_max = f64::MIN;
        for &corner in &query.region.corners() {
            let q = inv.transform(corner);
            let qi = q.x - q.y / SQRT_3;
            let qj = (2.0 / SQRT_3) * q.y;
            i_min = i_min.min(qi);
            i_max = i_max.max(qi);
            j_min = j_min.min(qj);
            j_max = j_max.max(qj);
        }
        let region = BBox::new(Vec2::new(i_min, j_min), Vec2::new(i_max, j_max));
        if region.area() <= 1e-12 {
            return fail("empty view");
        }

        // Star tip radius in cell units (lattice constant is 1).
        let t = params.float(STAR_RADIUS_ID, 0.5).clamp(0.05, 0.5);

        // Pre-generation budget (mirrors lattice.rs): the index range
        // spans ~region_area cells plus a 1-cell pad, and each cell
        // emits at most three tiles (star + two kissing hexagons).
        let n_cells =
            (i_max.ceil() as i64 - i_min.floor() as i64 + 3) * (j_max.ceil() as i64 - j_min.floor() as i64 + 3);
        if n_cells as f64 * 3.0 > MAX_TILES as f64 {
            return fail("too many tiles for this view — zoom in");
        }
        let i0 = i_min.floor() as i64 - 1;
        let i1 = i_max.ceil() as i64 + 1;
        let j0 = j_min.floor() as i64 - 1;
        let j1 = j_max.ceil() as i64 + 1;

        // Exact coordinates (cell units). The {6/2} star has 6 tips at
        // radius t on the 60-degree grid and 6 notches at radius t/sqrt3
        // between them; all offsets below simplify to exact multiples of
        // t and sqrt3*t.
        let t2 = 0.5 * t;
        let s3t2 = 0.5 * SQRT_3 * t; // sqrt3*t/2 (tip y-coordinate)
        let s3t3 = (SQRT_3 / 3.0) * t; // sqrt3*t/3 (notch radius)
        let s3t6 = (SQRT_3 / 6.0) * t; // sqrt3*t/6 (notch offsets)

        // {6/2} star, CCW, centered on the cell corner P.
        let star = [
            Vec2::new(t, 0.0),
            Vec2::new(t2, s3t6),
            Vec2::new(t2, s3t2),
            Vec2::new(0.0, s3t3),
            Vec2::new(-t2, s3t2),
            Vec2::new(-t2, s3t6),
            Vec2::new(-t, 0.0),
            Vec2::new(-t2, -s3t6),
            Vec2::new(-t2, -s3t2),
            Vec2::new(0.0, -s3t3),
            Vec2::new(t2, -s3t2),
            Vec2::new(t2, -s3t6),
        ];

        // Interstice for T < 1/2: the cell P minus the four star corner
        // wedges — a 16-vertex polygon, CCW, with P as its first
        // vertex. Area sqrt3/2 - sqrt3*t^2 (so the star's sqrt3*t^2
        // plus the interstice fills the cell area sqrt3/2). The
        // 16-vertex combinatorics are stable for all T in (0.05, 0.5);
        // at T = 1/2 the edge segments between consecutive tips
        // vanish, so this branch is only taken for T < 1/2.
        let interstice = [
            Vec2::new(t, 0.0),
            Vec2::new(1.0 - t, 0.0),
            Vec2::new(1.0 - t2, s3t6),
            Vec2::new(1.0 - t2, s3t2),
            Vec2::new(1.0, s3t3),
            Vec2::new(1.0 + t2, s3t2),
            Vec2::new(1.5 - t2, 0.5 * SQRT_3 - s3t2),
            Vec2::new(1.5 - t2, 0.5 * SQRT_3 - s3t6),
            Vec2::new(1.5 - t, 0.5 * SQRT_3),
            Vec2::new(0.5 + t, 0.5 * SQRT_3),
            Vec2::new(0.5 + t2, 0.5 * SQRT_3 - s3t6),
            Vec2::new(0.5 + t2, 0.5 * SQRT_3 - s3t2),
            Vec2::new(0.5, 0.5 * SQRT_3 - s3t3),
            Vec2::new(0.5 - t2, 0.5 * SQRT_3 - s3t2),
            Vec2::new(t2, s3t2),
            Vec2::new(t2, s3t6),
        ];

        // Interstice at the kissing radius T = 1/2: the 16-gon above
        // degenerates into two regular hexagons — H(C1) centered at
        // C1 = P + (a1+a2)/3 = P + (1/2, sqrt3/6) and H(C2) centered at
        // C2 = P + (a1+a2) * 2/3 = P + (1, sqrt3/3) — circumradius
        // sqrt3/6 (edge sqrt3/6, area sqrt3/8 each), CCW, touching at
        // the cell center P + (3/4, sqrt3/4).
        let c = SQRT_3 / 6.0; // hexagon circumradius sqrt3/6
        let h1 = [
            Vec2::new(0.75, 1.5 * c),
            Vec2::new(0.5, 2.0 * c),
            Vec2::new(0.25, 1.5 * c),
            Vec2::new(0.25, 0.5 * c),
            Vec2::new(0.5, 0.0),
            Vec2::new(0.75, 0.5 * c),
        ];
        let h2 = [
            Vec2::new(1.25, 2.5 * c),
            Vec2::new(1.0, 3.0 * c),
            Vec2::new(0.75, 2.5 * c),
            Vec2::new(0.75, 1.5 * c),
            Vec2::new(1.0, c),
            Vec2::new(1.25, 1.5 * c),
        ];

        let mut tiles = Vec::with_capacity(n_cells as usize * 3);
        for j in j0..=j1 {
            for i in i0..=i1 {
                // Cell corner P(i,j) = i*a1 + j*a2 in cell units.
                let p = Vec2::new(i as f64 + 0.5 * j as f64, j as f64 * 0.5 * SQRT_3);
                // Star centered on lattice point P(i,j) (proto/seed 1).
                let mut sp = Vec::with_capacity(star.len());
                for q in &star {
                    sp.push(*q + p);
                }
                let poly = Polygon::from_slice(&sp).transform(&map);
                if poly.aabb().intersects(query.region) {
                    tiles.push(Tile { proto: 1, seed: 1, poly });
                }
                // Interstice: two hexagons at the kissing radius (the
                // 16-gon would degenerate to a cut-vertex polygon), one
                // 16-gon otherwise.
                if t == 0.5 {
                    for (proto, hex) in [(2_u8, h1), (3_u8, h2)] {
                        let mut hp = Vec::with_capacity(hex.len());
                        for q in &hex {
                            hp.push(*q + p);
                        }
                        let poly = Polygon::from_slice(&hp).transform(&map);
                        if poly.aabb().intersects(query.region) {
                            tiles.push(Tile {
                                proto,
                                seed: proto as u32,
                                poly,
                            });
                        }
                    }
                } else {
                    let mut ip = Vec::with_capacity(interstice.len());
                    for q in &interstice {
                        ip.push(*q + p);
                    }
                    let poly = Polygon::from_slice(&ip).transform(&map);
                    if poly.aabb().intersects(query.region) {
                        tiles.push(Tile { proto: 0, seed: 0, poly });
                    }
                }
            }
        }

        if tiles.len() > MAX_TILES {
            return fail("too many tiles for this view — zoom in");
        }

        TilingResult {
            tiles,
            symmetry: meta,
            periodic: Some([l1, l2]),
            message: None,
        }
    }
}
