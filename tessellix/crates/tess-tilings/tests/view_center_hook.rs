//! Focus: the `Tiling::default_view_center()` hook — the opt-in
//! default-center seed applied at `App::new` and at `reset_view` (never
//! at selection time). It is a provided method on `Tiling` defaulting to
//! `None`: a tiling that does not override it keeps its origin-centered
//! default frame byte-identically to pre-hook behavior.
//!
//! Pins:
//! - the two overriding tilings return the exact pinned values;
//! - every other registered tiling keeps the provided `None` default;
//! - `App::new` seeds from `registry.first()` (single-tiling registry);
//! - `reset_view` re-seeds from the current tiling;
//! - `select_id` never touches the viewport (user pan/zoom preserved).
use tess_core::geometry::Vec2;
use tess_core::{App, Registry};
use tess_tilings::families::quasiperiodic::golden_rhomboid::GoldenRhomboidTiling;
use tess_tilings::families::quasiperiodic::wis_12_fold::Wis12FoldTiling;

/// The two overriding tilings: id → exact hook value.
const OVERRIDES: &[(&str, (f64, f64))] = &[
    ("golden-rhomboid", (5.14, 3.67)),
    ("wis-12-fold", (1.366, 0.0)),
];

#[test]
fn overriding_tilings_return_exact_values() {
    let registry = tess_tilings::registry();
    for &(id, expected) in OVERRIDES {
        let t = registry.get(id).expect("override tiling exists");
        assert_eq!(
            t.default_view_center(),
            Some(expected),
            "{id} must return exactly the pinned center"
        );
    }
}

#[test]
fn every_other_tiling_keeps_the_provided_default() {
    // Byte-identity: the hook is provided-with-default `None`, so any
    // tiling that does not override it must return `None` — the
    // provided default is the whole safety argument.
    let registry = tess_tilings::registry();
    let overrides: Vec<&str> = OVERRIDES.iter().map(|&(id, _)| id).collect();
    let mut overridden = 0usize;
    for t in registry.iter() {
        match t.default_view_center() {
            Some(_) => {
                assert!(
                    overrides.contains(&t.id()),
                    "{} unexpectedly overrides default_view_center",
                    t.id()
                );
                overridden += 1;
            }
            None => {}
        }
    }
    assert_eq!(overridden, OVERRIDES.len(), "registry sweep must see exactly the two overrides");
}

/// `App::new` seeds the viewport from `registry.first()` — build a
/// single-tiling registry so the override is the first tiling.
#[test]
fn app_new_seeds_center_from_hook() {
    let golden = Registry::new().register(GoldenRhomboidTiling);
    let app = App::new(golden, 60.0);
    assert_eq!(app.selected_id(), "golden-rhomboid");
    let c = app.viewport.center;
    assert!((c.x - 5.14).abs() < 1e-12 && (c.y - 3.67).abs() < 1e-12,
        "App::new must seed the golden-rhomboid override, got {:?}", c);

    let wis = Registry::new().register(Wis12FoldTiling);
    let app = App::new(wis, 60.0);
    assert_eq!(app.selected_id(), "wis-12-fold");
    let c = app.viewport.center;
    assert!((c.x - 1.366).abs() < 1e-12 && c.y.abs() < 1e-12,
        "App::new must seed the wis-12-fold override, got {:?}", c);
}

/// A non-overriding tiling keeps the origin-centered viewport at birth
/// and at reset — byte-identical to pre-hook behavior.
#[test]
fn non_overriding_tiling_keeps_origin_centered_viewport() {
    // Registry::first() is not an override; the full-registry birth path
    // therefore starts origin-centered.
    let app = App::new(tess_tilings::registry(), 60.0);
    assert_eq!(app.viewport.center, Vec2::ZERO);
}

#[test]
fn reset_view_reseeds_and_select_id_preserves_pan_zoom() {
    // Full registry: a real overriding tiling selected via select_id.
    let registry = tess_tilings::registry();
    let mut app = App::new(registry, 60.0);
    app.select_id("wis-12-fold");
    // A user pan moves the camera; select_id must NOT snap it back.
    app.viewport.center = Vec2::new(9.0, 9.0);
    app.select_id("wis-12-fold");
    let c = app.viewport.center;
    assert!((c.x - 9.0).abs() < 1e-12 && (c.y - 9.0).abs() < 1e-12,
        "select_id must preserve the user's pan, got {:?}", c);
    // reset_view re-seeds from the current tiling's hook.
    app.reset_view();
    let c = app.viewport.center;
    assert!((c.x - 1.366).abs() < 1e-12 && c.y.abs() < 1e-12,
        "reset_view must re-seed wis-12-fold's override, got {:?}", c);
    // Switch to a non-overriding tiling and reset: back to the origin.
    app.select_id("square");
    app.reset_view();
    assert_eq!(app.viewport.center, Vec2::ZERO,
        "non-overriding tiling resets to the origin (pre-hook behavior)");
}
