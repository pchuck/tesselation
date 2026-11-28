//! b2b4-hazard-verify HAZARD 1 pin — registry tilings whose card-preview
//! tiles classify to index 8, the one-past-the-end index of the 8-entry
//! identity palette the GUI card path passes to `raster`.
//!
//! The full b2b4-recon probe walks every registered tiling through the
//! exact GUI card pipeline (`card_texture`: probe region,
//! `preview_window`, shrink-and-retry, then a per-tile class lookup).
//! Of the 169 registered tilings, 42 produce a `Metatile` index-8 tile
//! and 1 (`golden-bee`) a `Level` index-8 tile — so the hazard is
//! reachable through the production GUI card path, not theoretical.
//! This pin is intentionally small (a handful of named tilings) so it
//! is not brittle against unrelated registry additions.

use tess_core::app::{mean_tile_side, preview_window};
use tess_core::geometry::{BBox, Vec2};
use tess_core::params::{ParamSet, ParamValue};
use tess_core::tiling::Query;
use tess_core::view::ColorMode;

const GLB_BINARY_N7_ID: &str = "glb-binary-n7";

/// Faithful copy of the GUI card-preview window derivation
/// (`tess-gui/src/ui.rs::card_texture` — probe ±10, `preview_window`,
/// shrink-and-retry on refusal).
fn card_tiles(id: &str) -> Vec<tess_core::tiling::Tile> {
    let registry = tess_tilings::registry();
    let tiling = registry.get(id).expect("id registered");
    let params = ParamSet::default_for(tiling.params());
    let preview_params = if tiling.id() == GLB_BINARY_N7_ID {
        let mut p = params.clone();
        p.set(
            tess_tilings::families::quasiperiodic::glb_binary_n7::DEPTH_ID,
            ParamValue::Int(3),
        );
        p
    } else {
        params.clone()
    };
    let probe = tiling.generate(
        &params,
        &Query { region: BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)) },
    );
    let mut window = preview_window(
        probe.periodic,
        mean_tile_side(&probe.tiles),
        120.0 / 90.0,
    );
    let mut result = tiling.generate(&preview_params, &Query { region: window });
    let mut shrink = 0.4;
    while result.message.is_some() && shrink > 0.05 {
        let cx = (window.min.x + window.max.x) / 2.0;
        let cy = (window.min.y + window.max.y) / 2.0;
        let hx = (window.max.x - window.min.x) / 2.0 * shrink;
        let hy = (window.max.y - window.min.y) / 2.0 * shrink;
        window = BBox::new(Vec2::new(cx - hx, cy - hy), Vec2::new(cx + hx, cy + hy));
        result = tiling.generate(&preview_params, &Query { region: window });
        shrink *= 0.4;
    }
    assert!(!result.tiles.is_empty(), "{id} produced tiles");
    result.tiles
}

fn index_8_count(id: &str, mode: ColorMode) -> usize {
    card_tiles(id)
        .iter()
        .filter(|t| match mode {
            ColorMode::Metatile => tess_core::paint::metatile_class(t),
            ColorMode::Level => tess_core::paint::level_class(t),
            _ => 0,
        } == 8)
        .count()
}

/// A registry tiling whose card preview reaches Metatile index 8.
/// (The probe's full list — 42 tilings, 10–645 index-8 tiles per
/// tiling — includes every one of these; pin a small subset so the
/// test is stable against registry growth.)
#[test]
fn metatile_index_8_is_reachable_through_the_card_pipeline() {
    for id in ["snub-square", "snub-trihexagonal", "kuniform-n9", "basketweave-3"] {
        assert!(
            index_8_count(id, ColorMode::Metatile) > 0,
            "{id}: expected ≥1 Metatile index-8 tile in the card pipeline"
        );
    }
}

/// The only registered tiling whose card preview reaches Level index 8.
#[test]
fn level_index_8_is_reachable_through_the_card_pipeline() {
    assert!(
        index_8_count("golden-bee", ColorMode::Level) > 0,
        "golden-bee: expected ≥1 Level index-8 tile in the card pipeline"
    );
}
