//! Penrose P2 (kites & darts) tiling by half-tile substitution.
//!
//! The tiling is built from *half-tiles* glued along a leg edge:
//! * `A` — a kite half: 36°-72°-72°, legs φ (apex→g and apex→w), base 1;
//!   two A halves glued along a φ leg form a **kite** (angles
//!   72°, 72°, 72°, 144°, sides φ, 1, 1, φ);
//! * `O` — a dart half: 36°-108°-36°, sides apex→g = 1, apex→w = φ;
//!   two O halves glued along a length-1 leg form a **dart** (angles
//!   36°, 36°, 72°, 216°, sides φ, 1, 1, φ).
//!
//! Substitution by 1/φ (deflation view), on the half-tiles:
//!
//! ```text
//! A -> 2 A + 1 O      (kite -> 2 kites + 1 dart)
//! O -> 1 A + 1 O      (dart -> 1 kite + 1 dart)
//! ```
//!
//! The rule tables place each child in the φ-inflated canonical parent
//! frame; `Half::frame` maps that frame onto the actual parent (a general
//! linear map, so mirrored parents are handled automatically). Half-darts
//! pair *across* parent boundaries — this is why the rules are only valid
//! on patches whose interior tiles are complete (the sun/star seeds), the
//! classical caveat for P2's half-tile rules.
//!
//! The child placements are the unique globally consistent choice for
//! this half-tile labelling (found by an exhaustive 8192-combination
//! search over frame/glue label assignments pruned by sun/star deflation
//! checks). They are verified in exact arithmetic over
//! K = Q(φ, sin 36°): each rule exactly tiles the φ-inflated parent, and
//! the grown sun/star patches satisfy — exactly, at every generation —
//! the glue census (≤ 2 halves per glue edge, opposite sides), the exact
//! kite/dart side patterns of merged tiles, edge census ≤ 2, absence of
//! T-junctions, boundary-only lone glue edges, the Penrose count law
//! K′ = 2K + D, D′ = K + D, and the exact patch area. The Rust tests
//! re-check all of this in float arithmetic.
//!
//! Generation: seed with the "sun" (5 kites, a regular decagon) or the
//! "star" (5 darts) covering the requested region, then repeatedly
//! refine — each half is replaced by its children at 1/φ scale, which
//! tile the parent exactly, so no overlaps can occur. Tiles are culled to
//! the region at every step (a child never leaves its parent), until the
//! tile scale drops to 1. Finally halves are paired back into kites and
//! darts (boundary halves stay triangles, as in the P3 generator).

use std::collections::BTreeMap;

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamId, ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::TilingCategory;
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, MAX_TILES, ROTATION_ID, SCALE_ID};

const PHI: f64 = 1.618_033_988_749_895; // (1+√5)/2
const S36: f64 = 0.587_785_252_292_473_1; // sin 36°
const P2: f64 = PHI * PHI; // φ²
const P3H: f64 = 0.5 * PHI * PHI * PHI; // φ³/2
const P2H: f64 = 0.5 * PHI * PHI; // φ²/2
const PS: f64 = PHI * S36; // φ·sin36° = sin 72°
const P2S: f64 = PHI * PHI * S36; // φ²·sin36°
const STEP_CAP: usize = 24; // φ^24 ≈ 1e5 units of seed scale

pub const SEED_STAR_ID: ParamId = ParamId::new("star seed");

/// Generator parameters: the similarity map plus the seed choice.
const PARAMS: [ParamSpec; 3] = [
    ParamSpec::float(SCALE_ID, "scale", 0.1, 50.0, 0.05, 1.0),
    ParamSpec::angle(ROTATION_ID, "rotation (deg)", 0.0, 360.0, 5.0, 0.0),
    ParamSpec::bool(SEED_STAR_ID, "star seed (else sun)", false),
];

/// A half-tile. `apex` is the 36° vertex; `g` and `w` are the remaining
/// vertices in the canonical frame roles (canonical A: apex (0,0),
/// g (φ,0), w (φ²/2, φ·S36); canonical O: apex (0,0), g (1,0),
/// w (φ²/2, φ·S36)). `glue` indexes the two vertices of `pts()` joined
/// by the glue edge — an A half glues along a φ leg, an O half along its
/// length-1 leg; bases are never glue edges.
#[derive(Clone, Copy, Debug)]
struct Half {
    obtuse: bool, // O (dart half) when true, A (kite half) when false
    apex: Vec2,
    g: Vec2,
    w: Vec2,
    glue: (u8, u8),
}

impl Half {
    fn pts(&self) -> [Vec2; 3] {
        [self.apex, self.g, self.w]
    }

    /// Rigid (rotation *or* reflection) affine map taking the canonical
    /// parent frame to this half: apex ↦ apex, canonical g0 ↦ g − apex,
    /// canonical w0 ↦ w − apex. Returns
    /// `[m00, m01, m10, m11, tx, ty]` with `z' = M z + t`.
    fn frame(&self) -> [f64; 6] {
        let (g0x, g0y) = if self.obtuse { (1.0, 0.0) } else { (PHI, 0.0) };
        let (w0x, w0y) = (P2H, PS);
        let u = self.g - self.apex;
        let v = self.w - self.apex;
        let det = g0x * w0y - g0y * w0x;
        let inv = 1.0 / det;
        let m00 = (u.x * w0y - v.x * g0y) * inv;
        let m01 = (v.x * g0x - u.x * w0x) * inv;
        let m10 = (u.y * w0y - v.y * g0y) * inv;
        let m11 = (v.y * g0x - u.y * w0x) * inv;
        [m00, m01, m10, m11, self.apex.x, self.apex.y]
    }

    /// Substitution children in this half's own coordinates, one
    /// generation down (at 1/φ of this half's scale). The rule tables
    /// below place children in the φ-inflated canonical parent frame
    /// (A parent: apex (0,0), G (φ²,0), W (φ³/2, φ²·S36); O parent:
    /// apex (0,0), G (φ,0), W (φ³/2, φ²·S36)); mapping them through the
    /// frame map divided by φ tiles the parent exactly (deflation view).
    fn children(&self) -> Vec<Half> {
        const RULE_A: [(bool, [Vec2; 3], (u8, u8)); 3] = [
            // A at the inflated g vertex, glued to sibling along apex-w.
            (false, [Vec2::new(P2, 0.0), Vec2::new(P3H, P2S), Vec2::new(P2H, PS)], (0, 2)),
            // A at the inflated g vertex, mirror layout, glued to sibling.
            (false, [Vec2::new(P2, 0.0), Vec2::new(1.0, 0.0), Vec2::new(P2H, PS)], (0, 2)),
            // O at the parent apex, glue edge on the parent's g edge.
            (true, [Vec2::ZERO, Vec2::new(1.0, 0.0), Vec2::new(P2H, PS)], (0, 1)),
        ];
        const RULE_O: [(bool, [Vec2; 3], (u8, u8)); 2] = [
            // A at the parent apex (mirror layout), glue on parent's w edge.
            (false, [Vec2::ZERO, Vec2::new(P2H, PS), Vec2::new(PHI, 0.0)], (0, 2)),
            // O at the parent w vertex, glue on the parent's g edge.
            (true, [Vec2::new(P3H, P2S), Vec2::new(P2H, PS), Vec2::new(PHI, 0.0)], (0, 1)),
        ];
        let m = self.frame();
        let f = |p: Vec2| {
            let x = p.x / PHI;
            let y = p.y / PHI;
            Vec2::new(m[0] * x + m[1] * y + m[4], m[2] * x + m[3] * y + m[5])
        };
        let rule: &[(bool, [Vec2; 3], (u8, u8))] =
            if self.obtuse { &RULE_O } else { &RULE_A };
        rule.iter()
            .map(|&(obtuse, pts, glue)| Half {
                obtuse,
                apex: f(pts[0]),
                g: f(pts[1]),
                w: f(pts[2]),
                glue,
            })
            .collect()
    }

    fn aabb(&self) -> BBox {
        let mut lo = self.apex;
        let mut hi = self.apex;
        for &q in &[self.g, self.w] {
            lo.x = lo.x.min(q.x);
            lo.y = lo.y.min(q.y);
            hi.x = hi.x.max(q.x);
            hi.y = hi.y.max(q.y);
        }
        BBox::new(lo, hi)
    }

    /// Third vertex (the one off the glue edge).
    fn third(&self) -> Vec2 {
        let pts = self.pts();
        let (i, j) = (self.glue.0 as usize, self.glue.1 as usize);
        (0..3).find(|&k| k != i && k != j).map(|k| pts[k]).unwrap()
    }

    /// Point-in-triangle (boundary counts as inside), orientation-agnostic.
    #[cfg(test)]
    fn contains(&self, q: Vec2) -> bool {
        let pts = self.pts();
        let d1 = (pts[1] - pts[0]).cross(q - pts[0]);
        let d2 = (pts[2] - pts[1]).cross(q - pts[1]);
        let d3 = (pts[0] - pts[2]).cross(q - pts[2]);
        (d1 >= -1e-9 && d2 >= -1e-9 && d3 >= -1e-9)
            || (d1 <= 1e-9 && d2 <= 1e-9 && d3 <= 1e-9)
    }

    #[cfg(test)]
    fn area(&self) -> f64 {
        (self.g - self.apex).cross(self.w - self.apex).abs() * 0.5
    }
}

/// The "sun" seed: 5 kites meeting at the origin — a regular decagon of
/// circumradius φ·scale — as 10 A halves.
fn seed_sun(scale: f64) -> Vec<Half> {
    let mut v = Vec::with_capacity(10);
    let d36 = 36.0_f64.to_radians();
    for k in 0..5 {
        let th = std::f64::consts::FRAC_PI_2 + k as f64 * 2.0 * std::f64::consts::PI / 5.0;
        let tp = Vec2::new(th.cos(), th.sin()) * (PHI * scale);
        let wp = Vec2::new((th + d36).cos(), (th + d36).sin()) * (PHI * scale);
        let wm = Vec2::new((th - d36).cos(), (th - d36).sin()) * (PHI * scale);
        v.push(Half { obtuse: false, apex: Vec2::ZERO, g: tp, w: wp, glue: (0, 1) });
        v.push(Half { obtuse: false, apex: Vec2::ZERO, g: tp, w: wm, glue: (0, 1) });
    }
    v
}

/// The "star" seed: 5 darts meeting at the origin — 10 O halves.
fn seed_star(scale: f64) -> Vec<Half> {
    let mut v = Vec::with_capacity(10);
    let d36 = 36.0_f64.to_radians();
    for k in 0..5 {
        let th = std::f64::consts::FRAC_PI_2 + k as f64 * 2.0 * std::f64::consts::PI / 5.0;
        let gp = Vec2::new(th.cos(), th.sin()) * scale;
        let wp = Vec2::new((th + d36).cos(), (th + d36).sin()) * (PHI * scale);
        let wm = Vec2::new((th - d36).cos(), (th - d36).sin()) * (PHI * scale);
        v.push(Half { obtuse: true, apex: Vec2::ZERO, g: gp, w: wp, glue: (0, 1) });
        v.push(Half { obtuse: true, apex: Vec2::ZERO, g: gp, w: wm, glue: (0, 1) });
    }
    v
}

/// One refinement (deflation) step: replace every half by its children
/// (scale 1/φ), which exactly tile the parent.
///
/// Culling is sound: a child never leaves its parent, so a final tile
/// intersecting `region` has an ancestor chain whose aabbs all intersect
/// `region` — nothing needed is ever dropped.
fn refine_step(halves: &[Half], region: &BBox) -> Vec<Half> {
    let mut out = Vec::with_capacity(halves.len() * 3);
    for h in halves {
        for ch in h.children() {
            if ch.aabb().intersects(*region) {
                out.push(ch);
            }
        }
    }
    out
}

/// Key of the unordered glue segment `(a, b)`: min of the rounded endpoint
/// pairs. Halves sharing a glue edge get the same key; halves touching
/// only at a point do not.
fn seg_key(a: Vec2, b: Vec2) -> (i64, i64, i64, i64) {
    let (ax, ay) = ((a.x * 1e7).round() as i64, (a.y * 1e7).round() as i64);
    let (bx, by) = ((b.x * 1e7).round() as i64, (b.y * 1e7).round() as i64);
    if (ax, ay) <= (bx, by) {
        (ax, ay, bx, by)
    } else {
        (bx, by, ax, ay)
    }
}

/// An assembled patch: merged kites/darts plus boundary lone halves.
struct Assembled {
    /// `(is_dart, vertices)` for every merged kite/dart (quads) and lone
    /// half (triangles).
    polys: Vec<(bool, Vec<Vec2>)>,
}

/// Pair halves along their glue edges: a glued pair forms a kite (A) or
/// dart (O); a lone half is kept as its triangle (patch boundary).
/// Errs if some glue edge is claimed by more than two halves.
fn assemble(halves: &[Half]) -> Result<Assembled, &'static str> {
    let mut groups: BTreeMap<(i64, i64, i64, i64), Vec<usize>> = BTreeMap::new();
    for (i, h) in halves.iter().enumerate() {
        let pts = h.pts();
        let key = seg_key(pts[h.glue.0 as usize], pts[h.glue.1 as usize]);
        groups.entry(key).or_default().push(i);
    }
    let mut polys = Vec::with_capacity(groups.len());
    for (_, idxs) in groups {
        match idxs.as_slice() {
            [i, j] => {
                let a = halves[*i];
                let b = halves[*j];
                let pa = a.pts();
                let quad = vec![
                    pa[a.glue.0 as usize],
                    a.third(),
                    pa[a.glue.1 as usize],
                    b.third(),
                ];
                polys.push((a.obtuse, quad));
            }
            [i] => {
                let h = halves[*i];
                polys.push((h.obtuse, vec![h.apex, h.g, h.w]));
            }
            _ => return Err("internal error: glue edge shared by >2 halves"),
        }
    }
    Ok(Assembled { polys })
}

/// Penrose P2 kite-dart tiling (aperiodic, 5-fold local symmetry).
pub struct PenroseP2Tiling;

impl Tiling for PenroseP2Tiling {
    fn id(&self) -> &'static str {
        "penrose_p2"
    }

    fn name(&self) -> &'static str {
        "Penrose P2 (kite and dart)"
    }

    fn description(&self) -> &'static str {
        "Quasiperiodic kite-dart tiling by half-tile substitution; local 5-fold symmetry."
    }

    fn category(&self) -> TilingCategory {
        TilingCategory::Quasiperiodic
    }

    fn params(&self) -> &'static [ParamSpec] {
        &PARAMS
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

        // Pre-generation budget (mirrors lattice.rs): no P2 tile exceeds
        // the kite area φ²·sin36° at unit scale, so region.area()/(φ²·
        // sin36°·s²) is a lower bound on the tile count — checked before
        // any refinement work so an extreme zoom-out cannot OOM first.
        let s = params.float(SCALE_ID, 1.0);
        if query.region.area() / (P2 * S36 * s * s) > MAX_TILES as f64 {
            return fail("too many tiles for this view — zoom in");
        }

        // Seed scale: the patch's incircle must cover the region box.
        // Sun: a regular decagon of circumradius φ·scale, apothem
        // φ²·sin36°·scale. Star: dart inner vertices at distance
        // 1·scale are the closest boundary points.
        let star = params.bool(SEED_STAR_ID, false);
        let in_r = if star { 1.0 } else { P2 * S36 };
        let mut scale = 0.0f64;
        for &corner in &region.corners() {
            scale = scale.max(corner.length());
        }
        if scale < 1e-9 {
            return fail("empty view");
        }
        scale /= in_r;

        // Refine until the tile scale reaches 1 (the patch stays put:
        // children tile their parents; each step divides the scale by φ).
        let mut halves = if star { seed_star(scale) } else { seed_sun(scale) };
        // Substitution depth actually applied (level coloring).
        let mut depth = 0_usize;
        for _ in 0..STEP_CAP {
            if scale <= 1.0 + 1e-12 {
                break;
            }
            halves = refine_step(&halves, &region);
            scale /= PHI;
            depth += 1;
        }
        if halves.is_empty() || scale > 1.0 + 1e-12 {
            return fail("view too far from the seed — pan back or zoom in");
        }

        // Pair halves back into kites/darts and emit.
        let asm = match assemble(&halves) {
            Ok(a) => a,
            Err(msg) => return fail(msg),
        };
        let mut tiles = Vec::with_capacity(asm.polys.len());
        for (is_dart, verts) in &asm.polys {
            let proto = if *is_dart { 1 } else { 0 };
            let poly = Polygon::from_slice(verts).transform(&map);
            if poly.aabb().intersects(query.region) {
                tiles.push(Tile {
                    proto,
                    seed: proto as u32, level: depth.min(255) as u8,
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

    /// Deterministic xorshift64 rng (same style as the lib tests).
    struct Rng(u64);
    impl Rng {
        fn f64(&mut self) -> f64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            (v >> 11) as f64 * (1.0 / 9007199254740992.0)
        }
    }

    fn canonical_a() -> Half {
        Half {
            obtuse: false,
            apex: Vec2::ZERO,
            g: Vec2::new(PHI, 0.0),
            w: Vec2::new(P2H, PS),
            glue: (0, 1),
        }
    }

    fn canonical_o() -> Half {
        Half {
            obtuse: true,
            apex: Vec2::ZERO,
            g: Vec2::new(1.0, 0.0),
            w: Vec2::new(P2H, PS),
            glue: (0, 1),
        }
    }

    /// The children of `h` must exactly tile `h` itself (deflation view):
    /// every sampled point inside h lies in exactly one child, and the
    /// child areas sum to the parent's area.
    fn check_substitution(h: &Half, label: &str) {
        let kids = h.children();
        let pa = h.area();
        let kids_area: f64 = kids.iter().map(|k| k.area()).sum();
        assert!(
            (kids_area - pa).abs() <= 1e-9 * pa,
            "{label}: child areas {kids_area} != parent {pa}"
        );

        let bb = h.aabb();
        let mut rng = Rng(0x517C_C1B7_2722_0A95);
        let mut checked = 0;
        for _ in 0..4000 {
            let q = Vec2::new(
                bb.min.x + rng.f64() * bb.size().x,
                bb.min.y + rng.f64() * bb.size().y,
            );
            if !h.contains(q) {
                continue;
            }
            let n = kids.iter().filter(|k| k.contains(q)).count();
            assert!(n == 1, "{label}: point {q:?} covered by {n} children");
            checked += 1;
        }
        assert!(checked > 1500, "{label}: too few samples landed");
    }

    #[test]
    fn kite_half_substitution_tiles_parent() {
        check_substitution(&canonical_a(), "A -> 2A + O");
    }

    #[test]
    fn dart_half_substitution_tiles_parent() {
        check_substitution(&canonical_o(), "O -> A + O");
    }

    // ---------- global consistency: refine the sun/star ----------
    //
    // Deflation is a refinement (children tile their parent exactly), so
    // overlaps and gaps are impossible by construction; what must hold is
    // the *labelling*: glue edges pair same-type halves from opposite
    // sides into proper kites/darts, boundary lone halves stay on the
    // boundary, the kite/dart census obeys the Penrose law
    // K' = 2K + D, D' = K + D, merged patches are edge-to-edge, and the
    // area is preserved exactly.

    /// Substitute every half once (no culling).
    fn refine_all(halves: &[Half]) -> Vec<Half> {
        let mut out = Vec::with_capacity(halves.len() * 3);
        for h in halves {
            out.extend(h.children());
        }
        out
    }

    /// Does point `q` lie strictly in the interior of segment `a-b`?
    fn on_edge_interior(q: Vec2, a: Vec2, b: Vec2) -> bool {
        let ab = b - a;
        let aq = q - a;
        if aq.cross(ab).abs() > 1e-9 {
            return false;
        }
        let s = aq.dot(ab);
        let l2 = ab.dot(ab);
        s > 1e-9 && s < l2 - 1e-9
    }

    fn poly_area(v: &[Vec2]) -> f64 {
        let n = v.len();
        (0..n)
            .map(|i| v[i].cross(v[(i + 1) % n]))
            .sum::<f64>()
            .abs()
            * 0.5
    }

    /// Census of an assembled patch: (kites, darts, lone_a, lone_o).
    /// Merged tiles are quads, lone halves triangles.
    fn census(asm: &Assembled) -> (usize, usize, usize, usize) {
        let mut c = (0, 0, 0, 0);
        for (dart, v) in &asm.polys {
            match (*dart, v.len()) {
                (false, 4) => c.0 += 1,
                (true, 4) => c.1 += 1,
                (false, 3) => c.2 += 1,
                (true, 3) => c.3 += 1,
                _ => panic!("unexpected assembled polygon of len {}", v.len()),
            }
        }
        c
    }

    #[test]
    fn refined_sun_counts_follow_penrose_law() {
        // K' = 2K + D, D' = K + D, starting from the sun (K,D) = (5,0):
        // lone boundary halves complete to whole tiles in the limit, so
        // the census counts them as halves of tiles (they must pair up
        // evenly). Sun: (10,5), (25,15), (65,40).
        let mut halves = seed_sun(1.0);
        let (mut pk, mut pd) = (5usize, 0usize);
        for depth in 0..3u32 {
            halves = refine_all(&halves);
            let asm = assemble(&halves).expect("assembly");
            let (_, _, la, lo) = census(&asm);
            assert_eq!(la % 2, 0, "depth {depth}: odd lone A halves");
            assert_eq!(lo % 2, 0, "depth {depth}: odd lone O halves");
            let (kites, darts, _, _) = census(&asm);
            let (fk, fd) = (kites + la / 2, darts + lo / 2);
            assert_eq!(
                (fk, fd),
                (2 * pk + pd, pk + pd),
                "depth {depth}: census ({fk}, {fd}) != ({}, {})",
                2 * pk + pd,
                pk + pd
            );
            (pk, pd) = (fk, fd);
        }
    }

    #[test]
    fn refined_star_counts_follow_penrose_law() {
        // Star: (K,D) = (0,5) -> (5,5) -> (15,10) -> (40,25).
        let mut halves = seed_star(1.0);
        let (mut pk, mut pd) = (0usize, 5usize);
        for depth in 0..3u32 {
            halves = refine_all(&halves);
            let asm = assemble(&halves).expect("assembly");
            let (_, _, la, lo) = census(&asm);
            assert_eq!(la % 2, 0, "depth {depth}: odd lone A halves");
            assert_eq!(lo % 2, 0, "depth {depth}: odd lone O halves");
            let (kites, darts, _, _) = census(&asm);
            let (fk, fd) = (kites + la / 2, darts + lo / 2);
            assert_eq!(
                (fk, fd),
                (2 * pk + pd, pk + pd),
                "depth {depth}: census ({fk}, {fd}) != ({}, {})",
                2 * pk + pd,
                pk + pd
            );
            (pk, pd) = (fk, fd);
        }
    }

    /// Full structural check of a refined patch at depth `depth`:
    /// exact area, glue census, edge census <= 2, no T-junctions, and
    /// lone glue edges only on the patch boundary.
    fn check_refined_patch(halves: &[Half], seed_area: f64, label: &str) {
        // glue census: <= 2 halves per glue edge, same type, opposite sides
        let mut groups: BTreeMap<(i64, i64, i64, i64), Vec<usize>> = BTreeMap::new();
        for (i, h) in halves.iter().enumerate() {
            let pts = h.pts();
            groups
                .entry(seg_key(pts[h.glue.0 as usize], pts[h.glue.1 as usize]))
                .or_default()
                .push(i);
        }
        for idxs in groups.values() {
            if idxs.len() == 2 {
                let a = halves[idxs[0]];
                let b = halves[idxs[1]];
                let pa = a.pts();
                let g0 = pa[a.glue.0 as usize];
                let g1 = pa[a.glue.1 as usize];
                let dir = g1 - g0;
                let s1 = dir.cross(a.third() - g0);
                let s2 = dir.cross(b.third() - g0);
                assert_eq!(a.obtuse, b.obtuse, "{label}: glue pairs A with O");
                assert!(s1 * s2 < 0.0, "{label}: glued halves on the same side");
            }
        }
        let asm = assemble(halves).expect("assembly");

        // exact area
        let area: f64 = asm.polys.iter().map(|(_, v)| poly_area(v)).sum();
        assert!(
            (area - seed_area).abs() <= 1e-9 * seed_area,
            "{label}: area {area} != {seed_area}"
        );

        // edge census <= 2
        let mut edges: BTreeMap<(i64, i64, i64, i64), usize> = BTreeMap::new();
        for (_, v) in &asm.polys {
            let n = v.len();
            for i in 0..n {
                let key = seg_key(v[i], v[(i + 1) % n]);
                *edges.entry(key).or_insert(0) += 1;
            }
        }
        assert!(
            edges.values().all(|&c| c <= 2),
            "{label}: some edge shared by >2 tiles"
        );

        // no T-junctions at the merged-tile level
        for (i, (_, v)) in asm.polys.iter().enumerate() {
            for &p in v {
                for (j, (_, w)) in asm.polys.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    for k in 0..w.len() {
                        assert!(
                            !on_edge_interior(p, w[k], w[(k + 1) % w.len()]),
                            "{label}: T-junction at {p:?}"
                        );
                    }
                }
            }
        }

        // lone glue edges must be patch-boundary edges
        for (seg, idxs) in &groups {
            if idxs.len() == 1 {
                assert_eq!(edges[seg], 1, "{label}: interior lone glue {seg:?}");
            }
        }
    }

    #[test]
    fn refined_sun_is_edge_to_edge() {
        let sun_area = 5.0 * P2 * S36;
        let mut halves = seed_sun(1.0);
        for depth in 1..=2u32 {
            halves = refine_all(&halves);
            check_refined_patch(&halves, sun_area, &format!("sun g{depth}"));
        }
    }

    #[test]
    fn refined_star_is_edge_to_edge() {
        let star_area = 5.0 * PHI * S36;
        let mut halves = seed_star(1.0);
        for depth in 1..=2u32 {
            halves = refine_all(&halves);
            check_refined_patch(&halves, star_area, &format!("star g{depth}"));
        }
    }

    #[test]
    fn merged_tiles_have_kite_dart_shape() {
        // Every glued pair must form a proper kite (72,72,72,144°) or
        // dart (36,36,72,216°) with side pattern (φ,1,1,φ)·scale.
        let halves = {
            let mut h = seed_sun(1.0);
            for _ in 0..2 {
                h = refine_all(&h);
            }
            h
        };
        let mut groups: BTreeMap<(i64, i64, i64, i64), Vec<usize>> = BTreeMap::new();
        for (i, h) in halves.iter().enumerate() {
            let pts = h.pts();
            groups
                .entry(seg_key(pts[h.glue.0 as usize], pts[h.glue.1 as usize]))
                .or_default()
                .push(i);
        }
        let s2 = P2 * P2; // φ⁴: two refinement steps shrink edges by φ²
        let (long2, short2) = (PHI * PHI / s2, 1.0 / s2);
        let mut checked = 0;
        for (_, idxs) in groups {
            if idxs.len() != 2 {
                continue;
            }
            let a = halves[idxs[0]];
            let b = halves[idxs[1]];
            let pa = a.pts();
            let quad = [
                pa[a.glue.0 as usize],
                a.third(),
                pa[a.glue.1 as usize],
                b.third(),
            ];
            let ds2: Vec<f64> = (0..4)
                .map(|i| (quad[(i + 1) % 4] - quad[i]).dot(quad[(i + 1) % 4] - quad[i]))
                .collect();
            let want = [long2, short2, short2, long2];
            assert!(
                (0..4).any(|r| (0..4)
                    .all(|k| (ds2[k] - want[(k + r) % 4]).abs() <= 1e-9 * long2)),
                "merged tile side squares {ds2:?} not a kite/dart"
            );
            checked += 1;
        }
        assert!(checked >= 35, "too few merged tiles checked: {checked}");
    }
}