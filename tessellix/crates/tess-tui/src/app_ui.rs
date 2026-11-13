//! TUI application: event loop, key handling, rendering.

use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{Frame, Terminal};

use tess_core::app::App;
use tess_core::paint;

use crate::raster;

const SIDE_WIDTH: u16 = 30;

#[derive(PartialEq)]
enum Mode {
    List,
    Params,
    Search,
}

struct Tui {
    app: App,
    mode: Mode,
    palette256: Vec<u8>,
    border256: u8,
    bg256: u8,
    gap256: u8,
    canvas_size: (u32, u32),
    raster: Option<raster::Raster>,
    list_state: ListState,
    thumbs: crate::thumb::Cache,
    dirty: bool,
}

impl Tui {
    fn new() -> Self {
        let registry = tess_tilings::registry();
        let palette256: Vec<u8> = paint::PALETTE
            .iter()
            .map(|&c| paint::rgb_to_256(c))
            .collect();
        Self {
            app: App::new(registry, 8.0),
            mode: Mode::List,
            palette256,
            border256: paint::rgb_to_256(paint::BORDER),
            bg256: paint::rgb_to_256(paint::BACKGROUND),
            gap256: 124,
            canvas_size: (0, 0),
            raster: None,
            list_state: ListState::default(),
            thumbs: crate::thumb::Cache::new(),
            dirty: true,
        }
    }

    fn handle_key(&mut self, code: KeyCode, mods: KeyModifiers) -> bool {
        // Raw mode disables ISIG, so SIGINT never fires: Ctrl-C/Ctrl-Q
        // arrive as plain 'c'/'q' with CONTROL and would cycle color or
        // toggle mode. Treat them as quit.
        if mods.contains(KeyModifiers::CONTROL)
            && matches!(code, KeyCode::Char('c') | KeyCode::Char('q'))
        {
            return false;
        }
        match code {
            KeyCode::Char('q') | KeyCode::Esc if self.mode != Mode::Search => {
                if self.mode == Mode::Params {
                    self.mode = Mode::List;
                    self.dirty = true;
                } else {
                    return false;
                }
            }
            _ => {}
        }
        match self.mode {
            Mode::Search => match code {
                KeyCode::Esc | KeyCode::Enter => {
                    self.mode = Mode::List;
                    self.dirty = true;
                }
                KeyCode::Backspace => {
                    self.app.search.pop();
                    self.dirty = true;
                }
                KeyCode::Char(c) => {
                    self.app.search.push(c);
                    self.dirty = true;
                }
                _ => {}
            },
            Mode::List | Mode::Params => match code {
                KeyCode::Up | KeyCode::Down => {
                    let d = if code == KeyCode::Up { -1 } else { 1 };
                    if self.mode == Mode::Params {
                        self.app.move_param_cursor(d);
                    } else {
                        self.app.move_selection(d);
                    }
                    self.dirty = true;
                }
                KeyCode::Left => {
                    if self.mode == Mode::Params {
                        self.app.adjust_param(-1);
                    } else {
                        self.app.move_selection(-1);
                    }
                    self.dirty = true;
                }
                KeyCode::Right => {
                    if self.mode == Mode::Params {
                        self.app.adjust_param(1);
                    } else {
                        self.app.move_selection(1);
                    }
                    self.dirty = true;
                }
                KeyCode::Tab => {
                    self.mode = match self.mode {
                        Mode::List => Mode::Params,
                        Mode::Params => Mode::List,
                        _ => unreachable!(),
                    };
                    self.dirty = true;
                }
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
                KeyCode::Char('/') => {
                    self.mode = Mode::Search;
                    self.dirty = true;
                }
                _ => {}
            },
        }
        true
    }

    fn pan(&mut self, dir_x: i32, dir_y: i32) {
        let (w, h) = self.canvas_size;
        // pan_screen drags the content by (dx, dy) screen pixels (y down);
        // negate both so the keys reveal in the direction they name:
        // h -> left, l -> right, j -> down, k -> up (matching vim scroll).
        self.app.pan(
            -dir_x as f64 * w as f64 * 0.3,
            -dir_y as f64 * h as f64 * 0.3,
        );
        self.dirty = true;
    }
}

pub fn run() -> io::Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    enable_raw()?;
    let result = run_loop(&mut terminal);
    restore_raw()?;
    result
}

fn enable_raw() -> io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    io::stdout().execute(crossterm::terminal::EnterAlternateScreen)?;
    Ok(())
}

fn restore_raw() -> io::Result<()> {
    io::stdout().execute(crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn run_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    let mut tui = Tui::new();
    loop {
        // Draw only when something changed: render() regenerates the whole
        // tiling (app.result), so an idle window must not spin on it.
        // Every state change (key, resize) sets `dirty`; render clears it.
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
            Event::Resize(_, _) => tui.dirty = true,
            _ => {}
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, tui: &mut Tui) {
    let area = frame.area();
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(6), Constraint::Length(1)])
        .split(area);
    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(SIDE_WIDTH), Constraint::Min(1)])
        .split(outer[0]);
    let canvas = main[1];
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    tui.canvas_size = (w, h);

    // Generate tiling for the current view.
    let result = tui.app.result(w, h);

    // Side panel. The preview block needs ~11 rows; on short terminals
    // it is dropped in favor of list + params + info.
    let with_preview = area.height >= 24;
    let side = Layout::default()
        .direction(Direction::Vertical)
        .constraints(if with_preview {
            vec![
                Constraint::Length(1),
                Constraint::Min(4),
                Constraint::Length(1),
                Constraint::Min(3),
                Constraint::Length(11),
                Constraint::Min(3),
            ]
        } else {
            vec![
                Constraint::Length(1),
                Constraint::Min(4),
                Constraint::Length(1),
                Constraint::Min(3),
                Constraint::Min(2),
            ]
        })
        .split(main[0]);
    render_side(frame, tui, &side, &result, with_preview);

    // Canvas.
    if w > 0 && h > 0 {
        if let Some(msg) = result.message {
            frame.render_widget(
                Paragraph::new(msg.to_string())
                    .style(Style::default().fg(Color::Red))
                    .alignment(ratatui::layout::Alignment::Center),
                canvas,
            );
        } else {
            if tui.raster.as_ref().is_none_or(|r| r.w != w || r.h != h) || tui.dirty {
                let bbox = tui.app.viewport.world_bbox(w, h);
                tui.raster = Some(raster::raster(
                    &result.tiles,
                    &bbox,
                    w,
                    h,
                    tui.app.color_mode,
                    &tui.palette256,
                ));
            }
            if let Some(r) = &tui.raster {
                let buf = frame.buffer_mut();
                for sy in 0..h {
                    for sx in 0..w {
                        let fg = r.at(sx, sy);
                        let fg = match fg {
                            raster::BORDER => Color::Indexed(tui.border256),
                            raster::GAP => Color::Indexed(tui.gap256),
                            c => Color::Indexed(c),
                        };
                        let cell = &mut buf[(canvas.x + sx as u16, canvas.y + sy as u16)];
                        cell.set_char(' ');
                        cell.set_style(Style::default().fg(fg).bg(Color::Indexed(tui.bg256)));
                    }
                }
            }
        }
    }
    tui.dirty = false;

    // Status bar.
    let t = tui.app.tiling();
    let search = if tui.mode == Mode::Search {
        "search> "
    } else {
        ""
    };
    let status = format!(
        " {} [{}] · {} · tiles {} · {} · z {:.1} {} │ ↑↓ tiling/param  tab list/params  ←→ adjust  hjkl pan  +/- zoom  c color  r reset  / search  q quit",
        t.name(),
        t.id(),
        result.symmetry,
        result.tiles.len(),
        tui.app.color_mode.label(),
        tui.app.viewport.zoom,
        search
    );
    frame.render_widget(
        Paragraph::new(status)
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::TOP)),
        outer[1],
    );
}

fn render_side(
    frame: &mut Frame,
    tui: &mut Tui,
    side: &[ratatui::layout::Rect],
    result: &tess_core::tiling::TilingResult,
    with_preview: bool,
) {
    let t = tui.app.tiling();

    // Tiling list, grouped by declared category. Headers are rendered
    // rows but not selectable: move_selection walks the selectable set
    // (App::visible_tilings), so the arrow keys skip them naturally.
    // A ListState keeps the scroll offset and follows the selection.
    let mut items: Vec<ListItem> = Vec::new();
    let mut selected_item: Option<usize> = None;
    for (cat, tilings) in tui.app.visible_grouped() {
        items.push(
            ListItem::new(format!("{} · {}", cat, tilings.len())).style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::DarkGray),
            ),
        );
        for t in tilings {
            let selected = t.id() == tui.app.selected_id();
            if selected {
                selected_item = Some(items.len());
            }
            let mut style = Style::default();
            if selected {
                style = style.add_modifier(Modifier::BOLD).fg(Color::Cyan);
            }
            items.push(ListItem::new(format!(" {} · {}", t.name(), t.id())).style(style));
        }
    }
    tui.list_state.select(selected_item);
    let list = List::new(items)
        .block(
            Block::default()
                .title(" tilings ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::REVERSED)
                .fg(Color::Cyan),
        );
    frame.render_stateful_widget(list, side[1], &mut tui.list_state);

    // Params.
    let specs = t.params();
    let param_items: Vec<ListItem> = specs
        .iter()
        .enumerate()
        .map(|(i, spec)| {
            let v = tui
                .app
                .param_value(spec.id)
                .map(|v| v.display())
                .unwrap_or_else(|| spec.default_value().display());
            let cursor = i == tui.app.param_cursor && tui.mode == Mode::Params;
            let mut style = Style::default();
            if cursor {
                style = style.add_modifier(Modifier::BOLD).fg(Color::Yellow);
            }
            let marker = if cursor { "▶ " } else { "  " };
            ListItem::new(format!("{}{} = {}", marker, spec.label, v)).style(style)
        })
        .collect();
    let params = List::new(param_items).block(
        Block::default()
            .title(" params (tab to edit) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    frame.render_widget(params, side[3]);

    // Info.
    let info_idx = if with_preview { 5 } else { 4 };
    let info = {
        let mut lines = vec![
            format!("cat    {}", t.category().label()),
            format!("id     {}", t.id()),
            format!("group  {}", result.symmetry.group),
            format!("lattice {}", result.symmetry.lattice),
        ];
        if let Some(n) = result.symmetry.notes {
            lines.push(n.to_string());
        }
        lines.into_iter().map(Line::from).collect::<Vec<_>>()
    };
    let info = Paragraph::new(Text::from(info))
        .block(
            Block::default()
                .title(" info ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(info, side[info_idx]);

    // Preview: the selected tiling as a half-block mini-render. Cached
    // per (id, params, color mode, size); generation is bounded by the
    // small window, so browsing stays responsive.
    if !with_preview {
        return;
    }
    let pv = side[4];
    let block = Block::default()
        .title(" preview ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    frame.render_widget(block.clone(), pv);
    let pw = pv.width.saturating_sub(2) as u32;
    let ph = pv.height.saturating_sub(2) as u32;
    if pw == 0 || ph == 0 {
        return;
    }
    let thumb = tui.thumbs.get_or_generate(
        crate::thumb::ThumbCtx {
            tiling: t,
            specs,
            params: tui.app.params(),
            mode: tui.app.color_mode,
            palette256: &tui.palette256,
        },
        pw,
        ph,
    );
    let map = |c: u8| match c {
        raster::BORDER => Color::Indexed(tui.border256),
        raster::GAP => Color::Indexed(tui.bg256),
        c => Color::Indexed(c),
    };
    let buf = frame.buffer_mut();
    for r in 0..(ph as usize).min(thumb.rows.len()) {
        for x in 0..(pw as usize).min(thumb.w as usize) {
            let (top, bot) = thumb.rows[r][x];
            let cell = &mut buf[(pv.x + 1 + x as u16, pv.y + 1 + r as u16)];
            cell.set_char('▀');
            cell.set_style(Style::default().fg(map(top)).bg(map(bot)));
        }
    }
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

    #[test]
    fn render_draws_preview_and_follows_selection() {
        let terminal = Terminal::new(TestBackend::new(100, 30)).unwrap();
        let mut term = terminal;
        let mut tui = Tui::new();
        // Two selection moves exercise the list state + thumb cache.
        tui.app.move_selection(1);
        tui.app.move_selection(1);
        term.draw(|f| render(f, &mut tui)).unwrap();

        // Preview block: many half-block cells painted.
        let thumbs = count_symbol(&term, "▀");
        assert!(thumbs > 50, "expected preview pixels, got {thumbs}");
        // Info box gained the category/id lines.
        let text = buffer_text(&term);
        assert!(text.contains("cat    "), "info box missing category line");
        assert!(text.contains("id     "), "info box missing id line");
        // Preview block title present.
        assert!(text.contains("preview"), "preview block missing");
        // Second draw (no state change) must not panic and stays stable.
        term.draw(|f| render(f, &mut tui)).unwrap();
        assert_eq!(count_symbol(&term, "▀"), thumbs);
    }

    #[test]
    fn render_short_terminal_drops_preview() {
        let terminal = Terminal::new(TestBackend::new(100, 20)).unwrap();
        let mut term = terminal;
        let mut tui = Tui::new();
        term.draw(|f| render(f, &mut tui)).unwrap();
        assert_eq!(count_symbol(&term, "▀"), 0, "preview should be dropped");
        let text = buffer_text(&term);
        assert!(text.contains("cat    "));
    }
}
