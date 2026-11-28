//! Landing A pin — per-family palette rules.
//!
//! * Penrose (4 cards) and Andritz (4 cards): the color is a FUNCTION
//!   of the polygon identity (shape signature), never of the card —
//!   the same polygon type renders one color across all cards of its
//!   family. The one genuinely shared shape type across Penrose cards
//!   is the thin rhomb (P3 rhombs `penrose`, P1 diamond in
//!   `penrose-p1`); the pinned per-card proto→slot tables (see
//!   `paint::family_slot`) give it one color. Andritz's regular
//!   hexagon also reads the Uniform-tabs hexagon color.
//! * M192 sync: `paint::tile_rgb` and raster's `color_index` agree —
//!   for tagged tiles the raster slot at the tile centroid equals the
//!   `paint::family_slot` the color function used.
//! * The family rule touches only the four identity modes; the other
//!   six modes never consult it (`paint::family_slot` returns `None`).

use std::collections::{BTreeMap, BTreeSet};
use tess_core::app::{mean_tile_side, preview_window};
use tess_core::geometry::{BBox, Vec2};
use tess_core::paint::{family_slot, shape_signature};
use tess_core::params::ParamSet;
use tess_core::raster;
use tess_core::tiling::{Query, Tiling};
use tess_core::view::ColorMode;

const IDENTITY_MODES: [ColorMode; 4] = [
    ColorMode::Pattern,
    ColorMode::Proto,
    ColorMode::Role,
    ColorMode::Metatile,
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

/// Within each family, (polygon signature) -> color is a function
/// across ALL cards of that family, and each card view shows pairwise
/// distinct colors for pairwise distinct signatures.
fn check_family_functional(ids: &[&str]) {
    let r = tess_tilings::registry();
    for mode in IDENTITY_MODES {
        let mut color_of: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
        for id in ids {
            let t = r.get(id).unwrap_or_else(|| panic!("missing {id}"));
            let tiles = preview_tiles(t);
            let mut card_seen: BTreeMap<String, (u8, u32)> = BTreeMap::new();
            for tile in &tiles {
                let key = sig_key(&tile.poly);
                let c = tess_core::paint::tile_rgb(mode, tile);
                color_of.entry(key.clone()).or_default().insert(c);
                if let Some(prev) = card_seen.insert(key.clone(), (tile.proto, c)) {
                    assert_eq!(prev.1, c, "{id}/{mode:?}: unstable color for {key}");
                }
            }
            // Within one card view, two DISTINCT signatures may share
            // a color only when they belong to the same proto; a single
            // signature under two protos (the mirrored pentagon pair)
            // sharing one color is the point of the rule, not a defect.
            let by_color: BTreeMap<u32, BTreeSet<(u8, String)>> =
                card_seen.iter().fold(BTreeMap::new(), |mut m, (k, (p, c))| {
                    m.entry(*c).or_default().insert((*p, k.clone()));
                    m
                });
            for (c, keys) in &by_color {
                let protos: BTreeSet<u8> = keys.iter().map(|(p, _)| *p).collect();
                let sigs: BTreeSet<&String> = keys.iter().map(|(_, s)| s).collect();
                assert!(
                    protos.len() <= 1 || sigs.len() == 1,
                    "{id}/{mode:?}: color {c} shared by distinct polygon types {keys:?}"
                );
            }
        }
        for (sig, colors) in &color_of {
            assert_eq!(
                colors.len(),
                1,
                "{mode:?}: polygon type {sig} renders {colors:?} across family cards (must be one color)"
            );
        }
    }
}

#[test]
fn penrose_palette_is_one_function_of_polygon_type() {
    check_family_functional(&[
        "penrose",
        "penrose-p1",
        "penrose_p2",
        "penrose-pentagon",
    ]);
}

#[test]
fn andritz_palette_is_one_function_of_polygon_type() {
    check_family_functional(&[
        "andritz-bowtie-hexagon",
        "andritz-bowtie-hexagon-10gon-1",
        "andritz-bowtie-hexagon-10gon-2",
        "andritz-bowtie-hexagon-10gon-3",
    ]);
}

/// The thin rhomb keeps one color across `penrose` (proto 1, 4-gon)
/// and `penrose-p1` (diamond, proto 3): the explicit cross-card pin of
/// the shared polygon type.
#[test]
fn thin_rhomb_color_matches_across_penrose_cards() {
    let r = tess_tilings::registry();
    let find_rhomb = |id: &str, proto: u8| -> Option<(String, u32)> {
        preview_tiles(r.get(id).unwrap())
            .iter()
            .find(|t| t.proto == proto && shape_signature(&t.poly).first() == Some(&4))
            .map(|t| (sig_key(&t.poly), tess_core::paint::tile_rgb(ColorMode::Proto, t)))
    };
    let a = find_rhomb("penrose", 1).expect("penrose thin rhomb");
    let b = find_rhomb("penrose-p1", 3).expect("penrose-p1 diamond");
    assert_eq!(a.0, b.0, "thin rhomb and P1 diamond are one shape type");
    assert_eq!(a.1, b.1, "shared shape type must share its color");
}

/// M192 sync: raster's private `color_index` agrees with
/// `paint::tile_rgb` on the family slot — rasterized through an
/// 8-entry identity ramp, every non-boundary pixel of a single-tile
/// render equals that tile's `paint::family_slot` (per-shape
/// representatives, so non-convex geometry cannot fake a pass).
#[test]
fn raster_and_tile_rgb_agree_on_family_slot() {
    let r = tess_tilings::registry();
    let palette: Vec<u8> = (0..8u8).collect();
    for id in [
        "penrose",
        "penrose-p1",
        "penrose_p2",
        "penrose-pentagon",
        "andritz-bowtie-hexagon",
        "andritz-bowtie-hexagon-10gon-1",
        "square",
        "truncated-hexagonal",
        "kuniform-p6m",
        "kuniform-n5",
    ] {
        let t = r.get(id).unwrap_or_else(|| panic!("missing {id}"));
        let tiles = preview_tiles(t);
        assert!(!tiles.is_empty());
        // One representative tile per (proto, signature).
        let mut reps: BTreeMap<(u8, String), tess_core::tiling::Tile> = BTreeMap::new();
        for tile in &tiles {
            reps.entry((tile.proto, sig_key(&tile.poly)))
                .or_insert_with(|| tile.clone());
        }
        for mode in IDENTITY_MODES {
            let mut colored_total = 0;
            for ((proto, sig), tile) in &reps {
                let want = family_slot(mode, tile).expect("tagged tile has a family slot");
                let ab = tile.poly.aabb();
                let padx = ((ab.max.x - ab.min.x) * 0.1).max(1e-6);
                let pady = ((ab.max.y - ab.min.y) * 0.1).max(1e-6);
                let bb = BBox::new(
                    Vec2::new(ab.min.x - padx, ab.min.y - pady),
                    Vec2::new(ab.max.x + padx, ab.max.y + pady),
                );
                let out = raster::raster(std::slice::from_ref(tile), &bb, 33, 33, mode, &palette);
                for v in &out.pixels {
                    if *v == raster::BORDER || *v == raster::GAP {
                        continue; // boundary cell: index not visible
                    }
                    assert_eq!(
                        *v as usize, want,
                        "{id}/{mode:?}: raster slot {v} != family slot {want} \
                         for single-tile render of proto {proto} sig {sig}"
                    );
                    colored_total += 1;
                }
            }
            assert!(colored_total > 0, "{id}/{mode:?}: nothing colored");
        }
    }
}

/// The rule consults only the identity modes: everywhere else
/// `family_slot` is `None` even for tagged tiles.
#[test]
fn family_rule_only_covers_identity_modes() {
    let r = tess_tilings::registry();
    let tiles = preview_tiles(r.get("square").unwrap());
    for tile in &tiles {
        for mode in ColorMode::ALL {
            let got = family_slot(mode, tile);
            if IDENTITY_MODES.contains(&mode) {
                assert!(got.is_some(), "{mode:?} must consult the family rule");
            } else {
                assert!(got.is_none(), "{mode:?} must NOT consult the family rule");
            }
        }
    }
}
