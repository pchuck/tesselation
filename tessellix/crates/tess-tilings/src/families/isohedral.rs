//! Isohedral family: the parameterized isohedral (IH) types —
//! Grünbaum–Shephard's IH1–IH93 monohedral tiling types realized as
//! incidence symbols over their topologies, with shaped edges as
//! exact per-orbit polylines (the "Escher arc", PROPOSED.md §13).
//!
//! Owned here: module declarations, `register` (the family's registry
//! slice, in within-tab order), `classification` arms, the family's
//! lattice-culling entries and the family census pin. The family doc is
//! `docs/families/isohedral.md`. Design doc: `docs/isohedral-generator-design.md`.

use tess_core::Registry;
use tess_core::classify::TilingClass;

/// Submodules: the exact `Q(√3)` field, the generic engine over pinned
/// Tactile type records, the shared `Tiling` plumbing, the generated
/// per-type data tables (`data_ih*`, spike5 codegen), and one module per
/// shipped instance.
pub(crate) mod engine;
pub(crate) mod field;
pub(crate) mod instance;

mod data_ih1;
mod data_ih27;
mod data_ih29;
mod data_ih6;
mod data_ih7;
mod ih1;
mod ih27;
mod ih29;
mod ih6;
mod ih7;

/// The family's registry slice, in within-tab order.
pub fn register(mut r: Registry) -> Registry {
    r = r.register(ih1::TILING);
    r = r.register(ih6::TILING);
    r = r.register(ih7::TILING);
    r = r.register(ih27::TILING);
    r = r.register(ih29::TILING);
    r
}

/// The family's classification arms (see `crate::classify` for the
/// composer and the shared constructor helpers).
///
/// Every shipped type is 1-isohedral by definition (`isohedral: Some(1)`
/// — the field counts isohedral classes, not the IH type number; the
/// type number lives in `method`). Topologies verified by dual census
/// (design doc §6): IH1/6/7 on the 6³ honeycomb net (dual: triangular),
/// IH27/29 on the Cairo net V3.3.4.3.4 (dual: snub square), so all five
/// take primary Laves and the pentagons add secondary PentagonTypes
/// (the `cairo-pentagonal` precedent). The Wallpaper tab is never
/// touched — these are realizations, not group representatives.
pub(crate) fn classification(id: &str) -> Option<TilingClass> {
    use crate::classify::laves;
    Some(match id {
        "ih1-p1-hexagon" => TilingClass {
            isohedral: Some(1),
            method: Some("Kaplan Tactile incidence symbol (IH1)"),
            translational: Some("periodic"),
            cell_content: Some("1 tile per translation cell"),
            prototiles: Some(1),
            source: Some("Kaplan (Tactile)"),
            ..laves(&["Laves"], "V3.3.3.3.3.3", "triangular")
        },
        "ih6-pgg-hexagon" => TilingClass {
            isohedral: Some(1),
            method: Some("Kaplan Tactile incidence symbol (IH6)"),
            translational: Some("periodic"),
            cell_content: Some("4 tiles per translation cell"),
            prototiles: Some(1),
            source: Some("Kaplan (Tactile)"),
            ..laves(&["Laves"], "V3.3.3.3.3.3", "triangular")
        },
        "ih7-p3-hexagon" => TilingClass {
            isohedral: Some(1),
            method: Some("Kaplan Tactile incidence symbol (IH7)"),
            translational: Some("periodic"),
            cell_content: Some("3 tiles per translation cell"),
            prototiles: Some(1),
            source: Some("Kaplan (Tactile)"),
            ..laves(&["Laves"], "V3.3.3.3.3.3", "triangular")
        },
        "ih27-pgg-pentagon" => TilingClass {
            isohedral: Some(1),
            method: Some("Kaplan Tactile incidence symbol (IH27)"),
            translational: Some("periodic"),
            cell_content: Some("4 tiles per translation cell"),
            prototiles: Some(1),
            source: Some("Kaplan (Tactile)"),
            ..laves(&["Laves", "Pentagon"], "V3.3.4.3.4", "snub square")
        },
        "ih29-p4g-pentagon" => TilingClass {
            isohedral: Some(1),
            method: Some("Kaplan Tactile incidence symbol (IH29)"),
            translational: Some("periodic"),
            cell_content: Some("4 tiles per translation cell"),
            prototiles: Some(1),
            source: Some("Kaplan (Tactile)"),
            ..laves(&["Laves", "Pentagon"], "V3.3.4.3.4", "snub square")
        },
        _ => return None,
    })
}

/// This family's entries for the global lattice-culling cross-check
/// (`lattice_culling_tests` in `lib.rs`).
#[cfg(test)]
#[cfg(test)]
mod census {
    use super::*;
    use std::collections::BTreeSet;

    /// The family's registry ids — by construction also its
    /// classification arm ids. Update alongside any membership change
    /// (this test fails on drift in either direction: a tiling
    /// registered but not classified, or classified but not
    /// registered).
    const IDS: &[&str] = &[
        "ih1-p1-hexagon",
        "ih6-pgg-hexagon",
        "ih7-p3-hexagon",
        "ih27-pgg-pentagon",
        "ih29-p4g-pentagon",
    ];

    #[test]
    fn family_census_matches() {
        let r = register(Registry::new());
        assert_eq!(
            r.len(),
            5,
            "isohedral family census — update when adding a tiling to this family"
        );
        let have: BTreeSet<&str> = r.iter().map(|t| t.id()).collect();
        let want: BTreeSet<&str> = IDS.iter().copied().collect();
        assert_eq!(
            have, want,
            "isohedral family membership drift: register() vs IDS"
        );
        for id in IDS {
            assert!(
                classification(id).is_some(),
                "{id} has no classification arm in this family"
            );
        }
    }
}