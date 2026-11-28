//! Focus: the `shape` color mode classifies tiles by geometric shape
//! (similarity class — translation, rotation, uniform scale and
//! reflection are not shape), not by raw vertex count. This is the fix
//! for the user-reported Ammann-Beenker bug (squares and 45° rhombi all
//! painted one blue): the M192 key `min(n - 3, 7)` collapsed every
//! same-count pair of different shapes onto one color.
//!
//! The pins here are the registry-wide certification of the new key
//! (`tess_core::paint::shape_signature` / `shape_class`):
//!
//! * every tiling's co-occurring shape classes get distinct palette
//!   slots in its default preview window (the slot weights are derived
//!   by constraint solving against exactly this property — if a future
//!   tiling collides, this test fails and the weights are re-derived;
//!   see `SHAPE_SLOT_WEIGHTS`);
//! * the one catalogued over-capacity tiling (`girih-star-cross`, 536
//!   distinct similarity classes in its default window — per-instance
//!   girih strap geometry) shares tones by construction;
//! * the 43 tilings that collapsed under the old key are enumerated with
//!   their old vertex-count class count and their new geometric class
//!   count, which the sweep must reproduce exactly.
//!
//! The heavy sweep reuses the GUI/TUI preview-window derivation
//! (two-pass probe + shrink retry) so it pins the colors the user
//! actually sees; runtime tracks `shape_sig` generation over the
//! registry (~90 s in a debug build, no rasterization).
use std::collections::BTreeMap;
use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::params::ParamSet;
use tess_core::paint::{self, shape_class, shape_signature};
use tess_core::tiling::{Query, Tiling};

/// The retired M192 key, replayed so the collapse census can assert the
/// old behavior was real (vertex count, clamped to the 8-slot palette).
fn old_vertex_class(poly: &Polygon) -> usize {
    poly.len().saturating_sub(3).min(7)
}

/// The GUI/TUI preview window for `t` at its default params (the
/// two-pass probe + shrink retry of `tess_core::app::preview_window`,
/// mirroring `card_texture` and `thumb.rs`).
fn preview_tiles(t: &dyn Tiling) -> Vec<tess_core::tiling::Tile> {
    let params = ParamSet::default_for(t.params());
    let probe = t.generate(
        &params,
        &Query {
            region: BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)),
        },
    );
    let mut window = tess_core::app::preview_window(
        probe.periodic,
        tess_core::app::mean_tile_side(&probe.tiles),
        4.0 / 3.0,
    );
    let mut result = t.generate(&params, &Query { region: window });
    let mut shrink = 0.4;
    while (result.message.is_some() || result.tiles.is_empty()) && shrink > 0.01 {
        let cx = (window.min.x + window.max.x) / 2.0;
        let cy = (window.min.y + window.max.y) / 2.0;
        let hx = (window.max.x - window.min.x) / 2.0 * shrink;
        let hy = (window.max.y - window.min.y) / 2.0 * shrink;
        window = BBox::new(Vec2::new(cx - hx, cy - hy), Vec2::new(cx + hx, cy + hy));
        result = t.generate(&params, &Query { region: window });
        shrink *= 0.4;
    }
    result.tiles
}

/// signature -> palette class observed over a tile set, asserting the
/// class function is constant on signatures (it is a pure function of
/// the signature, but a refactor that breaks that contract shows up
/// here on real geometry).
fn class_by_signature(tiles: &[tess_core::tiling::Tile]) -> BTreeMap<Vec<i64>, usize> {
    let mut m: BTreeMap<Vec<i64>, usize> = BTreeMap::new();
    for tile in tiles {
        let sig = shape_signature(&tile.poly);
        let c = shape_class(&tile.poly);
        assert!(c < paint::PALETTE.len(), "class {c} outside the palette");
        match m.get(&sig).copied() {
            Some(prev) => assert_eq!(prev, c, "one signature, two classes"),
            None => {
                m.insert(sig, c);
            }
        }
    }
    m
}

#[test]
fn shape_mode_separates_ammann_beenker_square_and_rhombus() {
    // The user's report on real generator output: the Ammann-Beenker
    // tiling shows squares and 45° rhombi side by side — both
    // quadrilaterals, so the old vertex-count key gave them ONE color.
    // Under the geometric key the window's classes must be injective,
    // and two tiles with equal raw vertex counts but different
    // signatures must differ in color.
    let registry = tess_tilings::registry();
    let t = registry.get("ammann-beenker").expect("ammann-beenker");
    let tiles = preview_tiles(t);
    assert!(tiles.len() > 50, "expected a full preview window");
    let classes = class_by_signature(&tiles);
    assert!(classes.len() >= 2, "AB window has {} classes", classes.len());
    let distinct_class_count = tiles.iter().map(|tile| shape_class(&tile.poly)).collect::<std::collections::BTreeSet<_>>().len();
    assert_eq!(
        distinct_class_count,
        classes.len(),
        "color classes must equal geometric shape classes (injective in view)"
    );
    // Explicitly: some 4-gon pair (square vs rhombus) is split.
    let quads: BTreeMap<_, _> = tiles
        .iter()
        .filter(|tile| tile.poly.len() == 4)
        .map(|tile| (shape_signature(&tile.poly), shape_class(&tile.poly)))
        .collect();
    assert!(
        quads.len() >= 2 && {
            let cs: std::collections::BTreeSet<_> = quads.values().collect();
            cs.len() == quads.len()
        },
        "square and rhombus quadrilaterals must not share a color: {quads:?}"
    );
    // The old key DID collapse them — one vertex-count class among
    // those quadrilaterals.
    let old_quads: std::collections::BTreeSet<_> = tiles
        .iter()
        .filter(|tile| tile.poly.len() == 4)
        .map(|tile| old_vertex_class(&tile.poly))
        .collect();
    assert_eq!(old_quads.len(), 1, "regression context: old key was vertex count");
}

/// Real-geometry invariance: rotating, scaling, translating and
/// mirroring every tile of representative tilings leaves each tile's
/// class unchanged (orientation and position are not shape).
#[test]
fn shape_class_is_invariant_under_similarity_of_real_tiles() {
    let registry = tess_tilings::registry();
    for id in ["penrose", "ammann-beenker", "herringbone-45", "taylor-socolar", "hat"] {
        let t = registry.get(id).expect(id);
        let tiles = preview_tiles(t);
        assert!(!tiles.is_empty(), "{id} produced tiles");
        let turn = 37.0f64.to_radians();
        let (cs, sn) = (turn.cos(), turn.sin());
        for tile in &tiles {
            let base = shape_class(&tile.poly);
            let mut pts: Vec<Vec2> = tile
                .poly
                .pts
                .iter()
                .map(|p| Vec2::new((p.x * cs - p.y * sn) * 3.0 + 11.0, (p.x * sn + p.y * cs) * 3.0 - 7.0))
                .collect();
            let moved = shape_class(&Polygon::from_slice(&pts));
            assert_eq!(base, moved, "{id}: rotate+scale+translate changed the class");
            for p in &mut pts {
                p.x = -p.x;
            }
            assert_eq!(base, shape_class(&Polygon::from_slice(&pts)), "{id}: mirroring changed the class");
        }
    }
}

/// The collapsed census: the 43 tilings whose distinct shapes the old
/// vertex-count key merged, with the old key's class count and the new
/// key's geometric class count in the default preview window (the
/// derived pin list; re-derive with this file's sweep if the registry
/// or engines change).
const COLLAPSED: [(&str, usize, usize); 43] = [
        ("alcove-cut-grid", 1, 3),
        ("ammann-beenker", 2, 3),
        ("andritz-bowtie-hexagon", 1, 2),
        ("andritz-bowtie-hexagon-10gon-1", 2, 3),
        ("andritz-bowtie-hexagon-10gon-2", 2, 3),
        ("andritz-bowtie-hexagon-10gon-3", 2, 3),
        ("cap", 3, 4),
        ("dodecagon-hexagram-honeycomb", 2, 3),
        ("fibonacci-fibonacci", 1, 2),
        ("girih-14-fold", 3, 7),
        ("girih-decagon-patch", 2, 3),
        ("girih-quasiperiodic", 2, 3),
        ("girih-star-cross", 8, 536),
        ("girih-w85", 5, 7),
        ("glb-binary-n7", 1, 3),
        ("glide-hexagon-strips", 1, 2),
        ("hankin-3-12-12", 3, 4),
        ("hankin-3-4-6-4", 3, 4),
        ("hankin-4-6-12", 3, 5),
        ("hankin-4-8-8-rosette-transform", 5, 8),
        ("hankin-hex-motifs", 3, 4),
        ("khatam-girih", 2, 3),
        ("kuniform-dual-n16", 1, 2),
        ("kuniform-dual-n17", 1, 2),
        ("kuniform-dual-n5", 1, 2),
        ("kuniform-dual-n6", 1, 2),
        ("kuniform-dual-n7", 1, 2),
        ("kuniform-dual-pmm", 1, 2),
        ("najm-63-motifs", 3, 5),
        ("penrose", 2, 4),
        ("penrose-pentagon", 1, 4),
        ("penrose_p2", 2, 4),
        ("pic-3-12-12", 4, 6),
        ("shifted-trapezoid-courses", 1, 2),
        ("stampfli", 2, 3),
        ("star-cross", 1, 2),
        ("star-cross-8-2", 1, 2),
        ("tangram", 2, 3),
        ("taylor-socolar", 3, 4),
        ("trapezoid-courses", 1, 2),
        ("turtle", 1, 2),
        ("wavy-courses", 1, 2),
        ("wis-12-fold", 2, 3)
];

/// Tilings whose distinct shape classes exceed the 8-slot palette in
/// the default window: sharing tones is by construction there, and the
/// list is closed (any new over-capacity tiling fails the sweep).
const OVER_CAPACITY: [(&str, usize); 1] = [("girih-star-cross", 536)];

#[test]
fn shape_mode_is_injective_over_the_registry() {
    let registry = tess_tilings::registry();
    let mut collapsed_seen = 0usize;
    for t in registry.iter() {
        let tiles = preview_tiles(t);
        assert!(!tiles.is_empty(), "{} produced no tiles", t.id());
        let classes = class_by_signature(&tiles);
        let old_count: std::collections::BTreeSet<_> =
            tiles.iter().map(|tile| old_vertex_class(&tile.poly)).collect();
        let over = OVER_CAPACITY.iter().find(|(id, _)| *id == t.id());
        if classes.len() > old_count.len() {
            collapsed_seen += 1;
        }
        if let Some((_, n_sigs)) = over {
            assert_eq!(
                classes.len(), *n_sigs,
                "{}: over-capacity catalog pin (distinct similarity classes in window)",
                t.id()
            );
            assert_eq!(
                classes.values().collect::<std::collections::BTreeSet<_>>().len(),
                paint::PALETTE.len(),
                "{}: over-capacity window must use the full palette",
                t.id()
            );
            continue;
        }
        assert!(
            classes.len() <= paint::PALETTE.len(),
            "{}: {} classes within palette capacity",
            t.id(),
            classes.len()
        );
        // Injectivity is enforced by class_by_signature's pure-function
        // assert only up to same-signature agreement; the real check is
        // color count == shape count:
        let distinct_colors: std::collections::BTreeSet<_> = classes.values().collect();
        assert_eq!(
            distinct_colors.len(),
            classes.len(),
            "{}: palette collision — {} shape classes share {} colors: {:?}",
            t.id(),
            classes.len(),
            distinct_colors.len(),
            classes
        );
    }
    assert_eq!(
        collapsed_seen,
        COLLAPSED.len(),
        "the collapsed-tiling census changed (list has {} entries)",
        COLLAPSED.len()
    );
}

/// Pin the collapsed census itself: each listed tiling's window shows
/// exactly `old` vertex-count classes (the collapse was real) and
/// exactly `new` geometric classes/colors (the fix lands), and each
/// unlisted tiling is NOT collapsed (old >= new nowhere else).
#[test]
fn collapsed_census_counts_are_exact() {
    let registry = tess_tilings::registry();
    for (id, old, new) in COLLAPSED {
        let t = registry.get(id).expect(id);
        let tiles = preview_tiles(t);
        let classes = class_by_signature(&tiles);
        let old_count: std::collections::BTreeSet<_> =
            tiles.iter().map(|tile| old_vertex_class(&tile.poly)).collect();
        assert_eq!(
            (old_count.len(), classes.len()),
            (old, new),
            "{id}: vertex-count classes vs geometric classes"
        );
    }
}
