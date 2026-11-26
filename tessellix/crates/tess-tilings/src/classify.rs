//! The per-tiling classification composer: [`classification`] maps a
//! registry id to its declared [`TilingClass`] — the optional,
//! per-family display facts the UIs render (only the fields that are
//! present; a Penrose card shows no wallpaper group, a pentagon card
//! shows no vertex configuration, and nothing renders as "N/A").
//!
//! Classification is *declared*, not derived: like the primary
//! category it states the taxonomy honestly (a nonperiodic
//! realization of a tile that also admits periodic tilings must not
//! claim quasiperiodicity). The arms live in the family modules
//! (`crate::families`), one `classification` fn per family; `tess-core` only defines the [`TilingClass`] shape
//! and the rendering rules (`tess_core::browse::attr_lines`), and both
//! UIs inject `classification` into
//! [`tess_core::browse::browse_tabs_with`], which also drives the
//! declared tab sort orders (pentagon-type sequence, 2-uniform vertex
//! configurations, Laves ranks, wallpaper crystal systems).
//!
//! Conventions:
//! - `vertex_config` / `face_config` use the registry's notation
//!   (superscript counts, `;` between vertex orbits, `V` prefix on a
//!   Laves face configuration). For the two 2-uniform pairs that
//!   share a configuration, the disambiguating subscript (₁/₂) is part
//!   of the configuration, matching the card names (M113).
//! - `dual_of` names the dual partner (short form; "self-dual" for
//!   the regular trio).
//! - `contact_angle` and `inflation` are display-ready strings
//!   ("θK 45°", "1+√2", "λ = 2").
//! - `laves_rank` orders the Laves tab: 0 = self-dual regular, 1 =
//!   Laves proper, 2 = 2-uniform dual.
//! - `pentagon_type` orders the Pentagon tab (types 1–15; cairo = 4
//!   and floret = 5 join as secondary members).

use tess_core::classify::TilingClass;

/// The declared classification facts for the tiling registered under
/// `id` (empty when the id has no entry — the coverage test below
/// fails on any registered tiling missing from this table).

use crate::families;

/// The declared classification facts for the tiling registered under
/// `id` (empty when the id has no entry — the coverage test below
/// fails on any registered tiling missing from the family tables).
pub fn classification(id: &str) -> TilingClass {
    families::classify_into(id).unwrap_or(TilingClass::EMPTY)
}

/// A 2-uniform tiling class: `k = 2` and the pair of vertex
/// configurations (with the ₁/₂ subscript when a configuration pair
/// needs disambiguating, matching the M113 card names).
// The helpers below are called from whichever families' classification
// arms use them; in subset builds that disable those families they go
// dead, which is expected (the full family set is the default build).
#[allow(dead_code)]
pub(crate) fn two_uniform(families: &'static [&'static str], vertex_config: &'static str) -> TilingClass {
    TilingClass {
        families,
        vertex_config: Some(vertex_config),
        k: Some(2),
        ..TilingClass::EMPTY
    }
}

/// A Laves-proper class: face configuration, dual partner, rank 1.
#[allow(dead_code)]
pub(crate) fn laves(
    families: &'static [&'static str],
    face_config: &'static str,
    dual_of: &'static str,
) -> TilingClass {
    TilingClass {
        families,
        face_config: Some(face_config),
        dual_of: Some(dual_of),
        laves_rank: Some(1),
        ..TilingClass::EMPTY
    }
}

/// A machine-dualized 2-uniform dual: V-face configuration of both
/// vertex orbits, primal reference, rank 2.
#[allow(dead_code)]
pub(crate) fn kuniform_dual(face_config: &'static str, primal: &'static str) -> TilingClass {
    TilingClass {
        families: &["Laves"],
        face_config: Some(face_config),
        dual_of: Some(primal),
        laves_rank: Some(2),
        k: Some(2),
        ..TilingClass::EMPTY
    }
}

/// A pentagon-type class: type number (the Pentagon-tab sort key),
/// isohedral class, attribution.
#[allow(dead_code)]
pub(crate) fn pentagon(
    families: &'static [&'static str],
    pentagon_type: u8,
    isohedral: u8,
    source: &'static str,
    year: &'static str,
    chirality: Option<&'static str>,
) -> TilingClass {
    TilingClass {
        families,
        pentagon_type: Some(pentagon_type),
        isohedral: Some(isohedral),
        source: Some(source),
        year: Some(year),
        chirality,
        ..TilingClass::EMPTY
    }
}

/// A Hankin polygons-in-contact entry.
#[allow(dead_code)]
pub(crate) fn hankin_pic(base_tiling: &'static str, contact_angle: &'static str) -> TilingClass {
    TilingClass {
        families: &["Islamic"],
        method: Some("Hankin polygons-in-contact"),
        base_tiling: Some(base_tiling),
        contact_angle: Some(contact_angle),
        ..TilingClass::EMPTY
    }
}

/// A Hankin greedy-inference entry (optional star polygon).
#[allow(dead_code)]
pub(crate) fn hankin_greedy(base_tiling: &'static str, contact_angle: &'static str, star: &'static [&'static str]) -> TilingClass {
    TilingClass {
        families: &["Islamic"],
        method: Some("Hankin greedy inference"),
        base_tiling: Some(base_tiling),
        contact_angle: Some(contact_angle),
        star_type: star.first().copied(),
        ..TilingClass::EMPTY
    }
}

/// A limit-periodic rep-tile realization (armchair, chair, sphinx,
/// P-pentomino).
#[allow(dead_code)]
pub(crate) fn rep_tile(tile_set: &'static str) -> TilingClass {
    TilingClass {
        families: &["Nonperiodic"],
        method: Some("substitution"),
        inflation: Some("λ = 2"),
        prototiles: Some(1),
        tile_set: Some(tile_set),
        translational: Some("limit-periodic"),
        ..TilingClass::EMPTY
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tess_core::category::TilingCategory;

    /// Every registered tiling must carry a non-empty class — this is
    /// the guard that forces a classify-table entry whenever a new
    /// tiling registers (and the reason the "hide absent fields" UI
    /// rule never degrades to a bare description).
    #[test]
    fn classification_covers_every_registered_tiling() {
        for t in crate::registry().iter() {
            let class = classification(t.id());
            assert!(
                !class.is_empty(),
                "{} ({}) has no classification entry",
                t.id(),
                t.name()
            );
            assert!(
                !class.families.is_empty(),
                "{} declares no families",
                t.id()
            );
        }
    }

    /// The Pentagon tab sort key: types 2–15 all declared, cairo = 4
    /// and floret = 5 as the aliased secondary members.
    #[test]
    fn pentagon_types_are_declared_sequentially() {
        for (id, ty) in [
            ("cairo-pentagonal", 4),
            ("floret-pentagonal", 5),
            ("pentagon-2", 2),
            ("pentagon-3", 3),
            ("pentagon-6", 6),
            ("pentagon-7", 7),
            ("pentagon-8", 8),
            ("pentagon-9", 9),
            ("pentagon-10", 10),
            ("pentagon-11", 11),
            ("pentagon-12", 12),
            ("pentagon-13", 13),
            ("pentagon-14", 14),
            ("pentagon-15", 15),
        ] {
            assert_eq!(classification(id).pentagon_type, Some(ty), "{id}");
        }
    }

    /// Every Laves-tab member declares a rank (0 self-dual regular,
    /// 1 Laves proper, 2 2-uniform dual) and a face configuration or
    /// Schläfli symbol.
    #[test]
    fn laves_members_declare_ranks() {
        for t in crate::registry().iter() {
            let member = t.category() == TilingCategory::Laves
                || t.secondary_categories().contains(&TilingCategory::Laves);
            if !member {
                continue;
            }
            let class = classification(t.id());
            assert!(class.laves_rank.is_some(), "{} lacks laves_rank", t.id());
            assert!(
                class.face_config.is_some() || class.schlafli.is_some(),
                "{} lacks a face configuration",
                t.id()
            );
        }
    }

    /// Every 2-uniform primary declares its vertex configuration (the
    /// Two-Uniform tab sort key) and k = 2.
    #[test]
    fn two_uniform_primaries_declare_configs() {
        for t in crate::registry().iter() {
            if t.category() != TilingCategory::TwoUniform {
                continue;
            }
            let class = classification(t.id());
            assert!(
                class.vertex_config.is_some(),
                "{} lacks vertex_config",
                t.id()
            );
            assert_eq!(class.k, Some(2), "{}", t.id());
        }
    }
}