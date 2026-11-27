//! Focus: the supertile-highlight export pass
//! (`tess_core::svg::render_svg_with_walls_highlight`) driven through
//! the registry in the exact view the CLI export renders by default
//! (1200×800, scale 1, rotation 0 → a 30×20 world region centered at
//! the origin). Pins the two regimes the shipped engines fall into:
//! flat tilings (every tile level 0 — the flag is a byte-for-byte
//! no-op, one base-outline group) and uniform-depth supertile views
//! (every tile level > 0 — the on-state is the off-state bytes with
//! exactly the outline stroke doubled, one group, and nothing else
//! moving).
use tess_core::geometry::{BBox, Vec2};
use tess_core::params::{ParamSet, ParamValue};
use tess_core::tiling::{Query, Tile};
use tess_core::view::ColorMode;

/// Base / heavy outline stroke at the default view's 30×20 extent:
/// `0.004·min(30, 20)` formatted `{:.4}`, heavy = 2× base.
const BASE_W: &str = r#"stroke-width="0.0800""#;
const HEAVY_W: &str = r#"stroke-width="0.1600""#;

/// The default export view (what `tess-tui export --tiling ID` renders
/// with no `--width`/`--height`/`--scale`/`--rotation`).
fn region() -> BBox {
    BBox::new(Vec2::new(-15.0, -10.0), Vec2::new(15.0, 10.0))
}

/// The tiles of one tiling's default export view.
fn view_of(id: &str) -> Vec<Tile> {
    let registry = tess_tilings::registry();
    let tiling = registry
        .get(id)
        .unwrap_or_else(|| panic!("{id} registered"));
    let mut params = ParamSet::default_for(tiling.params());
    params.set(tess_tilings::lattice::SCALE_ID, ParamValue::Float(1.0));
    params.set(
        tess_tilings::lattice::ROTATION_ID,
        ParamValue::Angle(0.0),
    );
    let result = tiling.generate(&params, &Query { region: region() });
    assert!(
        result.message.is_none(),
        "{id} refused the default view: {:?}",
        result.message
    );
    result.tiles
}

/// Tiles with at least three vertices — the ones that emit a `<path>`
/// in both the fill and outline passes.
fn non_degenerate(tiles: &[Tile]) -> usize {
    tiles.iter().filter(|t| t.poly.pts.len() >= 3).count()
}

/// The `<g fill="none" ...>` blocks (the outline groups — this test
/// passes no walls, so no wall group), as (start, end) byte ranges in
/// document order.
fn outline_group_spans(svg: &str) -> Vec<(usize, usize)> {
    svg.match_indices("<g fill=\"none\"")
        .map(|(i, _)| {
            let end = i + svg[i..].find("</g>").expect("outline group is closed");
            (i, end + "</g>".len())
        })
        .collect()
}

/// The fill pass: everything before the first outline group.
fn fill_prefix(svg: &str) -> &str {
    let cut = svg.find("<g fill=\"none\"").expect("outline group present");
    &svg[..cut]
}

/// Flat engines (every tile level 0): the highlight flag must be a
/// byte-for-byte no-op, and the on-state keeps exactly one (base)
/// outline group.
#[test]
fn flat_views_highlight_is_byte_identical() {
    for id in ["square", "hexagon", "fractalizing-3-12-12"] {
        let tiles = view_of(id);
        assert!(!tiles.is_empty(), "{id} produced tiles");
        assert!(
            tiles.iter().all(|t| t.level == 0),
            "{id} must be flat (all level 0) for this test"
        );
        let off = tess_core::svg::render_svg_with_walls(&tiles, &region(), ColorMode::Pattern, &[]);
        let on = tess_core::svg::render_svg_with_walls_highlight(&tiles, &region(), ColorMode::Pattern, &[], true);
        assert_eq!(on, off, "{id}: flat tiling, highlight must be a byte-for-byte no-op");
        assert_eq!(
            outline_group_spans(&on).len(),
            1,
            "{id}: one base outline group in the on state"
        );
        assert!(on.contains(BASE_W), "{id}: base stroke present");
        assert!(!on.contains(HEAVY_W), "{id}: no heavy stroke in a flat view");
    }
}

/// Uniform-depth supertile engines (every tile level > 0, the shape of
/// every substitution engine's default view): the on-state emits a
/// single heavy group at exactly 2× the base stroke — the diff against
/// the off state is only the stroke width.
#[test]
fn supertile_views_single_heavy_group() {
    for id in ["chair", "ammann-beenker", "sphinx", "golden-bee"] {
        let tiles = view_of(id);
        assert!(!tiles.is_empty(), "{id} produced tiles");
        assert!(
            tiles.iter().all(|t| t.level > 0),
            "{id} must be a supertile view (all level > 0) for this test"
        );
        let off = tess_core::svg::render_svg_with_walls(&tiles, &region(), ColorMode::Pattern, &[]);
        let on = tess_core::svg::render_svg_with_walls_highlight(&tiles, &region(), ColorMode::Pattern, &[], true);

        // The fill pass is untouched.
        assert_eq!(
            fill_prefix(&off),
            fill_prefix(&on),
            "{id}: the fill pass must be byte-identical"
        );

        // Off: one base group, no heavy stroke anywhere.
        assert_eq!(off.matches(BASE_W).count(), 1, "{id}: one base group in the off state");
        assert!(!off.contains(HEAVY_W), "{id}: no heavy stroke in the off state");
        assert_eq!(outline_group_spans(&off).len(), 1, "{id}");

        // On: one heavy group at 2× width, no base stroke left.
        assert_eq!(on.matches(HEAVY_W).count(), 1, "{id}: one heavy group in the on state");
        assert!(!on.contains(BASE_W), "{id}: no base stroke in the on state");
        assert_eq!(outline_group_spans(&on).len(), 1, "{id}: single heavy group");

        // The entire diff is the doubled stroke width.
        assert_eq!(on, off.replace(BASE_W, HEAVY_W), "{id}: only the stroke width may move");

        // Every non-degenerate tile is outlined exactly once.
        let spans = outline_group_spans(&on);
        let g = &on[spans[0].0..spans[0].1];
        assert_eq!(
            g.matches("<path ").count(),
            non_degenerate(&tiles),
            "{id}: one outline path per non-degenerate tile"
        );
    }
}
