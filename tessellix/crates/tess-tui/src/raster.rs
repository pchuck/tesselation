//! Re-export of the shared rasterizer.
//!
//! The rasterizer itself moved to `tess_core::raster` (UI-2) so the
//! GUI's card previews share it instead of duplicating it. This module
//! is kept as a re-export so existing `crate::raster` paths keep
//! working.

pub use tess_core::raster::*;
