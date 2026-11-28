//! GUI application (egui) — a single-pane, two-mode browser mirroring
//! the TUI (UI-3; the owner prefers the TUI layout):
//!
//! * **Browse** — the whole window is the card grid: a tab band (the
//!   ten category tabs + a search field, one wrapping row) over
//!   section headers (crystal systems on the Wallpaper tab, rank bands
//!   on the Laves tab) and cards with lazily generated previews
//!   (shared `tess_core::raster`). The Wallpaper tab opens
//!   with a catalog note (since M85 all 17 groups are represented, the
//!   note reports the completed catalog).
//!   Clicking a card opens Render mode.
//! * **Render** — the whole window is the canvas (drag-pan,
//!   wheel-zoom) with a top strip (`← back`, `◀`/`▶` prev/next in the
//!   tab order, a params toggle, a zoom cluster `− [nnn%] +` whose
//!   percentage label doubles as zoom-reset, reset) and a collapsible
//!   bottom panel holding the params, color mode, and symmetry
//!   readout. The `+`/`−` keys zoom around the canvas center (wheel
//!   anchors at the cursor). Escape or `← back` returns to Browse
//!   exactly as left (same tab + selection).
//! * **Physical zoom controls** — the wheel/pinch gestures don't
//!   arrive on every desktop (virtual/X11 sessions can strip them), so
//!   both modes carry clickable `−`/`+` buttons: the Render strip
//!   zooms the canvas, the Browse tab row steps the card grid by ONE
//!   column per press (the grid fills the window, so raw width steps
//!   are invisible until they cross a column threshold — column steps
//!   always reflow visibly). Each cluster's middle label resets to the
//!   default; the Browse keys (+/−) step columns the same way.
//!
//! No fixed-width side panel in either mode: grid and canvas each own
//! the full window, so the layout mirrors the TUI by construction.

use std::collections::HashMap;

use tess_core::paint;
use tess_core::params::{ParamKind, ParamSet, ParamSpec, ParamValue};
use tess_core::view::ColorMode;

/// Card preview image size in pixels (the TUI thumbnail is 20×16
/// half-block cells = 40×32 px; the GUI uses a larger, crisper card).
const CARD_IMG_W: u32 = 120;
const CARD_IMG_H: u32 = 90;
/// Target card width for the responsive browse grid: the column count
/// is the number of these that fit in the window width — fully
/// dynamic, no upper bound ([`columns_for_width`]), with this width
/// as the squash floor that keeps the preview and title text
/// comfortable; the cards then stretch to fill their columns,
/// landing around 220–260 px wide on typical windows.
const CARD_TARGET_W: f32 = 240.0;
/// Horizontal grid spacing between card columns, in pixels.
const GRID_SPX: f32 = 10.0;
/// Character cap applied to descriptions before they reach the
/// card's label: a backstop that keeps the text-layout job short
/// (it matters on very wide single-column windows). Width overflow
/// at normal column widths is handled by the label's own
/// `truncate()` (ellipsis at the card's width), which kicks in
/// first.
const DESC_CHARS: usize = 40;
/// Character budget per card-title line: a name longer than this
/// wraps to the card's second title line (still ellipsized at the
/// card's width by the label's own `truncate()`).
const NAME_CHARS_PER_LINE: usize = 24;
/// Card inner margin (inset on all four sides), in pixels. The card's
/// content child is bounded by [`card_inner_w`] × [`card_inner_h`],
/// inset by this margin from the card's allocated rect — which IS the
/// grid cell, exactly the column width, so the grid fills the window
/// width without overflowing.
const CARD_INNER_MARGIN: f32 = 5.0;
/// Vertical spacing between card rows (image → name line 1 → name
/// line 2 → description → symmetry line), in pixels.
const CARD_ROW_SPX: f32 = 3.0;
/// Fixed heights of the card's text rows, in pixels: name (two lines,
/// 16 px strong each), description (13 px), symmetry line (12 px).
/// Each is the galley row height of egui's default proportional font
/// at that size, plus headroom, so the text content always fits the
/// card height ([`card_inner_h`]) (pinned by a unit test against the
/// real galley row heights).
const CARD_NAME_LINE_H: f32 = 18.5;
const CARD_DESC_LINE_H: f32 = 15.0;
const CARD_SYM_LINE_H: f32 = 14.0;
/// Inner content height of a card at an image height of `img_h`:
/// image + the two-line title + description + symmetry line + the row
/// spacings between them. Every card is at least this tall, so all
/// card boxes are the same size and the grid is orthogonal — every
/// card in every row aligns (pinned by a unit test). The default
/// size's value is the historical `CARD_INNER_H` constant (168 px).
fn card_inner_h(img_h: u32) -> f32 {
    img_h as f32
        + 2.0 * CARD_NAME_LINE_H
        + CARD_DESC_LINE_H
        + CARD_SYM_LINE_H
        + 4.0 * CARD_ROW_SPX
}
/// Total card height at an image height of `img_h` — the inner
/// content plus the two vertical margins. `draw_card` allocates
/// exactly `col_w` × [`card_total_h`] per cell, so every card is
/// identical and the grid's cell sizes never depend on how the
/// content lays out inside (the LTR-inheritance bug grew cells
/// sideways).
fn card_total_h(img_h: u32) -> f32 {
    card_inner_h(img_h) + 2.0 * CARD_INNER_MARGIN
}
/// Card-size zoom bounds: the smallest and largest card target
/// widths reachable through ctrl+wheel / pinch / the +/- keys. The
/// default ([`CARD_TARGET_W`]) sits comfortably inside the window.
const CARD_W_MIN: f32 = 150.0;
const CARD_W_MAX: f32 = 480.0;
/// Multiplicative zoom factor per physical +/− step on the Render
/// canvas (the TUI's convention), for both the top-strip buttons and
/// the keyboard keys.
const RENDER_ZOOM_STEP: f64 = 1.3;
/// The card's preview-image pixel size at a card target width of
/// `target_w`: the default [`CARD_TARGET_W`] renders the historical
/// [`CARD_IMG_W`]×[`CARD_IMG_H`] preview; the size scales linearly
/// with the zoom (keeping the 4:3 preview aspect), so bigger cards
/// get bigger previews. Quantized to whole pixels (the texture cache
/// is keyed by the exact pixel size).
fn card_img_size(target_w: f32) -> (u32, u32) {
    let w = ((CARD_IMG_W as f32 * (target_w / CARD_TARGET_W)).round() as u32).max(32);
    (w, w * CARD_IMG_H / CARD_IMG_W)
}

// Card frame colors (dark theme): neutral outline, hover outline, and
// the selected card's fill + cyan stroke (the TUI's cyan-border
// highlight); section headers in bold gray.
const CARD_STROKE: egui::Color32 = egui::Color32::from_rgb(72, 76, 94);
const HOVER_STROKE: egui::Color32 = egui::Color32::from_rgb(122, 130, 154);
const SELECTED_STROKE: egui::Color32 = egui::Color32::from_rgb(94, 186, 255);
const SELECTED_FILL: egui::Color32 = egui::Color32::from_rgb(38, 54, 76);
const SECTION_COLOR: egui::Color32 = egui::Color32::from_rgb(148, 154, 176);
const SYM_COLOR: egui::Color32 = egui::Color32::from_rgb(128, 134, 156);
const DESC_COLOR: egui::Color32 = egui::Color32::from_rgb(110, 116, 138);

/// The two modes of the single-pane browser (mirrors the TUI):
/// Browse is the full-window card grid; Render is the full-window
/// canvas of the selected tiling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GuiMode {
    Browse,
    Render,
}

pub struct GuiApp {
    pub app: tess_core::App,
    /// Current mode: Browse (full-window grid) or Render (full-window
    /// canvas of the selected tiling).
    mode: GuiMode,
    /// Render mode: the collapsible params/color-mode/symmetry panel
    /// is shown. Expanded by default on every entry to Render (the
    /// user can collapse it; the next entry re-expands it).
    params_open: bool,
    /// Cached render-panel metadata: (tiling id, params fingerprint,
    /// symmetry, periodic basis). Regenerated only when the tiling or its
    /// parameter values change — never per frame — so panel rendering
    /// cannot trigger view-sized tile generation (e.g. Hirschhorn growth).
    meta: Option<Meta>,
    /// The cached canvas result: everything `draw_canvas` paints is
    /// derived from this, so the tiling is only regenerated when the key
    /// changes (tiling, params, viewport, canvas size). Color mode is
    /// read live at paint time and never enters the key.
    render_cache: Option<RenderCache>,
    /// Diagnostics: the number of canvas regenerations (one per cache
    /// miss). Counted so tests can pin the cache behavior — production
    /// only ever increments it.
    render_cache_misses: usize,
    /// The shared browser model (`tess_core::browse`), built once at
    /// startup: one bounded ±10-world-unit probe per tiling, the same
    /// cost the TUI pays for its tab bar.
    tabs: Vec<tess_core::Tab>,
    /// Active tab: position in `TilingCategory::ALL` order.
    active_tab: usize,
    /// Per-tab selection memory: the last-selected tiling id per tab,
    /// so switching back to a tab restores that tab's card (and,
    /// WYSIWYG, the canvas shows it immediately).
    tab_selected: Vec<Option<String>>,
    /// The search text the active tab was last auto-jumped for. A
    /// different text re-jumps the active tab to the first tab with a
    /// match (the TUI's search behavior); clearing does not jump.
    search_at_jump: String,
    /// (active tab, selected id) at the last scroll-to-visible, so the
    /// scroll happens only when the selection actually changes
    /// (the TUI's `last_highlighted` guard; without it, manual
    /// scrolling would be snapped back on every render).
    last_highlighted: (usize, String),
    /// Lazily built card preview textures, keyed by (id, params-string,
    /// effective preview mode, size). Textures are generated in the draw
    /// pass for visible cards only — generating all 63 up front would
    /// stall the first frame.
    card_textures: HashMap<String, egui::TextureHandle>,
    /// Current card target width for the browse grid (1.0× =
    /// [`CARD_TARGET_W`]). Ctrl+wheel / trackpad pinch (egui's
    /// `zoom_delta` — egui diverts ctrl-scroll away from scrolling, so
    /// the grid never jumps) and the +/- keys scale it, clamped to
    /// [`CARD_W_MIN`, `CARD_W_MAX`]. The preview image scales with it
    /// ([`card_img_size`]); changing it prunes stale texture sizes
    /// ([`GuiApp::prune_card_textures`]).
    card_target_w: f32,
    /// The browser's search text (name/id/description, all tabs).
    /// Kept in GuiApp rather than `app.search` because the TUI and the
    /// tess-core App.search filter on different fields.
    search: String,
    /// Diagnostics for the headless geometry test: the card rects
    /// allocated by the last `draw_card_grid` pass (cleared at its top,
    /// pushed per card), plus the column width `col_w` that pass used.
    /// Production code never reads these — the `allow(dead_code)` keeps
    /// the 0-warning bar outside `cfg(test)` (the UI-1 precedent).
    #[cfg_attr(not(test), allow(dead_code))]
    card_rects: Vec<egui::Rect>,
    #[cfg_attr(not(test), allow(dead_code))]
    card_grid_col_w: f32,
    /// The column count and available width recorded by the last
    /// `draw_card_grid` pass — the count feeds the tab-band card-size
    /// label, the width is the baseline the `−`/`+` card-size controls
    /// step from (one column per press, always a visible reflow).
    /// Zero avail = no grid drawn yet (the controls no-op). Both are
    /// read live from the recorded pass, so a window resize between
    /// passes makes the first post-resize step land on the old
    /// geometry once; the next press self-corrects.
    card_grid_cols: usize,
    card_grid_avail: f32,
    /// The canvas rect of the last Render pass — the zoom buttons'
    /// anchor (its center), so button zoom keeps the view centered the
    /// way keyboard zoom does. `None` before the first canvas pass
    /// (zoom then anchors at the viewport center).
    canvas_rect: Option<egui::Rect>,
    /// The viewport zoom the app was constructed with — the 100%
    /// baseline for the zoom label and its click-to-reset.
    zoom_default: f64,
    /// The About panel is open: app name + version, the data
    /// attributions rendered content draws on, and the pointer at the
    /// per-tiling source docs. Toggled by the H key and the title-bar
    /// About button; closed by H again or the panel's × button.
    /// Purely an overlay — it enters no RenderKey/cache key.
    about_open: bool,
    /// The Browse preview color mode override (the preview band's mode
    /// selector): `None` = each card previews in its tiling's own
    /// default preview mode (the historical behavior); `Some(m)` =
    /// every preview renders in `m`. One shared state with the Render
    /// detail selector — both selectors write through
    /// [`GuiApp::set_preview_mode`], which also drives `app.color_mode`
    /// (the canvas's live mode), so Browse→Render and Render→Browse
    /// stay in sync. `select_id` resets `app.color_mode` to the
    /// selected tiling's default (the tess-core App behavior the TUI
    /// shares); while an override is set, the GUI re-applies it at
    /// every selection so an explicit choice sticks across card clicks
    /// until the band's `default` clears it.
    preview_mode: Option<ColorMode>,
    /// Diagnostics for the headless selector test: each preview-band
    /// mode label's `(mode, rect)` recorded by the last
    /// `draw_preview_mode_row` pass (`None` = the `default` label).
    /// Production code never reads these — the `allow(dead_code)`
    /// keeps the 0-warning bar outside `cfg(test)` (the card_rects
    /// precedent).
    #[cfg_attr(not(test), allow(dead_code))]
    preview_mode_rects: Vec<(Option<ColorMode>, egui::Rect)>,
}

/// What determines the canvas `TilingResult`: the tiling id, the
/// parameter fingerprint (every current parameter value, `|id=value`),
/// the viewport (center + zoom) and the canvas size. Two frames with
/// equal keys draw identical geometry, so `draw_canvas` only regenerates
/// on a real change. Color mode is deliberately excluded: painting reads
/// it live every frame and it never affects the generated geometry.
#[derive(Clone, Debug, PartialEq)]
struct RenderKey {
    id: String,
    params: String,
    center_x: f64,
    center_y: f64,
    zoom: f64,
    w: u32,
    h: u32,
}

/// The cached canvas result. Stored so the canvas repaints the same
/// geometry instead of regenerating on every frame; `render_cache_misses`
/// counts the regenerations.
struct RenderCache {
    key: RenderKey,
    result: tess_core::TilingResult,
}

/// Region-independent metadata shown in the side panel.
struct Meta {
    key: String,
    symmetry: tess_core::SymmetryMeta,
    periodic: Option<[tess_core::Vec2; 2]>,
}

/// Layout fingerprint of a polygon's vertex list: equal exactly when two
/// vertex lists are direct-similar images of each other with the index
/// correspondence i↔i — same polygon up to translation, uniform scale,
/// and rotation, starting at the same structural vertex and traversed
/// the same way. This is the condition under which one triangulation's
/// index triples are valid for the other polygon, so it is what the
/// GUI's per-prototile triangulation cache must key on.
///
/// Computed from the complex ratios `(v_i − v_0) / (v_1 − v_0)` for
/// i ≥ 2 (invariant under translation/rotation/scale; quantized to
/// 1e-6 and FNV-1a-hashed). Cyclic shifts start the ratios at a
/// different vertex and mirrors conjugate them (imaginary part flips),
/// so both get distinct fingerprints — their index triples must not be
/// shared.
fn layout_fingerprint(pts: &[tess_core::Vec2]) -> u64 {
    let quant = |v: f64| -> i64 {
        if v.is_finite() {
            (v * 1.0e6).round() as i64
        } else {
            i64::MAX
        }
    };
    let mut h = 0xcbf2_9ce4_8422_2325_u64;
    let fold = |h: &mut u64, q: i64| {
        for b in q.to_le_bytes() {
            *h = (*h ^ b as u64).wrapping_mul(0x100_0000_01b3);
        }
    };
    fold(&mut h, pts.len() as i64);
    if pts.len() < 3 {
        return h;
    }
    let (ax, ay) = (pts[1].x - pts[0].x, pts[1].y - pts[0].y);
    let den = ax * ax + ay * ay;
    if den <= 0.0 || den.is_nan() {
        // Degenerate first edge (the generators never emit one): fall
        // back to raw quantized coordinates — still deterministic.
        for p in pts {
            fold(&mut h, quant(p.x));
            fold(&mut h, quant(p.y));
        }
        return h;
    }
    for p in &pts[2..] {
        let (bx, by) = (p.x - pts[0].x, p.y - pts[0].y);
        // (b + i·b') / (a + i·a'): real and imaginary parts of the
        // complex quotient — a pure rotation cancels, so rotated copies
        // of one layout hash equal while cyclic shifts do not.
        fold(&mut h, quant((bx * ax + by * ay) / den));
        fold(&mut h, quant((by * ax - bx * ay) / den));
    }
    h
}

// ---------------------------------------------------------------------------
// Pure browser logic (the egui-free core of the tab/card browser; the TUI
// has its own copies of the equivalent decisions). These are unit-tested
// below without constructing any egui types.
// ---------------------------------------------------------------------------

/// Case-insensitive card search over name, id, and description — the
/// same three fields the TUI search filters on. An empty (whitespace)
/// query matches everything.
fn search_matches(card: &tess_core::CardMeta, query: &str) -> bool {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return true;
    }
    card.name.to_lowercase().contains(&q)
        || card.id.to_lowercase().contains(&q)
        || card.description.to_lowercase().contains(&q)
}

/// Number of cards in `tab` matching `query`.
fn tab_match_count(tab: &tess_core::Tab, query: &str) -> usize {
    tab.cards.iter().filter(|c| search_matches(c, query)).count()
}

/// Index (position in `TilingCategory::ALL` order) of the first tab
/// with at least one match for `query`; `None` for an empty query or
/// when nothing matches anywhere.
fn first_matching_tab(tabs: &[tess_core::Tab], query: &str) -> Option<usize> {
    if query.trim().is_empty() {
        return None;
    }
    tabs.iter().position(|t| t.cards.iter().any(|c| search_matches(c, query)))
}

/// One maximal run of a card list sharing one `section_key`.
/// `header` is `""` for runs without a header: cards in tabs with no
/// sections, and cards without the declared facts behind their tab's
/// sections (e.g. a Wallpaper card with no declared group).
struct CardSection<'a> {
    header: &'static str,
    cards: &'a [tess_core::CardMeta],
}

/// Split a card list into maximal runs of equal
/// [`tess_core::section_key`], in first-appearance order. On the
/// Wallpaper tab this yields the crystal-system sections
/// (oblique/rectangular/centered/square/hexagonal, in that order); on
/// the Laves tab it yields the three rank bands (self-dual regulars &
/// Catalan duals, isohedral variants, 2-uniform duals); every other
/// tab is a single headerless run. `tab_category` is the tab the
/// cards are displayed in — the section key is a property of the
/// (tab, card) pair, since the Wallpaper tab's cards are secondary
/// members whose primary category is their own family.
fn sectionize<'a>(
    tab_category: tess_core::TilingCategory,
    cards: &'a [tess_core::CardMeta],
) -> Vec<CardSection<'a>> {
    let mut out: Vec<CardSection<'a>> = Vec::new();
    let mut i = 0;
    while i < cards.len() {
        let key = tess_core::section_key(tab_category, &cards[i]);
        let mut j = i + 1;
        while j < cards.len() && tess_core::section_key(tab_category, &cards[j]) == key {
            j += 1;
        }
        out.push(CardSection { header: key, cards: &cards[i..j] });
        i = j;
    }
    out
}

/// The card to select when (re-)entering a tab: the remembered
/// selection if it is still in the tab, else the first card, else
/// `None` (empty tab — the caller keeps the current selection).
fn entry_selection(tab: &tess_core::Tab, remembered: Option<&str>) -> Option<&'static str> {
    if let Some(r) = remembered {
        if tab.card_index_of(r).is_some() {
            return Some(tab.cards[tab.card_index_of(r).unwrap()].id);
        }
    }
    tab.cards.first().map(|c| c.id)
}

/// Responsive column count for the browse grid: how many `card_w`-wide
/// columns fit in `width` — fully dynamic, with no upper bound (`card_w`
/// is the squash floor: wider windows get more columns, not ever
/// wider cards); at least one (a very narrow window still shows a
/// single column). Pure, unit-tested.
fn columns_for_width(width: f32, card_w: f32) -> usize {
    let raw = if width.is_finite() && card_w.is_finite() && card_w > 0.0 {
        (width / card_w).floor() as usize
    } else {
        1
    };
    raw.max(1)
}

/// The card target width that lays the grid out as exactly `cols`
/// columns at `avail` — the inverse of [`columns_for_width`] taken at
/// the regime's upper edge (`avail/cols` floors back to `cols`); a
/// relative epsilon guards the f32 division from rounding one ulp
/// below the count. Column-count stepping (the `−`/`+` buttons and
/// keys) lands the target exactly here, so every step reflows the grid
/// visibly — unlike raw width steps, which sit invisible inside a
/// column regime. Pure, unit-tested.
fn card_target_for_cols(avail: f32, cols: usize) -> f32 {
    if cols == 0 {
        return CARD_W_MIN;
    }
    avail / cols as f32 * (1.0 - 1e-5)
}

/// Column width for a browse grid of `cols` columns given `avail`
/// available width: budget the inter-column spacing and split the
/// rest among the columns. Each card's outer rect (the grid cell,
/// [`card_outer_w`]) is exactly this wide, so the grid's total width
/// `cols·col_w + (cols−1)·spacing` equals `avail` — the grid fills
/// the window width exactly instead of overflowing. Pure,
/// unit-tested.
fn card_col_w(avail: f32, cols: usize, spacing: f32) -> f32 {
    if cols == 0 {
        return 0.0;
    }
    ((avail - (cols - 1) as f32 * spacing) / cols as f32).max(0.0)
}

/// The card's inner ui width for a `col_w`-wide column: the column
/// minus the frame's two inner margins. Constraining the inner ui to
/// this width makes the frame's outer rect — which egui computes as
/// inner + 2·margin — exactly `col_w` (the inverse of
/// [`card_col_w`]'s budget), so no card is wider than its column.
/// Pure, unit-tested.
fn card_inner_w(col_w: f32, margin: f32) -> f32 {
    (col_w - 2.0 * margin).max(0.0)
}

/// The card frame's outer (grid-cell) width for an inner ui width:
/// the inner width plus the frame's two inner margins — the inverse
/// of [`card_inner_w`] (for widths not clamped at zero). Used by the
/// width-math unit tests; the production path consumes
/// [`card_inner_w`] directly.
#[allow(dead_code)] // only the unit tests reference this inverse
fn card_outer_w(inner_w: f32, margin: f32) -> f32 {
    inner_w + 2.0 * margin
}

/// Next card index when walking `step` (±1; any integer wraps the
/// same way) from `cur` in a tab of `n` cards — the TUI's render-mode
/// prev/next. `None` for an empty tab; a missing or out-of-range
/// `cur` starts at the first card.
fn walk_index(n: usize, cur: Option<usize>, step: i32) -> Option<usize> {
    if n == 0 {
        return None;
    }
    let cur = cur.unwrap_or(0).min(n - 1) as i32;
    Some(((cur + step).rem_euclid(n as i32)) as usize)
}

/// The shared color-mode selector row: all ten modes in
/// [`ColorMode::ALL`] order as selectable labels, `Some(current)`
/// highlighting the active one, optionally led by a `default` label
/// (option `None`) — the Browse band's "follow each tiling's own
/// default" choice. Both mode-selection sites draw through here — the
/// Render detail panel and the Browse preview band — so the option
/// list and click semantics cannot drift; each caller applies the
/// clicked option through [`GuiApp::set_preview_mode`], the single
/// state funnel (canvas `app.color_mode` + preview override together).
/// Returns each option's `(mode, rect, clicked)`; the rects feed the
/// headless click test (the `card_rects` precedent).
fn mode_selector_row(
    ui: &mut egui::Ui,
    current: Option<ColorMode>,
    include_default: bool,
) -> Vec<(Option<ColorMode>, egui::Rect, bool)> {
    let mut out = Vec::with_capacity(ColorMode::ALL.len() + 1);
    ui.horizontal(|ui| {
        if include_default {
            let r = ui.selectable_label(current.is_none(), "default");
            out.push((None, r.rect, r.clicked()));
        }
        for m in ColorMode::ALL {
            let r = ui.selectable_label(current == Some(m), m.label());
            out.push((Some(m), r.rect, r.clicked()));
        }
    });
    out
}

/// Card-preview cache key: (id, params-string, effective preview mode,
/// size). The mode is part of the key so a preview-band mode change
/// regenerates the textures (the old keys simply miss; the stale ones
/// are dropped in [`GuiApp::set_preview_mode`]). The size stays the
/// last segment — [`GuiApp::set_card_target_w_impl`] prunes on its
/// `|WxH` suffix.
fn card_key(id: &str, params: &str, mode: ColorMode, w: u32, h: u32) -> String {
    format!("{id}|{params}|{}|{w}x{h}", mode.label())
}

/// `glb-binary-n7` id — the only tiling whose card preview changes.
const GLB_BINARY_N7_ID: &str = "glb-binary-n7";
/// Preview-only depth pin for `glb-binary-n7` (see `preview_params`).
const GLB_BINARY_N7_PREVIEW_DEPTH: i64 = 3;

/// The params the card preview actually renders with: the tiling
/// defaults, except for `glb-binary-n7`, whose preview is pinned to
/// depth 3 (the same cap the e01fc76 scale override enforced, now via
/// the tiling's own `depth` param). Everything else (probe, window
/// derivation, all other tilings) renders exactly as before.
fn preview_params(default_params: &ParamSet, tiling_id: &str) -> ParamSet {
    if tiling_id != GLB_BINARY_N7_ID {
        return default_params.clone();
    }
    let mut p = default_params.clone();
    p.set(
        tess_tilings::families::quasiperiodic::glb_binary_n7::DEPTH_ID,
        ParamValue::Int(GLB_BINARY_N7_PREVIEW_DEPTH),
    );
    p
}

/// The params-string of the values the card preview actually renders
/// with — the cache key's params component. Same declaration-order
/// format as the former `default_params_str`, but carrying the
/// overridden value for `glb-binary-n7`.
fn preview_params_str(specs: &[ParamSpec], params: &ParamSet) -> String {
    specs
        .iter()
        .map(|s| {
            let v = params.get(s.id).unwrap_or_else(|| s.default_value());
            format!("{}={}", s.id.as_str(), v.display())
        })
        .collect::<Vec<_>>()
        .join(",")
}

/// Character-safe truncation with an ellipsis (descriptions contain
/// non-ASCII characters, e.g. the en dash in "Ammann–Beenker", so byte
/// slicing is not allowed).
fn truncate_text(s: &str, max: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max {
        return s.to_string();
    }
    let mut out: String = chars[..max - 1].iter().collect();
    out.push('…');
    out
}

/// The card title in exactly two lines of at most
/// `NAME_CHARS_PER_LINE` characters each: the whole name on line 1
/// (line 2 empty) when it fits the budget, otherwise the name broken
/// at the last word boundary inside the first budget (a bare
/// character cut only when no boundary is reachable), so line two
/// never starts mid-word (a name longer than the 2-line budget
/// truncates on its second line). Both lines keep label-level
/// ellipsis truncation as a backstop at the card's width, and both
/// rows are always reserved so every card keeps the same height.
fn title_lines(name: &str, per_line: usize) -> [String; 2] {
    if name.chars().count() <= per_line {
        return [name.to_string(), String::new()];
    }
    let chars: Vec<char> = name.chars().collect();
    // Prefer the last space in the first budget, but not so early
    // that line 1 degenerates into a stub.
    let min_split = (per_line * 2) / 3;
    let split = (min_split..per_line).rev().find(|&i| chars[i] == ' ');
    let (first, rest): (String, String) = match split {
        Some(i) => (
            chars[..i].iter().collect(),
            chars[i + 1..].iter().collect(),
        ),
        None => (
            chars[..per_line].iter().collect(),
            chars[per_line..].iter().collect(),
        ),
    };
    [first, truncate_text(&rest, per_line * 2)]
}

impl GuiApp {
    pub fn new() -> Self {
        let app = tess_core::App::new(tess_tilings::registry(), 60.0);
        // The zoom label's 100% baseline: the construction zoom (the
        // render bar has no other record of "default zoom").
        let zoom_default = app.viewport.zoom;
        // The classification table drives the declared tab sort orders
        // (pentagon types, 2-uniform configurations, Laves ranks,
        // wallpaper crystal systems) and supplies each card's schema.
        let tabs =
            tess_core::browse_tabs_with(app.registry(), tess_tilings::classification);
        let selected = app.selected_id().to_string();
        let active_tab = tabs
            .iter()
            .position(|t| t.cards.iter().any(|c| c.id == selected))
            .unwrap_or(0);
        let mut tab_selected = vec![None; tabs.len()];
        tab_selected[active_tab] = Some(selected.clone());
        Self {
            app,
            mode: GuiMode::Browse,
            params_open: true,
            meta: None,
            render_cache: None,
            render_cache_misses: 0,
            tabs,
            active_tab,
            tab_selected,
            search_at_jump: String::new(),
            last_highlighted: (active_tab, selected),
            card_textures: HashMap::new(),
            card_target_w: CARD_TARGET_W,
            search: String::new(),
            card_rects: Vec::new(),
            card_grid_col_w: 0.0,
            card_grid_cols: 0,
            card_grid_avail: 0.0,
            canvas_rect: None,
            zoom_default,
            about_open: false,
            preview_mode: None,
            preview_mode_rects: Vec::new(),
        }
    }

    /// The shared params fingerprint: every current parameter value as
    /// `|id={:?}` fragments, in spec order. The tiling id is added by
    /// each caller — the metadata cache prepends it inline, the canvas
    /// cache stores it as its own key field.
    fn params_fingerprint(&self) -> String {
        let mut key = String::new();
        for spec in self.app.tiling().params() {
            let v = self
                .app
                .param_value(spec.id)
                .unwrap_or_else(|| spec.default_value());
            key.push_str(&format!("|{}={:?}", spec.id, v));
        }
        key
    }

    /// The cached metadata for the current tiling + parameters, refreshing
    /// it on a miss. The cache key is the tiling id plus every current
    /// parameter value. The probe region is a tiny box at the origin,
    /// which every generator covers instantly (aperiodic patches grow
    /// from the origin) — so a refresh can never kick off expensive
    /// generation, and panning the view never touches this path.
    fn metadata(&mut self) -> &Meta {
        let key = format!("{}{}", self.app.selected_id(), self.params_fingerprint());
        if self.meta.as_ref().is_none_or(|m| m.key != key) {
            let r = self.app.result_region(tess_core::BBox::new(
                tess_core::Vec2::new(-0.5, -0.5),
                tess_core::Vec2::new(0.5, 0.5),
            ));
            self.meta = Some(Meta {
                key,
                symmetry: r.symmetry,
                periodic: r.periodic,
            });
        }
        self.meta.as_ref().unwrap()
    }

}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("titlebar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("tessellix");
                ui.separator();
                ui.label(self.app.tiling().name());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("About").clicked() {
                        self.about_open = !self.about_open;
                    }
                });
            });
        });

        match self.mode {
            GuiMode::Browse => self.draw_browse(ctx),
            GuiMode::Render => self.draw_render(ctx),
        }

        // The H key toggles the About panel. Guarded against text-field
        // focus (Browse's search field) exactly like the +/- zoom keys —
        // typing H there keeps its character.
        if !ctx.wants_keyboard_input() && ctx.input(|i| i.key_pressed(egui::Key::H)) {
            self.about_open = !self.about_open;
        }
        self.draw_about(ctx);
    }
}

impl GuiApp {
    /// The About panel: app name + version, the data attributions
    /// rendered content actually draws on, and a one-line pointer at
    /// the per-tiling source docs (`docs/tilings/*.md`). A centered,
    /// non-modal, non-resizable egui window toggled by the H key and
    /// the title-bar About button — purely an overlay, so it enters
    /// no cache key or RenderKey. Closed by H again, the panel's ×
    /// button, or the About button (which flips the state).
    fn draw_about(&mut self, ctx: &egui::Context) {
        if !self.about_open {
            return;
        }
        egui::Window::new("About")
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .movable(false)
            .collapsible(false)
            .resizable(false)
            .fade_in(false)
            .default_width(400.0)
            .open(&mut self.about_open)
            .show(ctx, |ui| {
                ui.heading("tessellix");
                ui.label(format!("v{}", env!("CARGO_PKG_VERSION")));
                ui.separator();
                ui.label(egui::RichText::new("Data attributions").strong());
                // Kaplan/Tactile: the five shipped isohedral instances
                // are transcribed Tactile incidence-symbol data (the
                // credit lives here; the cards carry none). The five
                // IH tiles each carry their own docs/tilings/*.md.
                ui.label(
                    "Kaplan/Tactile incidence-symbol data — the five shipped isohedral (IH) instances \
                     (docs/families/isohedral.md)",
                );
                ui.label(
                    "Jeandel–Rao 11-tile Wang tiles, arXiv:1506.06492 — the author's canonical \
                     100×100 patch (docs/tilings/jeandel-rao.md)",
                );
                ui.separator();
                ui.label("Per-tiling sources: docs/tilings/*.md");
                ui.label("H toggles this panel · × closes");
            });
    }
}

impl GuiApp {
    /// Browse mode: the tab band (tabs + search, one wrapping row) over
    /// the card grid filling the window.
    fn draw_browse(&mut self, ctx: &egui::Context) {
        // Card-size controls, consumed BEFORE the scroll area is
        // created. Ctrl+wheel and trackpad pinch arrive through egui's
        // `zoom_delta()` (egui 0.29 diverts ctrl/cmd-scroll away from
        // `raw_scroll_delta`, so the grid never scroll-jumps); the +/-
        // keys step the same target. Typing + or - in the search field
        // keeps its characters (the keyboard guard skips the keys
        // while any text field owns the keyboard).
        let zd = ctx.input(|i| i.zoom_delta());
        if zd != 1.0 {
            self.set_card_target_w(self.card_target_w * zd);
        }
        if !ctx.wants_keyboard_input() {
            let (plus, minus) = ctx.input(|i| {
                (
                    i.key_pressed(egui::Key::Equals),
                    i.key_pressed(egui::Key::Minus),
                )
            });
            // One COLUMN per press — a raw width step is invisible until
            // it crosses a column threshold (the grid fills the window),
            // which read as dead keys. "+" = bigger cards (fewer
            // columns), "−" = smaller cards (more columns).
            if plus {
                self.step_card_columns(-1);
            }
            if minus {
                self.step_card_columns(1);
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            self.search_jump();
            self.draw_tab_row(ui);
            self.draw_preview_mode_row(ui);
            ui.separator();
            egui::ScrollArea::vertical().id_salt("browser").show(ui, |ui| {
                self.draw_card_grid(ctx, ui);
            });
        });
    }

    /// Set a new card target width (clamped to
    /// [`CARD_W_MIN, CARD_W_MAX`]) and, when it actually changed,
    /// prune the preview textures of other sizes — every card renders
    /// at one shared size, so old-size textures are dead weight (the
    /// cache is keyed by exact pixel size, and continuous wheel zoom
    /// would otherwise accumulate one generation per step).
    fn set_card_target_w(&mut self, w: f32) {
        self.set_card_target_w_impl(w.clamp(CARD_W_MIN, CARD_W_MAX));
    }

    /// Column-stepping path: the target IS the column geometry
    /// (`avail/cols`, [`card_target_for_cols`]), which legitimately
    /// leaves `[CARD_W_MIN, CARD_W_MAX]` on either side — few huge
    /// cards or many small ones are the point of a column step. The
    /// wheel path keeps the clamp; this one only prunes.
    fn set_card_target_w_exact(&mut self, w: f32) {
        self.set_card_target_w_impl(w);
    }

    fn set_card_target_w_impl(&mut self, w: f32) {
        if w != self.card_target_w {
            self.card_target_w = w;
            let (iw, ih) = card_img_size(w);
            let suffix = format!("|{iw}x{ih}");
            self.card_textures
                .retain(|k, _| k.ends_with(&suffix));
        }
    }

    /// Step the card grid by `delta` columns (+ = more columns,
    /// smaller cards; − = fewer, bigger), landing the target exactly on
    /// the new regime's edge ([`card_target_for_cols`]) so every press
    /// reflows the grid visibly. The current count is derived from the
    /// target itself (not the last pass's recorded count), so repeated
    /// presses within one frame step correctly. Clamped to `[1, the
    /// column count at [`CARD_W_MIN`]]`; a no-op before the first grid
    /// pass records the geometry.
    fn step_card_columns(&mut self, delta: i32) {
        let avail = self.card_grid_avail;
        if !(avail > 0.0) {
            return;
        }
        let cur = columns_for_width(avail, self.card_target_w);
        let max_cols = columns_for_width(avail, CARD_W_MIN).max(1);
        let next = (cur as i32 + delta).clamp(1, max_cols as i32) as usize;
        if next != cur {
            self.set_card_target_w_exact(card_target_for_cols(avail, next));
        }
    }

    /// The single funnel for both color-mode selector sites (the
    /// Render detail panel, the Browse preview band). Sets the preview
    /// override (`None` = each tiling's own default preview mode) and
    /// drives `app.color_mode` — the mode the canvas paints with — in
    /// lockstep, so the two views share one state:
    ///
    /// * `Some(m)`: previews and canvas show `m` (previews additionally
    ///   pass through [`ColorMode::preview_mode`], keeping the
    ///   Single→Proto thumbnail fallback);
    /// * `None`: previews revert to each tiling's `default_color_mode`
    ///   and the canvas to the mode the current selection opened with.
    ///
    /// A real override change clears the card-texture cache (keys embed
    /// the effective mode), so previews regenerate lazily in the new
    /// colors; the canvas needs no invalidation — painting reads
    /// `color_mode` live every frame (the RenderKey invariant).
    fn set_preview_mode(&mut self, m: Option<ColorMode>) {
        if self.preview_mode != m {
            self.preview_mode = m;
            self.card_textures.clear();
        }
        self.app.color_mode = m.unwrap_or_else(|| self.app.tiling().default_color_mode());
    }

    /// The effective preview color mode for `tiling`: the explicit
    /// override while one is set, else the tiling's own default.
    /// [`GuiApp::card_texture`] renders in this mode's
    /// [`ColorMode::preview_mode`].
    fn card_preview_mode(&self, tiling: &dyn tess_core::tiling::Tiling) -> ColorMode {
        self.preview_mode.unwrap_or_else(|| tiling.default_color_mode())
    }

    /// `App::select_id` plus the preview-override re-application:
    /// `select_id` resets `app.color_mode` to the newly selected
    /// tiling's default (the shared tess-core behavior), but an
    /// explicit mode choice made in either selector is meant to
    /// survive card navigation, so while an override is set the canvas
    /// returns to it. With no override the historical per-tiling
    /// default behavior is untouched.
    fn select_id_synced(&mut self, id: &str) {
        self.app.select_id(id);
        if let Some(m) = self.preview_mode {
            self.app.color_mode = m;
        }
    }

    /// The Browse preview band's color-mode row: `default` (follow
    /// each tiling's own preview mode) plus the same ten options as
    /// the Render detail panel, through the shared
    /// [`mode_selector_row`]. Records every option's rect for the
    /// headless click test (the `card_rects` precedent).
    fn draw_preview_mode_row(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Color mode");
            let picks = mode_selector_row(ui, self.preview_mode, true);
            self.preview_mode_rects =
                picks.iter().map(|(m, r, _)| (*m, *r)).collect();
            if let Some((m, _, true)) = picks.iter().find(|(_, _, c)| *c) {
                self.set_preview_mode(*m);
            }
        });
    }

    /// Render mode: Escape/arrow handling, the top strip (back / prev /
    /// next / params / reset), the full-window canvas, and the
    /// collapsible bottom params / color-mode / symmetry panel.
    fn draw_render(&mut self, ctx: &egui::Context) {
        // Escape returns to Browse exactly as left (same tab +
        // selection). Nothing in the render view can swallow the key:
        // the search field is Browse-only, a focused param DragValue
        // deliberately ignores left/right arrows in keyboard-edit mode
        // (egui 0.29, drag_value.rs) and buttons never take focus on
        // click, so Escape always reaches us.
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.mode = GuiMode::Browse;
        }
        // Left/Right walk the tab order (the TUI's render-mode
        // prev/next). No widget in this view consumes left/right
        // arrows — a focused param DragValue only eats Up/Down — so
        // the keys are safe to bind unconditionally (key_pressed
        // respects widget consumption, so a consumer would silence
        // the walk automatically).
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            self.walk_render(-1);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            self.walk_render(1);
        }

        egui::TopBottomPanel::top("render_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("← back").clicked() {
                    self.mode = GuiMode::Browse;
                }
                ui.separator();
                if ui.button("◀").clicked() {
                    self.walk_render(-1);
                }
                if ui.button("▶").clicked() {
                    self.walk_render(1);
                }
                let params_label = if self.params_open { "params ▾" } else { "params ▴" };
                if ui.button(params_label).clicked() {
                    self.params_open = !self.params_open;
                }
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label(self.app.tiling().name());
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⟲ reset").clicked() {
                        self.app.reset_view();
                    }
                    ui.separator();
                    // Physical zoom cluster (wheel/pinch don't arrive on
                    // every desktop): +/− step around the canvas center,
                    // the percentage label doubles as zoom-reset.
                    if ui.button("+").clicked() {
                        self.zoom_by_step(1);
                    }
                    if ui
                        .button(self.zoom_label())
                        .on_hover_text("reset zoom to 100%")
                        .clicked()
                    {
                        self.zoom_reset();
                    }
                    if ui.button("−").clicked() {
                        self.zoom_by_step(-1);
                    }
                });
            });
        });

        if self.params_open {
            egui::TopBottomPanel::bottom("render_params").show(ctx, |ui| {
                self.draw_params(ui);
                ui.separator();
                ui.label("Color mode");
                let picks = mode_selector_row(ui, Some(self.app.color_mode), false);
                if let Some((m, _, true)) = picks.iter().find(|(_, _, c)| *c) {
                    // Same funnel as the Browse preview band: the
                    // canvas recolors live (paint reads `color_mode`)
                    // and the preview override follows, so Browse
                    // previews recolor to the detail view's choice.
                    self.set_preview_mode(*m);
                }
                ui.separator();
                let m = self.metadata();
                ui.horizontal(|ui| {
                    ui.label(format!("wallpaper group: {}", m.symmetry.group));
                    ui.label(format!("lattice: {}", m.symmetry.lattice));
                });
                if let Some([l1, l2]) = m.periodic {
                    ui.label(format!(
                        "basis: ({:.2},{:.2}) ({:.2},{:.2})",
                        l1.x, l1.y, l2.x, l2.y
                    ));
                }
                if let Some(n) = m.symmetry.notes {
                    ui.label(n);
                }
            });
        }

        self.draw_canvas(ctx);
    }

    /// The full-width tab band: one selectable label per category in
    /// `TilingCategory::ALL` order with the card count (the match
    /// count while a search is active), then the search field, all in
    /// one wrapping row.
    fn draw_tab_row(&mut self, ui: &mut egui::Ui) {
        let searching = !self.search.trim().is_empty();
        // Two-phase: read tab state in the band, apply the click after,
        // so the band loop's borrow of `self.tabs` does not collide
        // with the mutable `select_tab` call.
        let mut clicked: Option<usize> = None;
        ui.horizontal_wrapped(|ui| {
            for (i, tab) in self.tabs.iter().enumerate() {
                let count = if searching {
                    tab_match_count(tab, &self.search)
                } else {
                    tab.card_count()
                };
                if ui
                    .selectable_label(
                        i == self.active_tab,
                        format!("{} ({})", tab.category, count),
                    )
                    .clicked()
                {
                    clicked = Some(i);
                }
            }
            ui.add(
                egui::TextEdit::singleline(&mut self.search)
                    .hint_text("search")
                    .desired_width(160.0),
            );
            // Physical card-size controls (the wheel/pinch gestures
            // don't arrive on every desktop): one column per press, the
            // count label doubles as reset-to-default. "+" = bigger
            // cards (fewer columns), "−" = smaller (more).
            ui.separator();
            if ui.button("−").clicked() {
                self.step_card_columns(1);
            }
            let cols_label = if self.card_grid_cols >= 1 {
                format!("{}×", self.card_grid_cols)
            } else {
                "cards".to_string()
            };
            if ui
                .button(cols_label)
                .on_hover_text("reset card size")
                .clicked()
            {
                self.set_card_target_w(CARD_TARGET_W);
            }
            if ui.button("+").clicked() {
                self.step_card_columns(-1);
            }
        });
        if let Some(i) = clicked {
            self.select_tab(i);
        }
    }

    /// Prev/next in the selected tiling's own tab (the tab you entered
    /// render from), wrapping — mirrors the TUI's `walk`, which
    /// follows the selected tiling's category. Updates the selection
    /// and that tab's selection memory so returning to Browse lands on
    /// the walked card.
    fn walk_render(&mut self, step: i32) {
        let ci = self
            .tabs
            .iter()
            .position(|t| t.card_index_of(self.app.selected_id()).is_some())
            .unwrap_or(self.active_tab);
        let tab = &self.tabs[ci];
        let next = match walk_index(
            tab.cards.len(),
            tab.card_index_of(self.app.selected_id()),
            step,
        ) {
            Some(i) => i,
            None => return,
        };
        let id = tab.cards[next].id;
        self.select_id_synced(id);
        self.tab_selected[ci] = Some(id.to_string());
    }

    /// The TUI's search jump: when the search text changes, the active
    /// tab moves to the first tab with at least one match. The
    /// selection itself is untouched (the user picks a card explicitly);
    /// an empty query never jumps.
    fn search_jump(&mut self) {
        if self.search.trim() == self.search_at_jump.trim() {
            return;
        }
        if let Some(idx) = first_matching_tab(&self.tabs, &self.search) {
            self.active_tab = idx;
        }
        self.search_at_jump = self.search.clone();
    }

    /// Select a tiling from the card grid: the app selection plus the
    /// active tab's selection memory, then enter Render mode (the
    /// full-window canvas), with the params panel re-expanded — the
    /// TUI's `open_render` (cursor kept in sync so `Esc` returns to
    /// this card).
    fn select_card(&mut self, id: &str) {
        self.select_id_synced(id);
        self.tab_selected[self.active_tab] = Some(id.to_string());
        self.mode = GuiMode::Render;
        self.params_open = true;
    }

    /// Switch the active tab, restoring that tab's remembered card
    /// (per-tab selection memory; the canvas shows it on the next
    /// entry to Render).
    fn select_tab(&mut self, idx: usize) {
        if idx == self.active_tab {
            return;
        }
        self.active_tab = idx;
        if let Some(id) = entry_selection(&self.tabs[idx], self.tab_selected[idx].as_deref()) {
            self.select_id_synced(id);
            self.tab_selected[idx] = Some(id.to_string());
        }
    }

    /// The card grid filling the window: section headers (from
    /// `tess_core::section_key` — the crystal systems on the Wallpaper
    /// tab, the rank bands on the Laves tab) between rows of cards,
    /// each card = preview image + name + truncated description +
    /// small "group · lattice" line. Responsive column count
    /// ([`columns_for_width`]) with the cards stretched to fill: the
    /// column width is the available width minus the inter-column
    /// spacing, split among the columns ([`card_col_w`]), and each
    /// card allocates exactly that width ([`draw_card`]'s explicit
    /// allocation), so the grid fills the window edge to edge without
    /// overflowing.
    fn draw_card_grid(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // Geometry diagnostics for the headless pass test: record the
        // rects of THIS pass only, so assertions read the last pass.
        self.card_rects.clear();
        let tab = &self.tabs[self.active_tab];
        // Owned snapshot of the filtered, in-order cards. The grid
        // closure below needs `&mut self` (texture cache, selection),
        // so no borrow of `self.tabs` may survive into it: every field
        // of `CardMeta` is `Copy`, so a struct literal copies each card
        // without requiring a `Clone` impl on the shared type.
        let cards: Vec<tess_core::CardMeta> = tab
            .cards
            .iter()
            .filter(|c| search_matches(c, &self.search))
            .map(|c| tess_core::CardMeta {
                id: c.id,
                name: c.name,
                description: c.description,
                category: c.category,
                group: c.group,
                lattice: c.lattice,
                notes: c.notes,
                params: c.params,
                class: c.class,
            })
            .collect();
        if cards.is_empty() {
            ui.label(if self.search.trim().is_empty() {
                "(no tilings in this tab)"
            } else {
                "(no matches)"
            });
            return;
        }
        // The Wallpaper tab (one card per group) opens with the
        // explicit catalog note — since M85 all 17 groups are
        // represented, the note reports the completed catalog.
        if tab.category == tess_core::TilingCategory::Wallpaper {
            ui.add(
                egui::Label::new(egui::RichText::new(
                    tess_core::wallpaper_gap_note(),
                )
                .small()
                .weak())
                .wrap_mode(egui::TextWrapMode::Wrap),
            );
            ui.add_space(2.0);
        }
        // Responsive columns over the current card target width ([`
        // columns_for_width`], fully dynamic — at least one, no
        // upper bound); the column width is the available width
        // minus the grid's horizontal spacing, split among the
        // columns ([`card_col_w`]). The preview image scales with
        // the target ([`card_img_size`] — ctrl+wheel / pinch / +/-
        // change it), and the card height follows so the grid stays
        // orthogonal at every zoom level.
        let avail = ui.available_width();
        let cols = columns_for_width(avail, self.card_target_w);
        let col_w = card_col_w(avail, cols, GRID_SPX);
        let img = card_img_size(self.card_target_w);
        self.card_grid_col_w = col_w;
        // Record the pass geometry for the +/− column-stepping
        // controls (they compute the next regime's target from these).
        self.card_grid_avail = avail;
        self.card_grid_cols = cols;
        // Copy the category out of the tab before the grid closure:
        // the closure takes `&mut self`, so no borrow of `self.tabs`
        // (or of `tab`) may survive into it.
        let tab_category = tab.category;
        // Scroll the selection into view only when (tab, selection)
        // actually changed since the last frame — the TUI's
        // last_highlighted guard; without it, manual scrolling would
        // be snapped back on every render.
        let changed = self.last_highlighted.0 != self.active_tab
            || self.last_highlighted.1 != self.app.selected_id();
        egui::Grid::new("card_grid").spacing([GRID_SPX, 8.0]).show(ui, |ui| {
            // Section the (already-ordered) card snapshot by
            // `section_key`; the snapshot is a local, so the closure
            // holds no borrows of `self` besides the `&mut` it needs
            // for the texture cache and click handling.
            for section in sectionize(tab_category, &cards) {
                if !section.header.is_empty() {
                    ui.end_row();
                    ui.add(egui::Label::new(egui::RichText::new(
                        section.header.to_string(),
                    )
                    .strong()
                    .color(SECTION_COLOR)));
                    ui.end_row();
                }
                for row in section.cards.chunks(cols) {
                    for card in row {
                        self.draw_card(ctx, ui, card, col_w, img, changed);
                    }
                    ui.end_row();
                }
            }
        });
        if changed {
            self.last_highlighted = (
                self.active_tab,
                self.app.selected_id().to_string(),
            );
        }
    }

    /// One card: preview texture (generated lazily, visible cards
    /// only), the two-line name, a truncated description, and a small
    /// "group · lattice" symmetry line — all STACKED vertically.
    ///
    /// The body is built with explicit geometry, no inherited layout:
    /// egui's `Grid` places cells inside a left-to-right ui, and
    /// `Frame::show`'s content child inherits that layout (`Ui::new_child`
    /// keeps the parent's `Layout` when the builder gives none), which
    /// flowed image + name + description + symmetry sideways — the
    /// description truncated to "…" and each card's min_rect grew past
    /// its column, overflowing the row. So instead of a Frame: the card
    /// allocates its exact cell rect ([`col_w`] × [`card_total_h`]) up
    /// front, paints fill/stroke on that rect (rounding 6.0 — the
    /// look the old `Frame` had), and draws the content in an explicit
    /// [`egui::Layout::top_down`] child inset by [`CARD_INNER_MARGIN`],
    /// where each single-line (ellipsized, never wrapped) label
    /// truncates at the full inner width. Every card gets the identical
    /// rect, so the grid stays orthogonal. Clicking selects the tiling
    /// and opens Render mode; the selected card gets the highlight fill
    /// and stroke.
    fn draw_card(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        card: &tess_core::CardMeta,
        col_w: f32,
        img: (u32, u32),
        scroll_to_selected: bool,
    ) {
        let selected = card.id == self.app.selected_id();
        let img_size = egui::vec2(img.0 as f32, img.1 as f32);
        let total_h = card_total_h(img.1);
        // The exact cell. Inside the grid, `next_cell` = max(allocated,
        // prev_col_width) = col_w, so the cell size — and the grid's
        // total width — are decided here, independent of the content.
        let (card_rect, _cell) =
            ui.allocate_exact_size(egui::vec2(col_w, total_h), egui::Sense::hover());
        self.card_rects.push(card_rect);
        // Lazy texture generation: generate (or reuse) the texture only
        // when the card's rect is near the scroll area's clip rect, so
        // the first frame pays only for the cards actually in view.
        let handle = if ui.clip_rect().expand(32.0).intersects(card_rect) {
            Some(self.card_texture(ctx, card, img.0, img.1))
        } else {
            None
        };
        // The card's look, painted straight onto the allocated rect:
        // fill + outline with the old Frame's rounding and colors
        // (selected: highlight fill + thicker cyan stroke).
        ui.painter().rect_filled(
            card_rect,
            6.0,
            if selected { SELECTED_FILL } else { egui::Color32::TRANSPARENT },
        );
        ui.painter().rect_stroke(
            card_rect,
            6.0,
            egui::Stroke::new(
                if selected { 2.0_f32 } else { 1.0_f32 },
                if selected { SELECTED_STROKE } else { CARD_STROKE },
            ),
        );
        // The content: an explicit TopDown child inset by the margin.
        // (Explicit `layout` is the whole point — without it, `new_child`
        // inherits the grid's horizontal flow.) Detached from the
        // parent's layout on purpose: the cell was already allocated,
        // and the child's painter shares the context's shape list, so
        // its shapes paint without merging min_rects back.
        let mut content = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(card.id)
                .max_rect(egui::Rect::from_min_size(
                    card_rect.min + egui::vec2(CARD_INNER_MARGIN, CARD_INNER_MARGIN),
                    egui::vec2(card_inner_w(col_w, CARD_INNER_MARGIN), card_inner_h(img.1)),
                ))
                .layout(egui::Layout::top_down(egui::Align::LEFT)),
        );
        // Explicit row spacing (not the style's item spacing): the
        // card's content height is a constant, so all cards line up in
        // an orthogonal grid.
        content.spacing_mut().item_spacing.y = 0.0;
        match &handle {
            Some(h) => {
                content.add(egui::Image::from_texture(h).fit_to_exact_size(img_size));
            }
            None => {
                content.allocate_exact_size(img_size, egui::Sense::hover());
            }
        }
        content.add_space(CARD_ROW_SPX);
        // The card title: up to two lines of the name, each ellipsized
        // at the inner width. Both rows are always reserved (an empty
        // second line still takes its height, same pattern as the
        // empty-description case), so every card keeps the uniform
        // card size.
        let tlines = title_lines(card.name, NAME_CHARS_PER_LINE);
        content.add(
            egui::Label::new(egui::RichText::new(tlines[0].clone()).strong().size(16.0))
                .truncate(),
        );
        content.add_space(CARD_ROW_SPX);
        content.add(
            egui::Label::new(egui::RichText::new(tlines[1].clone()).strong().size(16.0))
                .truncate(),
        );
        content.add_space(CARD_ROW_SPX);
        // The schema summary sits directly under the name (the declared
        // classification facts, composed by tess-core; falls back to the
        // description fragment when the schema adds nothing). An empty
        // label still reserves the row's height, so a card without a
        // summary keeps the uniform card size.
        let (sym, summary) = tess_core::browse::attr_lines(card);
        let desc = if summary.is_empty() {
            truncate_text(card.description, DESC_CHARS)
        } else {
            summary
        };
        content.add(
            egui::Label::new(egui::RichText::new(desc).size(13.0).color(DESC_COLOR))
                .truncate(),
        );
        content.add_space(CARD_ROW_SPX);
        // Symmetry line: "group · lattice" for periodic cards; the
        // declared translational character for the nonperiodic world
        // (never a bare "— · —").
        content.add(
            egui::Label::new(egui::RichText::new(sym).size(12.0).color(SYM_COLOR))
                .truncate(),
        );
        drop(content);
        // Click/hover on the exact cell rect, with a stable explicit id
        // (survives search/filter changes re-ordering the auto ids).
        let resp = ui.interact(card_rect, ui.id().with(card.id), egui::Sense::click());
        if resp.hovered() && !selected {
            ui.painter()
                .rect_stroke(card_rect, 6.0, egui::Stroke::new(1.0_f32, HOVER_STROKE));
        }
        if scroll_to_selected && selected {
            ui.scroll_to_rect(card_rect, Some(egui::Align::Center));
        }
        if resp.clicked() {
            let id = card.id.to_string();
            self.select_card(&id);
        }
    }

    /// The card preview texture for `card` at `w`×`h` pixels, lazily
    /// generated on first use and cached per (id, params-string, size).
    ///
    /// Window derivation is the TUI thumbnail's two-pass approach,
    /// shared via `tess_core::app`: a bounded ±10-world-unit probe
    /// captures the lattice basis (or the mean tile extent for
    /// aperiodic patches), `preview_window` fits the card aspect, and
    /// the generation retries with a shrinking window on refusal.
    ///
    /// Deliberately stays origin-centered: the `Tiling::default_view_center()`
    /// hook is consumed by the main view only (`App::new` / `reset_view`,
    /// via the hooked viewport), so a recentered thumbnail here would
    /// diverge from the main view's default framing. The ±10-unit probe
    /// and `preview_window` never read the hook; this card stays an
    /// identity view exactly like the TUI thumbnail, and the main view's
    /// per-tiling default is what changes.
    /// Rasterization uses the shared `tess_core::raster` with the
    /// identity palette, so pixel indices are positions in the
    /// effective mode's ramp (full 8-bit colors; the TUI maps the same
    /// indices to xterm-256). Colors follow the effective preview mode
    /// ([`GuiApp::card_preview_mode`]: the preview band's override
    /// while one is set, else the tiling's default) through
    /// [`ColorMode::preview_mode`] (`Single` falls back to `Proto`,
    /// which would render a featureless slab) — the TUI thumb
    /// semantics when no override is active. The ramp is selected per
    /// mode (mirroring [`paint::tile_rgb`]'s match) before the pixel
    /// loop: `Gray`/`PatternGray`/`Role`/`Metatile`/`Orientation`/
    /// `Level` each use their dedicated ramp; `Single`/`Pattern`/`Proto`
    /// /`Shape` fall back to the shared rainbow `PALETTE`. The identity
    /// palette has one entry per slot of the selected ramp, so a
    /// classification class lands on the ramp's own slot — the
    /// 8-entry rainbow-only map was hazard 2 and is gone.
    ///
    /// `glb-binary-n7` is the one tiling whose preview is rendered
    /// with overridden params (see `preview_params`): at its default
    /// scale 2.75 the preview window lands in the depth-4 budget band
    /// (cell-frame area ≈ 11.7 → est d=3 = 226, est d=4 = 858,
    /// est d=5 = 3264 > VIEW_BUDGET 2500 — the budget rule picks the
    /// largest d within budget, depth 4), which is ~7× more expensive
    /// per tile than the depth-3 probe and ~80% of the card's wall
    /// time. The preview is pinned to depth 3 via the tiling's own
    /// `depth` param instead of the rescaled-scale override it
    /// replaces (est d=3 = 226 ✓ — the preview generates ≈ that many
    /// tiles, same structural win). The window is unchanged (derived
    /// from the unmodified probe, which still runs at scale 2.75 and
    /// itself lands on depth 3); the rendered pattern keeps the
    /// default scale, so it is chunkier than the old finer-scaled
    /// preview — acceptable at card sizes. Only the preview generate
    /// + shrink-retry see the pin; the probe, the window derivation
    /// and every other tiling are untouched.
    fn card_texture(
        &mut self,
        ctx: &egui::Context,
        card: &tess_core::CardMeta,
        w: u32,
        h: u32,
    ) -> egui::TextureHandle {
        let tiling = self
            .app
            .registry()
            .get(card.id)
            .expect("card id must be registered");
        let params = ParamSet::default_for(tiling.params());
        let preview_params = preview_params(&params, tiling.id());
        let params_str = preview_params_str(tiling.params(), &preview_params);
        let mode = self.card_preview_mode(tiling).preview_mode();
        let key = card_key(card.id, &params_str, mode, w, h);
        if let Some(t) = self.card_textures.get(&key) {
            return t.clone();
        }
        let aspect = f64::from(w) / f64::from(h.max(1));
        let probe = tiling.generate(&params, &tess_core::Query {
            region: tess_core::BBox::new(
                tess_core::Vec2::new(-10.0, -10.0),
                tess_core::Vec2::new(10.0, 10.0),
            ),
        });
        let mut window = tess_core::app::preview_window(
            probe.periodic,
            tess_core::app::mean_tile_side(&probe.tiles),
            aspect,
        );
        let mut result = tiling.generate(&preview_params, &tess_core::Query { region: window });
        let mut shrink = 0.4;
        while result.message.is_some() && shrink > 0.05 {
            let cx = (window.min.x + window.max.x) / 2.0;
            let cy = (window.min.y + window.max.y) / 2.0;
            let hx = (window.max.x - window.min.x) / 2.0 * shrink;
            let hy = (window.max.y - window.min.y) / 2.0 * shrink;
            window = tess_core::BBox::new(
                tess_core::Vec2::new(cx - hx, cy - hy),
                tess_core::Vec2::new(cx + hx, cy + hy),
            );
            result = tiling.generate(&preview_params, &tess_core::Query { region: window });
            shrink *= 0.4;
        }
        // Ramp selection, once per mode (mirroring `paint::tile_rgb`'s
        // match): the fixed 8-entry rainbow-only map was hazard 2 —
        // every non-rainbow mode rendered rainbow. Each mode shades
        // from its own categorical ramp; `Single`(→`Proto` fallback)/
        // `Pattern`/`Proto`/`Shape` fall back to the shared rainbow
        // `PALETTE`.
        let ramp: &[u32] = match mode {
            ColorMode::Gray => &paint::GRAY_PALETTE,
            ColorMode::PatternGray => &paint::PATTERN_GRAY_PALETTE,
            ColorMode::Role => &paint::ROLE_PALETTE,
            ColorMode::Metatile => &paint::METATILE_PALETTE,
            ColorMode::Orientation => &paint::ORIENTATION_PALETTE,
            ColorMode::Level => &paint::LEVEL_PALETTE,
            _ => &paint::PALETTE,
        };
        // Identity palette with one entry per slot of the selected
        // ramp: raster outputs a slot index in `0..ramp.len()` and the
        // pixel loop maps it through the ramp.
        let identity: Vec<u8> = (0..ramp.len()).map(|i| i as u8).collect();
        let raster = if result.tiles.is_empty() {
            None
        } else {
            Some(tess_core::raster::raster(
                &result.tiles,
                &window,
                w,
                h,
                mode,
                &identity,
            ))
        };
        let (br, bgr, bb) = paint::rgb(paint::BORDER);
        let (ar, ag, ab) = paint::rgb(paint::BACKGROUND);
        let mut pixels = vec![0u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                let idx = raster
                    .as_ref()
                    .map_or(tess_core::raster::GAP, |r| r.at(x, y));
                let (r, g, b) = match idx {
                    tess_core::raster::BORDER => (br, bgr, bb),
                    tess_core::raster::GAP => (ar, ag, ab),
                    i => paint::rgb(ramp[i as usize]),
                };
                let o = ((y * w + x) as usize) * 4;
                pixels[o] = r;
                pixels[o + 1] = g;
                pixels[o + 2] = b;
                pixels[o + 3] = 255;
            }
        }
        let handle = ctx.load_texture(
            key.clone(),
            egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &pixels),
            egui::TextureOptions::LINEAR,
        );
        self.card_textures.insert(key, handle.clone());
        handle
    }
}

impl GuiApp {
    /// One physical zoom step on the Render canvas (`dir` +1 in, −1
    /// out, factor [`RENDER_ZOOM_STEP`] — the TUI's convention),
    /// anchored at the last canvas pass's center so the view stays
    /// put while zooming. Before the first canvas pass there is no
    /// anchor and the viewport center holds.
    fn zoom_by_step(&mut self, dir: i32) {
        let factor = if dir >= 0 {
            RENDER_ZOOM_STEP
        } else {
            1.0 / RENDER_ZOOM_STEP
        };
        let anchor = self.canvas_rect.map(|rect| {
            let c = rect.center();
            self.app.viewport.screen_to_world(
                (c.x - rect.min.x) as f64,
                (c.y - rect.min.y) as f64,
                rect.width() as f64,
                rect.height() as f64,
            )
        });
        self.app.zoom(factor, anchor);
    }

    /// Reset the zoom to the construction baseline (the zoom label's
    /// 100%); the center is untouched (⟲ reset re-centers).
    fn zoom_reset(&mut self) {
        if self.zoom_default > 0.0 {
            self.app.viewport.zoom = self.zoom_default;
        }
    }

    /// The zoom label shown between the −/+ buttons — the current zoom
    /// as a percentage of the construction baseline. Doubles as the
    /// reset button.
    fn zoom_label(&self) -> String {
        if self.zoom_default > 0.0 {
            let pct = (self.app.viewport.zoom / self.zoom_default * 100.0)
                .round()
                .max(1.0);
            format!("{pct:.0}%")
        } else {
            "zoom".to_string()
        }
    }

    /// The canvas cache key for `w` x `h`: the tiling id, the parameter
    /// fingerprint, the viewport (center + zoom) and the canvas size —
    /// everything that determines the generated geometry. Callers pass
    /// the current viewport size so a window resize is a distinct key.
    /// Color mode is deliberately absent (see [`RenderKey`]).
    fn cache_key(&self, w: u32, h: u32) -> RenderKey {
        let v = &self.app.viewport;
        RenderKey {
            id: self.app.selected_id().to_string(),
            params: self.params_fingerprint(),
            center_x: v.center.x,
            center_y: v.center.y,
            zoom: v.zoom,
            w,
            h,
        }
    }

    /// Whether the cached canvas result is stale for `key` (never seen,
    /// or any component differs). A cheap field-wise comparison — the
    /// whole point of the cache is that this, not the generator, decides
    /// whether a frame regenerates.
    fn should_regen(&self, key: &RenderKey) -> bool {
        self.render_cache.as_ref().map(|c| &c.key) != Some(key)
    }

    fn draw_canvas(&mut self, ctx: &egui::Context) {
        let (painter, response, rect) = egui::CentralPanel::default()
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let resp = ui.interact(rect, ui.id().with("canvas"), egui::Sense::drag());
                (ui.painter_at(rect), resp, rect)
            })
            .inner;

        // egui's zoom_delta() is 1.0 when there is no zoom this frame
        // (0.0 is NOT the neutral value — that check would run every frame).
        let z = ctx.input(|i| i.zoom_delta());
        if z != 1.0 {
            let anchor = response.hover_pos().map(|p| {
                self.app.viewport.screen_to_world(
                    (p.x - rect.min.x) as f64,
                    (p.y - rect.min.y) as f64,
                    rect.width() as f64,
                    rect.height() as f64,
                )
            });
            self.app.zoom(z as f64, anchor);
        }
        // Physical zoom keys (+/−): same step as the top-strip buttons,
        // anchored at the canvas center (the wheel anchors at the
        // cursor; keys have none). Skipped while a widget owns the
        // keyboard — a param DragValue in edit mode may want '-' as a
        // sign.
        if !ctx.wants_keyboard_input() {
            let (zin, zout) = ctx.input(|i| {
                (
                    i.key_pressed(egui::Key::Equals),
                    i.key_pressed(egui::Key::Minus),
                )
            });
            let anchor = Some(self.app.viewport.screen_to_world(
                rect.width() as f64 / 2.0,
                rect.height() as f64 / 2.0,
                rect.width() as f64,
                rect.height() as f64,
            ));
            if zin {
                self.app.zoom(RENDER_ZOOM_STEP, anchor);
            }
            if zout {
                self.app.zoom(1.0 / RENDER_ZOOM_STEP, anchor);
            }
        }
        // Record for the zoom buttons' anchor (before the message
        // early-return below, so they anchor correctly even while the
        // too-many-tiles message shows).
        self.canvas_rect = Some(rect);
        if response.dragged() {
            let d = response.drag_delta();
            self.app.pan(d.x as f64, d.y as f64);
        }

        let w = rect.width().max(1.0) as u32;
        let h = rect.height().max(1.0) as u32;
        // Only regenerate when something that determines the result
        // changed: the tiling, any parameter, the viewport, or the size.
        // The canvas is painted every frame, but frames that do not
        // change any of those reuse the stored result — otherwise every
        // queued X11 event during a slow-frame would kick off another
        // multi-second generation (the OS "not responding" storm).
        let key = self.cache_key(w, h);
        if self.should_regen(&key) {
            let result = self.app.result(w, h);
            self.render_cache_misses += 1;
            self.render_cache = Some(RenderCache { key, result });
        }
        let result = &self.render_cache.as_ref().unwrap().result;

        let (br, bg_, bb) = paint::rgb(paint::BORDER);
        let (ar, ag, ab) = paint::rgb(paint::BACKGROUND);
        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(ar, ag, ab));

        if let Some(msg) = &result.message {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                msg,
                egui::FontId::proportional(16.0),
                egui::Color32::from_rgb(224, 108, 117),
            );
            return;
        }

        let zoom = self.app.viewport.zoom as f32;
        let center = self.app.viewport.center;
        let to_screen = |p: tess_core::Vec2| {
            egui::pos2(
                rect.center().x + (p.x - center.x) as f32 * zoom,
                rect.center().y - (p.y - center.y) as f32 * zoom,
            )
        };

        let border = egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(br, bg_, bb));
        // One ear-clipped triangulation per (proto, vertex-count, layout)
        // triple: the index triples transfer between two copies of a
        // prototile only when their vertex lists are *directly similar
        // with the index correspondence i↔i* — same polygon up to
        // translation, rotation, and uniform scale, starting at the same
        // structural vertex, traversed the same way. The first two key
        // components handle penrose, whose unpaired boundary triangles
        // (3 vertices) share proto 0/1 with 4-vertex rhombi; reusing a
        // 4-vertex triangulation on a 3-vertex mesh yields an
        // out-of-range index (epaint asserts in Mesh::is_valid → crash),
        // and reusing a 3-vertex one on a rhombus leaves it half
        // unfilled (dark background shows through). The layout
        // fingerprint handles everything else: the chair tiling emits
        // four rotation classes that all share proto 0 and 6 vertices,
        // but whose vertex lists start at different structural corners —
        // reusing one class's triangulation on another overfilled the
        // notch (concave reflex vertex) and the fill bled across the
        // tile walls into the neighboring chairs. The fingerprint is
        // invariant under translation/rotation/scale but changes under
        // cyclic shifts and mirrors, so every genuinely distinct vertex
        // layout gets its own triangulation. (egui's `convex_polygon`
        // fans the fill from vertex 0, which overfills concave tiles
        // such as the star-cross 16-gons; the mesh fill + line-loop
        // outline below is concave-safe.)
        let mut tri_cache: std::collections::HashMap<(u8, usize, u64), Vec<[u32; 3]>> =
            std::collections::HashMap::new();
        // Two passes, all fills before all outlines (the same fix SVG
        // export documents at `crates/tess-core/src/svg.rs` L15-31):
        // egui paints shapes in add order, and a `closed_line` stroke
        // (1.0 px) straddles its polygon edge — half a pixel pokes into
        // EVERY neighbor — so an interleaved per-tile pass lets a
        // later-drawn neighbor's fill cover the earlier tile's outline
        // half along the shared edge. For taylor-socolar, whose hexagons
        // are partitioned into 8 convex pieces including thin chord
        // strips (the "straps"), a strip loses its grout border on
        // every side shared with a later-drawn piece and visually merges
        // into the surrounding hexagon. Emitting all outlines after all
        // fills makes the border symmetric and order-independent.
        let mut outline_pts: Vec<Vec<egui::Pos2>> = Vec::with_capacity(result.tiles.len());
        for tile in &result.tiles {
            let pts = &tile.poly.pts;
            if pts.len() < 3 {
                continue;
            }
            let screen: Vec<egui::Pos2> = pts.iter().map(|p| to_screen(*p)).collect();
            let (r, g, b) = paint::rgb(paint::tile_rgb(self.app.color_mode, tile));
            let color = egui::Color32::from_rgb(r, g, b);
            let tris = tri_cache
                .entry((tile.proto, pts.len(), layout_fingerprint(pts)))
                .or_insert_with(|| tile.poly.triangulate());
            let mut mesh = egui::Mesh::with_texture(egui::TextureId::default());
            for p in &screen {
                mesh.colored_vertex(*p, color);
            }
            for t in tris {
                mesh.add_triangle(t[0], t[1], t[2]);
            }
            painter.add(egui::Shape::Mesh(mesh));
            outline_pts.push(screen);
        }
        for screen in &outline_pts {
            painter.add(egui::Shape::closed_line(screen.clone(), border));
        }
    }

    fn draw_params(&mut self, ui: &mut egui::Ui) {
        ui.label("Parameters");
        let specs = self.app.tiling().params();
        if specs.is_empty() {
            ui.label("(none)");
            return;
        }
        for spec in specs {
            let cur = self
                .app
                .param_value(spec.id)
                .unwrap_or_else(|| spec.default_value());
            let next = ui
                .horizontal(|ui| {
                    ui.label(spec.label);
                    match spec.kind {
                        ParamKind::Float { min, max, step, .. } => {
                            let mut f = cur.as_f64().unwrap_or(0.0);
                            let changed = ui
                                .add(egui::DragValue::new(&mut f).range(min..=max).speed(step))
                                .changed();
                            changed.then_some(ParamValue::Float(f))
                        }
                        ParamKind::Angle { min, max, step, .. } => {
                            let mut f = cur.as_f64().unwrap_or(0.0);
                            let changed = ui
                                .add(egui::DragValue::new(&mut f).range(min..=max).speed(step))
                                .changed();
                            changed.then_some(ParamValue::Angle(f))
                        }
                        ParamKind::Int { min, max, step, .. } => {
                            let mut i = cur.as_i64().unwrap_or(0);
                            let changed = ui
                                .add(
                                    egui::DragValue::new(&mut i)
                                        .range(min..=max)
                                        .speed(step as f64),
                                )
                                .changed();
                            changed.then_some(ParamValue::Int(i))
                        }
                        ParamKind::Bool { default } => {
                            let mut b = cur.as_bool().unwrap_or(default);
                            let changed = ui.checkbox(&mut b, "").changed();
                            changed.then_some(ParamValue::Bool(b))
                        }
                    }
                })
                .inner;
            if let Some(v) = next {
                self.app.set_param_value(spec.id, v);
            }
        }
    }
}

// The canvas fills each tile with a mesh whose index triples come from
// a triangulation cached per prototile. Penrose is the only tiling that
// emits 3-vertex tiles: a Robinson triangle whose twin was culled at
// the view boundary (penrose.rs culls children per refinement step).
// Such a triangle shares its proto (0/1) with 4-vertex rhombi, so the
// cache must key on the vertex count as well: reusing a 4-vertex
// triangulation on a 3-vertex mesh yields an out-of-range index (the
// tessellator then panics — the crash-on-resize), and reusing a
// 3-vertex one on a rhombus leaves half of it unfilled (the dark
// sliver that reads as "dark-gray tiles" in mono mode).
//
// The metadata tests lock the side-panel contract: the probe region is a
// tiny box at the origin and must produce tiles for every registered
// tiling, and panel metadata must come from the cache, not from
// view-sized generation.
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::OnceLock;

    use super::layout_fingerprint;

    /// Shared, process-wide browse-tabs fixture: built once per test
    /// binary and reused by every test that needs the browser model.
    /// The tabs are pure (registry + classification, no test state),
    /// so sharing is behavior-identical to each test rebuilding them
    /// (the expensive ~10 s `browse_tabs_with` copy).
    static SHARED_TABS: OnceLock<Vec<tess_core::Tab>> = OnceLock::new();

    fn shared_tabs() -> &'static Vec<tess_core::Tab> {
        SHARED_TABS.get_or_init(|| {
            tess_core::browse_tabs_with(
                &tess_tilings::registry(),
                tess_tilings::classification,
            )
        })
    }

    /// A fresh [`super::GuiApp`] with the shared tabs fixture, mirroring
    /// `GuiApp::new()`'s field initialization exactly (the registry, the
    /// selection, the active tab, and the zoom baseline are all rebuilt
    /// per test, exactly as `new()` does). Private-field access is fine:
    /// these tests are a child module of `ui`.
    fn fresh_gui() -> super::GuiApp {
        fresh_gui_with(tess_tilings::registry())
    }

    /// Variant of [`fresh_gui`] with a caller-supplied registry — used by
    /// the `default_view_center` hook pin, which needs a single-tiling
    /// registry so the override (and not the 169-tiling full registry's
    /// first tiling) is what `App::new` seeds.
    fn fresh_gui_with(registry: tess_core::Registry) -> super::GuiApp {
        let app = tess_core::App::new(registry, 60.0);
        // The zoom label's 100% baseline: the construction zoom.
        let zoom_default = app.viewport.zoom;
        // Rebuild the tabs from the shared fixture: cheap (one
        // `Vec<CardMeta>` clone per tab; `CardMeta` is `Copy` and the
        // `TilingCategory` is `Copy`, but `Tab` itself is not `Clone`).
        let tabs: Vec<tess_core::Tab> = shared_tabs()
            .iter()
            .map(|t| tess_core::Tab {
                category: t.category,
                cards: t.cards.clone(),
            })
            .collect();
        let selected = app.selected_id().to_string();
        let active_tab = tabs
            .iter()
            .position(|t| t.cards.iter().any(|c| c.id == selected))
            .unwrap_or(0);
        let mut tab_selected = vec![None; tabs.len()];
        tab_selected[active_tab] = Some(selected.clone());
        super::GuiApp {
            app,
            mode: super::GuiMode::Browse,
            params_open: true,
            meta: None,
            render_cache: None,
            render_cache_misses: 0,
            tabs,
            active_tab,
            tab_selected,
            search_at_jump: String::new(),
            last_highlighted: (active_tab, selected),
            card_textures: HashMap::new(),
            card_target_w: super::CARD_TARGET_W,
            search: String::new(),
            card_rects: Vec::new(),
            card_grid_col_w: 0.0,
            card_grid_cols: 0,
            card_grid_avail: 0.0,
            canvas_rect: None,
            zoom_default,
            about_open: false,
            preview_mode: None,
            preview_mode_rects: Vec::new(),
        }
    }

    #[test]
    fn gui_app_new_seeds_viewport_from_default_view_center_hook() {
        use tess_tilings::families::quasiperiodic::golden_rhomboid::GoldenRhomboidTiling;
        let registry = tess_core::Registry::new().register(GoldenRhomboidTiling);
        let gui = fresh_gui_with(registry);
        assert_eq!(gui.app.selected_id(), "golden-rhomboid");
        let c = gui.app.viewport.center;
        assert!(
            (c.x - 5.14).abs() < 1e-12 && (c.y - 3.67).abs() < 1e-12,
            "GUI GuiApp::new must seed the hook, got {:?}",
            c
        );
    }

    #[test]
    fn penrose_cache_valid_for_all_tiles() {
        let mut app = tess_core::App::new(tess_tilings::registry(), 60.0);
        app.select_id("penrose");
        let result = app.result(800, 600);
        assert!(result.message.is_none(), "{:?}", result.message);
        let mut cache: HashMap<(u8, usize, u64), Vec<[u32; 3]>> = HashMap::new();
        let mut n3 = 0usize;
        for tile in &result.tiles {
            let n = tile.poly.pts.len();
            if n == 3 {
                n3 += 1;
            }
            let tris = cache
                .entry((tile.proto, n, layout_fingerprint(&tile.poly.pts)))
                .or_insert_with(|| tile.poly.triangulate());
            for [a, b, c] in tris {
                assert!(
                    (*a as usize) < n && (*b as usize) < n && (*c as usize) < n,
                    "proto {} tile with {n} vertices got out-of-range cached index",
                    tile.proto
                );
            }
        }
        assert!(
            n3 > 0,
            "expected 3-vertex boundary triangles in the default penrose view, got 0"
        );
    }

    #[test]
    fn metadata_probe_covers_every_tiling() {
        let mut app = tess_core::App::new(tess_tilings::registry(), 60.0);
        let probe = tess_core::BBox::new(
            tess_core::Vec2::new(-0.5, -0.5),
            tess_core::Vec2::new(0.5, 0.5),
        );
        let ids: Vec<&'static str> = app.registry().iter().map(|t| t.id()).collect();
        for id in ids {
            app.select_id(id);
            let r = app.result_region(probe);
            assert!(
                !r.tiles.is_empty(),
                "{id}: metadata probe region produced no tiles"
            );
        }
    }

    #[test]
    fn metadata_cache_tracks_tiling_switch() {
        let mut gui = fresh_gui();
        let before = gui.metadata().symmetry.group;
        gui.app.select_id("penrose");
        let after = gui.metadata().symmetry.group;
        assert_ne!(before, after, "metadata did not refresh on tiling switch");
    }

    /// The canvas fills each tile from a triangulation cached per
    /// (proto, vertex-count) — the exact cache `draw_canvas` builds.
    /// Simulate it for EVERY registered tiling at the default view and
    /// assert the cached triangulation is complete (n − 2 triangles) and
    /// valid (indices in range). This guards the whole class of
    /// "some tiles render unfilled" bugs: a cache entry computed from a
    /// polygon of a different shape, or a broken triangulation, shows up
    /// here as a wrong triangle count or an out-of-range index.
    /// A cached triangulation is only valid for a tile whose vertex list is
/// directly similar (i↔i) to the tile that seeded the cache entry. The
/// failure mode this guards against — the chair bug — is invisible to
/// index-bounds and triangle-count checks: a fan triangulation computed
/// for one orientation, applied to a rotated copy whose vertex 0 sits
/// at a different structural corner, still yields n−2 in-range
/// triangles but *overfills the concave notch*, and the fill bleeds
/// across the tile walls. Overfill shows up as Σ|triangle areas| >
/// polygon area, so every cache HIT is area-checked here; fresh
/// entries are triangulated from the tile itself and covered by the
/// tess-core triangulate tests.
#[test]
    fn tri_cache_valid_for_all_tilings() {
        let mut app = tess_core::App::new(tess_tilings::registry(), 60.0);
        let ids: Vec<&'static str> = app.registry().iter().map(|t| t.id()).collect();
        for id in ids {
            app.select_id(id);
            let result = app.result(800, 600);
            assert!(result.message.is_none(), "{id}: {:?}", result.message);
            assert!(!result.tiles.is_empty(), "{id}: no tiles at default view");
            let mut cache: HashMap<(u8, usize, u64), Vec<[u32; 3]>> = HashMap::new();
            for tile in &result.tiles {
                let n = tile.poly.pts.len();
                assert!(n >= 3, "{id}: {n}-vertex tile");
                let (tris, fresh) = match cache
                    .entry((tile.proto, n, layout_fingerprint(&tile.poly.pts)))
                {
                    std::collections::hash_map::Entry::Occupied(e) => {
                        (e.into_mut(), false)
                    }
                    std::collections::hash_map::Entry::Vacant(e) => {
                        (e.insert(tile.poly.triangulate()), true)
                    }
                };
                assert_eq!(tris.len(), n - 2, "{id}: {n}-gon triangulation size");
                for [a, b, c] in tris.iter() {
                    assert!(
                        (*a as usize) < n && (*b as usize) < n && (*c as usize) < n,
                        "{id}: out-of-range index for proto {} {n}-gon",
                        tile.proto
                    );
                }
                if !fresh {
                    // Cached reuse: the seeded triangulation must still
                    // cover exactly this tile's polygon.
                    let pa = tile.poly.area();
                    let ta: f64 = tris
                        .iter()
                        .map(|[a, b, c]| {
                            let p = &tile.poly.pts[*a as usize];
                            let q = &tile.poly.pts[*b as usize];
                            let r = &tile.poly.pts[*c as usize];
                            ((q.x - p.x) * (r.y - p.y)
                                - (q.y - p.y) * (r.x - p.x))
                                .abs()
                                * 0.5
                        })
                        .sum();
                    assert!(
                        (ta - pa).abs() <= 1e-6 * pa,
                        "{id}: cached triangulation for proto {} {n}-gon \
                         covers {ta} != polygon {pa} (misaligned vertex \
                         lists — fill would bleed across tile walls)",
                        tile.proto
                    );
                }
            }
        }
    }

    /// The layout fingerprint must distinguish exactly the cases where
    /// sharing a triangulation is valid: rotation/translation copies
    /// hash equal (index correspondence survives), while cyclic shifts
    /// and mirrors hash differently (they do not).
    #[test]
    fn layout_fingerprint_matches_only_aligned_copies() {
        let v2 = |x: f64, y: f64| tess_core::Vec2::new(x, y);
        // A concave L-shape (the chair prototile).
        let a = [
            v2(0.0, 0.0),
            v2(2.0, 0.0),
            v2(2.0, 1.0),
            v2(1.0, 1.0),
            v2(1.0, 2.0),
            v2(0.0, 2.0),
        ];
        // 90° rotation + translation: every vertex maps i↔i — same key.
        let rot: Vec<tess_core::Vec2> =
            a.iter().map(|p| v2(-p.y + 5.0, p.x + 3.0)).collect();
        assert_eq!(
            layout_fingerprint(&a),
            layout_fingerprint(&rot),
            "rotation/translation copies must share a cache entry"
        );
        // Uniform scale: same key.
        let scaled: Vec<tess_core::Vec2> = a.iter().map(|p| v2(p.x * 3.0, p.y * 3.0)).collect();
        assert_eq!(layout_fingerprint(&a), layout_fingerprint(&scaled));
        // Cyclic shift of the same polygon: vertex 0 sits at a different
        // structural corner — must NOT share (this is exactly the chair
        // failure mode).
        let mut shifted = a[1..].to_vec();
        shifted.push(a[0]);
        assert_ne!(
            layout_fingerprint(&a),
            layout_fingerprint(&shifted),
            "cyclically shifted lists must get their own triangulation"
        );
        // Mirror: conjugate ratios — must not share.
        let mirrored: Vec<tess_core::Vec2> = a.iter().map(|p| v2(p.x, -p.y)).collect();
        assert_ne!(layout_fingerprint(&a), layout_fingerprint(&mirrored));
    }

    // ---- Tab/card browser pure logic (UI-2) -------------------------

    fn card_fix(
        id: &'static str,
        category: tess_core::TilingCategory,
        group: Option<tess_core::WallpaperGroup>,
        lattice: Option<tess_core::LatticeKind>,
    ) -> tess_core::CardMeta {
        tess_core::CardMeta {
            id,
            name: id,
            description: "",
            category,
            group,
            lattice,
            notes: None,
            params: 0,
            class: tess_core::TilingClass::EMPTY,
        }
    }

    /// Search matches on name, id, or description, case-insensitively;
    /// an empty (whitespace) query matches everything.
    #[test]
    fn search_matches_fields_and_empty_query() {
        let tabs = shared_tabs();
        let real = tabs
            .iter()
            .find_map(|t| t.cards.iter().find(|c| c.id == "penrose"))
            .expect("penrose card");
        assert!(super::search_matches(real, ""), "empty query must match");
        assert!(super::search_matches(real, "   "), "whitespace query must match");
        let name = real.name.to_lowercase();
        assert!(super::search_matches(real, &name), "name match");
        assert!(
            super::search_matches(real, real.name.to_uppercase().as_str()),
            "case-insensitive name match"
        );
        assert!(super::search_matches(real, "PENROSE"), "id match (case-insensitive)");
        // A description fragment (char-safe, not a byte slice).
        let desc: String = real.description.chars().take(5).collect();
        assert!(!desc.is_empty(), "penrose must have a description");
        assert!(super::search_matches(real, &desc), "description match");
        assert!(!super::search_matches(real, "zzz-no-match"), "no field contains it");
        let other = card_fix("other", tess_core::TilingCategory::Uniform, None, None);
        assert!(!super::search_matches(&other, "penrose"));
    }

    /// `tab_match_count` counts exactly the matching cards in a tab.
    #[test]
    fn tab_match_count_counts_only_matches() {
        let cards = vec![
            card_fix("a", tess_core::TilingCategory::Uniform, None, None),
            card_fix(
                "b",
                tess_core::TilingCategory::Uniform,
                None,
                None,
            ),
        ];
        let mut tab = tess_core::Tab {
            category: tess_core::TilingCategory::Uniform,
            cards,
        };
        tab.cards[1].name = "needle";
        assert_eq!(super::tab_match_count(&tab, "needle"), 1);
        assert_eq!(super::tab_match_count(&tab, ""), 2, "empty query: all");
        assert_eq!(super::tab_match_count(&tab, "zzz"), 0);
    }

    /// The search jump target is the FIRST tab with a match, in tab
    /// order; empty query or no match anywhere is `None`.
    #[test]
    fn first_matching_tab_is_the_first_tab_with_a_match() {
        let mk = |cat: tess_core::TilingCategory, id: &'static str| tess_core::Tab {
            category: cat,
            cards: vec![card_fix(id, cat, None, None)],
        };
        let mut tabs = vec![
            mk(tess_core::TilingCategory::Uniform, "alpha"),
            mk(tess_core::TilingCategory::Laves, "beta"),
            mk(
                tess_core::TilingCategory::Quasiperiodic,
                "gamma",
            ),
        ];
        // Give tabs 1 and 2 a shared name fragment that tab 0 lacks.
        tabs[1].cards[0].name = "betta-hunter";
        tabs[2].cards[0].name = "betta-fighter";
        assert_eq!(
            super::first_matching_tab(&tabs, "betta"),
            Some(1),
            "first matching tab wins"
        );
        assert_eq!(super::first_matching_tab(&tabs, ""), None);
        assert_eq!(super::first_matching_tab(&tabs, "zzz"), None);
    }

    /// Sectioning is by maximal runs of equal `section_key` in
    /// first-appearance order (non-contiguous repeats get their own
    /// runs), preserving card order.
    #[test]
    fn sectionize_runs_preserve_order_and_split_noncontiguous_keys() {
        let wp = tess_core::TilingCategory::Wallpaper;
        let p1 = tess_core::WallpaperGroup::P1;
        let cmm = tess_core::WallpaperGroup::Cmm;
        let cards = vec![
            card_fix("o1", wp, Some(p1), Some(tess_core::LatticeKind::Oblique)),
            card_fix("r1", wp, Some(tess_core::WallpaperGroup::Pm), Some(tess_core::LatticeKind::Rectangular)),
            card_fix("c1", wp, Some(cmm), Some(tess_core::LatticeKind::CenteredRectangular)),
            card_fix("r2", wp, Some(tess_core::WallpaperGroup::Pg), Some(tess_core::LatticeKind::Rectangular)),
            card_fix("s1", wp, Some(tess_core::WallpaperGroup::P4m), Some(tess_core::LatticeKind::Square)),
            card_fix("h1", wp, Some(tess_core::WallpaperGroup::P6m), Some(tess_core::LatticeKind::Hexagonal)),
            card_fix(
                "n1",
                tess_core::TilingCategory::Uniform,
                None,
                None,
            ),
        ];
        let sections = super::sectionize(wp, &cards);
        let headers: Vec<&str> = sections.iter().map(|s| s.header).collect();
        assert_eq!(
            headers,
            vec!["oblique", "rectangular", "centered", "rectangular", "square", "hexagonal", ""]
        );
        // Every card exactly once, in order.
        let flat: Vec<&str> = sections
            .iter()
            .flat_map(|s| s.cards.iter())
            .map(|c| c.id)
            .collect();
        assert_eq!(flat, vec!["o1", "r1", "c1", "r2", "s1", "h1", "n1"]);
    }

    /// On the real Wallpaper tab the sections are the crystal-system
    /// runs in `browse_tabs` order; the first card of each section sets
    /// the header, and every card is covered exactly once.
    #[test]
    fn sectionize_real_wallpaper_tab() {
        let tabs = shared_tabs();
        let wp = tabs
            .iter()
            .find(|t| t.category == tess_core::TilingCategory::Wallpaper)
            .expect("wallpaper tab");
        assert!(!wp.cards.is_empty(), "wallpaper tab must not be empty");
        let sections = super::sectionize(tess_core::TilingCategory::Wallpaper, &wp.cards);
        let mut covered = 0usize;
        for s in &sections {
            assert!(!s.cards.is_empty());
            if !s.header.is_empty() {
                assert_eq!(
                    s.header,
                    tess_core::section_key(tess_core::TilingCategory::Wallpaper, &s.cards[0]),
                    "header must come from the section's first card"
                );
            }
            covered += s.cards.len();
        }
        assert_eq!(covered, wp.cards.len());
        // Crystal-system order: no section may re-appear before a later
        // crystal system (oblique before rectangular before centered
        // before square before hexagonal).
        let rank = |h: &str| match h {
            "" => 9,
            "oblique" => 0,
            "rectangular" => 1,
            "centered" => 2,
            "square" => 3,
            "hexagonal" => 4,
            _ => panic!("unexpected header {h:?}"),
        };
        let mut max_rank = 0u8;
        for s in &sections {
            let r = rank(s.header);
            assert!(
                r >= max_rank,
                "section order violated: {:?}",
                sections.iter().map(|x| x.header).collect::<Vec<_>>()
            );
            max_rank = r;
        }
    }

    /// Entering a tab restores the remembered card if it is still in
    /// the tab, else the first card.
    #[test]
    fn entry_selection_restores_remembered_or_first() {
        let cat = tess_core::TilingCategory::Uniform;
        let mut tab = tess_core::Tab {
            category: cat,
            cards: vec![
                card_fix("first", cat, None, None),
                card_fix("second", cat, None, None),
            ],
        };
        assert_eq!(
            super::entry_selection(&tab, Some("second")),
            Some("second"),
            "remembered card in tab is restored"
        );
        assert_eq!(
            super::entry_selection(&tab, Some("missing")),
            Some("first"),
            "remembered card gone: fall back to first"
        );
        assert_eq!(super::entry_selection(&tab, None), Some("first"));
        // A tab with no cards at all.
        tab.cards.clear();
        assert_eq!(super::entry_selection(&tab, None), None);
    }

    /// The texture cache key distinguishes (id, params, mode, size).
    /// The mode component (the preview band's recolor) must not let a
    /// mode change reuse a stale texture, and the size stays the last
    /// `|WxH` segment the col-pruning relies on.
    #[test]
    fn card_key_distinguishes_id_params_mode_size() {
        use tess_core::view::ColorMode;
        assert_eq!(
            super::card_key("chair", "scale=1", ColorMode::Proto, 120, 90),
            "chair|scale=1|proto|120x90"
        );
        assert_ne!(super::card_key("chair", "scale=1", ColorMode::Proto, 120, 90), super::card_key("chair", "scale=2", ColorMode::Proto, 120, 90));
        assert_ne!(super::card_key("chair", "scale=1", ColorMode::Proto, 120, 90), super::card_key("chair", "scale=1", ColorMode::Proto, 121, 90));
        assert_ne!(super::card_key("chair", "scale=1", ColorMode::Proto, 120, 90), super::card_key("other", "scale=1", ColorMode::Proto, 120, 90));
        // The new mode axis: same id/params/size, different mode → different key.
        assert_ne!(super::card_key("chair", "scale=1", ColorMode::Proto, 120, 90), super::card_key("chair", "scale=1", ColorMode::Shape, 120, 90));
        // The `|WxH` suffix contract the prune relies on survives the new segment.
        assert!(super::card_key("chair", "scale=1", ColorMode::Orientation, 120, 90).ends_with("|120x90"));
    }

    /// The params component of the cache key is the preview's actual
    /// values in declaration order — pinned so the preview cache key is
    /// stable across runs (the determinism contract).
    #[test]
    fn preview_params_str_pinned() {
        let registry = tess_tilings::registry();
        let star = registry.get("star-cross").expect("star-cross registered");
        let specs = star.params();
        assert_eq!(specs.len(), 3);
        assert_eq!(
            specs.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(),
            vec!["scale", "rotation", "star_radius"]
        );
        let got = super::preview_params_str(specs, &tess_core::params::ParamSet::default_for(specs));
        assert_eq!(got, "scale=1,rotation=0°,star_radius=0.50");
        assert_eq!(
            got,
            super::preview_params_str(specs, &tess_core::params::ParamSet::default_for(specs)),
            "deterministic"
        );
        // A parameterless declaration yields the empty params-string.
        assert_eq!(
            super::preview_params_str(&[], &tess_core::params::ParamSet::EMPTY),
            ""
        );
    }

    /// Truncation is char-safe (descriptions hold non-ASCII, e.g. the
    /// en dash) and appends an ellipsis only when it cuts.
    #[test]
    fn truncate_text_char_safe() {
        assert_eq!(super::truncate_text("abc", 5), "abc");
        assert_eq!(super::truncate_text("abcd", 4), "abcd");
        assert_eq!(super::truncate_text("abcdef", 4), "abc…");
        assert_eq!(super::truncate_text("abcdef", 1), "…");
        // The en dash in the Ammann–Beenker name: no byte-slice panic.
        let s = "Ammann–Beenker tiling";
        assert_eq!(super::truncate_text(s, 6), "Amman…");
        assert_eq!(super::truncate_text(s, s.chars().count()), s);
    }

    // ---- Single-pane browser pure logic (UI-3) ----------------------

    /// `columns_for_width` fits whole target-width cards with no
    /// upper bound (the target width is the squash floor, not a
    /// column cap), and degrades to a single column on degenerate
    /// widths.
    #[test]
    fn columns_for_width_is_fully_dynamic() {
        assert_eq!(super::columns_for_width(240.0, 240.0), 1, "exactly one target width");
        assert_eq!(super::columns_for_width(239.0, 240.0), 1, "narrow → 1");
        assert_eq!(super::columns_for_width(480.0, 240.0), 2, "exactly 2");
        assert_eq!(super::columns_for_width(479.0, 240.0), 1);
        assert_eq!(super::columns_for_width(640.0, 240.0), 2, "mid-size unchanged");
        assert_eq!(super::columns_for_width(1200.0, 240.0), 5, "previously clamped to 4");
        assert_eq!(super::columns_for_width(4000.0, 240.0), 16, "ultra-wide");
        assert_eq!(super::columns_for_width(100_000.0, 240.0), 416, "no upper clamp");
        assert_eq!(super::columns_for_width(0.0, 240.0), 1, "zero width");
        assert_eq!(super::columns_for_width(f32::NAN, 240.0), 1, "non-finite width");
        assert_eq!(super::columns_for_width(1000.0, 0.0), 1, "non-positive card width");
    }

    /// `walk_index` wraps at both ends, handles empty tabs, and
    /// tolerates a missing or out-of-range cursor.
    #[test]
    fn walk_index_wraps_and_degrades() {
        assert_eq!(super::walk_index(5, Some(0), -1), Some(4), "prev wraps to last");
        assert_eq!(super::walk_index(5, Some(4), 1), Some(0), "next wraps to first");
        assert_eq!(super::walk_index(5, Some(2), 1), Some(3));
        assert_eq!(super::walk_index(5, Some(2), -1), Some(1));
        assert_eq!(super::walk_index(1, Some(0), 1), Some(0), "single card: stays");
        assert_eq!(super::walk_index(0, Some(0), 1), None, "empty tab");
        assert_eq!(super::walk_index(5, None, 1), Some(1), "missing cursor starts at 0");
        assert_eq!(super::walk_index(5, None, -1), Some(4));
        assert_eq!(
            super::walk_index(5, Some(9), 1),
            Some(0),
            "out-of-range cursor clamps to last, then wraps"
        );
    }

    // ---- Browse grid width/height math (UI-4) -----------------------

    /// The width math fills the window exactly: with
    /// `col_w = card_col_w(avail, cols, GRID_SPX)` each card's outer
    /// (grid-cell) width is exactly `col_w`, so
    /// `cols·col_w + (cols−1)·GRID_SPX == avail` — this is what keeps
    /// the right-most column from being clipped.
    #[test]
    fn card_grid_fills_the_width_exactly() {
        for &avail in &[800.0, 960.0, 1280.0, 1920.0, 100_000.0] {
            let cols = super::columns_for_width(avail, super::CARD_TARGET_W);
            // Fully dynamic: the count is exactly the floor of
            // `avail / CARD_TARGET_W` (≥1), not a clamped range.
            assert_eq!(cols, (avail / super::CARD_TARGET_W).floor() as usize);
            let col_w = super::card_col_w(avail, cols, super::GRID_SPX);
            let cell_w = super::card_outer_w(
                super::card_inner_w(col_w, super::CARD_INNER_MARGIN),
                super::CARD_INNER_MARGIN,
            );
            let total = cols as f32 * cell_w + (cols - 1) as f32 * super::GRID_SPX;
            assert!(
                (total - avail).abs() < 1e-3,
                "avail={avail} cols={cols}: total {total} must equal avail"
            );
        }
    }

    /// `card_inner_w` and `card_outer_w` are inverses (the inner
    /// width clamps at zero when the column is narrower than the two
    /// margins).
    #[test]
    fn card_width_inner_outer_round_trip() {
        for &col_w in &[20.0, 100.0, 240.0, 230.5, 1000.0] {
            let w = super::card_inner_w(col_w, super::CARD_INNER_MARGIN);
            assert!(w >= 0.0);
            assert!(
                (super::card_outer_w(w, super::CARD_INNER_MARGIN) - col_w).abs() < 1e-6,
                "col_w={col_w}"
            );
        }
        // A column narrower than the two margins clamps the inner
        // width to 0; the frame keeps its margins.
        assert_eq!(super::card_inner_w(4.0, super::CARD_INNER_MARGIN), 0.0);
        assert_eq!(
            super::card_outer_w(0.0, super::CARD_INNER_MARGIN),
            2.0 * super::CARD_INNER_MARGIN
        );
    }

    /// The preview image scales linearly with the card target width
    /// (default → the historical 120×90) and keeps the 4:3 preview
    /// aspect at every zoom level.
    #[test]
    fn card_img_size_scales_with_target() {
        assert_eq!(super::card_img_size(super::CARD_TARGET_W), (120, 90));
        assert_eq!(super::card_img_size(super::CARD_W_MAX), (240, 180));
        assert_eq!(super::card_img_size(super::CARD_W_MIN), (75, 56));
        assert_eq!(super::card_img_size(192.0), (96, 72));
        for &t in &[
            super::CARD_W_MIN,
            192.0,
            super::CARD_TARGET_W,
            300.0,
            super::CARD_W_MAX,
        ] {
            let (w, h) = super::card_img_size(t);
            assert_eq!(h, w * super::CARD_IMG_H / super::CARD_IMG_W, "4:3 at {t}");
        }
    }

    /// The card-size control clamps to [`super::CARD_W_MIN,
    /// super::CARD_W_MAX`] and, on every actual change, prunes preview
    /// textures of other sizes (the cache is keyed by exact pixel
    /// size, so stale-size textures are dead weight under continuous
    /// wheel zoom). Pruning is exercised through a real headless pass:
    /// textures are generated at the default size, resizing drops
    /// them, and the next pass regenerates at the new size.
    #[test]
    fn set_card_target_w_clamps_and_prunes_textures() {
        let ctx = egui::Context::default();
        let mut gui = fresh_gui();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1200.0, 900.0));
        let pass = |gui: &mut super::GuiApp| {
            let input = egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("browser")
                        .show(ui, |ui| gui.draw_card_grid(ctx, ui));
                });
            });
        };
        for _ in 0..2 {
            pass(&mut gui);
        }
        assert!(
            gui.card_textures.keys().any(|k| k.ends_with("|120x90")),
            "default pass generates 120x90 textures"
        );
        // Resize: every stale-size texture is pruned immediately (the
        // new size regenerates on the next draw pass).
        gui.set_card_target_w(360.0);
        assert_eq!(gui.card_target_w, 360.0);
        assert!(
            gui.card_textures.is_empty(),
            "120x90 textures pruned on resize"
        );
        pass(&mut gui);
        assert!(
            gui.card_textures.keys().all(|k| k.ends_with("|180x135")),
            "zoomed pass regenerates at the new size"
        );
        // Clamps: way past the max lands exactly on it, way below the
        // min on it.
        gui.set_card_target_w(10_000.0);
        assert_eq!(gui.card_target_w, super::CARD_W_MAX);
        gui.set_card_target_w(0.0);
        assert_eq!(gui.card_target_w, super::CARD_W_MIN);
    }

    /// PRUNE-ON-TARGET-CHANGE (phase-2a recon hardening): while target
    /// A (the default 120x90 preview) is active, textures are generated
    /// and cached; switching the target width prunes every stale-size
    /// entry at the moment of the change (the `retain` in
    /// `set_card_target_w_impl`) — the next generation under B never
    /// reuses A's textures. Distinct from the headless-pass prune pin:
    /// this one drives the production cache directly and pins the
    /// synchronous (not lazy, not next-pass) prune semantics.
    #[test]
    fn card_texture_cache_pruned_on_target_change() {
        let ctx = egui::Context::default();
        let mut gui = fresh_gui();
        let card = card_fix(
            "penrose",
            tess_core::TilingCategory::Uniform,
            None,
            None,
        );
        // While target A is active: generate (the default 120x90 size).
        let _a_handle = gui.card_texture(&ctx, &card, 120, 90);
        assert_eq!(
            gui.card_textures.len(), 1,
            "A's texture generated and cached"
        );
        assert!(
            gui.card_textures.keys().all(|k| k.ends_with("|120x90")),
            "A-size textures under the default target"
        );
        // Switch target A → B (360.0 → 180x135): A's entries are dropped
        // synchronously, before any regeneration pass runs.
        gui.set_card_target_w(360.0);
        assert_eq!(
            gui.card_textures.len(), 0,
            "A's entries pruned at the target-change moment"
        );
        // Generate under target B: only B-size entries exist; A's stay gone.
        let _b_handle = gui.card_texture(&ctx, &card, 180, 135);
        assert_eq!(gui.card_textures.len(), 1);
        assert!(
            gui.card_textures.keys().all(|k| k.ends_with("|180x135")),
            "B's textures regenerated at the new size"
        );
    }

    /// KEY SENSITIVITY (phase-2a recon hardening): the card-texture cache
    /// is keyed by (id, params_str, mode, size) — `card_key`. Same id + same
    /// params + same size through the real production path = a HIT (no
    /// regeneration; the same texture handle is reused). The same id at
    /// the same size with a different params_str keys to a DIFFERENT
    /// entry, so the cache never silently reuses a stale texture across
    /// a params change. The production card path always renders at the
    /// tiling's default params (the TUI card semantics), so the
    /// params_str-change side is pinned via the production `card_key`
    /// plus the real map entry count — no production counter is needed.
    #[test]
    fn card_texture_cache_key_sensitive_to_params_str() {
        let ctx = egui::Context::default();
        let mut gui = fresh_gui();
        let card = card_fix(
            "star-cross",
            tess_core::TilingCategory::Uniform,
            None,
            None,
        );
        // Same id + same params + same size → HIT.
        let h1 = gui.card_texture(&ctx, &card, 120, 90);
        let entries = gui.card_textures.len();
        let h2 = gui.card_texture(&ctx, &card, 120, 90);
        assert_eq!(
            gui.card_textures.len(), entries,
            "same key must hit (no regeneration)"
        );
        assert!(h1 == h2, "the hit reuses the same texture handle");
        // Same id + same size + different params_str → different key.
        let specs = gui
            .app
            .registry()
            .get(card.id)
            .expect("star-cross is registered")
            .params();
        let default_str = super::preview_params_str(
            specs,
            &tess_core::params::ParamSet::default_for(specs),
        );
        let changed_str = "scale=2,rotation=0°,star_radius=0.50";
        assert_ne!(default_str, changed_str);
        let k_default = super::card_key(card.id, &default_str, tess_core::view::ColorMode::Proto, 120, 90);
        let k_changed = super::card_key(card.id, &changed_str, tess_core::view::ColorMode::Proto, 120, 90);
        assert_ne!(
            k_default, k_changed,
            "params_str must be part of the cache key — otherwise a params change reuses a stale texture"
        );
    }

    /// Per-size keying (phase-2a recon hardening): the same card id at
    /// two sizes is two distinct map entries — the cache is keyed by the
    /// exact pixel size, so one size never silently reuses another's
    /// texture.
    #[test]
    fn card_texture_cache_keyed_per_size() {
        let ctx = egui::Context::default();
        let mut gui = fresh_gui();
        let card = card_fix(
            "penrose",
            tess_core::TilingCategory::Uniform,
            None,
            None,
        );
        let _h120 = gui.card_texture(&ctx, &card, 120, 90);
        let _h180 = gui.card_texture(&ctx, &card, 180, 135);
        assert_eq!(
            gui.card_textures.len(), 2,
            "one entry per exact (id, params, size)"
        );
        assert!(gui.card_textures.keys().any(|k| k.ends_with("|120x90")));
        assert!(gui.card_textures.keys().any(|k| k.ends_with("|180x135")));
    }

    /// Headless full-registry card sweep — the trilogy-closing
    /// generation test: every built-in tiling, driven through the
    /// production `card_texture` path at the default card size
    /// (120x90): probe → shrink-retry window → raster → texture
    /// handle. Generation must not panic, and exactly one texture
    /// handle must be cached per registry id. The count assert is
    /// pinned to 170, so a registry change fails loudly right here.
    #[test]
    fn card_generation_sweep_covers_full_registry() {
        let registry = tess_tilings::registry();
        let ids: Vec<&'static str> = registry.iter().map(|t| t.id()).collect();
        assert_eq!(
            ids.len(),
            170,
            "sweep must enumerate the full registry — {} ids, expected 170",
            ids.len()
        );

        // The real cards from the canonical browse model — the same
        // source the production card grid passes to the card path.
        let tabs =
            tess_core::browse_tabs_with(&registry, tess_tilings::classification);
        let mut cards: HashMap<&'static str, tess_core::CardMeta> = HashMap::new();
        for tab in &tabs {
            for card in &tab.cards {
                cards.insert(card.id, *card);
            }
        }

        let ctx = egui::Context::default();
        let mut gui = super::GuiApp::new();
        let (w, h) = (120, 90);
        let mut family_times: HashMap<String, (usize, std::time::Duration)> = HashMap::new();
        let mut slowest: (&'static str, std::time::Duration) = ("", std::time::Duration::ZERO);
        let t0 = std::time::Instant::now();
        for id in &ids {
            let card = cards
                .get(id)
                .unwrap_or_else(|| panic!("{id}: no card in browse tabs"));
            let s = std::time::Instant::now();
            let _handle = gui.card_texture(&ctx, card, w, h);
            let dt = s.elapsed();
            if dt > slowest.1 {
                slowest = (*id, dt);
            }
            let cat = registry
                .get(id)
                .map(|t| format!("{:?}", t.category()))
                .unwrap_or_else(|| "?".into());
            let entry = family_times.entry(cat).or_default();
            entry.0 += 1;
            entry.1 += dt;
        }
        let elapsed = t0.elapsed();

        // (a) reaching this point means generation completed for every
        // id — the production path returned a handle without panicking.
        // (b) the cache now holds exactly one keyed entry per id, and
        // every entry is at the fixed card size.
        assert_eq!(
            gui.card_textures.len(),
            ids.len(),
            "one texture handle per registry id"
        );
        assert!(
            gui.card_textures
                .keys()
                .all(|k| k.ends_with("|120x90")),
            "all textures at the fixed card size"
        );
        eprintln!(
            "card_generation_sweep: {} ids in {:?} | slowest: {} {:?} | per-family: {}",
            ids.len(),
            elapsed,
            slowest.0,
            slowest.1,
            family_times
                .iter()
                .map(|(cat, (n, sum))| format!("{cat}={n}x{:?}", sum))
                .collect::<Vec<_>>()
                .join(", ")
        );

        // Semantic spot-assert (kept inside the sweep so the new
        // test count is exactly +1, not +2): at the 120x90 card
        // scale, "square" generates exactly the pinned tile count
        // through the same probe window the production card path
        // derives, proving the sweep's "generation succeeds" is not
        // vacuously green.
        let square = registry.get("square").expect("square is registered");
        let params = tess_core::params::ParamSet::default_for(square.params());
        let probe = square.generate(
            &params,
            &tess_core::Query {
                region: tess_core::BBox::new(
                    tess_core::Vec2::new(-10.0, -10.0),
                    tess_core::Vec2::new(10.0, 10.0),
                ),
            },
        );
        let window = tess_core::app::preview_window(
            probe.periodic,
            tess_core::app::mean_tile_side(&probe.tiles),
            120.0 / 90.0,
        );
        let result = square.generate(&params, &tess_core::Query { region: window });
        assert!(
            result.message.is_none(),
            "square should not refuse at card scale: {:?}",
            result.message
        );
        assert_eq!(
            result.tiles.len(),
            16,
            "pinned square tile count at 120x90 card scale"
        );
    }

    /// `card_target_for_cols` is the exact inverse of
    /// `columns_for_width` at the regime edge: for a sweep of
    /// width/cols pairs the round trip reproduces `cols` — the
    /// guarantee that makes column stepping land on a visible reflow
    /// every press. The cols=0 guard returns the min width.
    #[test]
    fn card_target_for_cols_round_trips_columns() {
        for &avail in &[320.0_f32, 901.0, 1280.0, 2560.0] {
            for cols in 1..=12 {
                let target = super::card_target_for_cols(avail, cols);
                assert_eq!(
                    super::columns_for_width(avail, target),
                    cols,
                    "avail {avail}, cols {cols}: target {target}"
                );
            }
        }
        assert_eq!(super::card_target_for_cols(901.0, 0), super::CARD_W_MIN);
    }

    /// The `−`/`+` card-size controls step exactly one column per
    /// press (each step lands the target on the new regime's edge),
    /// clamp at the 1-column and squash-floor column counts, and no-op
    /// before the first grid pass records the geometry.
    #[test]
    fn step_card_columns_steps_one_column_and_clamps() {
        let mut gui = fresh_gui();
        let avail = 901.0_f32;
        gui.card_grid_avail = avail;
        gui.card_target_w = super::card_target_for_cols(avail, 5);

        gui.step_card_columns(-1); // bigger cards: 5 -> 4 columns
        assert_eq!(gui.card_target_w, super::card_target_for_cols(avail, 4));
        gui.step_card_columns(1); // smaller cards: back to 5
        assert_eq!(gui.card_target_w, super::card_target_for_cols(avail, 5));

        // Clamp at one column.
        for _ in 0..10 {
            gui.step_card_columns(-1);
        }
        assert_eq!(gui.card_target_w, super::card_target_for_cols(avail, 1));

        // ...and at the column count of the squash floor (the same
        // ceiling the wheel path's CARD_W_MIN clamp induces).
        let max_cols = super::columns_for_width(avail, super::CARD_W_MIN);
        for _ in 0..max_cols + 5 {
            gui.step_card_columns(1);
        }
        assert_eq!(
            super::columns_for_width(avail, gui.card_target_w),
            max_cols,
            "stepping + never goes below the squash-floor column count"
        );

        // The no-op path only reads `card_grid_avail` (0.0 = no grid
        // pass yet): force that state and expect no change.
        let before = gui.card_target_w;
        gui.card_grid_avail = 0.0;
        gui.step_card_columns(1);
        gui.step_card_columns(-1);
        assert_eq!(gui.card_target_w, before);
    }

    /// The render zoom cluster: `+`/`−` multiply/divide the viewport
    /// zoom by exactly [`super::RENDER_ZOOM_STEP`], the label reads the
    /// percentage of the construction baseline, and clicking it resets
    /// to 100% (center untouched).
    #[test]
    fn render_zoom_steps_label_and_reset() {
        let mut gui = fresh_gui();
        assert_eq!(gui.zoom_label(), "100%");
        gui.zoom_by_step(1);
        assert!(
            (gui.app.viewport.zoom - 60.0 * super::RENDER_ZOOM_STEP).abs() < 1e-9,
            "one + step is one RENDER_ZOOM_STEP"
        );
        gui.zoom_by_step(-1);
        assert!((gui.app.viewport.zoom - 60.0).abs() < 1e-9);
        gui.app.viewport.zoom = 180.0;
        assert_eq!(gui.zoom_label(), "300%");
        gui.zoom_reset();
        assert_eq!(gui.app.viewport.zoom, 60.0);
        assert_eq!(gui.zoom_label(), "100%");
    }

    /// A real headless pass at a zoomed card size: every card
    /// allocates the zoomed cell (width = the pass's column width,
    /// height = [`super::card_total_h`] of the zoomed preview), and
    /// the grid still fills the window width exactly — the dynamic
    /// height path end-to-end, not just arithmetic.
    #[test]
    fn headless_pass_at_zoomed_card_size() {
        let ctx = egui::Context::default();
        let mut gui = fresh_gui();
        gui.set_card_target_w(360.0);
        let (_iw, ih) = super::card_img_size(360.0);
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1200.0, 900.0));
        let mut col_w = 0.0;
        for _ in 0..3 {
            let input = egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("browser")
                        .show(ui, |ui| gui.draw_card_grid(ctx, ui));
                });
            });
            col_w = gui.card_grid_col_w;
        }
        assert!(!gui.card_rects.is_empty(), "cards painted");
        for r in &gui.card_rects {
            assert!(
                (r.width() - col_w).abs() <= 1.0,
                "card width {} != col_w {col_w}",
                r.width()
            );
            assert!(
                (r.height() - super::card_total_h(ih)).abs() <= 0.01,
                "card height {} != card_total_h({ih}) {}",
                r.height(),
                super::card_total_h(ih)
            );
        }
    }

    /// The fixed card height is factored purely from constants:
    /// image, the two-line title, description, symmetry line, and the
    /// four row spacings, with the line heights ordered name ≥
    /// description ≥ symmetry. The pinned literal catches a
    /// card-height formula edited without re-deriving the sum.
    #[test]
    fn card_inner_h_is_factored_from_constants() {
        assert_eq!(
            super::card_inner_h(super::CARD_IMG_H),
            super::CARD_IMG_H as f32
                + 2.0 * super::CARD_NAME_LINE_H
                + super::CARD_DESC_LINE_H
                + super::CARD_SYM_LINE_H
                + 4.0 * super::CARD_ROW_SPX
        );
        // The default size keeps the historical constant's value.
        assert_eq!(super::card_inner_h(super::CARD_IMG_H), 168.0);
        assert!(
            super::card_inner_h(super::CARD_IMG_H) > super::CARD_IMG_H as f32,
            "text rows must add height below the image"
        );
        // Ordering of the line heights (local copies keep the
        // comparison non-const, so this is a real runtime check).
        let (name_h, desc_h, sym_h) =
            (super::CARD_NAME_LINE_H, super::CARD_DESC_LINE_H, super::CARD_SYM_LINE_H);
        assert!(name_h >= desc_h, "name row is the tallest text row");
        assert!(desc_h >= sym_h);
    }

    /// The line-height constants must cover the actual galley row
    /// heights at the card's font sizes; otherwise the text content
    /// would exceed the fixed card height and the cards would no
    /// longer be uniform. The default font is the embedded
    /// Ubuntu-Light, so the comparison is deterministic.
    #[test]
    fn card_line_heights_cover_the_galley_rows() {
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |_| {});
        let row_h = |size: f32| ctx.fonts(|f| f.row_height(&egui::FontId::proportional(size)));
        assert!(
            super::CARD_NAME_LINE_H >= row_h(16.0),
            "name line {} < galley row height at 16 px",
            super::CARD_NAME_LINE_H
        );
        assert!(
            super::CARD_DESC_LINE_H >= row_h(13.0),
            "description line {} < galley row height at 13 px",
            super::CARD_DESC_LINE_H
        );
        assert!(
            super::CARD_SYM_LINE_H >= row_h(12.0),
            "symmetry line {} < galley row height at 12 px",
            super::CARD_SYM_LINE_H
        );
    }

    /// Global-space rects of all painted text. In epaint 0.29 a galley
    /// is painted as `Shape::Text` whose `galley.rect` is relative to
    /// the shape's `pos`; `Shape::Vec` is walked recursively for
    /// robustness.
    fn painted_text_rects(shapes: &[egui::epaint::ClippedShape]) -> Vec<egui::Rect> {
        fn walk(shape: &egui::Shape, out: &mut Vec<egui::Rect>) {
            match shape {
                egui::Shape::Text(ts) => out.push(ts.galley.rect.translate(ts.pos.to_vec2())),
                egui::Shape::Vec(inner) => inner.iter().for_each(|s| walk(s, out)),
                _ => {}
            }
        }
        let mut out = Vec::new();
        for cs in shapes {
            walk(&cs.shape, &mut out);
        }
        out
    }

    /// The full (un-elided) text of every painted galley, one entry
    /// per text shape, following the same walk as
    /// [`painted_text_rects`]. In egui 0.29 a galley's `text()` is
    /// the full non-elided job text — exactly what an attribution
    /// assertion wants (an ellipsis would hide the line's tail).
    fn painted_text(shapes: &[egui::epaint::ClippedShape]) -> Vec<String> {
        fn walk(shape: &egui::Shape, out: &mut Vec<String>) {
            match shape {
                egui::Shape::Text(ts) => out.push(ts.galley.text().to_string()),
                egui::Shape::Vec(inner) => inner.iter().for_each(|s| walk(s, out)),
                _ => {}
            }
        }
        let mut out = Vec::new();
        for cs in shapes {
            walk(&cs.shape, &mut out);
        }
        out
    }

    /// Run the real browse grid through real headless egui passes at
    /// `w`×`h`; return the LAST pass's `FullOutput` plus the card rects
    /// and column width that pass recorded.
    ///
    /// The Grid runs an invisible sizing pass and `request_discard`s on
    /// its first frame (`ctx.run` loops internally up to `max_passes`,
    /// default 2, and `FullOutput::append` keeps only the newest pass's
    /// shapes) — three whole runs anyway, so every asserted pass is far
    /// past any first-frame sizing behavior. `GuiApp::new()` is
    /// display-free (pure `tess_core` browser state), and the card
    /// textures raster through `tess_core::raster` +
    /// `ctx.load_texture` — all fine with no window.
    fn headless_grid_at(w: f32, h: f32) -> (egui::FullOutput, Vec<egui::Rect>, f32) {
        let ctx = egui::Context::default();
        let mut gui = super::GuiApp::new();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(w, h));
        let mut out = egui::FullOutput::default();
        for _ in 0..3 {
            let input = egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            };
            out = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical().id_salt("browser").show(ui, |ui| {
                        gui.draw_card_grid(ctx, ui);
                    });
                });
            });
        }
        (out, gui.card_rects.clone(), gui.card_grid_col_w)
    }

    /// The owner's complaints — rows overflowing the window, and the
    /// description rendering as "…" to the right of the name instead of
    /// below it — came from egui's Grid handing the card a left-to-right
    /// layout that `Frame::show`'s content child inherited. Arithmetic
    /// property tests failed twice to predict egui, so this test runs a
    /// REAL headless egui pass (no model of egui's layout math) and
    /// asserts on the actually-painted geometry at three window sizes —
    /// including a wide one (~10 columns), the regression guard for a
    /// fully dynamic column count at high counts:
    ///
    /// 1. No horizontal overflow: no painted shape's visible rect
    ///    (clipped) crosses the screen's right edge (+1 px slack).
    /// 2. Orthogonal grid: all recorded card rects share one width and
    ///    one height (±0.5), the exact allocated [`CARD_TOTAL_H`], and
    ///    a width equal to the pass's column width (±1.0).
    /// 3. Text stacks vertically: every galley centered inside a card
    ///    ends at or before the card's right edge (+1 px), and at least
    ///    one card holds ≥3 galleys whose left edges sit at the inner
    ///    left margin with strictly increasing min.y — name above
    ///    description above symmetry line (an LTR flow puts only the
    ///    name at the margin and drags the rest rightward).
    #[test]
    fn headless_egui_pass_grid_geometry() {
        for &(w, h) in &[(1200.0f32, 900.0f32), (640.0, 800.0), (2560.0, 900.0)] {
            let (out, card_rects, col_w) = headless_grid_at(w, h);
            let tag = format!("{w}x{h}");

            // (1) Nothing paints beyond the screen's right edge.
            for cs in &out.shapes {
                let vis = cs.clip_rect.intersect(cs.shape.visual_bounding_rect());
                assert!(
                    !vis.is_positive() || vis.right() <= w + 1.0,
                    "{tag}: shape paints past the screen's right edge: {:?}",
                    cs.shape
                );
            }

            // (2) Uniform, exactly-sized cards.
            assert!(
                card_rects.len() >= 3,
                "{tag}: the default tab shows several cards, got {}",
                card_rects.len()
            );
            let first = card_rects[0];
            for r in &card_rects {
                assert!(
                    (r.width() - first.width()).abs() <= 0.5
                        && (r.height() - first.height()).abs() <= 0.5,
                    "{tag}: card rects are not uniform: {first:?} vs {r:?}"
                );
                assert!(
                    (r.width() - col_w).abs() <= 1.0,
                    "{tag}: card width {} != col_w {col_w}",
                    r.width()
                );
                assert!(
                    (r.height() - super::card_total_h(super::CARD_IMG_H)).abs() <= 0.01,
                    "{tag}: card height {} != card_total_h(IMG_H) {}",
                    r.height(),
                    super::card_total_h(super::CARD_IMG_H)
                );
            }

            // (3) Text stacks vertically inside each card. Two parts:
            // every galley that lies in a card ends at/before its right
            // edge, AND — the stacking proof — each card shows ≥3
            // galleys whose left edge sits at the inner-left margin
            // (TopDown+LEFT puts every row there; an LTR-inherited flow
            // puts the name there and drags the description/symmetry
            // line to its right, where they truncate to "…") and whose
            // vertical center lies in the card, at strictly increasing
            // min.y: name above description above symmetry line.
            let galleys = painted_text_rects(&out.shapes);
            let mut cards_stacked = 0;
            for cr in &card_rects {
                let contained: Vec<egui::Rect> = galleys
                    .iter()
                    .copied()
                    .filter(|g| cr.contains(g.center()))
                    .collect();
                for g in &contained {
                    assert!(
                        g.right() <= cr.right() + 1.0,
                        "{tag}: text galley {g:?} sticks out right of card {cr:?}"
                    );
                    // No-clipping at the two-line-title budget: every
                    // card's text must fit the card height (the
                    // 2-line name grew the budget; a stale formula
                    // would clip the symmetry line below the card).
                    assert!(
                        g.bottom() <= cr.bottom() + 1.0,
                        "{tag}: text galley {g:?} is clipped below card {cr:?} (CARD_INNER_H budget too small)"
                    );
                    assert!(
                        g.top() >= cr.top() - 1.0,
                        "{tag}: text galley {g:?} is clipped above card {cr:?}"
                    );
                }
                let left_rows: Vec<egui::Rect> = galleys
                    .iter()
                    .copied()
                    .filter(|g| {
                        g.left() >= cr.left() - 0.5
                            && g.left() <= cr.left() + super::CARD_INNER_MARGIN + 1.0
                            && g.center().y > cr.top()
                            && g.center().y < cr.bottom()
                    })
                    .collect();
                if left_rows.len() >= 3 {
                    cards_stacked += 1;
                    let mut tops: Vec<f32> = left_rows.iter().map(|g| g.min.y).collect();
                    tops.sort_by(|a, b| a.total_cmp(b));
                    for pair in tops.windows(2) {
                        assert!(
                            pair[0] < pair[1],
                            "{tag}: text rows are not stacked: min.y {} !< {}",
                            pair[0],
                            pair[1]
                        );
                    }
                }
            }
            assert!(
                cards_stacked >= 1,
                "{tag}: no card stacks the name/description/symmetry lines\
                 at its inner-left margin ({} cards, {} galleys painted)",
                card_rects.len(),
                galleys.len()
            );

            // Wide case only: with a fully dynamic column count the
            // wide window runs ~10 columns — the tab's cards must
            // span more than one row of them (the `chunks(cols)`
            // path exercised at high column counts).
            if w >= 2560.0 {
                let cols = super::columns_for_width(w, super::CARD_TARGET_W);
                assert!(
                    card_rects.len() > cols,
                    "{tag}: {} cards do not span >1 row of {cols} columns",
                    card_rects.len()
                );
            }
        }
    }

    /// Render-mode prev/next walks the selected tiling's tab order and
    /// updates the per-tab selection memory, wrapping — so returning
    /// to Browse lands on the walked card (mirrors the TUI's `walk`).
    /// Also pins the mode switch on card selection.
    #[test]
    fn walk_render_walks_tab_order_and_updates_memory() {
        let mut gui = fresh_gui();
        let at = gui.active_tab;
        // Copy the &'static str ids out first: the walks below need
        // `&mut gui`, so no borrow of `gui.tabs` may survive.
        let (n, first_id, second_id, last_id): (usize, &'static str, &'static str, &'static str) =
            (
                gui.tabs[at].cards.len(),
                gui.tabs[at].cards[0].id,
                gui.tabs[at].cards[1].id,
                gui.tabs[at].cards.last().unwrap().id,
            );
        assert!(n >= 3, "initial tab has several cards");

        // Clicking a card enters Render mode with the panel expanded.
        gui.select_card(first_id);
        assert_eq!(gui.mode, super::GuiMode::Render);
        assert!(gui.params_open);
        assert_eq!(gui.app.selected_id(), first_id);

        // Walk forward: the selection and the tab memory follow.
        gui.walk_render(1);
        assert_eq!(gui.app.selected_id(), second_id, "next card in tab order");
        assert_eq!(gui.tab_selected[at].as_deref(), Some(second_id));

        // Step back, then walk all the way to the last card and
        // across the wrap to the first.
        gui.walk_render(-1);
        assert_eq!(gui.app.selected_id(), first_id);
        for _ in 1..n {
            gui.walk_render(1);
        }
        assert_eq!(gui.app.selected_id(), last_id, "walked to the last card");
        gui.walk_render(1);
        assert_eq!(gui.app.selected_id(), first_id, "wraps to the first card");
        assert_eq!(gui.tab_selected[at].as_deref(), Some(first_id));
    }

    /// The canvas result cache, driven headlessly through `draw_canvas`
    /// itself — the same path the render loop takes. First draw on a
    /// view = exactly one miss; an unchanged view = zero more; zoom,
    /// pan, param change, selection change and canvas resize each add
    /// exactly one (the one-to-one correspondence between a trigger and
    /// a regeneration is the whole point of the cache).
    #[test]
    fn canvas_cache_tracks_view_params_and_selection() {
        let ctx = egui::Context::default();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
        let mut gui = fresh_gui();
        let pass = |gui: &mut super::GuiApp, screen: egui::Rect| {
            let input = egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                gui.draw_canvas(ctx);
            });
        };

        pass(&mut gui, screen);
        assert_eq!(gui.render_cache_misses, 1, "first draw generates");
        pass(&mut gui, screen);
        assert_eq!(gui.render_cache_misses, 1, "same view stays cached");

        // Zoom (the zoom label/button path mutates the viewport; the
        // cache key reads it fresh each pass).
        gui.app.viewport.zoom = 120.0;
        pass(&mut gui, screen);
        assert_eq!(gui.render_cache_misses, 2, "zoom regenerates");

        // Pan, restoring the baseline zoom first so this pass's miss is
        // the pan itself.
        gui.app.viewport.zoom = 60.0;
        gui.app.pan(7.0, 3.0);
        pass(&mut gui, screen);
        assert_eq!(gui.render_cache_misses, 3, "pan regenerates");

        // Parameter change (square tiling's `scale` param).
        let scale = tess_core::params::ParamId::new("scale");
        gui.app.set_param_value(scale, tess_core::params::ParamValue::Float(2.0));
        pass(&mut gui, screen);
        assert_eq!(gui.render_cache_misses, 4, "param change regenerates");

        // Selection change.
        gui.app.select_id("penrose");
        pass(&mut gui, screen);
        assert_eq!(gui.render_cache_misses, 5, "selection change regenerates");

        // Canvas resize (a different screen rect is a different key).
        let screen2 = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(900.0, 500.0));
        pass(&mut gui, screen2);
        assert_eq!(gui.render_cache_misses, 6, "canvas resize regenerates");
        pass(&mut gui, screen2);
        assert_eq!(gui.render_cache_misses, 6, "same resized view stays cached");
    }

    /// The canvas cache stores a MAX_TILES refusal like any other
    /// result: two passes at a view that refuses (far-zoomed-out) draw
    /// it once, not once per frame.
    #[test]
    fn canvas_cache_caches_max_tiles_refusals() {
        let ctx = egui::Context::default();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
        let mut gui = fresh_gui();
        // Tiny zoom → a huge world region → a refused generation.
        gui.app.viewport.zoom = 0.01;
        let pass = |gui: &mut super::GuiApp| {
            let input = egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                gui.draw_canvas(ctx);
            });
        };
        pass(&mut gui);
        assert_eq!(gui.render_cache_misses, 1, "first draw generates (refusal)");
        assert!(
            gui.render_cache.as_ref().unwrap().result.message.is_some(),
            "zoom 0.01 at this canvas size refuses"
        );
        pass(&mut gui);
        assert_eq!(gui.render_cache_misses, 1, "refusal cached across frames");
    }

    /// Canvas z-order regression (the taylor-socolar "strap-eating" bug):
    /// the painter must emit every tile fill before every outline. A
    /// `closed_line` stroke (1.0 px) straddles its polygon edge — half a
    /// pixel pokes into EVERY neighbor — so an interleaved per-tile pass
    /// lets a later-drawn neighbor's fill cover the earlier tile's
    /// outline half along their shared edge. For taylor-socolar, whose
    /// hexagons are partitioned into 8 convex pieces including thin
    /// chord strips (the "straps"), a strip loses its grout border on
    /// every side shared with a later-drawn piece and visually merges
    /// into the surrounding hexagon. The identical bug and two-pass fix
    /// are documented for SVG export in
    /// `crates/tess-core/src/svg.rs` (L15-31). Pinned headlessly for
    /// taylor-socolar (the reported tiling) and chair (non-convex
    /// L-tromino pieces).
    #[test]
    fn canvas_paints_all_fills_before_all_outlines() {
        /// Walk a `ClippedShape` tree recursively, keeping everything in
        /// order (the canvas emits no nested Vec shapes in this path,
        /// but the walk mirrors `painted_text_rects` for robustness).
        fn walk(shape: &egui::Shape, out: &mut Vec<egui::Shape>) {
            match shape {
                egui::Shape::Vec(inner) => inner.iter().for_each(|s| walk(s, out)),
                s => out.push(s.clone()),
            }
        }

        for id in ["taylor-socolar", "chair"] {
            let ctx = egui::Context::default();
            let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
            let mut gui = super::GuiApp::new();
            gui.app.select_id(id);
            let input = egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            };
            let out = ctx.run(input, |ctx| {
                gui.draw_canvas(ctx);
            });

            let (message, expect) = {
                let r = &gui.render_cache.as_ref().unwrap().result;
                (
                    r.message,
                    r.tiles.iter().filter(|t| t.poly.pts.len() >= 3).count(),
                )
            };
            assert!(
                message.is_none(),
                "{id}: default view refused: {message:?}"
            );
            assert!(expect >= 1, "{id}: expected at least one painted tile");

            let mut shapes = Vec::new();
            for cs in &out.shapes {
                walk(&cs.shape, &mut shapes);
            }

            // (a) The two-pass invariant: once the first outline appears,
            // no mesh may follow it (otherwise fills are interleaved with
            // outlines and later fills eat earlier outlines).
            let first_outline = shapes
                .iter()
                .position(|s| matches!(s, egui::Shape::Path(p) if p.closed));
            if let Some(pos) = first_outline {
                for s in shapes.iter().skip(pos + 1) {
                    assert!(
                        !matches!(s, egui::Shape::Mesh(_)),
                        "{id}: mesh painted after the first closed outline — \
                         fills and outlines are interleaved"
                    );
                }
            }

            // (b) One mesh and one outline per non-degenerate tile.
            let meshes = shapes
                .iter()
                .filter(|s| matches!(s, egui::Shape::Mesh(_)))
                .count();
            let outlines = shapes
                .iter()
                .filter(|s| matches!(s, egui::Shape::Path(p) if p.closed))
                .count();
            assert_eq!(meshes, expect, "{id}: fill count != non-degenerate tile count");
            assert_eq!(outlines, expect, "{id}: outline count != non-degenerate tile count");
        }
    }

    /// `cache_key` is exactly the geometry the generator reads —
    /// stable under identical reads, divergent on zoom/pan/size/param/
    /// selection, and deliberately blind to color mode (painting reads
    /// it fresh every frame).
    #[test]
    fn cache_key_is_stable_and_sensitive() {
        let mut gui = fresh_gui();
        let key = gui.cache_key(800, 600);
        assert_eq!(key, gui.cache_key(800, 600), "stable across identical reads");

        let baseline_zoom = gui.app.viewport.zoom;
        gui.app.viewport.zoom = 120.0;
        assert_ne!(key, gui.cache_key(800, 600), "zoom enters the key");
        gui.app.viewport.zoom = baseline_zoom;
        assert_eq!(key, gui.cache_key(800, 600), "zoom restore");

        let baseline_center = gui.app.viewport.center;
        gui.app.viewport.center = tess_core::Vec2::new(1.0, -2.0);
        assert_ne!(key, gui.cache_key(800, 600), "center enters the key");
        gui.app.viewport.center = baseline_center;
        assert_eq!(key, gui.cache_key(800, 600), "center restore");

        assert_ne!(key, gui.cache_key(801, 600), "width enters the key");
        assert_ne!(key, gui.cache_key(800, 601), "height enters the key");

        let scale = tess_core::params::ParamId::new("scale");
        let old_scale = gui.app.param_value(scale).unwrap();
        gui.app.set_param_value(scale, tess_core::params::ParamValue::Float(2.0));
        assert_ne!(key, gui.cache_key(800, 600), "params enter the key");
        gui.app.set_param_value(scale, old_scale);
        assert_eq!(key, gui.cache_key(800, 600), "param restore");

        gui.app.select_id("penrose");
        assert_ne!(key, gui.cache_key(800, 600), "selection enters the key");
        gui.app.select_id("square");
        assert_eq!(key, gui.cache_key(800, 600), "selection restore");

        gui.app.cycle_color_mode();
        assert_eq!(key, gui.cache_key(800, 600), "color mode is excluded");
    }

    /// Fresh `should_regen` is the miss; same key is the hit — the
    /// exact decision `draw_canvas` makes per frame.
    #[test]
    fn should_regen_mirrors_cache_freshness() {
        let mut gui = fresh_gui();
        assert!(gui.should_regen(&gui.cache_key(800, 600)), "empty cache: regen");
        let key = gui.cache_key(800, 600);
        let result = gui.app.result(800, 600);
        gui.render_cache = Some(super::RenderCache { key: key.clone(), result });
        assert!(!gui.should_regen(&key), "same key: hit");
        gui.app.viewport.zoom = 120.0;
        assert!(gui.should_regen(&gui.cache_key(800, 600)), "new key: regen");
    }

    /// COLOR-MODE FAST PATH (phase-2a safety net): the geometry cache is
    /// blind to color mode — painting reads it live every frame and it
    /// never affects the generated geometry. Pinned through the headless
    /// `draw_canvas` pass itself: one pass generates, then cycling the
    /// paint-time state and re-running the pass reuses the stored result
    /// (misses unchanged, stored RenderKey identical, result verbatim).
    /// This is the contract the phase-2a off-thread generation must
    /// preserve — color must never become a regeneration trigger.
    ///
    /// The honest observable here is `render_cache_misses` plus the stored
    /// key identity (RenderCache exposes no generation counter or texture
    /// handle): a regeneration would both increment the miss counter and
    /// produce a fresh stored key.
    #[test]
    fn canvas_cache_recolors_without_regeneration() {
        let ctx = egui::Context::default();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
        let mut gui = fresh_gui();
        let pass = |gui: &mut super::GuiApp, screen: egui::Rect| {
            let input = egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                gui.draw_canvas(ctx);
            });
        };

        pass(&mut gui, screen);
        assert_eq!(gui.render_cache_misses, 1, "first draw generates");
        let mode_before = gui.app.color_mode;
        let key_before = gui.render_cache.as_ref().unwrap().key.clone();
        let tiles_before = gui.render_cache.as_ref().unwrap().result.tiles.len();

        // A color-mode change is paint-time state, not a generation trigger.
        gui.app.cycle_color_mode();
        assert_ne!(gui.app.color_mode, mode_before, "color mode actually changed");

        pass(&mut gui, screen);
        assert_eq!(gui.render_cache_misses, 1, "color-mode change must not regenerate");
        let after = gui.render_cache.as_ref().unwrap();
        assert_eq!(after.key, key_before, "same stored RenderKey after a color-mode-only change");
        assert_eq!(after.result.tiles.len(), tiles_before, "cached result reused verbatim");
    }

    /// SELECTION-CHANGE POLICY (phase-2a safety net): switching the
    /// tiling id regenerates exactly once and REPLACES the stored entry —
    /// a single slot, wholesale replacement — with no multi-entry retain.
    /// Pinned through the headless `draw_canvas` pass: select A, then B,
    /// then re-select A — misses go 1 -> 2 -> 3, and the re-selected A
    /// regenerates the identical key (state is not retained across the
    /// switch).
    #[test]
    fn canvas_cache_selection_change_is_wholesale_replacement() {
        let ctx = egui::Context::default();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
        let mut gui = fresh_gui();
        let pass = |gui: &mut super::GuiApp, screen: egui::Rect| {
            let input = egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                gui.draw_canvas(ctx);
            });
        };

        pass(&mut gui, screen);
        assert_eq!(gui.render_cache_misses, 1, "first draw generates");
        let key_a = gui.render_cache.as_ref().unwrap().key.clone();

        gui.app.select_id("penrose");
        pass(&mut gui, screen);
        assert_eq!(gui.render_cache_misses, 2, "selection change regenerates exactly once");
        let key_b = gui.render_cache.as_ref().unwrap().key.clone();
        assert_ne!(key_b, key_a, "stored entry replaced by the new selection's key");
        assert_eq!(key_b.id, "penrose", "stored key is the new selection's");

        // Re-selecting regenerates from scratch — no multi-entry retain.
        gui.app.select_id("square");
        pass(&mut gui, screen);
        assert_eq!(gui.render_cache_misses, 3, "re-selecting regenerates");
        let key_a2 = gui.render_cache.as_ref().unwrap().key.clone();
        assert_eq!(key_a2, key_a, "re-selection re-generates the identical key (no retained state)");
    }

    /// PER-AXIS SIZE KEY (phase-2a safety net): a width-only or height-only
    /// change is a distinct key — pinning that the canvas-size fields are
    /// individually sensitive, not just jointly (existing one-to-one
    /// pin changes both axes together).
    #[test]
    fn canvas_cache_size_change_regenerates_per_axis() {
        let ctx = egui::Context::default();
        let mut gui = fresh_gui();
        let pass = |gui: &mut super::GuiApp, w: f32, h: f32| {
            let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(w, h));
            let input = egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                gui.draw_canvas(ctx);
            });
        };

        pass(&mut gui, 800.0, 600.0);
        assert_eq!(gui.render_cache_misses, 1, "first draw generates");
        pass(&mut gui, 800.0, 600.0);
        assert_eq!(gui.render_cache_misses, 1, "same size stays cached");

        pass(&mut gui, 801.0, 600.0);
        assert_eq!(gui.render_cache_misses, 2, "width-only change regenerates");
        pass(&mut gui, 801.0, 600.0);
        assert_eq!(gui.render_cache_misses, 2, "same resized view stays cached");

        pass(&mut gui, 801.0, 599.0);
        assert_eq!(gui.render_cache_misses, 3, "height-only change regenerates");

        pass(&mut gui, 800.0, 600.0);
        assert_eq!(gui.render_cache_misses, 4, "both-axis restore regenerates");
    }

    /// The About panel's headless pin: closed, nothing attribution-flavored
    /// paints; open, the app name + version, both attribution lines, and
    /// the docs pointer all paint. Driven through the same
    /// `draw_about` the render loop calls — attribution presence/absence
    /// is the whole contract (the H toggle / title-bar button live in
    /// `update`, untestable headlessly, as the other key handlers are).
    ///
    /// A window first does a sizing pass (invisible, no shapes painted)
    /// and the real paint lands on a later pass, so each state is run
    /// a few passes like the other headless window-adjacent tests —
    /// `pass` drives one `ctx.run`, mirroring the `headless_grid_at`
    /// loop.
    #[test]
    fn about_panel_attribution_text_toggles() {
        let ctx = egui::Context::default();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(900.0, 700.0));
        let input = egui::RawInput {
            screen_rect: Some(screen),
            ..Default::default()
        };
        let mut gui = fresh_gui();
        // Open toggles the state; the panel itself is drawn from a
        // fresh `draw_about` per pass. `out` is the last pass's output
        // (egui paints only the newest pass's shapes).
        let pass = |gui: &mut super::GuiApp, input: egui::RawInput| {
            let mut last = egui::FullOutput::default();
            for _ in 0..4 {
                last = ctx.run(input.clone(), |ctx| gui.draw_about(ctx));
            }
            last
        };
        assert!(!gui.about_open);
        let out = pass(&mut gui, input.clone());
        let text = painted_text(&out.shapes).join("\n");
        assert!(!text.contains("Tactile"), "closed panel painted a Tactile line: {text:?}");
        assert!(!text.contains("Jeandel"), "closed panel painted a Jeandel–Rao line: {text:?}");
        assert!(!text.contains("docs/tilings"), "closed panel painted the docs pointer: {text:?}");
        assert!(!text.contains(env!("CARGO_PKG_VERSION")), "closed panel painted its version line: {text:?}");

        // Open: every line paints.
        gui.about_open = true;
        let out = pass(&mut gui, input);
        let text = painted_text(&out.shapes).join("\n");
        for needle in [
            "tessellix",
            env!("CARGO_PKG_VERSION"),
            "Kaplan/Tactile",
            "isohedral",
            "Jeandel–Rao",
            "arXiv:1506.06492",
            "docs/tilings/*.md",
        ] {
            assert!(text.contains(needle), "open panel missing {needle:?}: {text:?}");
        }
    }

    // ---- preview color-mode band (the Browse selector re-color job) ----

    /// Drive one egui pass of the preview band's mode row through the
    /// production `draw_preview_mode_row`, optionally injecting a
    /// primary-button press+release at `click_at` (hovered first) —
    /// the headless click recipe; the M199 About test's pass-loop
    /// pattern, single pass per state.
    fn preview_row_pass(
        gui: &mut super::GuiApp,
        ectx: &egui::Context,
        screen: egui::Rect,
        click_at: Option<egui::Pos2>,
    ) -> egui::FullOutput {
        let mut events: Vec<egui::Event> = Vec::new();
        if let Some(pos) = click_at {
            events.push(egui::Event::PointerMoved(pos));
            events.push(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            });
            events.push(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            });
        }
        let input = egui::RawInput {
            screen_rect: Some(screen),
            events,
            ..Default::default()
        };
        ectx.run(input, |ectx| {
            egui::CentralPanel::default().show(ectx, |ui| gui.draw_preview_mode_row(ui));
        })
    }

    /// The band paints `default` plus the ten [`ColorMode::ALL`] labels
    /// and records eleven hit rects — the same option list as the
    /// Render detail panel through the shared `mode_selector_row`
    /// (whose render variant, without `include_default`, pins ten).
    #[test]
    fn preview_mode_row_lists_default_plus_ten_modes() {
        use tess_core::view::ColorMode;
        let ectx = egui::Context::default();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1200.0, 700.0));
        let mut gui = fresh_gui();
        let out = preview_row_pass(&mut gui, &ectx, screen, None);
        assert_eq!(gui.preview_mode_rects.len(), ColorMode::ALL.len() + 1);
        let text = painted_text(&out.shapes).join(" ");
        assert!(text.contains("default"), "band missing the default label: {text:?}");
        for m in ColorMode::ALL {
            assert!(text.contains(m.label()), "band missing mode label {:?}: {text:?}", m.label());
        }
        // The shared row's render-site variant (include_default = false,
        // as `draw_render` calls it) offers exactly the ten modes.
        let out = ectx.run(
            egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            },
            |_ectx| {
                egui::CentralPanel::default().show(_ectx, |ui| {
                    let picks = super::mode_selector_row(ui, None, false);
                    assert_eq!(picks.len(), ColorMode::ALL.len());
                    assert!(picks.iter().all(|(m, ..)| m.is_some()), "render variant must not offer default");
                });
            },
        );
        let text = painted_text(&out.shapes).join(" ");
        assert!(!text.contains("default"), "render variant offered a default label: {text:?}");
    }

    /// Real headless clicks on the band drive the shared state: picking
    /// `shape` sets the override AND the canvas mode (`app.color_mode`),
    /// picking `default` clears the override and returns the canvas to
    /// the selected tiling's own default mode.
    #[test]
    fn preview_mode_row_clicks_drive_shared_state() {
        use tess_core::view::ColorMode;
        let ectx = egui::Context::default();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1200.0, 700.0));
        let mut gui = fresh_gui();
        // Pass 1: record the hit rects.
        preview_row_pass(&mut gui, &ectx, screen, None);
        let pos_of = |want: Option<ColorMode>| -> egui::Pos2 {
            gui.preview_mode_rects
                .iter()
                .find(|(m, _)| *m == want)
                .map(|(_, r)| *r)
                .expect("label rect recorded")
                .center()
        };
        let p_shape = pos_of(Some(ColorMode::Shape));
        let p_role = pos_of(Some(ColorMode::Role));
        let p_default = pos_of(None);
        let tiling_default = gui.app.tiling().default_color_mode();
        // Click "shape": preview override + canvas move together.
        preview_row_pass(&mut gui, &ectx, screen, Some(p_shape));
        assert_eq!(gui.preview_mode, Some(ColorMode::Shape));
        assert_eq!(gui.app.color_mode, ColorMode::Shape);
        // Click "role": the override follows, canvas stays in lockstep.
        preview_row_pass(&mut gui, &ectx, screen, Some(p_role));
        assert_eq!(gui.preview_mode, Some(ColorMode::Role));
        assert_eq!(gui.app.color_mode, ColorMode::Role);
        // Click "default": override cleared, canvas back to the
        // selected tiling's default mode.
        preview_row_pass(&mut gui, &ectx, screen, Some(p_default));
        assert_eq!(gui.preview_mode, None);
        assert_eq!(gui.app.color_mode, tiling_default);
    }

    /// The funnel's state contract, headless and direct (the click
    /// wiring above already proves the band reaches it):
    /// * `set_preview_mode(Some(m))` drives `app.color_mode = m`;
    /// * a real change clears the card-texture cache (keys embed the
    ///   effective mode, so previews regenerate in the new colors);
    /// * `card_preview_mode` is the override while set, else the
    ///   tiling's own default;
    /// * `select_id_synced` keeps an override across selection (where
    ///   bare `select_id` would reset the canvas mode), and with no
    ///   override reproduces the historical per-tiling default.
    #[test]
    fn preview_mode_funnel_state_and_selection_flow() {
        use tess_core::view::ColorMode;
        let mut gui = fresh_gui();
        let penrose_default = gui
            .app
            .registry()
            .get("penrose")
            .expect("penrose registered")
            .default_color_mode();
        let square_default = gui
            .app
            .registry()
            .get("square")
            .expect("square registered")
            .default_color_mode();
        // No override: each tiling previews in its own default mode.
        let penrose = gui.app.registry().get("penrose").expect("penrose");
        let square = gui.app.registry().get("square").expect("square");
        assert_eq!(gui.card_preview_mode(penrose), penrose_default);
        assert_eq!(gui.card_preview_mode(square), square_default);
        // Override: every tiling previews in the chosen mode, and the
        // canvas reads it live.
        gui.set_preview_mode(Some(ColorMode::Orientation));
        let penrose = gui.app.registry().get("penrose").expect("penrose");
        let square = gui.app.registry().get("square").expect("square");
        assert_eq!(gui.card_preview_mode(penrose), ColorMode::Orientation);
        assert_eq!(gui.card_preview_mode(square), ColorMode::Orientation);
        assert_eq!(gui.app.color_mode, ColorMode::Orientation);
        // Selection must NOT drop an explicit override (bare
        // `App::select_id` resets the canvas mode; the synced wrapper
        // re-applies it).
        gui.select_id_synced("square");
        assert_eq!(gui.app.selected_id(), "square");
        assert_eq!(gui.app.color_mode, ColorMode::Orientation);
        // Clearing the override returns the canvas to the CURRENT
        // tiling's default...
        gui.set_preview_mode(None);
        assert_eq!(gui.app.color_mode, square_default);
        // ...and selection then behaves historically (per-tiling default).
        gui.select_id_synced("penrose");
        assert_eq!(gui.app.color_mode, penrose_default);
    }

    /// End-to-end recolor through the production `card_texture` path:
    /// the default preview keys under the tiling's own default mode;
    /// picking Shape re-keys (and, by the funnel's clear, regenerates)
    /// the texture in shape colors — the user's report that previews
    /// could not be re-colored at all.
    #[test]
    fn card_texture_rekeys_under_preview_mode_override() {
        use tess_core::view::ColorMode;
        let ectx = egui::Context::default();
        let mut gui = fresh_gui();
        let card = card_fix(
            "penrose",
            tess_core::TilingCategory::Uniform,
            None,
            None,
        );
        let default_mode = gui
            .app
            .registry()
            .get("penrose")
            .expect("penrose registered")
            .default_color_mode()
            .preview_mode();
        let _h = gui.card_texture(&ectx, &card, 120, 90);
        assert_eq!(gui.card_textures.len(), 1);
        let key0 = super::card_key("penrose", "", default_mode, 120, 90);
        // Whatever the default, it is NOT shape mode here (penrose's
        // default is not Shape, else this test's contrast collapses).
        assert_ne!(default_mode, ColorMode::Shape);
        assert!(
            gui.card_textures.keys().any(|k| k.split('|').last() == key0.split('|').last()
                && k.split('|').nth(2) == Some(default_mode.label())),
            "default preview keyed under the default mode: {:?}",
            gui.card_textures.keys().collect::<Vec<_>>()
        );
        // Picking Shape clears the cache and re-keys the next generation.
        gui.set_preview_mode(Some(ColorMode::Shape));
        assert_eq!(gui.card_textures.len(), 0, "override change drops stale-mode textures");
        let _h2 = gui.card_texture(&ectx, &card, 120, 90);
        assert_eq!(gui.card_textures.len(), 1);
        let k = gui.card_textures.keys().next().unwrap().clone();
        assert_eq!(k.split('|').nth(2), Some(ColorMode::Shape.label()));
        assert!(k.ends_with("|120x90"), "size stays the final |WxH segment for pruning");
    }

    /// Drain the pending texture upload for `handle` from the headless
    /// `egui::Context`'s texture-manager delta queue. `card_texture` is
    /// test-only exercised without painting; the delta is the only
    /// channel through which the production pixel buffer can be read.
    fn drain_card_texture_bytes(ctx: &egui::Context, handle: &egui::TextureHandle) -> Vec<u8> {
        let id = handle.id();
        let delta = ctx.tex_manager().write().take_delta();
        let mut out = Vec::new();
        for (tid, img) in delta.set {
            if tid == id {
                if let egui::ImageData::Color(c) = img.image {
                    for p in c.pixels.iter() {
                        out.extend_from_slice(&[p.r(), p.g(), p.b(), p.a()]);
                    }
                }
            }
        }
        assert!(!out.is_empty(), "texture {id:?} must appear in the delta");
        out
    }

    /// Render one card in a given preview mode through the production
    /// `card_texture` path and return its raw RGBA bytes. Each call uses
    /// a fresh `GuiApp` so the cache starts empty and every mode pays
    /// the full rasterize+upload path (the M215 precedent).
    fn card_bytes(mode: tess_core::view::ColorMode) -> Vec<u8> {
        let ectx = egui::Context::default();
        let mut gui = fresh_gui();
        let card = card_fix(
            "penrose",
            tess_core::TilingCategory::Uniform,
            None,
            None,
        );
        gui.set_preview_mode(Some(mode));
        let handle = gui.card_texture(&ectx, &card, 120, 90);
        drain_card_texture_bytes(&ectx, &handle)
    }

    /// b2b4-hazard-verify HAZARD 2 pin, resolved by the ramp-selection
    /// fix: the mode-aware card path.
    ///
    /// `card_texture` classifies tiles with the effective preview mode
    /// (`mode = self.card_preview_mode(tiling).preview_mode()`) — the
    /// same classification indexes every mode of the SAME index-space
    /// group (`Pattern`/`PatternGray`/`Role` all classify as `seed % 8`) —
    /// and now maps each index through the mode's OWN ramp. The
    /// pre-fix pin asserted byte-identity across that group (both
    /// palettes were the same hardcoded rainbow `PALETTE`); the fix
    /// flips the pin: the same card in `Pattern`, `Role` and
    /// `PatternGray` now produces DIFFERENT buffers even though the
    /// classification index is identical — the three modes promise
    /// different ramps (`PALETTE` rainbow vs `ROLE_PALETTE` earth tones
    /// vs `PATTERN_GRAY_PALETTE` grays) and the card must show them.
    /// Same-ramp modes stay byte-identical: `Pattern` vs `Single`
    /// (Single falls back to `Proto` via `preview_mode` and the penrose
    /// card's `seed == proto`, so both render `PALETTE[seed % 8]`).
    /// The pre-fix `Pattern`/`Role` byte-identity was evidence of the
    /// mode-blind cop-out and is no longer a valid invariant.
    #[test]
    fn card_texture_pixels_differ_across_same_index_space_modes() {
        use tess_core::view::ColorMode;
        let pattern = card_bytes(ColorMode::Pattern);
        let role = card_bytes(ColorMode::Role);
        let pattern_gray = card_bytes(ColorMode::PatternGray);
        let single = card_bytes(ColorMode::Single);
        assert_ne!(pattern, role, "Pattern vs Role must differ (same index space, different ramps)");
        assert_ne!(pattern, pattern_gray, "Pattern vs PatternGray must differ (same index space, different ramps)");
        assert_eq!(pattern, single, "Pattern vs Single must stay byte-identical (same identity ramp; Single falls back to Proto)");
        // Non-vacuous: the card is not a solid block — the preview
        // genuinely contains more than one distinct color.
        let distinct: std::collections::HashSet<u32> = pattern
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], 0]))
            .collect();
        assert!(distinct.len() >= 2, "card should contain ≥2 distinct colors, got {}", distinct.len());
    }

    /// The same card in a mode whose classification actually differs
    /// (`Shape`: polygon-derived similarity class, not seed%8) must
    /// produce a byte-DIFFERENT buffer — the contrast pin shows the
    /// same-index-space pin cannot be vacuous: differing indices really
    /// land in this buffer, not just identical ramps.
    #[test]
    fn card_texture_pixel_bytes_differ_across_classification_modes() {
        use tess_core::view::ColorMode;
        let pattern = card_bytes(ColorMode::Pattern);
        let shape = card_bytes(ColorMode::Shape);
        assert_ne!(pattern, shape, "Pattern vs Shape must differ (different classification)");
    }
}
