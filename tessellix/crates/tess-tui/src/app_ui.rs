//! TUI application: two-mode browser over the shared
//! [`tess_core::browse`] model, plus event loop and rendering.
//!
//! * **Browse** — a category tab bar over a responsive card grid.
//!   Wallpaper cards are grouped into crystal-system sections (oblique,
//!   rectangular, centered, square, hexagonal) with a bold header row
//!   above each section; the Wallpaper tab also opens with a catalog
//!   note (since M85 all 17 groups are represented, it reports the
//!   completed catalog); the grid scrolls by rows.
//! * **Render** — the selected tiling full-screen (one cell = one
//!   pixel, as before) with a status bar; `←`/`→` walk prev/next tiling
//!   in the current tab's order (the browse cursor follows, so `Esc`
//!   returns to exactly the right card); `p` toggles a params overlay
//!   strip at the bottom.
//! * **Search** — `/` filters cards by name/id/description across all
//!   tabs; the tab bar shows per-tab match counts and the active tab
//!   jumps to the first tab with a match.
//!
//! Mouse: left-click a tab to switch, left-click a card to select +
//! open Render, wheel scrolls the grid (Browse) or zooms (Render).
//! Hit-testing uses the card/tab rects stored during the last render;
//! [`tab_at`] / [`card_at`] are pure fns unit-tested without mouse
//! events (TestBackend cannot synthesize them).
//!
//! The dirty-flag render loop is kept: only a dirty frame regenerates
//! anything (the full tiling in Render, or ≤ one page of cached card
//! previews in Browse).

use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::event::{
    self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, Terminal};

use tess_core::app::App;
use tess_core::browse::{self, CardMeta, Tab};
use tess_core::category::TilingCategory;
use tess_core::paint;
use tess_core::params::ParamSet;

use crate::raster;
use crate::thumb;

/// Target card size: ~26–30 columns × 12–14 rows (a two-line title
/// needs one extra row); the grid fits as many full-size cards as the
/// viewport allows (≥ 1 column, ≥ 1 row) and stretches them to fill.
const TARGET_CARD_W: u16 = 28;
/// Card-size bounds for the +/- keys and ctrl/shift+wheel (the
/// height follows at half the width; the default card is 28×14, so
/// the height target is `TARGET_CARD_W/2`).
const MIN_CARD_W: u16 = 22;
const MAX_CARD_W: u16 = 44;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mode {
    Browse,
    Render,
    Search,
}

/// One row of the card grid plan: either a crystal-system section
/// header (or the Wallpaper tab's catalog note) or one row of cards
/// (indices into the tab's card list).
#[derive(Debug)]
enum GridRow {
    Header(String),
    Cards(Vec<usize>),
}

struct Tui {
    app: App,
    tabs: Vec<Tab>,
    /// Active tab (Browse / Search) and the tab prev/next walks in
    /// Render (the selected tiling's own category).
    tab_idx: usize,
    /// Per-tab card cursor, indexed by [`TilingCategory::ALL`] position.
    cursors: [usize; 10],
    /// Per-tab grid scroll: index of the topmost grid row, same
    /// indexing. Clamped against the rendered plan on every draw.
    scroll: [usize; 10],
    mode: Mode,
    search: String,
    /// Params overlay open in Render mode.
    params_open: bool,
    palette256: Vec<u8>,
    border256: u8,
    gap256: u8,
    /// Last rendered canvas size; pans scale with it.
    canvas_size: (u32, u32),
    /// Last rendered grid geometry (for cursor steps and paging).
    grid_cols: u16,
    grid_rows: u16,
    /// Last `(tab, highlighted card)` the grid auto-scrolled for: the
    /// grid snaps to the highlight only when it moves, so manual
    /// PgUp/PgDn/wheel scrolling is not undone on the next render.
    last_highlighted: (usize, Option<usize>),
    thumbs: thumb::CardCache,
    dirty: bool,
    /// Hit-test geometry from the last Browse render.
    tab_rects: Vec<Rect>,
    card_rects: Vec<(Rect, usize)>,
    /// Target card width for the browse grid (the height follows at
    /// half the width, keeping the 28×14 default aspect). Adjusted by
    /// Ctrl+wheel in steps of 2 within [22, 44]; plain wheel keeps
    /// scrolling.
    card_w: u16,
}

impl Tui {
    fn new() -> Self {
        let registry = tess_tilings::registry();
        // The classification table (tess-tilings::classification)
        // supplies each card's declared schema and drives the declared
        // tab sort orders (pentagon types, 2-uniform configurations,
        // Laves ranks, wallpaper crystal systems).
        let tabs = browse::browse_tabs_with(&registry, tess_tilings::classification);
        let palette256: Vec<u8> = paint::PALETTE
            .iter()
            .map(|&c| paint::rgb_to_256(c))
            .collect();
        Self {
            app: App::new(registry, 8.0),
            tabs,
            tab_idx: 0,
            cursors: [0; 10],
            scroll: [0; 10],
            mode: Mode::Browse,
            search: String::new(),
            params_open: false,
            palette256,
            border256: paint::rgb_to_256(paint::BORDER),
            gap256: 124,
            canvas_size: (0, 0),
            grid_cols: 1,
            grid_rows: 1,
            thumbs: thumb::CardCache::new(),
            dirty: true,
            tab_rects: Vec::new(),
            card_rects: Vec::new(),
            card_w: TARGET_CARD_W,
            last_highlighted: (0, None),
        }
    }

    fn active_tab(&self) -> &Tab {
        &self.tabs[self.tab_idx]
    }

    fn cat_idx(&self, c: TilingCategory) -> usize {
        TilingCategory::ALL.iter().position(|a| *a == c).expect("ALL variant")
    }

    fn switch_tab(&mut self, delta: i32) {
        let n = self.tabs.len() as i32;
        self.tab_idx = ((self.tab_idx as i32) + delta).rem_euclid(n) as usize;
        self.dirty = true;
    }

    fn switch_tab_to(&mut self, idx: usize) {
        if idx < self.tabs.len() {
            self.tab_idx = idx;
            self.dirty = true;
        }
    }

    /// Move the browse cursor by `step` (±1 horizontally, ±cols
    /// vertically), wrapping within the active tab's card list.
    fn move_cursor(&mut self, step: i32) {
        let n = self.active_tab().cards.len() as i32;
        if n == 0 {
            return;
        }
        let ci = self.cat_idx(self.active_tab().category);
        let cur = (self.cursors[ci] as i32).rem_euclid(n);
        self.cursors[ci] = ((cur + step).rem_euclid(n)) as usize;
        self.dirty = true;
    }

    fn scroll_by(&mut self, step: i32) {
        let ci = self.cat_idx(self.active_tab().category);
        self.scroll[ci] = (self.scroll[ci] as i32 + step).max(0) as usize;
        self.dirty = true;
    }

    /// Card-size step: the target card width moves by `delta`
    /// (positive = larger cards), clamped to
    /// [`MIN_CARD_W, MAX_CARD_W`]; the height follows at half the
    /// width so the default 28×14 aspect is preserved. Resizing
    /// invalidates the thumbnail cache through its size-keyed entries.
    /// Bound to +/- in Browse and to ctrl/shift+wheel where the
    /// terminal reports wheel modifiers.
    fn resize_cards(&mut self, delta: i32) {
        let w = (self.card_w as i32 + delta).clamp(MIN_CARD_W as i32, MAX_CARD_W as i32);
        if w != self.card_w as i32 {
            self.card_w = w as u16;
            self.dirty = true;
        }
    }

    /// Cards of the active tab passing the search filter, in tab order.
    fn visible_cards(&self) -> Vec<usize> {
        let q = self.search.trim().to_lowercase();
        (0..self.active_tab().cards.len())
            .filter(|&i| q.is_empty() || matches_card(&self.active_tab().cards[i], &q))
            .collect()
    }

    /// The card the Browse grid highlights: the cursor's card when the
    /// search filter is empty (or the cursor's card still matches),
    /// otherwise the first match in the active tab.
    fn highlighted_card(&self) -> Option<usize> {
        let tab = self.active_tab();
        if tab.cards.is_empty() {
            return None;
        }
        let q = self.search.trim().to_lowercase();
        if q.is_empty() {
            return Some(self.cursors[self.cat_idx(tab.category)].min(tab.cards.len() - 1));
        }
        let cur = self.cursors[self.cat_idx(tab.category)];
        if cur < tab.cards.len() && matches_card(&tab.cards[cur], &q) {
            return Some(cur);
        }
        tab.cards.iter().position(|c| matches_card(c, &q))
    }

    /// Match counts per tab for the current search query.
    fn match_counts(&self) -> Vec<usize> {
        let q = self.search.trim().to_lowercase();
        self.tabs
            .iter()
            .map(|t| {
                if q.is_empty() {
                    t.cards.len()
                } else {
                    t.cards.iter().filter(|c| matches_card(c, &q)).count()
                }
            })
            .collect()
    }

    /// After a search keystroke: if the active tab has no match but
    /// some tab does, jump to the first matching tab (canonical order).
    fn apply_search_jump(&mut self) {
        let counts = self.match_counts();
        if counts[self.tab_idx] == 0 && counts.iter().any(|&c| c > 0) {
            self.tab_idx = counts.iter().position(|&c| c > 0).expect("checked");
            self.scroll[self.cat_idx(self.active_tab().category)] = 0;
        }
    }

    /// Select a card and enter the full-screen Render view. The browse
    /// cursor is kept in sync so `Esc` returns to this card.
    fn open_render(&mut self, card_idx: usize) {
        let id = match self.active_tab().cards.get(card_idx) {
            Some(c) => c.id,
            None => return,
        };
        let cat = self.active_tab().category;
        self.app.select_id(id);
        self.cursors[self.cat_idx(cat)] = card_idx;
        self.mode = Mode::Render;
        self.params_open = false;
        self.dirty = true;
    }

    /// Prev/next tiling in the selected tiling's own category tab
    /// (the tab order), updating the browse cursor so `Esc` lands on
    /// the walked card.
    fn walk(&mut self, dir: i32) {
        let ci = self.cat_idx(self.app.tiling().category());
        let tab = &self.tabs[ci];
        let n = tab.cards.len() as i32;
        if n == 0 {
            return;
        }
        let cur = tab.card_index_of(self.app.selected_id()).unwrap_or(0) as i32;
        let next = ((cur + dir).rem_euclid(n)) as usize;
        let id = tab.cards[next].id;
        self.cursors[ci] = next;
        self.app.select_id(id);
        self.dirty = true;
    }

    fn pan(&mut self, dir_x: i32, dir_y: i32) {
        let (w, h) = self.canvas_size;
        // pan_screen drags the content by (dx, dy) screen pixels (y
        // down); negate both so the keys reveal in the direction they
        // name (h left, l right, j down, k up — vim scroll).
        self.app.pan(
            -dir_x as f64 * w as f64 * 0.3,
            -dir_y as f64 * h as f64 * 0.3,
        );
        self.dirty = true;
    }

    fn handle_key(&mut self, code: KeyCode, mods: KeyModifiers) -> bool {
        // Raw mode disables ISIG, so SIGINT never fires: Ctrl-C/Ctrl-Q
        // arrive as plain 'c'/'q' with CONTROL and would cycle color or
        // type. Treat them as quit.
        if mods.contains(KeyModifiers::CONTROL)
            && matches!(code, KeyCode::Char('c') | KeyCode::Char('q'))
        {
            return false;
        }
        match self.mode {
            Mode::Search => match code {
                KeyCode::Esc => {
                    self.search.clear();
                    self.mode = Mode::Browse;
                    self.dirty = true;
                }
                KeyCode::Enter => {
                    if let Some(i) = self.highlighted_card() {
                        self.open_render(i);
                    } else {
                        self.search.clear();
                        self.mode = Mode::Browse;
                        self.dirty = true;
                    }
                }
                KeyCode::Backspace => {
                    self.search.pop();
                    self.apply_search_jump();
                    self.dirty = true;
                }
                KeyCode::Tab => self.switch_tab(1),
                KeyCode::BackTab => self.switch_tab(-1),
                // Digits are reserved for tab jumps in search too.
                KeyCode::Char(c @ '1'..='9') => self.switch_tab_to((c as usize) - ('1' as usize)),
                KeyCode::Char('0') => self.switch_tab_to(9),
                KeyCode::Char(c) if c.is_ascii_graphic() || c == ' ' => {
                    self.search.push(c);
                    self.apply_search_jump();
                    self.dirty = true;
                }
                _ => {}
            },
            Mode::Browse => match code {
                KeyCode::Char('q') => return false,
                KeyCode::Char('/') => {
                    self.search.clear();
                    self.mode = Mode::Search;
                    self.dirty = true;
                }
                KeyCode::Tab => self.switch_tab(1),
                KeyCode::BackTab => self.switch_tab(-1),
                KeyCode::Char(c @ '1'..='9') => self.switch_tab_to((c as usize) - ('1' as usize)),
                KeyCode::Char('0') => self.switch_tab_to(9),
                // Card-grid size: +/- grows/shrinks the browse cards
                // (the height follows at half the width). The same
                // keys zoom the canvas in Render; ctrl/shift+wheel
                // does the same here, where the terminal reports
                // modifiers on wheel events.
                KeyCode::Char('+') | KeyCode::Char('=') => self.resize_cards(2),
                KeyCode::Char('-') | KeyCode::Char('_') => self.resize_cards(-2),
                KeyCode::Up | KeyCode::Char('k') => self.move_cursor(-(self.grid_cols as i32)),
                KeyCode::Down | KeyCode::Char('j') => self.move_cursor(self.grid_cols as i32),
                KeyCode::Left | KeyCode::Char('h') => self.move_cursor(-1),
                KeyCode::Right | KeyCode::Char('l') => self.move_cursor(1),
                KeyCode::Enter | KeyCode::Char(' ') => {
                    if let Some(i) = self.highlighted_card() {
                        self.open_render(i);
                    }
                }
                KeyCode::PageUp => self.scroll_by(-(self.grid_rows as i32)),
                KeyCode::PageDown => self.scroll_by(self.grid_rows as i32),
                _ => {}
            },
            Mode::Render => match code {
                KeyCode::Esc | KeyCode::Char('b') => {
                    self.mode = Mode::Browse;
                    self.params_open = false;
                    self.dirty = true;
                }
                KeyCode::Char('p') => {
                    if !self.app.tiling().params().is_empty() {
                        self.params_open = !self.params_open;
                        self.dirty = true;
                    }
                }
                KeyCode::Left => {
                    if self.params_open {
                        self.app.adjust_param(-1);
                    } else {
                        self.walk(-1);
                    }
                }
                KeyCode::Right => {
                    if self.params_open {
                        self.app.adjust_param(1);
                    } else {
                        self.walk(1);
                    }
                }
                KeyCode::Up if self.params_open => self.app.move_param_cursor(-1),
                KeyCode::Down if self.params_open => self.app.move_param_cursor(1),
                KeyCode::Char('h') => self.pan(-1, 0),
                KeyCode::Char('l') => self.pan(1, 0),
                KeyCode::Char('j') => self.pan(0, 1),
                KeyCode::Char('k') => self.pan(0, -1),
                KeyCode::Char('+') | KeyCode::Char('=') => {
                    self.app.zoom(1.3, None);
                    self.dirty = true;
                }
                KeyCode::Char('-') | KeyCode::Char('_') => {
                    self.app.zoom(1.0 / 1.3, None);
                    self.dirty = true;
                }
                KeyCode::Char('c') => {
                    self.app.cycle_color_mode();
                    self.dirty = true;
                }
                KeyCode::Char('r') => {
                    self.app.reset_view();
                    self.dirty = true;
                }
                _ => {}
            },
        }
        true
    }

    /// Mouse events. Hit-testing uses the rects from the last render;
    /// TestBackend cannot synthesize mouse events, so the hit tests
    /// themselves are covered by the pure [`tab_at`]/[`card_at`].
    fn handle_mouse(&mut self, me: MouseEvent) {
        match self.mode {
            Mode::Browse => match me.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    if let Some(i) = tab_at(&self.tab_rects, me.column, me.row) {
                        self.switch_tab_to(i);
                    } else if let Some(i) = card_at(&self.card_rects, me.column, me.row) {
                        self.open_render(i);
                    }
                }
                // Resize on ctrl/shift+wheel: terminals that report wheel
                // modifiers let both through; terminals that swallow
                // them (some intercept ctrl+wheel for their own zoom)
                // still have the +/- keys.
                MouseEventKind::ScrollUp
                    if me
                        .modifiers
                        .intersects(KeyModifiers::CONTROL | KeyModifiers::SHIFT) =>
                {
                    self.resize_cards(2);
                }
                MouseEventKind::ScrollDown
                    if me
                        .modifiers
                        .intersects(KeyModifiers::CONTROL | KeyModifiers::SHIFT) =>
                {
                    self.resize_cards(-2);
                }
                MouseEventKind::ScrollUp => self.scroll_by(-1),
                MouseEventKind::ScrollDown => self.scroll_by(1),
                _ => {}
            },
            Mode::Render => match me.kind {
                MouseEventKind::ScrollUp => {
                    self.app.zoom(1.3, None);
                    self.dirty = true;
                }
                MouseEventKind::ScrollDown => {
                    self.app.zoom(1.0 / 1.3, None);
                    self.dirty = true;
                }
                _ => {}
            },
            Mode::Search => {}
        }
    }
}

/// Which tab rect contains (x, y)? Pure hit-test (unit-tested).
fn tab_at(rects: &[Rect], x: u16, y: u16) -> Option<usize> {
    rects
        .iter()
        .position(|r| r.contains(Position { x, y }))
}

/// Which card (index into its tab's card list) contains (x, y)?
/// Pure hit-test (unit-tested).
fn card_at(rects: &[(Rect, usize)], x: u16, y: u16) -> Option<usize> {
    rects
        .iter()
        .find(|(r, _)| r.contains(Position { x, y }))
        .map(|(_, i)| *i)
}

/// Case-insensitive card match on (name, id, description).
fn matches_card(c: &CardMeta, q: &str) -> bool {
    c.id.to_lowercase().contains(q)
        || c.name.to_lowercase().contains(q)
        || c.description.to_lowercase().contains(q)
}

/// Truncate to `max_chars` characters, appending `…` when cut.
fn truncate(s: &str, max_chars: usize) -> String {
    let n = s.chars().count();
    if n <= max_chars {
        return s.to_string();
    }
    let mut t: String = s.chars().take(max_chars.saturating_sub(1)).collect();
    t.push('…');
    t
}

/// Split a card title into at most two lines of at most `per_line`
/// characters each: the whole name when it fits one line, otherwise
/// the name broken at the last word boundary inside the first budget
/// (a bare character cut only when no boundary is reachable), so
/// line two never starts mid-word. Each line keeps the
/// label-level truncation as a backstop, so a name longer than
/// 2 × `per_line` still fits its card.
fn title_lines(name: &str, per_line: usize) -> Vec<String> {
    if per_line == 0 {
        return Vec::new();
    }
    if name.chars().count() <= per_line {
        return vec![name.to_string()];
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
    vec![first, truncate(&rest, per_line)]
}

/// Card grid geometry for a viewport: as many target-size cards as fit
/// (≥ 1 column, ≥ 1 row), stretched to fill. The card height target is
/// half the width target (the default 28×14 aspect).
fn grid_cols_rows(area: Rect, card_w: u16) -> (u16, u16, u16, u16) {
    let card_h = (card_w / 2).max(6);
    let cols = (area.width / card_w).max(1);
    let rows = (area.height / card_h).max(1);
    let cw = (area.width / cols).max(1);
    let ch = (area.height / rows).max(1);
    (cols, cw, rows, ch)
}

/// Plan the grid rows for the visible card indices of a tab: runs of
/// equal section key, each preceded by a header row when the key is
/// non-empty, split into rows of `cols` cards. The Wallpaper tab
/// opens with the explicit catalog note: while any wallpaper group
/// lacked a realizer it listed them, and since M85 (all 17 groups
/// represented) it reports the completed catalog.
fn plan_grid_rows(tab: &Tab, visible: &[usize], cols: u16) -> Vec<GridRow> {
    let mut rows = Vec::new();
    if tab.category == TilingCategory::Wallpaper && !visible.is_empty() {
        rows.push(GridRow::Header(browse::wallpaper_gap_note()));
    }
    let mut i = 0;
    while i < visible.len() {
        let key = browse::section_key(tab.category, &tab.cards[visible[i]]);
        let mut j = i;
        while j < visible.len()
            && browse::section_key(tab.category, &tab.cards[visible[j]]) == key
        {
            j += 1;
        }
        if !key.is_empty() {
            rows.push(GridRow::Header(key.to_string()));
        }
        for chunk in visible[i..j].chunks(cols as usize) {
            rows.push(GridRow::Cards(chunk.to_vec()));
        }
        i = j;
    }
    rows
}

pub fn run() -> io::Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    crossterm::terminal::enable_raw_mode()?;
    io::stdout().execute(crossterm::terminal::EnterAlternateScreen)?;
    io::stdout().execute(crossterm::event::EnableMouseCapture)?;
    let result = run_loop(&mut terminal);
    io::stdout().execute(crossterm::event::DisableMouseCapture)?;
    io::stdout().execute(crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    result
}

fn run_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    let mut tui = Tui::new();
    loop {
        // Draw only when something changed: render() regenerates the
        // whole tiling (Render) or a page of card previews (Browse), so
        // an idle window must not spin on it. Every state change (key,
        // mouse, resize) sets `dirty`; render clears it.
        if tui.dirty {
            terminal.draw(|f| render(f, &mut tui))?;
        }
        if !event::poll(Duration::from_millis(60))? {
            continue;
        }
        match event::read()? {
            Event::Key(k) if k.kind == KeyEventKind::Press => {
                if !tui.handle_key(k.code, k.modifiers) {
                    break;
                }
            }
            Event::Mouse(me) => tui.handle_mouse(me),
            Event::Resize(_, _) => tui.dirty = true,
            _ => {}
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, tui: &mut Tui) {
    let area = frame.area();
    match tui.mode {
        Mode::Render => render_render_view(frame, tui, area),
        Mode::Browse | Mode::Search => render_browse(frame, tui, area),
    }
    tui.dirty = false;
}

// -------------------------------------------------------------------
// Browse mode: [tab bar] / [card grid] / [status bar]
// -------------------------------------------------------------------

fn render_browse(frame: &mut Frame, tui: &mut Tui, area: Rect) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);
    render_tab_bar(frame, tui, outer[0]);
    render_card_grid(frame, tui, outer[1]);
    render_status_browse(frame, tui, outer[2]);
}

fn render_tab_bar(frame: &mut Frame, tui: &mut Tui, rect: Rect) {
    let counts = tui.match_counts();
    let mut spans = Vec::with_capacity(tui.tabs.len());
    tui.tab_rects.clear();
    let mut x = 0u16;
    for (i, tab) in tui.tabs.iter().enumerate() {
        let text = format!(" {} ({}) ", tab.category.label(), counts[i]);
        let w = text.chars().count() as u16;
        let style = if i == tui.tab_idx {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::REVERSED)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        spans.push(Span::raw(text).style(style));
        tui.tab_rects.push(Rect::new(rect.x + x, rect.y, w, 1));
        x += w;
    }
    frame
        .render_widget(Paragraph::new(Line::from(spans)), rect);
}

/// Height of one planned grid row in terminal rows.
fn row_height(row: &GridRow, card_h: u16) -> u16 {
    match row {
        GridRow::Header(_) => 1,
        GridRow::Cards(_) => card_h,
    }
}

/// How many consecutive plan rows fit below offset `off`.
fn visible_rows(plan: &[GridRow], off: usize, card_h: u16, height: u16) -> usize {
    let mut n = 0;
    let mut used = 0usize;
    for row in plan.iter().skip(off) {
        let rh = row_height(row, card_h) as usize;
        if used + rh > height as usize {
            break;
        }
        used += rh;
        n += 1;
    }
    n
}

/// Largest offset at which plan row `r` still renders in full: walk
/// backwards from `r` while the rows above still fit alongside `r`
/// itself (row `r` must fully occupy its own slot).
fn max_offset_for_row(plan: &[GridRow], r: usize, card_h: u16, height: u16) -> usize {
    let mut off = r;
    let mut used = row_height(&plan[r], card_h) as usize;
    while off > 0 {
        let rh = row_height(&plan[off - 1], card_h) as usize;
        if used + rh > height as usize {
            break;
        }
        used += rh;
        off -= 1;
    }
    off
}

fn render_card_grid(frame: &mut Frame, tui: &mut Tui, rect: Rect) {
    let (cols, card_w, _rows, card_h) = grid_cols_rows(rect, tui.card_w);
    tui.grid_cols = cols;
    let ci = tui.cat_idx(tui.active_tab().category);
    let visible = tui.visible_cards();
    let plan = plan_grid_rows(tui.active_tab(), &visible, cols);
    if plan.is_empty() {
        tui.scroll[ci] = 0;
        return;
    }
    // Keep the highlighted card in view — but only when the highlight
    // actually moved since the last draw, so manual PgUp/PgDn/wheel
    // scrolling is not snapped back to the cursor on every render.
    let hl = tui.highlighted_card();
    if tui.last_highlighted != (tui.tab_idx, hl) {
        tui.last_highlighted = (tui.tab_idx, hl);
        if let Some(h) = hl {
            let r = plan.iter().position(|row| match row {
                GridRow::Cards(idxs) => idxs.contains(&h),
                _ => false,
            })
            .unwrap_or(0);
            let off = tui.scroll[ci];
            tui.scroll[ci] = if r < off {
                r
            } else if r >= off.saturating_add(visible_rows(&plan, off, card_h, rect.height)) {
                max_offset_for_row(&plan, r, card_h, rect.height)
            } else {
                off
            };
        }
    }
    let off = tui.scroll[ci].min(plan.len().saturating_sub(1));

    tui.card_rects.clear();
    let mut y = rect.y;
    for row in plan[off..].iter() {
        let row_h = match row {
            GridRow::Header(_) => 1,
            GridRow::Cards(_) => card_h,
        };
        if y + row_h > rect.y + rect.height {
            break;
        }
        match row {
            GridRow::Header(key) => {
                let line = Line::from(Span::raw(format!("  {key}")).style(
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ));
                frame.render_widget(
                    Paragraph::new(line),
                    Rect::new(rect.x, y, rect.width, 1),
                );
            }
            GridRow::Cards(idxs) => {
                let hl = tui.highlighted_card();
                for (k, &card_i) in idxs.iter().enumerate() {
                    let crect = Rect::new(
                        rect.x + (k as u16) * card_w,
                        y,
                        card_w,
                        card_h,
                    );
                    tui.card_rects.push((crect, card_i));
                    render_card(frame, tui, crect, card_i, Some(card_i) == hl);
                }
            }
        }
        y += row_h;
    }
}

fn render_card(
    frame: &mut Frame,
    tui: &mut Tui,
    rect: Rect,
    card_idx: usize,
    selected: bool,
) {
    // Extract the card data first: the thumb cache lookup below takes
    // `&mut tui` and must not overlap a borrow of `tui.tabs`.
    // CardMeta is Copy, so the whole card copies out.
    let card: CardMeta = tui.active_tab().cards[card_idx];
    let (id, nparams) = (card.id, card.params);
    let border_style = if selected {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::REVERSED)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let iw = rect.width.saturating_sub(2) as usize;
    // The title: up to two lines (a long name wraps at the inner
    // width). The first line is the block title on the top border
    // (centered, as before); a wrapped second line is painted as the
    // first inner row, so the content starts under it.
    let tlines = title_lines(card.name, iw);
    let title_extra = tlines.len().saturating_sub(1);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(tlines[0].as_str());
    frame.render_widget(block, rect);
    if title_extra > 0 {
        frame.render_widget(
            Paragraph::new(truncate(&tlines[1], iw)).alignment(Alignment::Center),
            Rect::new(rect.x + 1, rect.y + 1, iw as u16, 1),
        );
    }

    let ih = rect.height.saturating_sub(2) as usize;
    if iw == 0 || ih == 0 {
        return;
    }
    let has_params = nparams > 0;
    // Attribute rows: the symmetry line (group · lattice for periodic
    // cards; the declared translational character for the nonperiodic
    // world — never a bare "— · —"), then the schema summary line
    // wrapped over up to two rows, then "params: n" when parameterized.
    let attr_rows = 3 + usize::from(has_params);
    let content = ih.saturating_sub(title_extra);
    let thumb_rows = content.saturating_sub(attr_rows).max(1);

    // Thumbnail: the tiling at its default params, in its default
    // preview color mode (proto fallback for `Single`). Cached per
    // (id, params, size); the first page of a tab generates ≤ one
    // screenful of small previews, never the whole tab.
    let t = tui
        .app
        .registry()
        .get(id)
        .expect("card id is registered");
    let params = ParamSet::default_for(t.params());
    let pv = tui.thumbs.get_or_generate(
        t,
        &params,
        &tui.palette256,
        iw as u32,
        thumb_rows as u32,
    );
    let map = |c: u8| match c {
        raster::BORDER => Color::Indexed(tui.border256),
        raster::GAP => Color::Indexed(tui.gap256),
        c => Color::Indexed(c),
    };
    let buf = frame.buffer_mut();
    let thumb = &pv.thumb;
    for r in 0..thumb_rows.min(thumb.rows.len()) {
        for x in 0..iw.min(thumb.w as usize) {
            let (top, bot) = thumb.rows[r][x];
            let cell = &mut buf[Position {
                x: rect.x + 1 + x as u16,
                y: rect.y + 1 + title_extra as u16 + r as u16,
            }];
            cell.set_char('▀');
            cell.set_style(Style::default().fg(map(top)).bg(map(bot)));
        }
    }

    // Attribute lines below the thumbnail: the symmetry line, the
    // schema summary (or the description fragment for unclassified
    // cards), and "params: n" only for parameterized tilings.
    let y0 = rect.y + 1 + title_extra as u16 + thumb_rows as u16;
    if y0 + attr_rows as u16 <= rect.y + rect.height {
        let (line1, line2) = browse::attr_lines(&card);
        let dim = Style::default().fg(Color::Gray);
        let dimmer = Style::default().fg(Color::DarkGray);
        frame.render_widget(
            Paragraph::new(truncate(&line1, iw)).style(dim),
            Rect::new(rect.x + 1, y0, iw as u16, 1),
        );
        frame.render_widget(
            Paragraph::new(line2)
                .style(dimmer)
                .wrap(Wrap { trim: false }),
            Rect::new(rect.x + 1, y0 + 1, iw as u16, 2),
        );
        if has_params {
            frame.render_widget(
                Paragraph::new(truncate(&format!("params: {}", nparams), iw)).style(dim),
                Rect::new(rect.x + 1, y0 + 3, iw as u16, 1),
            );
        }
    }
}

fn render_status_browse(frame: &mut Frame, tui: &mut Tui, rect: Rect) {
    let status = match tui.mode {
        Mode::Search => format!(
            " search> {} │ {} match(es) · enter open · esc cancel · tab/1-9/0 switch",
            truncate(&tui.search, 24),
            tui.match_counts().iter().sum::<usize>()
        ),
        _ => format!(
            " {} (tab {}/10) │ arrows/hjkl move · enter open · tab/1-9/0 tabs · pgup/pgdn/wheel scroll · +/- card size · / search · q quit",
            tui.active_tab().category.label(),
            tui.tab_idx + 1
        ),
    };
    frame.render_widget(
        Paragraph::new(truncate(&status, rect.width as usize))
            .style(Style::default().fg(Color::DarkGray)),
        rect,
    );
}

// -------------------------------------------------------------------
// Render mode: [full-area canvas] / [params strip] / [status bar]
// -------------------------------------------------------------------

fn render_render_view(frame: &mut Frame, tui: &mut Tui, area: Rect) {
    let specs = tui.app.tiling().params();
    let strip_h = if tui.params_open {
        (specs.len() as u16).saturating_add(2).max(3)
    } else {
        0
    };
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(strip_h),
            Constraint::Length(1),
        ])
        .split(area);
    let canvas = outer[0];
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    tui.canvas_size = (w, h);

    // Generate the tiling for the full-area view; raster at 1 cell =
    // 1 px (a space glyph shows only its background, so the pixel
    // color goes to `bg`).
    let result = tui.app.result(w, h);
    if w > 0 && h > 0 {
        if let Some(msg) = result.message {
            frame.render_widget(
                Paragraph::new(msg.to_string())
                    .style(Style::default().fg(Color::Red))
                    .alignment(Alignment::Center),
                canvas,
            );
        } else {
            let bbox = tui.app.viewport.world_bbox(w, h);
            let r = raster::raster(
                &result.tiles,
                &bbox,
                w,
                h,
                tui.app.color_mode,
                &tui.palette256,
            );
            let buf = frame.buffer_mut();
            for sy in 0..h {
                for sx in 0..w {
                    let px = match r.at(sx, sy) {
                        raster::BORDER => Color::Indexed(tui.border256),
                        raster::GAP => Color::Indexed(tui.gap256),
                        c => Color::Indexed(c),
                    };
                    let cell = &mut buf[Position {
                        x: canvas.x + sx as u16,
                        y: canvas.y + sy as u16,
                    }];
                    cell.set_char(' ');
                    cell.set_style(Style::default().fg(px).bg(px));
                }
            }
        }
    }

    if tui.params_open {
        render_params_strip(frame, tui, outer[1]);
    }

    let t = tui.app.tiling();
    let status = format!(
        " {} [{}] · {} · tiles {} │ esc back · ←/→ prev/next · hjkl pan · +/- zoom · c color · r reset · p params",
        t.name(),
        t.id(),
        result.symmetry.group,
        result.tiles.len(),
    );
    frame.render_widget(
        Paragraph::new(truncate(&status, outer[2].width as usize))
            .style(Style::default().fg(Color::DarkGray)),
        outer[2],
    );
}

fn render_params_strip(frame: &mut Frame, tui: &mut Tui, rect: Rect) {
    let t = tui.app.tiling();
    let w = rect.width.saturating_sub(2) as usize;
    let max_lines = rect.height.saturating_sub(2) as usize;
    let lines: Vec<Line> = t
        .params()
        .iter()
        .enumerate()
        .take(max_lines)
        .map(|(i, spec)| {
            let v = tui
                .app
                .param_value(spec.id)
                .map(|v| v.display())
                .unwrap_or_else(|| spec.default_value().display());
            let cursor = i == tui.app.param_cursor;
            let marker = if cursor { "▶ " } else { "  " };
            let style = if cursor {
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Gray)
            };
            Line::from(truncate(&format!("{}{} = {}", marker, spec.label, v), w)).style(style)
        })
        .collect();
    frame.render_widget(
        Paragraph::new(Text::from(lines)).block(
            Block::default()
                .title(" params (p/esc close) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        ),
        rect,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;

    /// Count cells with the given symbol across the whole buffer.
    fn count_symbol(terminal: &Terminal<TestBackend>, sym: &str) -> usize {
        terminal
            .backend()
            .buffer()
            .content
            .iter()
            .filter(|c| c.symbol() == sym)
            .count()
    }

    /// Whole-buffer text with newlines per row (for substring checks).
    fn buffer_text(terminal: &Terminal<TestBackend>) -> String {
        let buf = terminal.backend().buffer();
        let w = buf.area.width as usize;
        let mut s = String::new();
        for (i, c) in buf.content.iter().enumerate() {
            if i > 0 && i % w == 0 {
                s.push('\n');
            }
            s.push_str(c.symbol());
        }
        s
    }

    fn key(tui: &mut Tui, code: KeyCode) {
        assert!(tui.handle_key(code, KeyModifiers::NONE));
    }

    // ---- browse model × real registry ------------------------------

    #[test]
    fn registry_tabs_classify_every_tiling_once() {
        let tui = Tui::new();
        assert_eq!(tui.tabs.len(), 10, "one tab per TilingCategory::ALL entry");
        // Multi-tab membership (M76): a tiling appears in its primary
        // tab plus every secondary tab, so the tab sizes sum to the
// 136 memberships of the 122 registered tilings, not to 122.
        let total: usize = tui.tabs.iter().map(|t| t.cards.len()).sum();
        assert_eq!(total, 136, "122 tilings carry 136 tab memberships");
        let counts: Vec<usize> = tui.tabs.iter().map(|t| t.cards.len()).collect();
assert_eq!(counts, vec![11, 17, 24, 17, 15, 21, 8, 12, 6, 5]);
    }

    #[test]
    fn wallpaper_tab_sorted_by_crystal_system_then_group() {
        let tui = Tui::new();
        let wall = &tui.tabs[3];
        assert_eq!(wall.category, TilingCategory::Wallpaper);
        let keys: Vec<&'static str> = wall
            .cards
            .iter()
            .map(|c| browse::section_key(TilingCategory::Wallpaper, c))
            .collect();
        // Section (conventional crystal-system) rank is non-decreasing
        // along the tab — the sort key is the wallpaper group's
        // conventional lattice, not the representative's measured
        // lattice (a p1 card on a centered-rectangular cell still
        // displays under oblique).
        let ranks: Vec<u8> = wall
            .cards
            .iter()
            .map(|c| {
                section_rank(browse::section_key(TilingCategory::Wallpaper, c))
            })
            .collect();
        assert!(
            ranks.windows(2).all(|w| w[0] <= w[1]),
            "section rank must be non-decreasing: {ranks:?}"
        );
        // Sections appear in first-appearance order: oblique,
        // rectangular, centered, square, hexagonal.
        let mut seen: Vec<&'static str> = Vec::new();
        for &k in &keys {
            if !k.is_empty() && !seen.contains(&k) {
                seen.push(k);
            }
        }
        assert_eq!(
            seen,
            vec!["oblique", "rectangular", "centered", "square", "hexagonal"],
            "sections must appear in crystal-system order"
        );
    }

    /// The crystal-system rank of a Wallpaper section key (mirrors the
    /// conventional-lattice display order).
    fn section_rank(key: &'static str) -> u8 {
        match key {
            "oblique" => 0,
            "rectangular" => 1,
            "centered" => 2,
            "square" => 3,
            "hexagonal" => 4,
            _ => 9,
        }
    }

    // ---- tab bar ----------------------------------------------------

    #[test]
    fn tab_bar_shows_all_ten_categories_with_counts() {
        let mut term = Terminal::new(TestBackend::new(200, 30)).unwrap();
        let mut tui = Tui::new();
        term.draw(|f| render(f, &mut tui)).unwrap();
        let text = buffer_text(&term);
        let row0 = text.lines().next().unwrap();
        for (label, n) in [
            ("Uniform", 11),
("Two-Uniform", 17),
            ("Laves (Catalan duals)", 24),
            ("Wallpaper", 17),
("Pentagon Types", 15),
("Islamic", 21),
            ("Arrangements", 8),
            ("Quasiperiodic", 12),
            ("Aperiodic", 6),
            ("Nonperiodic", 5),
        ] {
            assert!(
                row0.contains(&format!("{} ({n})", label)),
                "tab bar missing {label} ({n}): {row0}"
            );
        }
    }

    // ---- card grid ----------------------------------------------------

    #[test]
    fn grid_paints_wallpaper_cards_and_sections() {
        let mut term = Terminal::new(TestBackend::new(120, 30)).unwrap();
        let mut tui = Tui::new();
        key(&mut tui, KeyCode::Char('4')); // Wallpaper tab
        term.draw(|f| render(f, &mut tui)).unwrap();
        let mut text = buffer_text(&term);
        // First card name painted.
        assert!(
            text.contains(tui.tabs[3].cards[0].name),
            "first wallpaper card name missing"
        );
        // The catalog note opens the tab. Since M85 all 17 groups are
        // represented, it reports the completed catalog; the header is
        // a single-line row that truncates, so pin the truncation-proof
        // prefix.
        assert!(
            text.contains("complete catalog: all 17 wallpaper groups"),
            "wallpaper catalog note missing: {text}"
        );
        assert!(
            count_symbol(&term, "▀") > 100,
            "card thumbnails not painted"
        );
        // Section headers: scroll through the whole tab and collect
        // the distinct headers in first-appearance order.
        let mut seen: Vec<&str> = Vec::new();
        for _ in 0..12 {
            for h in ["oblique", "rectangular", "centered", "square", "hexagonal"] {
                if text.contains(h) && !seen.contains(&h) {
                    seen.push(h);
                }
            }
            key(&mut tui, KeyCode::PageDown);
            term.draw(|f| render(f, &mut tui)).unwrap();
            text = buffer_text(&term);
        }
        assert_eq!(
            seen,
            vec!["oblique", "rectangular", "centered", "square", "hexagonal"],
            "wallpaper sections must appear in crystal-system order"
        );
    }

    #[test]
    fn tab_switch_keeps_per_tab_cursor_and_scroll() {
        let mut term = Terminal::new(TestBackend::new(120, 30)).unwrap();
        let mut tui = Tui::new();
        term.draw(|f| render(f, &mut tui)).unwrap();
        key(&mut tui, KeyCode::Right); // Uniform cursor → 1
        assert_eq!(tui.cursors[0], 1);
        key(&mut tui, KeyCode::Tab); // → Two-Uniform
        assert_eq!(tui.tab_idx, 1);
        key(&mut tui, KeyCode::Right);
        assert_eq!(tui.cursors[1], 1);
        key(&mut tui, KeyCode::BackTab); // → Uniform again
        assert_eq!(tui.tab_idx, 0);
        assert_eq!(tui.cursors[0], 1, "Uniform cursor remembered");
        assert_eq!(tui.cursors[1], 1, "Two-Uniform cursor remembered");
        // Number keys jump.
        key(&mut tui, KeyCode::Char('6'));
        assert_eq!(tui.tab_idx, 5);
        key(&mut tui, KeyCode::Char('0'));
        assert_eq!(tui.tab_idx, 9);
        // Wrap both ways.
        key(&mut tui, KeyCode::Tab);
        assert_eq!(tui.tab_idx, 0);
        key(&mut tui, KeyCode::BackTab);
        assert_eq!(tui.tab_idx, 9);
    }

    #[test]
    fn cursor_moves_wrap_and_autoscroll() {
        let mut term = Terminal::new(TestBackend::new(100, 30)).unwrap();
        let mut tui = Tui::new();
        term.draw(|f| render(f, &mut tui)).unwrap();
        let n = tui.tabs[0].cards.len();
        key(&mut tui, KeyCode::Right);
        assert_eq!(tui.cursors[0], 1);
        // Walk to the last card of the tab.
        for _ in 1..(n - 1) {
            key(&mut tui, KeyCode::Right);
        }
        assert_eq!(tui.cursors[0], n - 1, "cursor at the last card");
        // The grid auto-scrolls so the cursor stays in view.
        term.draw(|f| render(f, &mut tui)).unwrap();
        assert!(tui.scroll[0] > 0, "grid should auto-scroll to the cursor");
        let last_name = tui.tabs[0].cards[n - 1].name;
        let text = buffer_text(&term);
        // Card titles are two-line: compare against the same split the
        // renderer uses (line 1, and line 2 when the name wraps).
        for line in title_lines(last_name, 31) {
            assert!(
                text.contains(&line),
                "cursor card title line {line:?} should be visible after auto-scroll"
            );
        }
        // Wrapping within the tab: last → 0 → last.
        key(&mut tui, KeyCode::Right);
        assert_eq!(tui.cursors[0], 0, "cursor wraps within the tab");
        key(&mut tui, KeyCode::Left);
        assert_eq!(tui.cursors[0], n - 1);
    }

    // ---- render round trip --------------------------------------------

    #[test]
    fn enter_opens_render_and_esc_returns_to_grid() {
        let mut term = Terminal::new(TestBackend::new(100, 30)).unwrap();
        let mut tui = Tui::new();
        term.draw(|f| render(f, &mut tui)).unwrap();
        key(&mut tui, KeyCode::Char('4')); // Wallpaper
        key(&mut tui, KeyCode::Right); // cursor → card 1
        let (tab, card) = (3usize, 1usize);
        let id = tui.tabs[tab].cards[card].id;
        let name = tui.tabs[tab].cards[card].name;
        key(&mut tui, KeyCode::Enter);
        assert_eq!(tui.mode, Mode::Render);
        assert_eq!(tui.app.selected_id(), id);
        term.draw(|f| render(f, &mut tui)).unwrap();
        let text = buffer_text(&term);
        assert!(
            text.contains("esc back"),
            "render status bar missing the esc back hint: {text}"
        );
        assert!(text.contains(name), "status bar should name the tiling");
        // Canvas painted full-area (no more 30-col side panel): count
        // the blank cells with a non-default background.
        let buf = term.backend().buffer();
        let w = buf.area.width as usize;
        let mut painted = 0usize;
        for y in 0..(buf.area.height as usize - 1) {
            for x in 0..w {
                let c = &buf.content[y * w + x];
                if c.symbol() == " " && c.style().bg != Some(Color::Reset) {
                    painted += 1;
                }
            }
        }
        assert!(
            painted > 1500,
            "full-area canvas should be painted, got {painted} cells"
        );
        // Esc → Browse, same tab + cursor (selection preserved).
        key(&mut tui, KeyCode::Esc);
        assert_eq!(tui.mode, Mode::Browse);
        assert_eq!(tui.tab_idx, tab);
        assert_eq!(tui.cursors[tab], card);
        assert_eq!(tui.app.selected_id(), id);
        term.draw(|f| render(f, &mut tui)).unwrap();
        assert!(
            count_symbol(&term, "▀") > 50,
            "grid should be back with card thumbnails"
        );
    }

    #[test]
    fn render_prev_next_walks_tab_order() {
        let mut term = Terminal::new(TestBackend::new(100, 30)).unwrap();
        let mut tui = Tui::new();
        term.draw(|f| render(f, &mut tui)).unwrap();
        key(&mut tui, KeyCode::Char('5')); // Pentagon Types
        // The walk follows the SELECTED tiling's own category tab; the
        // Pentagon Types tab is primary membership, so walking stays in
        // it (the Wallpaper tab is all secondary membership).
        let (i0, i1, n1, last_id, last_idx) = {
            let tab = &tui.tabs[4];
            (
                tab.cards[0].id,
                tab.cards[1].id,
                tab.cards[1].name,
                tab.cards[tab.cards.len() - 1].id,
                tab.cards.len() - 1,
            )
        };
        key(&mut tui, KeyCode::Enter);
        assert_eq!(tui.app.selected_id(), i0);
        key(&mut tui, KeyCode::Right);
        assert_eq!(tui.app.selected_id(), i1);
        assert_eq!(tui.cursors[4], 1, "browse cursor follows the walk");
        term.draw(|f| render(f, &mut tui)).unwrap();
        let text = buffer_text(&term);
        assert!(text.contains(n1), "status bar should show the next tiling");
        // Wrap around: ← twice from card 1 lands on the last card.
        key(&mut tui, KeyCode::Left);
        key(&mut tui, KeyCode::Left);
        assert_eq!(tui.app.selected_id(), last_id);
        assert_eq!(tui.cursors[4], last_idx);
    }

    #[test]
    fn render_paints_full_area_canvas_regression() {
        // Regression: the canvas blit puts the pixel color on the
        // BACKGROUND of a space glyph; the whole canvas region must be
        // painted (nothing left at the default background).
        let mut term = Terminal::new(TestBackend::new(100, 30)).unwrap();
        let mut tui = Tui::new();
        term.draw(|f| render(f, &mut tui)).unwrap();
        key(&mut tui, KeyCode::Enter); // first tiling → Render
        term.draw(|f| render(f, &mut tui)).unwrap();
        let buf = term.backend().buffer();
        let w = buf.area.width as usize;
        let h = (buf.area.height as usize) - 1; // above the status bar
        let mut painted = 0usize;
        for y in 0..h {
            for x in 0..w {
                let c = &buf.content[y * w + x];
                if c.style().bg != Some(Color::Reset) {
                    painted += 1;
                }
            }
        }
        assert!(
            painted > w * h * 9 / 10,
            "canvas should be almost fully painted, got {painted}/{total}",
            total = w * h
        );
    }

    // ---- search ---------------------------------------------------------

    #[test]
    fn search_filters_cards_jumps_tab_and_opens_match() {
        let mut term = Terminal::new(TestBackend::new(120, 30)).unwrap();
        let mut tui = Tui::new();
        term.draw(|f| render(f, &mut tui)).unwrap();
        key(&mut tui, KeyCode::Char('4')); // start in Wallpaper
        key(&mut tui, KeyCode::Char('/'));
        for c in "penrose".chars() {
            key(&mut tui, KeyCode::Char(c));
        }
        assert_eq!(tui.mode, Mode::Search);
        // Active tab jumped to the first tab with a match.
        let counts = tui.match_counts();
        assert!(counts.iter().any(|&c| c > 0), "penrose must match");
        assert_eq!(tui.tab_idx, counts.iter().position(|&c| c > 0).unwrap());
        assert!(counts[tui.tab_idx] > 0);
        term.draw(|f| render(f, &mut tui)).unwrap();
        let text = buffer_text(&term);
        assert!(
            text.contains("search> penrose"),
            "status bar should show the query"
        );
        // Non-matching cards of the active tab are hidden.
        let q = "penrose";
        let nonmatch = tui.tabs[tui.tab_idx]
            .cards
            .iter()
            .find(|c| !matches_card(c, q))
            .expect("active tab has non-matching cards");
        assert!(
            !text.contains(nonmatch.name),
            "non-matching card {} should be hidden",
            nonmatch.name
        );
        // Enter opens the highlighted match.
        let hl = tui.highlighted_card().expect("a match is highlighted");
        let expected = tui.tabs[tui.tab_idx].cards[hl].id;
        key(&mut tui, KeyCode::Enter);
        assert_eq!(tui.mode, Mode::Render);
        assert_eq!(tui.app.selected_id(), expected);
        assert_eq!(tui.cursors[tui.tab_idx], hl);
        // Esc back to Browse cancels the query (back to the grid).
        key(&mut tui, KeyCode::Esc);
        assert_eq!(tui.mode, Mode::Browse);
        key(&mut tui, KeyCode::Char('/'));
        key(&mut tui, KeyCode::Char('p'));
        key(&mut tui, KeyCode::Esc); // cancel
        assert_eq!(tui.mode, Mode::Browse);
        assert!(tui.search.is_empty());
        term.draw(|f| render(f, &mut tui)).unwrap();
        // All cards of the tab visible again.
        let text = buffer_text(&term);
        let mut shown = 0;
        for c in &tui.tabs[tui.tab_idx].cards {
            if text.contains(c.name) {
                shown += 1;
            }
        }
        // At least the first visible row of cards is back.
        assert!(shown >= 1);
    }

    #[test]
    fn search_tab_bar_shows_match_counts() {
        let mut term = Terminal::new(TestBackend::new(200, 30)).unwrap();
        let mut tui = Tui::new();
        term.draw(|f| render(f, &mut tui)).unwrap();
        key(&mut tui, KeyCode::Char('/'));
        for c in "hat".chars() {
            key(&mut tui, KeyCode::Char(c));
        }
        term.draw(|f| render(f, &mut tui)).unwrap();
        let text = buffer_text(&term);
        let row0 = text.lines().next().unwrap();
        let counts = tui.match_counts();
        for (i, n) in counts.iter().enumerate() {
            let label = tui.tabs[i].category.label();
            assert!(
                row0.contains(&format!("{label} ({n})")),
                "search tab bar should show match count for {label}: {row0}"
            );
        }
    }

    // ---- params overlay ---------------------------------------------------

    #[test]
    fn params_overlay_toggles_and_adjusts() {
        let mut term = Terminal::new(TestBackend::new(100, 30)).unwrap();
        let mut tui = Tui::new();
        term.draw(|f| render(f, &mut tui)).unwrap();
        key(&mut tui, KeyCode::Char('4'));
        let ci = tui.tabs[3]
            .cards
            .iter()
            .position(|c| c.params > 0)
            .expect("the Wallpaper tab has parameterized tilings");
        tui.cursors[3] = ci;
        key(&mut tui, KeyCode::Enter);
        let specs = tui.app.tiling().params();
        assert!(!specs.is_empty());
        let before = tui.app.param_value(specs[0].id).map(|v| v.display());
        key(&mut tui, KeyCode::Char('p'));
        assert!(tui.params_open);
        term.draw(|f| render(f, &mut tui)).unwrap();
        let text = buffer_text(&term);
        assert!(
            text.contains("params (p/esc close)"),
            "params strip missing: {text}"
        );
        assert!(text.contains(specs[0].label), "param label missing");
        key(&mut tui, KeyCode::Right); // adjust under the cursor
        let after = tui.app.param_value(specs[0].id).map(|v| v.display());
        assert_ne!(before, after, "the parameter should step");
        // Canvas still regenerated (no panic), status bar intact.
        term.draw(|f| render(f, &mut tui)).unwrap();
        assert!(buffer_text(&term).contains("esc back"));
        // p closes again.
        key(&mut tui, KeyCode::Char('p'));
        assert!(!tui.params_open);
    }

    // ---- mouse hit tests (pure fns) -------------------------------------

    #[test]
    fn tab_at_and_card_at_hit_test() {
        // rect (x, w) spans x..x+w (exclusive end): the second tab
        // covers 14..=40, so 41 is the first outside column.
        let tabs = vec![Rect::new(0, 0, 14, 1), Rect::new(14, 0, 27, 1)];
        assert_eq!(tab_at(&tabs, 3, 0), Some(0));
        assert_eq!(tab_at(&tabs, 15, 0), Some(1));
        assert_eq!(tab_at(&tabs, 40, 0), Some(1), "last column of tab 2");
        assert_eq!(tab_at(&tabs, 41, 0), None, "outside all tabs");
        assert_eq!(tab_at(&tabs, 5, 5), None, "wrong row");
        let cards = vec![
            (Rect::new(0, 1, 30, 14), 0usize),
            (Rect::new(30, 1, 30, 14), 3usize), // tab index, not slot
        ];
        assert_eq!(card_at(&cards, 5, 5), Some(0));
        assert_eq!(card_at(&cards, 35, 5), Some(3));
        assert_eq!(card_at(&cards, 5, 20), None, "below the cards");
        assert_eq!(card_at(&[], 0, 0), None, "no cards rendered yet");
    }

    #[test]
    fn mouse_clicks_switch_tab_and_open_render() {
        // Exercise the mouse path end to end: render a Browse frame so
        // the hit-test rects are populated, then dispatch synthesized
        // MouseEvents (the event loop's match arm is plain state
        // mutation — safe to drive from a test).
        let mut term = Terminal::new(TestBackend::new(100, 30)).unwrap();
        let mut tui = Tui::new();
        term.draw(|f| render(f, &mut tui)).unwrap();
        assert!(!tui.tab_rects.is_empty());
        assert!(!tui.card_rects.is_empty());
        // Click the first tab → stay on tab 0 (and it is a hit).
        let (rx, ry) = (tui.tab_rects[0].x, tui.tab_rects[0].y);
        let before = tui.tab_idx;
        tui.handle_mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: rx + 1,
            row: ry,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(tui.tab_idx, before);
        // Click a rendered card → Render on that card.
        let (crect, cidx) = tui.card_rects[0];
        tui.handle_mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: crect.x + 1,
            row: crect.y + 1,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(tui.mode, Mode::Render);
        assert_eq!(
            tui.app.selected_id(),
            tui.tabs[tui.tab_idx].cards[cidx].id
        );
        // Wheel in Render zooms.
        let z0 = tui.app.viewport.zoom;
        tui.handle_mouse(MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 5,
            row: 5,
            modifiers: KeyModifiers::NONE,
        });
        assert!(tui.app.viewport.zoom > z0);
        // Wheel in Browse scrolls the grid.
        key(&mut tui, KeyCode::Esc);
        term.draw(|f| render(f, &mut tui)).unwrap();
        let s0 = tui.scroll[0];
        tui.handle_mouse(MouseEvent {
            kind: MouseEventKind::ScrollDown,
            column: 5,
            row: 5,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(tui.scroll[0], s0 + 1);
        // Ctrl+wheel in Browse resizes the cards (plain wheel above
        // scrolled); the size clamps to the [22, 44] width window and
        // plain scroll still works after resizing. Shift+wheel is an
        // accepted alias for terminals that pass it through.
        tui.handle_mouse(MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 5,
            row: 5,
            modifiers: KeyModifiers::CONTROL,
        });
        assert_eq!(tui.card_w, TARGET_CARD_W + 2);
        tui.handle_mouse(MouseEvent {
            kind: MouseEventKind::ScrollDown,
            column: 5,
            row: 5,
            modifiers: KeyModifiers::SHIFT,
        });
        assert_eq!(tui.card_w, TARGET_CARD_W, "shift+wheel mirrors ctrl+wheel");
        for _ in 0..20 {
            tui.resize_cards(2);
        }
        assert_eq!(tui.card_w, MAX_CARD_W, "size clamps at the maximum");
        for _ in 0..40 {
            tui.resize_cards(-2);
        }
        assert_eq!(tui.card_w, MIN_CARD_W, "size clamps at the minimum");
        let s1 = tui.scroll[0];
        tui.handle_mouse(MouseEvent {
            kind: MouseEventKind::ScrollDown,
            column: 5,
            row: 5,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(tui.scroll[0], s1 + 1, "plain wheel still scrolls");
        // Larger cards → fewer columns fit the same viewport.
        tui.card_w = TARGET_CARD_W;
        let (c28, _, r28, _) = grid_cols_rows(Rect::new(0, 0, 280, 140), 28);
        let (c44, _, r44, _) = grid_cols_rows(Rect::new(0, 0, 280, 140), 44);
        assert_eq!((c28, r28), (10, 10));
        assert_eq!((c44, r44), (6, 6));
    }

    /// The +/- keys resize the browse card grid (the same keys zoom
    /// the canvas in Render mode — mode-dependent behavior), stepping
    /// by 2 and clamping to the [22, 44] width window.
    #[test]
    fn plus_minus_keys_resize_browse_cards() {
        let mut term = Terminal::new(TestBackend::new(100, 30)).unwrap();
        let mut tui = Tui::new();
        term.draw(|f| render(f, &mut tui)).unwrap();
        assert_eq!(tui.card_w, TARGET_CARD_W);
        // '=' (unshifted) and '+' (shifted on US layouts) both grow.
        key(&mut tui, KeyCode::Char('='));
        assert_eq!(tui.card_w, TARGET_CARD_W + 2);
        key(&mut tui, KeyCode::Char('+'));
        assert_eq!(tui.card_w, TARGET_CARD_W + 4);
        // '-' and '_' both shrink.
        key(&mut tui, KeyCode::Char('-'));
        key(&mut tui, KeyCode::Char('_'));
        assert_eq!(tui.card_w, TARGET_CARD_W);
        // Clamps hold via the keys too.
        for _ in 0..20 {
            key(&mut tui, KeyCode::Char('+'));
        }
        assert_eq!(tui.card_w, MAX_CARD_W);
        for _ in 0..30 {
            key(&mut tui, KeyCode::Char('-'));
        }
        assert_eq!(tui.card_w, MIN_CARD_W);
        // The keys never fire in Search mode: the search field gets
        // its characters instead.
        key(&mut tui, KeyCode::Char('/'));
        key(&mut tui, KeyCode::Char('+'));
        assert_eq!(tui.card_w, MIN_CARD_W, "search input does not resize");
        assert_eq!(tui.search, "+");
        key(&mut tui, KeyCode::Esc);
    }

    // ---- classification schema rendering ----------------------------------

    /// The schema summary adds information instead of echoing the
    /// card name: pinned lines for representative families.
    #[test]
    fn attr_lines_compose_schema_summaries() {
        let tabs = browse::browse_tabs_with(&tess_tilings::registry(), tess_tilings::classification);
        let line2 = |id: &str| -> String {
            for tab in &tabs {
                if let Some(c) = tab.cards.iter().find(|c| c.id == id) {
                    return browse::attr_lines(c).1;
                }
            }
            panic!("{id} not found in any tab");
        };
        let line1 = |id: &str| -> String {
            for tab in &tabs {
                if let Some(c) = tab.cards.iter().find(|c| c.id == id) {
                    return browse::attr_lines(c).0;
                }
            }
            panic!("{id} not found in any tab");
        };
        // Uniform: vertex config that is NOT in the name, Schläfli,
        // uniformity, dual.
        assert_eq!(
            line2("square"),
            "4.4.4.4 · {4,4} · regular · self-dual"
        );
        // "(4.8.8)" is already in the card name — the summary skips it.
        assert_eq!(
            line2("truncated-square"),
            "t{4,4} · semiregular · dual: tetrakis square"
        );
        // Pentagon: "type 6" is in the name, so the summary leads with
        // the isohedral class and attribution.
        assert_eq!(line2("pentagon-6"), "2-isohedral · Kershner 1968");
        // Cairo joins the Pentagon tab as the typed type-4 member.
        assert_eq!(
            line2("cairo-pentagonal"),
            "type 4 · 1-isohedral · dual: snub square"
        );
        assert_eq!(line1("cairo-pentagonal"), "p4g · square");
        // Quasiperiodic: the symmetry line replaces "— · —" with the
        // declared translational character + point order.
        assert_eq!(line1("penrose"), "quasiperiodic · 5-fold");
        assert_eq!(line2("penrose"), "substitution · φ · Penrose 1974");
        // Aperiodic monotile ("monotile" is echoed by the name and skipped;
        // chirality and the reflection fact follow).
        assert_eq!(line1("hat"), "aperiodic");
        assert_eq!(
            line2("hat"),
            "metatile substitution · φ² · Smith et al. 2023 · einstein · mixed chirality · reflected copies"
        );
        // Islamic construction data.
        assert_eq!(
            line2("pic-4-8-8"),
            "Hankin polygons-in-contact · 4.8.8 · θK 45°"
        );
        // Cut-and-project data.
        assert_eq!(
            line2("ammann-beenker"),
            "substitution · 1+√2 · R⁴ → R², octagonal window · Ammann & Beenker 1982"
        );
        // Limit-periodic rep-tile.
        assert_eq!(line1("chair"), "limit-periodic");
    }

    /// The "hide absent fields" rule, over every card in the registry:
    /// no attribute line is ever empty, no line ever renders a
    /// placeholder like "— · —" or "N/A", and nonperiodic cards
    /// (no declared group) state their translational character
    /// instead.
    #[test]
    fn attr_lines_never_render_placeholders() {
        let tabs = browse::browse_tabs_with(&tess_tilings::registry(), tess_tilings::classification);
        for tab in &tabs {
            for c in &tab.cards {
                let (l1, l2) = browse::attr_lines(c);
                assert!(!l1.is_empty(), "{}: empty symmetry line", c.id);
                assert!(!l2.is_empty(), "{}: empty summary line", c.id);
                assert!(!l1.contains("—"), "{}: em-dash in {l1:?}", c.id);
                assert!(!l2.contains("N/A"), "{}: N/A in summary", c.id);
                if c.group.is_none() {
                    assert!(
                        l1.contains("quasiperiodic")
                            || l1.contains("aperiodic")
                            || l1.contains("limit-periodic")
                            || l1.contains("non-periodic"),
                        "{}: nonperiodic symmetry line states the translational character: {l1:?}",
                        c.id
                    );
                }
            }
        }
    }

    /// The Pentagon tab sorts by the declared type number: the
    /// sequential type order 1, 2, 3, 4 (cairo), 5 (floret), …, 15 (the
    /// Hirschhorn pentagon is a Nonperiodic primary, not a type).
    #[test]
    fn pentagon_tab_orders_types_sequentially() {
        let tabs = browse::browse_tabs_with(&tess_tilings::registry(), tess_tilings::classification);
        let pent = &tabs[tabs
            .iter()
            .position(|t| t.category == TilingCategory::PentagonTypes)
            .unwrap()];
        let types: Vec<Option<u8>> = pent.cards.iter().map(|c| c.class.pentagon_type).collect();
        let expected: Vec<Option<u8>> = (1u8..=15).map(Some).collect();
        assert_eq!(types, expected, "types 1..=15 in sequence");
    }

    /// The Two-Uniform tab sorts by vertex configuration: shared
    /// configurations (the ₁/₂ pairs) sort together, ahead of the
    /// config-less remainder (none today).
    #[test]
    fn two_uniform_tab_groups_configurations() {
        let tabs = browse::browse_tabs_with(&tess_tilings::registry(), tess_tilings::classification);
        let tu = &tabs[tabs
            .iter()
            .position(|t| t.category == TilingCategory::TwoUniform)
            .unwrap()];
        assert_eq!(tu.cards.len(), 17);
        let configs: Vec<Option<&'static str>> =
            tu.cards.iter().map(|c| c.class.vertex_config).collect();
        assert!(
            configs.iter().all(|c| c.is_some()),
            "every 2-uniform card declares a configuration: {configs:?}"
        );
        // The three configuration pairs are adjacent (n4/n3, n14/n15,
        // n20/n19 — the ₁/₂ subscript sorts beside its sibling).
        let find = |prefix: &str| {
            configs
                .iter()
                .position(|c| c.unwrap_or("").starts_with(prefix))
                .unwrap()
        };
        for (a, b) in [
            ("(4⁴; 3³.4²)₁", "(4⁴; 3³.4²)₂"),
            ("(3⁶; 3³.4²)₁", "(3⁶; 3³.4²)₂"),
            ("(3⁶; 3⁴.6)₁", "(3⁶; 3⁴.6)₂"),
        ] {
            assert_eq!(find(a) + 1, find(b), "{a} and {b} must be adjacent");
        }
    }

    /// The Laves tab sorts by declared rank: the three self-dual
    /// regulars, then the Laves proper, then the 2-uniform duals.
    #[test]
    fn laves_tab_sorts_by_declared_rank() {
        let tabs = browse::browse_tabs_with(&tess_tilings::registry(), tess_tilings::classification);
        let lv = &tabs[tabs
            .iter()
            .position(|t| t.category == TilingCategory::Laves)
            .unwrap()];
        assert_eq!(lv.cards.len(), 24);
        let ranks: Vec<Option<u8>> = lv.cards.iter().map(|c| c.class.laves_rank).collect();
        let sorted = ranks.windows(2).all(|w| w[0] <= w[1]);
        assert!(sorted, "laves_rank must be non-decreasing: {ranks:?}");
        assert_eq!(ranks[0], Some(0), "self-dual regulars first");
        assert_eq!(*ranks.last().unwrap(), Some(2), "2-uniform duals last");
    }

    // ---- misc --------------------------------------------------------------

    #[test]
    fn dirty_loop_does_not_redraw_when_idle() {
        let mut term = Terminal::new(TestBackend::new(100, 30)).unwrap();
        let mut tui = Tui::new();
        term.draw(|f| render(f, &mut tui)).unwrap();
        assert!(!tui.dirty, "render clears the dirty flag");
        // A non-mutating key (e.g. 'x' in Browse) leaves state alone.
        assert!(tui.handle_key(KeyCode::Char('x'), KeyModifiers::NONE));
        assert!(!tui.dirty, "unbound keys must not mark dirty");
    }

    #[test]
    fn debug_dump_screen() {
        let mut term = Terminal::new(TestBackend::new(120, 30)).unwrap();
        let mut tui = Tui::new();
        key(&mut tui, KeyCode::Char('4'));
        term.draw(|f| render(f, &mut tui)).unwrap();
        println!("\n=== browse (wallpaper) ===\n{}", buffer_text(&term));
        key(&mut tui, KeyCode::Enter);
        term.draw(|f| render(f, &mut tui)).unwrap();
        println!("\n=== render ===\n{}", buffer_text(&term));
    }
}
