//! Stepped hexominoes (M78) — the first **p1** wallpaper realizer in
//! the catalog, expressed as a [`LatticeTiling`] data table.
//!
//! # The pattern
//!
//! A single asymmetric 2-step staircase hexomino tiled by its own
//! translation lattice. The prototile is the hexomino
//!
//! * `S`: unit squares `(0,0)`, `(0,1)`, `(1,1)`, `(1,2)`, `(2,1)`,
//!   `(2,2)` — the row `y ∈ [0,1]` spans `x ∈ [0,1]` and the two rows
//!   `y ∈ [1,3]` span `x ∈ [0,3]` minus the upper-left corner —
//!   a two-riser staircase of width three,
//!
//! on the lattice `Λ = ⟨(3,1), (0,2)⟩` (det 6); one unit cell holds
//! exactly one hexomino, area `6 = |det|`, so the lattice translates
//! tile the plane with no gaps and no overlaps. Every tile spans
//! exactly the columns `{3k, 3k+1, 3k+2}` of its cell's `λ_x = 3k`.
//!
//! # Edge-to-edge note
//!
//! The prototile polygon is emitted with all 12 unit boundary edges
//! as vertices — the four interior corners `(2,1)`, `(3,2)`, `(2,3)`,
//! `(0,1)` are `180°` (collinear) points where two unit edges meet —
//! so the partition is a genuine unit-edge, edge-to-edge one: every
//! interior unit edge is shared by exactly two tiles and every
//! interior vertex figure sums to 360°. (Structural note: the columns
//! `x ≡ 2 (mod 3)` are *complete* vertical tile-boundary lines — the
//! tiling decomposes into width-3 vertical bands. This is harmless to
//! the group: `(3,0) ∉ Λ`, so no two bands are congruent by a tiling
//! symmetry.)
//!
//! # Wallpaper group
//!
//! **p1** — translations only. The normalizer of `Λ` in the
//! isometry group of the plane is the centered-rectangular Klein
//! four-group `{I, R180, Mx, My}`, and the asymmetric staircase breaks
//! all three non-identity members: the 180° rotation, a horizontal
//! mirror, and a vertical mirror each map the staircase onto a
//! different hexomino shape (checked by exact vertex-set comparison).
//! The remaining D4 candidates (R90, R270, the diagonal and
//! anti-diagonal mirrors) do not normalize `Λ` in the first place.
//! Hence the symmetry group of the tiling is exactly its translation
//! group `Λ`.
//!
//! # Bravais kind
//!
//! **Centered rectangular**, declared on the *primitive* cell
//! `⟨(3,1), (0,2)⟩` (det 6) — the same convention as
//! `girih-star-cross`, which keeps a primitive cell of its centered
//! lattice. The centered cell is `⟨(0,2), (−6,0)⟩`: a 6×2 rectangle
//! (area 12 = 2·det) whose center `(−3,1) = −(3,1) + (0,2)` is a
//! lattice point. Measured from the declared basis, the lattice is
//! centered-rectangular (witness `(0,2), (−6,0)`: cross `12 = 2·det`,
//! orthogonal, cell center in `Λ`), so the legality test
//! (`declared_symmetry_is_legal_and_matches_the_measured_lattice`)
//! pins `LatticeKind::CenteredRectangular` here.
//!
//! # Seeds and protos
//!
//! `proto` = cell index (the single hexomino); `seed` = orientation
//! class — a single class (0), since every tile is the prototile
//! itself (1-isohedral, one orbit).
//!
//! # Verification
//!
//! Verified exactly before porting (Python, stdlib,
//! `m78_final_check.py`): coverage 0-bad over windows `W = 4/8/16`;
//! the six cell translates are pairwise incongruent mod `Λ`;
//! edge-to-edge confirmed by an exact T-junction test over windows
//! `W = 4/12/20`; the full D4 probe — R90/R270/Md/Ma fail the lattice
//! normalizer, R180/Mx/My fail the shape test, and all coset
//! candidates normalize to False — confirms p1 with normalizer
//! `{I, R180, Mx, My}`; an independent 12-tile patch carries no
//! non-identity symmetry; the vertex figures over `W = 12` (529
//! vertices) sum to 360° in every class
//! `((90,270), (180,180), (360), (90,90,90,90))`; and the polygon is
//! exact (shoelace `2·area = 12`, angle sum 1800° = 180°·(12−2)).
//! Rust: the standard trio, the brute-force culling cross-check
//! (`lattice_culling_matches_brute_force`), the in-module p1 group
//! probe (`stepped_hexominoes_wallpaper_group_is_p1`), and the
//! lattice-legality test above.

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

fn poly(pts: &[Vec2]) -> Polygon {
    Polygon::from_slice(pts)
}

/// The asymmetric 2-step staircase hexomino translated to `(x, y)`:
/// unit squares `(x,y)`, `(x,y+1)`, `(x+1,y+1)`, `(x+1,y+2)`,
/// `(x+2,y+1)`, `(x+2,y+2)`, emitted CCW with all 12 unit boundary
/// edges (four 180° collinear corners: `(x+2,y+1)`, `(x+3,y+2)`,
/// `(x+2,y+3)`, `(x,y+1)`).
fn stepped_hexomino(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y),
        Vec2::new(x + 1.0, y),
        Vec2::new(x + 1.0, y + 1.0),
        Vec2::new(x + 2.0, y + 1.0),
        Vec2::new(x + 3.0, y + 1.0),
        Vec2::new(x + 3.0, y + 2.0),
        Vec2::new(x + 3.0, y + 3.0),
        Vec2::new(x + 2.0, y + 3.0),
        Vec2::new(x + 1.0, y + 3.0),
        Vec2::new(x + 1.0, y + 2.0),
        Vec2::new(x, y + 2.0),
        Vec2::new(x, y + 1.0),
    ])
}

/// Stepped hexominoes — the asymmetric staircase hexomino on the
/// centered-rectangular lattice ⟨(3,1),(0,2)⟩ (p1).
pub fn stepped_hexominoes() -> LatticeTiling {
    // One hexomino; Λ = ⟨(3,1), (0,2)⟩ translates it into the full
    // tiling (each (3,1) step lands the next tile in the adjacent
    // width-3 column, each (0,2) step stacks within a column).
    let cells = vec![Cell {
        poly: stepped_hexomino(0.0, 0.0),
        seed: 0,
    }];

    LatticeTiling {
        id: "stepped-hexominoes",
        name: "Stepped hexominoes",
        description: "Asymmetric 2-step staircase hexomino on the centered rectangular lattice <(3,1),(0,2)> (primitive cell; centered cell 6x2 with center (-3,1) in the lattice). One hexomino per cell, edge-to-edge with 180 degrees at the four collinear corners. Wallpaper group p1: the centered-rectangular normalizer {I, R180, Mx, My} is broken by the asymmetric staircase — translations only.",
        lattice: [Vec2::new(3.0, 1.0), Vec2::new(0.0, 2.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P1,
        lattice_kind: LatticeKind::CenteredRectangular,
        category: TilingCategory::Wallpaper,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
