//! The isohedral generator engine: a generic interpreter over a pinned
//! [`TypeData`] record (Kaplan/Tactile incidence-symbol data, transcribed
//! exactly by the spike codegen into the `data_ih*.rs` modules).
//!
//! Everything geometric is computed in exact `Q(√3)` arithmetic
//! ([`super::field::Q`]); `f64` appears exactly once per emitted
//! coordinate, at the polygon-assembly boundary. That single-rounding
//! policy is what makes the shared-edge bit-identity contract hold
//! (docs/isohedral-generator-design.md §2): the two sides of a shared
//! edge evaluate the same exact field elements through different exact
//! pipelines, so equal values emit bit-identical floats.
//!
//! Semantics verified against Tactile `tiling.cpp::recompute()`:
//! edges[idx] = match(v_idx, v_{idx+1}) ∘ M_orients[2·fl + ro],
//! reversals[idx] = (fl != ro), aspect coefficient blocks row-major at
//! stride 6(np+1), translation blocks in order t1x t1y t2x t2y.

use tess_core::geometry::{BBox, Polygon, Vec2};

use crate::lattice::MAX_TILES;

use super::field::Q;

/// The pinned per-type data (one `static` per instance, in the
/// generated `data_ih*.rs` modules).
pub(crate) struct TypeData {
    pub np: usize,
    pub na: usize,
    pub nv: usize,
    /// Edge → orbit id (Tactile `edge_shape_ids`).
    pub sid: &'static [usize],
    /// Per edge (fl, ro) orientation flags (Tactile `edge_orientations`).
    pub ori: &'static [(bool, bool)],
    /// Adjacency symbol: per (aspect, edge) the neighbor across that
    /// edge as (aspect', di, dj, edge'). Derived in the spike by exact
    /// geometric edge matching and BFS-verified; valid at every lattice
    /// position (design doc §1.1). Read by the shared-edge and
    /// pairing-D batteries; generation culls geometrically instead.
    #[cfg_attr(not(test), allow(dead_code))]
    pub adj: &'static [(usize, i64, i64, usize)],
    /// The pinned exact parameters of the shipped instance.
    pub params: &'static [Q],
    /// Vertex coefficient blocks: 2·nv·(np+1) (x block then y block per
    /// vertex).
    pub vc: &'static [Q],
    /// Translation coefficient blocks: 4·(np+1) (t1x t1y t2x t2y).
    pub tc: &'static [Q],
    /// Aspect coefficient blocks: 6·na·(np+1), block (r·3+cc) row-major.
    pub ac: &'static [Q],
    /// Per orbit: the canonical curve as a flat (x, y, x, y, …) point
    /// list, including both endpoints (0,0) and (1,0).
    pub curves: &'static [&'static [Q]],
    /// Per orbit: the allowed pairing D-classes as indices into
    /// [`M_ORIENTS`] (0=ID, 1=ROT, 2=FLIP, 3=ROFL) — the subgroup H the
    /// canonical curve is invariant under (design doc §1.2). Consumed
    /// by the pairing-D battery.
    #[cfg_attr(not(test), allow(dead_code))]
    pub orbit_d: &'static [&'static [u8]],
}

/// Tactile's four unit-frame maps, as exact affines
/// (m00, m01, m10, m11, tx, ty), applied x' = m00·x + m01·y + tx.
pub(crate) const M_ORIENTS: [QAff; 4] = [
    QAff {
        a: Q::r(1, 1),
        b: Q::r(0, 1),
        c: Q::r(0, 1),
        d: Q::r(1, 1),
        e: Q::r(0, 1),
        f: Q::r(0, 1),
    }, // ID
    QAff {
        a: Q::r(-1, 1),
        b: Q::r(0, 1),
        c: Q::r(0, 1),
        d: Q::r(-1, 1),
        e: Q::r(1, 1),
        f: Q::r(0, 1),
    }, // ROT
    QAff {
        a: Q::r(-1, 1),
        b: Q::r(0, 1),
        c: Q::r(0, 1),
        d: Q::r(1, 1),
        e: Q::r(1, 1),
        f: Q::r(0, 1),
    }, // FLIP
    QAff {
        a: Q::r(1, 1),
        b: Q::r(0, 1),
        c: Q::r(0, 1),
        d: Q::r(-1, 1),
        e: Q::r(0, 1),
        f: Q::r(0, 1),
    }, // ROFL
];

/// An exact affine map in `Q(√3)`: (x, y) ↦ (a·x + b·y + e, c·x + d·y + f).
#[derive(Clone, Copy, Debug)]
pub(crate) struct QAff {
    pub a: Q,
    pub b: Q,
    pub c: Q,
    pub d: Q,
    pub e: Q,
    pub f: Q,
}

impl QAff {
    pub fn apply(&self, x: Q, y: Q) -> (Q, Q) {
        (
            self.a.mul(x).add(self.b.mul(y)).add(self.e),
            self.c.mul(x).add(self.d.mul(y)).add(self.f),
        )
    }

    /// `outer ∘ inner` (apply `inner` first).
    pub fn comp(outer: &QAff, inner: &QAff) -> QAff {
        QAff {
            a: outer.a.mul(inner.a).add(outer.b.mul(inner.c)),
            b: outer.a.mul(inner.b).add(outer.b.mul(inner.d)),
            c: outer.c.mul(inner.a).add(outer.d.mul(inner.c)),
            d: outer.c.mul(inner.b).add(outer.d.mul(inner.d)),
            e: outer.a.mul(inner.e).add(outer.b.mul(inner.f)).add(outer.e),
            f: outer.c.mul(inner.e).add(outer.d.mul(inner.f)).add(outer.f),
        }
    }

    pub fn translation(x: Q, y: Q) -> QAff {
        QAff {
            a: Q::one(),
            b: Q::zero(),
            c: Q::zero(),
            d: Q::one(),
            e: x,
            f: y,
        }
    }

    /// Exact inverse (design doc §2.1: conjugation formula in Q(√3)).
    /// Test-only: enters the pairing-D census (`D = (T_X·E_k)⁻¹·T_Y·E_l`);
    /// generation never inverts a map.
    #[cfg(test)]
    pub fn inverse(&self) -> Option<QAff> {
        let det = self.a.mul(self.d).sub(self.b.mul(self.c));
        let di = det.inv()?;
        Some(QAff {
            a: self.d.mul(di),
            b: self.b.mul(di).neg(),
            c: self.c.mul(di).neg(),
            d: self.a.mul(di),
            e: self.b.mul(self.f).sub(self.d.mul(self.e)).mul(di),
            f: self.c.mul(self.e).sub(self.a.mul(self.f)).mul(di),
        })
    }

    /// Equality as exact maps (test-only: the pairing-D census).
    #[cfg(test)]
    pub fn is_eq(&self, o: &QAff) -> bool {
        self.a == o.a
            && self.b == o.b
            && self.c == o.c
            && self.d == o.d
            && self.e == o.e
            && self.f == o.f
    }
}

/// Evaluate one coefficient block (length np+1) at the parameters:
/// `Σ block[i]·p[i] + block[np]`.
fn eval_form(block: &[Q], p: &[Q]) -> Q {
    let np = p.len();
    let mut acc = block[np];
    for i in 0..np {
        acc = acc.add(block[i].mul(p[i]));
    }
    acc
}

/// The assembled exact geometry of one type at one exact parameter point.
pub(crate) struct ExactGeom {
    /// Tile-local vertices, in boundary order. Read by the
    /// transcription spot-checks; assembly consumes it locally.
    #[cfg_attr(not(test), allow(dead_code))]
    pub verts: Vec<(Q, Q)>,
    pub t1: (Q, Q),
    pub t2: (Q, Q),
    /// Aspect transforms (the per-aspect coset representatives).
    pub aspects: Vec<QAff>,
    /// Tile-local edge maps E_k = match(v_k, v_{k+1}) ∘ M_orients.
    pub edges: Vec<QAff>,
}

/// Build the exact geometry at the given exact parameters.
pub(crate) fn build_geom(d: &TypeData, p: &[Q]) -> ExactGeom {
    let np1 = d.np + 1;
    let mut verts = Vec::with_capacity(d.nv);
    for k in 0..d.nv {
        let cx = &d.vc[2 * k * np1..(2 * k + 1) * np1];
        let cy = &d.vc[(2 * k + 1) * np1..(2 * k + 2) * np1];
        verts.push((eval_form(cx, p), eval_form(cy, p)));
    }
    let t1 = (
        eval_form(&d.tc[0..np1], p),
        eval_form(&d.tc[np1..2 * np1], p),
    );
    let t2 = (
        eval_form(&d.tc[2 * np1..3 * np1], p),
        eval_form(&d.tc[3 * np1..4 * np1], p),
    );
    let mut aspects = Vec::with_capacity(d.na);
    for a in 0..d.na {
        let blk = &d.ac[a * 6 * np1..(a + 1) * 6 * np1];
        // The block stores Tactile's 2×3 aspect matrix row-major
        // ([xx xy tx / yx yy ty], world = (xx·x + xy·y + tx, …)), so the
        // affine reads off in natural order: (xx, xy, yx, yy, tx, ty) =
        // (blk00, blk01, blk10, blk11, blk02, blk12) — verified against
        // spike2.build_exact and tiling.cpp (design doc §1.1). The two
        // off-diagonal entries are NOT swapped: pgg's symmetric glide
        // matrices hide a transpose, but p3/p4g rotations do not.
        let m =
            |r: usize, cc: usize| eval_form(&blk[(r * 3 + cc) * np1..(r * 3 + cc + 1) * np1], p);
        aspects.push(QAff {
            a: m(0, 0),
            b: m(0, 1),
            c: m(1, 0),
            d: m(1, 1),
            e: m(0, 2),
            f: m(1, 2),
        });
    }
    let mut edges = Vec::with_capacity(d.nv);
    for k in 0..d.nv {
        let (vx, vy) = verts[k];
        let (wx, wy) = verts[(k + 1) % d.nv];
        let (dx, dy) = (wx.sub(vx), wy.sub(vy));
        // match(p, q): (x, y) ↦ p + x·(q−p) + y·R90(q−p) with
        // R90(dx, dy) = (−dy, dx).
        let mt = QAff {
            a: dx,
            b: dy.neg(),
            c: dy,
            d: dx,
            e: vx,
            f: vy,
        };
        let (fl, ro) = d.ori[k];
        let which = if fl { 2 } else { 0 } + if ro { 1 } else { 0 };
        edges.push(QAff::comp(&mt, &M_ORIENTS[which]));
    }
    ExactGeom {
        verts,
        t1,
        t2,
        aspects,
        edges,
    }
}

/// The world tile transform `T_{a,i,j} = translate(i·t1 + j·t2) ∘ aspect_a`
/// (aspect applied first — Tactile's composition order).
pub(crate) fn tile_transform(g: &ExactGeom, a: usize, i: i64, j: i64) -> QAff {
    let ii = Q::r(i as i128, 1);
    let jj = Q::r(j as i128, 1);
    let off = QAff::translation(
        g.t1.0.mul(ii).add(g.t2.0.mul(jj)),
        g.t1.1.mul(ii).add(g.t2.1.mul(jj)),
    );
    QAff::comp(&off, &g.aspects[a])
}

/// The world edge map `W = T ∘ E_k` for the tile transform `T`, edge k.
pub(crate) fn edge_world(t: &QAff, k: usize, g: &ExactGeom) -> QAff {
    QAff::comp(t, &g.edges[k])
}

/// The canonical point list of edge k's orbit, oriented along the tile
/// boundary (v_k → v_{k+1}): reversed when `reversals[k]`.
fn oriented_canon(d: &TypeData, k: usize) -> Vec<(Q, Q)> {
    let flat = d.curves[d.sid[k]];
    let mut pts: Vec<(Q, Q)> = flat.chunks_exact(2).map(|c| (c[0], c[1])).collect();
    let (fl, ro) = d.ori[k];
    if fl != ro {
        pts.reverse();
    }
    pts
}

/// Assemble a tile's polygon from the transform `t` (= `T_{a,i,j}`): per
/// edge in boundary order, the orbit's canonical polyline (oriented,
/// joint deduplicated by skipping each edge's first point), mapped
/// through the exact world edge map, rounded to `f64` once per
/// coordinate (design doc §2.3).
pub(crate) fn tile_polygon(d: &TypeData, g: &ExactGeom, t: &QAff) -> Polygon {
    let cap = d.curves.iter().map(|c| c.len() / 2).max().unwrap_or(1) * d.nv;
    let mut pts: Vec<Vec2> = Vec::with_capacity(cap);
    for k in 0..d.nv {
        let w = edge_world(t, k, g);
        let canon = oriented_canon(d, k);
        for (idx, (x, y)) in canon.iter().enumerate() {
            if idx == 0 {
                continue; // the previous edge (mod nv) ends exactly here
            }
            let (wx, wy) = w.apply(*x, *y);
            pts.push(Vec2::new(wx.to_f64(), wy.to_f64()));
        }
    }
    Polygon::new(pts)
}

/// Test-only: the world point sequence of one edge of the tile `t`
/// (the exact sequences the shared-edge identity battery compares).
#[cfg(test)]
pub(crate) fn edge_points(d: &TypeData, g: &ExactGeom, t: &QAff, k: usize) -> Vec<(f64, f64)> {
    let w = edge_world(t, k, g);
    oriented_canon(d, k)
        .iter()
        .map(|(x, y)| {
            let (wx, wy) = w.apply(*x, *y);
            (wx.to_f64(), wy.to_f64())
        })
        .collect()
}

/// Test-only: the exact pairing isometry
/// `D = (T_X ∘ E_k)⁻¹ ∘ (T_Y ∘ E_l)` for the center tile's edge k and
/// its recorded neighbor (design doc §1.2). Must land in M_ORIENTS.
#[cfg(test)]
pub(crate) fn pairing_d(d: &TypeData, g: &ExactGeom, a: usize, k: usize) -> QAff {
    let (a2, di, dj, l) = d.adj[a * d.nv + k];
    let tx = tile_transform(g, a, 0, 0);
    let ty = tile_transform(g, a2, di, dj);
    let ix = QAff::comp(&tx, &g.edges[k]);
    let iy = QAff::comp(&ty, &g.edges[l]);
    QAff::comp(&ix.inverse().expect("pairing left side invertible"), &iy)
}

/// Float-parameter resolution (design doc §4): a float equal to the
/// declared default (bit-exact) resolves to the pinned exact rational;
/// any other value is quantized to the 2^−20 grid — far below the UI
/// parameter step — and handled exactly at that value. Quantization
/// bounds composition-chain denominators far inside the i128 range.
pub(crate) fn resolve_params(
    d: &TypeData,
    defaults: &[f64],
    ps: &tess_core::params::ParamSet,
    ids: &[tess_core::params::ParamId],
) -> Vec<Q> {
    const GRID: i128 = 1_048_576; // 2^20
    (0..d.np)
        .map(|i| {
            let f = ps.float(ids[i], defaults[i]);
            if f.to_bits() == defaults[i].to_bits() {
                d.params[i]
            } else {
                Q::r((f * (GRID as f64)).round() as i128, GRID)
            }
        })
        .collect()
}

/// Generation core shared by all instances: build the exact geometry at
/// the resolved parameters, emit every tile of the (aspect, lattice)
/// family intersecting the region (view map applied at emission), and
/// report the world lattice vectors. Returns `(tiles, [t1·map, t2·map])`
/// or an error message when the view exceeds the tile budget or the
/// lattice degenerates.
pub(crate) fn generate_tiles(
    d: &TypeData,
    defaults: &[f64],
    ids: &[tess_core::params::ParamId],
    params: &tess_core::params::ParamSet,
    region: &BBox,
    view_map: tess_core::geometry::Affine,
) -> Result<(Vec<(Polygon, u32)>, [Vec2; 2]), &'static str> {
    let p = resolve_params(d, defaults, params, ids);
    let g = build_geom(d, &p);

    // World lattice vectors (floats; view parameters apply here only).
    let t1w = view_map.transform(Vec2::new(g.t1.0.to_f64(), g.t1.1.to_f64()));
    let t2w = view_map.transform(Vec2::new(g.t2.0.to_f64(), g.t2.1.to_f64()));
    let det = t1w.x * t2w.y - t1w.y * t2w.x;
    if det.abs() < 1e-12 {
        return Err("degenerate lattice — adjust the shape parameters");
    }

    // The region in cell-world coordinates (the view map has no
    // translation, but invert its corners generally).
    let inv_map = view_map.inverse();
    let corners = [
        inv_map.transform(region.min),
        inv_map.transform(Vec2::new(region.max.x, region.min.y)),
        inv_map.transform(Vec2::new(region.min.x, region.max.y)),
        inv_map.transform(region.max),
    ];

    // Base polygons of every aspect at the lattice origin (exact, then
    // rounded once) — their combined extent pads the (i, j) search box.
    let mut pad_x = 0.0f64;
    let mut pad_y = 0.0f64;
    for a in 0..d.na {
        let bb = tile_polygon(d, &g, &g.aspects[a]).aabb();
        pad_x = pad_x.max((bb.max.x - bb.min.x).abs());
        pad_y = pad_y.max((bb.max.y - bb.min.y).abs());
    }

    // (i, j) bounds: invert the lattice basis on the padded region bbox;
    // the padded bbox corners subsume the region corners.
    let rx0 = corners.iter().map(|c| c.x).fold(f64::INFINITY, f64::min) - pad_x - 2.0;
    let ry0 = corners.iter().map(|c| c.y).fold(f64::INFINITY, f64::min) - pad_y - 2.0;
    let rx1 = corners
        .iter()
        .map(|c| c.x)
        .fold(f64::NEG_INFINITY, f64::max)
        + pad_x
        + 2.0;
    let ry1 = corners
        .iter()
        .map(|c| c.y)
        .fold(f64::NEG_INFINITY, f64::max)
        + pad_y
        + 2.0;
    let inv_det = 1.0 / det;
    // [x; y] = i·t1w + j·t2w  ⟺  [i; j] = adj·[x; y].
    let lattice_i = |x: f64, y: f64| (x * t2w.y - y * t2w.x) * inv_det;
    let lattice_j = |x: f64, y: f64| (y * t1w.x - x * t1w.y) * inv_det;
    let pts = [(rx0, ry0), (rx1, ry0), (rx0, ry1), (rx1, ry1)];
    let (mut i0, mut i1, mut j0, mut j1) = (0i64, 0i64, 0i64, 0i64);
    for (x, y) in pts {
        let fi = lattice_i(x, y);
        let fj = lattice_j(x, y);
        if !fi.is_finite() || !fj.is_finite() {
            return Err("region too far from the origin — pan back");
        }
        i0 = i0.min(fi.floor() as i64 - 1);
        i1 = i1.max(fi.ceil() as i64 + 1);
        j0 = j0.min(fj.floor() as i64 - 1);
        j1 = j1.max(fj.ceil() as i64 + 1);
    }
    // f64 differences cannot overflow; guard the box before i64 math.
    if (i1 as f64 - i0 as f64) * (j1 as f64 - j0 as f64) * d.na as f64 > MAX_TILES as f64
        || i1 - i0 > 50_000
        || j1 - j0 > 50_000
    {
        return Err("too many tiles for this view — zoom in");
    }

    let mut out = Vec::new();
    for j in j0..=j1 {
        for i in i0..=i1 {
            for a in 0..d.na {
                let t = tile_transform(&g, a, i, j);
                let poly = tile_polygon(d, &g, &t).transform(&view_map);
                if poly.aabb().intersects(*region) {
                    out.push((poly, a as u32));
                }
            }
        }
    }
    Ok((out, [t1w, t2w]))
}

#[cfg(test)]
pub(crate) mod test_support {
    //! Battery helpers shared by the per-instance test modules: the
    //! irrational-offset exactly-once grid, the shared-edge f64
    //! bit-identity check with its edge census, the aspect-symmetry
    //! check, and the exact pairing-D census.

    use std::collections::{BTreeMap, BTreeSet};

    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamId, ParamSet};
    use tess_core::tiling::{Query, Tiling};

    use super::{build_geom, resolve_params, tile_transform, TypeData, M_ORIENTS};

    /// The exact geometry at the instance's pinned default parameters.
    pub(crate) fn geom_at_defaults(
        d: &TypeData,
        defaults: &[f64],
        ids: &[ParamId],
    ) -> super::ExactGeom {
        let p = resolve_params(d, defaults, &ParamSet::EMPTY, ids);
        build_geom(d, &p)
    }

    /// Irrational-offset exactly-once grid: sample the region on a fine
    /// grid shifted by (√2/3, √5/7) and require every sample point to be
    /// covered by EXACTLY one tile (catches gaps AND overlaps).
    pub(crate) fn check_exactly_once(
        t: &dyn Tiling,
        name: &str,
        params: &ParamSet,
        region: BBox,
        nx: usize,
        ny: usize,
    ) {
        let q = t.generate(params, &Query { region });
        assert!(!q.tiles.is_empty(), "{name}: no tiles generated");
        let sx = region.size().x / (nx as f64 + 1.0);
        let sy = region.size().y / (ny as f64 + 1.0);
        let ox = (std::f64::consts::SQRT_2 / 3.0).fract();
        let oy = (5.0f64.sqrt() / 7.0).fract();
        for gy in 0..ny {
            for gx in 0..nx {
                let px = region.min.x + sx * (gx as f64 + ox);
                let py = region.min.y + sy * (gy as f64 + oy);
                let n = q
                    .tiles
                    .iter()
                    .filter(|tl| tl.poly.contains(Vec2::new(px, py)))
                    .count();
                assert_eq!(
                    n, 1,
                    "{name}: point ({px}, {py}) covered {n} times (want exactly 1)"
                );
            }
        }
    }

    /// Shared-edge bit-identity: for every interior edge pairing of a
    /// ±W lattice window, the two pipelines' emitted f64 point sequences
    /// must be equal or bit-exact reversals. Returns the per-(orbit,
    /// direction) census for reporting. Fails unless every (aspect,
    /// edge) pairing is exercised at least once.
    pub(crate) fn check_shared_edge_identity(
        d: &TypeData,
        defaults: &[f64],
        ids: &[ParamId],
        name: &str,
        w: i64,
    ) -> BTreeMap<(usize, &'static str), usize> {
        let g = geom_at_defaults(d, defaults, ids);
        let mut census: BTreeMap<(usize, &'static str), usize> = BTreeMap::new();
        let mut covered: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut fails = 0usize;
        for j in -w..=w {
            for i in -w..=w {
                for a in 0..d.na {
                    let t = tile_transform(&g, a, i, j);
                    for k in 0..d.nv {
                        let (a2, di, dj, l) = d.adj[a * d.nv + k];
                        let (i2, j2) = (i + di, j + dj);
                        if i2 < -w || i2 > w || j2 < -w || j2 > w {
                            continue; // rim: neighbor outside the window
                        }
                        covered.insert((a, k));
                        let t2 = tile_transform(&g, a2, i2, j2);
                        let xs = super::edge_points(d, &g, &t, k);
                        let ys = super::edge_points(d, &g, &t2, l);
                        let same = xs == ys;
                        let rev = !same && {
                            let mut r = ys.clone();
                            r.reverse();
                            xs == r
                        };
                        if !(same || rev) {
                            fails += 1;
                            if fails <= 3 {
                                panic!(
                                    "{name}: shared edge ({a},{i},{j},{k}) vs ({a2},{i2},{j2},{l}) \
                                     differs: X={xs:?} Y={ys:?}"
                                );
                            }
                        }
                        *census
                            .entry((d.sid[k], if same { "fwd" } else { "rev" }))
                            .or_default() += 1;
                    }
                }
            }
        }
        assert_eq!(fails, 0, "{name}: {fails} shared-edge identity failures");
        let all: BTreeSet<(usize, usize)> = (0..d.na)
            .flat_map(|a| (0..d.nv).map(move |k| (a, k)))
            .collect();
        assert_eq!(
            covered, all,
            "{name}: window too small — some (aspect, edge) pairings never checked"
        );
        census
    }

    /// Aspect-symmetry check: each aspect transform X_a maps tiles to
    /// tiles. For a grid of sample tiles around the origin, the
    /// X_a-image polygon must equal some window tile's polygon
    /// pointwise (cyclic rotation and/or reversal allowed — aspects may
    /// be orientation-reversing), tolerance 1e-6.
    pub(crate) fn check_aspect_symmetry(
        d: &TypeData,
        defaults: &[f64],
        ids: &[ParamId],
        name: &str,
    ) {
        let g = geom_at_defaults(d, defaults, ids);
        let tol = 1e-6;
        // Window tiles (±4) as f64 point lists.
        let mut tiles: Vec<Vec<Vec2>> = Vec::new();
        for j in -4..=4 {
            for i in -4..=4 {
                for a in 0..d.na {
                    let t = tile_transform(&g, a, i, j);
                    tiles.push(super::tile_polygon(d, &g, &t).pts.clone());
                }
            }
        }
        let matches = |img: &[Vec2], other: &[Vec2]| {
            if other.len() != img.len() {
                return false;
            }
            let n = img.len();
            (0..n).any(|off| {
                let fwd = (0..n).all(|s| {
                    let o = other[(s + off) % n];
                    (o.x - img[s].x).abs() < tol && (o.y - img[s].y).abs() < tol
                });
                let rev = (0..n).all(|s| {
                    let o = other[(off + n - s) % n];
                    (o.x - img[s].x).abs() < tol && (o.y - img[s].y).abs() < tol
                });
                fwd || rev
            })
        };
        for a in 0..d.na {
            let asp = &g.aspects[a];
            let xform = |p: tess_core::geometry::Vec2| {
                tess_core::geometry::Vec2::new(
                    asp.a.to_f64() * p.x + asp.b.to_f64() * p.y + asp.e.to_f64(),
                    asp.c.to_f64() * p.x + asp.d.to_f64() * p.y + asp.f.to_f64(),
                )
            };
            for j in -1..=1 {
                for i in -1..=1 {
                    for a2 in 0..d.na {
                        let t = tile_transform(&g, a2, i, j);
                        let pts = super::tile_polygon(d, &g, &t).pts;
                        let img: Vec<tess_core::geometry::Vec2> =
                            pts.iter().map(|p| xform(*p)).collect();
                        assert!(
                            tiles.iter().any(|other| matches(&img, other)),
                            "{name}: aspect {a} does not map tile ({a2},{i},{j}) onto the tiling"
                        );
                    }
                }
            }
        }
    }

    /// Exact pairing-D census: for the center tile, every edge's pairing
    /// isometry D must land exactly in its orbit's allowed M_ORIENTS set
    /// (design doc §1.2). Returns the per-orbit D-class count histogram
    /// (e.g. IH29 orbit 0 = {"ROT": 2, "ROFL": 2}).
    pub(crate) fn orbit_d_census(
        d: &TypeData,
        defaults: &[f64],
        ids: &[ParamId],
        name: &str,
    ) -> BTreeMap<usize, BTreeMap<&'static str, usize>> {
        let g = geom_at_defaults(d, defaults, ids);
        let names = ["ID", "ROT", "FLIP", "ROFL"];
        let mut hist: BTreeMap<usize, BTreeMap<&'static str, usize>> = BTreeMap::new();
        for a in 0..d.na {
            for k in 0..d.nv {
                let dd = super::pairing_d(d, &g, a, k);
                let orbit = d.sid[k];
                let hits: Vec<usize> = (0..4).filter(|ix| dd.is_eq(&M_ORIENTS[*ix])).collect();
                assert_eq!(
                    hits.len(),
                    1,
                    "{name}: pairing ({a},{k}) D is not a unit-frame map: {dd:?}"
                );
                let allowed = d.orbit_d[orbit];
                assert!(
                    allowed.contains(&(hits[0] as u8)),
                    "{name}: pairing ({a},{k}) D={} not in orbit {orbit} allowed set {:?}",
                    names[hits[0]],
                    allowed
                        .iter()
                        .map(|h| names[*h as usize])
                        .collect::<Vec<_>>()
                );
                *hist
                    .entry(orbit)
                    .or_default()
                    .entry(names[hits[0]])
                    .or_default() += 1;
            }
        }
        hist
    }
}
