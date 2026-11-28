//! §22e Stage-2 items 3+4 — the 22e motif layer on the two
//! quasiperiodic carriers (`ammann-beenker`, `penrose`).
//!
//! The emitters (diss. §3.10.1 placement, see the `motif22e` module
//! docs) put a monster medallion on **every tiling vertex**: the
//! `{16/8}` hexadecagon on the AB patch and the `{10/2}` decagon on
//! Penrose P3, inflated to apothem = substrate edge / 2 so medallions
//! meet at the tile-edge midpoints. These tests pin the wiring shape
//! (raw chord counts per vertex), the chord analyticity (lengths,
//! subtensions, contact multiplicities), the off-state byte identity
//! (the motif rows never fire under the plain weave), the fused wall
//! census at one classic angle per substrate, and the honest gate
//! truth (outline passes, interlace stays open NAMED — the odd-axis
//! AB diameters and the generic-angle ray ends float free, so no
//! classic δ closes this weave, exactly the M172 documented-blocker
//! shape).

use std::collections::{BTreeMap, BTreeSet, HashSet};

use tess_core::geometry::{BBox, Vec2};
use tess_core::params::ParamSet;
use tess_core::strapwork::{
    plan_decoration, plan_decoration_medallion, validity_stats, walls, walls_layered, Decoration,
    Wall,
};
use tess_core::svg::render_svg_with_walls_overlay;
use tess_core::tiling::{Query, Tile};
use tess_core::view::ColorMode;
use tess_tilings::medallion::motif22e;
use tess_tilings::medallion::weave;

/// sin 36° = √(10 − 2√5)/4 — the {10/2} chord / edge ratio.
const SIN36: f64 = 0.587_785_252_292_473_1;
/// (3 − √5)/2 = 1/φ² — the thin-rhomb short-diagonal² / edge² ratio.
const THIN_SHORT2: f64 = 0.381_966_011_250_105_15;

fn region() -> BBox {
    BBox::new(Vec2::new(-8.0, -8.0), Vec2::new(8.0, 8.0))
}

/// The Penrose generator refines by a global scale rule, so the patch
/// edge is region-dependent; the small window keeps the fused censuses
/// cheap. Edge here ≈ 0.6305 (region [-4, 4]²).
fn region_s() -> BBox {
    BBox::new(Vec2::new(-4.0, -4.0), Vec2::new(4.0, 4.0))
}

fn tiles_of(id: &str, r: BBox) -> Vec<Tile> {
    let reg = tess_tilings::registry();
    let t = reg.get(id).expect("carrier registered");
    let p = ParamSet::default_for(t.params());
    let res = t.generate(&p, &Query { region: r });
    assert!(res.message.is_none(), "{id}: {:?}", res.message);
    res.tiles
}

/// The uniform substrate edge, measured as in the emitter (max over
/// the 4-gon carriers' first edges).
fn substrate_edge(tiles: &[Tile]) -> f64 {
    let mut e = 0.0f64;
    for t in tiles {
        if t.poly.len() == 4 {
            let l = (t.poly.pts[1] - t.poly.pts[0]).length();
            if l > e {
                e = l;
            }
        }
    }
    e
}

fn key(p: Vec2) -> (i64, i64) {
    ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
}

fn vertices(tiles: &[Tile]) -> BTreeSet<(i64, i64)> {
    let mut v = BTreeSet::new();
    for t in tiles {
        for p in &t.poly.pts {
            v.insert(key(*p));
        }
    }
    v
}

/// Dedup raw chords on the 1e-6 endpoint grid (order-canonical),
/// mirroring the engine's [`SegKey`].
fn dedup(chords: &[Wall]) -> Vec<Wall> {
    let mut seen: HashSet<(u64, u64)> = HashSet::new();
    let mut out = Vec::new();
    for w in chords {
        let (ka, kb) = (key(w.a), key(w.b));
        let k = if ka <= kb { (ka, kb) } else { (kb, ka) };
        // Pack the two grid points into one hashable pair.
        let pk: (u64, u64) = (
            (k.0 .0 as u64) << 32 ^ (k.0 .1 as u64),
            (k.1 .0 as u64) << 32 ^ (k.1 .1 as u64),
        );
        if seen.insert(pk) {
            out.push(*w);
        }
    }
    out
}

// ---- table shape ----------------------------------------------------------

#[test]
fn table_shape_and_carriers() {
    let specs = weave::medallion_layer_specs();
    assert_eq!(specs.len(), 3, "M172 + two 22e rows");
    assert_eq!(weave::motif_carriers("22e"), vec!["ammann-beenker", "penrose"]);
    assert!(weave::motif_carriers("nope").is_empty());
    let ab = weave::motif_layer_spec("ammann-beenker", "22e").expect("AB row");
    let pen = weave::motif_layer_spec("penrose", "22e").expect("Penrose row");
    // Monster168 declares LandingIdentity, Monster102 BoundaryInference.
    assert_eq!(ab.record, "22e {16/8} hexadecagon monster (Ammann-Beenker)");
    assert_eq!(ab.absorption, "LandingIdentity");
    assert_eq!(ab.suppressed, &[], "the medallions are virtual carriers");
    assert_eq!(pen.record, "22e {10/2} decagon monster (Penrose)");
    assert_eq!(pen.absorption, "BoundaryInference");
    assert_eq!(pen.suppressed, &[]);
    // Ungated lookups never reach the motif rows.
    assert!(weave::motif_layer_spec("square", "22e").is_none());
    assert_eq!(weave::medallion_layer_specs()[0].motif, None);
}

// ---- off state ------------------------------------------------------------

#[test]
fn off_state_carriers_stay_on_the_plain_pipeline() {
    let reg = tess_tilings::registry();
    for (id, r) in [("ammann-beenker", region()), ("penrose", region_s())] {
        let t = reg.get(id).unwrap();
        let tiles = tiles_of(id, r);
        // No motif flag: fused_weave sees no row (the 22e rows are
        // motif-gated) and the wall set is the byte-identical plain
        // two-layer emission.
        assert!(weave::medallion_wiring(t, &tiles, &r).is_none(), "{id}: ungated wiring");
        let wv = weave::fused_weave(t, &tiles, &r, 45.0);
        assert!(wv.med.is_none(), "{id}: plain weave must not see the motif row");
        assert_eq!(wv.walls, walls(&tiles, 45.0), "{id}: off-state byte identity");
    }
}

#[test]
fn motif_refusals_are_named_by_none() {
    let reg = tess_tilings::registry();
    let r = region_s();
    // A tiling that carries nothing.
    let sq = reg.get("square").unwrap();
    let tiles = tiles_of("square", r);
    assert!(weave::fused_weave_motif(sq, &tiles, &r, 45.0, "22e").is_none());
    // A carrier asked for an unknown motif.
    let pen = reg.get("penrose").unwrap();
    assert!(weave::fused_weave_motif(pen, &tiles, &r, 45.0, "24x").is_none());
}

// ---- raw chord censuses (emitter, no fuse) --------------------------------

#[test]
fn ab_chord_census() {
    let tiles = tiles_of("ammann-beenker", region());
    let e = substrate_edge(&tiles);
    assert!((e - 1.0).abs() < 1e-12, "AB default patch is unit-edged, got {e}");
    let raw = motif22e::ab_extra_walls(&tiles);
    // 16 emitted pairs per polygon vertex (the 8 diameters twice).
    let pts: usize = tiles.iter().map(|t| t.poly.len()).sum();
    assert_eq!(raw.len(), 16 * pts);
    assert_eq!(raw.len(), 16 * 1344, "pinned region census ([-8,8]²)");
    // Every chord is an edge-length diameter centered on a vertex.
    let vs: Vec<Vec2> = tiles
        .iter()
        .flat_map(|t| t.poly.pts.iter().copied())
        .collect();
    for w in &raw {
        assert!(((w.b - w.a).length() - e).abs() < 1e-12, "diameter {}", (w.b - w.a).length());
        let m = (w.a + w.b) * 0.5;
        assert!(vs.iter().any(|v| (m - *v).length() < 1e-9), "diameter off-vertex");
    }
    // Deduped: exactly 8 diameters per vertex, and contact endpoints
    // are simple (single) or shared with exactly one other medallion
    // (a tile-edge midpoint — degree 2). No triple contacts exist:
    // any vertex difference equal to a half-diameter sum would put
    // two AB vertices closer than the edge.
    let uniq = dedup(&raw);
    let vs1 = vertices(&tiles);
    assert_eq!(uniq.len(), 8 * vs1.len(), "8 unique diameters per vertex");
    assert_eq!(vs1.len(), 365, "pinned vertex count ([-8,8]²)");
    let mut ends: BTreeMap<(i64, i64), usize> = BTreeMap::new();
    for w in &uniq {
        for p in [key(w.a), key(w.b)] {
            *ends.entry(p).or_default() += 1;
        }
    }
    assert!(ends.values().all(|&v| v == 1 || v == 2), "endpoint degrees {ends:?}");
}

#[test]
fn ab_rhomb_monster_pairs() {
    // The AB monster pair: the two 16-gons at the two 135° corners of
    // a 45° rhomb; their separation is the short diagonal, d² =
    // 2 − √2 (recon §6.1, the certified c = 3 lens).
    let tiles = tiles_of("ammann-beenker", region());
    let e2 = {
        let e = substrate_edge(&tiles);
        e * e
    };
    let mut rhombi = 0;
    for x in tiles.iter().filter(|x| x.seed == 1) {
        rhombi += 1;
        let p = &x.poly.pts;
        for i in 0..2 {
            let d = p[i + 2] - p[i];
            let d2 = d.dot(d);
            let want = if d2 < e2 { e2 * (2.0 - 2f64.sqrt()) } else { e2 * (2.0 + 2f64.sqrt()) };
            assert!((d2 - want).abs() < 1e-9, "rhomb diag² {d2} vs {want}");
        }
    }
    assert_eq!(rhombi, 192, "pinned AB rhombus count ([-8,8]²)");
}

#[test]
fn penrose_chord_census() {
    let tiles = tiles_of("penrose", region_s());
    let e = substrate_edge(&tiles);
    let raw = motif22e::penrose_extra_walls(&tiles);
    let pts: usize = tiles.iter().map(|t| t.poly.len()).sum();
    assert_eq!(raw.len(), 10 * pts);
    // Every {10/2} chord is e·sin 36° and subtends exactly 72° at its
    // medallion vertex.
    let vs: Vec<Vec2> = tiles
        .iter()
        .flat_map(|t| t.poly.pts.iter().copied())
        .collect();
    for w in &raw {
        let len = (w.b - w.a).length();
        assert!((len - e * SIN36).abs() < 1e-12, "chord length {len}");
        // The medallion vertex: equidistant (edge/2) from both ends.
        let v = vs
            .iter()
            .find(|v| ((w.a - **v).length() - e / 2.0).abs() < 1e-9
                && ((w.b - **v).length() - e / 2.0).abs() < 1e-9)
            .expect("chord vertex");
        let (u, t) = (w.a - *v, w.b - *v);
        let cos = u.dot(t) / (u.length() * t.length());
        assert!((cos - (5f64.sqrt() - 1.0) / 4.0).abs() < 1e-10, "chord subtends {cos}");
    }
    // Deduped: 10 chords per vertex; contact endpoints have degree 2
    // (a free contact: the medallion's own two pentagon sides) or 4
    // (a tile-edge midpoint: two contacts from each side medallion).
    let uniq = dedup(&raw);
    let vs1 = vertices(&tiles);
    assert_eq!(uniq.len(), 10 * vs1.len(), "10 unique chords per vertex");
    let mut ends: BTreeMap<(i64, i64), usize> = BTreeMap::new();
    for w in &uniq {
        for p in [key(w.a), key(w.b)] {
            *ends.entry(p).or_default() += 1;
        }
    }
    assert!(ends.values().all(|&v| v == 2 || v == 4), "endpoint degrees {:?}", ends.values().collect::<std::collections::BTreeSet<_>>());
}

#[test]
fn penrose_thin_rhomb_monster_pairs() {
    // The Penrose monster pair: decagons at the two 144° corners of a
    // thin rhomb, separation 1/φ on the mid-edge axis (recon §6.2:
    // the (10, 3) rung's axis). Thin rhombs are seed 1.
    let tiles = tiles_of("penrose", region_s());
    let e2 = {
        let e = substrate_edge(&tiles);
        e * e
    };
    let mut thin = 0;
    for x in tiles.iter().filter(|x| x.seed == 1 && x.poly.len() == 4) {
        thin += 1;
        let p = &x.poly.pts;
        for i in 0..2 {
            let d = p[i + 2] - p[i];
            let d2 = d.dot(d);
            let want = if d2 < e2 { e2 * THIN_SHORT2 } else { e2 * (4.0 - THIN_SHORT2) };
            assert!((d2 - want).abs() < 1e-9, "thin rhomb diag² {d2} vs {want}");
        }
    }
    assert!(thin > 0, "the small patch contains thin rhombs");
}

// ---- fused census + honest gate truth -------------------------------------

/// Every raw chord is exactly covered by the fused pieces of the
/// motif-only network (no gaps, no double coverage) — the M172 A3
/// idiom, run against `walls_layered(&[], δ, extra)` so the generic
/// emission cannot mask a dropped chord.
fn covered(ws: &[Wall], c: &Wall) -> bool {
    let d = c.b - c.a;
    let l = d.length();
    if l <= 0.0 {
        return true;
    }
    let u = d / l;
    let mut spans: Vec<(f64, f64)> = Vec::new();
    for w in ws {
        let wa = w.a - c.a;
        let wb = w.b - c.a;
        if wa.cross(u).abs() > 1e-9 || wb.cross(u).abs() > 1e-9 {
            continue;
        }
        let (mut s0, mut s1) = (wa.dot(u), wb.dot(u));
        if s0 > s1 {
            std::mem::swap(&mut s0, &mut s1);
        }
        let (a, b) = (s0.max(0.0), s1.min(l));
        if b > a {
            spans.push((a, b));
        }
    }
    spans.sort_by(|x, y| x.0.total_cmp(&y.0));
    let mut pos = 0.0f64;
    for (a, b) in spans {
        if a > pos + 1e-9 {
            return false;
        }
        if b > pos {
            pos = b;
        }
    }
    (pos - l).abs() < 1e-7
}

#[test]
fn ab_fused_at_delta_45() {
    let reg = tess_tilings::registry();
    let t = reg.get("ammann-beenker").unwrap();
    let r = region();
    let tiles = tiles_of("ammann-beenker", r);
    let wv = weave::fused_weave_motif(t, &tiles, &r, 45.0, "22e").expect("AB carries 22e");
    let med = wv.med.as_ref().expect("layer present");
    assert_eq!(med.record, "22e {16/8} hexadecagon monster (Ammann-Beenker)");
    assert_eq!(med.absorption, "LandingIdentity");
    assert_eq!(wv.walls.len(), 18_136, "fused wall count at δ=45");
    let only = walls_layered(&[], 45.0, &med.extra);
    assert_eq!(only.len(), 6800, "motif-only network at δ=45");
    for c in med.extra.iter().step_by(7) {
        assert!(covered(&only, c), "raw chord not covered by the motif pieces");
    }
    let s = validity_stats(&wv.walls, &r);
    assert_eq!((s.exit, s.corner, s.bad, s.joint), (2624, 0, 1256, 5025), "stats δ=45");
    // Honest gate truth: outline is ungated and passes; fill and the
    // interlace styles fail NAMED — the odd-axis {16/8} diameters end
    // free inside the patch, so no δ closes this weave (the M172
    // documented-blocker shape).
    let e = plan_decoration_medallion(&wv.walls, 0.3, Decoration::Fill, &r.padding(-1e-9), Some(&med.ctx()))
        .expect_err("the map is not Eulerian");
    assert!(e.to_string().contains("3880 odd-degree"), "fill count: {e}");
    plan_decoration(&wv.walls, 0.3, Decoration::Outline, &r.padding(-1e-9)).expect("outline ungated");
    for style in [Decoration::Interlace, Decoration::OutlineInterlace] {
        let e = plan_decoration_medallion(&wv.walls, 0.3, style, &r.padding(-1e-9), Some(&med.ctx()))
            .expect_err("the weave stays open at δ=45");
        assert!(e.to_string().contains("2624 open"), "open count {style:?}: {e}");
    }
    // The wall overlay reflects the fused count one <line> per wall.
    let svg = render_svg_with_walls_overlay(&tiles, &r, ColorMode::Pattern, &wv.walls, false, false, None);
    assert_eq!(svg.matches("<line ").count(), wv.walls.len(), "SVG line census");
}

#[test]
fn penrose_fused_at_delta_36() {
    let reg = tess_tilings::registry();
    let t = reg.get("penrose").unwrap();
    let r = region_s();
    let tiles = tiles_of("penrose", r);
    let wv = weave::fused_weave_motif(t, &tiles, &r, 36.0, "22e").expect("Penrose carries 22e");
    let med = wv.med.as_ref().expect("layer present");
    assert_eq!(med.record, "22e {10/2} decagon monster (Penrose)");
    assert_eq!(med.absorption, "BoundaryInference");
    assert_eq!(wv.walls.len(), 13_606, "fused wall count at δ=36");
    let only = walls_layered(&[], 36.0, &med.extra);
    assert_eq!(only.len(), 8078, "motif-only network at δ=36");
    for c in med.extra.iter() {
        assert!(covered(&only, c), "raw chord not covered by the motif pieces");
    }
    let s = validity_stats(&wv.walls, &r);
    assert_eq!((s.exit, s.corner, s.bad, s.joint), (163, 525, 550, 4182), "stats δ=36");
    let e = plan_decoration_medallion(&wv.walls, 0.3, Decoration::Fill, &r.padding(-1e-9), Some(&med.ctx()))
        .expect_err("the map is not Eulerian");
    assert!(e.to_string().contains("354 odd-degree"), "fill count: {e}");
    plan_decoration(&wv.walls, 0.3, Decoration::Outline, &r.padding(-1e-9)).expect("outline ungated");
    for style in [Decoration::Interlace, Decoration::OutlineInterlace] {
        let e = plan_decoration_medallion(&wv.walls, 0.3, style, &r.padding(-1e-9), Some(&med.ctx()))
            .expect_err("the weave stays open at δ=36");
        assert!(e.to_string().contains("688 open"), "open count {style:?}: {e}");
    }
    let svg = render_svg_with_walls_overlay(&tiles, &r, ColorMode::Pattern, &wv.walls, false, false, None);
    assert_eq!(svg.matches("<line ").count(), wv.walls.len(), "SVG line census");
}

/// Determinism (M172 A5, adapted): repeated calls give the identical
/// fused `Vec<Wall>` **bitwise**; tile-order permutations give the
/// identical map on the engine's own terms — equal wall counts, the
/// identical endpoint multiset on the engine's 1e-6 `SegKey` grid, and
/// pairwise drift under 1e-12 (observed ≤ 5e-15). Bitwise stability
/// under permutation is the engine's documented limitation, not the
/// emitter's: the raw chord multiset is bitwise permutation-invariant
/// (pinned below), and `fuse_layers` states that "generic-then-layer
/// insertion fixes the surviving endpoint values for grid-equal
/// duplicates" — so the surviving representative of a last-ulp pair
/// legitimately depends on input order.
#[test]
fn fused_output_is_deterministic() {
    let reg = tess_tilings::registry();
    let t = reg.get("penrose").unwrap();
    let r = region_s();
    let tiles = tiles_of("penrose", r);
    let a = weave::fused_weave_motif(t, &tiles, &r, 36.0, "22e").unwrap();
    let b = weave::fused_weave_motif(t, &tiles, &r, 36.0, "22e").unwrap();
    assert_eq!(a.walls, b.walls, "repeat call diverged");
    // The raw emitter is bitwise permutation-invariant as a multiset.
    let mut rev_tiles = tiles.clone();
    rev_tiles.reverse();
    assert_eq!(
        sorted_bitwise(&motif22e::penrose_extra_walls(&tiles)),
        sorted_bitwise(&motif22e::penrose_extra_walls(&rev_tiles)),
        "the raw chord multiset depends on tile order"
    );
    for (label, mut perm) in [
        ("reversal", {
            let mut v = tiles.clone();
            v.reverse();
            v
        }),
        ("rotation", {
            let mut v = tiles.clone();
            v.rotate_left(37);
            v
        }),
    ] {
        let c = weave::fused_weave_motif(t, &mut perm, &r, 36.0, "22e").unwrap();
        assert_eq!(a.walls.len(), c.walls.len(), "{label}: wall count changed");
        assert_eq!(
            sorted_grid(&a.walls),
            sorted_grid(&c.walls),
            "{label}: the fused map changed on the engine grid"
        );
        let mut worst = 0.0f64;
        for (x, y) in a.walls.iter().zip(&c.walls) {
            worst = worst.max((x.a - y.a).length().max((x.b - y.b).length()));
        }
        assert!(worst < 1e-12, "{label}: bitwise drift {worst:e} exceeds 1e-12");
    }
}

/// Chord multiset, canonicalized bitwise (endpoint-pair sorted).
fn sorted_bitwise(ws: &[Wall]) -> Vec<((u64, u64), (u64, u64))> {
    let k = |p: Vec2| (p.x.to_bits(), p.y.to_bits());
    let mut v: Vec<((u64, u64), (u64, u64))> = ws
        .iter()
        .map(|w| {
            let (ka, kb) = (k(w.a), k(w.b));
            if ka <= kb { (ka, kb) } else { (kb, ka) }
        })
        .collect();
    v.sort_unstable();
    v
}

/// Wall multiset on the engine's 1e-6 endpoint grid.
fn sorted_grid(ws: &[Wall]) -> Vec<((i64, i64), (i64, i64))> {
    let mut v: Vec<((i64, i64), (i64, i64))> = ws.iter().map(|w| (key(w.a), key(w.b))).collect();
    v.sort_unstable();
    v
}
