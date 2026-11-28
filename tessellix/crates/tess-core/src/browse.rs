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

/// The section key for a card displayed in the tab `tab_category`:
/// the header text a card grid should draw above the first card of
/// each section. The Wallpaper, Laves and Islamic tabs have sections:
///
/// * **Wallpaper** — the group catalog is sorted by crystal system,
///   and the key is the card's *conventional* lattice (see
///   [`WallpaperGroup::conventional_lattice`]): the textbook mapping
///   p1/p2 → oblique, pm/pg/pmm/pgg/pmg → rectangular, cm/cmm →
///   centered, p4* → square, p3*/p6* → hexagonal. This is deliberately
///   a display convention derived from the declared group, not from the
///   representative's measured lattice: a p1 tiling realized on a
///   centered-rectangular primitive cell (the stepped hexominoes)
///   still displays under oblique, together with p2, as the textbooks
///   lay out the 17 groups. The measured per-tiling lattice kind stays
///   on the card (it is the data-honest attribute line). Cards with
///   no declared group return `""`.
/// * **Laves** — the tab is sorted by the declared `laves_rank`
///   (self-dual regulars, then the Laves proper, then the 2-uniform
///   duals), and the key is derived from the same declared facts,
///   never the card id (see [`laves_section_key`]).
/// * **Islamic** — the key is derived from the declared construction
///   facts, never the card id (see [`islamic_section_key`]): the
///   five `docs/families/islamic.md` sections, §1 Classic compositions
///   (the residual bucket — the method-less hexagram sorts here), §2
///   Girih tiles, §3 Hankin constructions, §4 Najm templates, and the
///   Andritz σ-frame walks. Only the 30 primary-Islamic cards are on
///   the Islamic tab (classic 6 / girih 6 / hankin 15 / najm 3, in
///   register order); the 6 off-tab family members (the two
///   Darb-i-Imam realizations, primary Quasiperiodic/Nonperiodic, and
///   the four Andritz walks, primary Quasiperiodic) still derive their
///   §-key through [`islamic_section_key`] directly — the GUI detail
///   view's F3 block consumes that entry point, so the label it prints
///   can never drift from the section the card would sort under.
///
/// Cards in every other tab return `""` (no header).
pub fn section_key(tab_category: TilingCategory, card: &CardMeta) -> &'static str {
    match tab_category {
        TilingCategory::Wallpaper => match card.group.map(WallpaperGroup::conventional_lattice) {
            Some(LatticeKind::Oblique) => "oblique",
            Some(LatticeKind::Rectangular) => "rectangular",
            Some(LatticeKind::CenteredRectangular) => "centered",
            Some(LatticeKind::Square) => "square",
            Some(LatticeKind::Hexagonal) => "hexagonal",
            _ => "",
        },
        TilingCategory::Laves => laves_section_key(card),
        TilingCategory::Islamic => islamic_section_key(card),
        _ => "",
    }
}

/// The Laves-tab section key for a card: the header text a card grid
/// should draw above the first card of each section, derived from the
/// declared classification facts (`laves_rank`, `dual_of`), never the
/// card id. The tab keeps its rank banding, with one split carved out
/// of the rank-1 band:
///
/// * **isohedral variants** — rank-1 cards that declare no dual
///   partner (`dual_of: None`): the five isohedral-family secondaries
///   (ih1/ih6/ih7/ih27/ih29). They must take precedence over the rank
///   banding because they carry rank 1; a rank-1 card that declares a
///   dual partner is a Catalan dual, not an isohedral variant.
/// * **self-dual regulars & Catalan duals** — rank 0 (the self-dual
///   regular trio) and rank 1 (the Laves proper / Catalan primaries),
///   merged into one band.
/// * **2-uniform duals** — rank 2.
///
/// Cards with no declared rank return `""` (no header), matching the
/// Wallpaper arm's honesty rule.
///
/// Note the discriminator is `dual_of: None`, not the `isohedral`
/// class field: `isohedral: Some(1)` is the *k-isohedral class*
/// (one transitivity class of tiles) of the face-transitive cairo and
/// floret pentagons, a uniformity fact — it is not an isohedral-family
/// membership claim, and the isohedral-family secondaries declare no
/// `isohedral` field at all.
fn laves_section_key(card: &CardMeta) -> &'static str {
    if card.class.laves_rank == Some(1) && card.class.dual_of.is_none() {
        return "isohedral variants";
    }
    match card.class.laves_rank {
        Some(0) | Some(1) => "self-dual regulars & Catalan duals",
        Some(2) => "2-uniform duals",
        _ => "",
    }
}

/// The Islamic family's library section key for a card: the header text
/// a card grid should draw above the first card of each section, derived
/// from the declared classification facts (the construction `method`
/// plus the `[p,q]` substrate `base_tiling`), never the card id. The
/// five sections of `docs/families/islamic.md`:
///
/// * **§1 Classic compositions** — the hand-drawn star-and-cross, khatam,
///   medallion and Wichmann-panel realizations. This section is also the
///   *residual bucket*: a card with no declared method (the hexagram) —
///   and any future card whose method is not a named girih/hankin/najm/
///   andritz construction — sorts here rather than failing.
/// * **§2 Girih tiles** — the girih strapwork and prototile tilings plus
///   the two Darb-i-Imam realizations (the quasiperiodic girih and the
///   finite decagon patch, both off-tab but family members).
/// * **§3 Hankin constructions** — the polygons-in-contact, greedy-
///   inference, two-point, rosette-transform and motif-library
///   inferences.
/// * **§4 Najm templates** — the `[p,q]`-substrate najm templates. The
///   bracketed substrate is the discriminator: it also pulls
///   `najm-63-motifs` out of the Hankin motif band, whose method string
///   it shares with the two Hankin motif cards.
/// * **Andritz σ-frame walks** — the four quasiperiodic σ-frame walks
///   (off-tab, family members only).
///
/// Total over every `Islamic`-family card. The `[p,q]`-substrate guard
/// runs before the method match so the substrate fact outranks the
/// borrowed method string.
pub fn islamic_section_key(card: &CardMeta) -> &'static str {
    const CLASSIC: &str = "§1 — Classic compositions";
    const GIRIH: &str = "§2 — Girih tiles";
    const HANKIN: &str = "§3 — Hankin constructions";
    const NAJM: &str = "§4 — Najm templates";
    const ANDRITZ: &str = "Andritz σ-frame walks";

    let c = &card.class;
    // The [p,q] bracket substrate is the najm-template signature; it
    // takes precedence over the method so the motif variant that borrows
    // the Hankin motif-library method still sorts under §4.
    if c.base_tiling.map_or(false, |b| b.starts_with('[')) {
        return NAJM;
    }
    match c.method {
        // §2 Girih — the strapwork and prototile tilings plus the two
        // Darb-i-Imam realizations.
        Some("girih strapwork")
        | Some("girih prototile tiling")
        | Some("Darb-i Imam")
        | Some("Darb-i Imam substitution (finite patch)") => GIRIH,
        // §3 Hankin — the PIC / greedy-inference / two-point /
        // rosette-transform / motif-library inferences.
        Some("Hankin polygons-in-contact")
        | Some("Hankin greedy inference")
        | Some("Kaplan two-point")
        | Some("rosette transform + Hankin inference")
        | Some("Kaplan §3.5 motif library") => HANKIN,
        // §4 Najm — the named najm templates (the [p,q]-substrate motif
        // variant is caught by the guard above).
        Some("Najm template") => NAJM,
        // Andritz — the quasiperiodic σ-frame walks.
        Some("σ-frame walk") => ANDRITZ,
        // §1 Classic — the star-and-cross, khatam, medallion and
        // Wichmann compositions, plus the method-less hexagram (and any
        // unrecognized method).
        _ => CLASSIC,
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

    /// A fixture mirroring the real Laves tab's shape (not the same
    /// tilings): 3 rank-0 self-dual regulars, 8 rank-1 Catalan
    /// primaries (dual partner declared), 5 rank-1 isohedral-family
    /// secondaries (no dual partner declared) and 20 rank-2 2-uniform
    /// duals — 36 cards in within-tab order, so the stable rank sort
    /// yields rank 0, then rank-1 primaries, then the isohedral
    /// variants, then rank 2. Synthetic ids, leaked once per fixture
    /// (never shown to the user).
    fn laves_card(r: Registry, kind: &'static str, i: usize) -> (Registry, &'static str, TilingClass) {
        let id: &'static str = Box::leak(format!("{kind}-{i}").into_boxed_str());
        let class = match kind {
            "self" => TilingClass {
                laves_rank: Some(0),
                dual_of: Some("self-dual"),
                ..TilingClass::EMPTY
            },
            "catalan" => TilingClass {
                laves_rank: Some(1),
                dual_of: Some("dual"),
                ..TilingClass::EMPTY
            },
            "ih" => TilingClass {
                laves_rank: Some(1),
                dual_of: None,
                method: Some("IH incidence symbol"),
                ..TilingClass::EMPTY
            },
            "u2" => TilingClass {
                laves_rank: Some(2),
                dual_of: Some("dual"),
                ..TilingClass::EMPTY
            },
            _ => unreachable!("unknown fixture kind"),
        };
        let r = r.register(Fixed {
            id,
            cat: TilingCategory::Laves,
            secondary: &[],
            group: WallpaperGroup::P6m,
            lattice: LatticeKind::Hexagonal,
            params: 0,
        });
        (r, id, class)
    }

    fn laves_fixture() -> (Registry, Vec<(&'static str, TilingClass)>) {
        let mut r = Registry::new();
        let mut facts: Vec<(&'static str, TilingClass)> = Vec::new();
        for i in 0..3 {
            let (r_out, id, class) = laves_card(r, "self", i);
            r = r_out;
            facts.push((id, class));
        }
        for i in 0..8 {
            let (r_out, id, class) = laves_card(r, "catalan", i);
            r = r_out;
            facts.push((id, class));
        }
        for i in 0..5 {
            let (r_out, id, class) = laves_card(r, "ih", i);
            r = r_out;
            facts.push((id, class));
        }
        for i in 0..20 {
            let (r_out, id, class) = laves_card(r, "u2", i);
            r = r_out;
            facts.push((id, class));
        }
        (r, facts)
    }

    /// The sorted Laves tab for the fixture, with the declared facts
    /// injected through the classification lookup (the same seam both
    /// UIs use).
    fn laves_tabs(
        r: &Registry,
        facts: &[(&'static str, TilingClass)],
    ) -> Vec<Tab> {
        browse_tabs_with(r, |id| {
            facts
                .iter()
                .find(|(i, _)| *i == id)
                .map(|(_, c)| *c)
                .unwrap_or(TilingClass::EMPTY)
        })
    }

    #[test]
    fn section_keys_follow_laves_declared_facts() {
        // Mirrors `section_keys_follow_crystal_systems`: walk the
        // sorted Laves tab and assert each declared-fact class maps to
        // exactly its header, with exact counts.
        let (r, facts) = laves_fixture();
        let tabs = laves_tabs(&r, &facts);
        let lv = tabs
            .iter()
            .find(|t| t.category == TilingCategory::Laves)
            .unwrap();
        let mut isohedral = 0usize;
        let mut band = 0usize;
        let mut duals = 0usize;
        for c in &lv.cards {
            match section_key(TilingCategory::Laves, c) {
                "isohedral variants" => isohedral += 1,
                "self-dual regulars & Catalan duals" => band += 1,
                "2-uniform duals" => duals += 1,
                "" => panic!("{}: no section header despite declared facts", c.id),
                k => panic!("{}: unexpected section header {k}", c.id),
            }
        }
        assert_eq!(isohedral, 5, "the five isohedral-family secondaries");
        assert_eq!(band, 11, "3 self-dual regulars + 8 Catalan primaries");
        assert_eq!(duals, 20, "the 2-uniform duals");
        assert_eq!(lv.cards.len(), 36);
    }

    #[test]
    fn laves_section_keys_appear_in_first_appearance_order() {
        // Mirrors `section_keys_appear_in_first_appearance_order`:
        // the distinct keys come out in first-appearance order, and the
        // tab is exactly three contiguous runs.
        let (r, facts) = laves_fixture();
        let tabs = laves_tabs(&r, &facts);
        let lv = tabs
            .iter()
            .find(|t| t.category == TilingCategory::Laves)
            .unwrap();
        let mut seen: Vec<&'static str> = Vec::new();
        for c in &lv.cards {
            let k = section_key(TilingCategory::Laves, c);
            if !k.is_empty() && !seen.contains(&k) {
                seen.push(k);
            }
        }
        assert_eq!(
            seen,
            vec![
                "self-dual regulars & Catalan duals",
                "isohedral variants",
                "2-uniform duals"
            ],
            "sections must appear in first-appearance order"
        );
        // Exactly three contiguous runs, in that order, with the exact
        // widths of the real tab.
        let mut runs: Vec<(&'static str, usize)> = Vec::new();
        let mut i = 0;
        while i < lv.cards.len() {
            let k = section_key(TilingCategory::Laves, &lv.cards[i]);
            let mut j = i + 1;
            while j < lv.cards.len() && section_key(TilingCategory::Laves, &lv.cards[j]) == k {
                j += 1;
            }
            runs.push((k, j - i));
            i = j;
        }
        assert_eq!(
            runs,
            vec![
                ("self-dual regulars & Catalan duals", 11),
                ("isohedral variants", 5),
                ("2-uniform duals", 20),
            ],
            "exactly three contiguous runs, in first-appearance order"
        );
    }

    /// The 36 islamic-family cards in register order, each carrying the
    /// primary tab category (Islamic for the 30 on-tab primaries;
    /// Quasiperiodic / Nonperiodic for the 6 off-tab family members) and
    /// the declared facts `islamic_section_key` reads: the construction
    /// `method` plus the `[p,q]`-substrate `base_tiling`. `families` is
    /// carried so the full-family walk can select exactly the
    /// `Islamic`-tagged cards. The facts are verbatim from
    /// `tess-tilings`'s `islamic::classification` arms.
    fn islamic_class(
        families: &'static [&'static str],
        method: Option<&'static str>,
        base_tiling: Option<&'static str>,
    ) -> TilingClass {
        TilingClass { families, method, base_tiling, ..TilingClass::EMPTY }
    }

    fn islamic_specs() -> Vec<(&'static str, TilingCategory, TilingClass)> {
        use TilingCategory as C;
        let isl: &'static [&'static str] = &["Islamic"];
        let isl_q: &'static [&'static str] = &["Islamic", "Quasiperiodic"];
        let isl_n: &'static [&'static str] = &["Islamic", "Nonperiodic"];
        let mut s = Vec::new();
        // §1 Classic compositions (6, on-tab) — register block 1.
        s.push(("star-cross", C::Islamic, islamic_class(isl, Some("star-and-cross"), None)));
        s.push(("star-cross-8-2", C::Islamic, islamic_class(isl, Some("star-and-cross"), None)));
        s.push(("hexagram", C::Islamic, islamic_class(isl, None, None)));
        s.push(("khatam-girih", C::Islamic, islamic_class(isl, Some("girih decomposition"), None)));
        s.push(("dodecagon-hexagram-honeycomb", C::Islamic, islamic_class(isl, Some("medallion composition on a periodic net (24d)"), Some("6.6.6"))));
        s.push(("girih-w85", C::Islamic, islamic_class(isl, Some("Wichmann panel data"), None)));
        // §2 Girih tiles (8 total: 6 on-tab + 2 off-tab).
        s.push(("girih-shesh-band", C::Islamic, islamic_class(isl, Some("girih strapwork"), None)));
        s.push(("girih-star-cross", C::Islamic, islamic_class(isl, Some("girih strapwork"), None)));
        s.push(("girih-bowtie", C::Islamic, islamic_class(isl, Some("girih prototile tiling"), None)));
        s.push(("girih-decagon", C::Islamic, islamic_class(isl, Some("girih prototile tiling"), None)));
        s.push(("girih-pentagon", C::Islamic, islamic_class(isl, Some("girih prototile tiling"), None)));
        s.push(("girih-rhombus", C::Islamic, islamic_class(isl, Some("girih prototile tiling"), None)));
        s.push(("girih-quasiperiodic", C::Quasiperiodic, islamic_class(isl_q, Some("Darb-i Imam"), None)));
        s.push(("girih-decagon-patch", C::Nonperiodic, islamic_class(isl_n, Some("Darb-i Imam substitution (finite patch)"), None)));
        // §3 Hankin constructions (15, on-tab) — register block 3.
        s.push(("pic-4-8-8", C::Islamic, islamic_class(isl, Some("Hankin polygons-in-contact"), Some("4.8.8"))));
        s.push(("pic-3-6-3-6", C::Islamic, islamic_class(isl, Some("Hankin polygons-in-contact"), Some("3.6.3.6"))));
        s.push(("pic-3-12-12", C::Islamic, islamic_class(isl, Some("Hankin polygons-in-contact"), Some("3.12.12"))));
        s.push(("hankin-4-8-8-rosette", C::Islamic, islamic_class(isl, Some("Hankin greedy inference"), Some("4.8.8"))));
        s.push(("hankin-4-6-12", C::Islamic, islamic_class(isl, Some("Hankin greedy inference"), Some("4.6.12"))));
        s.push(("hankin-3-12-12", C::Islamic, islamic_class(isl, Some("Hankin greedy inference"), Some("3.12.12"))));
        s.push(("hankin-3-6-3-6", C::Islamic, islamic_class(isl, Some("Hankin greedy inference"), Some("3.6.3.6"))));
        s.push(("hankin-3-4-6-4", C::Islamic, islamic_class(isl, Some("Hankin greedy inference"), Some("3.4.6.4"))));
        s.push(("hankin-3-3-3-4-4", C::Islamic, islamic_class(isl, Some("Hankin greedy inference"), Some("3.3.3.4.4"))));
        s.push(("hankin-3-3-4-3-4", C::Islamic, islamic_class(isl, Some("Hankin greedy inference"), Some("3.3.4.3.4"))));
        s.push(("hankin-hex-two-point", C::Islamic, islamic_class(isl, Some("Kaplan two-point"), Some("6.6.6"))));
        s.push(("hankin-square-two-point", C::Islamic, islamic_class(isl, Some("Kaplan two-point"), Some("4.4.4.4"))));
        s.push(("hankin-4-8-8-rosette-transform", C::Islamic, islamic_class(isl, Some("rosette transform + Hankin inference"), Some("4.8.8"))));
        s.push(("hankin-4-8-8-motifs", C::Islamic, islamic_class(isl, Some("Kaplan §3.5 motif library"), Some("4.8.8"))));
        s.push(("hankin-hex-motifs", C::Islamic, islamic_class(isl, Some("Kaplan §3.5 motif library"), Some("6.6.6"))));
        // §4 Najm templates (3, on-tab) — register block 4.
        s.push(("najm-63-2e-0-3e-ac", C::Islamic, islamic_class(isl, Some("Najm template"), Some("[6,3]; 2e, 0, 3e (AC)"))));
        s.push(("najm-44-3e-0-0-a", C::Islamic, islamic_class(isl, Some("Najm template"), Some("[4,4]; 3e, 0, 0 (A)"))));
        s.push(("najm-63-motifs", C::Islamic, islamic_class(isl, Some("Kaplan §3.5 motif library"), Some("[6,3]; 2e, 0, 3e (AC)"))));
        // Andritz σ-frame walks (4, all off-tab) — register block 5.
        s.push(("andritz-bowtie-hexagon", C::Quasiperiodic, islamic_class(isl_q, Some("σ-frame walk"), None)));
        s.push(("andritz-bowtie-hexagon-10gon-1", C::Quasiperiodic, islamic_class(isl_q, Some("σ-frame walk"), None)));
        s.push(("andritz-bowtie-hexagon-10gon-2", C::Quasiperiodic, islamic_class(isl_q, Some("σ-frame walk"), None)));
        s.push(("andritz-bowtie-hexagon-10gon-3", C::Quasiperiodic, islamic_class(isl_q, Some("σ-frame walk"), None)));
        s
    }

    /// A registry holding all 36 islamic cards (in register order), plus
    /// the `(id, category, class)` facts keyed by id.
    fn islamic_fixture() -> (Registry, Vec<(&'static str, TilingCategory, TilingClass)>) {
        let mut r = Registry::new();
        let mut facts: Vec<(&'static str, TilingCategory, TilingClass)> = Vec::new();
        for (id, cat, class) in islamic_specs() {
            r = r.register(Fixed {
                id,
                cat,
                secondary: &[],
                group: WallpaperGroup::P6m,
                lattice: LatticeKind::Hexagonal,
                params: 0,
            });
            facts.push((id, cat, class));
        }
        (r, facts)
    }

    /// The browse tabs for the islamic fixture, with the declared facts
    /// injected through the classification lookup (the same seam both
    /// UIs use).
    fn islamic_tabs(
        r: &Registry,
        facts: &[(&'static str, TilingCategory, TilingClass)],
    ) -> Vec<Tab> {
        browse_tabs_with(r, |id| {
            facts
                .iter()
                .find(|(i, _, _)| *i == id)
                .map(|(_, _, c)| *c)
                .unwrap_or(TilingClass::EMPTY)
        })
    }

    /// Every Islamic-family card across all tabs, in browse order.
    fn all_islamic_cards(tabs: &[Tab]) -> Vec<&CardMeta> {
        let mut cards = Vec::new();
        for t in tabs {
            for c in &t.cards {
                if c.class.families.contains(&"Islamic") {
                    cards.push(c);
                }
            }
        }
        cards
    }

    #[test]
    fn section_keys_follow_islamic_declared_facts() {
        // The derivation is id-free and total over the full family:
        // walk all 36 `Islamic`-family cards and pin the edge cases that
        // make the mapping non-trivial (the method-less fallback and the
        // `[p,q]`-substrate guard that outranks a borrowed method).
        let (r, facts) = islamic_fixture();
        let tabs = islamic_tabs(&r, &facts);
        let cards = all_islamic_cards(&tabs);
        assert_eq!(cards.len(), 36, "the islamic family is 36 cards");
        let key_of = |id: &str| -> Option<&'static str> {
            cards.iter().find(|c| c.id == id).map(|c| islamic_section_key(*c))
        };
        // Classic residual: the method-less hexagram sorts to §1.
        assert_eq!(
            key_of("hexagram"),
            Some("§1 — Classic compositions"),
            "the method-less hexagram falls to the classic residual bucket"
        );
        // The [p,q]-substrate guard outranks the borrowed Hankin motif
        // method, pulling najm-63-motifs under §4.
        assert_eq!(
            key_of("najm-63-motifs"),
            Some("§4 — Najm templates"),
            "the [p,q]-substrate motif variant sorts under §4 despite the Hankin motif method"
        );
        // Off-tab family members still derive their section.
        assert_eq!(
            key_of("girih-quasiperiodic"),
            Some("§2 — Girih tiles"),
            "the off-tab quasiperiodic Darb-i-Imam card"
        );
        assert_eq!(
            key_of("girih-decagon-patch"),
            Some("§2 — Girih tiles"),
            "the off-tab finite-patch Darb-i-Imam card"
        );
        assert_eq!(
            key_of("andritz-bowtie-hexagon"),
            Some("Andritz σ-frame walks"),
            "the off-tab σ-frame walk"
        );
    }

    #[test]
    fn islamic_family_partition_counts() {
        // Full family (36): classic 6 / girih 8 / hankin 15 / najm 3 /
        // andritz 4. On the Islamic tab (30): the 6 off-tab family
        // members drop out → classic 6 / girih 6 / hankin 15 / najm 3.
        let (r, facts) = islamic_fixture();
        let tabs = islamic_tabs(&r, &facts);

        // Full-family partition over every Islamic-family card.
        let cards = all_islamic_cards(&tabs);
        let (mut classic, mut girih, mut hankin, mut najm, mut andritz) =
            (0usize, 0usize, 0usize, 0usize, 0usize);
        for c in &cards {
            match islamic_section_key(*c) {
                "§1 — Classic compositions" => classic += 1,
                "§2 — Girih tiles" => girih += 1,
                "§3 — Hankin constructions" => hankin += 1,
                "§4 — Najm templates" => najm += 1,
                "Andritz σ-frame walks" => andritz += 1,
                k => panic!("{}: unexpected section key {k}", c.id),
            }
        }
        assert_eq!(classic, 6, "classic compositions");
        assert_eq!(girih, 8, "girih tiles (6 on-tab + 2 off-tab Darb-i-Imam)");
        assert_eq!(hankin, 15, "hankin constructions");
        assert_eq!(najm, 3, "najm templates");
        assert_eq!(andritz, 4, "andritz σ-frame walks (off-tab only)");

        // On-tab partition over the Islamic tab alone.
        let isl = tabs.iter().find(|t| t.category == TilingCategory::Islamic).unwrap();
        assert_eq!(isl.cards.len(), 30, "the Islamic tab holds the 30 primary-Islamic cards");
        let (mut o_classic, mut o_girih, mut o_hankin, mut o_najm) =
            (0usize, 0usize, 0usize, 0usize);
        for c in &isl.cards {
            match section_key(TilingCategory::Islamic, c) {
                "§1 — Classic compositions" => o_classic += 1,
                "§2 — Girih tiles" => o_girih += 1,
                "§3 — Hankin constructions" => o_hankin += 1,
                "§4 — Najm templates" => o_najm += 1,
                k => panic!("{}: unexpected on-tab section key {k}", c.id),
            }
        }
        assert_eq!(o_classic, 6, "on-tab classic");
        assert_eq!(o_girih, 6, "on-tab girih (the 2 off-tab Darb-i-Imam drop out)");
        assert_eq!(o_hankin, 15, "on-tab hankin");
        assert_eq!(o_najm, 3, "on-tab najm");
    }

    #[test]
    fn islamic_section_keys_appear_in_first_appearance_order() {
        // Mirrors `laves_section_keys_appear_in_first_appearance_order`:
        // the distinct keys come out in first-appearance order, and the
        // Islamic tab is exactly four contiguous runs (the Andritz
        // section has no on-tab member, so it never appears).
        let (r, facts) = islamic_fixture();
        let tabs = islamic_tabs(&r, &facts);
        let isl = tabs.iter().find(|t| t.category == TilingCategory::Islamic).unwrap();
        let mut seen: Vec<&'static str> = Vec::new();
        for c in &isl.cards {
            let k = section_key(TilingCategory::Islamic, c);
            if !k.is_empty() && !seen.contains(&k) {
                seen.push(k);
            }
        }
        assert_eq!(
            seen,
            vec![
                "§1 — Classic compositions",
                "§2 — Girih tiles",
                "§3 — Hankin constructions",
                "§4 — Najm templates",
            ],
            "sections must appear in first-appearance order"
        );
        let mut runs: Vec<(&'static str, usize)> = Vec::new();
        let mut i = 0;
        while i < isl.cards.len() {
            let k = section_key(TilingCategory::Islamic, &isl.cards[i]);
            let mut j = i + 1;
            while j < isl.cards.len() && section_key(TilingCategory::Islamic, &isl.cards[j]) == k {
                j += 1;
            }
            runs.push((k, j - i));
            i = j;
        }
        assert_eq!(
            runs,
            vec![
                ("§1 — Classic compositions", 6),
                ("§2 — Girih tiles", 6),
                ("§3 — Hankin constructions", 15),
                ("§4 — Najm templates", 3),
            ],
            "exactly four contiguous runs in first-appearance order"
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
