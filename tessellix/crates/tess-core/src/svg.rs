//! SVG rendering of a tiling (shared by CLI export).

use crate::geometry::{BBox, Polygon, Vec2};
use crate::paint::{rgb, tile_rgb_reflected, BORDER, BACKGROUND};
use crate::strapwork::{DecorationPlan, Wall};
use crate::symmetry::overlay::SymmetryOverlay;
use crate::tiling::Tile;
use crate::view::ColorMode;

mod orbits;
pub use orbits::{vertex_orbits, VertexOrbit};

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

/// Accent color of the Landing B large-tile outline pass
/// ([`AperiodicToggles::large_tile_outline`]): vivid orange. Deliberately a
/// hue NO tiling ramp uses and distinct from every annotation accent already
/// in the file ([`OVERLAY_CELL_GOLD`], [`OVERLAY_ROT_ROSE`],
/// [`OVERLAY_MIRROR_GREEN`], [`OVERLAY_GLIDE_CYAN`], [`ORBIT_A`],
/// [`ORBIT_B`]), so the outline stays readable when the flags compose with
/// `--symmetry-overlay` or `--vertex-orbits`.
pub const LARGE_TILE_ACCENT: u32 = 0xFF7A1A;

/// Stroke-weight multiplier of that pass against the base tile-border
/// stroke. Its own constant (the same value as
/// [`SUPERTILE_STROKE_FACTOR`] today) because the two passes are keyed
/// differently — substitution depth vs. family — and must be able to diverge
/// without touching each other.
pub const LARGE_TILE_STROKE_FACTOR: f64 = 2.0;

/// The Landing B render toggles: the two passes that read per-tile family
/// and chirality facts. Bundled into one `Copy` struct so the full render
/// entries stay readable instead of growing a seventh and eighth positional
/// `bool`, and so `Default` is exactly "both off = the pre-Landing-B render".
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AperiodicToggles {
    /// `--highlight-reflected`: fill reflected tiles (`tile.reflected`) in
    /// the lightened accent of their own mode color
    /// ([`crate::paint::tile_rgb_reflected`]). Off ⇒ byte-identical.
    pub highlight_reflected: bool,
    /// `--large-tile-outline`: append a family-keyed accent outline group
    /// over the whole render (see [`large_tile_outlined`]). Off ⇒ nothing is
    /// appended, byte-identical.
    pub large_tile_outline: bool,
}

/// Which tiles the [`AperiodicToggles::large_tile_outline`] pass marks —
/// the "large tile" of each aperiodic family, i.e. the tile the family card
/// is ABOUT. Public because the TUI canvas marks exactly the same set
/// ([`crate::raster::raster_aperiodic`]): one law, both surfaces:
///
/// * `FamilyTag::Hat` / `FamilyTag::Spectre`: monohedral cards — every
///   tile is the prototile, so every tile is outlined.
/// * [`FamilyTag::Turtle`]: the turtle shape (`proto == 0`) only. The Mystic
///   Sigma's companion hat (`proto == 1`) is the other tile type, not the
///   card's subject, and stays unoutlined.
/// * Any other family (untagged cards included): nothing. The flag is a
///   no-op rather than an error, so a script can pass it uniformly.
pub fn large_tile_outlined(tile: &Tile) -> bool {
    use crate::tiling::FamilyTag;
    match tile.family {
        FamilyTag::Hat | FamilyTag::Spectre => true,
        FamilyTag::Turtle => tile.proto == 0,
        _ => false,
    }
}

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
/// reflected about the frame center, everything else untouched except
/// the `reflected` flag, which flips — a mirror adds one y-axis
/// reflection to each placement (the mirror pass is an involution on
/// the flag: canonical ↔ reflected).
fn mirror_tiles(tiles: &[Tile], cx: f64) -> Vec<Tile> {
    tiles
        .iter()
        .map(|t| Tile { family: t.family, reflected: !t.reflected,
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
///
/// Exactly [`render_svg_with_walls_overlay`] with `mirror = false`
/// and `overlay = None`: byte-identical output at this entry point
/// (M205 contract).
pub fn render_svg_with_walls_highlight(
    tiles: &[Tile],
    bbox: &BBox,
    mode: ColorMode,
    walls: &[Wall],
    highlight_supertiles: bool,
) -> String {
    render_svg_with_walls_overlay(tiles, bbox, mode, walls, highlight_supertiles, false, None)
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
    render_svg_with_walls_overlay(tiles, bbox, mode, walls, highlight_supertiles, true, None)
}

/// The SVG scaffolding shared by every tiling render: the `<svg>` element
/// with the y-axis flipped viewBox, the `<g transform="...">` y-flip group
/// and the paper background rect. Shared by the single-tiling and the
/// two-layer primal+dual renders so both emit identical scaffolding bytes.
fn svg_scaffold(bbox: &BBox) -> String {
    let w = bbox.size().x;
    let h = bbox.size().y;
    let (ar, ag, ab) = rgb(BACKGROUND);
    let mut s = String::new();
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
    s
}

/// The closing tag of the y-flip group and the `<svg>` element.
fn svg_close() -> &'static str {
    "</g></svg>"
}

/// The two-pass body of one tiling render: every tile fill first, then
/// every tile outline (SVG paints each element's fill and stroke in
/// document order, so a single interleaved pass lets a later tile's fill
/// cover the earlier tile's stroke half — the discipline below).
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
/// [mirror = true]: every emitted coordinate is reflected about the frame
/// center first (see [`render_svg_with_walls_highlight_mirrored`]).
fn svg_body(
    tiles: &[Tile],
    bbox: &BBox,
    mode: ColorMode,
    highlight_supertiles: bool,
    mirror: bool,
) -> String {
    svg_body_aperiodic(
        tiles,
        bbox,
        mode,
        highlight_supertiles,
        mirror,
        AperiodicToggles::default(),
    )
}

/// [`svg_body`] with the Landing B toggles. Both off (the delegator above)
/// is byte-identical by construction: `highlight_reflected = false` makes
/// [`tile_rgb_reflected`] read exactly [`crate::paint::tile_rgb`], and
/// `large_tile_outline = false` appends no group at all — every existing
/// byte golden (the M205 off-state pin included) stays green.
///
/// On-states, and why they sit where they do:
///
/// * `highlight_reflected` changes only the fill colors of pass 1, so the
///   outline passes (legacy single group or M205 level split) are untouched
///   and keep their own geometry and stroke weights. The two highlights are
///   orthogonal: M205 keys on `tile.level` (stroke weight), this keys on
///   `tile.reflected` (fill).
/// * `large_tile_outline` is a THIRD group appended after the last outline
///   pass, never a re-split of pass 2 — it must paint over every tile
///   border (that is the point of an outline of a whole patch region) and it
///   must not disturb the level split above it. Family-keyed membership
///   ([`large_tile_outlined`]); empty membership emits nothing, so the flag
///   is a no-op for every non-aperiodic card.
///
/// `mirror` reflects everything, including both new passes (the tiles slice
/// below is already the mirrored one).
fn svg_body_aperiodic(
    tiles: &[Tile],
    bbox: &BBox,
    mode: ColorMode,
    highlight_supertiles: bool,
    mirror: bool,
    toggles: AperiodicToggles,
) -> String {
    let cx = bbox.center().x;
    let owned_tiles;
    let tiles: &[Tile] = if mirror {
        owned_tiles = mirror_tiles(tiles, cx);
        &owned_tiles
    } else {
        tiles
    };
    let w = bbox.size().x;
    let h = bbox.size().y;
    let stroke = (0.004 * w.min(h)).max(0.001);
    let heavy = stroke * SUPERTILE_STROKE_FACTOR;
    let (br, bg_, bb) = rgb(BORDER);

    let mut s = String::with_capacity(tiles.len() * 300 + 256);
    // Pass 1: all fills, no outlines.
    for tile in tiles {
        if let Some(d) = path_d(tile) {
            let (r, g, b) = rgb(tile_rgb_reflected(mode, tile, toggles.highlight_reflected));
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
    // Pass 3 (Landing B): the large-tile outline, over everything above.
    // Appended, never interleaved: the two-pass discipline (all fills, then
    // all grout) stays exactly as it was, and this reads as one clearly
    // marked annotation group — `data-large-tile-outline` — that a test or a
    // stylesheet can address. Off, or for a family with no marked tile,
    // nothing is emitted at all.
    if toggles.large_tile_outline {
        let marked: Vec<&Tile> = tiles.iter().filter(|t| large_tile_outlined(t)).collect();
        if !marked.is_empty() {
            let (ar, ag, ab) = rgb(LARGE_TILE_ACCENT);
            let accent = stroke * LARGE_TILE_STROKE_FACTOR;
            s.push_str(&format!(
                r#"<g data-large-tile-outline="true" fill="none" stroke="rgb({ar},{ag},{ab})" stroke-width="{accent:.4}" stroke-linejoin="round">"#
            ));
            for tile in marked {
                if let Some(d) = path_d(tile) {
                    s.push_str(&format!(r#"<path d="{}"/>"#, d));
                }
            }
            s.push_str("</g>");
        }
    }
    s
}

/// The strapwork wall pass (round caps, on top of the grout): stroked in
/// the border color at a width proportional to the view extent
/// (`0.012·min(w, h)`, floored at `0.003`) — three times the tile-border
/// stroke — so the ribbons stay clearly readable above the fills at any
/// export size without swallowing thin cells. Empty input emits nothing.
fn svg_walls(walls: &[Wall], bbox: &BBox, mirror: bool) -> String {
    let cx = bbox.center().x;
    let owned_walls;
    let walls: &[Wall] = if mirror {
        owned_walls = mirror_walls(walls, cx);
        &owned_walls
    } else {
        walls
    };
    if walls.is_empty() {
        return String::new();
    }
    let w = bbox.size().x;
    let h = bbox.size().y;
    let wall_stroke = (0.012 * w.min(h)).max(0.003);
    let (br, bg_, bb) = rgb(BORDER);
    let mut s = String::with_capacity(walls.len() * 64 + 256);
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
    s
}

/// Symmetry-overlay accent colours (C5) — high-saturation annotations in
/// hues the tiling palette never uses:
///
/// * unit cell — gold outline;
/// * rotation centers — rose filled circle + `fold` spokes;
/// * mirror lines — solid green;
/// * glide axes — dashed cyan.
pub const OVERLAY_CELL_GOLD: u32 = 0xFFC107;
pub const OVERLAY_ROT_ROSE: u32 = 0xF43F5E;
pub const OVERLAY_MIRROR_GREEN: u32 = 0x10B981;
pub const OVERLAY_GLIDE_CYAN: u32 = 0x22D3EE;

/// Vertex-orbit accent colours (C1) — high-saturation filled dots for the
/// two vertex-figure classes a 2-uniform card displays. The hues are
/// chosen to sit clear of both the tile palette and the C5 overlay
/// accents, and to be maximally separable from each other (pink ≈ 330° vs
/// lime ≈ 83°) so the two classes read apart on the dark background at
/// the dot's small radius:
///
/// * [`ORBIT_A`] — the lexicographically first figure class;
/// * [`ORBIT_B`] — the lexicographically second figure class.
///
/// A 2-uniform tiling always yields exactly two classes, so the
/// two-colour scheme is complete; if a card ever produced more than two
/// classes, the colours alternate A/B/A/... in orbit order (documented
/// behaviour, not a claim about the current registry).
pub const ORBIT_A: u32 = 0xEC4899;
pub const ORBIT_B: u32 = 0xA3E635;

/// Full single-tiling render entry (C5): tiles + strapwork walls +
/// optional supertile highlight + optional mirror (enantiomorph) +
/// optional certified-symmetry overlay. The two-pass body, the wall pass
/// and the scaffolding are the shared helpers ([`svg_body`], [`svg_walls`],
/// [`svg_scaffold`], [`svg_close`]) exactly as the C2 dual entry threads
/// them; the legacy entries above are precisely this with
/// `mirror = false` / `overlay = None`.
///
/// [mirror = true]: every emitted coordinate is reflected about the frame
/// center first (see [`render_svg_with_walls_highlight_mirrored`]); the
/// overlay (when present) is appended after the mirrored walls.
///
/// Symmetry overlay (`Some(overlay)`, C5): after the walls group, a
/// `<g data-symmetry-overlay="true">` block is appended — the unit-cell
/// parallelogram (closed gold path), the certified rotation centers (rose
/// circle + `fold` spokes, one group per site), the certified mirror
/// lines (solid green) and the certified glide axes (dashed cyan) — all
/// in world coordinates inside the y-flipped group, line glyphs clipped
/// to `bbox`. With `overlay = None` the output is byte-identical to the
/// M205 entries for every input.
pub fn render_svg_with_walls_overlay(
    tiles: &[Tile],
    bbox: &BBox,
    mode: ColorMode,
    walls: &[Wall],
    highlight_supertiles: bool,
    mirror: bool,
    overlay: Option<&SymmetryOverlay>,
) -> String {
    let mut s = svg_scaffold(bbox);
    s.push_str(&svg_body(tiles, bbox, mode, highlight_supertiles, mirror));
    s.push_str(&svg_walls(walls, bbox, mirror));
    if let Some(ov) = overlay {
        append_symmetry_overlay(&mut s, ov, bbox);
    }
    s.push_str(svg_close());
    s
}

/// Full single-tiling render entry (C1, extending the C5 entry): tiles +
/// strapwork walls + optional supertile highlight + optional mirror +
/// optional certified-symmetry overlay + optional vertex-figure orbit
/// dots. Same shared-helper composition as
/// [`render_svg_with_walls_overlay`], plus the vertex-orbit group
/// appended after the symmetry overlay (or after the walls when the
/// overlay is absent) and before [`svg_close`].
///
/// With `orbits = None` the output is byte-identical to
/// [`render_svg_with_walls_overlay`] for every input (the C5/M205
/// byte-identity contract); `Some(&[])` is the same no-op (an empty orbit
/// list emits no group).
pub fn render_svg_with_walls_orbits(
    tiles: &[Tile],
    bbox: &BBox,
    mode: ColorMode,
    walls: &[Wall],
    highlight_supertiles: bool,
    mirror: bool,
    overlay: Option<&SymmetryOverlay>,
    orbits: Option<&[VertexOrbit]>,
) -> String {
    let mut s = svg_scaffold(bbox);
    s.push_str(&svg_body(tiles, bbox, mode, highlight_supertiles, mirror));
    s.push_str(&svg_walls(walls, bbox, mirror));
    if let Some(ov) = overlay {
        append_symmetry_overlay(&mut s, ov, bbox);
    }
    if let Some(os) = orbits {
        append_vertex_orbits(&mut s, os, bbox, mirror);
    }
    s.push_str(svg_close());
    s
}

/// Full single-tiling render entry (Landing B, extending the C1 entry):
/// everything [`render_svg_with_walls_orbits`] takes, plus
/// [`AperiodicToggles`]. Same helper composition, with the toggles threaded
/// into the body; the outline group lands inside the body (after the outline
/// passes), the wall/overlay/orbit groups after it in the same order as
/// before.
///
/// `toggles == AperiodicToggles::default()` is byte-identical to
/// [`render_svg_with_walls_orbits`] for every input — the same off-state
/// contract the M205/C5/C1 entries hold, so every existing golden holds.
pub fn render_svg_with_walls_orbits_aperiodic(
    tiles: &[Tile],
    bbox: &BBox,
    mode: ColorMode,
    walls: &[Wall],
    highlight_supertiles: bool,
    mirror: bool,
    overlay: Option<&SymmetryOverlay>,
    orbits: Option<&[VertexOrbit]>,
    toggles: AperiodicToggles,
) -> String {
    let mut s = svg_scaffold(bbox);
    s.push_str(&svg_body_aperiodic(
        tiles,
        bbox,
        mode,
        highlight_supertiles,
        mirror,
        toggles,
    ));
    s.push_str(&svg_walls(walls, bbox, mirror));
    if let Some(ov) = overlay {
        append_symmetry_overlay(&mut s, ov, bbox);
    }
    if let Some(os) = orbits {
        append_vertex_orbits(&mut s, os, bbox, mirror);
    }
    s.push_str(svg_close());
    s
}

/// Primal + dual in one frame (C2): two groups inside one `<svg>` —
/// `<g id="primal">` over the primal tiles, then `<g id="dual">` over the
/// dual tiles, so the dual (the card's subject) paints on top — sharing
/// the scaffolding (paper, y-flip transform, background rect). The
/// strapwork `walls` are the primal's walls and are emitted once, on top of
/// both groups: the dual group is a plain fill/outline group, never walls.
/// `highlight_supertiles` applies to both groups identically (both emitted
/// by [`svg_body`]); `mirror` reflects both groups about the frame center
/// identically. Both groups must be generated from the same params and
/// region (the caller's job) — the geometry is frame-aligned because the
/// dual shares the primal lattice and the similarity map.
pub fn render_svg_dual_overlay(
    primal: &[Tile],
    dual: &[Tile],
    bbox: &BBox,
    mode: ColorMode,
    walls: &[Wall],
    highlight_supertiles: bool,
    mirror: bool,
) -> String {
    render_svg_dual_overlay_aperiodic(
        primal,
        dual,
        bbox,
        mode,
        walls,
        highlight_supertiles,
        mirror,
        AperiodicToggles::default(),
    )
}

/// [`render_svg_dual_overlay`] with the Landing B toggles
/// (`--highlight-reflected` / `--large-tile-outline` compose with
/// `--dual-overlay`). Both groups receive the toggles, exactly as
/// `highlight_supertiles` and `mirror` do; the toggles key on per-tile facts
/// the centroid dual does not carry, so on the dual group they are inert by
/// construction — dual cells are generated untagged (`FamilyTag::None`, so
/// the outline pass marks none) and unreflected (`reflected: false`, so no
/// fill is lightened). Default toggles are byte-identical to
/// [`render_svg_dual_overlay`].
pub fn render_svg_dual_overlay_aperiodic(
    primal: &[Tile],
    dual: &[Tile],
    bbox: &BBox,
    mode: ColorMode,
    walls: &[Wall],
    highlight_supertiles: bool,
    mirror: bool,
    toggles: AperiodicToggles,
) -> String {
    let mut s = svg_scaffold(bbox);
    s.push_str(r#"<g id="primal">"#);
    s.push_str(&svg_body_aperiodic(
        primal,
        bbox,
        mode,
        highlight_supertiles,
        mirror,
        toggles,
    ));
    s.push_str("</g>");
    s.push_str(r#"<g id="dual">"#);
    s.push_str(&svg_body_aperiodic(
        dual,
        bbox,
        mode,
        highlight_supertiles,
        mirror,
        toggles,
    ));
    s.push_str("</g>");
    s.push_str(&svg_walls(walls, bbox, mirror));
    s.push_str(svg_close());
    s
}

// ──────────────────────────────────────────────────────────────────────────
// Symmetry overlay emission (C5).
// ──────────────────────────────────────────────────────────────────────────

/// Endpoints of the segment of the infinite line (point `p`, unit
/// direction `u`) that crosses the axis-aligned `bbox` — `None` when the
/// intersection has zero length (e.g. it only touches a corner, which the
/// derivation already drops).
fn line_bbox_endpoints(p: Vec2, u: Vec2, bbox: &BBox) -> Option<(Vec2, Vec2)> {
    let mut tlo = f64::NEG_INFINITY;
    let mut thi = f64::INFINITY;
    for (lo, hi, cu, c0) in [
        (bbox.min.x, bbox.max.x, u.x, p.x),
        (bbox.min.y, bbox.max.y, u.y, p.y),
    ] {
        if cu.abs() < 1e-12 {
            if !(lo - 1e-9 <= c0 && c0 <= hi + 1e-9) {
                return None;
            }
        } else {
            let a = (lo - c0) / cu;
            let b = (hi - c0) / cu;
            tlo = tlo.max(a.min(b));
            thi = thi.min(a.max(b));
        }
    }
    if thi - tlo < 1e-9 {
        return None;
    }
    Some((p + u * tlo, p + u * thi))
}

/// Closed four-point path data for a unit cell, `None` if any corner is
/// non-finite.
fn cell_path(cell: &[Vec2; 4]) -> Option<String> {
    for p in cell {
        if !p.x.is_finite() || !p.y.is_finite() {
            return None;
        }
    }
    let mut d = format!("M{:.4} {:.4}", cell[0].x, cell[0].y);
    for p in &cell[1..] {
        d.push_str(&format!("L{:.4} {:.4}", p.x, p.y));
    }
    d.push('Z');
    Some(d)
}

/// Emit the `<g data-symmetry-overlay="true">` block: unit cell, rotation
/// centers, mirrors, glides — in that order, in world coordinates inside
/// the y-flipped group. Deterministic: centers sorted by (x, y, fold);
/// lines sorted by (direction, offset).
fn append_symmetry_overlay(s: &mut String, ov: &SymmetryOverlay, bbox: &BBox) {
    let w = bbox.size().x;
    let h = bbox.size().y;
    let stroke = (0.008 * w.min(h)).max(0.002);
    let (cr, cg, cb) = rgb(OVERLAY_CELL_GOLD);
    let (rr, rg, rb) = rgb(OVERLAY_ROT_ROSE);
    let (mr, mg, mb) = rgb(OVERLAY_MIRROR_GREEN);
    let (gr, gg, gb) = rgb(OVERLAY_GLIDE_CYAN);
    let radius = 0.12 * ov.scale;
    let spoke = 0.28 * ov.scale;

    s.push_str("<g data-symmetry-overlay=\"true\" fill=\"none\" stroke-linecap=\"round\">");

    // Unit cell: closed gold parallelogram.
    if let Some(d) = cell_path(&ov.cell) {
        s.push_str(&format!(
            r#"<path data-sym="cell" d="{}" stroke="rgb({},{},{})" stroke-width="{:.4}" stroke-linejoin="round"/>"#,
            d, cr, cg, cb, stroke
        ));
    }

    // Rotation centers: rose filled circle + `fold` spokes at 2πk/fold.
    let mut centers = ov.centers.clone();
    centers.sort_by(|a, b| {
        a.point.x.partial_cmp(&b.point.x).unwrap()
            .then_with(|| a.point.y.partial_cmp(&b.point.y).unwrap())
            .then_with(|| a.fold.cmp(&b.fold))
    });
    for c in &centers {
        s.push_str(&format!(
            r#"<g data-sym="rot-{}" stroke="rgb({},{},{})" stroke-width="{:.4}">"#,
            c.fold, rr, rg, rb, stroke
        ));
        s.push_str(&format!(
            r#"<circle cx="{:.4}" cy="{:.4}" r="{:.4}" fill="rgb({},{},{})"/>"#,
            c.point.x, c.point.y, radius, rr, rg, rb
        ));
        let n = c.fold as i64;
        for k in 0..n {
            let ang = std::f64::consts::TAU * k as f64 / n as f64;
            s.push_str(&format!(
                r#"<line x1="{:.4}" y1="{:.4}" x2="{:.4}" y2="{:.4}"/>"#,
                c.point.x,
                c.point.y,
                c.point.x + spoke * ang.cos(),
                c.point.y + spoke * ang.sin()
            ));
        }
        s.push_str("</g>");
    }

    // Mirrors: solid green lines clipped to the bbox.
    let mut mirrors = ov.mirrors.clone();
    mirrors.sort_by(|a, b| {
        let sa = a.point.dot(Vec2::new(-a.direction.y, a.direction.x));
        let sb = b.point.dot(Vec2::new(-b.direction.y, b.direction.x));
        a.direction.x.partial_cmp(&b.direction.x).unwrap()
            .then_with(|| a.direction.y.partial_cmp(&b.direction.y).unwrap())
            .then_with(|| sa.partial_cmp(&sb).unwrap())
    });
    for m in &mirrors {
        if let Some((a, b)) = line_bbox_endpoints(m.point, m.direction, bbox) {
            s.push_str(&format!(
                r#"<line data-sym="mirror" x1="{:.4}" y1="{:.4}" x2="{:.4}" y2="{:.4}" stroke="rgb({},{},{})" stroke-width="{:.4}"/>"#,
                a.x, a.y, b.x, b.y, mr, mg, mb, stroke
            ));
        }
    }

    // Glides: dashed cyan lines (dasharray 3:2 of the stroke).
    let mut glides = ov.glides.clone();
    glides.sort_by(|a, b| {
        let sa = a.point.dot(Vec2::new(-a.direction.y, a.direction.x));
        let sb = b.point.dot(Vec2::new(-b.direction.y, b.direction.x));
        a.direction.x.partial_cmp(&b.direction.x).unwrap()
            .then_with(|| a.direction.y.partial_cmp(&b.direction.y).unwrap())
            .then_with(|| sa.partial_cmp(&sb).unwrap())
    });
    let dash_on = (3.0 * stroke).max(0.001);
    let dash_off = (2.0 * stroke).max(0.001);
    for g in &glides {
        if let Some((a, b)) = line_bbox_endpoints(g.point, g.direction, bbox) {
            s.push_str(&format!(
                r#"<line data-sym="glide" x1="{:.4}" y1="{:.4}" x2="{:.4}" y2="{:.4}" stroke="rgb({},{},{})" stroke-width="{:.4}" stroke-dasharray="{:.4} {:.4}"/>"#,
                a.x, a.y, b.x, b.y, gr, gg, gb, stroke, dash_on, dash_off
            ));
        }
    }

    s.push_str("</g>");
}

// ──────────────────────────────────────────────────────────────────────────
// Vertex figure orbit dots (C1).
// ──────────────────────────────────────────────────────────────────────────

/// Emit the `<g data-vertex-orbits="true">` block (C1): one inner
/// `<g data-orbit-figure="...">` per orbit (in orbit order — lexicographic
/// figure order, so orbit 0 = [`ORBIT_A`], orbit 1 = [`ORBIT_B`]; extras
/// alternate A/B/A/…), then one filled circle per vertex. Deterministic:
/// orbits arrive sorted by figure key and each orbit's points are sorted
/// by (x, y). Empty `orbits` emits nothing.
///
/// [mirror = true]: every dot is reflected about the frame center
/// (`x → 2·cx − x`) — the same convention the tile and wall passes apply,
/// so the dots track the mirrored geometry (not an SVG `transform`).
fn append_vertex_orbits(s: &mut String, orbits: &[VertexOrbit], bbox: &BBox, mirror: bool) {
    if orbits.is_empty() {
        return;
    }
    let w = bbox.size().x;
    let h = bbox.size().y;
    let radius = (0.008 * w.min(h)).max(0.003);
    let cx = bbox.center().x;
    s.push_str(r#"<g data-vertex-orbits="true">"#);
    for (i, orbit) in orbits.iter().enumerate() {
        let (r, g, b) = rgb(if i % 2 == 0 { ORBIT_A } else { ORBIT_B });
        s.push_str(&format!(
            r#"<g data-orbit-figure="{}" fill="rgb({},{},{})">"#,
            orbit.figure, r, g, b
        ));
        for p in &orbit.points {
            let p = if mirror { mirror_point(*p, cx) } else { *p };
            s.push_str(&format!(
                r#"<circle cx="{:.4}" cy="{:.4}" r="{:.4}"/>"#,
                p.x, p.y, radius
            ));
        }
        s.push_str("</g>");
    }
    s.push_str("</g>");
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
        Tile { family: crate::tiling::FamilyTag::None, reflected: false,
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
    fn mirror_tiles_forwards_family_and_flips_reflected() {
        // Landing A contract: the mirror pass keeps `proto`/`seed`/
        // `level`/`family` untouched and FLIPS `reflected` — a mirror
        // adds one y-axis reflection to each placement (the field's
        // definition), so the pass is an involution on the flag.
        let mut base = tile(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        base.family = crate::tiling::FamilyTag::Penrose;
        base.proto = 7;
        base.seed = 42;
        base.level = 3;
        assert!(!base.reflected);
        let mirrored = mirror_tiles(&[base.clone()], 0.5);
        assert_eq!(mirrored.len(), 1);
        let m = &mirrored[0];
        assert_eq!(m.family, crate::tiling::FamilyTag::Penrose, "family must survive the mirror");
        assert!(m.reflected, "mirror must flip false -> true");
        assert_eq!(m.proto, base.proto);
        assert_eq!(m.seed, base.seed);
        assert_eq!(m.level, base.level);
        // The flip is an involution: mirroring a `reflected` tile yields
        // a canonical one again (Landing B's Hat tagging will produce
        // such inputs; set-true semantics would corrupt them).
        let mut refl = tile(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        refl.reflected = true;
        let rm = mirror_tiles(&[refl], 0.5);
        assert!(!rm[0].reflected, "mirror must flip true -> false");
        // An untagged tile mirrors to an untagged (but reflected) tile.
        let plain = tile(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        let pm = mirror_tiles(&[plain], 0.0);
        assert_eq!(pm[0].family, crate::tiling::FamilyTag::None);
        assert!(pm[0].reflected);
    }

    #[test]
    fn pattern_gray_emission_is_gray_only() {
        // End-to-end SVG emission through tile_rgb: one tile per seed
        // 0..8 → the eight PATTERN_GRAY_PALETTE values, no hue palette
        // value anywhere.
        let tiles: Vec<Tile> = (0..8u32)
            .map(|seed| {
                let x = seed as f64;
                Tile { family: crate::tiling::FamilyTag::None, reflected: false,
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
            Tile { family: crate::tiling::FamilyTag::None, reflected: false,
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
            Tile { family: crate::tiling::FamilyTag::None, reflected: false,
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
        tiles.push(Tile { family: crate::tiling::FamilyTag::None, reflected: false,
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

    // ---- symmetry overlay (render_svg_with_walls_overlay, C5) ----------

    use crate::symmetry::overlay::{GlideAxis, MirrorLine, RotationCenter};
    use crate::symmetry::WallpaperGroup;

    /// A small synthetic overlay: one unit cell, one 4-fold center, one
    /// mirror and one glide. Enough structure to exercise every glyph
    /// class in the emitter.
    fn synth_overlay() -> SymmetryOverlay {
        SymmetryOverlay {
            group: WallpaperGroup::P4m,
            cell: [pt(0.0, 0.0), pt(1.0, 0.0), pt(1.0, 1.0), pt(0.0, 1.0)],
            centers: vec![RotationCenter {
                point: pt(0.5, 0.5),
                fold: 4,
            }],
            mirrors: vec![MirrorLine {
                point: pt(0.0, 0.5),
                direction: Vec2::new(1.0, 0.0),
            }],
            glides: vec![GlideAxis {
                point: pt(0.0, 1.5),
                direction: Vec2::new(1.0, 0.0),
                glide: Vec2::new(0.5, 0.0),
            }],
            scale: 1.0,
        }
    }

    #[test]
    fn overlay_off_output_is_byte_pinned() {
        // Full golden pin of the new entry with `overlay = None`: byte
        // identical to the legacy M205 off-state bytes, so the new
        // delegation cannot drift the default export. Same tiling and
        // bbox as highlight_off_output_is_byte_pinned.
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
        assert_eq!(
            render_svg_with_walls_overlay(&tiles, &bbox, ColorMode::Single, &[], false, false, None),
            expected,
            "None overlay must be the legacy bytes"
        );
        // The delegation is exact: the legacy entry and the new entry with
        // None produce the same bytes.
        assert_eq!(
            render_svg_with_walls_overlay(&tiles, &bbox, ColorMode::Single, &[], true, false, None),
            render_svg_with_walls_highlight(&tiles, &bbox, ColorMode::Single, &[], true),
        );
    }

    #[test]
    fn overlay_on_structure_and_zorder() {
        // On-state: one element per site type, correct z-order (after the
        // wall group), and on == off with the overlay group stripped.
        let tiles = square_grid(4);
        let ws = walls(&tiles, 45.0);
        assert!(!ws.is_empty(), "need a non-empty wall set");
        let bbox = BBox::new(pt(-2.0, -2.0), pt(6.0, 6.0));
        let ov = synth_overlay();
        let off = render_svg_with_walls_overlay(&tiles, &bbox, ColorMode::Pattern, &ws, false, false, None);
        let on = render_svg_with_walls_overlay(&tiles, &bbox, ColorMode::Pattern, &ws, false, false, Some(&ov));

        // Element inventory (one per site type in the synthetic overlay).
        assert_eq!(on.matches(r#"data-sym="cell""#).count(), 1);
        assert_eq!(on.matches(r#"data-sym="rot-4""#).count(), 1);
        assert_eq!(on.matches(r#"data-sym="mirror""#).count(), 1);
        assert_eq!(on.matches(r#"data-sym="glide""#).count(), 1);
        assert_eq!(on.matches(r#"<g data-symmetry-overlay="true""#).count(), 1);

        // The rot-4 group holds exactly one filled circle plus 4 spokes.
        let g0 = on.find(r#"<g data-sym="rot-4""#).unwrap();
        let g1 = &on[g0..];
        let gend = g1.find("</g>").unwrap() + "</g>".len();
        let gblock = &g1[..gend];
        assert_eq!(gblock.matches("<circle").count(), 1, "{gblock}");
        assert_eq!(gblock.matches("<line").count(), 4, "4 spokes for a 4-fold center: {gblock}");
        assert!(gblock.contains("fill=\"rgb(244,63,94)\""), "rot circle is rose: {gblock}");

        // Z-order: the overlay group is the LAST group in the document —
        // it follows every wall line and every outline. The wall group is
        // the second `<g fill="none">` span (outline group + wall group;
        // the overlay group does not match that opener, so it is excluded).
        let tail = "</g></svg>";
        assert!(on.ends_with(tail), "{on}");
        let marker = r#"<g data-symmetry-overlay="true""#;
        let idx = on.find(marker).unwrap();
        let spans = outline_group_spans(&on);
        assert_eq!(spans.len(), 2, "outline group + wall group (overlay excluded): {on}");
        let wall_end = spans[1].1;
        // The wall group's closing `</g>` ends exactly where the overlay
        // group begins (or the wall group is absent and the overlay
        // follows the outline group directly).
        assert!(wall_end <= idx, "overlay must be appended after the wall group");

        // on == off with the overlay block stripped: the overlay is the
        // tail group, so removing it recovers the off-state bytes exactly.
        assert_eq!(&on[..idx], &off[..off.len() - tail.len()], "on minus overlay == off");
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
    fn dual_overlay_marks_the_two_groups_and_shares_the_scaffolding() {
        // Two tile sets (a flat primal grid and a different dual set),
        // a non-empty wall list — the output must be:
        //   scaffolding + <g id="primal"> body(primal) </g>
        //               + <g id="dual"> body(dual) </g> + walls + close
        // byte-for-byte, with both groups' bodies and the single wall
        // pass re-derived from the same helpers.
        let primal = square_grid(2); // 4 tiles
        let dual = vec![
            tile(&[(0.0, 0.0), (1.5, 0.0), (1.5, 1.5), (0.0, 1.5)]),
            tile(&[(1.5, 1.5), (3.0, 1.5), (3.0, 3.0), (1.5, 3.0)]),
            tile(&[(0.0, 1.5), (1.5, 1.5), (1.5, 3.0), (0.0, 3.0)]),
        ]; // 3 tiles
        let walls = vec![
            Wall { a: pt(0.5, -0.5), b: pt(1.5, 1.5) },
            Wall { a: pt(2.0, 0.5), b: pt(2.5, 2.0) },
        ];
        let bbox = BBox::new(pt(-0.5, -0.5), pt(3.5, 3.5));

        let svg = render_svg_dual_overlay(
            &primal, &dual, &bbox, ColorMode::Pattern, &walls, false, false,
        );

        // Byte-for-byte against the shared scaffold + bodies + walls.
        let expected = format!(
            "{}<g id=\"primal\">{}</g><g id=\"dual\">{}</g>{}{}",
            svg_scaffold(&bbox),
            svg_body(&primal, &bbox, ColorMode::Pattern, false, false),
            svg_body(&dual, &bbox, ColorMode::Pattern, false, false),
            svg_walls(&walls, &bbox, false),
            svg_close(),
        );
        assert_eq!(svg, expected, "overlay output must equal the composed scaffolding");

        // Group layout: primal paints before dual, walls after both.
        let p = svg.find(r#"<g id="primal">"#).unwrap();
        let d = svg.find(r#"<g id="dual">"#).unwrap();
        assert!(p < d, "primal group must come before the dual group");

        // Balanced-tag interior of the first <g id="NAME"> group.
        fn group_slice<'a>(svg: &'a str, marker: &'a str) -> &'a str {
            let start = svg.find(marker).unwrap() + marker.len();
            let mut depth = 1usize;
            let mut pos = start;
            loop {
                let next_open = svg[pos..].find("<g ").map(|i| pos + i);
                let next_close = svg[pos..].find("</g>").map(|i| pos + i);
                match (next_open, next_close) {
                    (Some(o), Some(c)) if c < o => { depth -= 1; pos = c + 4; }
                    (Some(o), _) => { depth += 1; pos = o + 3; }
                    (None, Some(c)) => { depth -= 1; pos = c + 4; }
                    (None, None) => panic!("unbalanced group {marker:?}"),
                }
                if depth == 0 {
                    return &svg[start..pos - 4];
                }
            }
        }

        let walls_at = svg.rfind("<g fill=\"none\" stroke=\"rgb(") .unwrap();
        assert!(d < walls_at, "walls must come after the dual group");

        // Scaffolding prefix is byte-identical to the plain render's.
        let plain = render_svg_with_walls(&primal, &bbox, ColorMode::Pattern, &[]);
        let prefix = svg[..p].to_string();
        let plain_prefix = plain[..plain.find("<path ").unwrap()].to_string();
        assert_eq!(prefix, plain_prefix, "overlay scaffolding must be the plain render's scaffolding");

        // Per-group counts: one fill path and one outline path per tile.
        let p_slice = group_slice(&svg, r#"<g id="primal">"#);
        assert_eq!(p_slice.matches(r#"fill="rgb("#).count(), primal.len());
        assert_eq!(p_slice.matches("<path ").count(), 2 * primal.len());
        let d_slice = group_slice(&svg, r#"<g id="dual">"#);
        assert_eq!(d_slice.matches(r#"fill="rgb("#).count(), dual.len());
        assert_eq!(d_slice.matches("<path ").count(), 2 * dual.len());

        // Walls: exactly one line per wall, emitted once.
        assert_eq!(svg.matches("<line ").count(), walls.len());

        // Ends like the plain render.
        assert!(svg.ends_with("</g></svg>"), "overlay must end with the svg close tags");
    }

    #[test]
    fn mirror_on_degenerate_tiles_stay_degenerate() {
        let mut tiles = mixed_level_tiles();
        tiles.push(Tile { family: crate::tiling::FamilyTag::None, reflected: false,
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

    // ---- vertex figure orbits (C1) ---------------------------------------

    /// A 21-tile 2-uniform-shaped tiling: a 3×3 grid of unit squares, the
    /// four corner squares split into four triangles each by both
    /// diagonals. The complete vertices (wedges summing to 2π) are exactly
    /// the four split-square centers (figure "3.3.3.3": four 90° triangle
    /// wedges) and the four interior grid junctions (figure
    /// "3.3.4.4.4": the split corner square contributes two 45° triangle
    /// wedges, the three unsplit squares one 90° wedge each); every
    /// patch-boundary vertex has angle sum < 2π and drops out.
    fn two_class_tiles() -> Vec<Tile> {
        let mut v: Vec<Tile> = Vec::new();
        for (i, j) in [
            (0i64, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2),
        ] {
            let (x, y) = (i as f64, j as f64);
            let is_corner = (i == 0 || i == 2) && (j == 0 || j == 2);
            if !is_corner {
                v.push(tile(&[(x, y), (x + 1.0, y), (x + 1.0, y + 1.0), (x, y + 1.0)]));
                continue;
            }
            let c = pt(x + 0.5, y + 0.5);
            v.push(tile(&[(x, y), (x + 1.0, y), (c.x, c.y)]));
            v.push(tile(&[(x + 1.0, y), (x + 1.0, y + 1.0), (c.x, c.y)]));
            v.push(tile(&[(x + 1.0, y + 1.0), (x, y + 1.0), (c.x, c.y)]));
            v.push(tile(&[(x, y + 1.0), (x, y), (c.x, c.y)]));
        }
        v
    }

    /// The same 3×3 grid but with only the (0,0) corner split: a 12-tile
    /// tiling carrying three vertex classes ("3.3.3.3" the single center,
    /// "3.3.4.4" the (1,1) junction, "4.4.4.4" the other three junctions),
    /// and an asymmetric dot set (good for the mirror-reflection test).
    fn single_split_tiles() -> Vec<Tile> {
        let mut v: Vec<Tile> = Vec::new();
        for (i, j) in [
            (0i64, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2),
        ] {
            let (x, y) = (i as f64, j as f64);
            if i == 0 && j == 0 {
                let c = pt(x + 0.5, y + 0.5);
                v.push(tile(&[(x, y), (x + 1.0, y), (c.x, c.y)]));
                v.push(tile(&[(x + 1.0, y), (x + 1.0, y + 1.0), (c.x, c.y)]));
                v.push(tile(&[(x + 1.0, y + 1.0), (x, y + 1.0), (c.x, c.y)]));
                v.push(tile(&[(x, y + 1.0), (x, y), (c.x, c.y)]));
                continue;
            }
            v.push(tile(&[(x, y), (x + 1.0, y), (x + 1.0, y + 1.0), (x, y + 1.0)]));
        }
        v
    }

    /// Every `<circle cx cy .../>` in `svg` (the orbit dots carry the only
    /// `<circle>` elements when no symmetry overlay is present), as a
    /// sorted (x, y) point list.
    fn circle_points(svg: &str) -> Vec<(f64, f64)> {
        let mut out = Vec::new();
        for (i, _) in svg.match_indices("<circle ") {
            let rest = &svg[i..];
            let end = rest.find("/>").unwrap();
            let tag = &rest[..end];
            let get = |k: &str| -> f64 {
                let needle = format!("{k}=\"");
                let p = tag.find(&needle).unwrap() + needle.len();
                let s = &tag[p..];
                let e = s.find('"').unwrap();
                s[..e].parse().unwrap()
            };
            out.push((get("cx"), get("cy")));
        }
        out.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap()
                .then_with(|| a.1.partial_cmp(&b.1).unwrap())
        });
        out
    }

    /// The dedup key the orbit derivation uses, restated here for the
    /// census assertions.
    fn vk(p: Vec2) -> (i64, i64) {
        ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
    }

    #[test]
    fn orbits_off_output_is_byte_pinned() {
        // `orbits = None` must be byte-identical to the legacy C5 entry and
        // to the exact legacy golden: the new parameter cannot drift the
        // default export's preamble or body.
        let tiles = mixed_level_tiles();
        let bbox = BBox::new(pt(-1.0, -1.0), pt(3.0, 2.0));
        let legacy =
            render_svg_with_walls_overlay(&tiles, &bbox, ColorMode::Single, &[], false, false, None);
        let got =
            render_svg_with_walls_orbits(&tiles, &bbox, ColorMode::Single, &[], false, false, None, None);
        assert_eq!(got, legacy, "None orbits must be the C5 bytes");
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
        assert_eq!(got, expected, "off-state golden");
        assert!(!got.contains("data-vertex-orbits"), "no orbit markup when off");
    }

    #[test]
    fn orbits_on_structure_and_zorder() {
        // On-state: one orbit group (last in the document), two inner
        // figure groups with the expected dot counts and two distinct
        // accent fills, on == off with the orbit group stripped.
        let tiles = two_class_tiles();
        let ws = walls(&tiles, 45.0);
        assert!(!ws.is_empty(), "need a non-empty wall set");
        let bbox = BBox::new(pt(0.0, 0.0), pt(3.0, 3.0));
        let orbits = vertex_orbits(&tiles);
        assert_eq!(orbits.len(), 2, "the synthetic tiling has two classes");
        let off = render_svg_with_walls_orbits(&tiles, &bbox, ColorMode::Pattern, &ws, false, false, None, None);
        let on = render_svg_with_walls_orbits(&tiles, &bbox, ColorMode::Pattern, &ws, false, false, None, Some(&orbits));

        let marker = r#"<g data-vertex-orbits="true">"#;
        assert_eq!(on.matches(marker).count(), 1, "one orbit group");
        let tail = "</g></svg>";
        assert!(on.ends_with(tail), "{on}");
        let idx = on.find(marker).unwrap();
        assert_eq!(&on[..idx], &off[..off.len() - tail.len()], "on minus orbit group == off");

        // Z-order: the orbit group is the LAST group — after the wall group.
        let spans = outline_group_spans(&on);
        assert_eq!(spans.len(), 2, "outline + wall groups only (orbit excluded): {on}");
        assert!(spans[1].1 <= idx, "orbit group must follow the wall group");

        // Two inner figure groups, the census, and the two accent fills.
        assert_eq!(on.matches(r#"<g data-orbit-figure="#).count(), 2, "two figure groups");
        let (ar, ag, ab) = rgb(ORBIT_A);
        let (br, bg_, bb) = rgb(ORBIT_B);
        assert!(on.contains(&format!("fill=\"rgb({},{},{})\"", ar, ag, ab)), "ORBIT_A present");
        assert!(on.contains(&format!("fill=\"rgb({},{},{})\"", br, bg_, bb)), "ORBIT_B present");
        let a_idx = on.find(&format!(r#"<g data-orbit-figure="{}" fill="rgb({},{},{})">"#, orbits[0].figure, ar, ag, ab)).unwrap();
        let b_idx = on.find(&format!(r#"<g data-orbit-figure="{}" fill="rgb({},{},{})">"#, orbits[1].figure, br, bg_, bb)).unwrap();
        // Inner figure groups contain only self-closing circles, so the
        // first `</g>` after each opener is its own close.
        let a_block_end = on[a_idx..].find("</g>").unwrap() + a_idx;
        let a_block = &on[a_idx..a_block_end];
        let b_block_end = on[b_idx..].find("</g>").unwrap() + b_idx;
        let b_block = &on[b_idx..b_block_end];
        assert_eq!(a_block.matches("<circle").count(), orbits[0].points.len(), "orbit 0 dot count");
        assert_eq!(b_block.matches("<circle").count(), orbits[1].points.len(), "orbit 1 dot count");
    }

    #[test]
    fn vertex_orbits_census_two_classes() {
        // The 21-tile synthetic: exactly two classes with the declared
        // figure keys, the right point counts, the exact point sets, and
        // deterministic (x, y) order.
        let tiles = two_class_tiles();
        assert_eq!(tiles.len(), 21, "five whole squares + four split corners (16 triangles)");
        let orbits = vertex_orbits(&tiles);
        assert_eq!(orbits.len(), 2, "exactly two vertex classes");
        assert_eq!(orbits[0].figure, "3.3.3.3", "lex-first class is the triangle center");
        assert_eq!(orbits[1].figure, "3.3.4.4.4", "the grid junctions");
        assert_eq!(orbits[0].points.len(), 4, "four split-square centers");
        assert_eq!(orbits[1].points.len(), 4, "four interior junctions");

        let centers: std::collections::BTreeSet<(i64, i64)> = orbits[0].points.iter().map(|p| vk(*p)).collect();
        assert_eq!(
            centers,
            std::collections::BTreeSet::from([
                (500000, 500000), (2500000, 500000), (500000, 2500000), (2500000, 2500000),
            ]),
            "the 3.3.3.3 points are exactly the four split-square centers"
        );
        let junctions: std::collections::BTreeSet<(i64, i64)> = orbits[1].points.iter().map(|p| vk(*p)).collect();
        assert_eq!(
            junctions,
            std::collections::BTreeSet::from([
                (1000000, 1000000), (2000000, 1000000), (1000000, 2000000), (2000000, 2000000),
            ]),
            "the 3.3.4.4.4 points are exactly the four interior junctions"
        );

        // Deterministic emission order: strictly increasing (x, y).
        for o in &orbits {
            for pair in o.points.windows(2) {
                let (a, b) = (&pair[0], &pair[1]);
                assert!(a.x < b.x || (a.x == b.x && a.y < b.y), "orbit not (x, y) sorted: {a:?} then {b:?}");
            }
        }

        // Boundary vertices are excluded: total dots = 8, not the ~25 raw
        // grid/center vertex positions.
        assert_eq!(orbits.iter().map(|o| o.points.len()).sum::<usize>(), 8);
    }

    #[test]
    fn vertex_orbits_mirrored_composition() {
        // Mirror composes with the dots: the on-state dot set is the
        // exact reflection of the off-state dot set about the frame
        // center (x → 2·cx − x, y unchanged), a bijection at 4-decimal
        // precision. Uses the asymmetric single-split tiling.
        let tiles = single_split_tiles();
        let orbits = vertex_orbits(&tiles);
        assert_eq!(orbits.len(), 3, "the asymmetric tiling has three classes");
        let bbox = BBox::new(pt(0.0, 0.0), pt(3.0, 3.0));
        let cx = bbox.center().x;
        let off = render_svg_with_walls_orbits(&tiles, &bbox, ColorMode::Single, &[], false, false, None, Some(&orbits));
        let on = render_svg_with_walls_orbits(&tiles, &bbox, ColorMode::Single, &[], false, true, None, Some(&orbits));

        // The reflection x → 2·cx − x reverses the x-order, so the
        // (x, y)-sorted off list, re-sorted by (−x, y), must equal the
        // (x, y)-sorted on list. Both lists parse the same 4-decimal
        // emissions, so a 5e-4 margin is exact for these values.
        let mut off_pts = circle_points(&off);
        let on_pts = circle_points(&on);
        assert_eq!(off_pts.len(), on_pts.len(), "mirror preserves the dot count");
        assert!(off_pts.len() >= 3, "at least one dot per class");
        off_pts.sort_by(|a, b| {
            b.0.partial_cmp(&a.0)
                .unwrap()
                .then_with(|| a.1.partial_cmp(&b.1).unwrap())
        });
        for (a, b) in off_pts.iter().zip(&on_pts) {
            assert!(
                (2.0 * cx - a.0 - b.0).abs() < 5e-4,
                "dot x {} must reflect to {}",
                a.0, b.0
            );
            assert!((a.1 - b.1).abs() < 5e-4, "dot y is unchanged under mirror");
        }
    }

    #[test]
    fn vertex_orbits_empty_noop() {
        // Some(&[]) is the same byte no-op as None: an empty orbit list
        // emits no group at all.
        let tiles = two_class_tiles();
        let bbox = BBox::new(pt(0.0, 0.0), pt(3.0, 3.0));
        let none = render_svg_with_walls_orbits(&tiles, &bbox, ColorMode::Single, &[], false, false, None, None);
        let empty = render_svg_with_walls_orbits(&tiles, &bbox, ColorMode::Single, &[], false, false, None, Some(&[]));
        assert_eq!(none, empty, "Some(&[]) must be byte-identical to None");
        assert!(!none.contains("data-vertex-orbits"), "no orbit group when empty");
    }

    // ---- Landing B: aperiodic toggles ---------------------------------

    /// A tile with the emitted hat/spectre/turtle signature at `x`:
    /// `proto` as asked, `seed` = matching-rule label + 1.
    fn fam_tile(family: crate::tiling::FamilyTag, proto: u8, seed: u32, x: f64) -> Tile {
        Tile {
            family,
            reflected: false,
            proto,
            seed,
            level: 0,
            poly: Polygon::from_slice(&[pt(x, 0.0), pt(x + 1.0, 0.0), pt(x + 1.0, 1.0), pt(x, 1.0)]),
        }
    }

    /// `fill="rgb(...)"` values in document order (pass-1 fills only: the
    /// groups and wall lines carry `fill="none"`, pass-2 paths carry none).
    /// The background rect's fill is skipped: it is scaffold, not a tile.
    fn fills(svg: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut rest = &svg[svg.find("/>").unwrap() + 2..];
        while let Some(i) = rest.find(r#"fill="rgb("#) {
            let r = &rest[i + r#"fill="rgb("#.len()..];
            let end = r.find(')').unwrap();
            out.push(format!("rgb({})", &r[..end]));
            rest = &r[end + 1..];
        }
        out
    }

    /// Every byte except the pass-1 fill values (geometry + groups + tail).
    fn without_fills(svg: &str) -> String {
        let mut out = String::with_capacity(svg.len());
        let mut rest = svg;
        while let Some(i) = rest.find(r#"fill="rgb("#) {
            let r = &rest[i + r#"fill="rgb("#.len()..];
            let end = r.find(')').unwrap();
            out.push_str(&rest[..i]);
            rest = &r[end + 1..];
        }
        out.push_str(rest);
        out
    }

    /// The contents of the large-tile-outline group (empty `String` when the
    /// group is absent).
    fn outline_group(svg: &str) -> String {
        let Some(start) = svg.find(r#"<g data-large-tile-outline="true""#) else {
            return String::new();
        };
        let tail = &svg[start..];
        let end = tail.find("</g>").unwrap() + "</g>".len();
        tail[..end].to_string()
    }

    /// The large-tile-outline group removed: every other byte kept.
    fn without_outline_group(svg: &str) -> String {
        let Some(start) = svg.find(r#"<g data-large-tile-outline="true""#) else {
            return svg.to_string();
        };
        let tail = &svg[start..];
        let end = tail.find("</g>").unwrap() + "</g>".len();
        let mut out = String::with_capacity(svg.len());
        out.push_str(&svg[..start]);
        out.push_str(&tail[end..]);
        out
    }

    #[test]
    fn aperiodic_toggles_off_is_byte_identical() {
        // The off-state contract for BOTH new entries and BOTH flags: the
        // default struct is byte-for-byte the pre-Landing-B render. Includes
        // the mirror path and a supertile view with the M205 split on.
        let flat = square_grid(2);
        let flat_box = BBox::new(pt(-0.5, -0.5), pt(2.5, 2.5));
        let mixed = mixed_level_tiles();
        let mixed_box = BBox::new(pt(-1.0, -1.0), pt(3.0, 2.0));
        for (tiles, bbox) in [(&flat, &flat_box), (&mixed, &mixed_box)] {
            for mirror in [false, true] {
                for hl in [false, true] {
                    let base =
                        render_svg_with_walls_orbits(tiles, bbox, ColorMode::Pattern, &[], hl, mirror, None, None);
                    let off = render_svg_with_walls_orbits_aperiodic(
                        tiles,
                        bbox,
                        ColorMode::Pattern,
                        &[],
                        hl,
                        mirror,
                        None,
                        None,
                        AperiodicToggles::default(),
                    );
                    assert_eq!(base, off, "default toggles must be byte-identical (mirror {mirror}, m205 {hl})");
                }
            }
        }
        // The dual entry too.
        let dual = square_grid(1);
        let off = render_svg_dual_overlay(&flat, &dual, &flat_box, ColorMode::Pattern, &[], false, false);
        let on = render_svg_dual_overlay_aperiodic(
            &flat,
            &dual,
            &flat_box,
            ColorMode::Pattern,
            &[],
            false,
            false,
            AperiodicToggles::default(),
        );
        assert_eq!(off, on, "default toggles must not move the dual entry");
    }

    #[test]
    fn aperiodic_toggles_on_are_inert_without_the_facts() {
        // Both flags ON for an untagged, unreflected view is still byte
        // identity: no family matches the outline pass, no tile is reflected.
        let tiles = square_grid(2);
        let bbox = BBox::new(pt(-0.5, -0.5), pt(2.5, 2.5));
        let base = render_svg_with_walls_orbits(&tiles, &bbox, ColorMode::Pattern, &[], false, false, None, None);
        let on = render_svg_with_walls_orbits_aperiodic(
            &tiles,
            &bbox,
            ColorMode::Pattern,
            &[],
            false,
            false,
            None,
            None,
            AperiodicToggles { highlight_reflected: true, large_tile_outline: true },
        );
        assert_eq!(base, on);
        // The dual entry: centroid cells are untagged/unreflected too.
        let off = render_svg_dual_overlay(&tiles, &tiles, &bbox, ColorMode::Pattern, &[], false, false);
        let on = render_svg_dual_overlay_aperiodic(
            &tiles,
            &tiles,
            &bbox,
            ColorMode::Pattern,
            &[],
            false,
            false,
            AperiodicToggles { highlight_reflected: true, large_tile_outline: true },
        );
        assert_eq!(off, on, "the toggles must be inert on an untagged dual");
    }

    #[test]
    fn highlight_reflected_lightens_only_reflected_fills() {
        use crate::paint::{reflected_accent, rgb, tile_rgb};
        let mode = ColorMode::Pattern;
        let mut a = fam_tile(crate::tiling::FamilyTag::Hat, 0, 1, 0.0);
        a.reflected = true; // the H1 cluster hat
        let b = fam_tile(crate::tiling::FamilyTag::Hat, 0, 2, 1.0);
        let tiles = vec![a.clone(), b.clone()];
        let bbox = BBox::new(pt(-0.5, -0.5), pt(2.5, 1.5));
        let off = render_svg(&tiles, &bbox, mode);
        let on = render_svg_with_walls_orbits_aperiodic(
            &tiles,
            &bbox,
            mode,
            &[],
            false,
            false,
            None,
            None,
            AperiodicToggles { highlight_reflected: true, large_tile_outline: false },
        );
        let want_base = rgb(tile_rgb(mode, &b));
        let want_acc = rgb(reflected_accent(tile_rgb(mode, &a)));
        assert_eq!(
            fills(&on),
            vec![
                format!("rgb({},{},{})", want_acc.0, want_acc.1, want_acc.2),
                format!("rgb({},{},{})", want_base.0, want_base.1, want_base.2),
            ],
            "the reflected tile reads the accent, its neighbour its base color"
        );
        // Nothing else moves: same geometry, same outline groups, same tail.
        assert_eq!(without_fills(&off), without_fills(&on));
        assert_ne!(fills(&off), fills(&on), "the highlight must be visible");
    }

    #[test]
    fn highlight_reflected_follows_the_mirror_involution() {
        use crate::paint::{reflected_accent, rgb, tile_rgb};
        let mode = ColorMode::Pattern;
        let mut a = fam_tile(crate::tiling::FamilyTag::Hat, 0, 1, 0.0);
        a.reflected = true;
        let b = fam_tile(crate::tiling::FamilyTag::Hat, 0, 2, 1.0);
        let tiles = vec![a.clone(), b.clone()];
        let bbox = BBox::new(pt(0.0, 0.0), pt(2.0, 1.0));
        let c = |t: &Tile| {
            let (r, g, b) = rgb(tile_rgb(mode, t));
            format!("rgb({r},{g},{b})")
        };
        let acc = |t: &Tile| {
            let (r, g, b) = rgb(reflected_accent(tile_rgb(mode, t)));
            format!("rgb({r},{g},{b})")
        };
        let hl = AperiodicToggles { highlight_reflected: true, large_tile_outline: false };
        let plain = render_svg_with_walls_orbits_aperiodic(&tiles, &bbox, mode, &[], false, false, None, None, hl);
        let mirrored = render_svg_with_walls_orbits_aperiodic(&tiles, &bbox, mode, &[], false, true, None, None, hl);
        // Tile order is preserved by the mirror; only WHICH tile is
        // reflected flips. So the plain export accents a (canonical b),
        // the mirrored one accents b (now a canonical copy of a).
        assert_eq!(fills(&plain), vec![acc(&a), c(&b)], "plain: the reflected hat is accented");
        assert_eq!(fills(&mirrored), vec![c(&a), acc(&b)], "mirrored: exactly the other tile is accented");
        // The complement is exact: the highlighted SET flips, and every
        // accent is the lightened form of that same tile's own color.
        assert_ne!(fills(&plain)[0], fills(&mirrored)[0]);
    }

    #[test]
    fn large_tile_outline_group_is_family_keyed() {
        // Hat/Spectre: every tile (monohedral cards). Turtle: the turtle
        // cells only — the companion hat (`proto == 1`) is excluded.
        // Untagged and Isohedral (no large-tile notion): no group at all.
        use crate::tiling::FamilyTag;
        let bbox = BBox::new(pt(-0.5, -0.5), pt(2.5, 1.5));
        for (family, want) in [
            (FamilyTag::Hat, 3),
            (FamilyTag::Spectre, 3),
            (FamilyTag::Turtle, 2),
            (FamilyTag::None, 0),
            (FamilyTag::Isohedral, 0),
            (FamilyTag::Penrose, 0),
            (FamilyTag::Uniform, 0),
            (FamilyTag::TwoUniform, 0),
            (FamilyTag::Andritz, 0),
        ] {
            let tiles = vec![
                fam_tile(family, 0, 1, 0.0),
                fam_tile(family, 0, 5, 1.0),
                fam_tile(family, 1, 3, 2.0),
            ];
            let on = render_svg_with_walls_orbits_aperiodic(
                &tiles,
                &bbox,
                ColorMode::Metatile,
                &[],
                false,
                false,
                None,
                None,
                AperiodicToggles { highlight_reflected: false, large_tile_outline: true },
            );
            let off = render_svg(&tiles, &bbox, ColorMode::Metatile);
            let group = outline_group(&on);
            if want == 0 {
                assert!(group.is_empty(), "{family:?}: must emit no outline group");
                assert_eq!(on, off, "{family:?}: flag must be a byte no-op");
            } else {
                assert_eq!(
                    group.matches("<path ").count(),
                    want,
                    "{family:?}: marked tiles"
                );
                // Only the appended group differs: fills and the grout
                // group(s) are the off-state bytes.
                let pre = off.strip_suffix("</g></svg>").unwrap();
                assert!(on.starts_with(pre), "{family:?}: the body must be untouched");
            }
        }
    }

    #[test]
    fn large_tile_outline_group_bytes_are_pinned() {
        // Exact opening tag: gold-orange accent at 2× the base stroke.
        // bbox min side 1.0 → base stroke 0.004 → accent stroke 0.008.
        let tiles = vec![fam_tile(crate::tiling::FamilyTag::Hat, 0, 1, 0.0)];
        let bbox = BBox::new(pt(0.0, 0.0), pt(3.0, 1.0));
        let on = render_svg_with_walls_orbits_aperiodic(
            &tiles,
            &bbox,
            ColorMode::Single,
            &[],
            false,
            false,
            None,
            None,
            AperiodicToggles { highlight_reflected: false, large_tile_outline: true },
        );
        assert!(
            on.contains(
                r#"<g data-large-tile-outline="true" fill="none" stroke="rgb(255,122,26)" stroke-width="0.0080" stroke-linejoin="round">"#
            ),
            "{on}"
        );
        assert!(on.ends_with("</g></g></svg>"), "{on}");
    }

    #[test]
    fn large_tile_outline_appends_after_the_m205_split_and_before_the_overlays() {
        // M205 interaction: with `highlight_supertiles` on, the level-split
        // groups keep their exact bytes and order; the accent group is
        // APPENDED after them (never a re-split of pass 2).
        let tiles = mixed_level_tiles();
        let bbox = BBox::new(pt(-1.0, -1.0), pt(3.0, 2.0));
        let off = render_svg_with_walls_orbits(&tiles, &bbox, ColorMode::Pattern, &[], true, false, None, None);
        let on = render_svg_with_walls_orbits_aperiodic(
            &tiles,
            &bbox,
            ColorMode::Pattern,
            &[],
            true,
            false,
            None,
            None,
            AperiodicToggles { highlight_reflected: false, large_tile_outline: true },
        );
        // Family None here → no marked tile, so a byte no-op even with the
        // M205 split on (the strongest form of the interaction pin).
        assert_eq!(off, on, "an untagged mixed-level view must not change at all");

        // With a family present: the on-state is exactly the off-state with
        // one group inserted — nothing else moves. Without walls the
        // insertion point is the very end of the body (after the M205 split);
        // with walls it is between the grout groups and the wall pass.
        let hat: Vec<Tile> = tiles
            .iter()
            .map(|t| Tile { family: crate::tiling::FamilyTag::Hat, ..t.clone() })
            .collect();
        let on_t = AperiodicToggles { highlight_reflected: false, large_tile_outline: true };
        let off_nowalls =
            render_svg_with_walls_orbits(&hat, &bbox, ColorMode::Pattern, &[], true, false, None, None);
        let on_nowalls = render_svg_with_walls_orbits_aperiodic(
            &hat,
            &bbox,
            ColorMode::Pattern,
            &[],
            true,
            false,
            None,
            None,
            on_t,
        );
        assert_eq!(without_outline_group(&on_nowalls), off_nowalls);
        let pre = off_nowalls.strip_suffix("</g></svg>").unwrap();
        assert!(on_nowalls.starts_with(pre), "the M205 level-split bytes are untouched");
        let tail = &on_nowalls[pre.len()..];
        assert!(tail.starts_with(r#"<g data-large-tile-outline="true"#), "{tail}");
        assert_eq!(tail.matches("<path ").count(), hat.len(), "every hat is outlined");
        assert!(tail.ends_with("</g></g></svg>"), "{tail}");

        // With walls: the accent group is inside the body, i.e. painted
        // BEFORE the wall ribbons; the wall bytes themselves are unchanged.
        let ws = walls(&hat, 45.0);
        let off = render_svg_with_walls_orbits(&hat, &bbox, ColorMode::Pattern, &ws, true, false, None, None);
        let on =
            render_svg_with_walls_orbits_aperiodic(&hat, &bbox, ColorMode::Pattern, &ws, true, false, None, None, on_t);
        assert_eq!(without_outline_group(&on), off);
        let idx_outline = on.find(r#"<g data-large-tile-outline="true""#).unwrap();
        let idx_walls = on.find(r#"stroke-linecap="round""#).unwrap();
        assert!(idx_outline < idx_walls, "the wall pass paints over the accent");

        // With the symmetry overlay: still before the overlay group (the
        // certified annotations stay on top).
        let ov = synth_overlay();
        let on_ov = render_svg_with_walls_orbits_aperiodic(
            &hat,
            &bbox,
            ColorMode::Pattern,
            &ws,
            true,
            false,
            Some(&ov),
            None,
            on_t,
        );
        let idx_overlay = on_ov.find(r#"<g data-symmetry-overlay="true""#).unwrap();
        let idx_outline_ov = on_ov.find(r#"<g data-large-tile-outline="true""#).unwrap();
        assert!(idx_outline_ov < idx_overlay, "the annotation overlays stay on top");
    }
}
