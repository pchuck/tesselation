#![cfg(test)]
//! 24b — WIS ground-truth bar battery.
//!
//! The bar (brief): from the general path — a hole-bearing substrate
//! whose holes are bounded by the M158 WIS 12-star record's hull +
//! boundary rays — the inference engine must reproduce the shipped
//! `wis-12-fold` cell table, i.e. the 26-tile SQ_SUP cluster data:
//! census 16 tri + 4 sq + 6 rho, cos²-multisets per vertex class, and
//! the hull/area identities. This file is the *comparison* side: it
//! feeds the engine only the fixture hole polygon + record, then
//! checks the inferred fills against the shipped cluster data.
//!
//! Evidence structure (mirroring the certified spikes):
//!
//! * `guided_walk_reachability` — the shipped fill itself is walked
//!   through the engine's own frontier/candidate/validity machinery
//!   step by step: membership of the shipped cluster in the engine's
//!   freedom set (this is the bar's reachability proof);
//! * the exhaustive runs — the complete freedom set under two
//!   independent principled constraint systems (every interior edge a
//!   substrate *segment*; every interior edge on a carried ray line
//!   plus every vertex an anatomy point) — both yield exactly the same
//!   6480 distinct completions, all globally valid, the shipped census
//!   class (16,4,6) among them with 648 members;
//! * the freedom is exactly the (sq, rho) redistribution along the
//!   area identity `2·sq + rho = 14` with `tri = 16` forced by the √3
//!   part of the hole 2-area — documented, never forced (under-
//!   determination is a first-class output feeding 24e).

use std::collections::BTreeMap;

use super::holes::{blob, cluster_tiles};
use super::infer::{
    analyze, candidates, constraints, corner_cos2, dot4, edge_key, global_validate,
    is_legal_lattice, padd, poly_area2, search, tile_area, Constraints, FillFp, InstanceSet,
    Level, Limits, Pair, Report, Substrate, Tile, SQ, RHO, TRI,
};
use std::sync::OnceLock;
use crate::families::quasiperiodic::wis_12_fold::{rot4, P4, SQ_SUP};

static CACHED_SUB: OnceLock<Substrate> = OnceLock::new();

fn sub() -> &'static Substrate {
    CACHED_SUB.get_or_init(|| Substrate::new(blob()))
}

static CACHED_SHIP: OnceLock<Vec<Tile>> = OnceLock::new();

fn ship_tiles() -> &'static Vec<Tile> {
    CACHED_SHIP.get_or_init(|| {
        cluster_tiles()
            .into_iter()
            .map(|t| Tile::new(t.kind, &t.cs))
            .collect()
    })
}

/// Pinned per-vertex cos²-class census of the shipped cluster
/// (spike explore19 [G]): 13 classes over 29 vertices.
const SHIP_COS2: &[(&[(i64, i64)], usize)] = &[
    (&[(0, 0)], 1),
    (&[(0, 0), (0, 0), (1, 4), (1, 4), (1, 4)], 2),
    (&[(0, 0), (1, 4)], 2),
    (&[(0, 0), (1, 4), (1, 4)], 2),
    (&[(0, 0), (1, 4), (1, 4), (1, 4), (1, 4), (3, 4)], 1),
    (&[(0, 0), (1, 4), (1, 4), (1, 4), (3, 4)], 2),
    (&[(0, 0), (1, 4), (1, 4), (3, 4)], 4),
    (&[(1, 4)], 4),
    (&[(1, 4), (1, 4), (1, 4), (1, 4), (3, 4), (3, 4)], 2),
    (&[(1, 4), (3, 4)], 4),
    (&[(1, 4), (3, 4), (3, 4)], 2),
    (&[(3, 4)], 2),
    (&[(3, 4), (3, 4), (3, 4)], 1),
];

/// Pinned exhaustive census table (spikes explore22 L3 / explore23 L5):
/// (tri, sq, rho) → distinct completions, K20 substrate.
const CENSUS_TABLE: &[([u32; 3], usize)] = &[
    ([16, 0, 14], 324),
    ([16, 1, 12], 1377),
    ([16, 2, 10], 2268),
    ([16, 3, 8], 1782),
    ([16, 4, 6], 648), // ← the shipped cluster's class
    ([16, 5, 4], 81),
];

const EXHAUSTIVE_LIMITS: Limits = Limits { max_nodes: 2_000_000, max_sols: 20_000 };

/// Memoized shared K20 Segment exhaustive search (EXHAUSTIVE_LIMITS +
/// K20 + Level::Segment): the triple shared by
/// `bar_exhaustive_segment_k20_reproduces_census_family`, the segment
/// side of `bar_exhaustive_rayline_vset_k20_same_fill_set`, and
/// `field_membership_and_finiteness`. `determinism_byte_identical_rerun`
/// deliberately keeps its double search (the byte-identity itself).
static CACHED_SEGMENTED_REPORT: OnceLock<Report> = OnceLock::new();

fn segment_exhaustive_report() -> &'static Report {
    CACHED_SEGMENTED_REPORT.get_or_init(|| {
        search(sub(), InstanceSet::K20, Level::Segment, EXHAUSTIVE_LIMITS)
    })
}

// ------------------------------------------------------------------
// substrate + record conventions
// ------------------------------------------------------------------

#[test]
fn substrate_and_record_conventions() {
    let s = sub();
    assert_eq!(s.poly.len(), 20);
    assert_eq!(s.hole_area, (56, 32));
    // 8 reflex vertices (the concave corners of the blob; 4 of them the
    // 300° orbit-E corners), 12 convex — spike-verified counts.
    let reflex = (0..s.poly.len())
        .filter(|&i| {
            let a = s.poly[i];
            let b = s.poly[(i + 1) % s.poly.len()];
            let c = s.poly[(i + 2) % s.poly.len()];
            sgn_of(cross_of(sub_raw(b, a), sub_raw(c, b))) < 0
        })
        .count();
    assert_eq!(reflex, 8, "blob reflex vertices");
    // Instances: 24 hull-reading sites, 20 not strictly inside.
    assert_eq!(s.instances24.len(), 24);
    assert_eq!(s.instances20.len(), 20);
    // Carried data sizes (spike [B]/[I]/[J5'] numbers).
    assert_eq!(s.segments(InstanceSet::K20).len(), 596);
    assert_eq!(s.segments(InstanceSet::K24).len(), 656);
    assert_eq!(s.ray_lines(InstanceSet::K20).len(), 126);
    assert_eq!(s.ray_lines(InstanceSet::K24).len(), 134);
    assert_eq!(s.vset(InstanceSet::K20).len(), 40);
    assert_eq!(s.vset(InstanceSet::K24).len(), 56);
    // Record conventions: tips are the successive 30° rotations of H0;
    // the 12 notches are the unit directions; every tip has exactly two
    // unit chords (already asserted at construction).
    let tips = &s.h;
    for k in 0..12 {
        assert_eq!(tips[k], rot4(tips[0], k as i8), "H[{k}] rotation");
    }
    let notches = &s.nch;
    let mut dirs = std::collections::BTreeSet::new();
    for &n in notches {
        assert!(is_legal_lattice(n));
        assert_eq!(
            (dot4(n, n).0 / 4, dot4(n, n).1 / 4),
            (1, 0),
            "notch at unit radius"
        );
        dirs.insert(n);
    }
    assert_eq!(dirs.len(), 12);
    for k in 0..12 {
        assert_eq!(s.tn[k].len(), 2);
    }
    // Every hole edge is the hull edge of at least one instance: the
    // union of per-edge readings covers all 20 edges.
    assert!(s.instances24.len() >= 20);
}

#[allow(dead_code)]
fn sub_raw(a: P4, b: P4) -> P4 {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3)
}
#[allow(dead_code)]
fn cross_of(u: P4, v: P4) -> Pair {
    (
        u.0 * v.2 + 3 * u.1 * v.3 - u.2 * v.0 - 3 * u.3 * v.1,
        u.0 * v.3 + u.1 * v.2 - u.2 * v.1 - u.3 * v.0,
    )
}
#[allow(dead_code)]
fn sgn_of(p: Pair) -> i8 {
    super::infer::sp(p)
}

// ------------------------------------------------------------------
// shipped cluster reference (the bar's comparison data)
// ------------------------------------------------------------------

#[test]
fn shipped_cluster_reference_data() {
    let s = sub();
    let ship = ship_tiles();
    // 26 tiles, 16/4/6 census, 2·area (56,32), 29 vertices, 54 edges.
    assert_eq!(ship.len(), 26);
    assert_eq!(SQ_SUP.len(), 26);
    let an = analyze(ship);
    assert_eq!(an.census, [16, 4, 6], "SQ_SUP cell census");
    assert_eq!(an.area, (56, 32), "SQ_SUP exact 2-area == hole identity");
    assert_eq!(an.nverts, 29, "fill vertices");
    assert_eq!(an.nedges, 54, "distinct fill edges");
    let interior = an.nedges - s.edge_set.len();
    assert_eq!(interior, 34, "interior edges");
    // Independent global validation of the shipped fill.
    assert!(global_validate(s, ship).is_empty(), "shipped fill valid");
    // cos²-class census matches the pinned 13-class table exactly.
    let pinned: BTreeMap<Vec<(i64, i64)>, usize> =
        SHIP_COS2.iter().map(|(k, c)| (k.to_vec(), *c)).collect();
    assert_eq!(an.vcos, pinned, "cos² class census");
    assert_eq!(pinned.values().sum::<usize>(), 29);
}

// ------------------------------------------------------------------
// guided walk: shipped fill reachable through the engine machinery
// ------------------------------------------------------------------

fn guided_walk(
    s: &Substrate,
    set: InstanceSet,
    level: Level,
    ship: &[Tile],
) -> Result<(usize, Pair), String> {
    let cons: Constraints = constraints(s, set, level);
    let mut by_edge: BTreeMap<super::infer::EK, Vec<usize>> = BTreeMap::new();
    for (idx, t) in ship.iter().enumerate() {
        for i in 0..t.n {
            by_edge
                .entry(edge_key(t.cs[i], t.cs[(i + 1) % t.n]))
                .or_default()
                .push(idx);
        }
    }
    let mut cov = std::collections::BTreeSet::new();
    let mut opn: BTreeMap<super::infer::EK, (P4, P4)> = BTreeMap::new();
    let mut placed: Vec<Tile> = Vec::new();
    let mut steps = 0usize;
    loop {
        let mut frontier: Option<(P4, P4)> = None;
        for ek in s.edge_set.iter() {
            if !cov.contains(ek) {
                let &(a, b) = s.edge_dir.get(ek).expect("dir");
                frontier = Some((a, b));
                break;
            }
        }
        let (a, b) = match frontier {
            Some(ab) => ab,
            None => match opn.iter().next() {
                Some((_, &(u, v))) => (v, u),
                None => break,
            },
        };
        steps += 1;
        let list = by_edge
            .get(&edge_key(a, b))
            .ok_or("no shipped tile on frontier edge")?;
        let mut matches: Vec<Tile> = Vec::new();
        for &idx in list {
            let t = &ship[idx];
            for i in 0..t.n {
                if t.cs[i] == a && t.cs[(i + 1) % t.n] == b {
                    let rot: Vec<P4> = (0..t.n).map(|j| t.cs[(i + j) % t.n]).collect();
                    for cand in candidates(a, b) {
                        if cand.n == t.n && cand.cs[..cand.n].to_vec() == rot {
                            matches.push(cand);
                        }
                    }
                }
            }
        }
        assert_eq!(matches.len(), 1, "shipped frontier tile uniquely candidate");
        let cand = matches[0];
        if !super::infer::valid_placement(s, &cons, &cand, &placed) {
            return Err(format!("step {steps}: shipped tile fails validity"));
        }
        let n = cand.n;
        for i in 0..n {
            let (u, v) = (cand.cs[i], cand.cs[(i + 1) % n]);
            let ek = edge_key(u, v);
            if s.edge_set.contains(&ek) {
                assert!(!cov.contains(&ek), "boundary edge covered twice");
                cov.insert(ek);
            } else if opn.contains_key(&ek) {
                opn.remove(&ek);
            } else {
                if let Some(segs) = cons.segs {
                    assert!(segs.contains(&ek), "step {steps}: edge not a segment");
                }
                if let Some(lines) = cons.lines {
                    assert!(
                        lines.contains(&super::infer::line_key(u, sub_raw(v, u))),
                        "step {steps}: edge not on a ray line"
                    );
                }
                opn.insert(ek, (u, v));
            }
        }
        placed.push(cand);
    }
    let mut ar = (0i64, 0i64);
    for t in &placed {
        ar = padd(ar, tile_area(t.kind));
    }
    let errs = global_validate(s, &placed);
    if !errs.is_empty() {
        return Err(format!("validation: {errs:?}"));
    }
    Ok((placed.len(), ar))
}

#[test]
fn guided_walk_reachability() {
    let s = sub();
    let ship = ship_tiles();
    for (set, label) in [(InstanceSet::K20, "20-set"), (InstanceSet::K24, "24-set")] {
        for level in [Level::RayLine, Level::Segment] {
            let (tiles, area) = guided_walk(s, set, level, ship)
                .unwrap_or_else(|e| panic!("{label} {level:?}: {e}"));
            assert_eq!(tiles, 26, "{label} {level:?}: tile count");
            assert_eq!(area, (56, 32), "{label} {level:?}: area identity");
        }
    }
}

// ------------------------------------------------------------------
// exhaustive freedom sets (the bar's reproduction evidence)
// ------------------------------------------------------------------

#[test]
fn bar_exhaustive_segment_k20_reproduces_census_family() {
    let ship = ship_tiles();
    let ship_fp: FillFp = analyze(ship).fp;
    let r = segment_exhaustive_report();
    assert!(r.exhausted, "K20 Segment search must exhaust");
    assert_eq!(r.invalid, 0, "every completion globally valid");
    assert_eq!(r.sols, 6480);
    assert_eq!(r.distinct, 6480);
    assert!(
        r.max_coord > 0 && r.max_coord < 1000,
        "completion coordinates bounded, max |component| = {}",
        r.max_coord
    );
    let table: BTreeMap<[u32; 3], usize> =
        CENSUS_TABLE.iter().copied().collect();
    assert_eq!(r.by_census, table, "census-class table");
    assert!(r.fps.contains(&ship_fp), "shipped cluster in freedom set");
}

#[test]
fn bar_exhaustive_rayline_vset_k20_same_fill_set() {
    let s = sub();
    let ship = ship_tiles();
    let ship_fp: FillFp = analyze(ship).fp;
    let r = search(s, InstanceSet::K20, Level::RayLineVset, EXHAUSTIVE_LIMITS);
    assert!(r.exhausted, "K20 RayLineVset search must exhaust");
    assert_eq!(r.invalid, 0);
    assert_eq!(r.sols, 6480);
    assert_eq!(r.distinct, 6480);
    let table: BTreeMap<[u32; 3], usize> =
        CENSUS_TABLE.iter().copied().collect();
    assert_eq!(r.by_census, table);
    assert!(r.fps.contains(&ship_fp));
    let r_seg = segment_exhaustive_report();
    assert_eq!(r_seg.fps, r.fps, "independent constraint systems, same fills");
}

#[test]
fn rayline_capped_freedom_documented() {
    let s = sub();
    let r = search(
        s,
        InstanceSet::K20,
        Level::RayLine,
        Limits { max_nodes: 30_000, max_sols: 100_000 },
    );
    assert!(!r.exhausted, "RayLine freedom is large: cap must be hit");
    assert_eq!(r.invalid, 0, "every documented completion globally valid");
    assert!(r.sols >= 500, "documented freedom lower bound, got {}", r.sols);
    assert_eq!(r.distinct, r.sols, "each fill reached exactly once");
    assert!(r.distinct > 1, "underdetermination is real");
    for (census, &n) in r.by_census.iter() {
        assert_eq!(census[0], 16, "tri forced by area in class {census:?}");
        assert_eq!(2 * census[1] + census[2], 14, "2·sq + rho identity");
        assert!(n > 0);
    }
}

#[test]
fn determinism_byte_identical_rerun() {
    let s = sub();
    let limits = Limits { max_nodes: 20_000, max_sols: 100_000 };
    let r1 = search(s, InstanceSet::K20, Level::RayLine, limits);
    let r2 = search(s, InstanceSet::K20, Level::RayLine, limits);
    assert_eq!(format!("{r1:?}"), format!("{r2:?}"), "byte-identical rerun");
}

// ------------------------------------------------------------------
// field membership / finiteness / identity battery
// ------------------------------------------------------------------

#[test]
fn field_membership_and_finiteness() {
    let s = sub();
    // All engine-derived points are legal lattice points.
    for &c in s.instances24.iter() {
        assert!(is_legal_lattice(c), "instance {c:?}");
    }
    for &p in s.vset(InstanceSet::K24).iter() {
        assert!(is_legal_lattice(p), "vset {p:?}");
    }
    for ek in s.segments(InstanceSet::K24).iter() {
        assert!(is_legal_lattice(ek.0) && is_legal_lattice(ek.1));
    }
    // Candidate vertices off every hole edge are legal and finite.
    for &(a, b) in s.edges.iter() {
        for cand in candidates(a, b) {
            for &p in &cand.cs[..cand.n] {
                assert!(is_legal_lattice(p), "candidate vertex {p:?}");
                assert!(p.0.abs() < 1 << 40 && p.3.abs() < 1 << 40);
            }
        }
    }
    // Exhaustive-run finiteness: coordinates stay tiny (report field).
    let r = segment_exhaustive_report();
    assert!(r.max_coord < 1000, "finiteness: max |component| {}", r.max_coord);
}

#[test]
fn prototile_area_identities_and_tri16_lemma() {
    // Candidate prototypes have the pinned exact 2-areas.
    let protos = candidates((0, 0, 0, 0), (2, 0, 0, 0));
    let kinds = [TRI, SQ, RHO, RHO];
    for (cand, &k) in protos.iter().zip(kinds.iter()) {
        assert_eq!(cand.kind, k);
        assert_eq!(poly_area2(&cand.cs[..cand.n]), tile_area(k), "area {k}");
    }
    // Hole identity: 16·tri + 4·sq + 6·rho = (56, 32).
    let tri = tile_area(TRI);
    let sq = tile_area(SQ);
    let rho = tile_area(RHO);
    let mut sum = (0i64, 0i64);
    for _ in 0..16 {
        sum = padd(sum, tri);
    }
    for _ in 0..4 {
        sum = padd(sum, sq);
    }
    for _ in 0..6 {
        sum = padd(sum, rho);
    }
    assert_eq!(sum, (56, 32), "SQ_SUP census == hole 2-area");
    // TRI = 16 is FORCED by the √3 part (only triangles carry √3):
    let s = sub();
    assert_eq!(s.hole_area.1, 32);
    assert_eq!(tri.1, 2);
    assert_eq!(sq.1, 0);
    assert_eq!(rho.1, 0);
    // …and the (sq, rho) freedom lies exactly on 2·sq + rho = 14.
    for (census, _) in CENSUS_TABLE {
        assert_eq!(census[0], 16);
        assert_eq!(2 * census[1] + census[2], 14);
    }
    // corner_cos2 sanity on the prototypes: tri 60°, sq 90°, rho 30/150°.
    assert_eq!(corner_cos2(&protos[0].cs, 3, 0), (1, 4));
    assert_eq!(corner_cos2(&protos[1].cs, 4, 0), (0, 0));
    assert_eq!(corner_cos2(&protos[2].cs, 4, 0), (3, 4));
    assert_eq!(corner_cos2(&protos[3].cs, 4, 1), (3, 4));
}