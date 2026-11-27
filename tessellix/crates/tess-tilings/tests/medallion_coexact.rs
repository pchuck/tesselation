//! M203 anchor A2 (design doc `docs/medallion-third-layer-design.md`
//! §7): the coincidence regime. On uniform hosts whose classic angle
//! matches the record chord class, fusing the record chords over the
//! suppression-filtered generic firing reproduces the full generic map
//! EXACTLY — sorted segment-multiset equality on the 1e-6 dedup grid,
//! zero spurious vertices — and all four decoration styles pass on the
//! fused map.
//!
//! * 4.8.8 (truncated square) at δ = 45° with the khatam `{8/2}`
//!   chords (M53 record class): exact coincidence.
//! * 3.12.12 (truncated hexagonal) at δ = 60° with the rosette
//!   `{12/4}` chords (M56 record class): exact coincidence.
//! * 3.12.12 at δ = 30° with the same (60°) chords: the deliberate
//!   mismatch — the map fails the closure gates, and with a
//!   [`MedallionContext`] the §4.4 named-record error fires on the
//!   weave gate as a strict suffix of the plain message.

use tess_core::geometry::{BBox, Vec2};
use tess_core::params::ParamSet;
use tess_core::strapwork::{
    plan_decoration, plan_decoration_medallion, walls, walls_layered, Decoration, MedallionContext,
    Wall,
};
use tess_core::tiling::{Query, Tile};

fn region() -> BBox {
    BBox::new(Vec2::new(-8.0, -8.0), Vec2::new(8.0, 8.0))
}

fn gen(id: &str) -> Vec<Tile> {
    let reg = tess_tilings::registry();
    let t = reg.get(id).unwrap();
    let p = ParamSet::default_for(t.params());
    let res = t.generate(&p, &Query { region: region() });
    assert!(res.message.is_none());
    res.tiles
}

/// The `{n/step}` midpoint-chord figure of every seed-`seed` polygon:
/// midpoint `m_j → m_{j+step}` (the M53 `{8/2}` / M56 `{12/4}` classes).
fn chords(tiles: &[Tile], seed: u32, step: usize) -> Vec<Wall> {
    let mut out = Vec::new();
    for t in tiles.iter().filter(|x| x.seed == seed) {
        let q = &t.poly.pts;
        let n = q.len();
        let m: Vec<Vec2> = (0..n).map(|j| (q[j] + q[(j + 1) % n]) * 0.5).collect();
        for j in 0..n {
            out.push(Wall { a: m[j], b: m[(j + step) % n] });
        }
    }
    out
}

/// Grid keys (`1e6`, the engine's dedup resolution), orientation
/// normalized, sorted — multiset equality on the dedup grid.
fn keys(ws: &[Wall]) -> Vec<((i64, i64), (i64, i64))> {
    let key = |p: Vec2| ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64);
    let mut v: Vec<_> = ws
        .iter()
        .map(|w| {
            let (ka, kb) = (key(w.a), key(w.b));
            if ka <= kb {
                (ka, kb)
            } else {
                (kb, ka)
            }
        })
        .collect();
    v.sort_unstable();
    v
}

/// The coincidence regime: fused (kept generic ∪ chords, fused at
/// crossings) == generic over ALL tiles, as a sorted segment multiset.
#[test]
fn a2_coincidence_multiset_equality() {
    for (id, delta, step) in [
        ("truncated-square", 45.0f64, 2usize),
        ("truncated-hexagonal", 60.0, 4),
    ] {
        let tiles = gen(id);
        let generic = walls(&tiles, delta);
        let kept: Vec<Tile> = tiles.iter().filter(|x| x.seed != 0).cloned().collect();
        let ex = chords(&tiles, 0, step);
        assert!(!ex.is_empty());
        let fused = walls_layered(&kept, delta, &ex);
        assert_eq!(
            keys(&generic),
            keys(&fused),
            "{id}@{delta}: fused map != generic map (coincidence regime)"
        );
    }
}

/// All four decoration styles pass on the coincident fused maps, with
/// pinned face/side counts.
#[test]
fn a2_all_styles_pass_on_fused() {
    for (id, delta, step, faces, sides) in [
        ("truncated-square", 45.0f64, 2usize, 565usize, 42usize),
        ("truncated-hexagonal", 60.0, 4, 867, 56),
    ] {
        let r = region();
        let tiles = gen(id);
        let kept: Vec<Tile> = tiles.iter().filter(|x| x.seed != 0).cloned().collect();
        let fused = walls_layered(&kept, delta, &chords(&tiles, 0, step));
        let plan = plan_decoration(&fused, 0.3, Decoration::Fill, &r.padding(-1e-9))
            .expect("coincident fused map fills");
        assert_eq!(plan.faces.len(), faces, "{id}@{delta} fill faces");
        for st in [
            Decoration::Outline,
            Decoration::Interlace,
            Decoration::OutlineInterlace,
        ] {
            let p = plan_decoration(&fused, 0.3, st, &r.padding(-1e-9))
                .expect("coincident fused map weaves");
            assert_eq!(p.side_lines.len(), sides, "{id}@{delta} {st:?} sides");
        }
    }
}

/// The mismatch regime: 3.12.12 at δ = 30° with the fixed-60° chords
/// does NOT close — the plain gate fails unnamed for `fill` (unchanged
/// §4.4 code) and the weave gate fails with the named-record error as
/// a strict suffix of the byte-identical plain message.
#[test]
fn a2_mismatch_names_the_record() {
    let r = region();
    let tiles = gen("truncated-hexagonal");
    let kept: Vec<Tile> = tiles.iter().filter(|x| x.seed != 0).cloned().collect();
    let suppressed: Vec<Tile> = tiles.iter().filter(|x| x.seed == 0).cloned().collect();
    let ex = chords(&tiles, 0, 4);
    let fused = walls_layered(&kept, 30.0, &ex);
    let med = MedallionContext {
        record: "TEST {12/4}",
        absorption: "LandingIdentity",
        suppressed: &suppressed,
        extra: &ex,
    };
    // fill: odd-degree gate stays unnamed (design §4.4).
    let e = plan_decoration_medallion(&fused, 0.3, Decoration::Fill, &r.padding(-1e-9), Some(&med))
        .expect_err("the mismatched map is not Eulerian");
    let msg = e.to_string();
    assert!(msg.contains("264 odd-degree"), "{msg}");
    assert!(!msg.contains("medallion"), "fill must stay unnamed: {msg}");
    // interlace: open ends named to the record, as a suffix of plain.
    let plain = plan_decoration(&fused, 0.3, Decoration::Interlace, &r.padding(-1e-9))
        .expect_err("the mismatched weave is open");
    let named = plan_decoration_medallion(
        &fused,
        0.3,
        Decoration::Interlace,
        &r.padding(-1e-9),
        Some(&med),
    )
    .expect_err("the mismatched weave is open");
    let (p, n) = (plain.to_string(), named.to_string());
    assert!(n.starts_with(&p), "named message must extend the plain message byte-identically");
    assert!(n.len() > p.len());
    assert!(n.contains("264 open wall end(s)"), "{n}");
    assert!(n.contains("medallion layer 'TEST {12/4}'"), "{n}");
    assert!(n.contains("declared absorption LandingIdentity"), "{n}");
    assert!(n.contains("law (L) requires theta_record = delta"), "{n}");
}
