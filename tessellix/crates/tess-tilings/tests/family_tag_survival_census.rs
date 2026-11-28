//! Landing A pin — the Tile family/reflected contract survives every
//! generator, the tagged-card set is exactly pinned, and `mirror_tiles`
//! forwards both new fields.
//!
//! Tagged set (44 cards, derived programmatically from the registry at
//! Landing A time): 4 Penrose + 4 Andritz + 11 Uniform (archimedean) +
//! 20 Two-Uniform (kuniform) + 5 Isohedral gallery cards. Kuniform
//! *duals* register under the Duals-&-variants tab (category Laves) and
//! are untagged by contract (SPEC §polygon-type rule: "both duals of
//! the triangle/square/hexagon are in Laves, not these tabs").

use tess_core::app::{mean_tile_side, preview_window};
use tess_core::geometry::{BBox, Vec2};
use tess_core::params::ParamSet;
use tess_core::tiling::{FamilyTag, Query, Tiling};

const PENROSE: [&str; 4] = ["penrose", "penrose-p1", "penrose_p2", "penrose-pentagon"];
const ANDRITZ: [&str; 4] = [
    "andritz-bowtie-hexagon",
    "andritz-bowtie-hexagon-10gon-1",
    "andritz-bowtie-hexagon-10gon-2",
    "andritz-bowtie-hexagon-10gon-3",
];
const UNIFORM: [&str; 11] = [
    "square",
    "hexagon",
    "triangle",
    "truncated-square",
    "trihexagonal",
    "snub-trihexagonal",
    "elongated-triangular",
    "rhombitrihexagonal",
    "truncated-trihexagonal",
    "truncated-hexagonal",
    "snub-square",
];
const TWO_UNIFORM: [&str; 20] = [
    "kuniform-p6m",
    "kuniform-pmm",
    "kuniform-n18",
    "kuniform-n9",
    "kuniform-n8",
    "kuniform-n5",
    "kuniform-n1",
    "kuniform-n13",
    "kuniform-n20",
    "kuniform-n12",
    "kuniform-n19",
    "kuniform-n4",
    "kuniform-n14",
    "kuniform-n3",
    "kuniform-n15",
    "kuniform-n2",
    "kuniform-n16",
    "kuniform-n17",
    "kuniform-n7",
    "kuniform-n6",
];
const ISOHEDRAL: [&str; 5] = [
    "ih1-p1-hexagon",
    "ih6-pgg-hexagon",
    "ih7-p3-hexagon",
    "ih27-pgg-pentagon",
    "ih29-p4g-pentagon",
];

fn expected_family(id: &str) -> FamilyTag {
    if PENROSE.contains(&id) {
        FamilyTag::Penrose
    } else if ANDRITZ.contains(&id) {
        FamilyTag::Andritz
    } else if UNIFORM.contains(&id) {
        FamilyTag::Uniform
    } else if TWO_UNIFORM.contains(&id) {
        FamilyTag::TwoUniform
    } else if ISOHEDRAL.contains(&id) {
        FamilyTag::Isohedral
    } else {
        FamilyTag::None
    }
}

fn preview_tiles(t: &dyn Tiling) -> Vec<tess_core::tiling::Tile> {
    let params = ParamSet::default_for(t.params());
    let probe = t.generate(
        &params,
        &Query {
            region: BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)),
        },
    );
    let mut window = preview_window(probe.periodic, mean_tile_side(&probe.tiles), 4.0 / 3.0);
    let mut result = t.generate(
        &params,
        &Query {
            region: window,
        },
    );
    let mut shrink = 0.4;
    while (result.message.is_some() || result.tiles.is_empty()) && shrink > 0.01 {
        let cx = (window.min.x + window.max.x) / 2.0;
        let cy = (window.min.y + window.max.y) / 2.0;
        let hx = (window.max.x - window.min.x) / 2.0 * shrink;
        let hy = (window.max.y - window.min.y) / 2.0 * shrink;
        window = BBox::new(Vec2::new(cx - hx, cy - hy), Vec2::new(cx + hx, cy + hy));
        result = t.generate(
            &params,
            &Query {
                region: window,
            },
        );
        shrink *= 0.4;
    }
    result.tiles
}

/// Every registered card's emitted tiles carry exactly the expected
/// family tag; `reflected` is `false` at emission everywhere.
#[test]
fn family_tag_survives_every_generator() {
    let r = tess_tilings::registry();
    assert_eq!(r.len(), 170, "registry size pin (Landing A base)");
    let mut tagged_cards = 0;
    for t in r.iter() {
        let want = expected_family(t.id());
        if want != FamilyTag::None {
            tagged_cards += 1;
        }
        let tiles = preview_tiles(t);
        assert!(!tiles.is_empty(), "{}: no tiles", t.id());
        for (i, tile) in tiles.iter().enumerate() {
            assert_eq!(
                tile.family, want,
                "{}: tile {i} carries {:?}, expected {want:?}",
                t.id(),
                tile.family
            );
            assert!(
                !tile.reflected,
                "{}: emitted tile {i} must not be reflected",
                t.id()
            );
        }
    }
    assert_eq!(tagged_cards, 44, "exactly 44 family-tagged cards");
}

// `mirror_tiles` field forwarding is pinned in tess-core
// (`svg::tests::mirror_tiles_forwards_family_and_flips_reflected` — the flip
// is an involution: canonical ↔ reflected across the mirror pass);
// the private helper is exercised there, and `mirror_export.rs` keeps
// covering the end-to-end mirror render.
