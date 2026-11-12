//! Penrose Pentagon tiling — an aperiodic φ-substitution on five pentagons.
//!
//! Five pentagon prototiles, every edge in {1, 1/φ, 1/φ², φ} and every
//! angle a multiple of 36°; three of the five carry a single 216° reflex
//! vertex:
//!
//! ```text
//! 50:  72·108·72·144·144   area 0.3368
//! 105: 72·108·108·144·108  area 0.5449
//! 451:  36·144·72·72·216   area 0.8817   (= φ² · area of 468)
//! 468:  36·144·72·72·216   area 0.3368
//! 484:  36·72·108·216·108  area 0.5449
//! ```
//!
//! The substitution (inflation by φ; the code runs it in the deflation
//! direction, where children at 1/φ scale tile each parent exactly):
//!
//! ```text
//! 50  -> 105 + 468
//! 105 -> 50 + 105 + 484
//! 451 -> 2·50 + 3·484
//! 468 -> 451
//! 484 -> 484 + 50 + 105
//! ```
//!
//! The tiling is chiral: children appear in both handednesses. The
//! reflection is folded into each deflation map (linear part = (1/φ)·O
//! with O a rotation *or* x-reflection-then-rotation by a multiple of
//! 36°), so the runtime only ever uses the five canonical CCW
//! prototiles.
//!
//! The five-shape set was derived by exhaustive search over a pool of
//! 567 pentagons built from the same edge/angle ring (the strict layer
//! admits no valid set; this one survives the 1-hole completion layer).
//! It is in the spirit of Penrose's "Pentagons and pentacles" (1974) —
//! *not* the P1 tiling, whose prototiles are a pentagon, star, boat and
//! diamond.
//!
//! Generation: seed with a single 451 tile scaled so that its inscribed
//! disk about its centroid (radius `SEED_R`) covers the query region —
//! the region's aabb lies inside the disk through its farthest corner —
//! then repeatedly refine: each tile is replaced by its children at 1/φ
//! scale, which tile the parent exactly, so no overlaps can ever occur.
//! Tiles are aabb-culled to the region at every step (a child never
//! leaves its parent, so nothing needed is dropped) until the tile
//! scale drops to 1.

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::{ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, MAX_TILES, SCALE_ID, SIMILARITY_PARAMS};

const PHI: f64 = 1.618_033_988_749_895; // (1+√5)/2
const IP: f64 = 1.0 / PHI; // 1/φ
const IP2: f64 = IP * IP; // 1/φ²
const IP2H: f64 = 0.5 * IP2; // 1/(2φ²)
const C36: f64 = PHI / 2.0; // cos 36°
const S36: f64 = 0.587_785_252_292_473_1; // sin 36°
const S36I: f64 = S36 * IP; // sin 36°/φ
const S72: f64 = 0.951_056_516_295_153_5; // sin 72°
const A451: f64 = 1.5 * S36; // area of proto 2 = 3·sin 36°/2
const SEED_R: f64 = 0.23; // inscribed-disk margin of 451 about its
                          // centroid (true dmin = 0.2355299302425834,
                          // attained at the reflex vertex)
const STEP_CAP: usize = 24; // φ^24 ≈ 1.7e5 units of seed scale

/// The five canonical prototiles (CCW, v0 at the origin, edge 0 along +x).
fn protos() -> [Polygon; 5] {
    [
        Polygon::from_slice(&[
            Vec2::new(0.0, 0.0),
            Vec2::new(IP, 0.0),
            Vec2::new(C36, S36),
            Vec2::new(C36 - IP2, S36),
            Vec2::new(IP - 0.5, S36I),
        ]),
        Polygon::from_slice(&[
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.5 + IP, S36I),
            Vec2::new(C36, S36),
            Vec2::new(0.5 * IP2, S36),
        ]),
        Polygon::from_slice(&[
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0 + C36, S36),
            Vec2::new(1.0, 2.0 * S36),
            Vec2::new(C36, S36),
        ]),
        Polygon::from_slice(&[
            Vec2::new(0.0, 0.0),
            Vec2::new(IP, 0.0),
            Vec2::new(0.5 + IP, S36I),
            Vec2::new(IP, 2.0 * S36I),
            Vec2::new(0.5, S36I),
        ]),
        Polygon::from_slice(&[
            Vec2::new(0.0, 0.0),
            Vec2::new(PHI, 0.0),
            Vec2::new(1.5, S36I),
            Vec2::new(0.5 + IP, S36I),
            Vec2::new(C36, S36),
        ]),
    ]
}

/// Deflation table, one row per parent proto. Each row is
/// `[child, m00, m01, m10, m11, tx, ty]`: the map
/// `p -> [[m00, m01], [m10, m11]]·p + (tx, ty)` takes the child's
/// canonical polygon to its placement inside the parent, and the row's
/// images tile the parent exactly. Every linear entry is of the form
/// (1/φ)·O with O a rotation or x-reflection-then-rotation by a multiple
/// of 36°, and every translation lies in the ring
/// Q(φ, s/4) with s = √(10−2√5).
const DEF: [&[&[f64; 7]]; 5] = [
    // parent 0 (50) -> 105 + 468
    &[
        &[1.0, -IP2H, -S36, -S36, IP2H, C36, S36],
        &[3.0, -0.5, S36I, -S36I, -0.5, IP - IP2H, S36],
    ],
    // parent 1 (105) -> 484 + 50 + 105
    &[
        &[4.0, -IP, 0.0, 0.0, IP, 1.0, 0.0],
        &[0.0, -IP, 0.0, 0.0, -IP, 1.5 * IP2, S36],
        &[1.0, -0.5, S36I, S36I, 0.5, 1.0, 0.0],
    ],
    // parent 2 (451) -> 484 + 50 + 484' + 50' + 484''
    &[
        &[4.0, IP, 0.0, 0.0, IP, 0.0, 0.0],
        &[0.0, -0.5, -S36I, S36I, -0.5, 1.5 * IP + IP2H, S36I],
        &[4.0, -0.5, -S36I, -S36I, 0.5, 1.0 + 0.5 * PHI, S36],
        &[0.0, -IP2H, S36, -S36, -IP2H, 1.5 * IP, S72],
        &[4.0, -0.5, -S36I, S36I, -0.5, 1.0 + 0.5 * PHI, S36],
    ],
    // parent 3 (468) -> 451
    &[&[2.0, IP, 0.0, 0.0, IP, 0.0, 0.0]],
    // parent 4 (484) -> 484 + 50 + 105
    &[
        &[4.0, IP, 0.0, 0.0, IP, 0.0, 0.0],
        &[0.0, -0.5, -S36I, S36I, -0.5, 1.5 * IP + IP2H, S36I],
        &[1.0, -IP, 0.0, 0.0, IP, PHI, 0.0],
    ],
];

/// Area centroid of a polygon (shoelace cross-product form).
fn centroid(p: &Polygon) -> Vec2 {
    let n = p.pts.len();
    let (mut a, mut cx, mut cy) = (0.0, 0.0, 0.0);
    for i in 0..n {
        let q = p.pts[i];
        let r = p.pts[(i + 1) % n];
        let cr = q.x * r.y - r.x * q.y;
        a += cr;
        cx += (q.x + r.x) * cr;
        cy += (q.y + r.y) * cr;
    }
    Vec2::new(cx / (3.0 * a), cy / (3.0 * a))
}

/// One tile in cell (tiling-native) coordinates: a prototile index and
/// the affine map from the canonical polygon to its placement.
#[derive(Clone, Copy)]
struct TileCell {
    proto: u8,
    m: [[f64; 2]; 2],
    t: Vec2,
}

impl TileCell {
    fn transform(&self, q: Vec2) -> Vec2 {
        Vec2::new(
            self.m[0][0] * q.x + self.m[0][1] * q.y + self.t.x,
            self.m[1][0] * q.x + self.m[1][1] * q.y + self.t.y,
        )
    }

    fn aabb(&self, protos: &[Polygon; 5]) -> BBox {
        let mut lo = Vec2::new(f64::MAX, f64::MAX);
        let mut hi = Vec2::new(f64::MIN, f64::MIN);
        for p in protos[self.proto as usize].pts.iter() {
            let q = self.transform(*p);
            lo.x = lo.x.min(q.x);
            lo.y = lo.y.min(q.y);
            hi.x = hi.x.max(q.x);
            hi.y = hi.y.max(q.y);
        }
        BBox::new(lo, hi)
    }

    fn poly(&self, protos: &[Polygon; 5]) -> Polygon {
        let pts: Vec<Vec2> = protos[self.proto as usize]
            .pts
            .iter()
            .map(|p| self.transform(*p))
            .collect();
        Polygon::from_slice(&pts)
    }
}

/// One refinement (deflation) step: replace every tile by its
/// substitution children (scale 1/φ), which exactly tile the parent.
///
/// Culling is sound: a child never leaves its parent, so a final tile
/// intersecting `region` has an ancestor chain whose aabbs all intersect
/// `region` — nothing needed is ever dropped.
fn refine_step(tiles: &[TileCell], protos: &[Polygon; 5], region: &BBox) -> Vec<TileCell> {
    let mut out = Vec::with_capacity(tiles.len() * 3);
    for t in tiles {
        for row in DEF[t.proto as usize] {
            let d = [
                [row[1], row[2]],
                [row[3], row[4]],
            ];
            let dt = Vec2::new(row[5], row[6]);
            let child = TileCell {
                proto: row[0] as u8,
                m: [
                    [t.m[0][0] * d[0][0] + t.m[0][1] * d[1][0], t.m[0][0] * d[0][1] + t.m[0][1] * d[1][1]],
                    [t.m[1][0] * d[0][0] + t.m[1][1] * d[1][0], t.m[1][0] * d[0][1] + t.m[1][1] * d[1][1]],
                ],
                t: Vec2::new(
                    t.m[0][0] * dt.x + t.m[0][1] * dt.y + t.t.x,
                    t.m[1][0] * dt.x + t.m[1][1] * dt.y + t.t.y,
                ),
            };
            if child.aabb(protos).intersects(*region) {
                out.push(child);
            }
        }
    }
    out
}

/// Penrose Pentagon tiling (aperiodic, 5-fold local symmetry).
pub struct PenrosePentagonTiling;

impl Tiling for PenrosePentagonTiling {
    fn id(&self) -> &'static str {
        "penrose-pentagon"
    }

    fn name(&self) -> &'static str {
        "Penrose Pentagon (5-tile φ-substitution)"
    }

    fn description(&self) -> &'static str {
        "Aperiodic tiling by five pentagons with a φ-substitution; local 5-fold symmetry."
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

        let protos = protos();
        let c = centroid(&protos[2]);

        // The seed tile must cover the region: the region's aabb lies in
        // the disk through its farthest corner (the Euclidean norm is
        // convex, so its max over the aabb is at a corner), and the disk
        // of radius SEED_R about the centroid is inscribed in the 451
        // polygon. Hence scale = max |corner| / SEED_R.
        let mut scale: f64 = 0.0;
        for &corner in &region.corners() {
            scale = scale.max(corner.length());
        }
        if scale < 1e-9 {
            return fail("empty view");
        }
        // Pre-generation budget (mirrors lattice.rs): final tiles have
        // scale ≤ 1, so each covers at most A451 cell units², and the
        // tiling must cover the query region — area / A451 is a lower
        // bound on the tile count, checked before any refinement work.
        let s = params.float(SCALE_ID, 1.0);
        if query.region.area() / (A451 * s * s) > MAX_TILES as f64 {
            return fail("too many tiles for this view — zoom in");
        }

        let s0 = scale / SEED_R;
        let mut tiles = vec![TileCell {
            proto: 2,
            m: [[s0, 0.0], [0.0, s0]],
            t: Vec2::new(-s0 * c.x, -s0 * c.y),
        }];
        let mut sc = s0;
        for _ in 0..STEP_CAP {
            if sc <= 1.0 + 1e-12 {
                break;
            }
            tiles = refine_step(&tiles, &protos, &region);
            sc /= PHI;
        }
        if tiles.is_empty() || sc > 1.0 + 1e-12 {
            return fail("view too far from the seed — pan back or zoom in");
        }

        let mut out = Vec::with_capacity(tiles.len());
        for t in &tiles {
            let world = t.poly(&protos).transform(&map);
            if world.aabb().intersects(query.region) {
                out.push(Tile {
                    proto: t.proto,
                    seed: t.proto as u32,
                    poly: world,
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
    use tess_core::geometry::Affine;

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

    /// The children of a parent must exactly tile it (deflation view):
    /// the child areas sum to the parent's, and every sampled interior
    /// point lies in exactly one child.
    fn check_substitution(pi: usize, label: &str) {
        let protos = protos();
        let parent = &protos[pi];
        let pa = parent.area();
        let kids: Vec<Polygon> = DEF[pi]
            .iter()
            .map(|row| {
                let a = Affine::new(
                    [[row[1], row[2]], [row[3], row[4]]],
                    Vec2::new(row[5], row[6]),
                );
                let k = protos[row[0] as usize].transform(&a);
                assert!(
                    (k.area() - protos[row[0] as usize].area() / (PHI * PHI))
                        .abs()
                        <= 1e-9,
                    "{label}: child {proto} not at 1/φ scale",
                    proto = row[0]
                );
                k
            })
            .collect();
        let ka: f64 = kids.iter().map(|k| k.area()).sum();
        assert!(
            (ka - pa).abs() <= 1e-9 * pa,
            "{label}: child areas {ka} != parent {pa}"
        );

        let bb = parent.aabb();
        let mut rng = Rng(0x517C_C1B7_2722_0A95);
        let mut checked = 0;
        for _ in 0..4000 {
            let q = Vec2::new(
                bb.min.x + rng.f64() * bb.size().x,
                bb.min.y + rng.f64() * bb.size().y,
            );
            if !parent.contains(q) {
                continue;
            }
            let n = kids.iter().filter(|k| k.contains(q)).count();
            assert!(n == 1, "{label}: point {q} covered by {n} children");
            checked += 1;
        }
        assert!(checked > 1500, "{label}: too few samples landed");
    }

    #[test]
    fn substitution_tiles_parents() {
        for pi in 0..5 {
            check_substitution(pi, &format!("proto {pi}"));
        }
    }

    /// The DEF table encodes the substitution count matrix: refining a
    /// single proto-2 (451) seed yields the observed tile counts.
    #[test]
    fn deflation_counts() {
        let mut cnt = [0usize; 5];
        cnt[2] = 1;
        let expected = [
            [2, 0, 0, 0, 3],    // level 1: 5 tiles
            [3, 5, 0, 2, 3],    // level 2: 13 tiles
            [8, 11, 2, 3, 8],   // level 3: 32 tiles
            [23, 27, 3, 8, 25], // level 4: 86 tiles
        ];
        for exp in expected {
            let mut next = [0usize; 5];
            for (pi, c) in cnt.iter().enumerate() {
                for row in DEF[pi] {
                    next[row[0] as usize] += c;
                }
            }
            assert_eq!(next, exp, "level counts");
            cnt = next;
        }
    }
}
