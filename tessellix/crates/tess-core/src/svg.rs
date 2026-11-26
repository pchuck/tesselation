//! SVG rendering of a tiling (shared by CLI export).

use crate::geometry::{BBox, Vec2};
use crate::paint::{rgb, tile_rgb, BORDER, BACKGROUND};
use crate::strapwork::{DecorationPlan, Wall};
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

// ──────────────────────────────────────────────────────────────────────────
// Kaplan §3.7 decoration rendering.
//
// Paint order (each element carries a `data-deco` attribute naming its
// layer, so the z-order is parseable and testable):
//
//   paper → face0/face1 (fill) → band (band fill) → side (side lines)
//         → break (background-coloured breaks) → patch (over-strand
//         repaint) → over (over-strand side-line segments)
//
// The decoration replaces the tile rendering: the tiling grout is not
// drawn, only the paper and the woven/coloured wall pattern (Kaplan's
// plates are figures of the decoration alone).
// ──────────────────────────────────────────────────────────────────────────

/// Band fill colour (ochre).
pub const DECOR_BAND: u32 = 0xC68A3D;
/// Face colour class 0 (lapis).
pub const DECOR_FACE_A: u32 = 0x2A5284;
/// Face colour class 1 (bone).
pub const DECOR_FACE_B: u32 = 0xE2D5BA;

/// `d` attribute for one polyline ring (closed with `Z` when `closed`).
fn deco_d(ring: &[Vec2], closed: bool) -> Option<String> {
    if ring.len() < 2 {
        return None;
    }
    // Defense-in-depth: a ring containing any non-finite coordinate is
    // invalid SVG and would abort parsers at its first NaN, silently
    // dropping the whole layer — drop the ring whole instead of
    // emitting a partial one. (Geometry must stay finite; this guards
    // against any future emission path regression.)
    if ring.iter().any(|p| !p.x.is_finite() || !p.y.is_finite()) {
        return None;
    }
    let mut d = format!("M{:.4} {:.4}", ring[0].x, ring[0].y);
    for p in &ring[1..] {
        d.push_str(&format!("L{:.4} {:.4}", p.x, p.y));
    }
    if closed {
        d.push('Z');
    }
    Some(d)
}

/// Render a [`DecorationPlan`] (see [`crate::strapwork::plan_decoration`])
/// on a plain paper background.
pub fn render_svg_decorated(bbox: &BBox, plan: &DecorationPlan) -> String {
    let w = bbox.size().x;
    let h = bbox.size().y;
    let (pr, pg, pb) = rgb(BACKGROUND);
    let (br, bg_, bb) = rgb(BORDER);
    let (wr, wg, wb) = rgb(DECOR_BAND);
    let (a_r, a_g, a_b) = rgb(DECOR_FACE_A);
    let (b_r, b_g, b_b) = rgb(DECOR_FACE_B);
    let lw = plan.line_w;

    let mut s = String::with_capacity(1024);
    s.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w:.4} {h:.4}">"#
    ));
    s.push_str(&format!(
        r#"<g transform="translate({:.4} {:.4}) scale(1 -1)">"#,
        -bbox.min.x, bbox.max.y
    ));
    s.push_str(&format!(
        r#"<rect data-deco="paper" x="{:.4}" y="{:.4}" width="{w:.4}" height="{h:.4}" fill="rgb({pr},{pg},{pb})"/>"#,
        bbox.min.x, bbox.min.y
    ));
    // Fill style: the two face colour classes of the wall map.
    for (ring, cls) in &plan.faces {
        if let Some(d) = deco_d(ring, true) {
            let (r, g, b) = if *cls == 0 {
                (a_r, a_g, a_b)
            } else {
                (b_r, b_g, b_b)
            };
            s.push_str(&format!(
                r#"<path data-deco="face{}" d="{}" fill="rgb({},{},{})"/>"#,
                cls, d, r, g, b
            ));
        }
    }
    // Band fills: outer ring plus optional hole (closed strand), even-odd
    // so the ring around a cycle shows the paper inside.
    for (outer, hole) in &plan.band_fills {
        let Some(mut d) = deco_d(outer, true) else {
            continue;
        };
        if let Some(ho) = hole {
            if let Some(hd) = deco_d(ho, true) {
                d.push_str(&hd); // second M starts the hole subpath (evenodd)
            }
        }
        s.push_str(&format!(
            r#"<path data-deco="band" d="{}" fill="rgb({wr},{wg},{wb})" fill-rule="evenodd"/>"#,
            d
        ));
    }
    // Side lines of every band.
    for (ring, closed) in &plan.side_lines {
        if let Some(d) = deco_d(ring, *closed) {
            s.push_str(&format!(
                r#"<path data-deco="side" d="{d}" fill="none" stroke="rgb({br},{bg_},{bb})" stroke-width="{lw:.4}" stroke-linejoin="round" stroke-linecap="round"/>"#
            ));
        }
    }
    // Interlace: background-coloured breaks cut into the under strand…
    for quad in &plan.break_quads {
        if let Some(d) = deco_d(quad, true) {
            s.push_str(&format!(
                r#"<path data-deco="break" d="{}" fill="rgb({pr},{pg},{pb})"/>"#,
                d
            ));
        }
    }
    // …then the over strand repainted across the break…
    for patch in &plan.break_patch {
        if let Some(d) = deco_d(patch, true) {
            s.push_str(&format!(
                r#"<path data-deco="patch" d="{}" fill="rgb({wr},{wg},{wb})"/>"#,
                d
            ));
        }
    }
    // …and its side lines redrawn, so the over band reads continuous.
    for segs in &plan.over_lines {
        for (a, b) in segs {
            // Same non-finite guard as deco_d: skip bad segments rather
            // than emit NaN coordinates into the `<line>` attribute.
            if !a.x.is_finite() || !a.y.is_finite() || !b.x.is_finite() || !b.y.is_finite() {
                continue;
            }
            s.push_str(&format!(
                r#"<line data-deco="over" x1="{:.4}" y1="{:.4}" x2="{:.4}" y2="{:.4}" stroke="rgb({br},{bg_},{bb})" stroke-width="{lw:.4}" stroke-linecap="butt"/>"#,
                a.x, a.y, b.x, b.y
            ));
        }
    }
    s.push_str("</g></svg>");
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Polygon;
    use crate::strapwork::{plan_decoration, walls, Decoration};

    fn pt(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }
    fn tile(pts: &[(f64, f64)]) -> Tile {
        Tile {
            proto: 0,
            seed: 0,
            poly: Polygon::from_slice(&pts.iter().map(|&(x, y)| pt(x, y)).collect::<Vec<_>>()),
        }
    }
    fn square_grid(n: i64) -> Vec<Tile> {
        let mut v = Vec::new();
        for i in 0..n {
            for j in 0..n {
                let (x, y) = (i as f64, j as f64);
                v.push(tile(&[(x, y), (x + 1.0, y), (x + 1.0, y + 1.0), (x, y + 1.0)]));
            }
        }
        v
    }

    #[test]
    fn pattern_gray_emission_is_gray_only() {
        // End-to-end SVG emission through tile_rgb: one tile per seed
        // 0..8 → the eight PATTERN_GRAY_PALETTE values, no hue palette
        // value anywhere.
        let tiles: Vec<Tile> = (0..8u32)
            .map(|seed| {
                let x = seed as f64;
                Tile {
                    proto: 0,
                    seed,
                    poly: Polygon::from_slice(&[
                        pt(x, 0.0),
                        pt(x + 0.9, 0.0),
                        pt(x + 0.9, 0.9),
                        pt(x, 0.9),
                    ]),
                }
            })
            .collect();
        let bbox = BBox::new(pt(-0.5, -0.5), pt(9.0, 1.0));
        let svg = render_svg(&tiles, &bbox, ColorMode::PatternGray);

        let table = crate::paint::PATTERN_GRAY_PALETTE;
        let hue: std::collections::BTreeSet<(u8, u8, u8)> = crate::paint::PALETTE
            .iter()
            .skip(1) // index 0 is the monochrome light gray; checked separately
            .map(|&c| crate::paint::rgb(c))
            .collect();
        let mut seen = std::collections::BTreeSet::new();
        let mut fill_count = 0usize;
        for fragment in svg.split("<path ").skip(1) {
            let Some(i) = fragment.find("fill=\"rgb(") else { continue };
            let rest = &fragment[i + "fill=\"rgb(".len()..];
            let end = rest.find(')').unwrap();
            let v: Vec<u8> = rest[..end]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            assert_eq!(v.len(), 3, "fill must be rgb(r,g,b)");
            assert_eq!(v[0], v[1], "fill not monochrome: {v:?}");
            assert_eq!(v[1], v[2], "fill not monochrome: {v:?}");
            assert!(
                !hue.contains(&(v[0], v[1], v[2])),
                "hue palette value leaked: {v:?}"
            );
            let hex = (v[0] as u32) << 16 | (v[1] as u32) << 8 | v[2] as u32;
            assert!(
                table.contains(&hex),
                "fill {hex:#x} not in PATTERN_GRAY_PALETTE"
            );
            seen.insert(hex);
            fill_count += 1;
        }
        assert_eq!(fill_count, tiles.len(), "every tile must emit a fill");
        assert_eq!(
            seen.len(),
            8,
            "every PATTERN_GRAY_PALETTE value must appear (seeds 0..8 → 8 entries)"
        );
    }

    #[test]
    fn plain_render_is_untouched_by_decoration() {
        let tiles = square_grid(2);
        let bbox = BBox::new(pt(-1.0, -1.0), pt(3.0, 3.0));
        // Plain export is byte-identical with and without an empty wall
        // slice, and carries no decoration markup whatsoever.
        let plain = render_svg(&tiles, &bbox, ColorMode::Pattern);
        assert_eq!(plain, render_svg_with_walls(&tiles, &bbox, ColorMode::Pattern, &[]));
        assert!(!plain.contains("data-deco"));
    }

    #[test]
    fn decorated_layers_and_zorder() {
        let ws = walls(&square_grid(4), 45.0);
        let bbox = BBox::new(pt(-2.0, -2.0), pt(6.0, 6.0));
        let core = BBox::new(pt(1.0, 1.0), pt(3.0, 3.0));
        let plan = plan_decoration(&ws, 0.12, Decoration::Interlace, &core).unwrap();
        let svg = render_svg_decorated(&bbox, &plan);
        let count = |tag: &str| svg.matches(tag).count();
        assert_eq!(count(r#"data-deco="paper""#), 1);
        assert_eq!(count(r#"data-deco="face"#), plan.faces.len());
        assert_eq!(count(r#"data-deco="band""#), plan.band_fills.len());
        assert_eq!(count(r#"data-deco="side""#), plan.side_lines.len());
        assert_eq!(count(r#"data-deco="break""#), plan.break_quads.len());
        assert_eq!(count(r#"data-deco="patch""#), plan.break_patch.len());
        // Two over-side-line segments per crossing.
        assert_eq!(count(r#"data-deco="over""#), plan.over_lines.len() * 2);
        // Z-order: the first element of each layer paints after the
        // previous layer's first element (paper → band → side → break →
        // patch → over).
        let first = |tag: &str| svg.find(&format!(r#"data-deco="{tag}""#));
        let seq = ["paper", "band", "side", "break", "patch", "over"]
            .map(|t| first(t).expect("layer present"));
        assert!(seq.windows(2).all(|w| w[0] < w[1]), "layer order {seq:?}");
    }

    #[test]
    fn decorated_fill_style_layers_faces_only() {
        let ws = walls(&square_grid(4), 45.0);
        let bbox = BBox::new(pt(-1.0, -1.0), pt(5.0, 5.0));
        let core = BBox::new(pt(1.0, 1.0), pt(3.0, 3.0));
        let plan = plan_decoration(&ws, 0.12, Decoration::Fill, &core).unwrap();
        assert!(!plan.faces.is_empty());
        let svg = render_svg_decorated(&bbox, &plan);
        let f0 = svg.matches(r#"data-deco="face0""#).count();
        let f1 = svg.matches(r#"data-deco="face1""#).count();
        assert_eq!(f0 + f1, plan.faces.len());
        assert!(f0 > 0 && f1 > 0);
        // No interlace or band geometry in a pure fill rendering.
        for tag in ["band", "side", "break", "patch", "over"] {
            assert_eq!(svg.matches(&format!(r#"data-deco="{tag}""#)).count(), 0, "{tag}");
        }
    }
}
