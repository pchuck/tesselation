//! M203 anchor A3 + A5 (design doc `docs/medallion-third-layer-design.md`
//! §7): the `dodecagon-hexagram-honeycomb` composition (M172) fused
//! with its medallion third layer.
//!
//! Ship shape (implementation record, design §5): suppression of the
//! two medallion seeds (0 dodecagon, 1 hexagram — the hull footprint,
//! §9.3) plus record chords — the dodecagon `{12/4}` class `m_j →
//! m_{j+4}` and the hexagram `{6/2}` figure triangles `t_j → t_{j+2}`.
//!
//! The doc's "δ = 60 must pass the closure gates" does NOT hold for
//! the generic composition (documented blocker): the dodecagon chords
//! meet the rim at 60° but the star chords cross the star edge
//! midpoints at 90°, and no single δ matches both families, so the
//! fused weave stays open at every δ. The honesty case (δ = 30 fails
//! with the §4.4 named-record error) therefore holds at BOTH classic
//! angles, and this test pins the honest truth: wall/exit/corner/bad
//! counts, the outline style passing, and the interlace styles failing
//! NAMED at δ ∈ {30, 60}. See the design doc implementation record.

use std::collections::BTreeMap;

use tess_core::geometry::{BBox, Vec2};
use tess_core::params::ParamSet;
use tess_core::strapwork::{
    plan_decoration, plan_decoration_medallion, validity_stats, Decoration, Wall,
};
use tess_core::tiling::{Query, Tile};
use tess_tilings::medallion::weave;

const S3: f64 = 1.732_050_807_568_877_2;

fn region() -> BBox {
    BBox::new(Vec2::new(-8.0, -8.0), Vec2::new(8.0, 8.0))
}

fn m172_tiles() -> Vec<Tile> {
    let reg = tess_tilings::registry();
    let t = reg.get("dodecagon-hexagram-honeycomb").expect("M172 registered");
    let p = ParamSet::default_for(t.params());
    let res = t.generate(&p, &Query { region: region() });
    assert!(res.message.is_none());
    res.tiles
}

/// The wiring shape: seeds {0, 1}, every medallion-seed tile
/// suppressed, and exactly 12 chords per dodecagon + 6 per star.
#[test]
fn a3_wiring_shape() {
    let tiles = m172_tiles();
    let reg = tess_tilings::registry();
    let t = reg.get("dodecagon-hexagram-honeycomb").unwrap();
    let w = weave::medallion_wiring(t, &tiles, &region()).expect("M172 carries a layer");
    assert_eq!(w.suppressed_seeds, vec![0, 1]);
    let n_dod = tiles.iter().filter(|x| x.seed == 0).count();
    let n_star = tiles.iter().filter(|x| x.seed == 1).count();
    assert_eq!(w.suppressed.len(), n_dod + n_star);
    assert_eq!(w.extra.len(), 12 * n_dod + 6 * n_star);
    // Pinned region census (default scale 1, region [-8, 8]^2).
    assert_eq!(n_dod, 105);
    assert_eq!(n_star, 228);
    assert_eq!(w.suppressed.len(), 333);
    assert_eq!(w.extra.len(), 2628);
    assert_eq!(w.record, "M172 {12/4} dodecagon \u{2295} {6/2} hexagram");
    assert_eq!(w.absorption, "LandingIdentity");
}

/// The chord shape, checked against the tile geometry analytically:
/// every dodecagon chord joins two EDGE MIDPOINTS of its tile, every
/// star chord joins two TIP vertices, and every chord subtends
/// exactly 120° at its tile center — endpoints equidistant `r` from
/// the centroid, length `r·√3` (1e-9). At default scale the tip class
/// has `r = 1/2` (length √3/2) and the midpoint class `r = cos 15°/2`
/// (the dodecagon apothem), so the class lengths sit at
/// `√3/2 · {1, cos 15°}`.
#[test]
fn a3_chord_shape_is_analytic() {
    let tiles = m172_tiles();
    let reg = tess_tilings::registry();
    let t = reg.get("dodecagon-hexagram-honeycomb").unwrap();
    let w = weave::medallion_wiring(t, &tiles, &region()).unwrap();
    let cos15 = (6.0f64.sqrt() + 2.0f64.sqrt()) / 4.0;
    let (mut l_dod, mut l_star) = (None, None);
    for (chord_i, wl) in w.extra.iter().enumerate() {
        // Attribute the chord to a tile by endpoint class membership.
        let mut classes: Vec<(u32, f64)> = Vec::new(); // (seed, centroid distance)
        for tile in &tiles {
            let q = &tile.poly.pts;
            let near = |p: Vec2, r: Vec2| (p - r).length() < 1e-9;
            let mut c = Vec2::ZERO;
            for p in q {
                c = c + *p;
            }
            c = c / q.len() as f64;
            match tile.seed {
                0 => {
                    let mid: Vec<Vec2> =
                        (0..q.len()).map(|j| (q[j] + q[(j + 1) % q.len()]) * 0.5).collect();
                    if mid.iter().any(|m| near(wl.a, *m)) && mid.iter().any(|m| near(wl.b, *m)) {
                        classes.push((0, (wl.a - c).length()));
                    }
                }
                1 => {
                    let rmax = q.iter().map(|p| (*p - c).length()).fold(0.0f64, f64::max);
                    let tips: Vec<Vec2> =
                        q.iter().cloned().filter(|p| (*p - c).length() > 0.9 * rmax).collect();
                    if tips.len() == 6
                        && tips.iter().any(|m| near(wl.a, *m))
                        && tips.iter().any(|m| near(wl.b, *m))
                    {
                        classes.push((1, (wl.a - c).length()));
                    }
                }
                _ => {}
            }
        }
        assert!(
            !classes.is_empty(),
            "chord {:?}→{:?} is not a midpoint/tip pair of any tile",
            wl.a,
            wl.b
        );
        let (seed, r) = classes[0];
        let len = (wl.b - wl.a).length();
        // 120° subtension: endpoints on one radius, chord = r·sqrt(3).
        assert!(
            (len - r * S3).abs() < 1e-9,
            "chord {chord_i} (seed {seed}) length {len} != r·sqrt(3) = {}",
            r * S3
        );
        // Exact class lengths at default scale.
        let expect = if seed == 0 { 0.5 * S3 * cos15 } else { 0.5 * S3 };
        assert!((len - expect).abs() < 1e-9, "chord {chord_i} class length {len} != {expect}");
        match seed {
            0 => l_dod = Some(len),
            _ => l_star = Some(len),
        }
    }
    // Both classes present, and their exact ratio is 1 / cos 15°.
    let (a, b) = (l_dod.expect("dodecagon chords present"), l_star.expect("star chords present"));
    assert!((b / a - 1.0 / cos15).abs() < 1e-9);
    // Every chord endpoint lies at a shared contact: dodecagon midpoints
    // carry two chord ends (m_j→m_{j+4}, m_{j-4}→m_j), and a star tip
    // kissing another star tip carries four (two per star). Tiles are
    // emitted whole, so every class end is shared and the grid parity
    // is even everywhere.
    let mut ends: BTreeMap<(i64, i64), usize> = BTreeMap::new();
    for wl in &w.extra {
        for p in [wl.a, wl.b] {
            *ends.entry(((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)).or_insert(0) += 1;
        }
    }
    assert!(
        ends.values().all(|&v| v % 2 == 0 && v <= 4),
        "chord ends are not pairwise at contacts: {:?}",
        ends.values().copied().collect::<std::collections::BTreeSet<_>>()
    );
}

/// Every raw chord is exactly covered by fused pieces: the fused
/// sub-segments collinear with a chord partition it with no gaps and
/// no double coverage (gaps would drop the record; spurious splits
/// would duplicate strands).
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
            return false; // gap
        }
        if b > pos {
            pos = b;
        }
    }
    (pos - l).abs() < 1e-7
}

#[test]
fn a3_chords_covered_by_fused_pieces() {
    let tiles = m172_tiles();
    let reg = tess_tilings::registry();
    let t = reg.get("dodecagon-hexagram-honeycomb").unwrap();
    for delta in [30.0, 60.0] {
        let wv = weave::fused_weave(t, &tiles, &region(), delta);
        let med = wv.med.as_ref().expect("layer present");
        for (i, c) in med.extra.iter().enumerate() {
            assert!(covered(&wv.walls, c), "chord {i} at δ={delta} not covered by fused pieces");
        }
    }
}

/// A5 determinism: pure emission — repeated calls and a tile-emit
/// permutation (reversal) give the identical fused `Vec<Wall>`.
#[test]
fn a5_fused_output_is_deterministic() {
    let tiles = m172_tiles();
    let reg = tess_tilings::registry();
    let t = reg.get("dodecagon-hexagram-honeycomb").unwrap();
    let r = region();
    let a = weave::fused_weave(t, &tiles, &r, 60.0);
    let b = weave::fused_weave(t, &tiles, &r, 60.0);
    assert_eq!(a.walls, b.walls, "repeat call diverged");
    let mut rev = tiles.clone();
    rev.reverse();
    let c = weave::fused_weave(t, &rev, &r, 60.0);
    assert_eq!(a.walls, c.walls, "tile permutation changed the fused map");
    let mut rot = tiles.clone();
    rot.rotate_left(37);
    let d = weave::fused_weave(t, &rot, &r, 60.0);
    assert_eq!(a.walls, d.walls, "rotation of the tile order changed the fused map");
}

/// The honest gate truth at both classic angles (see the module docs
/// for why the doc's δ = 60 PASS sub-claim cannot hold): fixed counts
/// and the §4.4 named-record error on the weave gates.
#[test]
fn a3_gate_truth_at_both_classic_angles() {
    let tiles = m172_tiles();
    let reg = tess_tilings::registry();
    let t = reg.get("dodecagon-hexagram-honeycomb").unwrap();
    let r = region();
    for (delta, walls, exit, corner, bad, joint, fill_odd) in
        [(30.0f64, 29784usize, 1212usize, 606usize, 5828usize, 8026usize, 3520usize), (60.0, 42972, 1212, 606, 5828, 13920, 4732)]
    {
        let wv = weave::fused_weave(t, &tiles, &r, delta);
        assert_eq!(wv.walls.len(), walls, "fused wall count at δ={delta}");
        let s = validity_stats(&wv.walls, &r);
        assert_eq!((s.exit, s.corner, s.bad, s.joint), (exit, corner, bad, joint), "stats δ={delta}");
        let med = wv.med.as_ref().unwrap();
        // fill: odd-degree gate, message unchanged (§4.4: unnamed).
        let e = plan_decoration_medallion(&wv.walls, 0.3, Decoration::Fill, &r.padding(-1e-9), Some(&med.ctx()))
            .expect_err("the map is not Eulerian at any δ");
        let msg = e.to_string();
        assert!(msg.contains(&format!("{fill_odd} odd-degree")), "fill count δ={delta}: {msg}");
        assert!(!msg.contains("medallion"), "fill must stay unnamed: {msg}");
        // outline: ungated, passes.
        plan_decoration(&wv.walls, 0.3, Decoration::Outline, &r.padding(-1e-9))
            .expect("outline is ungated");
        // interlace styles: open ends, NAMED error (law (L)).
        for st in [Decoration::Interlace, Decoration::OutlineInterlace] {
            let e = plan_decoration_medallion(&wv.walls, 0.3, st, &r.padding(-1e-9), Some(&med.ctx()))
                .expect_err("the weave does not close at any δ");
            let msg = e.to_string();
            assert!(msg.contains("1798 open wall end(s)"), "open count δ={delta} {st:?}: {msg}");
            assert!(msg.contains("medallion layer 'M172 {12/4} dodecagon \u{2295} {6/2} hexagram'"), "{msg}");
            assert!(msg.contains("declared absorption LandingIdentity"), "{msg}");
            assert!(msg.contains("law (L) requires theta_record = delta"), "{msg}");
        }
    }
}

/// Off-state through the wiring surface: a tiling with no layer row
/// and empty trait hooks resolves to `None` and plain `walls`.
#[test]
fn off_state_tilings_have_no_layer() {
    let reg = tess_tilings::registry();
    let r = region();
    for id in ["square", "hexagon", "truncated-square", "truncated-hexagonal"] {
        let t = reg.get(id).unwrap();
        let p = ParamSet::default_for(t.params());
        let res = t.generate(&p, &Query { region: r });
        let wv = weave::fused_weave(t, &res.tiles, &r, 45.0);
        assert!(wv.med.is_none(), "{id} must have no medallion layer");
        assert_eq!(wv.walls, tess_core::strapwork::walls(&res.tiles, 45.0), "{id} off-state walls");
    }
    // The layer table carries exactly the M172 row (M203 scope).
    assert_eq!(weave::medallion_layer_specs().len(), 1);
}
