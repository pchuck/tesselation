//! Shared application state, used by both the TUI and the GUI.

use std::collections::BTreeMap;

use crate::category::TilingCategory;
use crate::geometry::{BBox, Vec2};
use crate::params::{ParamId, ParamSet, ParamSpec, ParamValue};
use crate::registry::{group_by_category, Registry};
use crate::tiling::{Query, Tiling, TilingResult};
use crate::view::{ColorMode, Viewport};

/// High-level app state: which tiling is selected, its parameters, the
/// viewport, and the color mode. UIs translate input into these calls and
/// render from the results.
///
/// Invariant: `param_sets` always contains an entry for `selected_id`.
pub struct App {
    registry: Registry,
    selected_id: String,
    param_sets: BTreeMap<String, ParamSet>,
    pub param_cursor: usize,
    pub viewport: Viewport,
    pub color_mode: ColorMode,
    pub search: String,
}

impl App {
    pub fn new(registry: Registry, zoom: f64) -> Self {
        // [`App::tiling`] unwraps on an empty registry; fail fast here
        // with a clear message instead of panicking later on first render.
        assert!(!registry.is_empty(), "App requires at least one registered tiling");
        let first_id = registry.first().map(|t| t.id().to_string()).unwrap_or_default();
        let specs: &'static [ParamSpec] = registry.first().map(|t| t.params()).unwrap_or(&[]);
        let mut param_sets = BTreeMap::new();
        if !first_id.is_empty() {
            param_sets.insert(first_id.clone(), ParamSet::default_for(specs));
        }
        let color_mode = registry.first().map(|t| t.default_color_mode()).unwrap_or_default();
        Self {
            registry,
            selected_id: first_id,
            param_sets,
            param_cursor: 0,
            viewport: Viewport::new(zoom),
            color_mode,
            search: String::new(),
        }
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn selected_id(&self) -> &str {
        &self.selected_id
    }

    pub fn tiling(&self) -> &dyn Tiling {
        self.registry
            .get(&self.selected_id)
            .unwrap_or_else(|| self.registry.first().expect("empty registry"))
    }

    /// Tilings matching the search filter (id, name, or category label,
    /// case-insensitive).
    pub fn visible_tilings(&self) -> Vec<&dyn Tiling> {
        let q = self.search.trim().to_lowercase();
        self.registry
            .iter()
            .filter(|t| {
                q.is_empty()
                    || t.id().to_lowercase().contains(&q)
                    || t.name().to_lowercase().contains(&q)
                    || t.category().label().to_lowercase().contains(&q)
            })
            .collect()
    }

    /// The visible (search-filtered) tilings grouped by declared
    /// category, in canonical [`TilingCategory::ALL`] order. This is
    /// what UIs render as grouped lists; completeness (every visible
    /// tiling appears exactly once) is guaranteed by
    /// [`crate::registry::group_by_category`].
    pub fn visible_grouped(&self) -> Vec<(TilingCategory, Vec<&dyn Tiling>)> {
        group_by_category(self.visible_tilings())
    }

    /// Select a tiling by id. Parameters are reset to defaults the first
    /// time a tiling is selected in this session.
    pub fn select_id(&mut self, id: &str) {
        if self.registry.get(id).is_none() {
            return;
        }
        let specs: &'static [ParamSpec] = self.registry.get(id).unwrap().params();
        let color_mode = self.registry.get(id).unwrap().default_color_mode();
        self.selected_id = id.to_string();
        self.param_cursor = 0;
        self.color_mode = color_mode;
        self.param_sets.entry(self.selected_id.clone()).or_insert_with(|| {
            ParamSet::default_for(specs)
        });
    }

    /// Move the selection by `delta` within the visible (filtered) list, wrapping.
    pub fn move_selection(&mut self, delta: i32) {
        let visible = self.visible_tilings();
        if visible.is_empty() {
            return;
        }
        let cur = visible
            .iter()
            .position(|t| t.id() == self.selected_id)
            .unwrap_or(0);
        let n = visible.len() as i32;
        let next = ((cur as i32) + delta).rem_euclid(n) as usize;
        let id = visible[next].id();
        self.select_id(id);
    }

    pub fn params(&self) -> &ParamSet {
        self.param_sets
            .get(&self.selected_id)
            .expect("invariant: selected tiling has a param set")
    }

    pub fn param_value(&self, id: ParamId) -> Option<ParamValue> {
        self.params().get(id)
    }

    pub fn set_param_value(&mut self, id: ParamId, v: ParamValue) {
        self.params_mut().set(id, v);
    }

    pub fn move_param_cursor(&mut self, delta: i32) {
        let n = self.tiling().params().len();
        if n == 0 {
            return;
        }
        self.param_cursor = ((self.param_cursor as i32) + delta).rem_euclid(n as i32) as usize;
    }

    /// Adjust the parameter under the cursor by one step.
    pub fn adjust_param(&mut self, dir: i32) {
        let specs = self.tiling().params();
        if specs.is_empty() {
            return;
        }
        let spec = specs[self.param_cursor.min(specs.len() - 1)];
        let current = self.params().get(spec.id).unwrap_or_else(|| spec.default_value());
        let stepped = spec.step_by(current, dir);
        self.params_mut().set(spec.id, stepped);
    }

    fn params_mut(&mut self) -> &mut ParamSet {
        self.param_sets
            .get_mut(&self.selected_id)
            .expect("invariant: selected tiling has a param set")
    }

    pub fn pan(&mut self, dx_px: f64, dy_px: f64) {
        self.viewport.pan_screen(dx_px, dy_px);
    }

    pub fn zoom(&mut self, factor: f64, anchor: Option<Vec2>) {
        self.viewport.zoom_by(factor, anchor);
    }

    /// Reset the viewport center to the origin (keeps zoom).
    pub fn reset_view(&mut self) {
        self.viewport.center = Vec2::ZERO;
    }

    pub fn cycle_color_mode(&mut self) {
        self.color_mode = self.color_mode.next();
    }

    /// Generate tiles for the current viewport of `w` x `h` pixels.
    /// Generation cost varies enormously across tilings, view sizes and
    /// parameter values — a cheap periodic grid in a small window is
    /// instant, while a growth-based tiling at a wide zoom can take
    /// multiple seconds — so callers must not assume it is free. The
    /// GUI caches the result per (tiling id, params, viewport, canvas
    /// size) and reuses it across repaints that change none of those.
    pub fn result(&self, w: u32, h: u32) -> TilingResult {
        let region = self.viewport.world_bbox(w, h);
        self.result_region(region)
    }

    /// Generate tiles for an explicit region (independent of the
    /// viewport). Used by UIs for region-independent metadata (symmetry,
    /// periodic basis) so that panel rendering cannot trigger view-sized
    /// generation.
    pub fn result_region(&self, region: BBox) -> TilingResult {
        let params = self.params().clone();
        self.tiling().generate(&params, &Query { region })
    }

    /// The world window a small preview of the selected tiling should
    /// be rendered over, derived from the tiling itself: periodic
    /// tilings span a fixed multiple of their lattice basis, aperiodic
    /// ones span a fixed multiple of the mean tile extent measured by a
    /// bounded probe pass. The registry spans pitch scales from ~1
    /// world unit (square lattice) to ~80 (measured panels) — a fixed
    /// window would show either a single tile or a shapeless mass.
    ///
    /// `aspect` = preview pixel width / height.
    pub fn preview_region(&self, aspect: f64) -> BBox {
        // Bounded probe: ±10 world units around the origin. Aperiodic
        // patches grow from the origin; periodic tables return their
        // intersecting cells; nothing here can trigger view-sized
        // generation.
        let probe_region = BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0));
        let probe = self.result_region(probe_region);
        preview_window(probe.periodic, mean_tile_side(&probe.tiles), aspect)
    }

    /// Tiles for a preview of the selected tiling over a window
    /// derived by [`App::preview_region`], with the same shrink-and-
    /// retry on generation refusal as the main view. Returns the
    /// (possibly shrunken) window actually used alongside the tiles.
    pub fn preview_result(&self, aspect: f64) -> (TilingResult, BBox) {
        let mut window = self.preview_region(aspect);
        let mut result = self.result_region(window);
        let mut shrink = 0.4;
        while result.message.is_some() && shrink > 0.05 {
            let cx = (window.min.x + window.max.x) / 2.0;
            let cy = (window.min.y + window.max.y) / 2.0;
            let hx = (window.max.x - window.min.x) / 2.0 * shrink;
            let hy = (window.max.y - window.min.y) / 2.0 * shrink;
            window = BBox::new(Vec2::new(cx - hx, cy - hy), Vec2::new(cx + hx, cy + hy));
            result = self.result_region(window);
            shrink *= 0.4;
        }
        (result, window)
    }
}

/// Mean tile bounding-box side over `tiles` (0 if none usable).
pub fn mean_tile_side(tiles: &[crate::tiling::Tile]) -> f64 {
    let mut total = 0.0;
    let mut n = 0usize;
    for t in tiles.iter().take(60_000) {
        let bb = t.poly.aabb();
        let s = (bb.size().x + bb.size().y) / 2.0;
        if s.is_finite() && s > 0.0 {
            total += s;
            n += 1;
        }
    }
    if n == 0 { 0.0 } else { total / n as f64 }
}

/// The world window for a tiling preview (see
/// [`App::preview_region`]). `aspect` = pixel width / height.
pub fn preview_window(
    periodic: Option<[Vec2; 2]>,
    mean_tile: f64,
    aspect: f64,
) -> BBox {
    const PERIODIC_SPAN: f64 = 2.6;
    const APERIODIC_SPAN: f64 = 9.0;
    const FALLBACK_HALF: f64 = 8.0;
    const HALF_LIMITS: (f64, f64) = (0.2, 5000.0);
    let base = match periodic {
        Some([a1, a2]) => PERIODIC_SPAN * a1.length().max(a2.length()),
        None if mean_tile > 0.0 => APERIODIC_SPAN * mean_tile,
        _ => 2.0 * FALLBACK_HALF,
    };
    let half = (base / 2.0).clamp(HALF_LIMITS.0, HALF_LIMITS.1);
    // Aspect fit: the tight dimension keeps `half`; the other stretches
    // so the preview is filled edge to edge.
    let (hx, hy) = if aspect >= 1.0 {
        (half * aspect, half)
    } else {
        (half, half / aspect)
    };
    BBox::new(Vec2::new(-hx, -hy), Vec2::new(hx, hy))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::category::TilingCategory;
    use crate::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};

    struct Fixed {
        id: &'static str,
        cat: TilingCategory,
    }
    impl Tiling for Fixed {
        fn id(&self) -> &'static str { self.id }
        fn name(&self) -> &'static str { "Fixed" }
        fn description(&self) -> &'static str { "fixture" }
        fn category(&self) -> TilingCategory { self.cat }
        fn params(&self) -> &'static [ParamSpec] { &[] }
        fn generate(&self, _: &ParamSet, _: &Query) -> TilingResult {
            TilingResult {
                tiles: vec![],
                symmetry: SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None),
                periodic: None,
                message: None,
            }
        }
    }

    fn fixture() -> App {
        let reg = Registry::new()
            .register(Fixed { id: "u1", cat: TilingCategory::Uniform })
            .register(Fixed { id: "a1", cat: TilingCategory::Aperiodic })
            .register(Fixed { id: "u2", cat: TilingCategory::Uniform })
            .register(Fixed { id: "l1", cat: TilingCategory::Laves });
        App::new(reg, 10.0)
    }

    #[test]
    fn search_matches_category_label() {
        let mut app = fixture();
        app.search = "duals".into();
        let ids: Vec<&str> = app.visible_tilings().iter().map(|t| t.id()).collect();
        assert_eq!(ids, vec!["l1"]);
        app.search = "aperiodic".into();
        let ids: Vec<&str> = app.visible_tilings().iter().map(|t| t.id()).collect();
        assert_eq!(ids, vec!["a1"]);
        app.search = "  APERIODIC ".into(); // trimmed + case-insensitive
        let ids: Vec<&str> = app.visible_tilings().iter().map(|t| t.id()).collect();
        assert_eq!(ids, vec!["a1"]);
    }

    #[test]
    fn visible_grouped_is_canonical_and_complete() {
        let app = fixture();
        let g = app.visible_grouped();
        let cats: Vec<TilingCategory> = g.iter().map(|(c, _)| *c).collect();
        assert_eq!(
            cats,
            vec![TilingCategory::Uniform, TilingCategory::Laves, TilingCategory::Aperiodic]
        );
        assert_eq!(
            g[0].1.iter().map(|t| t.id()).collect::<Vec<_>>(),
            vec!["u1", "u2"],
            "registration order preserved within a category"
        );
        assert_eq!(g.iter().map(|(_, ts)| ts.len()).sum::<usize>(), 4);
    }

    fn bbox_half(b: BBox) -> (f64, f64) {
        ((b.max.x - b.min.x) / 2.0, (b.max.y - b.min.y) / 2.0)
    }

    #[test]
    fn preview_window_spans_lattice_basis() {
        let b = super::preview_window(
            Some([Vec2::new(3.0, 0.0), Vec2::new(0.0, 4.0)]),
            0.0,
            1.0,
        );
        let (hx, hy) = bbox_half(b);
        assert!((hx - 5.2).abs() < 1e-9 && (hy - 5.2).abs() < 1e-9);
        // Wide aspect stretches x only.
        let b = super::preview_window(
            Some([Vec2::new(3.0, 0.0), Vec2::new(0.0, 4.0)]),
            0.0,
            2.0,
        );
        let (hx, hy) = bbox_half(b);
        assert!((hx - 10.4).abs() < 1e-9 && (hy - 5.2).abs() < 1e-9);
    }

    #[test]
    fn preview_window_spans_mean_tile_and_falls_back() {
        let b = super::preview_window(None, 0.5, 1.0);
        let (hx, hy) = bbox_half(b);
        assert!((hx - 2.25).abs() < 1e-9 && (hy - 2.25).abs() < 1e-9);
        let b = super::preview_window(None, 0.0, 1.0);
        let (hx, _) = bbox_half(b);
        assert!((hx - 8.0).abs() < 1e-9);
        // Tall aspect stretches y only.
        let b = super::preview_window(None, 1.0, 0.5);
        let (hx, hy) = bbox_half(b);
        assert!((hx - 4.5).abs() < 1e-9 && (hy - 9.0).abs() < 1e-9);
    }

    #[test]
    fn mean_tile_side_ignores_degenerate_tiles() {
        let mut poly = crate::geometry::Polygon::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(2.0, 1.0),
            Vec2::new(0.0, 1.0),
        ]);
        let t = crate::tiling::Tile { poly: poly.clone(), proto: 0, seed: 0 };
        assert!((super::mean_tile_side(&[t]) - 1.5).abs() < 1e-9);
        poly = crate::geometry::Polygon::new(vec![Vec2::new(0.0, 0.0); 3]);
        let t = crate::tiling::Tile { poly, proto: 0, seed: 0 };
        assert_eq!(super::mean_tile_side(&[t]), 0.0);
        assert_eq!(super::mean_tile_side(&[]), 0.0);
    }

    #[test]
    fn preview_mode_reveals_structure() {
        assert_eq!(ColorMode::Single.preview_mode(), ColorMode::Proto);
        assert_eq!(ColorMode::Proto.preview_mode(), ColorMode::Proto);
        assert_eq!(ColorMode::Pattern.preview_mode(), ColorMode::Pattern);
        assert_eq!(ColorMode::Gray.preview_mode(), ColorMode::Gray);
    }
}
