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
use crate::classify::TilingClass;
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
    /// The declared classification schema record ([`TilingClass`]).
    /// Empty for unclassified tilings — UIs render only the fields
    /// that are present, never a placeholder.
    pub class: TilingClass,
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
/// centered rectangular, square, hexagonal; non-lattice entries (if any)
/// sort last.
pub fn lattice_rank(l: LatticeKind) -> u8 {
    match l {
        LatticeKind::Oblique => 0,
        LatticeKind::Rectangular => 1,
        LatticeKind::CenteredRectangular => 2,
        LatticeKind::Square => 3,
        LatticeKind::Hexagonal => 4,
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

/// The crystal-system section key for a card displayed in the tab
/// `tab_category`: the header text a card grid should draw above the
/// first card of each section. Only the Wallpaper tab has sections —
/// the group catalog is sorted by crystal system, and the key is the
/// card's *conventional* lattice (see
/// [`WallpaperGroup::conventional_lattice`]): the textbook mapping
/// p1/p2 → oblique, pm/pg/pmm/pgg/pmg → rectangular, cm/cmm →
/// centered, p4* → square, p3*/p6* → hexagonal. This is deliberately
/// a display convention derived from the declared group, not from the
/// representative's measured lattice: a p1 tiling realized on a
/// centered-rectangular primitive cell (the stepped hexominoes)
/// still displays under oblique, together with p2, as the textbooks
/// lay out the 17 groups. The measured per-tiling lattice kind stays
/// on the card (it is the data-honest attribute line). Cards in every
/// other tab, and cards with no declared group, return `""` (no
/// header).
pub fn section_key(tab_category: TilingCategory, card: &CardMeta) -> &'static str {
    if tab_category != TilingCategory::Wallpaper {
        return "";
    }
    match card.group.map(WallpaperGroup::conventional_lattice) {
        Some(LatticeKind::Oblique) => "oblique",
        Some(LatticeKind::Rectangular) => "rectangular",
        Some(LatticeKind::CenteredRectangular) => "centered",
        Some(LatticeKind::Square) => "square",
        Some(LatticeKind::Hexagonal) => "hexagonal",
        _ => "",
    }
}

/// The wallpaper groups with no registered realizer (M76). The Wallpaper
/// tab renders one card per group it does have; both UIs show the groups
/// in this table as an explicit gap note in that tab. As roadmap tilings
/// land, groups are removed from this table — M85 (alcove-cut grid,
/// p3m1) closed the last gap, so the table is empty and the Wallpaper
/// tab's one-card-per-group catalog is complete: all 17 wallpaper
/// groups are represented.
pub const WALLPAPER_GROUP_GAPS: [WallpaperGroup; 0] = [];

/// The catalog note the Wallpaper tab renders above its cards: while
/// [`WALLPAPER_GROUP_GAPS`] is non-empty it lists the groups with no
/// registered realizer yet (roadmap queue); since M85 the table is
/// empty and the note reports the completed catalog instead. Both UIs
/// derive this text from the gap table.
pub fn wallpaper_gap_note() -> String {
    if WALLPAPER_GROUP_GAPS.is_empty() {
        "complete catalog: all 17 wallpaper groups have a registered realizer (M85 closed the p3m1 gap)"
            .to_string()
    } else {
        let names: Vec<String> = WALLPAPER_GROUP_GAPS.iter().map(ToString::to_string).collect();
        format!("no realizer registered for: {} (roadmap queue)", names.join(" · "))
    }
}

/// Capture a tiling's declared metadata as card data. Symmetry facts are
/// read from a bounded probe generation (see the module docs).
///
/// The group/lattice fields are set for the periodic categories (every
/// category whose cards are wallpaper tilings); they are `None` for the
/// nonperiodic world (quasiperiodic / aperiodic / nonperiodic), whose
/// generators report `P1` / `None` as a "not a wallpaper tiling"
/// placeholder, not as a claim of p1 symmetry.
pub fn card_meta(t: &dyn Tiling) -> CardMeta {
    card_meta_with(t, TilingClass::EMPTY)
}

/// [`card_meta`] with a declared [`TilingClass`] attached (used by
/// [`browse_tabs_with`]; classification is host-supplied so tess-core
/// never encodes tiling ids).
pub fn card_meta_with(t: &dyn Tiling, class: TilingClass) -> CardMeta {
    let params = ParamSet::default_for(t.params());
    let region = BBox::new(PROBE_REGION.0, PROBE_REGION.1);
    let result = t.generate(&params, &Query { region });
    let periodic = matches!(
        t.category(),
        TilingCategory::Uniform
            | TilingCategory::TwoUniform
            | TilingCategory::Laves
            | TilingCategory::Wallpaper
            | TilingCategory::PentagonTypes
            | TilingCategory::Islamic
            | TilingCategory::Arrangements
    );
    CardMeta {
        id: t.id(),
        name: t.name(),
        description: t.description(),
        category: t.category(),
        group: periodic.then_some(result.symmetry.group),
        lattice: periodic.then_some(result.symmetry.lattice),
        notes: result.symmetry.notes,
        params: t.params().len(),
        class,
    }
}

/// Build the browser: one [`Tab`] per category in canonical
/// [`TilingCategory::ALL`] order (all tabs, including any that are
/// empty — a stable tab bar), with no classification attached (see
/// [`browse_tabs_with`]).
pub fn browse_tabs(registry: &Registry) -> Vec<Tab> {
    browse_tabs_with(registry, |_| TilingClass::EMPTY)
}

/// [`browse_tabs`] with a classification lookup: `classify(id)` supplies
/// each card's [`TilingClass`] (the host injects its per-tiling table so
/// tess-core never encodes tiling ids), and the declared schema fields
/// drive the per-tab display ordering:
///
/// * **Wallpaper** — by (conventional lattice of the declared group,
///   group rank): the textbook crystal-system sections.
/// * **Pentagon Types** — by the declared `pentagon_type` (sequential
///   type order; untyped cards keep registry order at the end).
/// * **Two-Uniform** — by the declared `vertex_config` string (pairs of
///   a configuration sort together; config-less cards keep registry
///   order at the end).
/// * **Laves** — by the declared `laves_rank` (self-dual regulars, then
///   the Laves proper, then the 2-uniform duals).
///
/// Every other tab keeps membership order (primary cards in registry
/// order, then secondaries). All sorts are stable.
pub fn browse_tabs_with(
    registry: &Registry,
    classify: impl Fn(&str) -> TilingClass,
) -> Vec<Tab> {
    let mut tabs = Vec::with_capacity(TilingCategory::ALL.len());
    for cat in TilingCategory::ALL {
        let mut cards: Vec<CardMeta> = registry
            .iter()
            .filter(|t| t.category() == cat)
            .map(|t| card_meta_with(t, classify(t.id())))
            .collect();
        for t in registry.iter().filter(|t| {
            t.category() != cat && t.secondary_categories().contains(&cat)
        }) {
            cards.push(card_meta_with(t, classify(t.id())));
        }
        match cat {
            TilingCategory::Wallpaper => {
                cards.sort_by_key(|c| {
                    (
                        lattice_rank(
                            c.group
                                .map(WallpaperGroup::conventional_lattice)
                                .unwrap_or(LatticeKind::None),
                        ),
                        group_rank(c.group),
                    )
                });
            }
            TilingCategory::PentagonTypes => {
                cards.sort_by_key(|c| c.class.pentagon_type.unwrap_or(u8::MAX));
            }
            TilingCategory::TwoUniform => {
                cards.sort_by_key(|c| c.class.vertex_config.unwrap_or("\u{10FFFF}"));
            }
            TilingCategory::Laves => {
                cards.sort_by_key(|c| c.class.laves_rank.unwrap_or(u8::MAX));
            }
            _ => {}
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
        secondary: &'static [TilingCategory],
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
        fn secondary_categories(&self) -> &'static [TilingCategory] {
            self.secondary
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
            secondary: &[],
            group: WallpaperGroup::P6m,
            lattice: LatticeKind::Hexagonal,
            params: 0,
        });
        r = r.register(Fixed {
            id: "obl-a",
            cat: TilingCategory::Wallpaper,
            secondary: &[],
            group: WallpaperGroup::P1,
            lattice: LatticeKind::Oblique,
            params: 0,
        });
        r = r.register(Fixed {
            id: "sq-b",
            cat: TilingCategory::Wallpaper,
            secondary: &[],
            group: WallpaperGroup::P2,
            lattice: LatticeKind::Square,
            params: 0,
        });
        r = r.register(Fixed {
            id: "sq-a",
            cat: TilingCategory::Wallpaper,
            secondary: &[],
            group: WallpaperGroup::P1,
            lattice: LatticeKind::Square,
            params: 0,
        });
        r = r.register(Fixed {
            id: "ctr-a",
            cat: TilingCategory::Wallpaper,
            secondary: &[],
            group: WallpaperGroup::Cmm,
            lattice: LatticeKind::CenteredRectangular,
            params: 0,
        });
        r = r.register(Fixed {
            id: "primsq-a",
            cat: TilingCategory::Wallpaper,
            secondary: &[],
            group: WallpaperGroup::Pmm,
            lattice: LatticeKind::Rectangular,
            params: 0,
        });
        r = r.register(Fixed {
            id: "u1",
            cat: TilingCategory::Uniform,
            secondary: &[],
            group: WallpaperGroup::P4m,
            lattice: LatticeKind::Square,
            params: 0,
        });
        r = r.register(Fixed {
            id: "a1",
            cat: TilingCategory::Aperiodic,
            secondary: &[],
            group: WallpaperGroup::P1,
            lattice: LatticeKind::None,
            params: 0,
        });
        // Laves-primary with a Wallpaper secondary membership (the
        // multi-tab member); registered last so it ties hex-a in the
        // stable Wallpaper sort.
        r = r.register(Fixed {
            id: "l1",
            cat: TilingCategory::Laves,
            secondary: &[TilingCategory::Wallpaper],
            group: WallpaperGroup::P6m,
            lattice: LatticeKind::Hexagonal,
            params: 0,
        });
        r
    }

    #[test]
    fn tabs_cover_every_tiling_through_membership() {
        let r = reg();
        let tabs = browse_tabs(&r);
        assert_eq!(tabs.len(), TilingCategory::ALL.len());
        for (i, c) in TilingCategory::ALL.iter().enumerate() {
            assert_eq!(tabs[i].category, *c);
        }
        // Membership semantics: one card per primary category, plus one
        // card per declared secondary (l1's Wallpaper membership).
        let total: usize = tabs.iter().map(|t| t.cards.len()).sum();
        assert_eq!(total, r.len() + 1, "one secondary membership");
        let mut ids: Vec<&str> = tabs
            .iter()
            .flat_map(|t| t.cards.iter().map(|c| c.id))
            .collect();
        let distinct = ids.len();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), r.len(), "no tiling dropped");
        assert_eq!(distinct, r.len() + 1, "l1 appears in exactly two tabs");
        for t in r.iter() {
            let primary = tabs
                .iter()
                .find(|tab| tab.category == t.category())
                .unwrap();
            assert!(primary.card_index_of(t.id()).is_some(), "primary tab missing {}", t.id());
            for sec in t.secondary_categories() {
                let tab = tabs.iter().find(|tab| tab.category == *sec).unwrap();
                assert!(tab.card_index_of(t.id()).is_some(), "secondary tab {} missing {}", sec, t.id());
            }
        }
    }

    #[test]
    fn wallpaper_ordering_is_conventional_lattice_then_group() {
        let r = reg();
        let tabs = browse_tabs(&r);
        let wall = tabs
            .iter()
            .find(|t| t.category == TilingCategory::Wallpaper)
            .unwrap();
        let ids: Vec<&str> = wall.cards.iter().map(|c| c.id).collect();
        // Sections follow the CONVENTIONAL lattice of the declared
        // group (not the representative's measured lattice): the P1
        // card on a square lattice (sq-a) and the P2 card (sq-b) both
        // display under oblique with obl-a; then rectangular (pmm),
        // centered (cmm), hexagonal (the P6m tie between hex-a and l1
        // keeps registry order).
        assert_eq!(
            ids,
            vec!["obl-a", "sq-a", "sq-b", "primsq-a", "ctr-a", "hex-a", "l1"]
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
        // The Laves primary card (l1) is here in registry order; its
        // Wallpaper secondary is sorted in the Wallpaper tab.
        let lav = tabs
            .iter()
            .find(|t| t.category == TilingCategory::Laves)
            .unwrap();
        assert_eq!(lav.cards.iter().map(|c| c.id).collect::<Vec<_>>(), vec!["l1"]);
        // Categories with no members at all still appear (stable tab
        // bar).
        let qp = tabs
            .iter()
            .find(|t| t.category == TilingCategory::Quasiperiodic)
            .unwrap();
        assert!(qp.cards.is_empty());
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
                .map(|c| section_key(TilingCategory::Wallpaper, c))
                .unwrap()
        };
        assert_eq!(key("obl-a"), "oblique");
        assert_eq!(key("primsq-a"), "rectangular");
        assert_eq!(key("ctr-a"), "centered");
        // sq-a declares a square lattice but a p1 group: the section
        // follows the group's conventional lattice (oblique), while
        // the measured lattice stays on the card's attribute line.
        assert_eq!(key("sq-a"), "oblique");
        assert_eq!(key("hex-a"), "hexagonal");
        // Cards in other tabs have no section, whatever their lattice.
        let uni = &tabs
            .iter()
            .find(|t| t.category == TilingCategory::Uniform)
            .unwrap()
            .cards[0];
        assert_eq!(section_key(TilingCategory::Uniform, uni), "");
        let a1 = &tabs
            .iter()
            .find(|t| t.category == TilingCategory::Aperiodic)
            .unwrap()
            .cards[0];
        assert_eq!(section_key(TilingCategory::Aperiodic, a1), "");
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
            let k = section_key(TilingCategory::Wallpaper, c);
            if !k.is_empty() && !seen.contains(&k) {
                seen.push(k);
            }
        }
        assert_eq!(
            seen,
            vec!["oblique", "rectangular", "centered", "hexagonal"],
            "sections must appear in conventional crystal-system order"
        );
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

    #[test]
    fn wallpaper_group_gaps_are_exactly_the_unrepresented_groups() {
        // The gap table plus the groups represented by the real registry
        // must partition the 17 groups; the 17 represented groups are
        // pinned by the tess-tilings census test, which cross-checks the
        // table against the actual declarations. M85 (alcove-cut grid)
        // closed the last gap — p3m1 — so the table is empty and the
        // shared note reports the completed catalog.
        assert_eq!(
            WALLPAPER_GROUP_GAPS.len(),
            0,
            "the p3m1 gap was closed in M85"
        );
        let mut gaps = WALLPAPER_GROUP_GAPS.to_vec();
        gaps.sort_unstable_by_key(|g| group_rank(Some(*g)));
        let n = gaps.len();
        gaps.dedup();
        assert_eq!(gaps.len(), n, "gap table has no duplicates");
        for g in WALLPAPER_GROUP_GAPS {
            assert!(
                WallpaperGroup::ALL.contains(&g),
                "gap table lists non-wallpaper group {g}"
            );
        }
        assert_eq!(
            wallpaper_gap_note(),
            "complete catalog: all 17 wallpaper groups have a registered realizer (M85 closed the p3m1 gap)"
        );
    }
}
