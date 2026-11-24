//! The eleven tiling families — the unit of parallel work in this crate.
//!
//! Every tiling belongs to exactly one family; a family owns its tiling
//! modules, its `register` slice, its classification arms, its share of
//! the lattice-culling cross-check and its census pin (see the `census`
//! test in each family file). Adding a tiling to an existing family
//! touches only that family's file and the new tiling module — nothing
//! at the crate root. Adding a family is an orchestrator edit: a module
//! here, one line in the `registry()` chain in `lib.rs`, one in
//! `classify_into` below, one feature in `Cargo.toml`.
//!
//! The chain order in `lib.rs::registry()` is also the within-tab card
//! order contract: cards inside a tab appear in family-chain order, then
//! in the family's own registration order. Do not reorder without
//! updating the docs' catalog tables.
#[cfg(feature = "uniform")]
pub mod uniform;
#[cfg(feature = "quasiperiodic")]
pub mod quasiperiodic;
#[cfg(feature = "arrangements")]
pub mod arrangements;
#[cfg(feature = "pentagons")]
pub mod pentagons;
#[cfg(feature = "laves")]
pub mod laves;
#[cfg(feature = "kuniform")]
pub mod kuniform;
#[cfg(feature = "reptiles")]
pub mod reptiles;
#[cfg(feature = "aperiodic")]
pub mod aperiodic;
#[cfg(feature = "pavers")]
pub mod pavers;
#[cfg(feature = "islamic")]
pub mod islamic;
#[cfg(feature = "isohedral")]
pub mod isohedral;

use tess_core::Registry;
use tess_core::classify::TilingClass;

/// The classification composer's walk order (family chain; first family
/// with an arm for the id wins — arms are disjoint, guarded by the
/// classify coverage tests).
// `id` is unused only in degenerate subset builds with no families enabled.
#[allow(unused_variables)]
pub fn classify_into(id: &str) -> Option<TilingClass> {
    #[cfg(feature = "uniform")]
    {
        if let Some(c) = uniform::classification(id) {
            return Some(c);
        }
    }
    #[cfg(feature = "quasiperiodic")]
    {
        if let Some(c) = quasiperiodic::classification(id) {
            return Some(c);
        }
    }
    #[cfg(feature = "arrangements")]
    {
        if let Some(c) = arrangements::classification(id) {
            return Some(c);
        }
    }
    #[cfg(feature = "pentagons")]
    {
        if let Some(c) = pentagons::classification(id) {
            return Some(c);
        }
    }
    #[cfg(feature = "laves")]
    {
        if let Some(c) = laves::classification(id) {
            return Some(c);
        }
    }
    #[cfg(feature = "kuniform")]
    {
        if let Some(c) = kuniform::classification(id) {
            return Some(c);
        }
    }
    #[cfg(feature = "reptiles")]
    {
        if let Some(c) = reptiles::classification(id) {
            return Some(c);
        }
    }
    #[cfg(feature = "aperiodic")]
    {
        if let Some(c) = aperiodic::classification(id) {
            return Some(c);
        }
    }
    #[cfg(feature = "pavers")]
    {
        if let Some(c) = pavers::classification(id) {
            return Some(c);
        }
    }
    #[cfg(feature = "islamic")]
    {
        if let Some(c) = islamic::classification(id) {
            return Some(c);
        }
    }
    #[cfg(feature = "isohedral")]
    {
        if let Some(c) = isohedral::classification(id) {
            return Some(c);
        }
    }
    None
}

/// The registration chain (see the module docs): families register in
/// this order; `lib.rs::registry()` threads the builder through them.
// `mut` is unneeded only in degenerate subset builds with no families.
#[allow(unused_mut)]
pub fn chain(mut r: Registry) -> Registry {
    #[cfg(feature = "uniform")]
    {
        r = uniform::register(r);
    }
    #[cfg(feature = "quasiperiodic")]
    {
        r = quasiperiodic::register(r);
    }
    #[cfg(feature = "arrangements")]
    {
        r = arrangements::register(r);
    }
    #[cfg(feature = "pentagons")]
    {
        r = pentagons::register(r);
    }
    #[cfg(feature = "laves")]
    {
        r = laves::register(r);
    }
    #[cfg(feature = "kuniform")]
    {
        r = kuniform::register(r);
    }
    #[cfg(feature = "reptiles")]
    {
        r = reptiles::register(r);
    }
    #[cfg(feature = "aperiodic")]
    {
        r = aperiodic::register(r);
    }
    #[cfg(feature = "pavers")]
    {
        r = pavers::register(r);
    }
    #[cfg(feature = "islamic")]
    {
        r = islamic::register(r);
    }
    #[cfg(feature = "isohedral")]
    {
        r = isohedral::register(r);
    }
    r
}
