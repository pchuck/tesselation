//! GUI application (egui) — a single-pane, two-mode browser mirroring
//! the TUI (UI-3; the owner prefers the TUI layout):
//!
//! * **Browse** — the whole window is the card grid: a tab band (the
//!   six category tabs + a search field, one wrapping row) over
//!   crystal-system section headers and cards with lazily generated
//!   previews (shared `tess_core::raster`). Clicking a card opens
//!   Render mode.
//! * **Render** — the whole window is the canvas (drag-pan,
//!   wheel-zoom) with a top strip (`← back`, `◀`/`▶` prev/next in the
//!   tab order, a params toggle, reset) and a collapsible bottom panel
//!   holding the params, color mode, and symmetry readout. Escape or
//!   `← back` returns to Browse exactly as left (same tab + selection).
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
/// is the number of these that fit in the window width (clamped
/// 1..=4 by [`columns_for_width`]); the cards then stretch to fill
/// their columns, landing around 220–260 px wide on typical windows.
const CARD_TARGET_W: f32 = 240.0;
/// Horizontal grid spacing between card columns, in pixels.
const GRID_SPX: f32 = 10.0;
/// Truncated-description fragment length in characters (the TUI uses
/// ~40 chars for its 28-cell-wide cards).
const DESC_CHARS: usize = 40;

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
    /// size). Textures are generated in the draw pass for visible cards
    /// only — generating all 59 up front would stall the first frame.
    card_textures: HashMap<String, egui::TextureHandle>,
    /// The browser's search text (name/id/description, all tabs).
    /// Kept in GuiApp rather than `app.search` because the TUI and the
    /// tess-core App.search filter on different fields.
    search: String,
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
/// `header` is `""` for runs without a header (every non-Wallpaper
/// card, and any Wallpaper card without a lattice).
struct CardSection<'a> {
    header: &'static str,
    cards: &'a [tess_core::CardMeta],
}

/// Split a card list into maximal runs of equal
/// [`tess_core::section_key`], in first-appearance order. On the
/// Wallpaper tab this yields the crystal-system sections
/// (oblique/rectangular/centered/square/hexagonal, in that order);
/// every other tab is a single headerless run.
fn sectionize<'a>(cards: &'a [tess_core::CardMeta]) -> Vec<CardSection<'a>> {
    let mut out: Vec<CardSection<'a>> = Vec::new();
    let mut i = 0;
    while i < cards.len() {
        let key = tess_core::section_key(&cards[i]);
        let mut j = i + 1;
        while j < cards.len() && tess_core::section_key(&cards[j]) == key {
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
/// columns fit in `width` — at least one (a very narrow window still
/// shows a single column) and at most four (wider cards beat more
/// columns). Pure, unit-tested.
fn columns_for_width(width: f32, card_w: f32) -> usize {
    let raw = if width.is_finite() && card_w.is_finite() && card_w > 0.0 {
        (width / card_w).floor() as usize
    } else {
        1
    };
    raw.clamp(1, 4)
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

/// Card-preview cache key: (id, params-string, size).
fn card_key(id: &str, params: &str, w: u32, h: u32) -> String {
    format!("{id}|{params}|{w}x{h}")
}

/// The tiling's default parameter values as a params-string. Card
/// previews always render the tiling at its default parameters (the
/// TUI cards do the same), so this is the key's params component.
fn default_params_str(specs: &[ParamSpec]) -> String {
    specs
        .iter()
        .map(|s| format!("{}={}", s.id.as_str(), s.default_value().display()))
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

impl GuiApp {
    pub fn new() -> Self {
        let app = tess_core::App::new(tess_tilings::registry(), 60.0);
        let tabs = tess_core::browse_tabs(app.registry());
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
            tabs,
            active_tab,
            tab_selected,
            search_at_jump: String::new(),
            last_highlighted: (active_tab, selected),
            card_textures: HashMap::new(),
            search: String::new(),
        }
    }

    /// The cached metadata for the current tiling + parameters, refreshing
    /// it on a miss. The cache key is the tiling id plus every current
    /// parameter value. The probe region is a tiny box at the origin,
    /// which every generator covers instantly (aperiodic patches grow
    /// from the origin) — so a refresh can never kick off expensive
    /// generation, and panning the view never touches this path.
    fn metadata(&mut self) -> &Meta {
        let mut key = self.app.selected_id().to_string();
        for spec in self.app.tiling().params() {
            let v = self
                .app
                .param_value(spec.id)
                .unwrap_or_else(|| spec.default_value());
            key.push_str(&format!("|{}={:?}", spec.id, v));
        }
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
            });
        });

        match self.mode {
            GuiMode::Browse => self.draw_browse(ctx),
            GuiMode::Render => self.draw_render(ctx),
        }
    }
}

impl GuiApp {
    /// Browse mode: the tab band (tabs + search, one wrapping row) over
    /// the card grid filling the window.
    fn draw_browse(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.search_jump();
            self.draw_tab_row(ui);
            ui.separator();
            egui::ScrollArea::vertical().id_salt("browser").show(ui, |ui| {
                self.draw_card_grid(ctx, ui);
            });
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
                });
            });
        });

        if self.params_open {
            egui::TopBottomPanel::bottom("render_params").show(ctx, |ui| {
                self.draw_params(ui);
                ui.separator();
                ui.label("Color mode");
                ui.horizontal(|ui| {
                    for m in [ColorMode::Single, ColorMode::Pattern, ColorMode::Proto] {
                        if ui
                            .selectable_label(self.app.color_mode == m, m.label())
                            .clicked()
                        {
                            self.app.color_mode = m;
                        }
                    }
                });
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
        self.app.select_id(id);
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
        self.app.select_id(id);
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
            self.app.select_id(id);
            self.tab_selected[idx] = Some(id.to_string());
        }
    }

    /// The card grid filling the window: crystal-system section
    /// headers (from `tess_core::section_key`) between rows of cards,
    /// each card = preview image + bold name + "group · lattice" line
    /// + truncated description. Responsive column count ([`
    /// columns_for_width`]) with the cards stretched to fill.
    fn draw_card_grid(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
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
        // Responsive columns over the full window width ([`
        // columns_for_width`], clamped 1..=4); the cards stretch to
        // the column width (minus the grid's horizontal spacing) so
        // the grid fills the window edge to edge.
        let avail = ui.available_width();
        let cols = columns_for_width(avail, CARD_TARGET_W);
        let col_w = (avail - (cols - 1) as f32 * GRID_SPX) / cols as f32;
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
            for section in sectionize(&cards) {
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
                        self.draw_card(ctx, ui, card, col_w, changed);
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

    /// One card: bordered frame with the preview texture (generated
    /// lazily, visible cards only), the bold name, the "group ·
    /// lattice" symmetry line, and a truncated description. The frame
    /// is `col_w` wide (the responsive column width); clicking selects
    /// the tiling and opens Render mode; the selected card gets a
    /// highlight stroke.
    fn draw_card(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        card: &tess_core::CardMeta,
        col_w: f32,
        scroll_to_selected: bool,
    ) {
        let selected = card.id == self.app.selected_id();
        let img_size = egui::vec2(CARD_IMG_W as f32, CARD_IMG_H as f32);
        // Lazy texture generation: the image allocates at the cursor;
        // generate (or reuse) the texture only when that rect is near
        // the scroll area's clip rect, so the first frame pays only
        // for the cards actually in view.
        let predicted = egui::Rect::from_min_size(ui.cursor().min + egui::vec2(5.0, 5.0), img_size);
        let handle = if ui.clip_rect().expand(32.0).intersects(predicted) {
            Some(self.card_texture(ctx, card, CARD_IMG_W, CARD_IMG_H))
        } else {
            None
        };
        let frame = egui::Frame::none()
            .fill(if selected { SELECTED_FILL } else { egui::Color32::TRANSPARENT })
            .rounding(6.0)
            .inner_margin(5.0)
            .stroke(egui::Stroke::new(
                if selected { 2.0_f32 } else { 1.0_f32 },
                if selected { SELECTED_STROKE } else { CARD_STROKE },
            ));
        let frame_resp = frame.show(ui, |ui| {
            ui.set_width(col_w);
            match &handle {
                Some(h) => {
                    ui.add(egui::Image::from_texture(h).fit_to_exact_size(img_size));
                }
                None => {
                    ui.allocate_exact_size(img_size, egui::Sense::hover());
                }
            }
            ui.add(egui::Label::new(egui::RichText::new(card.name.to_string()).strong()));
            ui.add(egui::Label::new(egui::RichText::new(format!(
                "{} · {}",
                card.group_str(),
                card.lattice_str()
            ))
            .small()
            .color(SYM_COLOR)));
            let desc = if card.description.is_empty() {
                card.notes.unwrap_or("").to_string()
            } else {
                truncate_text(card.description, DESC_CHARS)
            };
            if !desc.is_empty() {
                ui.add(egui::Label::new(egui::RichText::new(desc).small().color(DESC_COLOR)));
            }
        });
        let card_rect = frame_resp.response.rect;
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
    /// Rasterization uses the shared `tess_core::raster` with the
    /// identity palette, so pixel indices are positions in
    /// `paint::PALETTE` (full 8-bit colors; the TUI maps the same
    /// indices to xterm-256). Colors follow the tiling's default
    /// preview mode (proto fallback for `Single`, which would render a
    /// featureless slab) — the TUI thumb semantics.
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
        let params_str = default_params_str(tiling.params());
        let key = card_key(card.id, &params_str, w, h);
        if let Some(t) = self.card_textures.get(&key) {
            return t.clone();
        }
        let params = ParamSet::default_for(tiling.params());
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
        let mut result = tiling.generate(&params, &tess_core::Query { region: window });
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
            result = tiling.generate(&params, &tess_core::Query { region: window });
            shrink *= 0.4;
        }
        let mode = tiling.default_color_mode().preview_mode();
        let raster = if result.tiles.is_empty() {
            None
        } else {
            Some(tess_core::raster::raster(
                &result.tiles,
                &window,
                w,
                h,
                mode,
                &[0, 1, 2, 3, 4, 5, 6, 7],
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
                    i => paint::rgb(paint::PALETTE[i as usize]),
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
        if response.dragged() {
            let d = response.drag_delta();
            self.app.pan(d.x as f64, d.y as f64);
        }

        let w = rect.width().max(1.0) as u32;
        let h = rect.height().max(1.0) as u32;
        let result = self.app.result(w, h);

        let (br, bg_, bb) = paint::rgb(paint::BORDER);
        let (ar, ag, ab) = paint::rgb(paint::BACKGROUND);
        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(ar, ag, ab));

        if let Some(msg) = result.message {
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
            painter.add(egui::Shape::closed_line(screen, border));
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

    use super::layout_fingerprint;

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
        let mut gui = super::GuiApp::new();
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
        }
    }

    /// Search matches on name, id, or description, case-insensitively;
    /// an empty (whitespace) query matches everything.
    #[test]
    fn search_matches_fields_and_empty_query() {
        let tabs = tess_core::browse_tabs(&tess_tilings::registry());
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
            card_fix("r1", wp, Some(p1), Some(tess_core::LatticeKind::Rectangular)),
            card_fix("c1", wp, Some(cmm), Some(tess_core::LatticeKind::Rectangular)),
            card_fix("r2", wp, Some(p1), Some(tess_core::LatticeKind::Rectangular)),
            card_fix("s1", wp, Some(p1), Some(tess_core::LatticeKind::Square)),
            card_fix("h1", wp, Some(p1), Some(tess_core::LatticeKind::Hexagonal)),
            card_fix(
                "n1",
                tess_core::TilingCategory::Uniform,
                None,
                None,
            ),
        ];
        let sections = super::sectionize(&cards);
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
        let tabs = tess_core::browse_tabs(&tess_tilings::registry());
        let wp = tabs
            .iter()
            .find(|t| t.category == tess_core::TilingCategory::Wallpaper)
            .expect("wallpaper tab");
        assert!(!wp.cards.is_empty(), "wallpaper tab must not be empty");
        let sections = super::sectionize(&wp.cards);
        let mut covered = 0usize;
        for s in &sections {
            assert!(!s.cards.is_empty());
            if !s.header.is_empty() {
                assert_eq!(
                    s.header,
                    tess_core::section_key(&s.cards[0]),
                    "header must come from the section's first card"
                );
            }
            covered += s.cards.len();
        }
        assert_eq!(covered, wp.cards.len());
        // Crystal-system order: no section may re-appear before a later
        // crystal system (oblique before rectangular before square
        // before hexagonal).
        let rank = |h: &str| match h {
            "" => 9,
            "oblique" => 0,
            "rectangular" => 1,
            "centered" => 1,
            "square" => 2,
            "hexagonal" => 3,
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

    /// The texture cache key distinguishes (id, params, size).
    #[test]
    fn card_key_distinguishes_id_params_size() {
        assert_eq!(
            super::card_key("chair", "scale=1", 120, 90),
            "chair|scale=1|120x90"
        );
        assert_ne!(super::card_key("chair", "scale=1", 120, 90), super::card_key("chair", "scale=2", 120, 90));
        assert_ne!(super::card_key("chair", "scale=1", 120, 90), super::card_key("chair", "scale=1", 121, 90));
        assert_ne!(super::card_key("chair", "scale=1", 120, 90), super::card_key("other", "scale=1", 120, 90));
    }

    /// The params component of the cache key is the tiling's default
    /// values in declaration order — pinned so the preview cache key is
    /// stable across runs (the determinism contract).
    #[test]
    fn default_params_str_pinned() {
        let registry = tess_tilings::registry();
        let star = registry.get("star-cross").expect("star-cross registered");
        let specs = star.params();
        assert_eq!(specs.len(), 3);
        assert_eq!(
            specs.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(),
            vec!["scale", "rotation", "star_radius"]
        );
        let got = super::default_params_str(specs);
        assert_eq!(got, "scale=1,rotation=0°,star_radius=0.50");
        assert_eq!(got, super::default_params_str(specs), "deterministic");
        // A parameterless declaration yields the empty params-string.
        assert_eq!(super::default_params_str(&[]), "");
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

    /// `columns_for_width` fits whole target-width cards, clamps to
    /// 1..=4, and degrades to a single column on degenerate widths.
    #[test]
    fn columns_for_width_fits_and_clamps() {
        assert_eq!(super::columns_for_width(1000.0, 240.0), 4, "4.16 → 4");
        assert_eq!(super::columns_for_width(960.0, 240.0), 4, "exactly 4");
        assert_eq!(super::columns_for_width(959.0, 240.0), 3, "just under 4");
        assert_eq!(super::columns_for_width(480.0, 240.0), 2, "exactly 2");
        assert_eq!(super::columns_for_width(479.0, 240.0), 1);
        assert_eq!(super::columns_for_width(239.0, 240.0), 1, "lower clamp");
        assert_eq!(super::columns_for_width(0.0, 240.0), 1, "zero width");
        assert_eq!(super::columns_for_width(100_000.0, 240.0), 4, "upper clamp");
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

    /// Render-mode prev/next walks the selected tiling's tab order and
    /// updates the per-tab selection memory, wrapping — so returning
    /// to Browse lands on the walked card (mirrors the TUI's `walk`).
    /// Also pins the mode switch on card selection.
    #[test]
    fn walk_render_walks_tab_order_and_updates_memory() {
        let mut gui = super::GuiApp::new();
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
}
