//! 24b — fixture side: the WIS hole-bearing substrate built from the
//! shipped `SQ_SUP` cluster.
//!
//! **Declared provenance (circularity caveat).** This module uses the
//! shipped cluster data as the *answer key for the fixture*, exactly as
//! 24a's host declaration used its landed neighbourhood: the hole
//! polygon below is the boundary of the shipped 26-tile `SQ_SUP`
//! cluster. The inference engine ([`super::infer`]) never reads this
//! module or `SQ_SUP`; its input is the hole polygon plus the landed
//! M158 record tables only. The fixture/answer separation mirrors the
//! two-tier architecture of the design doc: the substrate construction
//! is a declared fixture, the engine run over it is answer-free.
//!
//! The blob is rebuilt exactly: cluster tiles = `base_corners` rotated
//! by `krot` and translated by `m` (all `SQ_SUP` rows are `piece: 0`,
//! so no fitted-piece offset applies), boundary = unit edges occurring
//! once, chained CCW.

use std::collections::BTreeMap;

use super::infer::{edge_key, poly_area2, sp, EK};
use crate::families::quasiperiodic::wis_12_fold::{add4, base_corners, rot4, P4, SQ_SUP};

/// A reconstructed cluster prototile (kind + exact CCW corners).
#[derive(Clone, Debug)]
pub struct TileC {
    pub kind: u8,
    pub cs: Vec<P4>,
}

/// The 26 shipped cluster tiles in exact raw lattice coordinates.
pub fn cluster_tiles() -> Vec<TileC> {
    let mut out = Vec::with_capacity(SQ_SUP.len());
    for row in SQ_SUP.iter() {
        let base = base_corners(row.kind);
        let cs: Vec<P4> = base
            .iter()
            .map(|&c| add4(rot4(c, row.krot), row.m))
            .collect();
        out.push(TileC { kind: row.kind, cs });
    }
    assert_eq!(out.len(), 26, "SQ_SUP cluster size");
    out
}

/// The hole polygon: the CCW boundary of the shipped cluster — 20 unit
/// edges, exact 2-area (56, 32) (world 4×(14+8√3) = the SQ_SUP
/// supertile identity), 4 reflex 300° vertices.
pub fn blob() -> Vec<P4> {
    let tiles = cluster_tiles();
    let mut ecount: BTreeMap<EK, u32> = BTreeMap::new();
    for t in &tiles {
        let n = t.cs.len();
        for i in 0..n {
            *ecount.entry(edge_key(t.cs[i], t.cs[(i + 1) % n])).or_insert(0) += 1;
        }
    }
    let mut adj: BTreeMap<P4, Vec<P4>> = BTreeMap::new();
    for (&(a, b), &c) in ecount.iter() {
        assert!(c == 1 || c == 2, "cluster not edge-to-edge");
        if c == 1 {
            adj.entry(a).or_default().push(b);
            adj.entry(b).or_default().push(a);
        }
    }
    let start = *adj.keys().next().expect("boundary exists");
    let mut poly = vec![start];
    let mut prev: Option<P4> = None;
    let mut cur = start;
    loop {
        let nxt = *adj[&cur]
            .iter()
            .find(|&&t| Some(t) != prev)
            .expect("boundary cycle");
        if nxt == start {
            break;
        }
        poly.push(nxt);
        prev = Some(cur);
        cur = nxt;
    }
    if sp(poly_area2(&poly)) < 0 {
        poly.reverse();
    }
    assert_eq!(poly_area2(&poly), (56, 32), "blob exact 2-area");
    assert_eq!(poly.len(), 20, "blob edge count");
    poly
}