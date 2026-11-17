//! Pentagon type 6 (Richard Kershner, 1968) tiling — 2-isohedral,
//! edge-to-edge, wallpaper group p2.
//!
//! Type 6 is the one-parameter family with conditions, in the repo's
//! vertex/side lettering, **`a = d = e`, `b = c`, `A = 2C`,
//! `C + E = 180°`** (equivalently `B + D = 360° − 2C`).  (The
//! Wikipedia article uses a shifted lettering — W-angle X at the
//! side-x/side-x+1 junction, so W-A = our-B, W-B = our-C, W-C = our-D,
//! W-D = our-E, W-E = our-A, sides unchanged; the article's
//! "B + D = 180°, 2B = E" is the same family restated in that
//! convention.)  The instance shipped here is the unique 30°-grid
//! member of the family (all edge directions on the 30° grid, hence all
//! coordinates exact Q(√3)):
//!
//! * angles (A, B, C, D, E) = (120°, 120°, 60°, 120°, 120°);
//! * sides (shipped scale): a = d = e = 1, b = c = 2;
//! * tile area: 7√3/4.
//!
//! The prototile is **achiral** — its mirror image is directly
//! congruent to it (mirror_y(T0) = R300·T0 + (−1/2, −√3/2)) — so the
//! usual chiral-prototile argument for excluding mirrors does not
//! apply.  The group is nevertheless **p2** (orbifold 2222),
//! established two ways: (1) a complete constructive mirror search —
//! every tiling corner T_j[q] (20 of them, 6 classes) as the image of
//! the prototile's A-corner, every length-1 tiling ray at that corner
//! as the image of side AB (22 candidates), with the det −1 linear part
//! forced in each case — finds 0 mirrors, and (2) the normalizer
//! argument: any tiling symmetry g = t + R conjugates the translation
//! group T = Λ to itself, so R must normalize Λ; no orientation-
//! reversing isometry normalizes Λ (both det −1 sign cases give
//! R(L1)·R(L2) = −3 ≠ 3).  Note the block check alone is not a
//! completeness proof: it admits 4 spurious "subset maps" (4-cell
//! image ok, but R(Λ) ≠ Λ) — the normalizer condition is what kills
//! them.
//!
//! The half-turn centres are exactly the four classes
//! **c0 + Λ/2 mod Λ** with c0 = (1/2, 0): an exhaustive search over all
//! 14 candidate Λ/2 cosets (400 corner sums) finds exactly one centre
//! coset, and every one of the four representatives is verified to
//! map the cell set to itself.  The half-turn permutes the cells as
//! 0 ↔ 1, 2 ↔ 3, so the tiling is **2-isohedral** with full-group
//! orbits {T0, T1} and {T2, T3} (the seeds).
//!
//! The tiling is **edge-to-edge at the pentagon level**: every stored
//! side has exactly one full reverse neighbour among the 20 stored
//! sides — 4 pairs glue in the unit (shift 0), 6 pairs across the
//! shifts ±w1, ±w2, ±(w1−w2) — an involution with side-pair census
//! a-a:1, a-d:2, b-b:1, b-c:2, c-c:1, d-e:2, e-e:1.  Mod the unit
//! lattice the 20 directed sides wrap to 18 directed classes, with one
//! double-covered arc: the (2,√3)→(0,√3)≡(3,0) arc carries two lifts
//! in each direction (T0e2, T3e1 / T1e2, T2e1; (3,0) = (0,√3) − w2).
//! The six vertex orbits (sizes 3, 3, 3, 3, 4, 4) carry the two closed
//! stars 2×{60, 60, 120, 120} and 4×{120, 120, 120} — Euler V − E + F
//! = 6 − 10 + 4 = 0.
//!
//! The cell was validated against the decompiled Jaap Scherphuis
//! applet's type-6 class (06a) at its "B = 60°" slider setting: the
//! applet's base cell is carried to the pinned cell vertex-exactly by
//! the similarity S(z) = (1 + i√3)z + 1 (a √3 scale, 60° rotation, and
//! one lattice translation), so the applet's cell and the shipped cell
//! are the same tiling up to the standard similarity (the applet's
//! default slider value is a different family member; the exact
//! Q(√3) member is shipped, M62/M63/M73 precedent).
//!
//! **Floret kinship**: the pinned prototile is congruent to the
//! floret-pentagonal tile (laves.rs `floret_pentagonal`):
//! pin = R30⁷·(√3 · floret6) + (2, √3), pointwise — a different tiling
//! (p2, 4-tile, 2-isohedral) of the same pentagon shape (floret's is
//! p6, 6-tile, 1-isohedral).
//!
//! Primitive cell: **4 tiles** on the oblique lattice
//! `w1 = (1, 2√3)`, `w2 = (−3, √3)` with
//! `|det(w1, w2)| = 7√3 = 4 × tile area` (`|w1|² = 13`, `|w2|² = 12`,
//! `w1 · w2 = 3`).

use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;
use tess_core::TilingCategory;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √3 — every coordinate in this module is a Q(√3) surd built from it
/// (computed at runtime; no decimal approximations are shipped).
pub(crate) fn s3() -> f64 {
    3.0f64.sqrt()
}

/// (x2/2, y2·√3/2) — every tile vertex lies on the half-integer ×
/// integer·√3/2 grid, so the helper takes doubled numerators.
fn c(x2: f64, y2: f64) -> Vec2 {
    Vec2::new(x2 / 2.0, y2 * s3() / 2.0)
}

#[cfg(test)]
/// Exact tile area, 7√3/4.
pub(crate) fn tile_area() -> f64 {
    7.0 * s3() / 4.0
}

#[cfg(test)]
/// 2-fold rotation centre of the tiling: c0 = (1/2, 0).  All half-turn
/// centres are c0 + Λ/2 (one coset); c0 is the representative at the
/// midpoint of the prototile's a-side.
pub(crate) fn two_fold_centre() -> Vec2 {
    c(1.0, 0.0)
}

/// Pentagon type 6 (Kershner 1968): 2-isohedral, 4 tiles per oblique
/// cell, wallpaper group p2, edge-to-edge.
pub fn spec() -> LatticeTiling {
    // Four pentagon cells in CCW order, canonical placement: the 60°
    // corner C sits at vertex index 2 of every cell, so each cell's
    // (A, B, C, D, E) vertex order matches the prototile T0's (the
    // cyclic alignment is the identity, s_k = 0 for all k).  The seeds
    // follow the two full-group isohedral orbits {T0, T1} / {T2, T3};
    // the half-turn about c0 swaps 0 <-> 1 and 2 <-> 3.
    let t0 = Polygon::from_slice(&[
        c(0.0, 0.0),
        c(2.0, 0.0),
        c(4.0, 2.0),
        c(0.0, 2.0),
        c(-1.0, 1.0),
    ]);
    let t1 = Polygon::from_slice(&[
        c(4.0, 4.0),
        c(2.0, 4.0),
        c(0.0, 2.0),
        c(4.0, 2.0),
        c(5.0, 3.0),
    ]);
    let t2 = Polygon::from_slice(&[
        c(5.0, -1.0),
        c(6.0, 0.0),
        c(4.0, 2.0),
        c(2.0, 0.0),
        c(3.0, -1.0),
    ]);
    let t3 = Polygon::from_slice(&[
        c(5.0, 3.0),
        c(4.0, 2.0),
        c(6.0, 0.0),
        c(8.0, 2.0),
        c(7.0, 3.0),
    ]);
    LatticeTiling {
        id: "pentagon-6",
        name: "Pentagon type 6 (Kershner)",
        description: "Pentagonal tiling of type 6 (Richard Kershner, 1968): A = B = D = E = 120°, C = 60°, a = d = e = 1, b = c = 2 (A = 2C, C + E = 180°); edge-to-edge (every side is matched by exactly one full partner side, all reverse-glued), 2-isohedral (seeds 0/1), 4 tiles per oblique cell, wallpaper group p2 (one half-turn centre coset (1/2, 0) + Λ/2, no mirrors — achiral prototile, excluded by complete ray search plus the normalizer argument; the prototile is congruent to the floret-pentagonal tile).",
        lattice: [c(2.0, 4.0), c(-6.0, 2.0)],
        cells: vec![
            Cell { poly: t0, seed: 0 },
            Cell { poly: t1, seed: 0 },
            Cell { poly: t2, seed: 1 },
            Cell { poly: t3, seed: 1 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P2,
        lattice_kind: LatticeKind::Oblique,
        category: TilingCategory::PentagonTypes,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
