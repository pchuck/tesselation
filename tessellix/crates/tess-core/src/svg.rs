//! SVG rendering of a tiling (shared by CLI export).

use crate::geometry::BBox;
use crate::paint::{rgb, tile_rgb, BORDER, BACKGROUND};
use crate::strapwork::Wall;
use crate::tiling::Tile;
use crate::view::ColorMode;

/// Render tiles as an SVG string. The y-axis is flipped so that the SVG
/// matches the math orientation (y up).
pub fn render_svg(tiles: &[Tile], bbox: &BBox, mode: ColorMode) -> String {
    render_svg_with_walls(tiles, bbox, mode, &[])
}

/// Render tiles plus an optional strapwork wall overlay
/// ([`crate::strapwork::walls`]). With an empty `walls` slice the output
/// is byte-identical to [`render_svg`].
///
/// Drawing is two-pass: every tile fill is emitted first, then every tile
/// outline. SVG paints each element's fill and stroke in document order,
/// so a single interleaved pass lets a later tile's fill cover the
/// earlier tile's stroke half, making the visible border depend on draw
/// order (thin decorated bands — e.g. taylor-socolar's chord strips —
/// get eaten by later-drawn neighbours at shared corners). Emitting all
/// outlines after all fills makes the border symmetric and
/// order-independent.
///
/// Walls are stroked in the border color with round caps — the classic
/// strapwork ribbon look — at a width proportional to the view extent
/// (`0.012·min(w, h)`, floored at `0.003`): three times the tile-border
/// stroke, which keeps the ribbons clearly readable above the fills at
/// any export size without swallowing thin cells. They are drawn last, so
/// ribbons stay on top of the grout.
/// Path data (SVG `d`) for a tile's polygon, or `None` for degenerate
/// tiles. Shared by the fill pass and the outline pass so both passes
/// emit identical geometry.
fn path_d(tile: &Tile) -> Option<String> {
    let pts = &tile.poly.pts;
    if pts.len() < 3 {
        return None;
    }
    let mut d = format!("M{:.4} {:.4}", pts[0].x, pts[0].y);
    for p in &pts[1..] {
        d.push_str(&format!("L{:.4} {:.4}", p.x, p.y));
    }
    d.push('Z');
    Some(d)
}

pub fn render_svg_with_walls(
    tiles: &[Tile],
    bbox: &BBox,
    mode: ColorMode,
    walls: &[Wall],
) -> String {
    let w = bbox.size().x;
    let h = bbox.size().y;
    let stroke = (0.004 * w.min(h)).max(0.001);
    let wall_stroke = (0.012 * w.min(h)).max(0.003);
    let (br, bg_, bb) = rgb(BORDER);
    let (ar, ag, ab) = rgb(BACKGROUND);

    let mut s = String::with_capacity(tiles.len() * 300 + walls.len() * 64 + 512);
    s.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w:.4} {h:.4}">"#
    ));
    // Maps world (x, y) -> svg (x - min.x, max.y - y).
    s.push_str(&format!(
        r#"<g transform="translate({:.4} {:.4}) scale(1 -1)">"#,
        -bbox.min.x, bbox.max.y
    ));
    s.push_str(&format!(
        r#"<rect x="{:.4}" y="{:.4}" width="{w:.4}" height="{h:.4}" fill="rgb({ar},{ag},{ab})"/>"#,
        bbox.min.x, bbox.min.y
    ));
    // Pass 1: all fills, no outlines.
    for tile in tiles {
        if let Some(d) = path_d(tile) {
            let (r, g, b) = rgb(tile_rgb(mode, tile));
            s.push_str(&format!(r#"<path d="{}" fill="rgb({},{},{})"/>"#, d, r, g, b));
        }
    }
    // Pass 2: all outlines, over every fill.
    s.push_str(&format!(
        r#"<g fill="none" stroke="rgb({br},{bg_},{bb})" stroke-width="{stroke:.4}" stroke-linejoin="round">"#
    ));
    for tile in tiles {
        if let Some(d) = path_d(tile) {
            s.push_str(&format!(r#"<path d="{}"/>"#, d));
        }
    }
    s.push_str("</g>");
    if !walls.is_empty() {
        s.push_str(&format!(
            r#"<g fill="none" stroke="rgb({br},{bg_},{bb})" stroke-width="{wall_stroke:.4}" stroke-linecap="round">"#
        ));
        for wall in walls {
            s.push_str(&format!(
                r#"<line x1="{:.4}" y1="{:.4}" x2="{:.4}" y2="{:.4}"/>"#,
                wall.a.x, wall.a.y, wall.b.x, wall.b.y
            ));
        }
        s.push_str("</g>");
    }
    s.push_str("</g></svg>");
    s
}
