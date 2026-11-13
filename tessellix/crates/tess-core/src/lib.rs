//! tess-core: core abstractions for the tessellix tiling generator.
//!
//! * [`geometry`] — vectors, affine maps, polygons, bounding boxes
//! * [`params`] — typed, self-describing tiling parameters
//! * [`tiling`] — the [`Tiling`] trait and generation results
//! * [`symmetry`] — wallpaper groups and symmetry metadata
//! * [`category`] — canonical display categories (declared per tiling)
//! * [`registry`] — the tiling registry
//! * [`strapwork`] — Hankin PIC strapwork layer engine (wall overlay)
//! * [`view`] — viewport (pan/zoom) and color modes
//! * [`paint`] — shared color palette
//! * [`app`] — shared application state for the TUI and GUI
//! * [`svg`] — SVG export

pub mod app;
pub mod category;
pub mod geometry;
pub mod paint;
pub mod params;
pub mod registry;
pub mod strapwork;
pub mod symmetry;
pub mod svg;
pub mod tiling;
pub mod view;

pub use app::App;
pub use category::TilingCategory;
pub use geometry::{Affine, BBox, Point, Polygon, Vec2};
pub use params::{ParamId, ParamKind, ParamSet, ParamSpec, ParamValue};
pub use registry::Registry;
pub use symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
pub use tiling::{Query, Tile, Tiling, TilingResult};
pub use view::{ColorMode, Viewport};
