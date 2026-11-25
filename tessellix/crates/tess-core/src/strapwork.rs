//! Hankin polygons-in-contact (PIC) strapwork **layer mode** (M57).
//!
//! The general engine behind the registered PIC partition tilings
//! (`pic-4-8-8`, `pic-3-6-3-6`, `pic-3-12-12` in `tess-tilings`): given
//! the tiles of an *arbitrary* host tiling and a contact angle δ, it
//! computes the wall network the PIC construction lays over that host —
//! an overlay for rendering, not a registered partition. All arithmetic
//! is f64 (hosts are arbitrary generators); exactness is asserted only
//! where the host is exact, and the three canonical substrates tie this
//! engine to the tiling-mode modules below.
//!
//! ## Construction
//!
//! For every edge midpoint M of every tile, two wall rays depart into
//! that tile at contact angle δ to the edge, symmetric about the inward
//! normal (the same convention as the PIC modules: δ measured from the
//! edge, so δ = 90° runs along the normal). Each ray travels to the tile
//! boundary; the adjacent tile across the edge fires the point-reflected
//! pair at the shared midpoint, so every wall is **continuous across the
//! edge, straight through at M**. Inside its tile a ray is split at
//! every crossing with another ray of the same tile, and each piece
//! between consecutive events (start, crossings, boundary exit) is
//! emitted as one [`Wall`].
//!
//! Crossing-splitting, not first-crossing termination, is what the
//! ground truth demands: the registered partitions cut strands at every
//! crossing — the rosette core of `pic-4-8-8` (octagon edges) and of
//! `pic-3-6-3-6` / `pic-3-12-12` (hexagon / dodecagon edges) is bounded
//! by the chord segments *between two crossings*, which a ray that stops
//! at its first crossing would never emit. The per-tile network is the
//! full strand from midpoint to boundary exit, segmented at crossings;
//! rays therefore terminate at the boundary (rule (b)), and a crossing
//! only ever subdivides (rule (a) refined — PIC modules are ground
//! truth per PROPOSED #14).
//!
//! ## Numerical guards
//!
//! * Tiles with < 3 vertices, or with twice-signed-area below
//!   `1e-12 · scale²` (collinear/degenerate), are skipped; zero-length
//!   edges fire no rays.
//! * Rays whose unit directions satisfy `|d₁ × d₂| < 1e-9` are treated
//!   as parallel and produce no crossing event. This also covers the
//!   collinear case that occurs everywhere: the two ends of one strand
//!   (`M → M'` and `M' → M`) overlap on the same line, and the identical
//!   pieces they emit are removed by the deduplication grid below rather
//!   than turned into spurious split points.
//! * A crossing counts only strictly inside both spans (margin
//!   `1e-9 · tile scale`), events closer than `1e-8 · tile scale` merge
//!   (multi-strand crossings), and pieces shorter than the start margin
//!   are dropped. A boundary hit at `s ≤ 1e-9 · scale` is the ray's own
//!   starting midpoint, never an exit.
//!
//! ## Determinism
//!
//! [`walls`] is a pure function of `(tiles, δ)` — no hidden state, no
//! RNG — and its output is ordered and deduplicated through a
//! [`BTreeMap`] keyed on the segment endpoints rounded to a 1e-6 grid
//! (endpoint-canonical, order- and direction-independent), so the
//! emitted vector is identical across calls on identical input.

use std::collections::BTreeMap;

use crate::geometry::{BBox, Vec2};
use crate::tiling::Tile;

/// One strapwork wall piece: the segment from `a` to `b`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wall {
    pub a: Vec2,
    pub b: Vec2,
}

impl Wall {
    pub fn length(&self) -> f64 {
        (self.b - self.a).length()
    }
}

/// Edges shorter than this (world units) are degenerate; no midpoint ray.
const EPS_EDGE: f64 = 1e-12;
/// Near-parallel guard on unit ray directions (`|d₁ × d₂| = |sin θ|`).
const EPS_PAR: f64 = 1e-9;
/// Dedup grid: endpoints round to 1e-6 world units (same key convention
/// as the tess-tilings cross-checks).
const GRID: f64 = 1e6;

type SegKey = ((i64, i64), (i64, i64));

fn key_pt(p: Vec2) -> (i64, i64) {
    ((p.x * GRID).round() as i64, (p.y * GRID).round() as i64)
}

/// Endpoint-canonical key: the two grid points in fixed order.
fn key_seg(a: Vec2, b: Vec2) -> SegKey {
    let (ka, kb) = (key_pt(a), key_pt(b));
    if ka <= kb {
        (ka, kb)
    } else {
        (kb, ka)
    }
}

/// A ray inside one tile: start point, unit direction, boundary-exit
/// distance along the direction.
struct Ray {
    p: Vec2,
    d: Vec2,
    exit: f64,
}

/// The PIC strapwork wall network over `tiles` at contact angle
/// `delta_deg` **in degrees**.
///
/// Each wall piece lies fully inside the host tile that produced it;
/// pieces are deduplicated on the 1e-6 endpoint grid and returned in a
/// deterministic (sorted) order. See the module docs for the
/// construction and the numerical guards.
pub fn walls(tiles: &[Tile], delta_deg: f64) -> Vec<Wall> {
    let (sd, cd) = delta_deg.to_radians().sin_cos();
    let mut out: BTreeMap<SegKey, Wall> = BTreeMap::new();

    for tile in tiles {
        let pts = &tile.poly.pts;
        let n = pts.len();
        if n < 3 {
            continue; // degenerate tile
        }
        // Twice the signed area + a per-tile scale for the tolerances.
        let mut a2 = 0.0f64;
        let mut scale = 0.0f64;
        for i in 0..n {
            let (p, q) = (pts[i], pts[(i + 1) % n]);
            a2 += p.cross(q);
            scale = scale.max((q - p).length());
        }
        if a2.abs() < 1e-12 * scale.max(1.0) * scale.max(1.0) {
            continue; // collinear / zero-area tile
        }
        let sgn = if a2 > 0.0 { 1.0 } else { -1.0 };
        let eps = 1e-9 * scale.max(1.0);
        let eps_merge = 1e-8 * scale.max(1.0);

        // Fire the two tilted rays at every non-degenerate edge midpoint.
        let mut rays: Vec<Ray> = Vec::new();
        for i in 0..n {
            let (va, vb) = (pts[i], pts[(i + 1) % n]);
            let e = vb - va;
            let len = e.length();
            if len < EPS_EDGE {
                continue;
            }
            let t = e / len;
            // Inward normal: left of the edge for CCW loops, right for CW.
            let nin = Vec2::new(-t.y * sgn, t.x * sgn);
            let m = (va + vb) * 0.5;
            for lean in [-1.0, 1.0] {
                // Unit: t ⊥ nin, both unit, lean² + … = cd² + sd² = 1.
                let d = Vec2::new(lean * cd * t.x + sd * nin.x, lean * cd * t.y + sd * nin.y);
                // Boundary exit: first hit (s > eps) with any tile edge.
                let mut exit = f64::INFINITY;
                for j in 0..n {
                    let (qa, qb) = (pts[j], pts[(j + 1) % n]);
                    let ee = qb - qa;
                    let den = d.cross(ee);
                    if den.abs() < EPS_PAR * ee.length() {
                        continue; // ray parallel to this edge
                    }
                    let q = qa - m;
                    let s = q.cross(ee) / den;
                    let u = q.cross(d) / den;
                    if s > eps && u > -1e-9 && u < 1.0 + 1e-9 && s < exit {
                        exit = s;
                    }
                }
                if exit.is_finite() && exit > eps_merge {
                    rays.push(Ray { p: m, d, exit });
                }
            }
        }

        // Split each ray at its crossings with the other rays of this
        // tile and emit the pieces.
        for (ri, r) in rays.iter().enumerate() {
            let mut events: Vec<f64> = Vec::new();
            for (rj, o) in rays.iter().enumerate() {
                if ri == rj {
                    continue;
                }
                let den = r.d.cross(o.d);
                if den.abs() < EPS_PAR {
                    continue; // parallel, incl. the collinear strand ends
                }
                let q = o.p - r.p;
                let s1 = q.cross(o.d) / den;
                let s2 = q.cross(r.d) / den;
                if s1 > eps && s1 < r.exit - eps && s2 > eps && s2 < o.exit - eps {
                    events.push(s1);
                }
            }
            events.sort_by(f64::total_cmp);
            let mut prev = 0.0;
            let mut cuts: Vec<f64> = Vec::new();
            for &s in &events {
                if s - prev > eps_merge && r.exit - s > eps_merge {
                    cuts.push(s);
                    prev = s;
                }
            }
            cuts.push(r.exit);
            prev = 0.0;
            for s in cuts {
                let a = r.p + r.d * prev;
                let b = r.p + r.d * s;
                prev = s;
                if (b - a).length() <= eps {
                    continue;
                }
                out.insert(key_seg(a, b), Wall { a, b });
            }
        }
    }

    out.into_values().collect()
}

// ──────────────────────────────────────────────────────────────────────────
// Kaplan §3.7 decoration styles: fill / outline / interlace over the wall
// network.
//
// Phase-A audit (see the decoration-suite brief) established the regimes:
// classic contact angles give 2/4-valent wall maps (even degrees, straight
// strand pairing at every joint) — the precondition domain for fill and
// interlace; generic angles add degree-1 exits, ray end-caps (deg-2
// corners) and unpaired crossings, where those styles are ill-formed and
// [`plan_decoration`] says so honestly.
//
// Conventions pinned by the Python spike (spike_mitre.py, all pins ported
// below at 1e-9):
//
// * Band = equidistant offset of a strand polyline at half-width d
//   (Kaplan §3.7.1: band thickness w, parallels at w/2, so d = w/2).
// * Mitered join at a corner V with interior angle θ: the corner point is
//   on the bisector at distance l from V. Euclidean derivation: the
//   parallels sit at distance d and meet the bisector under angle θ/2,
//   so |VC|·sin(θ/2) = d, i.e. **l = d / sin(θ/2)** — Kaplan §3.7.2's
//   Θ(l) = Θ(w)/sin(θ/2) evaluated at the offset (his Θ(x) = 2πx, his w
//   the parallels' separation in the join diagram). Both mitre points
//   (left/right of travel) lie on the bisector line at distance l, in
//   opposite directions.
// * θ→0 clamp: l grows without bound; when l > [`MITRE_CLAMP`]·d the join
//   falls back to a BEVEL through the four offset feet V ± d·n₁, V ± d·n₂.
// * Interlace (§3.7.3): over/under by strand-parity alternation — each
//   strand alternates at its successive crossings, and the assignment is
//   solved as a GF(2) system (union-find with parity). The break is drawn
//   as a background-coloured inset of the over-strand footprint (§3.7.4).
// ──────────────────────────────────────────────────────────────────────────

/// Side-line stroke width as a fraction of the band width.
pub const LINE_FRAC: f64 = 0.16;
/// Max mitre length as a multiple of the half-width d before the join
/// clamps to a bevel (§3.7.2 θ→0 spike policy).
pub const MITRE_CLAMP: f64 = 10.0;
/// Break-gap inset at interlace crossings, as a fraction of d: the
/// background cut is inset from the over band's edges by `BREAK_GAP·d`,
/// so the visible gap between the over band edge and the under strand's
/// break edge is exactly `BREAK_GAP·d` on both sides.
pub const BREAK_GAP: f64 = 0.3;

/// A Kaplan §3.7 rendering style over the wall network.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decoration {
    /// §3.7.1: two-colour the faces of the wall map (Eulerian precondition).
    Fill,
    /// §3.7.2: thicken each wall to a band (offsets + mitered joins), stroke
    /// the two side lines only.
    Outline,
    /// §3.7.3: filled bands with over/under breaks at every crossing.
    Interlace,
    /// §3.7.4 composition: outline the interlaced rendering (side lines +
    /// breaks, no band fill).
    OutlineInterlace,
}

impl Decoration {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "fill" => Some(Self::Fill),
            "outline" => Some(Self::Outline),
            "interlace" => Some(Self::Interlace),
            "outline-interlace" => Some(Self::OutlineInterlace),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Fill => "fill",
            Self::Outline => "outline",
            Self::Interlace => "interlace",
            Self::OutlineInterlace => "outline-interlace",
        }
    }
}

/// A style whose preconditions the wall map does not satisfy. The message
/// is user-facing: counts plus a witness vertex, never a panic upstream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecorationError {
    style: &'static str,
    reason: String,
}

impl std::fmt::Display for DecorationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "decoration '{}' precondition failed: {}",
            self.style, self.reason
        )
    }
}

fn deco_err(style: Decoration, reason: String) -> DecorationError {
    DecorationError {
        style: style.name(),
        reason,
    }
}

/// Everything a renderer needs to paint one decoration style: geometry in
/// world coordinates, in painting order per layer.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DecorationPlan {
    /// Requested band width w (world units); d = w/2 is the offset.
    pub band_w: f64,
    /// Side-line stroke width to use (world units).
    pub line_w: f64,
    /// Fill style: interior faces of the wall map with their colour class.
    pub faces: Vec<(Vec<Vec2>, u8)>,
    /// Strand offset boundary polylines (left, then right of travel),
    /// plus whether the strand is a closed cycle.
    pub side_lines: Vec<(Vec<Vec2>, bool)>,
    /// Band fill outlines: (outer loop, inner hole for closed strands).
    pub band_fills: Vec<(Vec<Vec2>, Option<Vec<Vec2>>)>,
    /// Interlace: background-coloured break quads (one per crossing).
    pub break_quads: Vec<Vec<Vec2>>,
    /// Interlace: over-strand repaint patches over each break.
    pub break_patch: Vec<Vec<Vec2>>,
    /// Interlace: over-strand side-line segments redrawn over the patch,
    /// ((a1, b1), (a2, b2)).
    pub over_lines: Vec<[(Vec2, Vec2); 2]>,
}

// --------------------------------------------------------------- wall map --

/// Vertex classes of the wall map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VKind {
    /// Degree-1 wall endpoint (a ray's boundary exit, or a clipped rim).
    Exit,
    /// Degree-2 bend: a ray end-cap on a host edge (generic δ), or a bend
    /// where two wall pieces meet at a non-straight angle.
    Corner,
    /// Degree 2, straight through (midpoint joint of two opposite pieces).
    Straight2,
    /// Degree ≥ 4 whose darts pair into antiparallel strand continuations.
    Joint,
    /// Anything else: odd degree ≥ 3, or a degree ≥ 4 vertex whose darts
    /// do not pair straight (or pair into fewer than two strands).
    Bad,
}

/// The planar map induced by wall segments: merged vertices (1e-6 grid,
/// centroided like the audit), neighbour sets, and the straight-through
/// dart pairing that defines strands.
struct WallMap {
    pts: Vec<Vec2>,
    adj: Vec<Vec<usize>>,
    pairs: Vec<Vec<Option<usize>>>, // aligned with adj[v]: continuation of dart v→adj[v][i]
    kind: Vec<VKind>,
}

fn unit(v: Vec2) -> Vec2 {
    v / v.length()
}

impl WallMap {
    fn deg(&self, v: usize) -> usize {
        self.adj[v].len()
    }

    /// The continuation of the dart that arrived at `v` from `prev`.
    fn pair_at(&self, v: usize, prev: usize) -> Option<usize> {
        self.adj[v]
            .iter()
            .position(|&w| w == prev)
            .and_then(|i| self.pairs[v][i])
    }

    /// Vertices inside `region`: the tiling contract guarantees the
    /// emitted tiles cover it, so every unshared host edge (and with it
    /// every windowing artefact — a degree-1 exit or ray end-cap on a
    /// boundary edge) lies outside, never strictly inside. Precondition
    /// gates therefore run on the region-interior vertices only (the
    /// audit's core convention, made region-based instead of
    /// bbox-margin-based, which fails on sheared patches).
    fn core(&self, region: &BBox) -> Vec<bool> {
        self.pts
            .iter()
            .map(|p| region.contains_point(*p))
            .collect()
    }

    fn witness(m: &WallMap, core: &[bool], pred: impl Fn(usize) -> bool) -> Option<usize> {
        (0..m.pts.len()).find(|&v| core[v] && pred(v))
    }
}

/// Merge wall endpoints on the 1e-6 grid (centroid-averaged, like the
/// audit's map) and compute vertex kinds + the straight-through pairing.
fn build_map(walls: &[Wall]) -> WallMap {
    let mut acc: BTreeMap<(i64, i64), (f64, f64, u32)> = BTreeMap::new();
    let mut edges: Vec<((i64, i64), (i64, i64))> = Vec::with_capacity(walls.len());
    for w in walls {
        let (ka, kb) = (key_pt(w.a), key_pt(w.b));
        if ka == kb {
            continue;
        }
        for (k, p) in [(ka, w.a), (kb, w.b)] {
            let e = acc.entry(k).or_insert((0.0, 0.0, 0));
            e.0 += p.x;
            e.1 += p.y;
            e.2 += 1;
        }
        edges.push((ka, kb));
    }
    let idx: BTreeMap<(i64, i64), usize> =
        acc.keys().enumerate().map(|(i, k)| (*k, i)).collect();
    let mut pts: Vec<Vec2> = vec![Vec2::new(0.0, 0.0); acc.len()];
    for (k, (sx, sy, n)) in &acc {
        pts[idx[k]] = Vec2::new(sx / *n as f64, sy / *n as f64);
    }
    let mut adjset: Vec<std::collections::BTreeSet<usize>> = vec![Default::default(); pts.len()];
    for (ka, kb) in &edges {
        let (a, b) = (idx[ka], idx[kb]);
        if a != b {
            adjset[a].insert(b);
            adjset[b].insert(a);
        }
    }
    let adj: Vec<Vec<usize>> = adjset.into_iter().map(|s| s.into_iter().collect()).collect();

    // Pairing: at every even-degree vertex pair darts by best-antiparallel
    // match (audit tol 1e-4 on |sin| of the deviation). Odd-degree
    // vertices pair nothing. Degree-2 bends pair their two darts (a band
    // turns there — the audit stops strand walks at corners, but band
    // geometry must follow them; corners still gate interlace).
    let mut pairs = vec![Vec::new(); adj.len()];
    let mut kind = vec![VKind::Bad; adj.len()];
    for v in 0..adj.len() {
        let deg = adj[v].len();
        pairs[v] = vec![None; deg];
        if deg == 0 {
            kind[v] = VKind::Bad;
            continue;
        }
        if deg == 1 {
            kind[v] = VKind::Exit;
            continue;
        }
        if deg % 2 == 1 {
            kind[v] = VKind::Bad; // odd degree ≥ 3
            continue;
        }
        if deg == 2 {
            let p = pts[v];
            let a = unit(pts[adj[v][0]] - p);
            let b = unit(pts[adj[v][1]] - p);
            let err = if a.dot(b) < 0.0 {
                a.cross(b).abs()
            } else {
                2.0
            };
            kind[v] = if err <= 1e-4 {
                VKind::Straight2
            } else {
                VKind::Corner
            };
            pairs[v][0] = Some(adj[v][1]);
            pairs[v][1] = Some(adj[v][0]);
            continue;
        }
        // deg ≥ 4: greedy antiparallel pairing.
        let p = pts[v];
        let dirs: Vec<Vec2> = adj[v].iter().map(|&w| unit(pts[w] - p)).collect();
        let mut used = vec![false; deg];
        let mut ok = true;
        for i in 0..deg {
            if used[i] {
                continue;
            }
            let (mut best, mut bj) = (f64::INFINITY, None);
            for j in i + 1..deg {
                if used[j] {
                    continue;
                }
                let err = if dirs[i].dot(dirs[j]) < 0.0 {
                    dirs[i].cross(dirs[j]).abs()
                } else {
                    2.0
                };
                if err < best {
                    best = err;
                    bj = Some(j);
                }
            }
            let Some(j) = bj else { ok = false; break };
            if best > 1e-4 {
                ok = false;
                break;
            }
            used[i] = true;
            used[j] = true;
            pairs[v][i] = Some(adj[v][j]);
            pairs[v][j] = Some(adj[v][i]);
        }
        kind[v] = if ok { VKind::Joint } else { VKind::Bad };
    }
    WallMap { pts, adj, pairs, kind }
}

// ------------------------------------------------------------ strand walk --

/// One pass of a strand through a crossing vertex.
#[derive(Debug, Clone, Copy)]
struct Visit {
    vtx: usize,
    /// Index of this crossing along the strand (alternation parity).
    pos: usize,
    /// Unit travel tangent (previous visit/endpoint → next).
    dir: Vec2,
    /// Arc length from the previous crossing (or strand end) to this one.
    dprev: f64,
    /// Arc length to the next crossing (or strand end); NaN while open.
    dnext: f64,
}

#[derive(Debug, Clone)]
struct Strand {
    /// Vertices in travel order (a cycle without repetition if closed).
    pts: Vec<usize>,
    closed: bool,
    visits: Vec<Visit>,
}

/// Decompose the wall map into strands: maximal walks that pass straight
/// through joints, turn at corners, and stop at exits / unpaired darts.
/// Returns the strands plus per-vertex visit lists (crossing census).
fn walk_strands(m: &WallMap) -> (Vec<Strand>, Vec<Vec<(usize, usize)>>) {
    let n = m.pts.len();
    let mut seen: std::collections::HashSet<(usize, usize)> = Default::default();
    let mut ends: Vec<(usize, usize)> = Vec::new();
    for v in 0..n {
        for (i, &w) in m.adj[v].iter().enumerate() {
            if m.pairs[v][i].is_none() {
                ends.push((v, w));
            }
        }
    }
    let mut strands: Vec<Strand> = Vec::new();
    {
        let mut run_starts = |cand: &[(usize, usize)]| {
            for &(v0, w0) in cand {
                if seen.contains(&(v0, w0)) {
                    continue;
                }
                strands.push(walk_one(m, v0, w0, &mut seen));
            }
        };
        run_starts(&ends);
    }
    // Remaining darts belong to closed cycles.
    for v in 0..n {
        for &w in &m.adj[v] {
            if seen.contains(&(v, w)) {
                continue;
            }
            strands.push(walk_one(m, v, w, &mut seen));
        }
    }
    let mut at = vec![Vec::new(); n];
    for (si, s) in strands.iter().enumerate() {
        for vi in &s.visits {
            at[vi.vtx].push((si, vi.pos));
        }
    }
    (strands, at)
}

fn walk_one(
    m: &WallMap,
    v0: usize,
    w0: usize,
    seen: &mut std::collections::HashSet<(usize, usize)>,
) -> Strand {
    let mut pts = vec![v0];
    let mut visits: Vec<Visit> = Vec::new();
    let (mut prev, mut cur) = (v0, w0);
    let mut pos = 0;
    let mut run = 0.0;
    let mut closed = false;
    loop {
        seen.insert((prev, cur));
        seen.insert((cur, prev));
        run += (m.pts[cur] - m.pts[prev]).length();
        if cur == v0 && m.pair_at(cur, prev) == Some(w0) {
            // Back at the start with the arriving dart closing onto the
            // starting dart: the pass-through at v0 pairs them. (A
            // self-crossing orbit may pass through v0 mid-cycle — then
            // the arriving dart pairs elsewhere and the walk continues.)
            closed = true;
            if m.deg(v0) >= 4 && m.kind[v0] == VKind::Joint {
                visits.push(Visit {
                    vtx: v0,
                    pos,
                    dir: unit(m.pts[w0] - m.pts[prev]),
                    dprev: run,
                    dnext: f64::NAN,
                });
            }
            break;
        }
        pts.push(cur);
        let Some(nxt) = m.pair_at(cur, prev) else {
            break; // strand end (exit or unpaired dart)
        };
        if m.deg(cur) >= 4 && m.kind[cur] == VKind::Joint {
            visits.push(Visit {
                vtx: cur,
                pos,
                dir: unit(m.pts[nxt] - m.pts[prev]),
                dprev: run,
                dnext: f64::NAN,
            });
            pos += 1;
            run = 0.0;
        }
        prev = cur;
        cur = nxt;
    }
    // Fill in the arc lengths between consecutive crossings.
    for i in 0..visits.len() {
        visits[i].dnext = if i + 1 < visits.len() {
            visits[i + 1].dprev
        } else {
            run // tail after the last crossing (also the cycle's closing leg)
        };
    }
    if closed {
        // pts already starts at v0; no repeated vertex to remove.
        if visits.len() > 1 {
            // Closing visit's forward distance is the ring's opening leg.
            let first = visits.first().map(|v| v.dprev).unwrap_or(f64::NAN);
            visits.last_mut().unwrap().dnext = first;
        }
    }
    Strand {
        pts,
        closed,
        visits,
    }
}

// ------------------------------------------------------------- parity UF --

/// Union-find over strands carrying seed parities: `x[a] ^ x[b] = w`
/// equations (interlace alternation), solved incrementally; `unite`
/// returns false on contradiction.
struct ParityUf {
    parent: Vec<usize>,
    rank: Vec<u8>,
    xr: Vec<bool>, // seed[x] ^ seed[parent[x]]
}

impl ParityUf {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            xr: vec![false; n],
        }
    }

    fn find(&mut self, x: usize) -> (usize, bool) {
        if self.parent[x] == x {
            return (x, false);
        }
        let (r, p) = self.find(self.parent[x]);
        self.xr[x] ^= p;
        self.parent[x] = r;
        (r, self.xr[x])
    }

    fn unite(&mut self, a: usize, b: usize, w: bool) -> bool {
        let (ra, pa) = self.find(a);
        let (rb, pb) = self.find(b);
        if ra == rb {
            return (pa ^ pb) == w;
        }
        // seed[a] = pa' relative to ra etc.; need seed[ra]^seed[rb] so that
        // seed[a]^seed[b] = w.
        let link = pa ^ pb ^ w; // seed[ra] ^ seed[rb]
        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
            self.xr[ra] = link;
        } else {
            self.parent[rb] = ra;
            self.xr[rb] = link;
            if self.rank[ra] == self.rank[rb] {
                self.rank[ra] += 1;
            }
        }
        true
    }
}

// --------------------------------------------------------------- faces ----

/// Half-edge face traversal (same CCW-order / prev-neighbour rule and
/// interior-face convention as the audit). Returns (cycle, signed area)
/// per face, plus a conflict flag if any dart is claimed twice.
fn face_traversal(m: &WallMap) -> (Vec<(Vec<usize>, f64)>, bool) {
    let n = m.pts.len();
    let order: Vec<Vec<usize>> = (0..n)
        .map(|v| {
            let mut o = m.adj[v].clone();
            o.sort_by(|&a, &b| {
                let p = m.pts[v];
                fa(m.pts[a].y - p.y, m.pts[a].x - p.x)
                    .partial_cmp(&fa(m.pts[b].y - p.y, m.pts[b].x - p.x))
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            o
        })
        .collect();
    // next[(u,v)] = (v, the neighbour of v just BEFORE u in CCW order).
    let mut nxt: BTreeMap<(usize, usize), (usize, usize)> = BTreeMap::new();
    for v in 0..n {
        let d = order[v].len();
        for i in 0..d {
            nxt.insert(
                (order[v][i], v),
                (v, order[v][(i + d - 1) % d]),
            );
        }
    }
    let mut seen: std::collections::HashSet<(usize, usize)> = Default::default();
    let mut faces: Vec<(Vec<usize>, f64)> = Vec::new();
    let mut conflict = false;
    let mut claimed: std::collections::HashSet<(usize, usize)> = Default::default();
    for d0 in nxt.keys().copied() {
        if seen.contains(&d0) {
            continue;
        }
        let mut cyc = Vec::new();
        let mut c = d0;
        while !seen.contains(&c) {
            seen.insert(c);
            if !claimed.insert(c) {
                conflict = true;
            }
            cyc.push(c.0);
            c = nxt[&c];
        }
        let mut area = 0.0;
        let k = cyc.len();
        for i in 0..k {
            let (p, q) = (m.pts[cyc[i]], m.pts[cyc[(i + 1) % k]]);
            area += p.cross(q);
        }
        faces.push((cyc, area / 2.0));
    }
    (faces, conflict)
}

fn fa(y: f64, x: f64) -> f64 {
    // atan2 mapped to [0, 2pi) with a total order
    let a = y.atan2(x);
    if a < 0.0 {
        a + std::f64::consts::TAU
    } else {
        a
    }
}

/// Fill style: interior face 2-colouring (dual BFS restricted to faces
/// whose cycle lies entirely in the core — clipped rim oddities are
/// window artefacts, audit convention).
fn faces_and_colors(
    m: &WallMap,
    core: &[bool],
    style: Decoration,
) -> Result<Vec<(Vec<Vec2>, u8)>, DecorationError> {
    let n = m.pts.len();
    let any = |k: VKind| (0..n).any(|v| core[v] && m.kind[v] == k);
    let odd = (0..n).filter(|&v| core[v] && m.deg(v) % 2 == 1).count();
    if odd > 0 {
        let w = WallMap::witness(m, core, |v| m.deg(v) % 2 == 1).unwrap();
        return Err(deco_err(
            style,
            format!(
                "the wall map has {odd} odd-degree vertex/vertices inside the view \
                 (e.g. at ({:.3}, {:.3})) — fill needs an Eulerian map (2- and \
                 4-valent vertices only, Kaplan §3.7.1); generic contact angles \
                 open the strands, try a classic angle",
                m.pts[w].x, m.pts[w].y
            ),
        ));
    }
    if any(VKind::Corner) {
        let w = WallMap::witness(m, core, |v| m.kind[v] == VKind::Corner).unwrap();
        return Err(deco_err(
            style,
            format!(
                "a ray ends on a host edge inside the view (a 2-valent end-cap \
                 at ({:.3}, {:.3})) — the wall map does not close into a face \
                 tiling at this angle",
                m.pts[w].x, m.pts[w].y
            ),
        ));
    }
    let (faces, conflict) = face_traversal(m);
    if conflict {
        return Err(deco_err(
            style,
            "the wall map is non-manifold (a dart is claimed by two faces)".into(),
        ));
    }
    let interior: Vec<bool> = faces
        .iter()
        .map(|(cyc, _)| cyc.iter().all(|&v| core[v]))
        .collect();
    // Dual adjacency across darts whose BOTH endpoints are core.
    let mut first: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    let mut adjf: Vec<Vec<usize>> = vec![Vec::new(); faces.len()];
    for (fi, (cyc, _)) in faces.iter().enumerate() {
        let k = cyc.len();
        for i in 0..k {
            first.insert((cyc[i], cyc[(i + 1) % k]), fi);
        }
    }
    for (d, fi) in &first {
        if !interior[*fi] {
            continue;
        }
        if !(core[d.0] && core[d.1]) {
            continue;
        }
        if let Some(&fj) = first.get(&(d.1, d.0)) {
            if fj != *fi && interior[fj] {
                adjf[*fi].push(fj);
                adjf[fj].push(*fi);
            }
        }
    }
    let mut col = vec![u8::MAX; faces.len()];
    for s in 0..faces.len() {
        if col[s] != u8::MAX || !interior[s] {
            continue;
        }
        col[s] = 0;
        let mut q = std::collections::VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            for &w in &adjf[u] {
                if col[w] == u8::MAX {
                    col[w] = col[u] ^ 1;
                    q.push_back(w);
                } else if col[w] == col[u] {
                    return Err(deco_err(
                        style,
                        "the face adjacency is not 2-colourable (odd dual cycle)"
                            .to_string(),
                    ));
                }
            }
        }
    }
    let out = faces
        .iter()
        .zip(&col)
        .filter(|((_, a), c)| **c != u8::MAX && *a > 1e-12)
        .map(|((cyc, _), &c)| (cyc.iter().map(|&v| m.pts[v]).collect(), c))
        .collect();
    Ok(out)
}

// ------------------------------------------------------------ join geometry --

/// Outcome of the offset-join computation at one polyline vertex
/// (spike_mitre.py `join_geometry`).
enum Join {
    /// Collinear: emit the plain offset feet.
    Noop,
    /// Mitre: (left, right) corner points on the bisector at distance
    /// l = d / sin(θ/2) from the vertex, in opposite directions.
    Mitre(Vec2, Vec2),
    /// Clamped near-hairpin: bevel through the offset feet
    /// (f1l, f1r, f2l, f2r), each at distance exactly d from the vertex.
    Bevel(Vec2, Vec2, Vec2, Vec2),
}

fn join_geometry(prev: Vec2, v: Vec2, next: Vec2, d: f64) -> Join {
    let u = unit(prev - v);
    let x = unit(next - v);
    let cos_t = u.dot(x).clamp(-1.0, 1.0);
    let theta = cos_t.acos();
    if std::f64::consts::PI - theta < 1e-12 {
        return Join::Noop;
    }
    // Left normals of travel: incoming direction = -u, outgoing = x.
    let n1 = Vec2::new(u.y, -u.x);
    let n2 = Vec2::new(-x.y, x.x);
    let s = (theta / 2.0).sin();
    if d / s > MITRE_CLAMP * d {
        return Join::Bevel(
            v + n1 * d,
            v - n1 * d,
            v + n2 * d,
            v - n2 * d,
        );
    }
    // Exact mitre: intersect the left parallels and the right parallels.
    let cl = isect(v + n1 * d, -u, v + n2 * d, x);
    let cr = isect(v - n1 * d, -u, v - n2 * d, x);
    Join::Mitre(cl, cr)
}

fn isect(p: Vec2, d1: Vec2, q: Vec2, d2: Vec2) -> Vec2 {
    let den = d1.cross(d2);
    let t = (q - p).cross(d2) / den;
    p + d1 * t
}

/// Feet of the offset at a polyline endpoint, given the travel direction.
fn feet(p: Vec2, dir: Vec2, d: f64) -> (Vec2, Vec2) {
    let dir = unit(dir);
    let n = Vec2::new(-dir.y, dir.x);
    (p + n * d, p - n * d)
}

/// Offset polylines (left, right) of a strand's centerline. Open strands
/// get butt feet at both ends; closed strands offset as rings. The mitre
/// law and clamp behaviour mirror the spike exactly (both pinned at 1e-9).
fn offset_pts(m: &WallMap, s: &Strand, d: f64) -> (Vec<Vec2>, Vec<Vec2>) {
    let p = &s.pts;
    let n = p.len();
    let mut left: Vec<Vec2> = Vec::new();
    let mut right: Vec<Vec2> = Vec::new();
    if s.closed {
        for i in 0..n {
            let (a, v, b) = (m.pts[p[(i + n - 1) % n]], m.pts[p[i]], m.pts[p[(i + 1) % n]]);
            match join_geometry(a, v, b, d) {
                Join::Noop => {
                    let dir = unit(b - v);
                    let nrm = Vec2::new(-dir.y, dir.x);
                    left.push(v + nrm * d);
                    right.push(v - nrm * d);
                }
                Join::Mitre(cl, cr) => {
                    left.push(cl);
                    right.push(cr);
                }
                Join::Bevel(f1l, f1r, f2l, f2r) => {
                    left.push(f1l);
                    left.push(f2l);
                    right.push(f1r);
                    right.push(f2r);
                }
            }
        }
    } else {
        if n < 2 {
            return (left, right);
        }
        let (fl, fr) = feet(m.pts[p[0]], m.pts[p[1]] - m.pts[p[0]], d);
        left.push(fl);
        right.push(fr);
        for i in 1..n - 1 {
            let (a, v, b) = (m.pts[p[i - 1]], m.pts[p[i]], m.pts[p[i + 1]]);
            match join_geometry(a, v, b, d) {
                Join::Noop => {}
                Join::Mitre(cl, cr) => {
                    left.push(cl);
                    right.push(cr);
                }
                Join::Bevel(f1l, f1r, f2l, f2r) => {
                    left.push(f1l);
                    left.push(f2l);
                    right.push(f1r);
                    right.push(f2r);
                }
            }
        }
        let (fl, fr) = feet(m.pts[p[n - 1]], m.pts[p[n - 1]] - m.pts[p[n - 2]], d);
        left.push(fl);
        right.push(fr);
    }
    (left, right)
}

/// Shoelace area of a polygon (absolute).
fn poly_area(p: &[Vec2]) -> f64 {
    let mut a = 0.0;
    for i in 0..p.len() {
        let (q, r) = (p[i], p[(i + 1) % p.len()]);
        a += q.cross(r);
    }
    a.abs() / 2.0
}

// ------------------------------------------------------------- planning ----

/// Build the render plan for one decoration style over `walls`.
///
/// `region` is the patch's view region: precondition gates run only on
/// vertices inside it (windowing artefacts — degree-1 exits and ray
/// end-caps on unshared boundary edges — cannot lie strictly inside a
/// covered region; see [`WallMap::core`]).
pub fn plan_decoration(
    walls: &[Wall],
    band_w: f64,
    style: Decoration,
    region: &BBox,
) -> Result<DecorationPlan, DecorationError> {
    if !band_w.is_finite() || band_w <= 0.0 {
        return Err(DecorationError {
            style: style.name(),
            reason: format!("band width must be a positive finite number, got {band_w}"),
        });
    }
    let line_w = LINE_FRAC * band_w;
    let mut plan = DecorationPlan {
        band_w,
        line_w,
        ..Default::default()
    };
    let m = build_map(walls);
    if m.pts.is_empty() {
        return Ok(plan);
    }
    let core = m.core(region);
    match style {
        Decoration::Fill => {
            plan.faces = faces_and_colors(&m, &core, style)?;
        }
        Decoration::Outline => {
            let (strands, _) = walk_strands(&m);
            for s in &strands {
                let (l, r) = offset_pts(&m, s, band_w / 2.0);
                plan.side_lines.push((l, s.closed));
                plan.side_lines.push((r, s.closed));
            }
        }
        Decoration::Interlace | Decoration::OutlineInterlace => {
            let (strands, at) = walk_strands(&m);
            check_interlace(&m, &core, &at, style)?;
            let seeds = parity_seeds(&m, &strands, &at, style)?;
            let d = band_w / 2.0;
            let lh = line_w / 2.0;
            for s in &strands {
                let (l, r) = offset_pts(&m, s, d);
                plan.side_lines.push((l.clone(), s.closed));
                plan.side_lines.push((r.clone(), s.closed));
                if style == Decoration::Interlace {
                    if s.closed {
                        let (outer, hole) = if poly_area(&l) >= poly_area(&r) {
                            (l, Some(r))
                        } else {
                            (r, Some(l))
                        };
                        plan.band_fills.push((outer, hole));
                    } else {
                        let mut ring = l;
                        ring.extend(r.into_iter().rev());
                        plan.band_fills.push((ring, None));
                    }
                }
            }
            for v in 0..m.pts.len() {
                if at[v].len() != 2 || m.deg(v) < 4 {
                    continue;
                }
                let (s1, p1) = at[v][0];
                let (s2, _p2) = at[v][1];
                if s1 == s2 {
                    continue; // self-crossing: gated out above
                }
                let over1 = seeds[s1] ^ (p1 & 1 == 1);
                let (u, o) = if over1 {
                    (&strands[s2], &strands[s1])
                } else {
                    (&strands[s1], &strands[s2])
                };
                let vu = u.visits.iter().find(|vi| vi.vtx == v).unwrap();
                let vo = o.visits.iter().find(|vi| vi.vtx == v).unwrap();
                if let Some((quad, patch, lines)) = break_at(&m, v, vu, vo, d, lh) {
                    plan.break_quads.push(quad);
                    plan.break_patch.push(patch);
                    plan.over_lines.push(lines);
                }
            }
        }
    }
    Ok(plan)
}

/// Preconditions for interlace (§3.7.3): the core map must be 2/4-valent
/// with straight pairing (Kaplan's regime), every core crossing crossed by
/// exactly two strand passes, and the alternation system must be
/// consistent. Each failure is an honest error — the audit showed these
/// are real properties of the angle, not numerical noise.
fn check_interlace(
    m: &WallMap,
    core: &[bool],
    at: &[Vec<(usize, usize)>],
    style: Decoration,
) -> Result<(), DecorationError> {
    let n = m.pts.len();
    // degree-1 exits or ray end-caps in the core
    let opens: Vec<usize> = (0..n)
        .filter(|&v| core[v] && matches!(m.kind[v], VKind::Exit | VKind::Corner))
        .collect();
    if !opens.is_empty() {
        let w = m.pts[opens[0]];
        return Err(deco_err(style, format!(
            "{n0} open wall end(s) inside the view (degree-1 exits or ray \
             end-caps, e.g. at ({x:.3}, {y:.3})) — strands do not close into \
             a weave at this angle; generic contact angles give open strands, \
             a classic angle (e.g. 45° on squares, 60° on hexagonal hosts) \
             is required",
            n0 = opens.len(),
            x = w.x,
            y = w.y
        )));
    }
    let odds: Vec<usize> = (0..n)
        .filter(|&v| core[v] && m.deg(v) % 2 == 1 && m.deg(v) >= 3)
        .collect();
    if !odds.is_empty() {
        let w = m.pts[odds[0]];
        return Err(deco_err(style, format!(
            "{} odd-degree vertex/vertices inside the view (e.g. at ({:.3}, {:.3})) \
             — interlace needs 2- and 4-valent crossings only",
            odds.len(),
            w.x,
            w.y
        )));
    }
    let bads: Vec<usize> = (0..n)
        .filter(|&v| core[v] && m.kind[v] == VKind::Bad)
        .collect();
    if !bads.is_empty() {
        let w = m.pts[bads[0]];
        return Err(deco_err(style, format!(
            "{} crossing(s) whose wall pieces do not pair into straight strands \
             inside the view (e.g. at ({:.3}, {:.3}))",
            bads.len(),
            w.x,
            w.y
        )));
    }
    let not2: Vec<usize> = (0..n)
        .filter(|&v| {
            core[v] && m.kind[v] == VKind::Joint && m.deg(v) >= 4 && at[v].len() != 2
        })
        .collect();
    if !not2.is_empty() {
        let w = m.pts[not2[0]];
        return Err(deco_err(style, format!(
            "{} crossing(s) are not crossed by exactly two strands inside the \
             view (e.g. at ({:.3}, {:.3})) — multi-strand crossings have no \
             total over/under order",
            not2.len(),
            w.x,
            w.y
        )));
    }
    Ok(())
}

/// Solve the strand-seed bits: at a crossing where strands s1, s2 meet at
/// positions p1, p2, exactly one is over ⇒ `x[s1] ^ x[s2] = 1 ^ (p1&1) ^
/// (p2&1)`; a self-crossing requires the two passes to differ. Over at
/// position i on strand s is then `x[s] ^ (i & 1)` — alternation holds by
/// construction.
fn parity_seeds(
    m: &WallMap,
    strands: &[Strand],
    at: &[Vec<(usize, usize)>],
    style: Decoration,
) -> Result<Vec<bool>, DecorationError> {
    let mut uf = ParityUf::new(strands.len());
    for v in 0..m.pts.len() {
        if m.deg(v) < 4 || at[v].len() != 2 {
            continue;
        }
        let (s1, p1) = at[v][0];
        let (s2, p2) = at[v][1];
        if s1 == s2 {
            if (p1 ^ p2) & 1 == 0 {
                let w = m.pts[v];
                return Err(deco_err(
                    style,
                    format!(
                        "a strand crosses itself in phase at ({:.3}, {:.3}) — \
                         over/under cannot alternate consistently",
                        w.x, w.y
                    ),
                ));
            }
            continue;
        }
        if !uf.unite(s1, s2, ((p1 ^ p2) & 1) == 0) {
            return Err(deco_err(
                style,
                "strand parity is inconsistent: the over/under alternation closes \
                 into a contradiction — this is a property of the substrate and \
                 angle, not a numerical failure"
                    .to_string(),
            ));
        }
    }
    let mut seeds = Vec::with_capacity(strands.len());
    for s in 0..strands.len() {
        seeds.push(uf.find(s).1);
    }
    Ok(seeds)
}

/// The break at one crossing: under-strand direction `vu`, over-strand
/// direction `vo`. Erase quad = under strip ∩ (over footprint inset by the
/// break gap); patch = the over band segment covering it; over lines =
/// the over strand's side lines redrawn through the patch.
fn break_at(
    m: &WallMap,
    v: usize,
    vu: &Visit,
    vo: &Visit,
    d: f64,
    lh: f64,
) -> Option<(Vec<Vec2>, Vec<Vec2>, [(Vec2, Vec2); 2])> {
    let (t_u, t_o) = (vu.dir, vo.dir);
    let sin = t_u.cross(t_o).abs();
    let cos = t_u.dot(t_o);
    if sin < 1e-3 {
        return None; // tangential crossing: draw continuous
    }
    let gap = BREAK_GAP * d;
    let hu = d + lh; // under-strip half-width (covers the side-line stroke)
    let hi = d - gap; // inset over-band footprint half-width
    let n_u = Vec2::new(-t_u.y, t_u.x);
    let n_o = Vec2::new(-t_o.y, t_o.x);
    // Solve [n_u; n_o] p = (A, B) for the strip intersection corners.
    let det = n_u.x * n_o.y - n_o.x * n_u.y;
    if det.abs() < 1e-6 {
        return None;
    }
    let solve = |a: f64, b: f64| Vec2::new((a * n_o.y - b * n_u.y) / det, (b * n_u.x - a * n_o.x) / det);
    let q = m.pts[v];
    let quad = vec![
        q + solve(hu, hi),
        q + solve(hu, -hi),
        q + solve(-hu, -hi),
        q + solve(-hu, hi),
    ];
    // Over patch: cover the erase quad plus half a gap, clamped well
    // inside the neighbouring crossings along the over strand.
    let mut t = (hi + hu * cos.abs()) / sin + 0.5 * gap;
    let cap = vo.dprev.min(vo.dnext);
    if cap.is_finite() && cap > 0.0 {
        t = t.min(0.45 * cap);
    }
    if t <= 0.0 {
        return None;
    }
    let patch = vec![
        q + t_o * (-t) + n_o * (-hu),
        q + t_o * t + n_o * (-hu),
        q + t_o * t + n_o * hu,
        q + t_o * (-t) + n_o * hu,
    ];
    let lines = [
        (q + t_o * (-t) + n_o * d, q + t_o * t + n_o * d),
        (q + t_o * (-t) - n_o * d, q + t_o * t - n_o * d),
    ];
    Some((quad, patch, lines))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Polygon;

    fn pt(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    fn tile(pts: &[(f64, f64)]) -> Tile {
        Tile {
            proto: 0,
            poly: Polygon::from_slice(&pts.iter().map(|&(x, y)| pt(x, y)).collect::<Vec<_>>()),
            seed: 0,
        }
    }

    /// `n × n` unit-square host.
    fn square_grid(n: i64) -> Vec<Tile> {
        let mut v = Vec::new();
        for i in 0..n {
            for j in 0..n {
                let (x, y) = (i as f64, j as f64);
                v.push(tile(&[
                    (x, y),
                    (x + 1.0, y),
                    (x + 1.0, y + 1.0),
                    (x, y + 1.0),
                ]));
            }
        }
        v
    }

    fn key(w: &Wall) -> SegKey {
        key_seg(w.a, w.b)
    }

    /// Distance from `p` to segment `ab`.
    fn seg_dist(p: Vec2, a: Vec2, b: Vec2) -> f64 {
        let e = b - a;
        let t = ((p - a).dot(e) / e.dot(e)).clamp(0.0, 1.0);
        (p - (a + e * t)).length()
    }

    #[test]
    fn square_grid_at_45_gives_median_chords() {
        // δ = 45° on the square grid: each square carries the four
        // midpoint-to-midpoint chords of its inscribed diamond, shared
        // midpoint pairs dedupe, so 2×2 tiles yield 16 pieces of length
        // √½ with endpoints exactly on edge midpoints.
        let ws = walls(&square_grid(2), 45.0);
        assert_eq!(ws.len(), 16, "walls: {ws:?}");
        for w in &ws {
            assert!(
                (w.length() - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-12,
                "bad wall {w:?} len {}",
                w.length()
            );
        }
    }

    #[test]
    fn walls_meet_host_edges_at_midpoints() {
        // Midpoint-crossing invariant: on the regular (square, δ = 45°)
        // host every wall endpoint is the midpoint of some host edge —
        // walls cross substrate edges exactly at midpoints by construction.
        let tiles = square_grid(2);
        for w in walls(&tiles, 45.0) {
            for p in [w.a, w.b] {
                let is_midpoint = tiles.iter().any(|t| {
                    let q = &t.poly.pts;
                    (0..q.len()).any(|i| (p - (q[i] + q[(i + 1) % q.len()]) * 0.5).length() < 1e-9)
                });
                assert!(is_midpoint, "endpoint {p} is not an edge midpoint: {w:?}");
            }
        }
    }

    #[test]
    fn walls_never_exit_their_tile() {
        // Termination behavior at δ = 30° on the square grid: no wall
        // leaves its host tile (both endpoints contained — squares are
        // convex), and every endpoint is either on a tile edge (boundary
        // exit or midpoint) or an interior crossing where strands meet
        // (endpoint multiplicity ≥ 4: each crossing splits two strands
        // into four piece-ends). A non-midpoint boundary exit occurs.
        let tiles = square_grid(2);
        let ws = walls(&tiles, 30.0);
        assert!(!ws.is_empty());
        let mut mult: std::collections::BTreeMap<(i64, i64), usize> = Default::default();
        for w in &ws {
            for p in [w.a, w.b] {
                *mult.entry(key_pt(p)).or_default() += 1;
            }
        }
        let mut saw_boundary_exit = false;
        for w in &ws {
            let host = tiles
                .iter()
                .find(|t| t.poly.contains(w.a) && t.poly.contains(w.b))
                .unwrap_or_else(|| panic!("wall {w:?} escapes every tile"));
            for p in [w.a, w.b] {
                let q = &host.poly.pts;
                let d = (0..q.len())
                    .map(|i| seg_dist(p, q[i], q[(i + 1) % q.len()]))
                    .fold(f64::INFINITY, f64::min);
                if d < 1e-9 {
                    let is_mid = (0..q.len())
                        .any(|i| (p - (q[i] + q[(i + 1) % q.len()]) * 0.5).length() < 1e-9);
                    let is_vertex = q.iter().any(|v| (*v - p).length() < 1e-9);
                    if !is_mid && !is_vertex {
                        saw_boundary_exit = true;
                    }
                } else {
                    assert!(
                        mult[&key_pt(p)] >= 4,
                        "interior endpoint {p} of {w:?} is not a strand crossing"
                    );
                }
            }
        }
        assert!(saw_boundary_exit, "no ray ended at a non-midpoint boundary point");
    }

    #[test]
    fn walls_are_deterministic() {
        let tiles = square_grid(2);
        let a = walls(&tiles, 50.0);
        let b = walls(&tiles, 50.0);
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(&b) {
            assert_eq!(x, y, "walls must be bit-identical across calls");
        }
    }

    #[test]
    fn degenerate_tiles_are_skipped() {
        // < 3 vertices, collinear "tiles" and zero-area loops emit nothing…
        let junk = vec![
            tile(&[(0.0, 0.0), (1.0, 0.0)]),
            tile(&[(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)]),
            tile(&[]),
        ];
        assert!(walls(&junk, 60.0).is_empty());
        // …and a zero-length edge (duplicated vertex) contributes no rays:
        // the split-edge triangle must yield exactly the plain triangle's walls.
        let tri = tile(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]);
        let dup = tile(&[(0.0, 0.0), (1.0, 0.0), (1.0, 0.0), (0.0, 1.0)]);
        let w_tri = walls(&[tri], 60.0);
        let w_dup = walls(&[dup], 60.0);
        assert_eq!(w_tri.len(), w_dup.len());
        let mut k1: Vec<_> = w_tri.iter().map(key).collect();
        let mut k2: Vec<_> = w_dup.iter().map(key).collect();
        k1.sort();
        k2.sort();
        assert_eq!(k1, k2);
    }

    // ------------------------------------------------------------ decoration --

    use Decoration::*;

    /// The n×n square host spans [0,n]²; its margin-1 core (the gated
    /// region — all unshared-edge artefacts sit on the boundary ring).
    fn sq_core4(n: i64) -> BBox {
        BBox::new(pt(1.0, 1.0), pt(n as f64 - 1.0, n as f64 - 1.0))
    }

    /// The same core for a host translated by λ = (1, 0).
    fn sq_core4_shift(n: i64) -> BBox {
        BBox::new(pt(2.0, 1.0), pt(n as f64, n as f64 - 1.0))
    }

    /// Core rect for the sheared n=3 triangle host (rhombus
    /// [0,3.5+1]×[0,3h] shrunk one unit off every bbox side; a unit
    /// margin excludes all rim end-caps only while n ≤ 3).
    fn tri_core(n: i64, dx: f64) -> BBox {
        let h = 3.0f64.sqrt() / 2.0;
        BBox::new(
            pt(dx + 1.0, 1.0),
            pt(n as f64 + 0.5 * n as f64 - 1.0, n as f64 * h - 1.0),
        )
    }

    fn join_corners(prev: Vec2, v: Vec2, next: Vec2, d: f64) -> (Vec2, Vec2) {
        match join_geometry(prev, v, next, d) {
            Join::Mitre(cl, cr) => (cl, cr),
            _ => panic!("expected a mitre join"),
        }
    }

    /// Pins ported from the Python spike (spike_mitre.py), 1e-9.
    #[test]
    fn mitre_join_pins_match_spike() {
        use std::f64::consts::{FRAC_PI_3, FRAC_PI_4, FRAC_PI_6};
        let d = 0.1;
        // (a) right-angle L: (0,0)->(1,0)->(1,1), θ = 90°.
        let (cl, cr) = join_corners(pt(0.0, 0.0), pt(1.0, 0.0), pt(1.0, 1.0), d);
        assert!((cl.x - (1.0 - d)).abs() < 1e-9 && (cl.y - d).abs() < 1e-9, "cl {cl}");
        assert!((cr.x - (1.0 + d)).abs() < 1e-9 && (cr.y + d).abs() < 1e-9, "cr {cr}");
        let l = d / (FRAC_PI_4).sin(); // l = d / sin(θ/2)
        assert!(((cl - pt(1.0, 0.0)).length() - l).abs() < 1e-12);
        assert!(((cr - pt(1.0, 0.0)).length() - l).abs() < 1e-12);
        // (b) θ = 120°: rays V→prev at 180°, V→next at 60°; corners on the
        // bisector at l = d/sin(60°), each at distance d from line 1 (the
        // x axis, the incoming strip's line).
        let (cl, cr) = join_corners(
            pt(-1.0, 0.0),
            pt(0.0, 0.0),
            pt(FRAC_PI_3.cos(), FRAC_PI_3.sin()),
            d,
        );
        let l120 = d / FRAC_PI_3.sin();
        assert!((cl.length() - l120).abs() < 1e-12, "cl {cl}");
        assert!((cr.length() - l120).abs() < 1e-12, "cr {cr}");
        assert!((cl.y.abs() - d).abs() < 1e-9, "cl {cl}");
        assert!((cr.y.abs() - d).abs() < 1e-9, "cr {cr}");
        // (c) θ = 60° sharp corner: prev at −30°, next at +30° — the outer
        // (left) mitre behind V at −2d, the inner (right) one at +2d.
        let (cl, cr) = join_corners(
            pt(FRAC_PI_6.cos(), -FRAC_PI_6.sin()),
            pt(0.0, 0.0),
            pt(FRAC_PI_6.cos(), FRAC_PI_6.sin()),
            d,
        );
        assert!((cl.x + 2.0 * d).abs() < 1e-9 && cl.y.abs() < 1e-9, "cl {cl}");
        assert!((cr.x - 2.0 * d).abs() < 1e-9 && cr.y.abs() < 1e-9, "cr {cr}");
        // (d) collinear: no-op.
        assert!(matches!(
            join_geometry(pt(0.0, 0.0), pt(1.0, 0.0), pt(2.0, 0.0), d),
            Join::Noop
        ));
        // (e) mirrored L (rays at 180° and 270°).
        let (cl, cr) = join_corners(pt(-1.0, 0.0), pt(0.0, 0.0), pt(0.0, -1.0), d);
        assert!((cl.x - d).abs() < 1e-9 && (cl.y - d).abs() < 1e-9, "cl {cl}");
        assert!((cr.x + d).abs() < 1e-9 && (cr.y + d).abs() < 1e-9, "cr {cr}");
        // (f) clamp: a near-hairpin just past θ_crit bevels with all four
        // feet at distance exactly d from V…
        let thh = 2.0 * (1.0 / (MITRE_CLAMP * 1.0000001)).asin();
        match join_geometry(
            pt(-(thh / 2.0).cos(), (thh / 2.0).sin()),
            pt(0.0, 0.0),
            pt(-(thh / 2.0).cos(), -(thh / 2.0).sin()),
            d,
        ) {
            Join::Bevel(f1l, f1r, f2l, f2r) => {
                for f in [f1l, f1r, f2l, f2r] {
                    assert!((f.length() - d).abs() < 1e-12, "foot {f} not at d");
                }
            }
            _ => panic!("near-hairpin join must clamp to a bevel"),
        }
        // …and just under the clamp it is still a mitre.
        let thm = 2.0 * (1.0 / (MITRE_CLAMP * 0.9999999)).asin();
        assert!(matches!(
            join_geometry(
                pt(-(thm / 2.0).cos(), (thm / 2.0).sin()),
                pt(0.0, 0.0),
                pt(-(thm / 2.0).cos(), -(thm / 2.0).sin()),
                d
            ),
            Join::Mitre(_, _)
        ));
    }

    /// Hand-built map + open strand on a synthetic polyline (no tiling
    /// needed) so band geometry can be pinned analytically.
    fn chain(strand_pts: &[(f64, f64)]) -> (WallMap, Strand) {
        let pts: Vec<Vec2> = strand_pts.iter().map(|&(x, y)| pt(x, y)).collect();
        let n = pts.len();
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for i in 0..n - 1 {
            adj[i].push(i + 1);
            adj[i + 1].push(i);
        }
        let pairs = adj
            .iter()
            .map(|a| vec![None; a.len()])
            .collect::<Vec<_>>();
        let m = WallMap {
            pts,
            adj,
            pairs,
            kind: vec![VKind::Straight2; n],
        };
        let s = Strand {
            pts: (0..n).collect(),
            closed: false,
            visits: Vec::new(),
        };
        (m, s)
    }

    fn band_outline_test(strand_pts: &[(f64, f64)], d: f64) -> Vec<Vec2> {
        let (m, s) = chain(strand_pts);
        let (l, r) = offset_pts(&m, &s, d);
        let mut ring = l;
        ring.extend(r.into_iter().rev());
        ring
    }

    #[test]
    fn band_areas_pinned_to_exact_values() {
        let d = 0.1;
        // Straight band: rectangle of width 2d and length 3.
        let a = poly_area(&band_outline_test(&[(0.0, 0.0), (3.0, 0.0)], d));
        assert!((a - 3.0 * 2.0 * d).abs() < 1e-12, "straight area {a}");
        // Right-angle L of two length-3 segments: outer mitre triangle and
        // inner cut cancel exactly at θ = 90° ⇒ 12d (spike pin).
        let a = poly_area(&band_outline_test(
            &[(0.0, 0.0), (3.0, 0.0), (3.0, 3.0)],
            d,
        ));
        assert!((a - 12.0 * d).abs() < 1e-12, "L area {a}");
    }

    #[test]
    fn clamped_hairpin_band_stays_within_d() {
        let d = 0.1;
        let hair = band_outline_test(&[(0.0, 0.0), (3.0, 0.0), (2.999, 1e-6)], d);
        assert!(!hair.is_empty());
        let dseg = |p: Vec2, a: Vec2, b: Vec2| {
            let e = b - a;
            let t = ((p - a).dot(e) / e.dot(e)).clamp(0.0, 1.0);
            (p - (a + e * t)).length()
        };
        for p in &hair {
            let dist = dseg(*p, pt(0.0, 0.0), pt(3.0, 0.0))
                .min(dseg(*p, pt(3.0, 0.0), pt(2.999, 1e-6)));
            assert!(dist <= d + 1e-12, "outline vertex {p} escapes to {dist}");
        }
    }

    // ------------------------------------------------------------- styles ----

    fn point_in_poly(p: Vec2, poly: &[Vec2]) -> bool {
        let mut inside = false;
        let n = poly.len();
        for i in 0..n {
            let (a, b) = (poly[i], poly[(i + 1) % n]);
            if (a.y > p.y) != (b.y > p.y) {
                let x = a.x + (p.y - a.y) * (b.x - a.x) / (b.y - a.y);
                if p.x < x {
                    inside = !inside;
                }
            }
        }
        inside
    }

    fn centroid(poly: &[Vec2]) -> Vec2 {
        let mut s = pt(0.0, 0.0);
        for p in poly {
            s += *p;
        }
        s / poly.len() as f64
    }

    #[test]
    fn fill_square45_colouring_is_valid_and_translation_invariant() {
        let ws = walls(&square_grid(4), 45.0);
        let plan = plan_decoration(&ws, 0.12, Fill, &sq_core4(4)).expect("fill on square@45");
        assert!(!plan.faces.is_empty());
        let mut classes = [0usize; 2];
        for (_, c) in &plan.faces {
            classes[*c as usize] += 1;
        }
        assert!(classes[0] > 0 && classes[1] > 0, "both classes {classes:?}");
        // Adjacency check: faces sharing an edge (rounded endpoint pair)
        // must be in opposite classes.
        let mut owners: BTreeMap<((i64, i64), (i64, i64)), Vec<usize>> = BTreeMap::new();
        for (fi, (poly, _)) in plan.faces.iter().enumerate() {
            let n = poly.len();
            for i in 0..n {
                let (a, b) = (poly[i], poly[(i + 1) % n]);
                let (ka, kb) = (key_pt(a), key_pt(b));
                let k = if ka <= kb { (ka, kb) } else { (kb, ka) };
                owners.entry(k).or_default().push(fi);
            }
        }
        for fs in owners.values() {
            for &x in fs {
                for &y in fs {
                    if x != y {
                        assert_ne!(
                            plan.faces[x].1, plan.faces[y].1,
                            "adjacent faces share an edge but carry one colour"
                        );
                    }
                }
            }
        }
        // Translation invariance: the face at p and the face at p + (1, 0)
        // (a host lattice vector of the square grid) carry one class.
        let mut checked = 0;
        for (poly, c) in &plan.faces {
            let q = centroid(poly) + pt(1.0, 0.0);
            for (poly2, c2) in &plan.faces {
                if point_in_poly(q, poly2) {
                    assert_eq!(c, c2, "class differs under translation by (1,0)");
                    checked += 1;
                    break;
                }
            }
        }
        assert!(checked >= plan.faces.len() / 2, "checked {checked} faces");
    }

    #[test]
    fn interlace_square45_one_over_alternation_and_fold() {
        let ws = walls(&square_grid(4), 45.0);
        let m = build_map(&ws);
        let (strands, at) = walk_strands(&m);
        let seeds = parity_seeds(&m, &strands, &at, Interlace).expect("parity");
        // Exactly one strand over at every two-pass crossing, and the
        // alternation equation holds at each: over_i = x_s ^ (pos & 1).
        let mut crossings = 0;
        for v in 0..m.pts.len() {
            if at[v].len() == 2 && m.deg(v) >= 4 {
                crossings += 1;
                let (s1, p1) = at[v][0];
                let (s2, p2) = at[v][1];
                let o1 = seeds[s1] ^ ((p1 & 1) == 1);
                let o2 = seeds[s2] ^ ((p2 & 1) == 1);
                assert_ne!(o1, o2, "crossing {v} must have exactly one over");
            }
        }
        assert_eq!(crossings, 24, "X-joints on a 4×4 square host");
        // Crossings are visited in consecutive order along each strand.
        for s in &strands {
            for (i, v) in s.visits.iter().enumerate() {
                assert_eq!(v.pos, i);
            }
        }
        // The plan agrees: one break + patch + over-lines per crossing.
        let plan = plan_decoration(&ws, 0.12, Interlace, &sq_core4(4)).expect("interlace");
        assert_eq!(plan.break_quads.len(), crossings);
        assert_eq!(plan.break_patch.len(), crossings);
        assert_eq!(plan.over_lines.len(), crossings);
        // The hard obligation at patch level: the assignment is invariant
        // under the host lattice translation λ = (1, 0) — fold the plan and
        // compare before/after. Break patches encode WHICH strand is over
        // (their long axis is the over direction), so this pins parity, not
        // just geometry.
        let mut shifted_tiles = square_grid(4);
        for t in &mut shifted_tiles {
            for p in &mut t.poly.pts {
                p.x += 1.0;
            }
        }
        let plan2 =
            plan_decoration(&walls(&shifted_tiles, 45.0), 0.12, Interlace, &sq_core4_shift(4))
                .expect("shifted");
        let rk = |v: Vec2, dx: f64| ((v.x - dx) * 1e7, (v.y * 1e7));
        let mut map_a: Vec<((f64, f64), Vec<(f64, f64)>)> = Vec::new();
        for (q, p) in plan.break_quads.iter().zip(&plan.break_patch) {
            let c = centroid(q);
            if c.x >= 2.0 && c.x <= 2.99 && c.y >= 1.0 && c.y <= 3.0 {
                map_a.push(((c.x, c.y), p.iter().map(|v| rk(*v, 0.0)).collect()));
            }
        }
        let mut map_b: Vec<((f64, f64), Vec<(f64, f64)>)> = Vec::new();
        for (q, p) in plan2.break_quads.iter().zip(&plan2.break_patch) {
            let c = centroid(q);
            if c.x >= 3.0 && c.x <= 3.99 && c.y >= 1.0 && c.y <= 3.0 {
                map_b.push(((c.x - 1.0, c.y), p.iter().map(|v| rk(*v, 1.0)).collect()));
            }
        }
        assert!(!map_a.is_empty(), "fold overlap region empty");
        let near = |a: f64, b: f64| (a - b).abs() < 1e-4;
        for (c, pa) in &map_a {
            let mut hit = false;
            for (c2, pb) in &map_b {
                if !near(c.0, c2.0) || !near(c.1, c2.1) {
                    continue;
                }
                assert_eq!(pa.len(), pb.len());
                for (a, b) in pa.iter().zip(pb) {
                    assert!(near(a.0, b.0) && near(a.1, b.1), "patch mismatch at {c:?}");
                }
                hit = true;
            }
            assert!(hit, "no translated counterpart for crossing at {c:?}");
        }
    }

    /// Triangular host: δ = 60° is its classic angle (audit: fold=True).
    /// The rhombic patch is sheared, so a margin-1.0 bbox window only keeps
    /// all ray end-caps out of the core while n ≤ 3 (shear per row is
    /// 0.5 world units); n = 3 still leaves a core with crossing interior.
    fn triangle_grid(n: i64) -> Vec<Tile> {
        let h = 3.0f64.sqrt() / 2.0;
        let v = |i: i64, j: i64| pt(i as f64 + 0.5 * j as f64, h * j as f64);
        let mut out = Vec::new();
        for j in 0..n {
            for i in 0..n {
                out.push(tile(&[
                    (v(i, j).x, v(i, j).y),
                    (v(i + 1, j).x, v(i + 1, j).y),
                    (v(i, j + 1).x, v(i, j + 1).y),
                ]));
                out.push(tile(&[
                    (v(i + 1, j).x, v(i + 1, j).y),
                    (v(i + 1, j + 1).x, v(i + 1, j + 1).y),
                    (v(i, j + 1).x, v(i, j + 1).y),
                ]));
            }
        }
        out
    }

    #[test]
    fn interlace_triangle60_solves_and_folds() {
        let ws = walls(&triangle_grid(3), 60.0);
        let plan = plan_decoration(&ws, 0.12, Interlace, &tri_core(3, 0.0)).expect("interlace on triangle@60");
        assert!(!plan.break_quads.is_empty());
        assert_eq!(plan.break_quads.len(), plan.break_patch.len());
        // Lattice translation λ = (1, 0) fold on the break pattern.
        let mut shifted = triangle_grid(3);
        for t in &mut shifted {
            for p in &mut t.poly.pts {
                p.x += 1.0;
            }
        }
        let plan2 =
            plan_decoration(&walls(&shifted, 60.0), 0.12, Interlace, &tri_core(3, 1.0))
                .expect("shifted tri");
        let mut found = 0;
        for (q, p) in plan.break_quads.iter().zip(&plan.break_patch) {
            let c = centroid(q);
            // Overlap of the two patches minus margins (x ∈ [2, 3] of the
            // unshifted plan is well inside both host windows).
            if !(2.05..=2.95).contains(&c.x) || !(1.0..=2.4).contains(&c.y) {
                continue;
            }
            let mut hit = false;
            for (q2, p2) in plan2.break_quads.iter().zip(&plan2.break_patch) {
                let c2 = centroid(q2);
                if (c2.x - c.x - 1.0).abs() > 1e-6 || (c2.y - c.y).abs() > 1e-6 {
                    continue;
                }
                let near = |a: f64, b: f64| (a - b).abs() < 1e-4;
                assert_eq!(p.len(), p2.len());
                let ok = p.iter().zip(p2).all(|(a, b)| {
                    near(a.x, b.x - 1.0) && near(a.y, b.y)
                });
                assert!(ok, "folded patches differ at crossing {c}");
                hit = true;
                found += 1;
            }
            assert!(hit, "crossing at {c} vanished under translation");
        }
        assert!(found >= 4, "fold overlap checked {found} crossings");
    }

    #[test]
    fn interlace_and_fill_gate_generic_delta() {
        // δ = 30° on the square host: rays end at non-midpoint boundary
        // points (the audit's exit1/d2c regime) — fill and interlace must
        // refuse with honest, style-named errors.
        let ws = walls(&square_grid(3), 30.0);
        let e = plan_decoration(&ws, 0.12, Fill, &sq_core4(3)).expect_err("fill at generic δ");
        assert!(e.to_string().contains("'fill'"), "{e}");
        let e = plan_decoration(&ws, 0.12, Interlace, &sq_core4(3)).expect_err("interlace at generic δ");
        assert!(e.to_string().contains("'interlace'"), "{e}");
        // Outline always renders (offset curves need no preconditions).
        let plan = plan_decoration(&ws, 0.12, Outline, &sq_core4(3)).expect("outline at generic δ");
        assert!(!plan.side_lines.is_empty());
    }

    #[test]
    fn decoration_parse_roundtrip() {
        for s in ["fill", "outline", "interlace", "outline-interlace"] {
            assert_eq!(Decoration::parse(s).unwrap().name(), s);
        }
        assert!(Decoration::parse("weave").is_none());
        // Band width must be a positive finite number.
        let ws = walls(&square_grid(2), 45.0);
        assert!(plan_decoration(&ws, 0.0, Outline, &sq_core4(2)).is_err());
        assert!(plan_decoration(&ws, f64::NAN, Outline, &sq_core4(2)).is_err());
    }
}
