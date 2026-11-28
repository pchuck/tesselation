//! Landing A pin — D6 polygon-type census for the Uniform +
//! Two-Uniform tagged cards (11 archimedean + 20 kuniform, default
//! preview windows).
//!
//! The union of distinct polygon types across both tabs is exactly
//! five regular polygons — triangle (3), square (4), hexagon (6),
//! octagon (8), dodecagon (12) — keyed by the vertex count of
//! `paint::shape_signature`. Union size 5 ≤ 8, so the palette rule
//! assigns each type a distinct slot (triangle 7, square 5, hexagon 3,
//! octagon 1, dodecagon 0). Raw `shape_class` alone would NOT do:
//! it folds octagon and hexagon both to slot 3 (they merely never
//! co-occur in one view, which is what M215 per-view injectivity
//! certifies — re-verified live here for the tagged subset).

use std::collections::{BTreeMap, BTreeSet};
use tess_core::app::{mean_tile_side, preview_window};
use tess_core::geometry::{BBox, Vec2};
use tess_core::paint::{shape_class, shape_signature};
use tess_core::params::ParamSet;
use tess_core::tiling::{Query, Tiling};
use tess_core::view::ColorMode;

const TAGGED: [&[&str]; 2] = [
    &[
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
    ],
    &[
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
    ],
];

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

fn sig_key(poly: &tess_core::geometry::Polygon) -> String {
    shape_signature(poly)
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(".")
}

#[test]
fn polygon_type_union_is_exactly_five_regular_types() {
    let r = tess_tilings::registry();
    let mut types: BTreeMap<usize, BTreeSet<String>> = BTreeMap::new(); // vertex-count -> sigs
    for group in TAGGED {
        for id in group {
            let t = r.get(id).unwrap_or_else(|| panic!("missing {id}"));
            for tile in preview_tiles(t) {
                let sig = shape_signature(&tile.poly);
                let n = sig.first().copied().unwrap_or(0);
                types.entry(n as usize).or_default().insert(sig_key(&tile.poly));
            }
        }
    }
    let union_size: usize = types.values().map(|s| s.len()).sum();
    assert_eq!(
        types.keys().collect::<Vec<_>>(),
        vec![&3, &4, &6, &8, &12],
        "union vertex-count set must be exactly {{3,4,6,8,12}}; got {types:?}"
    );
    // Each vertex class is a single similarity type (one signature).
    for (n, sigs) in &types {
        assert_eq!(
            sigs.len(),
            1,
            "vertex-count {n} holds {} distinct shapes: {sigs:?}",
            sigs.len()
        );
    }
    assert_eq!(union_size, 5, "D6 union size pinned to 5");
}

#[test]
fn shape_class_is_injective_per_tagged_card_view() {
    // M215 injectivity re-verified live on the tagged subset: within
    // every single card's default view, distinct shape signatures get
    // distinct `shape_class` slots.
    let r = tess_tilings::registry();
    for group in TAGGED {
        for id in group {
            let t = r.get(id).unwrap_or_else(|| panic!("missing {id}"));
            let mut slot_of: BTreeMap<String, usize> = BTreeMap::new();
            for tile in preview_tiles(t) {
                let key = sig_key(&tile.poly);
                let slot = shape_class(&tile.poly);
                if let Some(prev) = slot_of.insert(key.clone(), slot) {
                    assert_eq!(
                        prev, slot,
                        "{id}: shape_class collision {key} ({prev} vs {slot})"
                    );
                }
            }
        }
    }
}

#[test]
fn five_polygon_types_get_five_distinct_colors_across_both_tabs() {
    // The palette rule: one fixed color per polygon type across the
    // Uniform AND Two-Uniform tabs (per mode — each mode shades through
    // its own ramp); the five types pairwise differ within a mode.
    let r = tess_tilings::registry();
    let mut color_of: BTreeMap<(String, usize), BTreeSet<u32>> = BTreeMap::new();
    for group in TAGGED {
        for id in group {
            let t = r.get(id).unwrap_or_else(|| panic!("missing {id}"));
            for tile in preview_tiles(t) {
                let n = *shape_signature(&tile.poly).first().unwrap_or(&0) as usize;
                for mode in [
                    ColorMode::Pattern,
                    ColorMode::Proto,
                    ColorMode::Role,
                    ColorMode::Metatile,
                ] {
                    color_of
                        .entry((mode.label().to_string(), n))
                        .or_default()
                        .insert(tess_core::paint::tile_rgb(mode, &tile));
                }
            }
        }
    }
    for ((mode, n), colors) in &color_of {
        assert_eq!(
            colors.len(),
            1,
            "polygon type {n} in mode {mode} renders {colors:?} (must be ONE color across both tabs)"
        );
    }
    for mode in [
        ColorMode::Pattern,
        ColorMode::Proto,
        ColorMode::Role,
        ColorMode::Metatile,
    ] {
        let key = mode.label().to_string();
        let distinct: BTreeSet<u32> = (3..=12)
            .filter(|n| color_of.contains_key(&(key.clone(), *n)))
            .map(|n| *color_of[&(key.clone(), n)].iter().next().unwrap())
            .collect();
        let types = (3..=12)
            .filter(|n| color_of.contains_key(&(key.clone(), *n)))
            .count();
        assert_eq!(
            distinct.len(),
            types,
            "mode {mode:?}: polygon types must be pairwise distinct colors"
        );
    }
    assert_eq!(color_of.len(), 4 * 5);
}
