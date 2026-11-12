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
/// uniform, face-transitive Laves, other wallpaper), then the
/// non-repeating world by decreasing order (quasiperiodic — pure-point
/// diffraction; aperiodic — rules force non-repetition; nonperiodic —
/// one non-repeating realization of a tile that also admits periodic
/// tilings).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TilingCategory {
    /// Vertex-transitive tilings by regular polygons: the 11 uniform
    /// (Archimedean) tilings — a closed set.
    Uniform,
    /// Face-transitive Catalan duals of the uniform tilings — a closed
    /// set of 8 plus documented variants.
    Laves,
    /// All other periodic (wallpaper) tilings: decorative families,
    /// star-and-cross, girih bands, parameterized periodic families.
    Wallpaper,
    /// Quasiperiodic substitution tilings (pure-point diffraction /
    /// cut-and-project structure): the Penrose family, Ammann–Beenker,
    /// the girih Darb-i Imam substitution.
    Quasiperiodic,
    /// Aperiodic but not canonically quasiperiodic: the einstein family
    /// (hat, turtle, spectre), limit-periodic rep-tiles (chair, sphinx),
    /// the pinwheel (dense orientations).
    Aperiodic,
    /// Nonperiodic by construction whose prototile is not aperiodic
    /// (e.g. the Hirschhorn pentagon realization).
    Nonperiodic,
}

impl TilingCategory {
    /// Canonical display order (most-ordered → least-ordered).
    pub const ALL: [TilingCategory; 6] = [
        TilingCategory::Uniform,
        TilingCategory::Laves,
        TilingCategory::Wallpaper,
        TilingCategory::Quasiperiodic,
        TilingCategory::Aperiodic,
        TilingCategory::Nonperiodic,
    ];

    /// Human-readable label, used verbatim as the section header in
    /// both UIs and matched by the search filter.
    pub fn label(self) -> &'static str {
        match self {
            TilingCategory::Uniform => "Uniform",
            TilingCategory::Laves => "Laves (Catalan duals)",
            TilingCategory::Wallpaper => "Wallpaper",
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
        assert_eq!(TilingCategory::ALL.len(), 6);
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