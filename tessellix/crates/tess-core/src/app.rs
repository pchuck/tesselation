//! Shared application state, used by both the TUI and the GUI.

use std::collections::BTreeMap;

use crate::geometry::{BBox, Vec2};
use crate::params::{ParamId, ParamSet, ParamSpec, ParamValue};
use crate::registry::Registry;
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

    /// Tilings matching the search filter (id or name, case-insensitive).
    pub fn visible_tilings(&self) -> Vec<&dyn Tiling> {
        let q = self.search.trim().to_lowercase();
        self.registry
            .iter()
            .filter(|t| {
                q.is_empty()
                    || t.id().to_lowercase().contains(&q)
                    || t.name().to_lowercase().contains(&q)
            })
            .collect()
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
    /// Generation is cheap, so this regenerates on every call.
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
}
