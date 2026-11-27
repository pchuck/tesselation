//! SVG rendering of a tiling (shared by CLI export).

use crate::geometry::{BBox, Polygon, Vec2};
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
/// is byte-identical to [`render_svg`]. Non-highlighting entry point:
/// exactly [`render_svg_with_walls_highlight`] with
/// `highlight_supertiles = false`.
pub fn render_svg_with_walls(
    tiles: &[Tile],
    bbox: &BBox,
    mode: ColorMode,
    walls: &[Wall],
) -> String {
    render_svg_with_walls_highlight(tiles, bbox, mode, walls, false)
}

/// Stroke-weight multiplier for the supertile-highlight pass of
/// [`render_svg_with_walls_highlight`]: tiles carrying substitution
/// depth (`tile.level > 0` — members of a supertile, not flat-tiling
/// cells) are outlined at `SUPERTILE_STROKE_FACTOR ×` the base
/// tile-border stroke. 2× keeps the highlight emphatic without
/// swallowing thin cells: the base stroke is already
/// `0.004·min(w, h)` (0.08 world units ≈ 3.2 px at the 1200×800
/// default export).
pub const SUPERTILE_STROKE_FACTOR: f64 = 2.0;

/// Path data (SVG `d`) for a tile's polygon, or `None` for degenerate
/// tiles. Shared by the fill pass and the outline pass(es) so both emit
/// identical geometry.
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

/// Reflection about the frame center x (`x → 2·cx − x`, y unchanged) —
/// the enantiomorph transform (C6). The reflection axis is the vertical
/// line through the frame center: every emitted coordinate is
/// mirrored, not the view or an SVG `<g transform>`.
fn mirror_point(p: Vec2, cx: f64) -> Vec2 {
    Vec2::new(2.0 * cx - p.x, p.y)
}

/// The mirror images of every tile of the render: polygon vertices
/// reflected about the frame center, everything else untouched.
fn mirror_tiles(tiles: &[Tile], cx: f64) -> Vec<Tile> {
    tiles
        .iter()
        .map(|t| Tile {
            proto: t.proto,
            seed: t.seed,
            level: t.level,
            poly: Polygon::from_slice(
                &t.poly.pts.iter().map(|&p| mirror_point(p, cx)).collect::<Vec<_>>(),
            ),
        })
        .collect()
}

/// The mirror images of every wall segment (endpoints reflected).
fn mirror_walls(walls: &[Wall], cx: f64) -> Vec<Wall> {
    walls
        .iter()
        .map(|w| Wall {
            a: mirror_point(w.a, cx),
            b: mirror_point(w.b, cx),
        })
        .collect()
}

/// Render tiles plus an optional strapwork wall overlay, with an
/// optional supertile highlight.
///
/// [mirror = false]: the pre-C6 export byte-for-byte. Supertile
/// highlight (`highlight_supertiles = true`): the outline pass splits
/// into up to two groups by substitution depth — level-0 cells at the
/// base stroke, supertile members (`tile.level > 0`) at the heavy
/// stroke — so the heavier grout paints over shared edges. Empty
/// groups are skipped: a flat tiling emits exactly the legacy single
/// group. Walls are stroked last, on top of the grout.
pub fn render_svg_with_walls_highlight(
    tiles: &[Tile],
    bbox: &BBox,
    mode: ColorMode,
    walls: &[Wall],
    highlight_supertiles: bool,
) -> String {
    render_svg_core(tiles, bbox, mode, walls, highlight_supertiles, false)
}

/// Mirror-image export (C6): identical to
/// [`render_svg_with_walls_highlight`] except every emitted geometry —
/// tile polygon vertices and strapwork wall endpoints — is reflected
/// about the frame center (`x → 2·cx − x`, y unchanged) BEFORE path
/// stringification. The y-axis flip transform and all stroke/fill
/// rules are untouched: the on-state is a congruent copy of the
/// off-state with the pattern mirrored, never an SVG-level `transform`
/// attribute on the `<g>`.
///
/// Registry-wide: the flag works for every tiling. The meaningful use
/// is chiral tilings (e.g. `snub-trihexagonal`, p6), whose reflected
/// pattern is the enantiomorph; achiral tilings render a visibly
/// different but geometrically congruent copy. The decoration render
/// ([`render_svg_decorated`]) is not covered — it replaces the tiling
/// render entirely (CLI: named conflict).
pub fn render_svg_with_walls_highlight_mirrored(
    tiles: &[Tile],
    bbox: &BBox,
    mode: ColorMode,
    walls: &[Wall],
    highlight_supertiles: bool,
) -> String {
    render_svg_core(tiles, bbox, mode, walls, highlight_supertiles, true)
}

/// The two-pass emitter behind both public rendering entry points (the
/// [`render_svg_with_walls_highlight_mirrored`] variant reflects every
/// emitted coordinate about the frame center first). Every tile fill is
/// emitted first, then every tile outline (SVG paints each element fill/stroke
/// in document order, so interleaving lets a later tile's fill cover an
/// earlier tile's stroke half — the discipline below), then strapwork walls
/// (round caps, on top).
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
/// Supertile highlight (`highlight_supertiles = true`): the outline pass
/// splits into up to two groups in stroke-weight order — level-0 cells at
/// the base stroke, then supertile members (`tile.level > 0`) at
/// [`SUPERTILE_STROKE_FACTOR`]× base — so the heavier grout paints over
/// shared edges. Empty groups are skipped: a flat tiling (every tile
/// level 0) emits exactly the legacy single group, making the flag a
/// byte-for-byte no-op there, and with `highlight_supertiles = false`
/// the output is byte-identical to [`render_svg_with_walls`] for every
/// tiling.
///
/// Walls are stroked in the border color with round caps — the classic
/// strapwork ribbon look — at a width proportional to the view extent
/// (`0.012·min(w, h)`, floored at `0.003`): three times the tile-border
/// stroke, which keeps the ribbons clearly readable above the fills at
/// any export size without swallowing thin cells. They are drawn last, so
/// ribbons stay on top of the grout.
fn render_svg_core(
    tiles: &[Tile],
    bbox: &BBox,
    mode: ColorMode,
    walls: &[Wall],
    highlight_supertiles: bool,
    mirror: bool,
) -> String {
    let cx = bbox.center().x;
    let owned_tiles;
    let owned_walls;
    let (tiles, walls): (&[Tile], &[Wall]) = if mirror {
        owned_tiles = mirror_tiles(tiles, cx);
        owned_walls = mirror_walls(walls, cx);
        (&owned_tiles, &owned_walls)
    } else {
        (tiles, walls)
    };
    let w = bbox.size().x;
    let h = bbox.size().y;
    let stroke = (0.004 * w.min(h)).max(0.001);
    let heavy = stroke * SUPERTILE_STROKE_FACTOR;
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
    if !highlight_supertiles {
        // Single uniform group — the pre-highlight output, byte for byte.
        s.push_str(&format!(
            r#"<g fill="none" stroke="rgb({br},{bg_},{bb})" stroke-width="{stroke:.4}" stroke-linejoin="round">"#
        ));
        for tile in tiles {
            if let Some(d) = path_d(tile) {
                s.push_str(&format!(r#"<path d="{}"/>"#, d));
            }
        }
        s.push_str("</g>");
    } else {
        // Supertile highlight: split the outline pass by substitution
        // depth — level-0 cells at the base stroke first, supertile
        // members (level > 0) at the heavy stroke last — so the heavier
        // grout paints over the shared edges. Empty groups are skipped:
        // a flat tiling emits exactly the legacy single group, a uniform
        // supertile view exactly one (heavy) group.
        let base_tiles: Vec<&Tile> = tiles.iter().filter(|t| t.level == 0).collect();
        let heavy_tiles: Vec<&Tile> = tiles.iter().filter(|t| t.level > 0).collect();
        if !base_tiles.is_empty() {
            s.push_str(&format!(
                r#"<g fill="none" stroke="rgb({br},{bg_},{bb})" stroke-width="{stroke:.4}" stroke-linejoin="round">"#
            ));
            for tile in base_tiles {
                if let Some(d) = path_d(tile) {
                    s.push_str(&format!(r#"<path d="{}"/>"#, d));
                }
            }
            s.push_str("</g>");
        }
        if !heavy_tiles.is_empty() {
            s.push_str(&format!(
                r#"<g fill="none" stroke="rgb({br},{bg_},{bb})" stroke-width="{heavy:.4}" stroke-linejoin="round">"#
            ));
            for tile in heavy_tiles {
                if let Some(d) = path_d(tile) {
                    s.push_str(&format!(r#"<path d="{}"/>"#, d));
                }
            }
            s.push_str("</g>");
        }
    }
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
            seed: 0, level: 0,
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
                    seed, level: 0,
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

    // ---- supertile highlight (render_svg_with_walls_highlight) ----------

    /// A mixed-level tiling: one level-0 tile (flat-tiling cell) and one
    /// level-2 tile (supertile member), side by side.
    fn mixed_level_tiles() -> Vec<Tile> {
        vec![
            Tile {
                proto: 0,
                seed: 0,
                level: 0,
                poly: Polygon::from_slice(&[
                    pt(0.0, 0.0),
                    pt(1.0, 0.0),
                    pt(1.0, 1.0),
                    pt(0.0, 1.0),
                ]),
            },
            Tile {
                proto: 1,
                seed: 1,
                level: 2,
                poly: Polygon::from_slice(&[
                    pt(1.0, 0.0),
                    pt(2.0, 0.0),
                    pt(2.0, 1.0),
                    pt(1.0, 1.0),
                ]),
            },
        ]
    }

    /// The outline `<g>` blocks of a rendered SVG (outline and wall
    /// groups only — the sole `<g fill="none" ...>` openers), as
    /// (start, end) byte ranges in document order.
    fn outline_group_spans(svg: &str) -> Vec<(usize, usize)> {
        svg.match_indices("<g fill=\"none\"")
            .map(|(i, _)| {
                let end = i
                    + svg[i..]
                        .find("</g>")
                        .expect("outline/wall group is closed");
                (i, end + "</g>".len())
            })
            .collect()
    }

    #[test]
    fn highlight_off_output_is_byte_pinned() {
        // Full golden pin of the toggle-off output on a mixed-level
        // tiling: any change to the default export's markup, attribute
        // order or number formatting breaks this. Bbox 4×3 → base stroke
        // 0.004·3 = 0.012; Single mode → PALETTE[0] light gray.
        let tiles = mixed_level_tiles();
        let bbox = BBox::new(pt(-1.0, -1.0), pt(3.0, 2.0));
        let expected = concat!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 4.0000 3.0000">"#,
            r#"<g transform="translate(1.0000 2.0000) scale(1 -1)">"#,
            r#"<rect x="-1.0000" y="-1.0000" width="4.0000" height="3.0000" fill="rgb(40,42,58)"/>"#,
            r#"<path d="M0.0000 0.0000L1.0000 0.0000L1.0000 1.0000L0.0000 1.0000Z" fill="rgb(212,212,212)"/>"#,
            r#"<path d="M1.0000 0.0000L2.0000 0.0000L2.0000 1.0000L1.0000 1.0000Z" fill="rgb(212,212,212)"/>"#,
            r#"<g fill="none" stroke="rgb(30,30,46)" stroke-width="0.0120" stroke-linejoin="round">"#,
            r#"<path d="M0.0000 0.0000L1.0000 0.0000L1.0000 1.0000L0.0000 1.0000Z"/>"#,
            r#"<path d="M1.0000 0.0000L2.0000 0.0000L2.0000 1.0000L1.0000 1.0000Z"/>"#,
            r#"</g></g></svg>"#
        );
        assert_eq!(render_svg_with_walls(&tiles, &bbox, ColorMode::Single, &[]), expected);
        assert_eq!(
            render_svg_with_walls_highlight(&tiles, &bbox, ColorMode::Single, &[], false),
            expected,
            "flag-off must be the legacy bytes, not merely equivalent"
        );
    }

    #[test]
    fn highlight_on_weights_supertile_members_only() {
        // On-state on the same mixed tiling: the fill pass is untouched,
        // the outline pass splits into a base group (exactly the level-0
        // path) and a heavy group at 2× width (exactly the level-2
        // path), base before heavy.
        let tiles = mixed_level_tiles();
        let bbox = BBox::new(pt(-1.0, -1.0), pt(3.0, 2.0));
        let off = render_svg_with_walls(&tiles, &bbox, ColorMode::Single, &[]);
        let svg = render_svg_with_walls_highlight(&tiles, &bbox, ColorMode::Single, &[], true);
        let d0 = "M0.0000 0.0000L1.0000 0.0000L1.0000 1.0000L0.0000 1.0000Z";
        let d2 = "M1.0000 0.0000L2.0000 0.0000L2.0000 1.0000L1.0000 1.0000Z";

        // Fill pass byte-identical: everything up to the first outline
        // group is unchanged.
        let cut_off = off.find("<g fill=\"none\"").unwrap();
        let cut_on = svg.find("<g fill=\"none\"").unwrap();
        assert_eq!(&off[..cut_off], &svg[..cut_on], "fill pass must be untouched");

        // Exactly one group per weight class, base at 0.0120, heavy at
        // 2× = 0.0240.
        let base_attr = r#"stroke-width="0.0120""#;
        let heavy_attr = r#"stroke-width="0.0240""#;
        assert_eq!(svg.matches(base_attr).count(), 1, "{svg}");
        assert_eq!(svg.matches(heavy_attr).count(), 1, "{svg}");

        // Group contents: group 0 (base) carries exactly the level-0
        // path; group 1 (heavy) exactly the level-2 path.
        let spans = outline_group_spans(&svg);
        assert_eq!(spans.len(), 2, "base + heavy group, no walls: {svg}");
        let g0 = &svg[spans[0].0..spans[0].1];
        let g1 = &svg[spans[1].0..spans[1].1];
        assert!(g0.contains(base_attr) && !g0.contains(heavy_attr), "{g0}");
        assert!(g1.contains(heavy_attr) && !g1.contains(base_attr), "{g1}");
        assert_eq!(g0.matches("<path ").count(), 1);
        assert!(g0.contains(d0) && !g0.contains(d2), "base group must hold the level-0 path only");
        assert_eq!(g1.matches("<path ").count(), 1);
        assert!(g1.contains(d2) && !g1.contains(d0), "heavy group must hold the level-2 path only");
    }

    #[test]
    fn highlight_on_flat_tiling_is_byte_identical_to_off() {
        // No tile carries substitution depth: the heavy group is empty
        // and skipped, so flag-on must be the flag-off bytes — the
        // highlight is a pure no-op outside the substitution hierarchy.
        let tiles = square_grid(3);
        let bbox = BBox::new(pt(-2.0, -2.0), pt(4.0, 4.0));
        let off = render_svg_with_walls(&tiles, &bbox, ColorMode::Pattern, &[]);
        let on = render_svg_with_walls_highlight(&tiles, &bbox, ColorMode::Pattern, &[], true);
        assert_eq!(on, off, "flat tiling: highlight must be a byte-for-byte no-op");
        assert_eq!(on.matches("<g fill=\"none\"").count(), 1, "one base group only");
    }

    #[test]
    fn highlight_on_uniform_supertile_is_single_heavy_group() {
        // Every tile is a supertile member (a uniform-depth view, the
        // shape of every substitution engine's default view): the base
        // group is empty and skipped — the on-state is the legacy bytes
        // with the outline width scaled by SUPERTILE_STROKE_FACTOR and
        // nothing else. Bbox 4×4 → base 0.0160, heavy 0.0320.
        let mut tiles = square_grid(2);
        for t in &mut tiles {
            t.level = 3;
        }
        let bbox = BBox::new(pt(-1.0, -1.0), pt(3.0, 3.0));
        let off = render_svg_with_walls(&tiles, &bbox, ColorMode::Pattern, &[]);
        let on = render_svg_with_walls_highlight(&tiles, &bbox, ColorMode::Pattern, &[], true);
        let base_attr = r#"stroke-width="0.0160""#;
        let heavy_attr = r#"stroke-width="0.0320""#;
        assert_eq!(off.matches(base_attr).count(), 1, "one base group in the off state");
        assert_eq!(on.matches(heavy_attr).count(), 1, "{on}");
        assert!(!on.contains(base_attr), "no base stroke in the on state: {on}");
        assert_eq!(on, off.replace(base_attr, heavy_attr));
        assert_eq!(on.matches("<g fill=\"none\"").count(), 1, "single heavy group");
    }

    #[test]
    fn highlight_on_mixed_tiles_skips_degenerate_in_heavy_group() {
        // A degenerate (2-vertex) supertile member contributes no path
        // anywhere: the heavy group holds only the non-degenerate level-2
        // tile's path.
        let mut tiles = mixed_level_tiles();
        tiles.push(Tile {
            proto: 2,
            seed: 2,
            level: 2,
            poly: Polygon::from_slice(&[pt(2.0, 0.0), pt(3.0, 0.0)]),
        });
        let bbox = BBox::new(pt(-1.0, -1.0), pt(4.0, 2.0));
        let svg = render_svg_with_walls_highlight(&tiles, &bbox, ColorMode::Single, &[], true);
        let spans = outline_group_spans(&svg);
        assert_eq!(spans.len(), 2, "base + heavy group: {svg}");
        let g1 = &svg[spans[1].0..spans[1].1];
        assert_eq!(g1.matches("<path ").count(), 1, "degenerate tile must emit no path: {g1}");
        // Fill pass: the degenerate tile gets no fill either (path_d
        // None) — exactly the two non-degenerate tiles carry the Single
        // fill (PALETTE[0] light gray), not the background rect.
        assert_eq!(
            svg.matches(r#"fill="rgb(212,212,212)""#).count(),
            2,
            "only the two non-degenerate tiles get a fill: {svg}"
        );
    }

    #[test]
    fn highlight_on_keeps_walls_on_top() {
        // The wall pass stays last regardless of the highlight: every
        // wall line follows the last (heavy) outline path, and the two
        // outline groups (here just the heavy one) precede the wall
        // group. Bbox 8×8 → base 0.0320, heavy 0.0640, walls 0.0960.
        let mut tiles = square_grid(4);
        for t in &mut tiles {
            t.level = 2;
        }
        let ws = walls(&tiles, 45.0);
        assert!(!ws.is_empty(), "need a non-empty wall set");
        let bbox = BBox::new(pt(-2.0, -2.0), pt(6.0, 6.0));
        let svg = render_svg_with_walls_highlight(&tiles, &bbox, ColorMode::Pattern, &ws, true);
        let spans = outline_group_spans(&svg);
        assert_eq!(spans.len(), 2, "heavy outline group + wall group: {svg}");
        let heavy = &svg[spans[0].0..spans[0].1];
        let wallg = &svg[spans[1].0..spans[1].1];
        assert!(heavy.contains(r#"stroke-width="0.0640""#), "{heavy}");
        assert_eq!(heavy.matches("<path ").count(), tiles.len(), "every tile in the heavy group");
        assert!(!heavy.contains(r#"stroke-width="0.0320""#), "no base group when all tiles are supertile members");
        assert!(wallg.contains(r#"stroke-width="0.0960""#), "{wallg}");
        assert_eq!(wallg.matches("<line ").count(), ws.len());
        // Every fill precedes the first outline group.
        let mut last_fill = 0usize;
        for (i, _) in svg.match_indices("<path ") {
            let rest = &svg[i..];
            let tag_end = rest.find('>').unwrap();
            if rest[..tag_end].contains("fill=\"rgb(") {
                last_fill = i;
            }
        }
        assert!(last_fill < spans[0].0, "fills must precede the outline groups");
    }

    #[test]
    fn fills_precede_outlines_precede_walls() {
        // Document order pin (the two-pass discipline the module docs
        // document): every tile fill is emitted before every tile outline,
        // and every wall is emitted after every outline. A single
        // interleaved pass would let a later tile's fill cover the earlier
        // tile's stroke half; walls drawn before the outlines would drop
        // the ribbons below the grout.
        let tiles = square_grid(4);
        let ws = walls(&tiles, 45.0);
        assert!(!ws.is_empty(), "need a non-empty wall set");
        let bbox = BBox::new(pt(-2.0, -2.0), pt(6.0, 6.0));
        let svg = render_svg_with_walls(&tiles, &bbox, ColorMode::Pattern, &ws);

        // Pass 1 fills: `<path d="..." fill="rgb(r,g,b)"/>` — the only
        // paths carrying a fill attribute.
        // Pass 2 outlines: `<path d="..."/>` — no fill attribute (stroke
        // inherited from the `<g fill="none" ...>` wrapper).
        let mut fill_pos = Vec::new();
        let mut outline_pos = Vec::new();
        for (i, _) in svg.match_indices("<path ") {
            let rest = &svg[i..];
            let tag_end = rest.find('>').unwrap();
            if rest[..tag_end].contains("fill=\"rgb(") {
                fill_pos.push(i);
            } else {
                outline_pos.push(i);
            }
        }
        // Walls: `<line .../>` — unique to the wall pass (the decorated
        // renderer's `<line data-deco="over">` belongs to a different
        // function and is absent here).
        let wall_pos: Vec<usize> = svg.match_indices("<line ").map(|(i, _)| i).collect();

        assert_eq!(fill_pos.len(), tiles.len(), "every tile gets a fill");
        assert_eq!(outline_pos.len(), tiles.len(), "every tile gets an outline");
        assert_eq!(wall_pos.len(), ws.len(), "every wall gets a line");

        let last_fill = *fill_pos.iter().max().unwrap();
        let first_outline = *outline_pos.iter().min().unwrap();
        let first_wall = *wall_pos.iter().min().unwrap();
        assert!(
            last_fill < first_outline,
            "last fill at {last_fill} must precede first outline at {first_outline}"
        );
        assert!(
            first_outline < first_wall,
            "first outline at {first_outline} must precede first wall at {first_wall}"
        );
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

    #[test]
    fn decorated_layer_blocks_maxima_precede_minima() {
        // Document-order pin for the decorated renderer. The module docs
        // (svg.rs, Kaplan section) claim:
        //
        //   paper → face0/face1 (fill) → band (band fill) → side (side
        //   lines) → break (background-coloured breaks) → patch (over-
        //   strand repaint) → over (over-strand side-line segments)
        //
        // Interlace never emits faces and Fill never emits anything but
        // faces, so merge a Fill plan's face set with an Interlace plan's
        // other geometry to exercise the full documented order in one
        // render. The renderer treats the plan as pure data (it walks each
        // layer's vec in fixed document order), so the merge is valid.
        let ws = walls(&square_grid(4), 45.0);
        let bbox = BBox::new(pt(-2.0, -2.0), pt(6.0, 6.0));
        let core = BBox::new(pt(1.0, 1.0), pt(3.0, 3.0));
        let mut plan = plan_decoration(&ws, 0.12, Decoration::Interlace, &core).unwrap();
        let fill_faces = plan_decoration(&ws, 0.12, Decoration::Fill, &core)
            .unwrap()
            .faces;
        assert!(!fill_faces.is_empty(), "need non-empty fill faces");
        plan.faces = fill_faces;

        let svg = render_svg_decorated(&bbox, &plan);

        // Positions of each layer's elements in document order. The
        // `face` tag matches both `face0` and `face1`, as in the
        // existing layers test.
        let positions = |tag: &str| -> Vec<usize> {
            svg.match_indices(&format!(r#"data-deco="{tag}"#))
                .map(|(i, _)| i)
                .collect()
        };
        let paper = positions("paper");
        let faces = positions("face");
        let bands = positions("band");
        let sides = positions("side");
        let breaks = positions("break");
        let patches = positions("patch");
        let overs = positions("over");

        // Counts per layer class, then per face class.
        assert_eq!(paper.len(), 1, "one paper rect");
        assert_eq!(
            faces.len(),
            plan.faces.len(),
            "one face path per plan face"
        );
        assert_eq!(
            bands.len(),
            plan.band_fills.len(),
            "one band path per band fill"
        );
        assert_eq!(
            sides.len(),
            plan.side_lines.len(),
            "one side path per side line"
        );
        assert_eq!(
            breaks.len(),
            plan.break_quads.len(),
            "one break path per break quad"
        );
        assert_eq!(
            patches.len(),
            plan.break_patch.len(),
            "one patch path per break patch"
        );
        // Two over-side-line segments per crossing.
        assert_eq!(
            overs.len(),
            plan.over_lines.len() * 2,
            "two over segments per crossing"
        );
        assert!(
            !faces.is_empty()
                && !bands.is_empty()
                && !sides.is_empty()
                && !breaks.is_empty()
                && !patches.is_empty()
                && !overs.is_empty(),
            "every non-paper layer must be non-empty for this setup"
        );
        let f0 = svg.matches(r#"data-deco="face0""#).count();
        let f1 = svg.matches(r#"data-deco="face1""#).count();
        assert!(f0 > 0 && f1 > 0, "both face colour classes must appear");

        // Document order as blocks: every element of layer N precedes
        // every element of layer N+1 (max(layer N) < min(layer N+1)) —
        // the same pairing fills_precede_outlines_precede_walls uses,
        // extended from fills/outlines to the deco classes. This is
        // strictly stronger than the first-of-each-layer ordering the
        // existing decorated_layers_and_zorder test asserts: it also
        // catches, e.g., a band fill emitted late between side lines.
        let last = |v: &Vec<usize>| *v.iter().max().unwrap();
        let first = |v: &Vec<usize>| *v.iter().min().unwrap();
        assert!(last(&paper) < first(&faces), "paper must precede faces");
        assert!(last(&faces) < first(&bands), "faces must precede band fills");
        assert!(last(&bands) < first(&sides), "band fills must precede side lines");
        assert!(last(&sides) < first(&breaks), "side lines must precede breaks");
        assert!(last(&breaks) < first(&patches), "breaks must precede patches");
        assert!(last(&patches) < first(&overs), "patches must precede over lines");
    }

    #[test]
    fn decorated_empty_plan_emits_only_paper() {
        // Degenerate-claim pin: the decoration replaces the tile
        // rendering ("only the paper and the woven/coloured wall
        // pattern"), so an empty plan must emit the paper rect and
        // nothing else — no stray decoration markup.
        let bbox = BBox::new(pt(0.0, 0.0), pt(1.0, 1.0));
        let svg = render_svg_decorated(&bbox, &DecorationPlan::default());
        assert_eq!(svg.matches("data-deco=").count(), 1);
        assert!(svg.contains(r#"data-deco="paper""#));
        for tag in ["face", "band", "side", "break", "patch", "over"] {
            assert!(
                !svg.contains(&format!(r#"data-deco="{tag}"#)),
                "empty plan must not emit the {tag} layer"
            );
        }
    }
    // ---- mirror (render_svg_with_walls_highlight_mirrored) ----------

    /// Every `<path ...>` tag of the SVG, as (d, filled_with_rgb_fill) in
    /// document order (fills first, then outlines).
    fn path_tags(svg: &str) -> Vec<(String, bool)> {
        svg.match_indices("<path ")
            .filter_map(|(i, _)| {
                let rest = &svg[i..];
                let tag_end = rest.find('>')?;
                let tag = &rest[..tag_end];
                let d_start = tag.find("d=\"")? + 3;
                let d_end = tag[d_start..].find('"')?;
                let filled = tag.contains("fill=\"rgb(");
                Some((tag[d_start..d_start + d_end].to_string(), filled))
            })
            .collect()
    }

    /// Points of a path `d` ("M x yL x yL x y...Z").
    fn d_points(d: &str) -> Vec<(f64, f64)> {
        let body = d.strip_suffix('Z').unwrap_or(d);
        let mut pts = Vec::new();
        for seg in body.split(|c| c == 'M' || c == 'L') {
            let seg = seg.trim();
            if seg.is_empty() {
                continue;
            }
            let mut toks = seg.split_whitespace();
            let x: f64 = toks.next().unwrap().parse().unwrap();
            let y: f64 = toks.next().unwrap().parse().unwrap();
            pts.push((x, y));
        }
        pts
    }

    /// Wall `<line .../>` endpoints (x1, y1, x2, y2) in document order.
    fn line_endpoints(svg: &str) -> Vec<(f64, f64, f64, f64)> {
        svg.match_indices("<line ")
            .filter_map(|(i, _)| {
                let rest = &svg[i..];
                let tag_end = rest.find('>')?;
                let tag = &rest[..tag_end];
                let get = |attr: &str| -> f64 {
                    let at = tag.find(attr).unwrap() + attr.len();
                    let rest = &tag[at..];
                    let end = rest.find('"').unwrap();
                    rest[..end].parse().unwrap()
                };
                Some((get("x1=\""), get("y1=\""), get("x2=\""), get("y2=\"")))
            })
            .collect()
    }

    /// Assert the `on` state is exactly the `off` state reflected about the
    /// vertical axis through `cx`: identical path/line census and order,
    /// every x mirrored (`x -> 2*cx - x`), every y unchanged.
    fn assert_reflected(off: &str, on: &str, cx: f64) {
        let off_paths = path_tags(off);
        let on_paths = path_tags(on);
        assert_eq!(
            on_paths.len(),
            off_paths.len(),
            "on/off path count must match"
        );
        for (i, ((od, of), (nd, nf))) in off_paths.iter().zip(on_paths.iter()).enumerate() {
            assert_eq!(of, nf, "path {i}: fill/outline role must match");
            let op = d_points(od);
            let np = d_points(nd);
            assert_eq!(op.len(), np.len(), "path {i}: vertex count must match");
            for ((ox, oy), (nx, ny)) in op.iter().zip(np.iter()) {
                let want = 2.0 * cx - ox;
                assert!(
                    (nx - want).abs() < 1e-3,
                    "path {i}: x {ox} mirrored to {nx}, expected {want}"
                );
                assert!(
                    (ny - oy).abs() < 1e-3,
                    "path {i}: y must be unchanged ({oy} -> {ny})"
                );
            }
        }
        let off_lines = line_endpoints(off);
        let on_lines = line_endpoints(on);
        assert_eq!(
            on_lines.len(),
            off_lines.len(),
            "on/off wall line count must match"
        );
        for ((ax, ay, bx, by), (kx, ky, dx, dy)) in off_lines.iter().zip(on_lines.iter()) {
            assert!(
                (kx - (2.0 * cx - ax)).abs() < 1e-3,
                "wall endpoint x must mirror: {ax} -> {kx}"
            );
            assert!(
                (ky - ay).abs() < 1e-3,
                "wall endpoint y must be unchanged: {ay} -> {ky}"
            );
            assert!(
                (dx - (2.0 * cx - bx)).abs() < 1e-3,
                "wall endpoint x must mirror: {bx} -> {dx}"
            );
            assert!(
                (dy - by).abs() < 1e-3,
                "wall endpoint y must be unchanged: {by} -> {dy}"
            );
        }
    }

    #[test]
    fn mirror_off_render_is_byte_identical_to_pre_c6_output() {
        // The mirror refactor must leave the flag-off path exactly as it
        // was: render_svg_with_walls_highlight(.., false) == the legacy
        // render_svg_with_walls (the golden pin above it is one of the
        // pre-C6 full-string tests in this file).
        let tiles = square_grid(3);
        let bbox = BBox::new(pt(-2.0, -2.0), pt(4.0, 4.0));
        let legacy = render_svg_with_walls(&tiles, &bbox, ColorMode::Pattern, &[]);
        assert_eq!(
            legacy,
            render_svg_with_walls_highlight(&tiles, &bbox, ColorMode::Pattern, &[], false),
            "flag-off must be the legacy bytes"
        );
    }

    #[test]
    fn mirror_on_reflects_fill_and_outline_and_preamble() {
        let tiles = mixed_level_tiles();
        let bbox = BBox::new(pt(-1.0, -1.0), pt(3.0, 2.0));
        let cx = bbox.center().x;
        let off = render_svg_with_walls(&tiles, &bbox, ColorMode::Pattern, &[]);
        let on = render_svg_with_walls_highlight_mirrored(&tiles, &bbox, ColorMode::Pattern, &[], false);
        assert_ne!(on, off, "mirror must change at least one coordinate");
        assert_reflected(&off, &on, cx);
        // The preamble (viewBox / flip transform / paper rect) is byte-identical.
        let cut = off.find("<path ").unwrap();
        assert_eq!(
            &off[..cut],
            &on[..cut],
            "the preamble must be byte-identical"
        );
        // The closing tag is unchanged too (same total size).
        assert!(on.ends_with("</g></svg>"), "{on}");
    }

    #[test]
    fn mirror_on_flat_tiling_is_the_reflected_copy() {
        let tiles = square_grid(3);
        let bbox = BBox::new(pt(-2.0, -2.0), pt(4.0, 4.0));
        let cx = bbox.center().x;
        let off = render_svg_with_walls(&tiles, &bbox, ColorMode::Pattern, &[]);
        let on = render_svg_with_walls_highlight_mirrored(&tiles, &bbox, ColorMode::Pattern, &[], false);
        assert_reflected(&off, &on, cx);
        // Mirroring the flat tiling does move real bytes (the grid is not
        // symmetric about the frame center).
        assert_ne!(on, off, "the flat copy must differ from the off-state");
    }

    #[test]
    fn mirror_on_with_walls_reflects_line_endpoints() {
        let mut tiles = square_grid(4);
        for t in &mut tiles {
            t.level = 2;
        }
        let ws = walls(&tiles, 45.0);
        assert!(!ws.is_empty(), "need a non-empty wall set");
        let bbox = BBox::new(pt(-2.0, -2.0), pt(6.0, 6.0));
        let cx = bbox.center().x;
        let off = render_svg_with_walls(&tiles, &bbox, ColorMode::Pattern, &ws);
        let on = render_svg_with_walls_highlight_mirrored(&tiles, &bbox, ColorMode::Pattern, &ws, false);
        assert_reflected(&off, &on, cx);
    }

    #[test]
    fn mirror_on_composes_with_highlight_supertiles() {
        // highlight=true splits the outline pass into base + heavy groups;
        // the mirrored on-state must reflect every path of both groups,
        // keep both groups and the stroke weights, and leave the preamble
        // and the closing tag byte-identical.
        let tiles = mixed_level_tiles();
        let bbox = BBox::new(pt(-1.0, -1.0), pt(3.0, 2.0));
        let cx = bbox.center().x;
        let off = render_svg_with_walls_highlight(&tiles, &bbox, ColorMode::Pattern, &[], true);
        let on = render_svg_with_walls_highlight_mirrored(&tiles, &bbox, ColorMode::Pattern, &[], true);
        assert_reflected(&off, &on, cx);
        let spans = outline_group_spans(&on);
        assert_eq!(spans.len(), 2, "base + heavy group: {on}");
        let base = &on[spans[0].0..spans[0].1];
        let heavy = &on[spans[1].0..spans[1].1];
        assert!(base.contains(r#"stroke-width="0.0120""#), "{base}");
        assert!(heavy.contains(r#"stroke-width="0.0240""#), "{heavy}");
        assert!(!base.contains(r#"stroke-width="0.0240""#), "{base}");
        assert!(!heavy.contains(r#"stroke-width="0.0120""#), "{heavy}");
        let cut = off.find("<path ").unwrap();
        assert_eq!(&off[..cut], &on[..cut], "the preamble must be byte-identical");
    }

    #[test]
    fn mirror_on_flat_tiling_under_highlight_is_the_reflected_single_group() {
        // Flat tiling + highlight = the legacy single group; mirroring it
        // must reflect every path and keep exactly one group.
        let tiles = square_grid(2);
        let bbox = BBox::new(pt(0.0, 0.0), pt(2.0, 2.0));
        let cx = bbox.center().x;
        let off = render_svg_with_walls_highlight(&tiles, &bbox, ColorMode::Pattern, &[], true);
        let on = render_svg_with_walls_highlight_mirrored(&tiles, &bbox, ColorMode::Pattern, &[], true);
        assert_reflected(&off, &on, cx);
        assert_eq!(on.matches("<g fill=\"none\"").count(), 1, "single group");
    }

    #[test]
    fn mirror_on_degenerate_tiles_stay_degenerate() {
        let mut tiles = mixed_level_tiles();
        tiles.push(Tile {
            proto: 2,
            seed: 2,
            level: 0,
            poly: Polygon::from_slice(&[pt(2.0, 0.0), pt(3.0, 0.0)]),
        });
        let bbox = BBox::new(pt(-1.0, -1.0), pt(4.0, 2.0));
        let cx = bbox.center().x;
        let off = render_svg_with_walls(&tiles, &bbox, ColorMode::Pattern, &[]);
        let on = render_svg_with_walls_highlight_mirrored(&tiles, &bbox, ColorMode::Pattern, &[], false);
        assert_reflected(&off, &on, cx);
        // Degenerate tiles emit no path anywhere (fill or outline).
        assert_eq!(path_tags(&on).len(), path_tags(&off).len());
        assert_eq!(path_tags(&on).iter().filter(|(_, f)| *f).count(), 2, "only the two non-degenerate tiles carry a fill");
    }
}
