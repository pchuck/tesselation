//! Focus: the `orientation` color mode separates the two orientations
//! of the basketweave and herringbone pavers (and the 45° diagonal
//! diagonal pair). This is the mode's reason to exist — the brief pins
//! the separation property rather than "it compiles".
use std::collections::BTreeSet;
use tess_core::geometry::{BBox, Vec2};
use tess_core::params::ParamSet;
use tess_core::tiling::Query;
use tess_core::paint::orientation_class;

fn orientation_buckets(id: &str) -> BTreeSet<usize> {
    let registry = tess_tilings::registry();
    let t = registry.get(id).expect("tiling exists");
    let params = ParamSet::default_for(t.params());
    let region = BBox::new(Vec2::new(-4.0, -4.0), Vec2::new(4.0, 4.0));
    let result = t.generate(&params, &Query { region });
    assert!(!result.tiles.is_empty(), "{id} produced tiles");
    result
        .tiles
        .iter()
        .map(|tile| orientation_class(&tile.poly))
        .collect()
}

#[test]
fn orientation_mode_separates_paver_orientations() {
    // Basketweave and herringbone are axis-aligned (0°/90°): both
    // orientations must appear as distinct buckets, and neither the
    // 45° diagonal nor its antipode may appear.
    for id in ["basketweave", "herringbone"] {
        let buckets = orientation_buckets(id);
        assert!(
            buckets.len() >= 2
                && buckets.contains(&0)
                && buckets.contains(&2)
                && buckets.iter().all(|&b| b == 0 || b == 2),
            "{id}: must separate the two axis-aligned orientations into the 0°/90° buckets, got {buckets:?}"
        );
    }
    // herringbone-45 is the 45°-rotated variant: the axis-aligned pair
    // must be absent and the diagonal pair must appear instead.
    let buckets = orientation_buckets("herringbone-45");
    assert!(
        buckets.len() >= 2
            && buckets.contains(&1)
            && buckets.contains(&3)
            && buckets.iter().all(|&b| b == 1 || b == 3),
        "herringbone-45: must separate the two diagonal orientations into the 45°/135° buckets, got {buckets:?}"
    );
}
