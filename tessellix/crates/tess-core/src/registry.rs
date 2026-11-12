//! A registry of tiling generators.

use std::collections::HashMap;
use std::sync::Arc;

use crate::tiling::Tiling;

/// Collection of available tilings.
#[derive(Default)]
pub struct Registry {
    tilings: Vec<Arc<dyn Tiling>>,
    by_id: HashMap<&'static str, usize>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(mut self, t: impl Tiling + 'static) -> Self {
        debug_assert!(
            !self.by_id.contains_key(t.id()),
            "duplicate tiling id '{}' (would shadow the earlier registration)",
            t.id()
        );
        self.tilings.push(Arc::new(t));
        self.by_id.insert(self.tilings.last().unwrap().id(), self.tilings.len() - 1);
        self
    }

    pub fn len(&self) -> usize {
        self.tilings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tilings.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &dyn Tiling> {
        self.tilings.iter().map(|t| t.as_ref())
    }

    pub fn get(&self, id: &str) -> Option<&dyn Tiling> {
        self.by_id.get(id).map(|&i| self.tilings[i].as_ref())
    }

    pub fn first(&self) -> Option<&dyn Tiling> {
        self.tilings.first().map(|t| t.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::ParamSet;
    use crate::tiling::{Query, TilingResult};
    use crate::symmetry::{LatticeKind, SymmetryMeta};
    use crate::view::ColorMode;

    struct Dummy;
    impl Tiling for Dummy {
        fn id(&self) -> &'static str { "dummy" }
        fn name(&self) -> &'static str { "Dummy" }
        fn description(&self) -> &'static str { "test" }
        fn params(&self) -> &'static [crate::params::ParamSpec] { &[] }
        fn default_color_mode(&self) -> ColorMode { ColorMode::Single }
        fn generate(&self, _: &ParamSet, _: &Query) -> TilingResult {
            TilingResult {
                tiles: vec![],
                symmetry: SymmetryMeta::new(crate::symmetry::WallpaperGroup::P1, LatticeKind::None),
                periodic: None,
                message: None,
            }
        }
    }

    #[test]
    fn register_and_lookup() {
        let r = Registry::new().register(Dummy);
        assert_eq!(r.len(), 1);
        assert!(r.get("dummy").is_some());
        assert!(r.get("nope").is_none());
        assert_eq!(r.first().unwrap().id(), "dummy");
    }
}
