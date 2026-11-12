//! The core [`Tiling`] trait and output types.
//!
//! A tiling is anything that, given a bounded region and a set of
//! parameters, can produce the tiles intersecting that region. Periodic
//! tilings, substitution tilings (Penrose), and continuous families
//! (Hirschhorn) all fit this contract; UIs never need to know which one
//! they are dealing with.

use crate::geometry::{BBox, Polygon, Vec2};
use crate::params::ParamSet;
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
#[derive(Debug, Clone)]
pub struct Tile {
    pub proto: u8,
    pub poly: Polygon,
    pub seed: u32,
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
}

// Re-export for convenience in trait signature.
use crate::params::ParamSpec;
