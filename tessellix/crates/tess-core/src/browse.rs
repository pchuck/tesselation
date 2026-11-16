//! The shared browser model: category tabs, per-tab card ordering, and
//! crystal-system section keys for the tiling catalog.
//!
//! Both front ends consume this model: the terminal UI (tess-tui) renders
//! the tabs as a tab bar over a card grid, and the desktop GUI (tess-gui)
//! will consume the same `browse_tabs` output. The model stays generic —
//! it reads only declared tiling metadata (id, name, description,
//! category, parameters, symmetry) and never encodes tiling ids or names.
//!
//! Symmetry metadata is declared by each generator but surfaced through
//! its generation result (the [`Tiling`] trait has no separate symmetry
//! query), so [`card_meta`] captures it from a small bounded probe
//! generation: the same ±10-world-unit probe [`App::preview_region`]
//! uses, so no probe can trigger view-sized generation.
//!
//! [`Tiling`]: crate::tiling::Tiling
//! [`App::preview_region`]: crate::app::App::preview_region

use crate::category::TilingCategory;
use crate::geometry::{BBox, Vec2};
use crate::params::ParamSet;
use crate::registry::Registry;
use crate::symmetry::{LatticeKind, WallpaperGroup};
use crate::tiling::{Query, Tiling};

/// Bounded probe region used to capture a tiling's declared symmetry
/// metadata without generating view-sized patches.
const PROBE_REGION: (Vec2, Vec2) = (Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0));

/// Display metadata for one tiling card.
///
/// `group`/`lattice` are the wallpaper-group and lattice kind declared by
/// the tiling's generator. They are `None` for non-wallpaper categories
/// (quasiperiodic / aperiodic / nonperiodic): those generators report
/// `P1` / `None` as a "not a wallpaper tiling" placeholder, not as a
/// claim of p1 symmetry, so the model leaves them unset. `notes` carries
/// the generator's free-text symmetry note for every category.
pub struct CardMeta {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub category: TilingCategory,
    pub group: Option<WallpaperGroup>,
    pub lattice: Option<LatticeKind>,
    pub notes: Option<&'static str>,
    /// Number of declared parameters (`0` for fixed tilings).
    pub params: usize,
}

impl CardMeta {
    /// The declared wallpaper group as a string (`"—"` when unset), for
    /// one-line attribute displays.
    pub fn group_str(&self) -> String {
        self.group
            .map(|g| g.to_string())
            .unwrap_or_else(|| "—".to_string())
    }

    /// The declared lattice kind as a string (`"—"` when unset).
    pub fn lattice_str(&self) -> String {
        self.lattice
            .map(|l| l.to_string())
            .unwrap_or_else(|| "—".to_string())
    }
}

/// One tab of the browser: all cards of one declared category, in
/// display order (see [`browse_tabs`]).
pub struct Tab {
    pub category: TilingCategory,
    pub cards: Vec<CardMeta>,
}

impl Tab {
    pub fn card_count(&self) -> usize {
        self.cards.len()
    }

    /// Card by id, or `None` (the UI keeps its cursor by position).
    pub fn card_index_of(&self, id: &str) -> Option<usize> {
        self.cards.iter().position(|c| c.id == id)
    }
}

/// The display order of lattice kinds (crystal systems), used as the
/// primary sort key within the Wallpaper tab: oblique, rectangular,
/// square, hexagonal; non-lattice entries (if any) sort last.
pub fn lattice_rank(l: LatticeKind) -> u8 {
    match l {
        LatticeKind::Oblique => 0,
        LatticeKind::Rectangular => 1,
        LatticeKind::Square => 2,
        LatticeKind::Hexagonal => 3,
        LatticeKind::None => 9,
    }
}

/// The display rank of a wallpaper group: its position in
/// [`WallpaperGroup::ALL`] (crystallographic order P1..P6m). Unset
/// groups sort after all declared ones.
pub fn group_rank(g: Option<WallpaperGroup>) -> u8 {
    match g {
        Some(g) => WallpaperGroup::ALL.iter().position(|a| *a == g).unwrap_or(255) as u8,
        None => 255,
    }
}

/// The crystal-system section key for a card: the header text a card
/// grid should draw above the first card of each section. Only Wallpaper
/// cards have non-empty sections; centered-rectangular groups (Cm, Cmm,
/// Cmc) form their own "centered" section, distinct from primitive
/// rectangular. Non-wallpaper cards and lattice-less Wallpaper cards
/// return `""` (no header).
pub fn section_key(card: &CardMeta) -> &'static str {
    if card.category != TilingCategory::Wallpaper {
        return "";
    }
    match card.lattice {
        Some(LatticeKind::Oblique) => "oblique",
        Some(LatticeKind::Rectangular) => {
            match card.group {
                Some(WallpaperGroup::Cm | WallpaperGroup::Cmm | WallpaperGroup::Cmc) => {
                    "centered"
                }
                _ => "rectangular",
            }
        }
        Some(LatticeKind::Square) => "square",
        Some(LatticeKind::Hexagonal) => "hexagonal",
        _ => "",
    }
}

/// Capture a tiling's declared metadata as card data. Symmetry facts are
/// read from a bounded probe generation (see the module docs).
pub fn card_meta(t: &dyn Tiling) -> CardMeta {
    let params = ParamSet::default_for(t.params());
    let region = BBox::new(PROBE_REGION.0, PROBE_REGION.1);
    let result = t.generate(&params, &Query { region });
    let wallpaper = matches!(
        t.category(),
        TilingCategory::Uniform | TilingCategory::Laves | TilingCategory::Wallpaper
    );
    CardMeta {
        id: t.id(),
        name: t.name(),
        description: t.description(),
        category: t.category(),
        group: wallpaper.then_some(result.symmetry.group),
        lattice: wallpaper.then_some(result.symmetry.lattice),
        notes: result.symmetry.notes,
        params: t.params().len(),
    }
}

/// Build the browser: one [`Tab`] per category in canonical
/// [`TilingCategory::ALL`] order (all six tabs, including any that are
/// empty — a stable tab bar).
///
/// Ordering: the Wallpaper tab is sorted by (lattice rank, group rank),
/// i.e. crystal system first (oblique, rectangular, square, hexagonal)
/// and wallpaper group within a crystal system; ties keep registry
/// order (the sort is stable). Every other tab keeps registry order.
pub fn browse_tabs(registry: &Registry) -> Vec<Tab> {
    let mut tabs = Vec::with_capacity(TilingCategory::ALL.len());
    for cat in TilingCategory::ALL {
        let mut cards: Vec<CardMeta> = registry
            .iter()
            .filter(|t| t.category() == cat)
            .map(card_meta)
            .collect();
        if cat == TilingCategory::Wallpaper {
            cards.sort_by_key(|c| {
                (
                    lattice_rank(c.lattice.unwrap_or(LatticeKind::None)),
                    group_rank(c.group),
                )
            });
        }
        tabs.push(Tab { category: cat, cards });
    }
    tabs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::ParamSpec;
    use crate::symmetry::SymmetryMeta;
    use crate::tiling::TilingResult;
    use crate::view::ColorMode;

    struct Fixed {
        id: &'static str,
        cat: TilingCategory,
        group: WallpaperGroup,
        lattice: LatticeKind,
        params: usize,
    }

    impl Tiling for Fixed {
        fn id(&self) -> &'static str {
            self.id
        }
        fn name(&self) -> &'static str {
            "Fixed"
        }
        fn description(&self) -> &'static str {
            "fixture tiling"
        }
        fn category(&self) -> TilingCategory {
            self.cat
        }
        fn params(&self) -> &'static [ParamSpec] {
            match self.params {
                0 => &[],
                _ => panic!("fixture only declares 0 params"),
            }
        }
        fn default_color_mode(&self) -> ColorMode {
            ColorMode::Single
        }
        fn generate(&self, _: &ParamSet, _: &Query) -> TilingResult {
            TilingResult {
                tiles: vec![],
                symmetry: SymmetryMeta::new(self.group, self.lattice),
                periodic: None,
                message: None,
            }
        }
    }

    fn reg() -> Registry {
        // Deliberately out of (lattice, group) display order.
        let mut r = Registry::new();
        r = r.register(Fixed {
            id: "hex-a",
            cat: TilingCategory::Wallpaper,
            group: WallpaperGroup::P6m,
            lattice: LatticeKind::Hexagonal,
            params: 0,
        });
        r = r.register(Fixed {
            id: "obl-a",
            cat: TilingCategory::Wallpaper,
            group: WallpaperGroup::P1,
            lattice: LatticeKind::Oblique,
            params: 0,
        });
        r = r.register(Fixed {
            id: "sq-b",
            cat: TilingCategory::Wallpaper,
            group: WallpaperGroup::P2,
            lattice: LatticeKind::Square,
            params: 0,
        });
        r = r.register(Fixed {
            id: "sq-a",
            cat: TilingCategory::Wallpaper,
            group: WallpaperGroup::P1,
            lattice: LatticeKind::Square,
            params: 0,
        });
        r = r.register(Fixed {
            id: "ctr-a",
            cat: TilingCategory::Wallpaper,
            group: WallpaperGroup::Cmm,
            lattice: LatticeKind::Rectangular,
            params: 0,
        });
        r = r.register(Fixed {
            id: "primsq-a",
            cat: TilingCategory::Wallpaper,
            group: WallpaperGroup::Pmm,
            lattice: LatticeKind::Rectangular,
            params: 0,
        });
        r = r.register(Fixed {
            id: "u1",
            cat: TilingCategory::Uniform,
            group: WallpaperGroup::P4m,
            lattice: LatticeKind::Square,
            params: 0,
        });
        r = r.register(Fixed {
            id: "a1",
            cat: TilingCategory::Aperiodic,
            group: WallpaperGroup::P1,
            lattice: LatticeKind::None,
            params: 0,
        });
        r
    }

    #[test]
    fn tabs_cover_every_tiling_exactly_once() {
        let r = reg();
        let tabs = browse_tabs(&r);
        assert_eq!(tabs.len(), TilingCategory::ALL.len());
        for (i, c) in TilingCategory::ALL.iter().enumerate() {
            assert_eq!(tabs[i].category, *c);
        }
        let total: usize = tabs.iter().map(|t| t.cards.len()).sum();
        assert_eq!(total, r.len());
        let mut ids: Vec<&str> = tabs
            .iter()
            .flat_map(|t| t.cards.iter().map(|c| c.id))
            .collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), r.len(), "no card dropped or duplicated");
    }

    #[test]
    fn wallpaper_ordering_is_lattice_then_group() {
        let r = reg();
        let tabs = browse_tabs(&r);
        let wall = tabs
            .iter()
            .find(|t| t.category == TilingCategory::Wallpaper)
            .unwrap();
        let ids: Vec<&str> = wall.cards.iter().map(|c| c.id).collect();
        // oblique, then rectangular (Pmm before Cmm), then square (P1
        // before P2), then hexagonal: registry order is broken exactly
        // as the (lattice rank, group rank) keys demand.
        assert_eq!(
            ids,
            vec!["obl-a", "primsq-a", "ctr-a", "sq-a", "sq-b", "hex-a"]
        );
    }

    #[test]
    fn other_tabs_keep_registry_order() {
        let r = reg();
        let tabs = browse_tabs(&r);
        let uni = tabs
            .iter()
            .find(|t| t.category == TilingCategory::Uniform)
            .unwrap();
        assert_eq!(uni.cards.iter().map(|c| c.id).collect::<Vec<_>>(), vec!["u1"]);
        // Empty categories still appear (stable tab bar).
        let lav = tabs
            .iter()
            .find(|t| t.category == TilingCategory::Laves)
            .unwrap();
        assert!(lav.cards.is_empty());
    }

    #[test]
    fn ordering_is_stable_across_calls() {
        let r = reg();
        let a = browse_tabs(&r);
        let b = browse_tabs(&r);
        let seq = |tabs: &Vec<Tab>| {
            tabs.iter()
                .flat_map(|t| t.cards.iter().map(|c| c.id))
                .collect::<Vec<_>>()
        };
        assert_eq!(seq(&a), seq(&b));
    }

    #[test]
    fn section_keys_follow_crystal_systems() {
        let r = reg();
        let tabs = browse_tabs(&r);
        let wall = &tabs
            .iter()
            .find(|t| t.category == TilingCategory::Wallpaper)
            .unwrap()
            .cards;
        let key = |id: &str| {
            wall.iter()
                .find(|c| c.id == id)
                .map(|c| section_key(c))
                .unwrap()
        };
        assert_eq!(key("obl-a"), "oblique");
        assert_eq!(key("primsq-a"), "rectangular");
        assert_eq!(key("ctr-a"), "centered");
        assert_eq!(key("sq-a"), "square");
        assert_eq!(key("hex-a"), "hexagonal");
        // Non-wallpaper cards have no section.
        let uni = &tabs
            .iter()
            .find(|t| t.category == TilingCategory::Uniform)
            .unwrap()
            .cards[0];
        assert_eq!(section_key(uni), "");
        let a1 = &tabs
            .iter()
            .find(|t| t.category == TilingCategory::Aperiodic)
            .unwrap()
            .cards[0];
        assert_eq!(section_key(a1), "");
    }

    #[test]
    fn section_keys_appear_in_first_appearance_order() {
        // Walk the sorted Wallpaper cards; the distinct non-empty keys
        // must come out in first-appearance order, matching the sort.
        let r = reg();
        let tabs = browse_tabs(&r);
        let wall = &tabs
            .iter()
            .find(|t| t.category == TilingCategory::Wallpaper)
            .unwrap()
            .cards;
        let mut seen: Vec<&'static str> = Vec::new();
        for c in wall {
            let k = section_key(c);
            if !k.is_empty() && !seen.contains(&k) {
                seen.push(k);
            }
        }
        assert_eq!(seen, vec!["oblique", "rectangular", "centered", "square", "hexagonal"]);
    }

    #[test]
    fn non_wallpaper_groups_are_not_declared() {
        let r = reg();
        let tabs = browse_tabs(&r);
        // Aperiodic P1/None is a placeholder, not a symmetry claim.
        let a1 = &tabs
            .iter()
            .find(|t| t.category == TilingCategory::Aperiodic)
            .unwrap()
            .cards[0];
        assert_eq!(a1.group, None);
        assert_eq!(a1.lattice, None);
        // Uniform keeps its declared group/lattice.
        let u1 = &tabs
            .iter()
            .find(|t| t.category == TilingCategory::Uniform)
            .unwrap()
            .cards[0];
        assert_eq!(u1.group, Some(WallpaperGroup::P4m));
        assert_eq!(u1.lattice, Some(LatticeKind::Square));
    }
}
