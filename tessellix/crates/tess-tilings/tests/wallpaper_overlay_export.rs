//! Focus: the certified wallpaper-symmetry overlay (C5,
//! `tess_core::symmetry::overlay::derive_overlay`) driven through the
//! registry in the exact view the CLI export renders by default
//! (1200×800, scale 1, rotation 0 → a 30×20 world region centered at the
//! origin).
//!
//! # Certification model the verifier enforces
//!
//! `derive_overlay` works by **coset reduction**: a site's lattice coset
//! holds infinitely many copies, one representative (the one nearest the
//! region center — always interior) is *certified* against the patch's
//! vertex set, and every lattice translate of that representative falling
//! inside the region is drawn. A copy at the patch's jagged edge is a
//! lattice translate of an interior, certified site even when its own
//! local context is incomplete (a finite patch cannot certify beyond its
//! own edge — the overlay's documented boundary-skip).
//!
//! So every drawn glyph must **trace to a certified site**: the verifier
//! re-derives, independently (no hashing, no coset machinery, no module
//! internals), that the drawn site's coset contains a translate that
//! *naively* certifies on the patch's vertex set (full linear-scan
//! rotation / reflection membership with the same geometric rule: local
//! radius `R`, boundary-skip on the vertex aabb, ≥ [`MIN_TESTED`] tested,
//! non-collinear star / both-sides guard). A perturbed copy of that
//! witnessing site must fail (mutation spot-check), and a drawn glide
//! axis must NOT be a pure mirror (the documented mirror/glide
//! convention).
//!
//! # Cases
//!
//! Seven registry tilings get the full per-site re-verification
//! (p4, p6, p3, p31m, p1, pg, p6m — covering one representative of each
//! required operation type); the large cmm engine
//! (hankin-square-two-point) is invariants-only, because a per-site
//! re-verification at its vertex count would be O(V²·sites), and its
//! diagonal mirrors are suppressed by the boundary-skip on this
//! particular patch (a known, conservative completeness limitation —
//! the mirrors are genuine but a few boundary vertices cannot be
//! imaged; see the module doc).
use tess_core::geometry::{BBox, Vec2};
use tess_core::params::{ParamSet, ParamValue};
use tess_core::symmetry::overlay::{derive_overlay, MIN_TESTED, SymmetryOverlay};
use tess_core::symmetry::WallpaperGroup;
use tess_core::tiling::{Query, Tile};

/// The default export view (what `tess-tui export --tiling ID` renders
/// with no `--width`/`--height`/`--scale`/`--rotation`).
fn region() -> BBox {
    BBox::new(Vec2::new(-15.0, -10.0), Vec2::new(15.0, 10.0))
}

struct View {
    tiles: Vec<Tile>,
    periodic: [Vec2; 2],
    group: WallpaperGroup,
}

/// The tiles, period vectors and declared group of one tiling's default
/// export view.
fn view_of(id: &str) -> View {
    let registry = tess_tilings::registry();
    let tiling = registry
        .get(id)
        .unwrap_or_else(|| panic!("{id} registered"));
    let meta = tiling
        .declared_symmetry()
        .unwrap_or_else(|| panic!("{id} declares symmetry"));
    let mut params = ParamSet::default_for(tiling.params());
    params.set(tess_tilings::lattice::SCALE_ID, ParamValue::Float(1.0));
    params.set(
        tess_tilings::lattice::ROTATION_ID,
        ParamValue::Angle(0.0),
    );
    let result = tiling.generate(&params, &Query { region: region() });
    assert!(
        result.message.is_none(),
        "{id} refused the default view: {:?}",
        result.message
    );
    let periodic =
        result.periodic.unwrap_or_else(|| panic!("{id} is periodic in this test"));
    View {
        tiles: result.tiles,
        periodic,
        group: meta.group,
    }
}

/// The overlay's geometry parameters, restated here independently:
/// `(tol, R, max_edge)` — must match `derive_overlay`'s documented rule.
fn params(view: &View) -> (f64, f64, f64) {
    let [l1, l2] = view.periodic;
    let max_edge = l1.length().max(l2.length()).max(1.0);
    (1e-6 * max_edge, 1.5 * max_edge, max_edge)
}

/// Lattice coordinates `(m, n)` of `p` in the basis `⟨l1, l2⟩`.
fn lat_coords(p: Vec2, l1: Vec2, l2: Vec2, det: f64) -> (f64, f64) {
    (p.cross(l2) / det, l1.cross(p) / det)
}

// ──────────────────────────────────────────────────────────────────────────
// Independent naive verifier.
//
// Deliberately NOT the overlay module's machinery: no spatial hashes, no
// coset maps, no representative reuse — the patch's vertex set is built by
// linear-scan dedup, membership is a linear distance scan, and each
// witnessing site is checked literally against the rendered vertices.
// ──────────────────────────────────────────────────────────────────────────

/// The globally deduped patch vertex set, plus its padded bounding box
/// (the boundary-skip window).
struct VSet {
    vs: Vec<Vec2>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
}

impl VSet {
    fn new(vs: &[Vec2], tol: f64) -> Self {
        let (mut x0, mut x1) = (f64::INFINITY, f64::NEG_INFINITY);
        let (mut y0, mut y1) = (f64::INFINITY, f64::NEG_INFINITY);
        for &p in vs {
            x0 = x0.min(p.x);
            x1 = x1.max(p.x);
            y0 = y0.min(p.y);
            y1 = y1.max(p.y);
        }
        VSet {
            vs: vs.to_vec(),
            x0: x0 - tol,
            x1: x1 + tol,
            y0: y0 - tol,
            y1: y1 + tol,
        }
    }

    #[inline]
    fn in_aabb(&self, p: Vec2) -> bool {
        self.x0 <= p.x && p.x <= self.x1 && self.y0 <= p.y && p.y <= self.y1
    }

    /// Linear-scan membership (stricter than the module's 3×3 hash
    /// neighbourhood): the witness images land on exact vertices, so a
    /// plain distance test suffices.
    #[inline]
    fn has(&self, p: Vec2, tol: f64) -> bool {
        self.vs
            .iter()
            .any(|q| ((q.x - p.x) * (q.x - p.x) + (q.y - p.y) * (q.y - p.y)).sqrt() <= tol)
    }
}

/// The patch's globally deduped vertex set (linear dedup at `tol`).
fn patch_vertices(tiles: &[Tile], tol: f64) -> Vec<Vec2> {
    let mut vs: Vec<Vec2> = Vec::new();
    for t in tiles {
        for &p in &t.poly.pts {
            let close = vs.iter().any(|q| {
                ((q.x - p.x) * (q.x - p.x) + (q.y - p.y) * (q.y - p.y)).sqrt() <= tol
            });
            if !close {
                vs.push(p);
            }
        }
    }
    vs
}

/// Does a fold-`n` rotation about `c` certify on the patch vertex set:
/// every nearby vertex's `2π/n`-image is a patch vertex (boundary-skip on
/// the padded aabb), ≥ [`MIN_TESTED`] tested, and a non-collinear local
/// star.
fn v_rot(v: &VSet, c: Vec2, fold: u8, r: f64, tol: f64) -> bool {
    let theta = std::f64::consts::TAU / fold as f64;
    let (ct, st) = (theta.cos(), theta.sin());
    let mut tested = 0usize;
    let mut rel: Vec<(f64, f64)> = Vec::new();
    for &p in &v.vs {
        let (dx, dy) = (p.x - c.x, p.y - c.y);
        if (dx * dx + dy * dy).sqrt() > r {
            continue;
        }
        rel.push((dx, dy));
        let w = Vec2::new(c.x + dx * ct - dy * st, c.y + dx * st + dy * ct);
        if !v.in_aabb(w) {
            continue; // boundary-skip
        }
        tested += 1;
        if !v.has(w, tol) {
            return false;
        }
    }
    if tested < MIN_TESTED {
        return false;
    }
    let thr = tol * r;
    for i in 0..rel.len() {
        for j in i + 1..rel.len() {
            if (rel[i].0 * rel[j].1 - rel[i].1 * rel[j].0).abs() > thr {
                return true;
            }
        }
    }
    false
}

/// Reflect `p` in the line through `m` with unit direction `u`.
fn v_reflect(p: Vec2, m: Vec2, u: Vec2) -> Vec2 {
    let (wx, wy) = (p.x - m.x, p.y - m.y);
    let a = wx * u.x + wy * u.y;
    Vec2::new(m.x + u.x * (2.0 * a) - wx, m.y + u.y * (2.0 * a) - wy)
}

/// Does the line (a point `m` on it, unit direction `u`, optional glide
/// translation `g` along the axis) certify: every nearby vertex's
/// (glide-)reflection is a patch vertex (boundary-skip), ≥ [`MIN_TESTED`]
/// tested, with vertices on both sides.
fn v_line(v: &VSet, m: Vec2, u: Vec2, g: Option<Vec2>, r: f64, tol: f64) -> bool {
    let n = Vec2::new(-u.y, u.x);
    let mut tested = 0usize;
    let mut pos = 0usize;
    let mut neg = 0usize;
    for &p in &v.vs {
        let d = (p.x - m.x) * n.x + (p.y - m.y) * n.y;
        if d.abs() > r {
            continue;
        }
        if d > tol {
            pos += 1;
        } else if d < -tol {
            neg += 1;
        }
        let mut w = v_reflect(p, m, u);
        if let Some(gv) = g {
            w = Vec2::new(w.x + gv.x, w.y + gv.y);
        }
        if !v.in_aabb(w) {
            continue; // boundary-skip
        }
        tested += 1;
        if !v.has(w, tol) {
            return false;
        }
    }
    tested >= MIN_TESTED && pos >= 1 && neg >= 1
}

/// Coset-trace witness for a drawn rotation center: the lattice translate
/// of `p` nearest the region center that naively certifies at `fold`.
/// Returns the witnessing point (so the mutation check can perturb it).
fn center_witness(
    view: &View,
    v: &VSet,
    p: Vec2,
    fold: u8,
    rc: Vec2,
) -> Option<Vec2> {
    let (tol, r, _me) = params(view);
    let [l1, l2] = view.periodic;
    let det = l1.cross(l2);
    let (m0, n0) = lat_coords(rc - p, l1, l2, det);
    let (m0, n0) = (m0.round() as i64, n0.round() as i64);
    for dm in -1..=1i64 {
        for dn in -1..=1i64 {
            let q = p + l1 * ((m0 + dm) as f64) + l2 * ((n0 + dn) as f64);
            if v_rot(v, q, fold, r, tol) {
                return Some(q);
            }
        }
    }
    None
}

/// Coset-trace witness for a drawn line (offset `s`, unit normal `n`,
/// direction `u`): the coset offset nearest the region center whose line
/// naively certifies. Returns the witnessing point on the line.
fn line_witness(
    view: &View,
    v: &VSet,
    s: f64,
    n: Vec2,
    u: Vec2,
    g: Option<Vec2>,
    rc: Vec2,
) -> Option<Vec2> {
    let (tol, r, _me) = params(view);
    let [l1, l2] = view.periodic;
    let target = rc.dot(n);
    // Candidate coset offsets `s + (m·l1+n·l2)·n`, ordered by how close
    // their offset is to the region center's offset.
    let mut cand: Vec<(f64, f64)> = Vec::new();
    for m in -20..=20i64 {
        for n2 in -20..=20i64 {
            let off = (l1 * (m as f64) + l2 * (n2 as f64)).dot(n);
            let s2 = s + off;
            cand.push(((s2 - target).abs(), s2));
        }
    }
    cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (_d, s2) in cand {
        let m_pt = n * s2;
        if v_line(v, m_pt, u, g, r, tol) {
            return Some(m_pt);
        }
    }
    None
}

/// Re-verify every drawn site of the overlay by coset-trace: each drawn
/// center's coset contains a naively-certified translate, each drawn
/// mirror's and glide's does too, a perturbed witness fails, and a drawn
/// glide is not a pure mirror.
fn certify_all(view: &View, v: &VSet, ov: &SymmetryOverlay) {
    let rc = region().center();
    let (_tol, _r, me) = params(view);

    for c in &ov.centers {
        let w = center_witness(view, v, c.point, c.fold, rc).unwrap_or_else(|| {
            panic!(
                "fold-{} center ({:.4},{:.4}) has no certified coset translate",
                c.fold, c.point.x, c.point.y
            )
        });
        // Mutation spot-check: a perturbed witness must stop certifying.
        let d = 0.01 * me;
        let w2 = Vec2::new(w.x + d, w.y);
        let (_t, r, _) = params(view);
        assert!(
            !v_rot(v, w2, c.fold, r, _t),
            "mutation: fold-{} witness ({:.4},{:.4}) shifted 0.01·max_edge must fail",
            c.fold,
            w.x,
            w.y
        );
    }

    for m in &ov.mirrors {
        let n = Vec2::new(-m.direction.y, m.direction.x);
        let s = m.point.dot(n);
        let w = line_witness(view, v, s, n, m.direction, None, rc).unwrap_or_else(|| {
            panic!(
                "mirror offset {s:.4} (dir ({:.4},{:.4})) has no certified coset line",
                m.direction.x, m.direction.y
            )
        });
        let (_t, r, _) = params(view);
        let d = 0.01 * me;
        let w2 = Vec2::new(w.x + n.x * d, w.y + n.y * d);
        assert!(
            !v_line(v, w2, m.direction, None, r, _t),
            "mutation: mirror witness offset shifted 0.01·max_edge must fail"
        );
    }

    for g in &ov.glides {
        let n = Vec2::new(-g.direction.y, g.direction.x);
        let s = g.point.dot(n);
        let neg = Vec2::new(-g.glide.x, -g.glide.y);
        let w = line_witness(view, v, s, n, g.direction, Some(g.glide), rc)
            .or_else(|| line_witness(view, v, s, n, g.direction, Some(neg), rc))
            .unwrap_or_else(|| {
                panic!(
                    "glide offset {s:.4} (dir ({:.4},{:.4})) has no certified coset axis",
                    g.direction.x, g.direction.y
                )
            });
        // Documented mirror/glide convention: a drawn glide axis is NOT a
        // pure mirror.
        let (_t, r, _) = params(view);
        assert!(
            !v_line(v, w, g.direction, None, r, _t),
            "pure mirror at glide witness ({:.4},{:.4}) dir ({:.4},{:.4}) must fail",
            w.x,
            w.y,
            g.direction.x,
            g.direction.y
        );
        // Mutation spot-check on the glide witness.
        let d = 0.01 * me;
        let w2 = Vec2::new(w.x + n.x * d, w.y + n.y * d);
        assert!(
            !v_line(v, w2, g.direction, Some(g.glide), r, _t)
                && !v_line(v, w2, g.direction, Some(neg), r, _t),
            "mutation: glide witness offset shifted 0.01·max_edge must fail"
        );
    }
}

/// Structural invariants: the cell carries the declared period vectors as
/// its edges, and no two drawn sites coincide.
fn invariants(view: &View, ov: &SymmetryOverlay) {
    let (tol, _r, me) = params(view);
    let [l1, l2] = view.periodic;
    assert!(
        (ov.cell[1] - ov.cell[0] - l1).length() <= tol,
        "cell edge c→c+l1 must be the declared period vector l1"
    );
    assert!(
        (ov.cell[3] - ov.cell[0] - l2).length() <= tol,
        "cell edge c→c+l2 must be the declared period vector l2"
    );
    for i in 0..ov.centers.len() {
        for j in i + 1..ov.centers.len() {
            assert!(
                (ov.centers[i].point - ov.centers[j].point).length() > tol,
                "two drawn centers within tol: ({:.4},{:.4}) vs ({:.4},{:.4})",
                ov.centers[i].point.x,
                ov.centers[i].point.y,
                ov.centers[j].point.x,
                ov.centers[j].point.y
            );
        }
    }
    // No two drawn lines (mirror or glide) are the same line: same
    // undirected direction and offset within the cluster tolerance.
    let lines: Vec<(Vec2, Vec2)> = ov
        .mirrors
        .iter()
        .map(|m| (m.point, m.direction))
        .chain(ov.glides.iter().map(|g| (g.point, g.direction)))
        .collect();
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            let (pa, ua) = lines[i];
            let (pb, ub) = lines[j];
            if (ua.x * ub.x + ua.y * ub.y).abs() > 1.0 - 1e-9 {
                let n = Vec2::new(-ua.y, ua.x);
                let sa = pa.x * n.x + pa.y * n.y;
                let sb = pb.x * n.x + pb.y * n.y;
                assert!(
                    (sa - sb).abs() > 1e-4 * me,
                    "two drawn lines coincide (same direction, offset within cluster tol): {sa:.6} vs {sb:.6}"
                );
            }
        }
    }
}

/// SVG side: the rendered overlay group carries exactly one glyph per
/// drawn site (the overlay already drops zero-length lines, so every
/// emitted line crosses the region and survives the SVG clip), and the
/// on-state is the off-state bytes plus the overlay block.
fn svg_counts(view: &View, ov: &SymmetryOverlay) {
    let off = tess_core::svg::render_svg_with_walls_overlay(
        &view.tiles,
        &region(),
        tess_core::view::ColorMode::Pattern,
        &[],
        false,
        false,
        None,
    );
    let on = tess_core::svg::render_svg_with_walls_overlay(
        &view.tiles,
        &region(),
        tess_core::view::ColorMode::Pattern,
        &[],
        false,
        false,
        Some(ov),
    );
    let tail = "</g></svg>";
    let marker = r#"<g data-symmetry-overlay="true""#;
    let idx = on.find(marker).expect("overlay group missing from the on-state");
    assert_eq!(&on[..idx], &off[..off.len() - tail.len()], "on minus overlay == off");
    assert_eq!(
        on.matches(r#"<g data-symmetry-overlay="true""#).count(),
        1,
        "one overlay group"
    );
    assert_eq!(on.matches(r#"data-sym="cell""#).count(), 1, "one unit cell");
    for f in [2u8, 3, 4, 6] {
        let want = ov.centers.iter().filter(|c| c.fold == f).count();
        let got = on.matches(&format!(r#"data-sym="rot-{f}""#)).count();
        assert_eq!(got, want, "drawn rot-{f} glyphs must equal the overlay's fold-{f} centers");
    }
    assert_eq!(
        on.matches(r#"data-sym="mirror""#).count(),
        ov.mirrors.len(),
        "drawn mirror glyphs must equal the overlay's mirrors"
    );
    assert_eq!(
        on.matches(r#"data-sym="glide""#).count(),
        ov.glides.len(),
        "drawn glide glyphs must equal the overlay's glides"
    );
}

/// Certification battery: one representative per required operation type.
/// Every drawn glyph is re-verified by coset-trace against the patch, a
/// perturbed witness must fail, and the structural invariants hold.
#[test]
fn overlay_certification_battery() {
    // (id, group, min fold-2, min fold-3, min fold-4, min fold-6,
    //  min mirrors, min glides, centers zero, mirrors zero, glides zero)
    let cases = [
        ("windmill", WallpaperGroup::P4, 1, 0, 1, 0, 0, 0, false, true, true),
        ("snub-trihexagonal", WallpaperGroup::P6, 0, 0, 0, 1, 0, 0, false, true, true),
        ("chiral-cut-honeycomb", WallpaperGroup::P3, 0, 1, 0, 0, 0, 0, false, true, true),
        ("split-triangle-grid", WallpaperGroup::P31m, 0, 1, 0, 0, 1, 0, false, false, true),
        ("stepped-hexominoes", WallpaperGroup::P1, 0, 0, 0, 0, 0, 0, true, true, true),
        ("glide-hexagon-strips", WallpaperGroup::Pg, 0, 0, 0, 0, 0, 1, true, true, false),
        ("hexagon", WallpaperGroup::P6m, 0, 0, 0, 1, 1, 0, false, false, true),
    ];
    for (id, group, m2, m3, m4, m6, mm, mg, cz, mz, gz) in cases {
        let view = view_of(id);
        assert_eq!(view.group, group, "{id}: declared group");
        assert!(!view.tiles.is_empty(), "{id} produced tiles");
        let ov = derive_overlay(&view.tiles, view.group, view.periodic, region());

        let count = |f: u8| ov.centers.iter().filter(|c| c.fold == f).count();
        assert!(count(2) >= m2, "{id}: fold-2 centers ≥ {m2}");
        assert!(count(3) >= m3, "{id}: fold-3 centers ≥ {m3}");
        assert!(count(4) >= m4, "{id}: fold-4 centers ≥ {m4}");
        assert!(count(6) >= m6, "{id}: fold-6 centers ≥ {m6}");
        assert!(ov.mirrors.len() >= mm, "{id}: mirrors ≥ {mm}");
        assert!(ov.glides.len() >= mg, "{id}: glides ≥ {mg}");
        if cz {
            assert!(ov.centers.is_empty(), "{id}: no rotation sites");
        }
        if mz {
            assert!(ov.mirrors.is_empty(), "{id}: no mirrors");
        }
        if gz {
            assert!(ov.glides.is_empty(), "{id}: no glides");
        }

        let (tol, _r, _me) = params(&view);
        let vs = patch_vertices(&view.tiles, tol);
        let vset = VSet::new(&vs, tol);
        certify_all(&view, &vset, &ov);
        invariants(&view, &ov);
        svg_counts(&view, &ov);

        // p1's anchor: with no rotational sites the cell anchor is a
        // lattice point of the world-origin frame (all registry tables
        // use offset zero).
        if group == WallpaperGroup::P1 {
            let [l1, l2] = view.periodic;
            let det = l1.cross(l2);
            let c = ov.cell[0];
            let (m, n) = (c.cross(l2) / det, l1.cross(c) / det);
            for &x in &[m, n] {
                let frac = x.fract().abs().min(1.0 - x.fract().abs());
                assert!(
                    frac < 1e-9,
                    "p1 cell anchor must be a lattice point: lat coords ({m:.6},{n:.6})"
                );
            }
        }
    }
}

/// Invariants-only battery for the large cmm engine (4506 tiles, 6389
/// unique vertices): the full per-site naive re-verification would be
/// O(V²·sites) at that vertex count, and this patch's diagonal mirror
/// families are suppressed by the boundary-skip (a conservative
/// completeness limitation — the mirrors are genuine, but a few boundary
/// vertices cannot be imaged), so this case pins only the structural
/// invariants and the SVG glyph census.
#[test]
fn overlay_invariants_battery() {
    let view = view_of("hankin-square-two-point");
    assert_eq!(view.group, WallpaperGroup::Cmm, "declared group");
    let ov = derive_overlay(&view.tiles, view.group, view.periodic, region());
    assert!(!ov.centers.is_empty(), "cmm carries rotation centers");
    invariants(&view, &ov);
    svg_counts(&view, &ov);
}
