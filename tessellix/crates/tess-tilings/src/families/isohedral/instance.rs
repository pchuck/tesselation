//! Shared instance plumbing: one [`IhTiling`] per shipped IH type,
//! wiring a pinned [`TypeData`] record into the [`Tiling`] trait
//! (the `penrose.rs` direct-implementation pattern — no `LatticeTiling`
//! tables; the tiling is exactly aspects × ℤ² translations, so
//! generation is a bounded (i, j) box × aspects loop, engine.rs).

use tess_core::params::{ParamId, ParamSet, ParamSpec};
use tess_core::symmetry::{LatticeKind, SymmetryMeta, WallpaperGroup};
use tess_core::tiling::{Query, Tile, Tiling, TilingResult};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::similarity_map;

use super::engine::{self, TypeData};

/// Static description of one shipped isohedral instance.
pub(crate) struct IhTiling {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    /// The pinned Tactile type record (generated `data_ih*.rs`).
    pub data: &'static TypeData,
    /// Shape-parameter ids (length `data.np`; engine order).
    pub shape_ids: &'static [ParamId],
    /// Shape-parameter float defaults (length `data.np`; the same
    /// values the `specs` declare).
    pub defaults: &'static [f64],
    /// All declared parameters: the shape params first, then the shared
    /// view params (scale, rotation).
    pub specs: &'static [ParamSpec],
    pub group: WallpaperGroup,
    pub lattice: LatticeKind,
    pub category: TilingCategory,
    pub secondary: &'static [TilingCategory],
    pub symmetry_notes: &'static str,
}

impl Tiling for IhTiling {
    fn id(&self) -> &'static str {
        self.id
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn description(&self) -> &'static str {
        self.description
    }

    fn category(&self) -> TilingCategory {
        self.category
    }

    fn secondary_categories(&self) -> &'static [TilingCategory] {
        self.secondary
    }

    fn declared_symmetry(&self) -> Option<SymmetryMeta> {
        Some(self.symmetry())
    }

    fn params(&self) -> &'static [ParamSpec] {
        self.specs
    }

    fn default_color_mode(&self) -> ColorMode {
        ColorMode::Pattern
    }

    fn generate(&self, params: &ParamSet, query: &Query) -> TilingResult {
        let meta = self.symmetry();
        let map = similarity_map(params);
        match engine::generate_tiles(
            self.data,
            self.defaults,
            self.shape_ids,
            params,
            &query.region,
            map,
        ) {
            Ok((tiles, periodic)) => TilingResult {
                tiles: tiles
                    .into_iter()
                    .map(|(poly, seed)| Tile {
                        proto: 0,
                        poly,
                        seed, level: 0,
                    })
                    .collect(),
                symmetry: meta,
                periodic: Some(periodic),
                message: None,
            },
            Err(message) => TilingResult {
                tiles: Vec::new(),
                symmetry: meta,
                periodic: None,
                message: Some(message),
            },
        }
    }
}

impl IhTiling {
    fn symmetry(&self) -> SymmetryMeta {
        SymmetryMeta::new(self.group, self.lattice).with_notes(self.symmetry_notes)
    }
}

/// The standard view-region used by the isohedral batteries.
#[cfg(test)]
pub(crate) fn region(x0: f64, y0: f64, x1: f64, y1: f64) -> tess_core::geometry::BBox {
    tess_core::geometry::BBox::new(
        tess_core::geometry::Vec2::new(x0, y0),
        tess_core::geometry::Vec2::new(x1, y1),
    )
}
