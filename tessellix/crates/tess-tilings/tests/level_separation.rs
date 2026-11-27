//! Focus: the `level` color mode separates hierarchy from flatness.
//! The mode's reason to exist (E16: "fractalizing 3.12.12 — hierarchy
//! invisible") is that `Tile::level` carries real substitution depth at
//! emission. The shipped engines are uniform-depth per view (one
//! expansion depth is selected from the view and every tile of one
//! `generate` shares it), except the golden bee / golden triangle
//! irreptiles, whose children descend by 1 or 2 scale steps and so
//! carry genuinely mixed per-tile lineage depths within one view.
//! This test pins all three observable behaviors programmatically:
//! mixed per-tile depth in the irreptiles, exactly-neutral flatness for
//! non-substitution engines, and depth steps along a zoom series.
use std::collections::BTreeSet;
use tess_core::geometry::{BBox, Vec2};
use tess_core::params::ParamSet;
use tess_core::paint::level_class;
use tess_core::tiling::Query;

/// The level buckets (paint-visible classes) of a tiling's default
/// view.
fn level_buckets(id: &str) -> BTreeSet<usize> {
    let registry = tess_tilings::registry();
    let t = registry.get(id).expect("tiling exists");
    let params = ParamSet::default_for(t.params());
    let region = BBox::new(Vec2::new(-4.0, -4.0), Vec2::new(4.0, 4.0));
    let result = t.generate(&params, &Query { region });
    assert!(!result.tiles.is_empty(), "{id} produced tiles");
    result
        .tiles
        .iter()
        .map(|tile| level_class(tile))
        .collect()
}

/// The irreptile engines must show nested hierarchy in a single
/// default view: children descend by 1 or 2 scale steps, so at the
/// presentation floor D_PRESENT = 8 the emitted per-tile lineage
/// depths span 8..=16 — mapped through the 9-entry level ramp that is
/// at least 3 distinct buckets, and never the neutral class 0.
#[test]
fn level_mode_separates_irreptile_lineage_in_one_view() {
    for id in ["golden-bee", "golden-triangle"] {
        let buckets = level_buckets(id);
        assert!(
            buckets.len() >= 3,
            "{id}: a mixed-size irreptile view must show at least 3 level buckets, got {buckets:?}"
        );
        assert!(
            !buckets.contains(&0),
            "{id}: hierarchical tiles must not fall in the neutral bucket, got {buckets:?}"
        );
    }
    // golden bee: the exact lineage-depth window. Depth 8 at the
    // default view, each round adds 1 (large child) or 2 (small
    // child) scale steps → levels 8..=16, all nine classes present.
    let registry = tess_tilings::registry();
    let t = registry.get("golden-bee").expect("golden-bee exists");
    let params = ParamSet::default_for(t.params());
    let result = t.generate(
        &params,
        &Query {
            region: BBox::new(Vec2::new(-4.0, -4.0), Vec2::new(4.0, 4.0)),
        },
    );
    let raw: BTreeSet<u8> = result.tiles.iter().map(|tile| tile.level).collect();
    assert_eq!(
        raw,
        (8u8..=16).collect::<BTreeSet<u8>>(),
        "golden-bee: default view must emit the full lineage window 8..=16"
    );
}

/// Non-substitution engines know no depth: every tile is level 0 and
/// the whole view is exactly the neutral bucket.
#[test]
fn level_mode_is_neutral_for_non_hierarchical_tilings() {
    for id in ["square", "hexagon", "basketweave"] {
        let buckets = level_buckets(id);
        assert_eq!(
            buckets,
            BTreeSet::from([0]),
            "{id}: a non-hierarchical tiling must occupy exactly the neutral level bucket"
        );
    }
}

/// The uniform-depth engines carry their hierarchy as the patch's
/// position in the family: every view is internally level-uniform, and
/// the level steps as the view crosses substitution octaves. This is
/// the E16 behavior — zooming the fractalizing family makes the depth
/// visible as a color change.
#[test]
fn fractalizing_level_steps_across_zoom_octaves() {
    let registry = tess_tilings::registry();
    let t = registry.get("fractalizing-3-12-12").expect("tiling exists");
    let params = ParamSet::default_for(t.params());
    // View half-extents shrinking past the SPAN0 floor, one per p-octave.
    let mut seen = BTreeSet::new();
    for half in [4.0f64, 0.7, 0.12, 0.02, 0.003] {
        let region = BBox::new(Vec2::new(-half, -half), Vec2::new(half, half));
        let result = t.generate(&params, &Query { region });
        assert!(!result.tiles.is_empty(), "fractalizing view {half} produced tiles");
        assert!(result.message.is_none(), "fractalizing view {half}: {:?}", result.message);
        // Uniform depth per view: one engine depth for all tiles.
        let view_levels: BTreeSet<u8> = result.tiles.iter().map(|tile| tile.level).collect();
        assert_eq!(
            view_levels.len(),
            1,
            "fractalizing view {half} must be level-uniform, got {view_levels:?}"
        );
        seen.insert(view_levels.iter().next().copied().unwrap());
    }
    assert!(
        seen.len() >= 3,
        "zooming across octaves must walk at least 3 hierarchy levels, got {seen:?}"
    );
    // Strictly increasing with zoom-in (the depth selection is
    // monotone in the view diagonal).
    let mut prev = None;
    for half in [4.0f64, 0.7, 0.12, 0.02, 0.003] {
        let region = BBox::new(Vec2::new(-half, -half), Vec2::new(half, half));
        let result = t.generate(&params, &Query { region });
        let l = result.tiles[0].level;
        if let Some(p) = prev {
            assert!(l > p, "deeper view {half} must show a deeper level ({p} -> {l})");
        }
        prev = Some(l);
    }
}
