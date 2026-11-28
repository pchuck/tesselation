//! Medallion weave wiring — the third-layer surface between the
//! catalog records and the strapwork engine (design doc:
//! `docs/medallion-third-layer-design.md`).
//!
//! The trait hooks [`Tiling::suppressed_seeds`] / [`Tiling::extra_walls`]
//! default to empty for every `LatticeTiling`-backed tiling: the
//! registry and the culling list keep those tilings as *values*, so a
//! value cannot override the hooks. Composition tilings that are
//! values register their layer beside the registry instead, keyed by
//! tiling id ([`medallion_layer_specs`]); [`medallion_wiring`] merges
//! both sources into the fused construction. The engine only ever
//! sees `Vec<Wall>` segments and two opaque labels (design F7) — this
//! module adds no `Tiling` implementation and no registry movement
//! (the 169 built-ins / 188 memberships are untouched).
//!
//! Rows are either ungated — fused by [`medallion_wiring`] /
//! [`fused_weave`] under `--medallion-weave` — or **motif-gated**
//! ([`MedallionLayerSpec::motif`]): a `Some(m)` row fuses only when
//! `--motif m` names it ([`fused_weave_motif`]), so the §22e monster
//! layers stay off-state byte-identical under the plain weave.
//!
//! [`Tiling::suppressed_seeds`]: tess_core::tiling::Tiling::suppressed_seeds
//! [`Tiling::extra_walls`]: tess_core::tiling::Tiling::extra_walls

use tess_core::geometry::BBox;
use tess_core::strapwork::{walls, walls_layered, MedallionContext, Wall};
use tess_core::tiling::{Tiling, Tile};

/// One row of the layer table: the composition tiling's id, the two
/// opaque labels the §4.4 gate error names, the suppressed seeds (the
/// hull footprint, design §9.3), and the pure emitter deriving the
/// record segments from the emitted tiles.
pub struct MedallionLayerSpec {
    pub tiling_id: &'static str,
    pub record: &'static str,
    pub absorption: &'static str,
    pub suppressed: &'static [u32],
    /// Motif gate: `None` fuses under `--medallion-weave`; `Some(m)`
    /// fuses only under `--motif m` (the §22e monster layers — the
    /// medallions are vertex-centered and virtual, so `suppressed`
    /// stays empty: every tile is a wall carrier).
    pub motif: Option<&'static str>,
    pub extra: fn(&[Tile]) -> Vec<Wall>,
}

/// The layer table (see the module docs for why the table exists
/// beside the registry).
pub fn medallion_layer_specs() -> &'static [MedallionLayerSpec] {
    &[
        MedallionLayerSpec {
            tiling_id: "dodecagon-hexagram-honeycomb",
            record: "M172 {12/4} dodecagon \u{2295} {6/2} hexagram",
            absorption: "LandingIdentity",
            suppressed: &[0, 1],
            motif: None,
            extra: crate::families::islamic::dodecagon_hexagram_honeycomb::medallion_extra_walls,
        },
        MedallionLayerSpec {
            tiling_id: "ammann-beenker",
            record: "22e {16/8} hexadecagon monster (Ammann-Beenker)",
            absorption: "LandingIdentity",
            suppressed: &[],
            motif: Some("22e"),
            extra: crate::medallion::motif22e::ab_extra_walls,
        },
        MedallionLayerSpec {
            tiling_id: "penrose",
            record: "22e {10/2} decagon monster (Penrose)",
            absorption: "BoundaryInference",
            suppressed: &[],
            motif: Some("22e"),
            extra: crate::medallion::motif22e::penrose_extra_walls,
        },
    ]
}

/// The resolved layer for one emission of one tiling: what was
/// suppressed (seeds and tiles, for the gate context) and the fused
/// `extra` segments, with the labels a named error carries.
pub struct MedallionWiring {
    pub record: &'static str,
    pub absorption: &'static str,
    pub suppressed_seeds: Vec<u32>,
    pub suppressed: Vec<Tile>,
    pub extra: Vec<Wall>,
}

/// Merge the trait hooks and the layer table for `t`'s current
/// emission. `None` means no layer at all: the caller must stay on
/// the plain two-layer pipeline (the off-state, byte-identical).
/// Only ungated rows ([`MedallionLayerSpec::motif`] `= None`) take
/// part — motif rows need [`fused_weave_motif`].
pub fn medallion_wiring(t: &dyn Tiling, tiles: &[Tile], region: &BBox) -> Option<MedallionWiring> {
    wiring_with(t, tiles, region, None)
}

/// The shared merge: `motif = None` selects the ungated rows (the
/// `--medallion-weave` wiring), `motif = Some(m)` the rows gated by
/// `--motif m`. Trait hooks always contribute (they are the tiling's
/// own layer, never motif-gated).
fn wiring_with(
    t: &dyn Tiling,
    tiles: &[Tile],
    region: &BBox,
    motif: Option<&str>,
) -> Option<MedallionWiring> {
    let mut seeds = t.suppressed_seeds();
    let mut extra = t.extra_walls(tiles, region);
    let mut labels: Option<(&'static str, &'static str)> = None;
    if !seeds.is_empty() || !extra.is_empty() {
        labels = Some((t.name(), "tiling-supplied"));
    }
    if let Some(spec) = medallion_layer_specs()
        .iter()
        .find(|s| s.tiling_id == t.id() && s.motif == motif)
    {
        for s in spec.suppressed {
            if !seeds.contains(s) {
                seeds.push(*s);
            }
        }
        extra.extend((spec.extra)(tiles));
        labels = Some((spec.record, spec.absorption));
    }
    if seeds.is_empty() && extra.is_empty() {
        return None;
    }
    let suppressed: Vec<Tile> = tiles
        .iter()
        .filter(|x| seeds.contains(&x.seed))
        .cloned()
        .collect();
    let (record, absorption) = labels.unwrap_or(("layer", "unspecified"));
    Some(MedallionWiring {
        record,
        absorption,
        suppressed_seeds: seeds,
        suppressed,
        extra,
    })
}

/// Owned §4.4 gate context (the engine's [`MedallionContext`] borrows;
/// this owns the tile/segment lists so they can travel with a plan).
pub struct OwnedMed {
    pub record: &'static str,
    pub absorption: &'static str,
    pub suppressed: Vec<Tile>,
    pub extra: Vec<Wall>,
}

impl OwnedMed {
    /// Borrow as the engine's gate context.
    pub fn ctx(&self) -> MedallionContext<'_> {
        MedallionContext {
            record: self.record,
            absorption: self.absorption,
            suppressed: &self.suppressed,
            extra: &self.extra,
        }
    }
}

/// The fused third-layer wall emission plus the gate context.
pub struct FusedWeave {
    pub walls: Vec<Wall>,
    /// `None` = off-state (no layer): callers must use the plain
    /// [`tess_core::strapwork::plan_decoration`] path byte-identically.
    pub med: Option<OwnedMed>,
}

/// Fuse `t`'s medallion layer into the wall set at contact angle
/// `delta_deg` (the suppression-filtered generic firing plus the
/// record segments, [`walls_layered`]). With no layer this is exactly
/// `walls(tiles, delta_deg)`.
pub fn fused_weave(
    t: &dyn Tiling,
    tiles: &[Tile],
    region: &BBox,
    delta_deg: f64,
) -> FusedWeave {
    fuse(t, tiles, region, delta_deg, None)
}

/// The row of `tiling_id` gated by motif `motif`, if any.
pub fn motif_layer_spec(tiling_id: &str, motif: &str) -> Option<&'static MedallionLayerSpec> {
    medallion_layer_specs()
        .iter()
        .find(|s| s.tiling_id == tiling_id && s.motif == Some(motif))
}

/// The ids carrying motif `motif` (for the CLI's named refusal).
pub fn motif_carriers(motif: &str) -> Vec<&'static str> {
    medallion_layer_specs()
        .iter()
        .filter(|s| s.motif == Some(motif))
        .map(|s| s.tiling_id)
        .collect()
}

/// Fuse `t`'s **motif-gated** layer at contact angle `delta_deg`.
/// `None` means `t` carries no row for `motif`: the caller refuses by
/// name ([`motif_carriers`]) — never a silent fall back to the plain
/// pipeline. The ungated rows do not take part; the off-state (no
/// `--motif`) stays on the byte-identical two-layer path.
pub fn fused_weave_motif(
    t: &dyn Tiling,
    tiles: &[Tile],
    region: &BBox,
    delta_deg: f64,
    motif: &str,
) -> Option<FusedWeave> {
    if motif_layer_spec(t.id(), motif).is_none() {
        return None;
    }
    Some(fuse(t, tiles, region, delta_deg, Some(motif)))
}

/// The shared fusion: build the [`wiring_with`] layer, drop the
/// suppressed tiles, fuse the record segments through
/// [`walls_layered`].
fn fuse(
    t: &dyn Tiling,
    tiles: &[Tile],
    region: &BBox,
    delta_deg: f64,
    motif: Option<&str>,
) -> FusedWeave {
    match wiring_with(t, tiles, region, motif) {
        None => FusedWeave {
            walls: walls(tiles, delta_deg),
            med: None,
        },
        Some(w) => {
            let kept: Vec<Tile> = tiles
                .iter()
                .filter(|x| !w.suppressed_seeds.contains(&x.seed))
                .cloned()
                .collect();
            let walls = walls_layered(&kept, delta_deg, &w.extra);
            FusedWeave {
                walls,
                med: Some(OwnedMed {
                    record: w.record,
                    absorption: w.absorption,
                    suppressed: w.suppressed,
                    extra: w.extra,
                }),
            }
        }
    }
}
