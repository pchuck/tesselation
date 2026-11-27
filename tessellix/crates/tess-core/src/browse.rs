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
//! One display note: when the schema adds nothing to a card's line 2
//! (every declared fact is already echoed by the name), the summary
//! falls back to the description fragment. The fragment is cut at the
//! last word boundary within the one-line card budget (with an
//! ellipsis), never mid-word — a hard character cut overflowed the
//! ~35-char GUI window for cards like the golden-rhomboid-triangle,
//! whose name echoes every declared segment.
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
#[derive(Debug, Clone, Copy)]
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

/// The Arrangements tab rank: the paver-bond cards (the bond catalog
/// and its variants) first, then the Pythagorean section (the
/// windmill dissection and the Pythagorean square packing). The bucket
/// is derived from the declared `method` — the paver-bond cards are
/// exactly the ones whose method is the paver bond. Cards with no
/// declared method keep registry order at the end of the section.
pub fn arrangements_rank(c: &CardMeta) -> u8 {
    match c.class.method {
        Some("paver bond") => 0,
        _ => 1,
    }
}

/// The Quasiperiodic tab rank: by fold/point order (the four-fold 3-12-12-family
/// entry first, then the five-fold and ten-fold families, then the
/// seven-fold, then the fourteen-fold, then the twelve-fold, then the
/// eight-fold; cards with no declared point order keep registry order
/// at the end). This is the fold-symmetry presentation order, not a
/// family-order presentation.
pub fn quasiperiodic_rank(c: &CardMeta) -> u8 {
    match c.class.point_order {
        Some(4) => 1,
        Some(5) | Some(10) => 2,
        Some(7) => 3,
        Some(14) => 4,
        Some(12) => 5,
        Some(8) => 6,
        _ => 7,
    }
}

/// The Aperiodic tab rank: the einstein monotiles (hat, turtle, spectre)
/// first, then the cap, then the strongly-aperiodic forcing/Wang
/// realizations (Taylor-Socolar, Jeandel-Rao). The bucket is derived
/// from the declared `aperiodic_status` — the epoch-establishing
/// einstein realizations, the pure-point-spectrum cap, then the
/// strongly aperiodic realizations.
pub fn aperiodic_rank(c: &CardMeta) -> u8 {
    match c.class.aperiodic_status.unwrap_or("") {
        s if s.contains("einstein") => 0,
        s if s.contains("pure point") => 1,
        s if s.contains("strongly aperiodic") => 2,
        _ => 3,
    }
}

/// The Nonperiodic tab rank: the square-lattice rep-tile ominoes
/// (polyominoes — armchair, chair, P-pentomino, pregnant-chair) first,
/// then the triangular-lattice rep-tile ominoes (polyiamonds — sphinx,
/// octiamond reptile, the pentiamond factor-2/3 variants), then the
/// remaining non-repeating cards (the substitutions, the pinwheel,
/// the Hirschhorn pentagon, the tangram and the girih decagon patch).
/// The bucket is derived from the declared tile set's omino kind —
/// "iamond"/"triangular" mark the triangular lattice, "omino"/"chair"
/// the square lattice. Cards with no declared tile set (the
/// substitution arms and the Hirschhorn pentagon) keep registry
/// order at the end.
pub fn nonperiodic_rank(c: &CardMeta) -> u8 {
    let t = c.class.tile_set.unwrap_or("");
    if t.contains("iamond") || t.contains("triangular") {
        1
    } else if t.contains("omino") || t.contains("chair") {
        0
    } else {
        2
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

/// The wallpaper groups with no represented tiling (M76). The Wallpaper
/// tab renders one card per group it does have; both UIs show the groups
/// in this table as an explicit gap note in that tab. As new tilings
/// land, groups are removed from this table — the table is empty, so
/// the tab's one-card-per-group catalog is complete: all 17 wallpaper
/// groups are represented.
pub const WALLPAPER_GROUP_GAPS: [WallpaperGroup; 0] = [];

/// The catalog note the Wallpaper tab renders above its cards: while
/// [`WALLPAPER_GROUP_GAPS`] is non-empty it lists the groups with no
/// represented tiling yet; otherwise the table is empty and the note
/// reports the completed catalog. Both UIs derive this text from the
/// gap table.
pub fn wallpaper_gap_note() -> String {
    if WALLPAPER_GROUP_GAPS.is_empty() {
        "complete catalog: all 17 wallpaper groups represented".to_string()
    } else {
        let names: Vec<String> = WALLPAPER_GROUP_GAPS.iter().map(ToString::to_string).collect();
        format!("not yet represented: {}", names.join(" · "))
    }
}

/// The two attribute lines shown under a card thumbnail (shared by
/// the TUI and GUI card renderers):
///
/// * **line 1 — symmetry** — `group · lattice` for the periodic
///   categories; for the nonperiodic world (no declared group) the
///   declared translational character and point order instead
///   ("quasiperiodic · 5-fold", "aperiodic", "limit-periodic") — a
///   nonperiodic card never renders the meaningless "— · —".
/// * **line 2 — summary** — the declared classification facts,
///   composed from [`TilingClass`] in a fixed order (vertex/face
///   configuration, Schläfli, uniformity, pentagon type, isohedral
///   class, method, substrate, contact angle, star, inflation,
///   cut-and-project, prototile count, attribution, dual, aperiodic
///   status, chirality, reflections, cell content, tile set). A
///   segment is **skipped when the card name already says it** (the
///   "Truncated Square (4.8²)" card does not repeat "4.8²") so the
///   line adds information instead of echoing the title. When the
///   schema adds nothing (or the card is unclassified) the line falls
///   back to the description fragment, truncated at the last word
///   boundary within the one-line budget (never a mid-word cut).
///   Chirality shows only when it
///   is the notable fact (homochiral / mixed) — "achiral" is the
///   default expectation and stays off the card.
pub fn attr_lines(c: &CardMeta) -> (String, String) {
    (symmetry_line(c), summary_line(c))
}

/// Line 1: the symmetry footer (see [`attr_lines`]).
fn symmetry_line(c: &CardMeta) -> String {
    match c.group {
        Some(_) => format!("{} · {}", c.group_str(), c.lattice_str()),
        None => {
            let mut segs: Vec<String> = Vec::new();
            if let Some(t) = c.class.translational {
                segs.push(t.to_string());
            }
            if let Some(n) = c.class.point_order {
                segs.push(format!("{n}-fold"));
            }
            segs.join(" · ")
        }
    }
}

/// Line 2: the schema summary (see [`attr_lines`]).
fn summary_line(c: &CardMeta) -> String {
    let class = &c.class;
    if class.is_empty() {
        let frag = description_fragment(c);
        return if frag.is_empty() {
            c.notes.unwrap_or("").to_string()
        } else {
            frag
        };
    }
    let name_lc = c.name.to_lowercase();
    let mut segs: Vec<String> = Vec::new();
    let mut push = |seg: Option<String>| {
        if let Some(s) = seg {
            let s = s.trim().to_string();
            if !s.is_empty() && !name_lc.contains(&s.to_lowercase()) {
                segs.push(s);
            }
        }
    };
    push(class.vertex_config.map(str::to_string));
    push(class.schlafli.map(str::to_string));
    push(class.face_config.map(str::to_string));
    push(class.uniformity.map(str::to_string));
    push(class.pentagon_type.map(|n| format!("type {n}")));
    push(class.isohedral.map(|n| format!("{n}-isohedral")));
    push(class.method.map(str::to_string));
    push(class.base_tiling.map(str::to_string));
    push(class.contact_angle.map(str::to_string));
    push(class.star_type.map(str::to_string));
    push(class.inflation.map(str::to_string));
    push(class.cut_project.map(str::to_string));
    push(class.prototiles.map(|n| {
        if n == 1 {
            "monotile".to_string()
        } else {
            format!("{n} prototiles")
        }
    }));
    push(match (class.source, class.year) {
        (Some(s), Some(y)) => Some(format!("{s} {y}")),
        (Some(s), None) => Some(s.to_string()),
        (None, Some(y)) => Some(y.to_string()),
        (None, None) => None,
    });
    push(class.dual_of.map(|d| {
        if d == "self-dual" {
            "self-dual".to_string()
        } else {
            format!("dual: {d}")
        }
    }));
    push(class.aperiodic_status.map(str::to_string));
    push(class.chirality.filter(|s| *s != "achiral").map(|s| {
        if s == "mixed" {
            "mixed chirality".to_string()
        } else {
            s.to_string()
        }
    }));
    push(class
        .reflections
        .filter(|b| *b)
        .map(|_| "reflected copies".to_string()));
    push(class.cell_content.map(str::to_string));
    push(class.tile_set.map(str::to_string));
    if segs.is_empty() {
        // The schema added nothing (every declared field is already
        // echoed by the name): fall back to the description fragment.
        let frag = description_fragment(c);
        return if frag.is_empty() {
            c.notes.unwrap_or("").to_string()
        } else {
            frag
        };
    }
    segs.join(" · ")
}

/// The description fragment used when the schema adds nothing: the
/// card's description truncated to the one-line card budget. The cut
/// lands at the last word boundary within the budget (with an
/// ellipsis), never mid-word, so the line fits the card's one-line
/// budget in both UIs (the GUI truncates the summary at its label
/// width; the TUI wraps it into two rows). A description with no
/// word boundary inside the budget is hard-cut at the ellipsis.
fn description_fragment(c: &CardMeta) -> String {
    const BUDGET: usize = 40;
    let chars: Vec<char> = c.description.chars().collect();
    if chars.len() <= BUDGET {
        return c.description.to_string();
    }
    // Cut at the last word boundary that leaves room for the ellipsis.
    let mut cut = BUDGET - 1;
    while cut > 0 && chars[cut] != ' ' {
        cut -= 1;
    }
    if cut == 0 {
        // No word boundary inside the budget: hard-cut with ellipsis.
        let mut out: Vec<char> = chars[..BUDGET - 1].to_vec();
        out.push('…');
        return out.into_iter().collect();
    }
    let mut out: String = chars[..cut].iter().collect();
    out.push('…');
    out
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
            | TilingCategory::Isohedral
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
/// * **Arrangements** — the paver-bond catalog first, then the
///   Pythagorean section ([`arrangements_rank`]).
/// * **Quasiperiodic** — by the declared point order: the fourfold
///   entry, then the five/tenfold families, then the sevenfold,
///   fourteenfold, twelvefold and eightfold, then the point-order-less
///   cards ([`quasiperiodic_rank`]).
/// * **Aperiodic** — the einstein monotiles, then the cap, then the
///   strongly aperiodic forcing/Wang realizations
///   ([`aperiodic_rank`]).
/// * **Nonperiodic** — the square-lattice rep-tile ominoes, then the
///   triangular-lattice rep-tile ominoes, then the remaining cards
///   ([`nonperiodic_rank`]).
///
/// Every other tab keeps membership order (primary cards in registry
/// order, then secondaries). All sorts are stable: within one bucket,
/// registry order is preserved.
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
            TilingCategory::Arrangements => {
                cards.sort_by_key(|c| arrangements_rank(c));
            }
            TilingCategory::Quasiperiodic => {
                cards.sort_by_key(|c| quasiperiodic_rank(c));
            }
            TilingCategory::Aperiodic => {
                cards.sort_by_key(|c| aperiodic_rank(c));
            }
            TilingCategory::Nonperiodic => {
                cards.sort_by_key(|c| nonperiodic_rank(c));
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
        // table against the actual declarations. The table is empty, so
        // the shared note reports the completed catalog.
        assert_eq!(
            WALLPAPER_GROUP_GAPS.len(),
            0,
            "gap table is empty"
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
            "complete catalog: all 17 wallpaper groups represented"
        );
    }
}
