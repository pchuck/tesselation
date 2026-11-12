//! SVG rendering of a tiling (shared by CLI export).

use crate::geometry::BBox;
use crate::paint::{rgb, tile_rgb, BORDER, BACKGROUND};
use crate::tiling::Tile;
use crate::view::ColorMode;

/// Render tiles as an SVG string. The y-axis is flipped so that the SVG
/// matches the math orientation (y up).
pub fn render_svg(tiles: &[Tile], bbox: &BBox, mode: ColorMode) -> String {
    let w = bbox.size().x;
    let h = bbox.size().y;
    let stroke = (0.004 * w.min(h)).max(0.001);
    let (br, bg_, bb) = rgb(BORDER);
    let (ar, ag, ab) = rgb(BACKGROUND);

    let mut s = String::with_capacity(tiles.len() * 160 + 512);
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
    s.push_str(&format!(
        r#"<g stroke="rgb({br},{bg_},{bb})" stroke-width="{stroke:.4}" stroke-linejoin="round">"#
    ));
    for tile in tiles {
        let (r, g, b) = rgb(tile_rgb(mode, tile));
        let pts = &tile.poly.pts;
        if pts.len() < 3 {
            continue;
        }
        let mut d = format!("M{:.4} {:.4}", pts[0].x, pts[0].y);
        for p in &pts[1..] {
            d.push_str(&format!("L{:.4} {:.4}", p.x, p.y));
        }
        d.push('Z');
        s.push_str(&format!(r#"<path d="{}" fill="rgb({},{},{})"/>"#, d, r, g, b));
    }
    s.push_str("</g></g></svg>");
    s
}
