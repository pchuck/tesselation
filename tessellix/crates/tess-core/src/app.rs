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
        app.search = "laves".into();
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
}
