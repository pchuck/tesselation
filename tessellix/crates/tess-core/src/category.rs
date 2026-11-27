//! Canonical display categories for tilings.
//!
//! Categories are **declared** by each tiling (never derived from its
//! geometry or symmetry): the label drives how both UIs group their
//! tiling lists, and declaration keeps the taxonomy honest — e.g. a
//! nonperiodic realization of a tile that also admits periodic tilings
//! must not be labeled aperiodic (a property of tile *sets*).
//!
//! Adding a tiling means declaring its category. Adding a category is a
//! one-variant change here plus a slot in [`TilingCategory::ALL`] — UIs
//! render whatever exists, generically, and never learn the names.

use std::fmt;

/// Canonical display categories, ordered from most-ordered to least:
/// the periodic world by decreasing transitivity (vertex-transitive
/// uniform, two-uniform, face-transitive Laves, the one-card-per-group
/// wallpaper catalog, family tabs for pentagon / Islamic / arrangement
/// tiles), then the non-repeating world by decreasing order
/// (quasiperiodic — pure-point diffraction; aperiodic — rules force
/// non-repetition; nonperiodic — one non-repeating realization of a
/// tile that also admits periodic tilings).
///
/// M76: a tiling's *primary* category is its family; it may also
/// belong to extra tabs (e.g. the Wallpaper group catalog) through
/// [`crate::tiling::Tiling::secondary_categories`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TilingCategory {
    /// Vertex-transitive tilings by regular polygons: the 11 uniform
    /// (Archimedean) tilings — a closed set.
    Uniform,
    /// Two-uniform tilings: two vertex orbits (k-uniform, k = 2).
    TwoUniform,
    /// Face-transitive Catalan duals of the uniform tilings — a closed
    /// set of 8 plus documented variants. The three regular tilings
    /// (triangle, square, hexagon) are self-dual and join here as
    /// secondary members (see
    /// [`crate::tiling::Tiling::secondary_categories`]).
    Laves,
    /// The wallpaper-group catalog: one representative card per
    /// wallpaper group of the 17 (members join here as secondaries;
    /// groups with no registered realizer are noted as gaps).
    Wallpaper,
    /// The ten known families of convex pentagonal tilings (Kershner,
    /// James, Reynolds, Casey, Liben–Larson, ...), each with its own
    /// angle/side conditions.
    PentagonTypes,
    /// Kaplan isohedral (IH) templates: parameterized incidence-symbol
    /// realizations of the 93 isohedral types over the Laves topologies
    /// (the Escher arc). Members also appear under their topology's tab
    /// (Laves; the IH27/IH29 Cairo-net variants also under Laves).
    Isohedral,
    /// Girih, star-and-cross and related Islamic geometric patterns.
    Islamic,
    /// Paver / brick-laying arrangements (basketweave, herringbone,
    /// running bond, windmill, pythagorean) — decorative periodic
    /// layouts without regular polygons.
    Arrangements,
    /// Quasiperiodic substitution tilings (pure-point diffraction /
    /// cut-and-project structure): the Penrose family, Ammann–Beenker,
    /// the girih Darb-i Imam substitution.
    Quasiperiodic,
    /// Aperiodic but not canonically quasiperiodic: the einstein family
    /// (hat, cap, turtle, spectre) plus the forcing/Wang realizations
    /// (Taylor–Socolar, Jeandel–Rao).
    Aperiodic,
    /// Nonperiodic by construction whose prototile is not aperiodic
    /// (the Hirschhorn pentagon realization; the limit-periodic rep-tile
    /// realizations of the chair, sphinx and P-pentomino; the pinwheel
    /// — its bare triangle tiles periodically, the substitution forces
    /// nonperiodic order; the limit-periodic substitution tilings
    /// domino, half-hex, period-doubling, thue-morse and period-tripling
    /// — the tiles admit periodic tilings, the substitution forces
    /// nonperiodic order).
    Nonperiodic,
}

impl TilingCategory {
    /// Canonical display order (most-ordered → least-ordered).
    pub const ALL: [TilingCategory; 11] = [
        TilingCategory::Uniform,
        TilingCategory::TwoUniform,
        TilingCategory::Laves,
        TilingCategory::Wallpaper,
        TilingCategory::PentagonTypes,
        TilingCategory::Isohedral,
        TilingCategory::Islamic,
        TilingCategory::Arrangements,
        TilingCategory::Quasiperiodic,
        TilingCategory::Aperiodic,
        TilingCategory::Nonperiodic,
    ];

    /// Human-readable label, used verbatim as the section header in
    /// both UIs and matched by the search filter.
    pub fn label(self) -> &'static str {
        match self {
            TilingCategory::Uniform => "Uniform",
            TilingCategory::TwoUniform => "Two-Uniform",
            TilingCategory::Laves => "Duals & variants",
            TilingCategory::Wallpaper => "Wallpaper",
            TilingCategory::PentagonTypes => "Pentagon Types",
            TilingCategory::Isohedral => "Isohedral",
            TilingCategory::Islamic => "Islamic",
            TilingCategory::Arrangements => "Bonds & dissections",
            TilingCategory::Quasiperiodic => "Quasiperiodic",
            TilingCategory::Aperiodic => "Aperiodic",
            TilingCategory::Nonperiodic => "Nonperiodic",
        }
    }
}

impl fmt::Display for TilingCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_is_complete_with_distinct_labels() {
        assert_eq!(TilingCategory::ALL.len(), 11);
        let mut labels: Vec<&'static str> =
            TilingCategory::ALL.iter().map(|c| c.label()).collect();
        labels.sort_unstable();
        let n = labels.len();
        labels.dedup();
        assert_eq!(labels.len(), n, "category labels must be distinct");
        for c in TilingCategory::ALL {
            assert_eq!(c.to_string(), c.label());
        }
    }
}