//! Symmetry overlay (C5): the *certified* symmetry elements of a periodic
//! tiling — the unit cell, rotation centers, mirror lines and glide axes —
//! derived numerically from the rendered patch and emitted as pure data for
//! the SVG export side.
//!
//! # What is drawn
//!
//! * the **unit cell** — the period parallelogram `⟨l1, l2⟩` from
//!   `periodic`, anchored at a certified symmetry site (or the nearest
//!   lattice point when the group has no rotational sites, e.g. p1);
//! * **rotation centers** — each drawn at its true position with a per-fold
//!   marker (an `n`-spoke star);
//! * **mirror lines** — solid, one per certified axis;
//! * **glide axes** — dashed, one per certified axis. A glide is a
//!   reflection composed with the half-lattice translation along the axis;
//!   it is drawn only where the *pure* reflection is **not** a symmetry, so
//!   mirrors and glides never collide on the same line (the documented
//!   mirror/glide convention).
//!
//! # Derivation (data-driven, not a hardcoded group table)
//!
//! Sites are *derived from the patch itself*, then *certified* against the
//! patch's vertex set. Every emitted glyph traces to a certified site — a
//! candidate that cannot be verified against the rendered tiles is dropped,
//! so the overlay never draws a symmetry the tiling lacks.
//!
//! The **declared group** only restricts *which operations are looked for*
//! (a p4 tiling is never scanned for 6-fold centers; a pg tiling is never
//! scanned for mirrors). The candidate *positions* are group-agnostic, built
//! from tile-centroid pairs in the patch:
//!
//! * a 2-fold center is the midpoint of a centroid pair;
//! * a 3-/6-fold center is an equilateral-triangle apex on a pair base;
//! * a 4-fold center is a right-isosceles-triangle apex on a pair base;
//! * a mirror line is the perpendicular bisector of a pair whose separation
//!   is ⊥ the candidate direction;
//! * a glide axis is the same line, where the pair separation carries the
//!   half-period translation along the axis.
//!
//! Candidate directions are the lattice frame `D = {l1, l2, l1+l2, l1−l2}`,
//! deduped to undirected lines.
//!
//! # Coset reduction
//!
//! The sites of a lattice tiling come in whole lattices: a given coset of
//! the period lattice `Λ = ⟨l1, l2⟩` holds infinitely many copies of the
//! same site (all lattice translates), and every copy in a large patch has
//! the same local geometry. Deriving every copy directly would be O(V·k²).
//! Instead, candidate sites are *streamed* into a coset map — one
//! representative per coset, kept to the one nearest the region center — and
//! a site is certified once, then **all** of its lattice translates that fall
//! inside the region are emitted. The overlay therefore draws every site in
//! the view, not one per coset.
//!
//! A candidate is a true site iff its whole coset is; certification is done
//! on the (interior, deepest-in-the-patch) representative so it is robust
//! against the finite patch boundary.
//!
//! # Certification (the rule a glyph must satisfy to be drawn)
//!
//! A candidate site is **certified** iff, over every *patch vertex* within
//! the local radius `R` of the site (rotation) or band (line), the
//! transformed vertex is present in the patch's vertex set (within `tol`):
//!
//! * rotation of fold `n`: each nearby vertex's `2π/n`-image about the
//!   center is a patch vertex, with ≥ [`MIN_TESTED`] vertices tested and a
//!   non-collinear (genuinely 2-D) local star;
//! * mirror: each nearby vertex's reflection in the line is a patch vertex,
//!   with ≥ 1 tested vertex on each side of the line;
//! * glide: same as the mirror, but the reflected image is shifted by the
//!   half-period along the axis, and a glide is drawn only where the pure
//!   mirror fails.
//!
//! **Boundary-skip.** A vertex whose image falls *outside the patch's vertex
//! bounding box* (padded by `tol`) is *skipped*, not counted as a failure: a
//! finite patch cannot certify beyond its own edge, so the test only demands
//! that the image be a vertex where the patch actually has vertices. This is
//! what makes glide axes certifiable in a finite patch.
//!
//! **Completeness caveat (line sites on large jagged patches).** The vertex
//! bounding box is a *loose* bound: a genuine image can fall *inside* the box
//! yet be a boundary vertex the finite patch does not contain (the patch is a
//! whole-tile inclusion, so its true edge is jagged, not the box edge). When
//! that happens the site is rejected — a *false negative*, never a false
//! positive, because the skip only removes evidence, it never adds a fake
//! pass. Point (rotation) sites are immune in practice: their test disk has
//! radius `R` and stays near the site, so it does not reach a far, jagged
//! patch edge. *Line* sites (mirrors/glides) are exposed: their test *band*
//! spans the whole patch, so a long mirror whose band reaches a jagged corner
//! can be suppressed by a handful of un-imageable boundary vertices even
//! though the mirror is genuine. Example: `hankin-square-two-point` (cmm,
//! 6389 vertices) certifies every 2-fold center but drops its genuine
//! diagonal mirror families (each passes 781 of 785 nearby vertices; the 4
//! misses are boundary vertices at the patch's extreme edge). The overlay is
//! therefore *complete* on small/clean patches (the unit-cell fixtures) and
//! *conservatively incomplete* for line sites on large jagged ones; it is
//! never incorrect.
//!
//! "Patch vertex" is the globally deduped vertex set of the rendered tiles
//! (the image of one tile may land on a vertex shared with a neighbour).
//!
//! # Coset note (p1)
//!
//! The world lattice origin is not exposed through the [`Tiling`] trait
//! (the result carries the period vectors, not the offset), so the cell
//! anchor for p1 — the only group with no rotational sites — is the lattice
//! point of the world-origin frame closest to the region center. Every
//! registry `LatticeTiling` uses `offset = 0` and a similarity map, so that
//! frame *is* the pattern lattice in practice; for a table with a nonzero
//! offset the cell shape/orientation is still the true period parallelogram,
//! but its corner may sit in a sibling lattice coset.
//!
//! [`Tiling`]: crate::tiling::Tiling

use crate::geometry::{BBox, Vec2};
use crate::symmetry::WallpaperGroup;
use crate::tiling::Tile;
use std::collections::HashMap;

/// Minimum number of nearby patch vertices a site must be certified against
/// (a non-degenerate local star, not a 1-D row coincidence).
pub const MIN_TESTED: usize = 4;

/// A certified rotation center.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RotationCenter {
    pub point: Vec2,
    /// Rotation fold: 2, 3, 4, or 6.
    pub fold: u8,
}

/// A certified mirror line (a point on the line + its unit direction).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MirrorLine {
    /// The foot of the perpendicular from the origin onto the line,
    /// `direction × n`-style: `point = n * s` where `n = (-dir.y, dir.x)`.
    pub point: Vec2,
    /// Unit direction along the line.
    pub direction: Vec2,
}

/// A certified glide axis (point + unit direction + the half-period glide
/// translation along the axis).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlideAxis {
    /// The foot of the perpendicular from the origin onto the axis,
    /// `point = n * s` where `n = (-dir.y, dir.x)`.
    pub point: Vec2,
    /// Unit direction along the axis.
    pub direction: Vec2,
    /// The half-period translation (along `direction`, signed) that composes
    /// with the reflection about the axis to give the glide.
    pub glide: Vec2,
}

/// The derived overlay: unit cell + certified sites.
#[derive(Debug, Clone, PartialEq)]
pub struct SymmetryOverlay {
    pub group: WallpaperGroup,
    /// Unit-cell corners in order: `c, c+l1, c+l1+l2, c+l2`.
    pub cell: [Vec2; 4],
    pub centers: Vec<RotationCenter>,
    pub mirrors: Vec<MirrorLine>,
    pub glides: Vec<GlideAxis>,
    /// `max(|l1|, |l2|, 1.0)` — the glyph-sizing scale the SVG emitter uses.
    pub scale: f64,
}

// ──────────────────────────────────────────────────────────────────────────
// Group theory: which operations a group can carry.
// ──────────────────────────────────────────────────────────────────────────

/// The rotation folds a group admits, highest first (a site is recorded at
/// its *highest* fold, so a 6-fold center is not also drawn as 3- and 2-fold).
fn rotations_of(g: WallpaperGroup) -> &'static [u8] {
    match g {
        WallpaperGroup::P1 => &[],
        WallpaperGroup::P2
        | WallpaperGroup::Pm
        | WallpaperGroup::Pg
        | WallpaperGroup::Cm
        | WallpaperGroup::Pmm
        | WallpaperGroup::Pgg
        | WallpaperGroup::Cmm
        | WallpaperGroup::Pmg => &[2],
        WallpaperGroup::P4 | WallpaperGroup::P4m | WallpaperGroup::P4g => &[4, 2],
        WallpaperGroup::P3 | WallpaperGroup::P3m1 | WallpaperGroup::P31m => &[3],
        WallpaperGroup::P6 | WallpaperGroup::P6m => &[6, 3, 2],
    }
}

/// Whether the group carries mirror reflections.
fn has_mirrors(g: WallpaperGroup) -> bool {
    matches!(
        g,
        WallpaperGroup::Pm
            | WallpaperGroup::Pmm
            | WallpaperGroup::Pmg
            | WallpaperGroup::Cm
            | WallpaperGroup::Cmm
            | WallpaperGroup::P4m
            | WallpaperGroup::P4g
            | WallpaperGroup::P3m1
            | WallpaperGroup::P31m
            | WallpaperGroup::P6m
    )
}

/// Whether the group carries glide reflections.
fn has_glides(g: WallpaperGroup) -> bool {
    matches!(
        g,
        WallpaperGroup::Pg | WallpaperGroup::Pmg | WallpaperGroup::Cm | WallpaperGroup::Cmm | WallpaperGroup::P4g
    )
}

// ──────────────────────────────────────────────────────────────────────────
// Small geometry / lattice helpers.
// ──────────────────────────────────────────────────────────────────────────

fn centroid(pts: &[Vec2]) -> Vec2 {
    if pts.is_empty() {
        return Vec2::ZERO;
    }
    let mut s = Vec2::ZERO;
    for p in pts {
        s += *p;
    }
    s / pts.len() as f64
}

/// Rotate `p` about `c` by `theta`.
#[inline]
fn rot(p: Vec2, c: Vec2, theta: f64) -> Vec2 {
    c + (p - c).rotated(theta)
}

/// Reflect `p` in the line through `m` with unit direction `u`.
#[inline]
fn reflect_line(p: Vec2, m: Vec2, u: Vec2) -> Vec2 {
    let w = p - m;
    let a = w.dot(u);
    m + u * (2.0 * a) - w
}

/// Lattice coordinates `(m, n)` of `p` in the basis `⟨l1, l2⟩`.
#[inline]
fn lat_coords(p: Vec2, l1: Vec2, l2: Vec2, det: f64) -> (f64, f64) {
    (p.cross(l2) / det, l1.cross(p) / det)
}

/// The coset of `p` modulo the lattice `⟨l1, l2⟩`, as a quantized pair of
/// fractional lattice coordinates (resolution `1e-4`). Two points land in the
/// same coset iff they differ by a lattice vector.
#[inline]
fn coset_key(p: Vec2, l1: Vec2, l2: Vec2, det: f64) -> (i64, i64) {
    let (m, n) = lat_coords(p, l1, l2, det);
    let fx = (m - m.floor()) * 1e4;
    let fy = (n - n.floor()) * 1e4;
    (fx.round() as i64 % 10000, fy.round() as i64 % 10000)
}

/// The nearest integer-combination lattice point `m·l1 + n·l2` to `p`.
fn nearest_lattice_point(p: Vec2, l1: Vec2, l2: Vec2, det: f64) -> Vec2 {
    let (m, n) = lat_coords(p, l1, l2, det);
    l1 * m.round() + l2 * n.round()
}

/// `(unit direction, half-period glide length)` for each of the undirected
/// lattice-frame directions `l1, l2, l1+l2, l1−l2`. The glide length is half
/// the *smallest* lattice period along the direction (found by scanning
/// integer combinations `a·l1 + b·l2` in `[-4,4]²`).
fn direction_frame(l1: Vec2, l2: Vec2) -> Vec<(Vec2, f64)> {
    let raw = [l1, l2, l1 + l2, l1 - l2];
    let mut out: Vec<(Vec2, f64)> = Vec::new();
    for v in raw {
        let len = v.length();
        if len < 1e-12 {
            continue;
        }
        let mut u = v / len;
        // Canonicalize to an undirected line: +x preferred, then +y.
        if u.x < -1e-12 || (u.x.abs() <= 1e-12 && u.y < 0.0) {
            u = -u;
        }
        if out.iter().any(|(eu, _)| (*eu - u).length() < 1e-9) {
            continue;
        }
        let mut best: Option<Vec2> = None;
        for a in -4..=4 {
            for b in -4..=4 {
                let w = l1 * (a as f64) + l2 * (b as f64);
                let wl = w.length();
                if wl < 1e-12 {
                    continue;
                }
                if w.cross(u).abs() <= 1e-9 * wl {
                    if best.as_ref().map_or(true, |bw| wl < bw.length()) {
                        best = Some(w);
                    }
                }
            }
        }
        let glen = best.map_or(0.0, |w| w.length() * 0.5);
        out.push((u, glen));
    }
    out
}

/// The `[m, n]` lattice-index range that must be scanned to cover the region
/// when emitting translates of `p` (region corners, expanded by one period).
fn lat_range(p: Vec2, region: BBox, l1: Vec2, l2: Vec2, det: f64) -> (i64, i64, i64, i64) {
    let mut lomi = f64::INFINITY;
    let mut himi = f64::NEG_INFINITY;
    let mut lona = f64::INFINITY;
    let mut hina = f64::NEG_INFINITY;
    for c in region.corners() {
        let (m, n) = lat_coords(c - p, l1, l2, det);
        lomi = lomi.min(m);
        himi = himi.max(m);
        lona = lona.min(n);
        hina = hina.max(n);
    }
    (
        lomi.floor() as i64 - 1,
        himi.ceil() as i64 + 1,
        lona.floor() as i64 - 1,
        hina.ceil() as i64 + 1,
    )
}

/// Length of the segment `{ p = n*s + t*u : p ∈ region }` — 0.0 when the line
/// does not cross the region in a segment of positive length (e.g. it only
/// touches a corner). Used to drop degenerate glyphs after bbox clipping.
fn line_segment_length(s: f64, n: Vec2, u: Vec2, region: BBox) -> f64 {
    let p0 = n * s;
    let mut tlo = f64::NEG_INFINITY;
    let mut thi = f64::INFINITY;
    let comps = [
        (region.min.x, region.max.x, u.x, p0.x),
        (region.min.y, region.max.y, u.y, p0.y),
    ];
    for (lo, hi, cu, c0) in comps {
        if cu.abs() < 1e-12 {
            if !(lo - 1e-9 <= c0 && c0 <= hi + 1e-9) {
                return 0.0; // parallel and outside
            }
        } else {
            let a = (lo - c0) / cu;
            let b = (hi - c0) / cu;
            tlo = tlo.max(a.min(b));
            thi = thi.min(a.max(b));
        }
    }
    (thi - tlo).max(0.0)
}

// ──────────────────────────────────────────────────────────────────────────
// Certification predicates (full vertex scan, boundary-skip).
// ──────────────────────────────────────────────────────────────────────────

struct Verts<'a> {
    verts: &'a [Vec2],
    vmap: &'a HashMap<(i64, i64), Vec2>,
    vscale: f64,
    vx0: f64,
    vx1: f64,
    vy0: f64,
    vy1: f64,
}

impl<'a> Verts<'a> {
    #[inline]
    fn in_aabb(&self, p: Vec2) -> bool {
        self.vx0 <= p.x && p.x <= self.vx1 && self.vy0 <= p.y && p.y <= self.vy1
    }

    #[inline]
    fn has(&self, p: Vec2) -> bool {
        let k = ((p.x * self.vscale).round() as i64, (p.y * self.vscale).round() as i64);
        for dx in -1..=1i64 {
            for dy in -1..=1i64 {
                if self.vmap.contains_key(&(k.0 + dx, k.1 + dy)) {
                    return true;
                }
            }
        }
        false
    }
}

/// Does a fold-`n` rotation about `c` certify on the patch vertex set?
fn rotation_certified(v: &Verts, c: Vec2, fold: u8, r: f64, tol: f64) -> bool {
    let theta = std::f64::consts::TAU / fold as f64;
    let mut tested = 0usize;
    let mut rel: Vec<Vec2> = Vec::new();
    for &p in v.verts {
        if (p - c).length() > r {
            continue;
        }
        rel.push(p - c);
        let w = rot(p, c, theta);
        if !v.in_aabb(w) {
            continue; // boundary-skip
        }
        tested += 1;
        if !v.has(w) {
            return false;
        }
    }
    if tested < MIN_TESTED {
        return false;
    }
    let thr = tol * r;
    for i in 0..rel.len() {
        for j in i + 1..rel.len() {
            if rel[i].cross(rel[j]).abs() > thr {
                return true;
            }
        }
    }
    false
}

/// Does the line (offset `s` along unit normal `n`, direction `u`, optional
/// glide translation `g` along the axis) certify on the patch vertex set?
fn line_certified(v: &Verts, s: f64, n: Vec2, u: Vec2, g: Option<Vec2>, r: f64, tol: f64) -> bool {
    let m = n * s; // a point on the line
    let mut tested = 0usize;
    let mut pos = 0usize;
    let mut neg = 0usize;
    for &p in v.verts {
        let d = p.dot(n) - s; // signed distance of the vertex to the line
        if d.abs() > r {
            continue;
        }
        if d > tol {
            pos += 1;
        } else if d < -tol {
            neg += 1;
        }
        let mut w = reflect_line(p, m, u);
        if let Some(gv) = g {
            w += gv;
        }
        if !v.in_aabb(w) {
            continue; // boundary-skip
        }
        tested += 1;
        if !v.has(w) {
            return false;
        }
    }
    tested >= MIN_TESTED && pos >= 1 && neg >= 1
}

// ──────────────────────────────────────────────────────────────────────────
// The entry point.
// ──────────────────────────────────────────────────────────────────────────

/// Derive the certified symmetry overlay of a periodic tiling from its
/// rendered `tiles` (world coordinates), declared `group`, period vectors
/// `periodic = [l1, l2]` and view `region`.
pub fn derive_overlay(
    tiles: &[Tile],
    group: WallpaperGroup,
    periodic: [Vec2; 2],
    region: BBox,
) -> SymmetryOverlay {
    let [l1, l2] = periodic;
    let det = l1.cross(l2);
    let max_edge = l1.length().max(l2.length()).max(1.0);
    let tol = 1e-6 * max_edge;
    let cluster_tol = 1e-4 * max_edge;
    let cutoff = 3.0 * max_edge;
    let r = 1.5 * max_edge;
    let rc = region.center();

    let cell_for_anchor = |anchor: Vec2| -> [Vec2; 4] {
        [anchor, anchor + l1, anchor + l1 + l2, anchor + l2]
    };

    // Degenerate guard: no usable lattice -> collapsed cell, no sites.
    if det.abs() < 1e-12 {
        return SymmetryOverlay {
            group,
            cell: [rc, rc, rc, rc],
            centers: Vec::new(),
            mirrors: Vec::new(),
            glides: Vec::new(),
            scale: max_edge,
        };
    }

    // No tiles -> the lattice is still valid; anchor at the nearest lattice
    // point and draw a cell-only overlay.
    if tiles.is_empty() {
        let anchor = nearest_lattice_point(rc, l1, l2, det);
        return SymmetryOverlay {
            group,
            cell: cell_for_anchor(anchor),
            centers: Vec::new(),
            mirrors: Vec::new(),
            glides: Vec::new(),
            scale: max_edge,
        };
    }

    // 1. Globally deduped patch vertex set + membership hash.
    let vscale = 1.0 / (2.0 * tol);
    let mut vmap: HashMap<(i64, i64), Vec2> = HashMap::new();
    let mut verts: Vec<Vec2> = Vec::new();
    for t in tiles {
        for p in &t.poly.pts {
            let k = ((p.x * vscale).round() as i64, (p.y * vscale).round() as i64);
            if !vmap.contains_key(&k) {
                vmap.insert(k, *p);
                verts.push(*p);
            }
        }
    }
    let mut vx0 = f64::INFINITY;
    let mut vx1 = f64::NEG_INFINITY;
    let mut vy0 = f64::INFINITY;
    let mut vy1 = f64::NEG_INFINITY;
    for &p in &verts {
        vx0 = vx0.min(p.x);
        vx1 = vx1.max(p.x);
        vy0 = vy0.min(p.y);
        vy1 = vy1.max(p.y);
    }
    vx0 -= tol;
    vx1 += tol;
    vy0 -= tol;
    vy1 += tol;
    let v = Verts {
        verts: &verts,
        vmap: &vmap,
        vscale,
        vx0,
        vx1,
        vy0,
        vy1,
    };

    // 2. Centroids.
    let cts: Vec<Vec2> = tiles.iter().map(|t| centroid(&t.poly.pts)).collect();

    // 3. Operation sets from the declared group + direction frame.
    let folds = rotations_of(group);
    let mirrors_ok = has_mirrors(group);
    let glides_ok = has_glides(group);
    let dirs = direction_frame(l1, l2);

    // 4. Streaming coset maps (representative = closest to the region
    //    center, tie-break lexicographic).
    let mut rot_reps: HashMap<(i64, i64), (Vec2, f64)> = HashMap::new();
    let mut line_reps: HashMap<(usize, i64), (f64, f64)> = HashMap::new();

    let coset_key = |p: Vec2| coset_key(p, l1, l2, det);
    let offer_rot = |map: &mut HashMap<(i64, i64), (Vec2, f64)>, p: Vec2| {
        let k = coset_key(p);
        let d = (p - rc).length();
        match map.get(&k) {
            None => {
                map.insert(k, (p, d));
            }
            Some((q, d0)) => {
                let lex = p.x < q.x || (p.x == q.x && p.y < q.y);
                if d < *d0 - 1e-12 || ((d - *d0).abs() <= 1e-12 && lex) {
                    map.insert(k, (p, d));
                }
            }
        }
    };
    let offer_line = |map: &mut HashMap<(usize, i64), (f64, f64)>,
                      di: usize,
                      s: f64,
                      n: Vec2| {
        let sk = (s / cluster_tol).round() as i64;
        let d = (s - rc.dot(n)).abs();
        match map.get(&(di, sk)) {
            None => {
                map.insert((di, sk), (s, d));
            }
            Some((s0, d0)) => {
                if d < *d0 - 1e-12 || ((d - *d0).abs() <= 1e-12 && s < *s0) {
                    map.insert((di, sk), (s, d));
                }
            }
        }
    };

    // 5. Centroid spatial hash (cell size = cutoff ≥ radius) for the pair
    //    loop.
    let cell_of = |p: Vec2| ((p.x / cutoff).floor() as i64, (p.y / cutoff).floor() as i64);
    let mut h: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
    for (i, &cen) in cts.iter().enumerate() {
        h.entry(cell_of(cen)).or_default().push(i);
    }

    // 6. Candidate generation from centroid pairs -> streaming coset offers.
    for (i, &ci) in cts.iter().enumerate() {
        let (cx, cy) = cell_of(ci);
        for dx in -1..=1i64 {
            for dy in -1..=1i64 {
                let Some(list) = h.get(&(cx + dx, cy + dy)) else {
                    continue;
                };
                for &j in list {
                    if j <= i {
                        continue;
                    }
                    let cj = cts[j];
                    let d = cj - ci;
                    let l = d.length();
                    if l > cutoff || l <= tol {
                        continue;
                    }
                    let m = (ci + cj) * 0.5;
                    // 2-fold midpoint candidate.
                    offer_rot(&mut rot_reps, m);
                    let dn = d / l;
                    let perp = Vec2::new(-dn.y, dn.x);
                    // 3-/6-fold equilateral apices (base·√3/6 from the base).
                    let h3 = l * (3.0f64).sqrt() / 6.0;
                    offer_rot(&mut rot_reps, m + perp * h3);
                    offer_rot(&mut rot_reps, m - perp * h3);
                    // 4-fold right-isosceles apices (base/2 from the base).
                    let h4 = l * 0.5;
                    offer_rot(&mut rot_reps, m + perp * h4);
                    offer_rot(&mut rot_reps, m - perp * h4);
                    // Mirror / glide lines per frame direction.
                    for (di, (u, glen)) in dirs.iter().enumerate() {
                        let du = d.dot(*u);
                        let n = Vec2::new(-u.y, u.x);
                        let s = m.dot(n);
                        if du.abs() <= tol {
                            // Separation ⊥ direction -> mirror candidate.
                            offer_line(&mut line_reps, di, s, n);
                        } else if (du - glen).abs() <= 1e-5 * max_edge
                            || (du + glen).abs() <= 1e-5 * max_edge
                        {
                            // Separation carries the half-period along the
                            // direction -> glide candidate.
                            offer_line(&mut line_reps, di, s, n);
                        }
                    }
                }
            }
        }
    }

    // 7. Rotate coset representatives -> certify (highest allowed fold
    //    first) -> emit all lattice translates inside the region.
    let mut rot_pts: Vec<Vec2> = rot_reps.values().map(|&(p, _)| p).collect();
    rot_pts.sort_by(|a, b| {
        let da = (*a - rc).length();
        let db = (*b - rc).length();
        da.partial_cmp(&db)
            .unwrap()
            .then_with(|| a.x.partial_cmp(&b.x).unwrap())
            .then_with(|| a.y.partial_cmp(&b.y).unwrap())
    });
    let mut rot_kept: Vec<Vec2> = Vec::new();
    for &p in &rot_pts {
        if rot_kept.iter().all(|&q| (p - q).length() > cluster_tol) {
            rot_kept.push(p);
        }
    }

    let mut centers: Vec<(Vec2, u8)> = Vec::new();
    for &c in &rot_kept {
        for &fold in folds {
            if rotation_certified(&v, c, fold, r, tol) {
                centers.push((c, fold));
                break;
            }
        }
    }

    let mut emitted: Vec<(Vec2, u8)> = Vec::new();
    for &(c, fold) in &centers {
        let (lo_m, hi_m, lo_n, hi_n) = lat_range(c, region, l1, l2, det);
        for m in lo_m..=hi_m {
            for n in lo_n..=hi_n {
                let q = c + l1 * (m as f64) + l2 * (n as f64);
                if region.contains_point(q) {
                    emitted.push((q, fold));
                }
            }
        }
    }
    emitted.sort_by(|a, b| {
        a.0.x.partial_cmp(&b.0.x)
            .unwrap()
            .then_with(|| a.0.y.partial_cmp(&b.0.y).unwrap())
            .then_with(|| a.1.cmp(&b.1))
    });
    let final_centers: Vec<RotationCenter> =
        emitted.into_iter().map(|(p, fold)| RotationCenter { point: p, fold }).collect();

    // 8. Line coset representatives -> certify (mirror first, glide only
    //    where the mirror fails) -> emit all translates crossing the region.
    let mut line_pts: Vec<(usize, f64)> =
        line_reps.iter().map(|(&(di, _), &(s, _))| (di, s)).collect();
    line_pts.sort_by(|a, b| {
        let na = Vec2::new(-dirs[a.0].0.y, dirs[a.0].0.x);
        let nb = Vec2::new(-dirs[b.0].0.y, dirs[b.0].0.x);
        let da = (a.1 - rc.dot(na)).abs();
        let db = (b.1 - rc.dot(nb)).abs();
        da.partial_cmp(&db)
            .unwrap()
            .then_with(|| a.0.cmp(&b.0))
            .then_with(|| a.1.partial_cmp(&b.1).unwrap())
    });
    let mut kept_lines: Vec<(usize, f64)> = Vec::new();
    for (di, s) in line_pts {
        if kept_lines
            .iter()
            .all(|&(d2, s2)| !(d2 == di && (s - s2).abs() <= cluster_tol))
        {
            kept_lines.push((di, s));
        }
    }

    let emit_lines = |s0: f64, di: usize| -> Vec<f64> {
        let (u, _) = dirs[di];
        let n = Vec2::new(-u.y, u.x);
        let (lo_m, hi_m, lo_n, hi_n) = lat_range(Vec2::ZERO, region, l1, l2, det);
        let mut off_by_key: HashMap<i64, f64> = HashMap::new();
        for m in lo_m..=hi_m {
            for n2 in lo_n..=hi_n {
                let off = (l1 * (m as f64) + l2 * (n2 as f64)).dot(n);
                if off.abs() <= cluster_tol {
                    continue; // lattice vector parallel to the axis = same line
                }
                let key = (off / cluster_tol).round() as i64;
                off_by_key.entry(key).or_insert(off);
            }
        }
        let mut sl: Vec<f64> = off_by_key.values().map(|&off| s0 + off).collect();
        sl.push(s0);
        sl.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sl.dedup_by(|a, b| (*a - *b).abs() <= 1e-12);
        sl.retain(|&s2| line_segment_length(s2, n, u, region) > 1e-9 * max_edge);
        sl
    };

    let global_dedup = |lines: Vec<(usize, f64)>| -> Vec<(usize, f64)> {
        let mut per_di: Vec<Vec<f64>> = vec![Vec::new(); dirs.len()];
        for (di, s) in lines {
            per_di[di].push(s);
        }
        let mut out: Vec<(usize, f64)> = Vec::new();
        for (di, ss) in per_di.iter().enumerate() {
            let (u, _) = dirs[di];
            let n = Vec2::new(-u.y, u.x);
            let sc = rc.dot(n);
            let mut ss: Vec<f64> = ss.clone();
            ss.sort_by(|a, b| {
                let da = (a - sc).abs();
                let db = (b - sc).abs();
                da.partial_cmp(&db)
                    .unwrap()
                    .then_with(|| a.partial_cmp(b).unwrap())
            });
            let mut kept: Vec<f64> = Vec::new();
            for &s in &ss {
                if kept.iter().all(|&q| (s - q).abs() > cluster_tol) {
                    kept.push(s);
                }
            }
            for &s in &kept {
                out.push((di, s));
            }
        }
        out
    };

    let mut mirrors_raw: Vec<(usize, f64)> = Vec::new();
    let mut glides_raw: Vec<(usize, f64)> = Vec::new();
    for &(di, s) in &kept_lines {
        let (u, glen) = dirs[di];
        let n = Vec2::new(-u.y, u.x);
        if mirrors_ok && line_certified(&v, s, n, u, None, r, tol) {
            mirrors_raw.extend(emit_lines(s, di).into_iter().map(|s2| (di, s2)));
        } else if glides_ok {
            let g = u * glen;
            if line_certified(&v, s, n, u, Some(g), r, tol) {
                glides_raw.extend(emit_lines(s, di).into_iter().map(|s2| (di, s2)));
            } else if line_certified(&v, s, n, u, Some(-g), r, tol) {
                glides_raw.extend(emit_lines(s, di).into_iter().map(|s2| (di, s2)));
            }
        }
    }

    let mut mirrors_dedup = global_dedup(mirrors_raw);
    let mut glides_dedup = global_dedup(glides_raw);

    let sort_by_dir_s = |xs: &mut Vec<(usize, f64)>| {
        xs.sort_by(|a, b| {
            let (ua, _) = dirs[a.0];
            let (ub, _) = dirs[b.0];
            ua.x.partial_cmp(&ub.x)
                .unwrap()
                .then_with(|| ua.y.partial_cmp(&ub.y).unwrap())
                .then_with(|| a.1.partial_cmp(&b.1).unwrap())
        });
    };
    sort_by_dir_s(&mut mirrors_dedup);
    sort_by_dir_s(&mut glides_dedup);

    let final_mirrors: Vec<MirrorLine> = mirrors_dedup
        .into_iter()
        .map(|(di, s)| {
            let (u, _) = dirs[di];
            let n = Vec2::new(-u.y, u.x);
            MirrorLine { point: n * s, direction: u }
        })
        .collect();
    let final_glides: Vec<GlideAxis> = glides_dedup
        .into_iter()
        .map(|(di, s)| {
            let (u, glen) = dirs[di];
            let n = Vec2::new(-u.y, u.x);
            GlideAxis { point: n * s, direction: u, glide: u * glen }
        })
        .collect();

    // 9. Unit-cell anchor: highest-fold certified site nearest the region
    //    center; else the nearest lattice point (the p1 / no-site case).
    let anchor = final_centers
        .iter()
        .min_by(|a, b| {
            b.fold
                .cmp(&a.fold)
                .then_with(|| {
                    (a.point - rc).length().partial_cmp(&(b.point - rc).length()).unwrap()
                })
                .then_with(|| a.point.x.partial_cmp(&b.point.x).unwrap())
                .then_with(|| a.point.y.partial_cmp(&b.point.y).unwrap())
        })
        .map(|c| c.point)
        .unwrap_or_else(|| nearest_lattice_point(rc, l1, l2, det));

    SymmetryOverlay {
        group,
        cell: cell_for_anchor(anchor),
        centers: final_centers,
        mirrors: final_mirrors,
        glides: final_glides,
        scale: max_edge,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Polygon;

    fn tile(pts: &[(f64, f64)]) -> Tile {
        Tile {
            proto: 0,
            poly: Polygon::from_slice(
                &pts.iter().map(|&(x, y)| Vec2::new(x, y)).collect::<Vec<Vec2>>(),
            ),
            seed: 0,
            level: 0,
        }
    }

    fn four_squares() -> Vec<Tile> {
        let mut v = Vec::new();
        for (x, y) in [(-1.0, -1.0), (0.0, -1.0), (-1.0, 0.0), (0.0, 0.0)] {
            v.push(tile(&[
                (x, y),
                (x + 1.0, y),
                (x + 1.0, y + 1.0),
                (x, y + 1.0),
            ]));
        }
        v
    }

    fn pg_patch() -> Vec<Tile> {
        let mut v = Vec::new();
        for a in -3..=3 {
            for b in -1..=1 {
                let x = a as f64;
                let y = 2.0 * b as f64;
                // A: fat_strip0(x, y)
                v.push(tile(&[
                    (x, y),
                    (x + 0.5, y),
                    (x + 0.7, y + 0.4),
                    (x + 0.5, y + 1.0),
                    (x, y + 1.0),
                    (x - 0.25, y + 0.5),
                ]));
                // T: fat_strip1(x, y)
                v.push(tile(&[
                    (x - 0.5, y + 2.0),
                    (x - 0.75, y + 1.5),
                    (x - 0.5, y + 1.0),
                    (x, y + 1.0),
                    (x + 0.2, y + 1.6),
                    (x, y + 2.0),
                ]));
                // B: waist_strip0(x, y)
                v.push(tile(&[
                    (x + 0.5, y),
                    (x + 1.0, y),
                    (x + 0.75, y + 0.5),
                    (x + 1.0, y + 1.0),
                    (x + 0.5, y + 1.0),
                    (x + 0.7, y + 0.4),
                ]));
                // U: waist_strip1(x, y)
                v.push(tile(&[
                    (x, y + 2.0),
                    (x + 0.2, y + 1.6),
                    (x, y + 1.0),
                    (x + 0.5, y + 1.0),
                    (x + 0.25, y + 1.5),
                    (x + 0.5, y + 2.0),
                ]));
            }
        }
        v
    }

    fn near(a: Vec2, b: Vec2, eps: f64) -> bool {
        (a - b).length() <= eps
    }

    #[test]
    fn p4m_four_squares_exact() {
        let tiles = four_squares();
        let l = [Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)];
        let region = BBox::new(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0));
        let o = derive_overlay(&tiles, WallpaperGroup::P4m, l, region);

        assert_eq!(o.cell, [
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ]);

        let r4: Vec<Vec2> = o.centers.iter().filter(|c| c.fold == 4).map(|c| c.point).collect();
        let r2: Vec<Vec2> = o.centers.iter().filter(|c| c.fold == 2).map(|c| c.point).collect();
        assert_eq!(r4.len(), 13, "rot-4 count");
        assert_eq!(r2.len(), 12, "rot-2 count");
        let expect_r4: Vec<Vec2> = [
            (-1.0, -1.0), (-1.0, 0.0), (-1.0, 1.0), (-0.5, -0.5), (-0.5, 0.5),
            (0.0, -1.0), (0.0, 0.0), (0.0, 1.0), (0.5, -0.5), (0.5, 0.5),
            (1.0, -1.0), (1.0, 0.0), (1.0, 1.0),
        ]
        .iter()
        .map(|(x, y)| Vec2::new(*x, *y))
        .collect();
        for e in &expect_r4 {
            assert!(r4.iter().any(|&p| near(p, *e, 1e-9)), "missing rot-4 {e:?}");
        }
        let expect_r2: Vec<Vec2> = [
            (-1.0, -0.5), (-1.0, 0.5), (-0.5, -1.0), (-0.5, 0.0), (-0.5, 1.0),
            (0.0, -0.5), (0.0, 0.5), (0.5, -1.0), (0.5, 0.0), (0.5, 1.0),
            (1.0, -0.5), (1.0, 0.5),
        ]
        .iter()
        .map(|(x, y)| Vec2::new(*x, *y))
        .collect();
        for e in &expect_r2 {
            assert!(r2.iter().any(|&p| near(p, *e, 1e-9)), "missing rot-2 {e:?}");
        }

        assert_eq!(o.glides.len(), 0, "p4m has no glides");
        assert_eq!(o.mirrors.len(), 12, "p4m mirror count");
        // Each mirror: point = n*s, n = (-dir.y, dir.x). Check the 12 lines.
        // s values are world-unit signed distances (diagonals are c/√2).
        let sq2 = std::f64::consts::SQRT_2;
        let expect: Vec<(Vec2, f64)> = [
            // horizontal y = -1,0,1 : dir (1,0), n (0,1), point (0, y)
            (Vec2::new(1.0, 0.0), -1.0),
            (Vec2::new(1.0, 0.0), 0.0),
            (Vec2::new(1.0, 0.0), 1.0),
            // vertical x = -1,0,1 : dir (0,1), n (-1,0), point (x, 0)
            (Vec2::new(0.0, 1.0), -1.0),
            (Vec2::new(0.0, 1.0), 0.0),
            (Vec2::new(0.0, 1.0), 1.0),
            // diag y = x + c : dir (1/sqrt2,1/sqrt2), n (-1/sqrt2,1/sqrt2); s = c/sqrt2
            (Vec2::new(1.0 / sq2, 1.0 / sq2), -1.0 / sq2),
            (Vec2::new(1.0 / sq2, 1.0 / sq2), 0.0),
            (Vec2::new(1.0 / sq2, 1.0 / sq2), 1.0 / sq2),
            // anti-diag x + y = c : dir (1/sqrt2,-1/sqrt2), n (1/sqrt2,1/sqrt2); s = c/sqrt2
            (Vec2::new(1.0 / sq2, -1.0 / sq2), -1.0 / sq2),
            (Vec2::new(1.0 / sq2, -1.0 / sq2), 0.0),
            (Vec2::new(1.0 / sq2, -1.0 / sq2), 1.0 / sq2),
        ]
        .into_iter()
        .collect();
        for (d, s) in &expect {
            let n = Vec2::new(-d.y, d.x);
            let p = n * *s;
            let found = o.mirrors.iter().any(|ml| {
                (ml.direction - *d).length() <= 1e-6 && near(ml.point, p, 1e-6)
            });
            assert!(found, "missing mirror dir {d:?} s {s:?} (point {p:?})");
        }
    }

    #[test]
    fn p4_four_squares_group_gate() {
        let tiles = four_squares();
        let l = [Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)];
        let region = BBox::new(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0));
        let o = derive_overlay(&tiles, WallpaperGroup::P4, l, region);
        let r4 = o.centers.iter().filter(|c| c.fold == 4).count();
        let r2 = o.centers.iter().filter(|c| c.fold == 2).count();
        assert_eq!(r4, 13);
        assert_eq!(r2, 12);
        assert_eq!(o.mirrors.len(), 0, "p4 group gate: no mirrors");
        assert_eq!(o.glides.len(), 0, "p4 group gate: no glides");
        assert_eq!(o.cell, [
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ]);
    }

    #[test]
    fn p1_four_squares_cell_only() {
        let tiles = four_squares();
        let l = [Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)];
        let region = BBox::new(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0));
        let o = derive_overlay(&tiles, WallpaperGroup::P1, l, region);
        assert!(o.centers.is_empty());
        assert!(o.mirrors.is_empty());
        assert!(o.glides.is_empty());
        assert_eq!(o.cell, [
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ]);
    }

    #[test]
    fn pg_glide_hexagon_strips_exact() {
        let tiles = pg_patch();
        let l = [Vec2::new(1.0, 0.0), Vec2::new(0.0, 2.0)];
        let region = BBox::new(Vec2::new(-3.0, -2.0), Vec2::new(3.0, 2.0));
        let o = derive_overlay(&tiles, WallpaperGroup::Pg, l, region);
        assert!(o.centers.is_empty(), "pg: no rotation sites");
        assert!(o.mirrors.is_empty(), "pg: no mirrors");
        assert_eq!(o.glides.len(), 5, "pg: five glide axes, y = -2..2");
        for g in &o.glides {
            assert!(near(g.direction, Vec2::new(1.0, 0.0), 1e-9), "glide dir {g:?}");
        }
        let ys: Vec<f64> = o
            .glides
            .iter()
            .map(|g| g.point.y) // dir (1,0) -> n (0,1) -> point (0, y)
            .collect();
        let expect: Vec<f64> = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
        for e in &expect {
            assert!(ys.iter().any(|&y| (y - e).abs() <= 1e-9), "missing glide y={e}");
        }
        assert_eq!(o.cell, [
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(0.0, 2.0),
        ]);
    }

    #[test]
    fn empty_tiles_cell_only() {
        let l = [Vec2::new(3.0, 1.0), Vec2::new(0.0, 2.0)];
        let region = BBox::new(Vec2::new(-5.0, -5.0), Vec2::new(5.0, 5.0));
        let o = derive_overlay(&[], WallpaperGroup::P1, l, region);
        assert!(o.centers.is_empty());
        assert!(o.mirrors.is_empty());
        assert!(o.glides.is_empty());
        // anchor = nearest lattice point to origin = (0,0)
        assert_eq!(o.cell[0], Vec2::new(0.0, 0.0));
    }

    #[test]
    fn degenerate_lattice_collapses_cell() {
        let tiles = four_squares();
        let l = [Vec2::new(1.0, 0.0), Vec2::new(2.0, 0.0)]; // det = 0
        let region = BBox::new(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0));
        let o = derive_overlay(&tiles, WallpaperGroup::P4, l, region);
        assert!(o.centers.is_empty());
        assert!(o.mirrors.is_empty());
        assert!(o.glides.is_empty());
        assert_eq!(o.cell, [
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 0.0),
        ]);
    }
}
