//! Focus: the mirror-image export (`tess_core::svg::render_svg_with_walls_highlight_mirrored`)
//! driven through the registry in the exact view the CLI export renders by
//! default (1200×800, scale 1, rotation 0 → a 30×20 world region centered at
//! the origin). Pins the two regimes the shipped engines fall into: the
//! off-state (`--mirror` absent) is byte-identical to the pre-C6 pipeline —
//! the path-stringification discipline is unchanged — and the on-state is
//! exactly the off-state reflected about the frame center (`x → 2·cx − x`)
//! with every fill/outline rule, z-order and the preamble byte-identical.
//!
//! The chiral/achiral census (registry-wide flag, no chirality gate):
//! among the uniforms, only `snub-trihexagonal` (p6, 3.3.3.3.6) declares
//! `chirality: Some("chiral")`; `snub-square` (p4g, 3.3.4.3.4) is
//! explicitly achiral (p4g contains mirror axes) — but either way the
//! reflection evidence below is identical: the on-state is the reflected
//! copy, whatever the tiling's symmetry group.
use tess_core::geometry::{BBox, Vec2};
use tess_core::params::{ParamSet, ParamValue};
use tess_core::strapwork::walls;
use tess_core::tiling::{Query, Tile};
use tess_core::view::ColorMode;

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

/// Every `<path .../>` tag: (parsed `d` points, filled or outline).
fn svg_paths(svg: &str) -> Vec<(Vec<(f64, f64)>, bool)> {
    svg.match_indices("<path ")
        .filter_map(|(i, _)| {
            let rest = &svg[i..];
            let end = rest.find('>')?;
            let tag = &rest[..end];
            let d_start = tag.find("d=\"")? + 3;
            let d_end = tag[d_start..].find('"')?;
            let d = &tag[d_start..d_start + d_end];
            let mut pts = Vec::new();
            for seg in d.split(|c| c == 'M' || c == 'L') {
                let seg = seg.trim();
                if seg.is_empty() {
                    continue;
                }
                let mut toks = seg.split_whitespace();
                let x: f64 = toks.next()?.parse().ok()?;
                let y: f64 = toks.next()?.parse().ok()?;
                pts.push((x, y));
            }
            Some((pts, tag.contains("fill=\"rgb(")))
        })
        .collect()
}

/// Every `<line .../>` tag: (x1, y1, x2, y2).
fn svg_lines(svg: &str) -> Vec<(f64, f64, f64, f64)> {
    svg.match_indices("<line ")
        .filter_map(|(i, _)| {
            let rest = &svg[i..];
            let end = rest.find('>')?;
            let tag = &rest[..end];
            let get = |key: &str| -> Option<f64> {
                let needle = format!("{key}=\"");
                let j = tag.find(&needle)?;
                let after = &tag[j + needle.len()..];
                let e = after.find('"')?;
                after[..e].parse().ok()
            };
            Some((get("x1")?, get("y1")?, get("x2")?, get("y2")?))
        })
        .collect()
}

/// The frame center x of an exported SVG: the preamble `<rect>` carries the
/// world-space region. Byte-identical between off and on states (pinned by
/// `assert_reflected`), so parsing it from the off state is equivalent.
fn frame_cx(svg: &str) -> f64 {
    let i = svg.find("<rect ").expect("SVG has no <rect>");
    let rest = &svg[i..];
    let end = rest.find("/>").expect("rect not closed");
    let tag = &rest[..end];
    let get = |key: &str| -> f64 {
        let needle = format!("{key}=\"");
        let j = tag.find(&needle).unwrap_or_else(|| panic!("rect missing {key}: {tag}"));
        let after = &tag[j + needle.len()..];
        let e = after.find('"').expect("rect attribute truncated");
        after[..e].parse().unwrap()
    };
    get("x") + get("width") / 2.0
}

/// Assert the on-state is exactly the off-state reflected about the
/// vertical axis through `cx` (the frame center): identical preamble,
/// identical path/line census and order, every x mirrored (`x → 2·cx − x`),
/// every y unchanged. The on-state must also differ from the off-state in
/// at least one byte — a trivially-true reflection (the grid aligned with
/// its own axis) would pass the census assertions while proving nothing.
fn assert_reflected(off: &str, on: &str, what: &str) {
    assert_ne!(on, off, "{what}: the mirror must change at least one coordinate");
    let cut = off.find("<path ").expect("no paths");
    assert_eq!(
        &off[..cut],
        &on[..cut],
        "{what}: the preamble (viewBox, flip transform, paper rect) must be byte-identical"
    );
    let cx = frame_cx(off);
    let off_paths = svg_paths(off);
    let on_paths = svg_paths(on);
    assert_eq!(
        on_paths.len(),
        off_paths.len(),
        "{what}: path census must match"
    );
    for (i, ((op, of), (np, nf))) in off_paths.iter().zip(on_paths.iter()).enumerate() {
        assert_eq!(of, nf, "{what}: path {i}: filled/outline role must match");
        assert_eq!(op.len(), np.len(), "{what}: path {i}: vertex count must match");
        for ((ox, oy), (nx, ny)) in op.iter().zip(np.iter()) {
            assert!(
                (nx - (2.0 * cx - ox)).abs() < 1e-3,
                "{what}: path {i}: x {ox} mirrored to {nx}, expected {}",
                2.0 * cx - ox
            );
            assert!(
                (ny - oy).abs() < 1e-3,
                "{what}: path {i}: y must be unchanged ({oy} -> {ny})"
            );
        }
    }
    let off_lines = svg_lines(off);
    let on_lines = svg_lines(on);
    assert_eq!(
        on_lines.len(),
        off_lines.len(),
        "{what}: wall line census must match"
    );
    for ((ax, ay, bx, by), (kx, ky, dx, dy)) in off_lines.iter().zip(on_lines.iter()) {
        assert!(
            (kx - (2.0 * cx - ax)).abs() < 1e-3,
            "{what}: wall a.x must mirror: {ax} -> {kx}"
        );
        assert!((ky - ay).abs() < 1e-3, "{what}: wall a.y must be unchanged");
        assert!(
            (dx - (2.0 * cx - bx)).abs() < 1e-3,
            "{what}: wall b.x must mirror: {bx} -> {dx}"
        );
        assert!((dy - by).abs() < 1e-3, "{what}: wall b.y must be unchanged");
    }
}

/// Off-state: byte-identical to the pre-C6 pipeline. The mirror refactor
/// must leave `--mirror` absent output untouched (both the legacy entry
/// point and the default `highlight=false` flag).
#[test]
fn flag_off_is_byte_identical_to_legacy() {
    for id in ["square", "hexagon", "snub-trihexagonal"] {
        let tiles = view_of(id);
        let legacy = tess_core::svg::render_svg_with_walls(
            &tiles,
            &region(),
            ColorMode::Pattern,
            &[],
        );
        let legacy_flag_off = tess_core::svg::render_svg_with_walls_highlight(
            &tiles,
            &region(),
            ColorMode::Pattern,
            &[],
            false,
        );
        assert_eq!(
            legacy, legacy_flag_off,
            "{id}: the flag-off path must be the legacy bytes"
        );
    }
}

/// On-state: chiral and achiral tilings both render the reflected copy —
/// the enantiomorph for the chiral p6 tiling, the congruent copy for the
/// achiral grids (the census is symmetry-agnostic: the flag works for the
/// whole registry).
#[test]
fn on_state_is_the_reflected_copy() {
    for id in ["square", "hexagon", "snub-trihexagonal", "snub-square"] {
        let tiles = view_of(id);
        assert!(!tiles.is_empty(), "{id} produced tiles");
        let off = tess_core::svg::render_svg_with_walls(
            &tiles,
            &region(),
            ColorMode::Pattern,
            &[],
        );
        let on = tess_core::svg::render_svg_with_walls_highlight_mirrored(
            &tiles,
            &region(),
            ColorMode::Pattern,
            &[],
            false,
        );
        assert_reflected(&off, &on, id);
    }
}

/// Walls ride the same pass: --strapwork wall endpoints (a and b of every
/// segment) are reflected too, while the construction-time topology and
/// the preamble stay byte-identical.
#[test]
fn on_state_reflects_walls() {
    let tiles = view_of("square");
    let ws = walls(&tiles, 45.0);
    assert!(!ws.is_empty(), "square view must produce walls at 45°");
    let off = tess_core::svg::render_svg_with_walls(
        &tiles,
        &region(),
        ColorMode::Pattern,
        &ws,
    );
    let on = tess_core::svg::render_svg_with_walls_highlight_mirrored(
        &tiles,
        &region(),
        ColorMode::Pattern,
        &ws,
        false,
    );
    assert_reflected(&off, &on, "square + strapwork 45°");
}

/// Composition: --highlight-supertiles --mirror. Both groups reflect, the
/// heavy stroke stays heavy, and the preamble + closing tag are unchanged.
#[test]
fn on_state_composes_with_highlight_supertiles() {
    let tiles = view_of("chair");
    assert!(
        tiles.iter().all(|t| t.level > 0),
        "chair default view must be a uniform supertile view"
    );
    let off = tess_core::svg::render_svg_with_walls_highlight(
        &tiles,
        &region(),
        ColorMode::Pattern,
        &[],
        true,
    );
    let on = tess_core::svg::render_svg_with_walls_highlight_mirrored(
        &tiles,
        &region(),
        ColorMode::Pattern,
        &[],
        true,
    );
    assert_reflected(&off, &on, "chair + highlight + mirror");
    // One outline group (uniform supertile view) at the heavy stroke.
    assert_eq!(on.matches("<g fill=\"none\"").count(), 1, "one heavy group: {on}");
    assert!(on.contains(r#"stroke-width="0.1600""#), "heavy stroke survives: {on}");
    assert!(!on.contains(r#"stroke-width="0.0800""#), "no base stroke: {on}");
}

/// Degenerate tiles stay degenerate under the mirror: no invented path,
/// and the census still matches the off state.
#[test]
fn on_state_keeps_degenerate_tiles_degenerate() {
    // The anatomical pin: a clipped tile whose surviving polygon is
    // degenerate (< 3 vertices) emits no path in either pass.
    let mut tiles = view_of("square");
    tiles.push(Tile {
        proto: 2,
        seed: 2,
        level: 0,
        poly: tess_core::geometry::Polygon::from_slice(&[
            Vec2::new(10.0, 5.0),
            Vec2::new(11.0, 5.0),
        ]),
    });
    let off = tess_core::svg::render_svg_with_walls(
        &tiles,
        &region(),
        ColorMode::Pattern,
        &[],
    );
    let on = tess_core::svg::render_svg_with_walls_highlight_mirrored(
        &tiles,
        &region(),
        ColorMode::Pattern,
        &[],
        false,
    );
    assert_reflected(&off, &on, "square + degenerate tile");
}
