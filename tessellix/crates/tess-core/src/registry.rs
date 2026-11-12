//! A registry of tiling generators.

use std::collections::HashMap;
use std::sync::Arc;

use crate::category::TilingCategory;
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

    /// All tilings grouped by declared category (see
    /// [`group_by_category`]).
    pub fn grouped(&self) -> Vec<(TilingCategory, Vec<&dyn Tiling>)> {
        group_by_category(self.iter())
    }
}

/// Group tilings by declared category in canonical [`TilingCategory::ALL`]
/// order, preserving the input order within each category. Categories
/// with no members are omitted. Completeness — every input appears
/// exactly once — is what the UIs rely on.
pub fn group_by_category<'a>(
    tilings: impl IntoIterator<Item = &'a dyn Tiling>,
) -> Vec<(TilingCategory, Vec<&'a dyn Tiling>)> {
    let mut groups: Vec<(TilingCategory, Vec<&dyn Tiling>)> = Vec::new();
    for t in tilings {
        let cat = t.category();
        match groups.iter_mut().find(|(c, _)| *c == cat) {
            Some((_, v)) => v.push(t),
            None => groups.push((cat, vec![t])),
        }
    }
    groups.sort_by_key(|(c, _)| {
        TilingCategory::ALL
            .iter()
            .position(|a| a == c)
            .expect("category is an ALL variant")
    });
    groups
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
        fn category(&self) -> crate::category::TilingCategory {
            crate::category::TilingCategory::Wallpaper
        }
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

    #[test]
    fn grouped_is_canonical_complete_and_order_preserving() {
        use crate::category::TilingCategory;
        use crate::params::ParamSet;
        use crate::symmetry::{LatticeKind, WallpaperGroup};

        struct Fixed {
            id: &'static str,
            cat: TilingCategory,
        }
        impl Tiling for Fixed {
            fn id(&self) -> &'static str { self.id }
            fn name(&self) -> &'static str { "Fixed" }
            fn description(&self) -> &'static str { "fixture" }
            fn category(&self) -> TilingCategory { self.cat }
            fn params(&self) -> &'static [crate::params::ParamSpec] { &[] }
            fn generate(&self, _: &ParamSet, _: &crate::tiling::Query) -> TilingResult {
                TilingResult {
                    tiles: vec![],
                    symmetry: SymmetryMeta::new(WallpaperGroup::P1, LatticeKind::None),
                    periodic: None,
                    message: None,
                }
            }
        }

        let r = Registry::new()
            .register(Fixed { id: "u1", cat: TilingCategory::Uniform })
            .register(Fixed { id: "a1", cat: TilingCategory::Aperiodic })
            .register(Fixed { id: "u2", cat: TilingCategory::Uniform })
            .register(Fixed { id: "l1", cat: TilingCategory::Laves });

        let g = r.grouped();
        // Canonical order: ALL subsequence (empty categories omitted).
        let cats: Vec<TilingCategory> = g.iter().map(|(c, _)| *c).collect();
        assert_eq!(
            cats,
            vec![TilingCategory::Uniform, TilingCategory::Laves, TilingCategory::Aperiodic]
        );
        // Registration order preserved within a category.
        let u: Vec<&str> = g[0].1.iter().map(|t| t.id()).collect();
        assert_eq!(u, vec!["u1", "u2"]);
        // Partition: nothing dropped or duplicated.
        assert_eq!(g.iter().map(|(_, ts)| ts.len()).sum::<usize>(), 4);
    }
}
