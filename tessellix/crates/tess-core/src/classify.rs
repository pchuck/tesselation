//! The classification schema: optional, per-family display fields.
//!
//! Every tiling carries a [`TilingClass`] — a record of *declarable*
//! classification facts, each `Option`-typed so a field can be absent.
//! The UIs render only the fields that are present: a Penrose card
//! shows no wallpaper group (it has none to claim), a pentagon-type
//! card shows no vertex configuration, and nothing anywhere renders
//! as a bare "N/A".
//!
//! The schema is deliberately *not* derived from geometry: like
//! [`crate::category::TilingCategory`] it is declared, so the taxonomy
//! stays honest (a nonperiodic realization of a tile that also admits
//! periodic tilings must not claim quasiperiodicity). The data lives
//! in `tess-tilings`'s `classify` table (per-tiling knowledge belongs
//! beside the tilings); `tess-core` only defines the shape and the
//! rendering rules.
//!
//! Fields are grouped by the question they answer:
//!
//! * **Identity** — `families` (a tiling can carry several tags: the
//!   hexagonal tiling is Uniform, Wallpaper p6m and Laves-regular at
//!   once), `source` / `year`.
//! * **Tile description** — `tile_set`, `prototiles`, `reflections`,
//!   `chirality` (achiral / homochiral / mixed — the hat needs
//!   reflected copies, the Spectre is strictly chiral).
//! * **Combinatorics** — `vertex_config`, `face_config` (Laves duals),
//!   `k` (uniformity), `schlafli` (regular / truncated / star-derived
//!   tilings only), `dual_of`, `uniformity` (regular / semiregular /
//!   demiregular).
//! * **Symmetry** — wallpaper group and lattice live on
//!   [`crate::browse::CardMeta`] already (declared by the generator);
//!   here: `point_order` (the n of an n-fold quasiperiodic tiling),
//!   `translational` (periodic / quasiperiodic / aperiodic /
//!   non-periodic — the field that replaces the tab split as a *fact*,
//!   while the tabs stay family-declared), `cell_content`.
//! * **Construction** — `method` (Hankin polygons-in-contact, Kaplan
//!   two-point, Najm template, cut-and-project, substitution,
//!   matching rules), `base_tiling`, `contact_angle` (the Hankin θ),
//!   `star_type` (the {n/k} of a rosette — the star *polygon*, not the
//!   tiling's own Schläfli symbol), `inflation`, `matching_rules`,
//!   `cut_project` (dimension + window shape for cut-and-project
//!   tilings).
//! * **Properties** — `aperiodic_status`, `frequencies` (tile
//!   frequencies, a quasicrystal's signature), `colorability`,
//!   `ref_url`.
//!
//! Three fields double as declared **sort keys** for tab ordering
//! (see [`crate::browse::browse_tabs_with`]): `pentagon_type` orders
//! the Pentagon tab sequentially, `vertex_config` orders the
//! Two-Uniform tab, and `laves_rank` groups the Laves tab (self-dual
//! regulars, then the Laves proper, then the 2-uniform duals).

/// Classification facts a tiling can declare, all optional.
///
/// [`TilingClass::EMPTY`] is the zero value: an unknown/undeclared
/// tiling shows no classification fields at all (and never a
/// placeholder).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TilingClass {
    // -- Identity -----------------------------------------------------
    /// Family tags; a tiling may carry several ("Uniform", "Wallpaper
    /// p6m", "Laves-regular", "Pentagon type", "Islamic", ...).
    pub families: &'static [&'static str],
    /// Attribution ("Reinhardt", "Kershner", "Smith et al.", ...).
    pub source: Option<&'static str>,
    /// Publication year as part of the identity (pentagon types,
    /// aperiodic sets).
    pub year: Option<&'static str>,
    // -- Tile description ----------------------------------------------
    /// The prototile(s), e.g. "square + regular octagon", "hat".
    pub tile_set: Option<&'static str>,
    /// Number of prototiles (1 for a monotile, 2 for Penrose, 11 for
    /// Jeandel–Rao).
    pub prototiles: Option<u8>,
    /// Whether reflected copies of the prototile(s) are used (the hat
    /// needs them; the Spectre does not).
    pub reflections: Option<bool>,
    /// "achiral" | "homochiral" | "mixed".
    pub chirality: Option<&'static str>,
    // -- Combinatorics ---------------------------------------------------
    /// Vertex configuration(s), e.g. "3.4.6.4" or "(3⁶; 3².6²)".
    pub vertex_config: Option<&'static str>,
    /// Face configuration of a Laves dual, e.g. "V3.4.6.4".
    pub face_config: Option<&'static str>,
    /// Uniformity k: number of vertex orbits (1 = uniform, 2 = two-uniform).
    pub k: Option<u8>,
    /// Schläfli symbol — meaningful only for regular / truncated /
    /// star-derived tilings ("{4,4}", "t{4,4}", "{8/3}"); absent
    /// elsewhere rather than blank.
    pub schlafli: Option<&'static str>,
    /// The dual partner (display name of the dual tiling; "self-dual"
    /// for the regular trio).
    pub dual_of: Option<&'static str>,
    /// "regular" | "semiregular" | "demiregular" (classical
    /// uniform-tiling category).
    pub uniformity: Option<&'static str>,
    /// k-isohedral class (1 = isohedral: one transitivity class of
    /// tiles under the tiling's symmetries; 2/3 = two/three classes).
    /// Declared for the pentagon types and the face-transitive Laves
    /// cards; absent where not established.
    pub isohedral: Option<u8>,
    // -- Symmetry --------------------------------------------------------
    /// Rotational order of the point symmetry (5/8/10/12 for
    /// quasiperiodic tilings).
    pub point_order: Option<u8>,
    /// "periodic" | "quasiperiodic" | "aperiodic" | "non-periodic".
    pub translational: Option<&'static str>,
    /// Tiles per fundamental cell ("4 tiles per primitive cell").
    pub cell_content: Option<&'static str>,
    // -- Construction ------------------------------------------------------
    /// "Hankin polygons-in-contact", "Kaplan two-point", "Najm
    /// template", "cut-and-project", "substitution", "matching rules",
    /// "data table", ...
    pub method: Option<&'static str>,
    /// The underlying tiling a pattern is built on ("3.12.12", "4.8.8").
    pub base_tiling: Option<&'static str>,
    /// The Hankin contact / star angle ("θK = 45°", "δ = 0.5").
    pub contact_angle: Option<&'static str>,
    /// Star polygon {n/k} of a rosette (not the tiling's Schläfli).
    pub star_type: Option<&'static str>,
    /// Substitution / inflation factor ("φ", "1+√2", "2+√3", "√φ").
    pub inflation: Option<&'static str>,
    /// Matching rules ("yes — edge decorations", ...).
    pub matching_rules: Option<&'static str>,
    /// Cut-and-project data ("R⁴ → R², octagonal window").
    pub cut_project: Option<&'static str>,
    // -- Properties ---------------------------------------------------------
    /// "periodic" | "non-periodic" | "weakly aperiodic" | "strongly
    /// aperiodic" (+ notes, e.g. the pinwheel's dense orientations).
    pub aperiodic_status: Option<&'static str>,
    /// Tile frequencies / density ratios.
    pub frequencies: Option<&'static str>,
    /// Colorability note (what the thumbnails encode).
    pub colorability: Option<&'static str>,
    /// Reference URL (paper / encyclopedia entry).
    pub ref_url: Option<&'static str>,
    // -- Declared sort keys (see browse_tabs_with) ---------------------------
    /// Pentagon-type number 1–15 (orders the Pentagon tab).
    pub pentagon_type: Option<u8>,
    /// Laves-tab grouping rank: 0 = self-dual regular, 1 = Laves
    /// proper, 2 = 2-uniform dual.
    pub laves_rank: Option<u8>,
}

impl TilingClass {
    /// The undeclared class: no families, no fields (UIs render
    /// nothing rather than "N/A").
    pub const EMPTY: TilingClass = TilingClass {
        families: &[],
        source: None,
        year: None,
        tile_set: None,
        prototiles: None,
        reflections: None,
        chirality: None,
        vertex_config: None,
        face_config: None,
        k: None,
        schlafli: None,
        dual_of: None,
        uniformity: None,
        isohedral: None,
        laves_rank: None,
        pentagon_type: None,
        point_order: None,
        translational: None,
        cell_content: None,
        method: None,
        base_tiling: None,
        contact_angle: None,
        star_type: None,
        inflation: None,
        matching_rules: None,
        cut_project: None,
        aperiodic_status: None,
        frequencies: None,
        colorability: None,
        ref_url: None,
    };

    /// True when nothing at all is declared (no families, no fields):
    /// the UIs can fall back to the raw description in that case.
    pub fn is_empty(&self) -> bool {
        *self == TilingClass::EMPTY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_is_default_and_empty() {
        assert_eq!(TilingClass::EMPTY, TilingClass::default());
        assert!(TilingClass::EMPTY.is_empty());
        assert!(!TilingClass {
            source: Some("Reinhardt"),
            ..TilingClass::EMPTY
        }
        .is_empty());
    }
}