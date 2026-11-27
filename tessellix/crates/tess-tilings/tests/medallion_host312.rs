//! M203 anchor A1 (design doc `docs/medallion-third-layer-design.md`
//! §7): the WIS-interstitial march over the `3.12.12` host patch.
//!
//! The third layer here is the M158 star's own record geometry: the
//! 24 exit rays marched through the twelve declared neighbours with
//! the division-free exact [`first_hit`] engine, converted to f64
//! once at the view boundary (design §6). The pinned absorption
//! multiset `{1+√3: 12, 2+√3: 12}` holds in f64 to 1e-9 and agrees
//! with the exact certificate [`host312::bind_and_absorb`] — that is
//! the core A1 claim **[P — same march data]**.
//!
//! The doc's closure sub-claim ("zero degree-1 ends inside the view,
//! all four styles gate") does NOT hold for a hull-suppressed patch:
//! the rays land on vertices of the *unsuppressed* neighbouring
//! dodecagons, so each ray is a free end there (24 degree-1 exits,
//! 12 corners). Closing that weave requires the neighbours' own
//! medallion records to take the strands over — nested medallions,
//! the design §10 backlog. This test pins the honest truth: the
//! interlace styles fail with the §4.4 named-record error (correctly
//! naming M158: the interstitial absorption does not continue through
//! a host that is not itself wearing its record), `outline` passes,
//! and `fill` fails unnamed. See the design doc implementation record.

use std::collections::BTreeMap;

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::strapwork::{
    plan_decoration, plan_decoration_medallion, Decoration, MedallionContext, Wall,
};
use tess_core::tiling::Tile;
use tess_tilings::medallion::{first_hit, host312, qp_f64, rot30n, wis_m158};

const S3: f64 = 1.732_050_807_568_877_2;

fn q2v(p: [f64; 2]) -> Vec2 {
    Vec2::new(p[0], p[1])
}

fn key_pt(p: Vec2) -> (i64, i64) {
    ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
}

struct March {
    walls: Vec<Wall>,
    ts: Vec<f64>,
    landings: Vec<Vec2>,
}

/// March the 24 recorded exit rays through the host patch: each ray
/// splits at every tile edge it crosses strictly before its landing.
fn march() -> March {
    let w = host312::hole();
    let tiles = host312::neighbours();
    let polys: Vec<Vec<_>> = tiles.iter().map(|(_, p)| p.clone()).collect();
    let qpolys: Vec<Vec<Vec2>> = polys
        .iter()
        .map(|poly| poly.iter().map(|v| q2v(qp_f64(*v))).collect())
        .collect();

    let mut walls = Vec::new();
    let mut ts = Vec::new();
    let mut landings = Vec::new();
    for (i, d) in wis_m158::exit_rays() {
        let p = w[i];
        let u = rot30n(d, 4);
        let h = first_hit(p, u, &polys).expect("every recorded ray hits the patch");
        let t = h.t_num.fl() / h.t_den.fl();
        ts.push(t);
        let pf = q2v(qp_f64(p));
        let uf = q2v(qp_f64(u));
        landings.push(pf + uf * t);
        // Hop split at every crossed edge strictly between.
        let mut cuts: Vec<f64> = vec![0.0, t];
        for q in &qpolys {
            for j in 0..q.len() {
                let (a, b) = (q[j], q[(j + 1) % q.len()]);
                let e = b - a;
                let den = uf.cross(e);
                if den.abs() < 1e-12 {
                    continue;
                }
                let aa = a - pf;
                let s = aa.cross(e) / den;
                let r = aa.cross(uf) / den;
                if s > 1e-9 && s < t - 1e-9 && r > 1e-9 && r < 1.0 - 1e-9 {
                    cuts.push(s);
                }
            }
        }
        cuts.sort_by(f64::total_cmp);
        cuts.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
        for win in cuts.windows(2) {
            walls.push(Wall {
                a: pf + uf * win[0],
                b: pf + uf * win[1],
            });
        }
    }
    March {
        walls,
        ts,
        landings,
    }
}

/// The pinned absorption multiset in f64 at 1e-9, agreeing with the
/// exact certificate (same march data, one conversion).
#[test]
fn a1_absorption_multiset_matches_exact_certificate() {
    let m = march();
    assert_eq!(m.ts.len(), 24);
    let c1 = m.ts.iter().filter(|t| (**t - (1.0 + S3)).abs() < 1e-9).count();
    let c2 = m.ts.iter().filter(|t| (**t - (2.0 + S3)).abs() < 1e-9).count();
    assert_eq!((c1, c2), (12, 12), "f64 absorption multiset");
    // Every t is one of the two values (no third class sneaks in).
    assert!(m.ts.iter().all(|t| {
        (*t - (1.0 + S3)).abs() < 1e-9 || (*t - (2.0 + S3)).abs() < 1e-9
    }));
    // The exact certificate over the same rays: (t1, t2, vertex hits,
    // misses, census).
    let (e1, e2, vtx, miss, census) = host312::bind_and_absorb();
    assert_eq!((e1, e2, vtx, miss), (12, 24 - 12, 24, 0));
    assert_eq!(census.len(), 6, "four rays per dodecagon, none on triangles");
    assert!(census.values().all(|&v| v == 4));
}

/// Landings are exact dodecagon vertices (f64 mirror of the exact
/// vertex proof) and pairwise distinct (no two rays share a vertex).
#[test]
fn a1_landings_are_distinct_dodecagon_vertices() {
    let m = march();
    let tiles = host312::neighbours();
    let verts: Vec<Vec2> = tiles
        .iter()
        .filter(|(label, _)| label.starts_with("dod@"))
        .flat_map(|(_, poly)| poly.iter().map(|v| q2v(qp_f64(*v))))
        .collect();
    for (i, l) in m.landings.iter().enumerate() {
        assert!(
            verts.iter().any(|v| (*v - *l).length() < 1e-9),
            "landing {i} {l:?} is not a neighbour dodecagon vertex"
        );
    }
    let distinct: BTreeMap<_, _> = m.landings.iter().map(|l| (key_pt(*l), ())).collect();
    assert_eq!(distinct.len(), 24, "the 24 landings are pairwise distinct");
}

/// The march network's honest topology: 24 free ends at the landings'
/// contacts and 12 two-valent joints, zero unpaired crossings — and
/// the gate truth of the module docs (closure needs the §10 nested
/// medallions).
#[test]
fn a1_network_topology_and_gate_truth() {
    let m = march();
    let mut deg: BTreeMap<(i64, i64), usize> = BTreeMap::new();
    for wl in &m.walls {
        *deg.entry(key_pt(wl.a)).or_insert(0) += 1;
        *deg.entry(key_pt(wl.b)).or_insert(0) += 1;
    }
    let mut hist: BTreeMap<usize, usize> = BTreeMap::new();
    for v in deg.values() {
        *hist.entry(*v).or_insert(0) += 1;
    }
    assert_eq!(hist.get(&1), Some(&24), "24 free strand ends");
    assert_eq!(hist.get(&2), Some(&12), "12 two-valent joints");
    assert!(hist.keys().all(|&k| k <= 2), "no unpaired >= 3 crossings");

    let mut lo = Vec2::new(f64::MAX, f64::MAX);
    let mut hi = Vec2::new(f64::MIN, f64::MIN);
    for wl in &m.walls {
        for p in [wl.a, wl.b] {
            lo.x = lo.x.min(p.x);
            lo.y = lo.y.min(p.y);
            hi.x = hi.x.max(p.x);
            hi.y = hi.y.max(p.y);
        }
    }
    let region = BBox::new(Vec2::new(lo.x - 0.5, lo.y - 0.5), Vec2::new(hi.x + 0.5, hi.y + 0.5));

    let hole_tile = Tile {
        proto: 0,
        poly: Polygon::from_slice(
            &host312::hole()
                .iter()
                .map(|q| q2v(qp_f64(*q)))
                .collect::<Vec<_>>(),
        ),
        seed: 0,
        level: 0,
    };
    let med = MedallionContext {
        record: "WIS 12-point star (M158)",
        absorption: "Interstitial",
        suppressed: &[hole_tile],
        extra: &m.walls,
    };
    // fill: odd-degree gate, unnamed (§4.4 unchanged code).
    let e = plan_decoration_medallion(&m.walls, 0.3, Decoration::Fill, &region.padding(-1e-9), Some(&med))
        .expect_err("24 free ends are odd-degree");
    let msg = e.to_string();
    assert!(msg.contains("24 odd-degree"), "{msg}");
    assert!(!msg.contains("medallion"), "fill stays unnamed: {msg}");
    // outline: ungated.
    plan_decoration(&m.walls, 0.3, Decoration::Outline, &region.padding(-1e-9))
        .expect("outline is ungated");
    // interlace styles: the honest named failure (the record's
    // interstitial absorption does not close through bare hosts).
    for st in [Decoration::Interlace, Decoration::OutlineInterlace] {
        let e = plan_decoration_medallion(
            &m.walls,
            0.3,
            st,
            &region.padding(-1e-9),
            Some(&med),
        )
        .expect_err("free ends cannot weave");
        let msg = e.to_string();
        assert!(msg.contains("36 open wall end(s)"), "{st:?}: {msg}");
        assert!(msg.contains("medallion layer 'WIS 12-point star (M158)'"), "{msg}");
        assert!(msg.contains("declared absorption Interstitial"), "{msg}");
    }
}
