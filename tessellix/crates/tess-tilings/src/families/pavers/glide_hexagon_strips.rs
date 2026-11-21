//! Glide hexagon strips (M80) — the first **pg** wallpaper realizer in
//! the catalog, expressed as a [`LatticeTiling`] data table.
//!
//! # The pattern
//!
//! Two horizontal strips of height 1, each a periodic row of
//! interlocking hexagons, per 1×2 cell of the rectangular lattice
//! `Λ = ⟨(1,0), (0,2)⟩` (det 2). Strip 0 (`y ∈ [0,1]`) is the row
//! `A, B, A+(1,0), B+(1,0), …`; strip 1 (`y ∈ [1,2]`) is the row
//! `T, U, T+(1,0), U+(1,0), …`. The four cell tiles (CCW, areas sum
//! to det 2):
//!
//! * `A = (0,0), (1/2,0), (7/10,2/5), (1/2,1), (0,1), (−1/4,1/2)` —
//!   fat, strip 0, convex (area 29/40);
//! * `B = (1/2,0), (1,0), (3/4,1/2), (1,1), (1/2,1), (7/10,2/5)` —
//!   waist, strip 0, two reflex vertices `(3/4,1/2)` and
//!   `(7/10,2/5)` (area 11/40);
//! * `T = (−1/2,2), (−3/4,3/2), (−1/2,1), (0,1), (1/5,8/5), (0,2)` —
//!   fat, strip 1, convex (area 29/40);
//! * `U = (0,2), (1/5,8/5), (0,1), (1/2,1), (1/4,3/2), (1/2,2)` —
//!   waist, strip 1, two reflex vertices `(1/5,8/5)` and
//!   `(1/4,3/2)` (area 11/40).
//!
//! Strip 1 is the glide image of strip 0: with the glide reflection
//! `g(x,y) = (x + 1/2, −y)`, `T = g(A) + (−1,2)` and
//! `U = g(B) + (−1,2)`, and `g² = (1,0) ∈ Λ`. The strip-1 finger
//! therefore lands at `x = 1/5` — that x-position is *forced* by the
//! glide, `g(7/10,2/5) + (−1,2) = (1/5,8/5)`; a hand-placed finger at
//! `2/5` would break the glide and drop the group to p1.
//!
//! `A` and `T` are chiral twins (related by a reflection, not by any
//! direct isometry), as are `B` and `U`; the fat hexagons are
//! incongruent to the waist hexagons — areas 29/40 vs 11/40 and 0 vs
//! 2 reflex vertices. (The two classes have identical squared-edge
//! multisets; area and reflex count, not edge lengths, distinguish
//! them.)
//!
//! # Edge-to-edge note
//!
//! The partition is edge-to-edge: every edge is shared by exactly two
//! tiles and every vertex figure sums to 360°. The interlocking is
//! chain-exact:
//!
//! * `A`'s right chain `(1/2,0) → (7/10,2/5) → (1/2,1)` is exactly
//!   `B`'s left chain (the finger slides into `B`'s notch within one
//!   period);
//! * `B`'s right notch `(1,0) → (3/4,1/2) → (1,1)` is exactly the
//!   left chain of `A+(1,0)` (across the cell boundary);
//! * one strip up, the same story: `T`'s right chain
//!   `(0,1) → (1/5,8/5) → (0,2)` is `U`'s left chain and `U`'s right
//!   notch `(1/2,1) → (1/4,3/2) → (1/2,2)` is the left chain of
//!   `T+(1,0)`.
//!
//! Edge classes (exact lengths): horizontal half-unit edges on
//! `y ∈ ℤ` (length 1/2, pairing strip-0 tops with strip-1 bottoms
//! across `y = 1` and the cell boundaries `y = 0`, `y = 2`); finger
//! edges of slope `+2` and `−3` (lengths `√5/5` and `√10/5`);
//! chain edges of slope `±2` (length `√5/4`), the remaining links
//! of the interlocking chains. Every tile carries the squared-edge
//! multiset `{1/4, 1/4, 1/5, 2/5, 5/16, 5/16}` — edge lengths
//! alone do not separate the two shape classes. The
//! interface `y = 1` breaks at multiples of `1/2` from both sides
//! (strip-0 tops at `x ∈ {0, 1/2} + ℤ`, strip-1 bottoms at
//! `x ∈ {−1/2, 0} + ℤ`), so every interface vertex is a vertex of all
//! four incident tiles: no T-junctions. There are **no vertical
//! edges**; the five edge directions (mod 180°) are `0°`,
//! `arctan 2 = 63.4349°`, `arctan 3 = 71.5651°`,
//! `180° − arctan 2 = 116.5651°` and `180° − arctan 3 = 108.4349°`.
//!
//! # Vertex stars
//!
//! Four vertex types, all summing to 360° (exact census over a window
//! of 265 interior vertices):
//!
//! * finger tips `(7/10,2/5) + Λ` and `(1/5,8/5) + Λ` — a convex tip
//!   kissing a reflex notch, two tiles: `{225°, 135°}`;
//! * notch elbows `(3/4,1/2) + Λ` and `(1/4,3/2) + Λ` — a reflex
//!   notch tip meeting the adjacent fat hexagon's left-chain elbow,
//!   two tiles: `{233.1301°, 126.8699°}` =
//!   `{360° − arccos(−3/5), arccos(−3/5)}`;
//! * interface vertices `(0,1) + Λ` and `(1/2,1) + Λ`, four tiles:
//!   `{116.5651°, 108.4349°, 71.5651°, 63.4349°}` =
//!   `{180° − arctan 2, 180° − arctan 3, arctan 3, arctan 2}`;
//! * baseline vertices `(0,0) + Λ`, `(1/2,0) + Λ`, `(0,2) + Λ`,
//!   `(1/2,2) + Λ`, four tiles: `{116.5651° × 2, 63.4349° × 2}`.
//!
//! # Wallpaper group
//!
//! **pg** — translations plus ONE glide family, no mirrors, no
//! rotations, no perpendicular glides.
//!
//! * The glide `g(x,y) = (x + 1/2, −y)` (axis `y = 0`, glide
//!   distance 1/2) squares to the lattice translation `(1,0)`, and
//!   `g` maps the tile set onto itself: `g(A) = T + (1,−2)`,
//!   `g(B) = U + (1,−2)`, `g(T) = A + (0,−2)`, `g(U) = B + (0,−2)`.
//!   The composites `g ∘ (a, 2b)` are glides on the lines `y = b`
//!   (all with glide 1/2): the family `g · Λ` carries two parallel
//!   glide-line orbits, `y ∈ 2ℤ` and `y = 1 + 2ℤ` — e.g.
//!   `Φ = T_{(−1,2)} ∘ g = (x − 1/2, 2 − y)` is the glide on `y = 1`
//!   and `Φ² = (−1,0) ∈ Λ`. Both orbits come from the single family
//!   `g · Λ` (`Φ = g ∘ (−1,−2)`), so there is exactly one glide
//!   direction.
//! * No horizontal mirror or glide other than the family above:
//!   the complete 16-coset enumeration (in the probe test) shows
//!   the only positive coset of the linear part `(x, −y)` is the
//!   glide coset `(1/2, 0)` mod Λ. Sketch: `(x + s, 2c − y)` must
//!   send the finger tip `(7/10, 2/5)` to a vertex; the four
//!   vertex levels mod 1 carry x-positions `0: (1/2)ℤ`,
//!   `1/2: ±1/4 + ℤ` (line-dependent), `2/5: 7/10 + ℤ`,
//!   `3/5: 1/5 + ℤ`, and the only candidate surviving the finger
//!   tip is the half-glide `s ≡ 1/2, 2c ≡ 1` — killed by the
//!   elbow `(−1/4, 1/2) ↦ (1/4, 1/2)`, which is not a vertex
//!   (level `y = 1/2` carries strip-0 vertices only at
//!   `x ≡ 3/4 + ℤ`).
//! * No vertical mirror: `x = c` must preserve the half-unit edge
//!   pattern on the lines `y ∈ ℤ` (forcing `2c ∈ (1/2)ℤ`) and
//!   map the finger tip `(7/10, 2/5)` to a same-level vertex —
//!   level `2/5` carries vertices only at `x ≡ 7/10 + ℤ`, forcing
//!   `2c ≡ 7/5 (mod 1)`; but `7/5 ∉ (1/2)ℤ`. The exact probe
//!   kills all 16 candidate cosets.
//! * No half-turns: a center `(a, b)` with `b ≡ 0 (mod 1)` maps
//!   the finger tip `(7/10, 2/5)` to level `−2/5 ≡ 3/5`, which
//!   carries vertices only at `x ≡ 1/5 + ℤ`, forcing `2a ≡
//!   9/10 (mod 1)`, while the half-unit edge pattern on
//!   `y ∈ ℤ` forces `2a ∈ (1/2)ℤ`; `9/10 ∉ (1/2)ℤ`. Centers with
//!   `b ≡ 1/2 (mod 1)` give the identical `2a` constraint (level
//!   `3/5` again) and fail the same way. All 16 candidate cosets
//!   are killed by the exact probe.
//! * Diagonal mirrors/glides (`y = ±x + c`) and 90°/270° rotations:
//!   the linear parts do not normalize `Λ` (they send `(1,0)` to a
//!   non-lattice vector), so no such isometry acts at any center —
//!   independently, there are no vertical edges, so a 90° rotation
//!   could not even map the edge directions to themselves. The
//!   remaining point-symmetry candidates (120°/240°) do not
//!   normalize `Λ` either.
//!
//! The true translation subgroup is exactly `Λ`: the exact probe
//! shows the identity coset is the only positive translation coset,
//! so `g² = (1,0)` is a primitive translation, not a double one.
//!
//! # Bravais kind
//!
//! **Rectangular** (primitive): the declared basis `(1,0), (0,2)` is
//! an orthogonal pair of unequal length, and the lattice-legality test
//! pins the declaration to the measured kind.
//!
//! # Seeds and protos
//!
//! `proto` = cell index (`0` = `A`, `1` = `T`, `2` = `B`, `3` =
//! `U`); `seed` = shape class — two classes (0 = the fat hexagons
//! `A, T` and 1 = the waist hexagons `B, U`), so the Pattern
//! coloring distinguishes the two hexagon shapes.
//!
//! # Verification
//!
//! Verified exactly before porting (Python, stdlib,
//! `m80_verify.py`): the four cell polygons are CCW with no 180°
//! corners, areas 29/40, 11/40, 29/40, 11/40 summing to |det| = 2,
//! `A`/`T` convex and `B`/`U` with exactly 2 reflex vertices each;
//! the congruence census (`A ≅ T` and `B ≅ U` reflection-only
//! chiral pairs, all other pairs incongruent); the window
//! `W = [−2,4] × [−4,8]` partition (396 tiles, 0 crossings/overlaps,
//! all 352 interior edges paired exactly twice, no T-junctions among
//! 868 vertices, clipped coverage 72 = 72); every one of the 265
//! interior vertex figures sums to 360° (the four-star census above);
//! and the full probe battery — **74 positive** candidates (49
//! lattice translations `(a, 2b)`, `a, b ∈ [−3,3]`, and 25 glides
//! `(x + 1/2 + a, −y + 2b)`, `a, b ∈ [−2,2]`) against **62
//! exact-coset negative** candidates (15 off-lattice translation
//! cosets, 15 non-glide mirror/glide cosets, all 16 vertical
//! mirror/glide cosets, all 16 half-turn cosets) plus **6
//! non-normalizing linear parts** (diagonal and anti-diagonal
//! mirrors/glides, 90°/270°/120°/240° rotations) killed for *all*
//! centers by the lattice-normalizer argument (and sampled negative at
//! 6 centers each). Rust: the standard trio, the brute-force culling
//! cross-check (`lattice_culling_matches_brute_force`), the in-module
//! pg group probe (`glide_hexagon_strips_wallpaper_group_is_pg`), and
//! the lattice-legality test
//! (`declared_symmetry_is_legal_and_matches_the_measured_lattice`).

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

fn poly(pts: &[Vec2]) -> Polygon {
    Polygon::from_slice(pts)
}

/// The fat strip-0 hexagon `A` translated to `(x, y)`:
/// `(x, y), (x+1/2, y), (x+7/10, y+2/5), (x+1/2, y+1), (x, y+1),
/// (x−1/4, y+1/2)`, CCW, convex (area 29/40).
fn fat_strip0(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y),
        Vec2::new(x + 0.5, y),
        Vec2::new(x + 0.7, y + 0.4),
        Vec2::new(x + 0.5, y + 1.0),
        Vec2::new(x, y + 1.0),
        Vec2::new(x - 0.25, y + 0.5),
    ])
}

/// The waist strip-0 hexagon `B` translated to `(x, y)`:
/// `(x+1/2, y), (x+1, y), (x+3/4, y+1/2), (x+1, y+1), (x+1/2, y+1),
/// (x+7/10, y+2/5)`, CCW, two reflex vertices (area 11/40).
fn waist_strip0(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x + 0.5, y),
        Vec2::new(x + 1.0, y),
        Vec2::new(x + 0.75, y + 0.5),
        Vec2::new(x + 1.0, y + 1.0),
        Vec2::new(x + 0.5, y + 1.0),
        Vec2::new(x + 0.7, y + 0.4),
    ])
}

/// The fat strip-1 hexagon `T = g(A) + (−1, 2)` translated to
/// `(x, y)`: `(x−1/2, y+2), (x−3/4, y+3/2), (x−1/2, y+1), (x, y+1),
/// (x+1/5, y+8/5), (x, y+2)`, CCW, convex (area 29/40) — the chiral
/// twin of `A`.
fn fat_strip1(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x - 0.5, y + 2.0),
        Vec2::new(x - 0.75, y + 1.5),
        Vec2::new(x - 0.5, y + 1.0),
        Vec2::new(x, y + 1.0),
        Vec2::new(x + 0.2, y + 1.6),
        Vec2::new(x, y + 2.0),
    ])
}

/// The waist strip-1 hexagon `U = g(B) + (−1, 2)` translated to
/// `(x, y)`: `(x, y+2), (x+1/5, y+8/5), (x, y+1), (x+1/2, y+1),
/// (x+1/4, y+3/2), (x+1/2, y+2)`, CCW, two reflex vertices (area
/// 11/40) — the chiral twin of `B`.
fn waist_strip1(x: f64, y: f64) -> Polygon {
    poly(&[
        Vec2::new(x, y + 2.0),
        Vec2::new(x + 0.2, y + 1.6),
        Vec2::new(x, y + 1.0),
        Vec2::new(x + 0.5, y + 1.0),
        Vec2::new(x + 0.25, y + 1.5),
        Vec2::new(x + 0.5, y + 2.0),
    ])
}

/// Glide hexagon strips — two interlocking hexagon strips per 1×2 cell
/// on the rectangular lattice ⟨(1,0),(0,2)⟩ (pg).
pub fn glide_hexagon_strips() -> LatticeTiling {
    // Two horizontal hexagon strips per 1x2 cell. Strip 0 (y in [0,1])
    // is the row A (fat), B (waist), period (1,0); strip 1 (y in
    // [1,2]) is its glide image: T = g(A) + (-1,2), U = g(B) + (-1,2)
    // for the glide g(x, y) = (x + 1/2, -y), whose square is the
    // lattice translation (1,0). The strips interlock across the
    // interface y = 1: A's top edge (x in [0, 1/2]) pairs with U's
    // bottom edge, and B's top edge (x in [1/2, 1]) with T+(1,0)'s
    // bottom edge — the +1-shifted strip-1 tile — so the pairing runs
    // across cell boundaries, not just inside one cell.
    let cells = vec![
        Cell {
            poly: fat_strip0(0.0, 0.0),
            seed: 0,
        },
        Cell {
            poly: fat_strip1(0.0, 0.0),
            seed: 0,
        },
        Cell {
            poly: waist_strip0(0.0, 0.0),
            seed: 1,
        },
        Cell {
            poly: waist_strip1(0.0, 0.0),
            seed: 1,
        },
    ];

    LatticeTiling {
        id: "glide-hexagon-strips",
        name: "Glide hexagon strips",
        description: "Paver tiling: two interlocking hexagon strips per 1x2 rectangular cell <(1,0),(0,2)> (det 2): fat (area 29/40) and waist (area 11/40) hexagons, strip 1 the glide image of strip 0 under g(x,y) = (x+1/2,-y), g^2 = (1,0). Edge-to-edge (five edge directions, no verticals). The catalog's first pg realizer: one horizontal glide family (glide 1/2, axes y = integer, two parallel orbits y in 2Z and y = 1+2Z), no mirrors, no rotations, no perpendicular glides.",
        lattice: [Vec2::new(1.0, 0.0), Vec2::new(0.0, 2.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::Pg,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::Wallpaper,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
