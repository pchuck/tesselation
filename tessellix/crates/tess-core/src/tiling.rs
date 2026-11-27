//! The core [`Tiling`] trait and output types.
//!
//! A tiling is anything that, given a bounded region and a set of
//! parameters, can produce the tiles intersecting that region. Periodic
//! tilings, substitution tilings (Penrose), and continuous families
//! (Hirschhorn) all fit this contract; UIs never need to know which one
//! they are dealing with.

use crate::category::TilingCategory;
use crate::geometry::{BBox, Polygon, Vec2};
use crate::params::ParamSet;
use crate::strapwork::Wall;
use crate::symmetry::SymmetryMeta;
use crate::view::ColorMode;

/// What to generate: the tiles intersecting this region.
#[derive(Debug, Clone, Copy)]
pub struct Query {
    pub region: BBox,
}

/// One tile.
///
/// * `proto` — index of the prototile (shape) this tile is a copy of.
///   Monohedral tilings use a single prototile; Penrose uses two rhombs.
/// * `poly`  — the concrete (transformed) polygon in world coordinates.
/// * `seed`  — small integer driving "pattern" coloring; chosen by the
///   generator so that symmetry-related tiles share structure
///   (e.g. checkerboard parity for the square grid).
/// * `level` — substitution level (hierarchy depth) of the tile, as
///   tracked by a substitution/expansion engine at emission time: the
///   number of refinement (deflation) rounds — or, in mixed-size
///   irreptile engines, the tile's lineage scale-steps — applied to
///   produce it. `0` = non-hierarchical: the generator knows no
///   substitution depth for this tile (every periodic/closed-form
///   family, and any engine whose depth is unavailable without deep
///   restructuring). Engines here select one expansion depth per view,
///   so in the uniform engines all tiles of one `generate` share that
///   engine depth — the level reads as the patch's position in the
///   family's hierarchy and steps as the view crosses substitution
///   octaves; the irreptile engines (golden bee, golden triangle)
///   carry genuinely mixed per-tile lineage depths within one view.
///   Drives [`ColorMode::Level`](crate::view::ColorMode::Level); never
///   affects any other mode's decision function.
#[derive(Debug, Clone)]
pub struct Tile {
    pub proto: u8,
    pub poly: Polygon,
    pub seed: u32,
    pub level: u8,
}

/// The output of a generation.
#[derive(Debug, Clone)]
pub struct TilingResult {
    pub tiles: Vec<Tile>,
    /// Symmetry facts, known analytically by the generator.
    pub symmetry: SymmetryMeta,
    /// Lattice vectors in world coordinates if the tiling is periodic.
    pub periodic: Option<[Vec2; 2]>,
    /// Set when generation was refused (e.g. too many tiles for the view).
    pub message: Option<&'static str>,
}

/// A tiling generator.
///
/// Implementors are stateless; all variation comes from [`ParamSet`].
///
/// # Adding a new tiling
///
/// 1. Implement this trait (usually by wrapping a helper generator from
///    `tess-tilings::lattice` for periodic tilings).
/// 2. Register it in the registry. Done — both UIs pick it up.
pub trait Tiling {
    /// Short stable identifier, e.g. `"square"`.
    fn id(&self) -> &'static str;
    /// Human readable name, e.g. `"Square grid"`.
    fn name(&self) -> &'static str;
    /// One-line description shown in UIs.
    fn description(&self) -> &'static str;
    /// Canonical display category: how the UIs group their tiling
    /// lists. Declared per tiling — never derived from geometry or
    /// symmetry — so the taxonomy stays honest (see [`TilingCategory`]).
    fn category(&self) -> TilingCategory;
    /// Additional display categories this tiling also belongs to
    /// (multi-tab membership). Defaults to `&[]` — most tilings belong to
    /// exactly one tab. Used, for example, by the three regular tilings
    /// (which are their own Catalan duals, so they also appear under
    /// Laves), and by the wallpaper-group representatives (which also
    /// appear in the Wallpaper group catalog tab).
    fn secondary_categories(&self) -> &'static [TilingCategory] {
        &[]
    }
    /// The wallpaper group and lattice kind this tiling declares, or
    /// `None` for tilings that are not wallpaper tilings (aperiodic,
    /// quasiperiodic) or whose lattice kind varies with parameters.
    /// Defaults to `None`; the standard periodic generators override this
    /// so symmetry metadata can be queried without running `generate`.
    fn declared_symmetry(&self) -> Option<SymmetryMeta> {
        None
    }
    /// The declared lattice basis (cell space) for tilings with a fixed,
    /// parameter-independent lattice. `None` when the tiling has no fixed
    /// lattice or its lattice depends on parameters (e.g. Pythagorean).
    /// Defaults to `None`.
    fn lattice_basis(&self) -> Option<[Vec2; 2]> {
        None
    }
    /// Parameter declarations, discovered by UIs.
    fn params(&self) -> &'static [ParamSpec];
    /// Color mode the UI should start with for this tiling.
    fn default_color_mode(&self) -> ColorMode {
        ColorMode::Pattern
    }
    /// Generate the tiles intersecting `query.region`.
    ///
    /// Implementors must return exactly the tiles (fully or partially)
    /// inside the region — no gaps, no overlaps, no outside tiles.
    ///
    /// Finite-patch generators are a documented exception: their patch
    /// *is* the covered domain, so a query reaching beyond it
    /// legitimately returns only the patch tiles that intersect the
    /// region, and UIs render background for the rest.
    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult;

    /// Medallion third-layer hook (design doc
    /// `docs/medallion-third-layer-design.md` §5.2): the seeds of the
    /// faces whose generic strapwork firing is replaced by the record
    /// layer supplied through [`Tiling::extra_walls`]. The seed *is*
    /// the hull footprint (design §9.3): suppression never needs a
    /// clipper. Defaults to empty — every existing tiling keeps the
    /// standard two-layer pipeline byte-identically.
    fn suppressed_seeds(&self) -> Vec<u32> {
        Vec::new()
    }

    /// Medallion third-layer hook (design doc §5.1): additional wall
    /// segments for the fused map — literally "more walls", the `Wall`
    /// currency and nothing else (design F1). Receives the freshly
    /// emitted `tiles` alongside the `region` so a composition tiling
    /// derives the layer geometry from the same polygons it emitted
    /// (this is a deliberate deviation from the doc's region-only
    /// signature: it keeps layer placement correct under the
    /// scale/rotation parameters without a second source of truth).
    /// Defaults to empty.
    fn extra_walls(&self, _tiles: &[Tile], _region: &BBox) -> Vec<Wall> {
        Vec::new()
    }
}

// Re-export for convenience in trait signature.
use crate::params::ParamSpec;
