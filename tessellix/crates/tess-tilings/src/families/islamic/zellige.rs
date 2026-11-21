//! Zellige (Islamic geometric) patterns.
//!
//! * `{8/3}` star and cross — {8/3} octagram stars with concave crosses on
//!   a square lattice (p4m). One-parameter family: the star tip radius `T`
//!   (lattice constant 1) ranges over (0, 1/2]. At T = 1/2 the cross arms
//!   touch the star tips exactly (the classical khatam / "kissing"
//!   arrangement, a 16-gon cross); for T < 1/2 a flat gap opens along each
//!   cell edge and the cross grows to a 20-gon.
//!
//! Unlike the Archimedean tables, these tiles are concave polygons; the
//! geometry engine's even-odd containment and Sutherland–Hodgman clipping
//! handle them without special cases.

use std::f64::consts::SQRT_2;

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamId, ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, MAX_TILES, SIMILARITY_PARAMS};

/// Star tip radius as a fraction of the lattice constant. Range (0, 1/2];
/// 1/2 is the kissing (khatam) case, the classical default.
pub const STAR_RADIUS_ID: ParamId = ParamId::new("star_radius");

/// Parameters: the standard scale/rotation, plus the star radius.
pub const STAR_CROSS_PARAMS: [ParamSpec; 3] = [
    SIMILARITY_PARAMS[0],
    SIMILARITY_PARAMS[1],
    ParamSpec::float(STAR_RADIUS_ID, "star radius", 0.05, 0.5, 0.01, 0.5),
];

/// {8/3} star and cross tiling — one-parameter family (p4m, square lattice).
pub struct StarCrossTiling;

/// Remove consecutive duplicate vertices (and a wraparound duplicate), so
/// the kissing case (T = 1/2) yields a clean 16-gon instead of a 20-gon
/// with four zero-length edges.
fn dedupe_consecutive(pts: Vec<Vec2>) -> Vec<Vec2> {
    let mut out = Vec::with_capacity(pts.len());
    for p in pts {
        if out.last().is_none_or(|q| (p - *q).length() > 1e-12) {
            out.push(p);
        }
    }
    if out.len() > 1 && (out[0] - *out.last().unwrap()).length() <= 1e-12 {
        out.pop();
    }
    out
}

impl Tiling for StarCrossTiling {
    fn id(&self) -> &'static str {
        "star-cross"
    }

    fn name(&self) -> &'static str {
        "Star and Cross ({8/3})"
    }

    fn description(&self) -> &'static str {
        "Islamic star and cross pattern — {8/3} octagram stars with concave crosses, one-parameter family from wide gap to the kissing (khatam) case (p4m). Vertex figures 45.45.135.135 / 45.315 / 90.270 at the kissing radius."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Islamic
    }

    fn declared_symmetry(&self) -> Option<SymmetryMeta> {
        // The lattice is parameter-independent: the star radius scales the
        // tiles, never the translations.
        Some(SymmetryMeta::new(WallpaperGroup::P4m, LatticeKind::Square))
    }

    fn lattice_basis(&self) -> Option<[Vec2; 2]> {
        Some([Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)])
    }

    fn params(&self) -> &'static [ParamSpec] {
        &STAR_CROSS_PARAMS
    }

    fn default_color_mode(&self) -> ColorMode {
        ColorMode::Pattern
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let map = similarity_map(params);
        let l1 = map.transform(Vec2::new(1.0, 0.0));
        let l2 = map.transform(Vec2::new(0.0, 1.0));
        let meta = SymmetryMeta::new(WallpaperGroup::P4m, LatticeKind::Square)
            .with_notes("one-parameter family; star radius 0.5 = kissing (khatam) case");
        let fail = |message: &'static str| TilingResult {
            tiles: Vec::new(),
            symmetry: meta,
            periodic: Some([l1, l2]),
            message: Some(message),
        };

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

        // Star tip radius in cell units (lattice constant is 1).
        let t = params.float(STAR_RADIUS_ID, 0.5).clamp(0.05, 0.5);

        // Pre-generation budget (mirrors lattice.rs): the index range spans
        // ~region_area cells (area 1 each) plus a 1-cell pad, and each cell
        // emits at most two tiles (star + cross).
        let n_cells = (region.size().x + 3.0).ceil() * (region.size().y + 3.0).ceil();
        if n_cells * 2.0 > MAX_TILES as f64 {
            return fail("too many tiles for this view — zoom in");
        }
        let i0 = rmin.x.floor() as i64 - 1;
        let i1 = rmax.x.ceil() as i64 + 1;
        let j0 = rmin.y.floor() as i64 - 1;
        let j1 = rmax.y.ceil() as i64 + 1;

        // Exact coordinates (cell units). The {8/3} star has 8 tips at
        // radius t on the 45° grid and 8 notches at radius t·√((2−√2)/2)
        // between them; the derived offsets used below are all exact:
        //   t2 = t/2          (notch offset along the 22.5° direction, ×√2·cos22.5°)
        //   ts = t·√2/2       (diagonal tip coordinate)
        //   tn = t(√2−1)/2    (notch offset perpendicular to the tip axis)
        // (t2 = t·sin²22.5°·2, tn = t·sin22.5°·cos22.5°·… — all simplify to
        //  these forms; verified against the {8/3} octagram geometry.)
        let t2 = 0.5 * t;
        let ts = 0.5 * SQRT_2 * t;
        let tn = 0.5 * (SQRT_2 - 1.0) * t;

        // {8/3} star, CCW, centered at the cell origin.
        let star = [
            Vec2::new(t, 0.0),
            Vec2::new(t2, tn),
            Vec2::new(ts, ts),
            Vec2::new(tn, t2),
            Vec2::new(0.0, t),
            Vec2::new(-tn, t2),
            Vec2::new(-ts, ts),
            Vec2::new(-t2, tn),
            Vec2::new(-t, 0.0),
            Vec2::new(-t2, -tn),
            Vec2::new(-ts, -ts),
            Vec2::new(-tn, -t2),
            Vec2::new(0.0, -t),
            Vec2::new(tn, -t2),
            Vec2::new(ts, -ts),
            Vec2::new(t2, -tn),
        ];

        // Concave cross filling the unit cell [0,1]², CCW. Four flat
        // segments lie on the cell edges (length 1−2t each); the four
        // corner "bites" follow the star outline (two notches + one
        // diagonal tip per corner). At t = 1/2 the edge segments vanish
        // and the eight edge vertices collapse to four, giving the
        // classical 16-gon.
        let cross = dedupe_consecutive(vec![
            Vec2::new(t, 0.0),
            Vec2::new(1.0 - t, 0.0),
            Vec2::new(1.0 - t2, tn),
            Vec2::new(1.0 - ts, ts),
            Vec2::new(1.0 - tn, t2),
            Vec2::new(1.0, t),
            Vec2::new(1.0, 1.0 - t),
            Vec2::new(1.0 - tn, 1.0 - t2),
            Vec2::new(1.0 - ts, 1.0 - ts),
            Vec2::new(1.0 - t2, 1.0 - tn),
            Vec2::new(1.0 - t, 1.0),
            Vec2::new(t, 1.0),
            Vec2::new(t2, 1.0 - tn),
            Vec2::new(ts, 1.0 - ts),
            Vec2::new(tn, 1.0 - t2),
            Vec2::new(0.0, 1.0 - t),
            Vec2::new(0.0, t),
            Vec2::new(tn, t2),
            Vec2::new(ts, ts),
            Vec2::new(t2, tn),
        ]);

        let mut tiles = Vec::with_capacity(n_cells as usize * 2);
        for j in j0..=j1 {
            for i in i0..=i1 {
                let ox = i as f64;
                let oy = j as f64;
                let tr = Vec2::new(ox, oy);
                // Cross filling cell [i, i+1] × [j, j+1] (proto/seed 0 —
                // the largest tile).
                let mut cp = Vec::with_capacity(cross.len());
                for p in &cross {
                    cp.push(*p + tr);
                }
                let poly = Polygon::from_slice(&cp).transform(&map);
                if poly.aabb().intersects(query.region) {
                    tiles.push(Tile { proto: 0, seed: 0, poly });
                }
                // Star centered on lattice point (i, j) (proto/seed 1).
                let mut sp = Vec::with_capacity(star.len());
                for p in &star {
                    sp.push(*p + tr);
                }
                let poly = Polygon::from_slice(&sp).transform(&map);
                if poly.aabb().intersects(query.region) {
                    tiles.push(Tile { proto: 1, seed: 1, poly });
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
