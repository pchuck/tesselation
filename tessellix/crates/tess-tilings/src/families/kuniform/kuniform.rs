//! 2-uniform (k-uniform) tilings, expressed as [`LatticeTiling`] data
//! tables.
//!
//! Eighteen entries are from the closed list of 20 two-vertex-orbit
//! tilings by regular polygons (Krotenheerdt; see the Wikipedia
//! "k-uniform tilings" article — `2-uniform n10`, `n11`, `n18`, `n9`,
//! `n8`, `n5`, `n1`, `n13`, `n20`, `n12`, `n19`, `n4`, `n14`, `n3`,
//! `n15`, `n2`, `n16`, `n17`). The
//! list's entries have no individual proper names, so these are named by
//! their vertex configurations, following that convention:
//!
//! * `kuniform-p6m` — vertex figures 3⁶ ; 3².6², group p6m, t = 2,
//!   e = 3. The "1/3-dissected" honeycomb: the regular hexagonal tiling
//!   with every third hexagon (an index-3 sublattice of the hexagon
//!   centers) dissected into a fan of six unit triangles.
//! * `kuniform-pmm` — vertex figures 3.6.3.6 ; 3².6², group pmm,
//!   t = 2, e = 3. Flat-top unit hexagons in rectangular rows,
//!   adjacent hexagons sharing full horizontal edges, and every
//!   inter-hexagon rhombus split into two unit triangles.
//! * `kuniform-n18` — vertex figures 3⁶ ; 3².4.3.4, group p6m,
//!   t = 3, e = 3. Squares and two triangle orbits; the 3⁶ vertices are
//!   the lattice origin.
//! * `kuniform-n9` — vertex figures 3.4.6.4 ; 3².4.3.4, group p6m,
//!   t = 4, e = 4. Hexagons ringed by squares with triangle fills; the
//!   origin is a hexagon center.
//! * `kuniform-n8` — vertex figures 3.4.6.4 ; 3³.4², group p6m,
//!   t = 4, e = 4. The second p6m 3.4.6.4 form: hexagons ringed by
//!   squares with triangle fills; the origin is a hexagon center.
//! * `kuniform-n5` — vertex figures 3.4.6.4 ; 3.4².6, group p6m,
//!   t = 5, e = 5. Hexagons ringed by squares and triangles; the origin
//!   is a hexagon center, and the 6-fold center c is the center of the
//!   cell's single minority hexagon.
//! * `kuniform-n1` — vertex figures 4.6.12 ; 3.4.6.4, group p6m,
//!   t = 4, e = 4. The 3-4-6-12 tiling: dodecagons alternating with
//!   hexagons, squares and triangles; the origin is a hexagon center,
//!   and the 6-fold center c is the center of the cell's dodecagon.
//! * `kuniform-n13` — vertex figures 3⁶ ; 3².4.12, group p6m,
//!   t = 4, e = 4. A dodecagon ringed by squares and two triangle
//!   orbits around 3⁶ vertices; the origin is a 3⁶ vertex, and the
//!   6-fold center c is the center of the cell's dodecagon.
//! * `kuniform-n19` — vertex figures 3⁶ ; 3⁴.6, group p6m,
//!   t = 3, e = 3. The t = 3 flower form of the two 3⁴.6 entries:
//!   hexagon flowers on the index-9 sublattice ⟨(3,0),(0,3)⟩ of the
//!   unit triangular lattice. Both mirror classes verified exactly,
//!   so the wiki table's p6 label under-states the symmetry — n20
//!   below is the genuinely chiral p6 form of the same pair.
//! * `kuniform-n20` — vertex figures 3⁶ ; 3⁴.6, group p6 (the
//!   module's first chiral entry), t = 5, e = 6 (e = 7 in the
//!   Wikipedia parenthetical — the exact torus enumeration
//!   disagrees; see the function docs). Unit triangular lattice
//!   with hexagon flowers on an index-13 sublattice; the origin is
//!   a hexagon center and the 6-fold point.
//! * `kuniform-n12` — vertex figures 3².6² ; 3⁴.6, group cmm (the
//!   module's first cmm entry), t = 2, e = 4. Rows of edge-sharing
//!   hexagons with unit-triangle bands between the rows; the origin
//!   is a hexagon center and a 2-fold point, and the centered
//!   lattice's centering vector c = (√3/2, 5/2) is a basis vector.
//! * `kuniform-n4` — vertex figures 4⁴ ; 3³.4², group cmm, t = 2,
//!   e = 4. Staggered square bands (two squares tall) alternating with
//!   alternating-triangle bands one triangle high; the 4⁴ vertex is the
//!   interior corner of a square band crossing.
//! * `kuniform-n14` — vertex figures 3⁶ ; 3³.4², group pmm, t = 3,
//!   e = 4. Single square rows alternating with double triangle strips
//!   (two counterposed triangle rows); the origin is a 3³.4² vertex,
//!   the 3⁶ vertices sit at the strip-strip junction line.
//! * `kuniform-n3` — vertex figures 4⁴ ; 3³.4², group cmm, t = 3,
//!   e = 5. The n4 sibling: three-square-tall bands alternating with
//!   alternating-triangle bands; the interior 4⁴ crossings sit inside
//!   the three-row bands, two classes per cell.
//! * `kuniform-n15` — vertex figures 3⁶ ; 3³.4², group cmm, t = 4,
//!   e = 5. The n14 sibling: single square rows alternating with
//!   triple triangle strips (three rows, every horizontal edge shared
//!   back-to-back); the square row is flanked by one triangle orbit.
//! * `kuniform-n2` — vertex figures 3.12.12 ; 3.4.3.12, group p4m,
//!   t = 3, e = 3. The 3-4-3-12 tiling: unit dodecagons kissing flat-
//!   edge-to-flat-edge on a square lattice of spacing 2 + √3, with each
//!   four-dodecagon square hole filled by a unit square plus four edge
//!   triangles; the origin is the square's center.
//! * `kuniform-n16` — vertex figures 3³.4² ; 3².4.3.4, group p4g (the
//!   module's first p4g entry), t = 4, e = 5. Variant 1 of the
//!   33344-33434 tiling: staggered square rows and triangle bands on
//!   the diagonally-oriented square lattice ⟨(h,−h), (h,h)⟩,
//!   h = (3 + √3)/2, with the diagonal mirror classes true glides; the
//!   origin is a 3².4.3.4 vertex.
//! * `kuniform-n17` — vertex figures 3³.4² ; 3².4.3.4, group pgg (the
//!   module's first pgg entry), t = 3, e = 6. Variant 2 of the
//!   33344-33434 tiling, the pgg sibling of n16: stair-stepped
//!   square-and-triangle cells on the rectangular period lattice
//!   ⟨(1/2, 1+√3/2), (−2−√3, 1)⟩ (b1 ⊥ b2, |b2| = 2|b1|), with four
//!   2-fold centers per cell in two group orbits, one glide per axis
//!   and no reflections; the origin is a 3².4.3.4 vertex.
//!
//! All coordinates lie in Z + (√3/2)·Q, stored through [`H`].

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

const H: f64 = 0.8660254037844386; // √3/2
const S3: f64 = 2.0 * H; // √3

/// 2-uniform tiling (3⁶; 3².6²), wallpaper group p6m.
///
/// The regular hexagonal tiling with one of every three hexagons
/// dissected into a fan of six unit triangles (Wikipedia `2-uniform
/// n10`). Hexagon side 1; H = √3/2.
///
/// Hexagon centers form the honeycomb lattice
/// Λ = span{a1 = (3/2, H), a2 = (0, 2H)}; a hexagon at index (m, n)
/// is dissected exactly when m ≡ n (mod 3) — at every honeycomb
/// vertex exactly one of the three hexagons is dissected, so that
/// vertex sees two 120° hexagon corners and two 60° fan angles
/// (figure 3².6²), while the fan apexes are the only 3⁶ vertices.
/// The tiling's translation lattice is the index-3 sublattice L =
/// {m a1 + n a2 : m ≡ n (mod 3)}, generated by b1 = 2a1 − a2 =
/// (3, 0) and b2 = a1 + a2 = (3/2, 3H); |det| = 9√3/2 = two
/// hexagons (2·3√3/2) plus six triangles (6·√3/4).
///
/// One cell (the parallelogram [0,b1]×[0,b2], offset 0) holds two
/// intact hexagons centered at a1 and a2 and the six-triangle fan of
/// the dissected hexagon centered at the origin.
///
/// Pattern seeds by polygon class: hexagons seed 0, triangles seed 1.
pub fn kuniform_p6m() -> LatticeTiling {
    // Intact hexagon at a1 = (3/2, H).
    let hex_a = Polygon::from_slice(&[
        Vec2::new(2.5, H),
        Vec2::new(2.0, 2.0 * H),
        Vec2::new(1.0, 2.0 * H),
        Vec2::new(0.5, H),
        Vec2::new(1.0, 0.0),
        Vec2::new(2.0, 0.0),
    ]);
    // Intact hexagon at a2 = (0, 2H) = (0, √3).
    let hex_b = Polygon::from_slice(&[
        Vec2::new(1.0, 2.0 * H),
        Vec2::new(0.5, 3.0 * H),
        Vec2::new(-0.5, 3.0 * H),
        Vec2::new(-1.0, 2.0 * H),
        Vec2::new(-0.5, H),
        Vec2::new(0.5, H),
    ]);
    // Dissected hexagon at the origin: six unit-triangle fans
    // [(0,0), c_k, c_{k+1}], c_k the unit hexagon corners at 60k
    // degrees (CCW, so every fan is CCW).
    let c = [
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
        Vec2::new(-0.5, H),
        Vec2::new(-1.0, 0.0),
        Vec2::new(-0.5, -H),
        Vec2::new(0.5, -H),
    ];
    let mut cells = vec![
        Cell { poly: hex_a, seed: 0 },
        Cell { poly: hex_b, seed: 0 },
    ];
    for k in 0..6 {
        let fan = Polygon::from_slice(&[Vec2::ZERO, c[k], c[(k + 1) % 6]]);
        cells.push(Cell { poly: fan, seed: 1 });
    }
    LatticeTiling {
        id: "kuniform-p6m",
        name: "2-uniform (3⁶; 3².6²)",
        description: "2-uniform tiling (3⁶; 3².6²) — hexagonal tiling with one of every three hexagons dissected into six-triangle fans (p6m).",
        lattice: [Vec2::new(3.0, 0.0), Vec2::new(1.5, 3.0 * H)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (3.6.3.6; 3².6²), wallpaper group pmm.
///
/// Flat-top unit hexagons centered on the rectangular lattice
/// (2i, √3j) — hexagons in a row share full horizontal edges — with
/// the rhombus between two horizontally adjacent hexagons split into
/// two unit triangles (Wikipedia `2-uniform n11`). Hexagon side 1;
/// H = √3/2.
///
/// The primitive cell [(2,0), (0,√3)] (offset 0) holds one hexagon
/// (center (0,0)) and the two triangles of the rhombus between
/// (0,0) and (2,0): an apex-down one across the shared top gap and an
/// apex-up one (apex at (1, √3)) across the shared bottom edge. Area
/// 3√3/2 + 2·√3/4 = 2√3 = |det|. Hexagon-corner vertices (the
/// (2i+1, √3j) points) have figure 3.6.3.6; the triangle-base
/// vertices (the (2i+1/2, H+√3j) points) have figure 3².6² — the two
/// triangles share the edge (2i+1/2, H)–(2i+3/2, H), the two hexagons
/// the edge (2i−1/2, H)–(2i+1/2, H).
///
/// The group is pmm, not p6m: R60 sends b1 = (2,0) to (1, √3) =
/// b1/2 + b2, a non-lattice vector, so no 6-fold (and, by the same
/// lattice argument, no 9-fold or 4-fold) rotation can act.
///
/// Pattern seeds by polygon class: hexagons seed 0, triangles seed 1.
pub fn kuniform_pmm() -> LatticeTiling {
    // Unit flat-top hexagon centered at the origin.
    let hex = Polygon::from_slice(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
        Vec2::new(-0.5, H),
        Vec2::new(-1.0, 0.0),
        Vec2::new(-0.5, -H),
        Vec2::new(0.5, -H),
    ]);
    // Triangle between the hexagons at (0,0) and (2,0), apex down at
    // (1,0) (the shared vertex of the two hexagons), base across the
    // top gap. CCW.
    let t1 = Polygon::from_slice(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(1.5, H),
        Vec2::new(0.5, H),
    ]);
    // Mirror partner, apex up at (1, √3) on the shared edge below the
    // hexagon row above, same base edge (0.5,H)–(1.5,H). The vertex
    // order is CCW (the clockwise order would break sector-walk
    // figure computation).
    let t2 = Polygon::from_slice(&[
        Vec2::new(1.0, S3),
        Vec2::new(0.5, H),
        Vec2::new(1.5, H),
    ]);
    LatticeTiling {
        id: "kuniform-pmm",
        name: "2-uniform (3.6.3.6; 3².6²)",
        description: "2-uniform tiling (3.6.3.6; 3².6²) — hexagon rows with edge-to-edge triangle fills between hexagon pairs (pmm).",
        lattice: [Vec2::new(2.0, 0.0), Vec2::new(0.0, S3)],
        cells: vec![
            Cell { poly: hex, seed: 0 },
            Cell { poly: t1, seed: 1 },
            Cell { poly: t2, seed: 1 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::Pmm,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::TwoUniform,
        secondary: &[TilingCategory::Wallpaper],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (3⁶; 3².4.3.4), wallpaper group p6m.
///
/// Wikipedia "k-uniform tilings" list entry `2-uniform n18` (Krotenheerdt's
/// closed list of 20 two-uniform tilings by regular polygons): t = 3 tile
/// orbits (two triangle orbits + the squares), e = 3 edge orbits. H =
/// √3/2.
///
/// The translation lattice is a 60° hexagonal basis with |b1| = |b2| =
/// 1+√3: b1 = (0, 1+√3), b2 = ((3+√3)/2, (1+√3)/2); |det| = 3+2√3. The
/// origin is a 3⁶ vertex (six triangles around a point) — a genuine
/// 6-fold center of the tiling: the Python group determination verified
/// R60..R300 about the origin plus all six mirrors through it, so the
/// declared group is p6m.
///
/// The cell holds 11 tiles: 3 squares (seed 0, the largest class) and 8
/// equilateral triangles — 6 from the first triangle orbit (seed 1) and 2
/// from the second (seed 2). Area 3·1 + 8·(√3/4) = 3+2√3 = |det|.
pub fn kuniform_n18() -> LatticeTiling {
    // squares (seed 0, largest class — no hexagon)
    let mut cells: Vec<Cell> = vec![Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(5.0/2.0 + H, 5.0/2.0 + 5.0 * H),
            Vec2::new(2.0 + H, 5.0/2.0 + 4.0 * H),
            Vec2::new(2.0 + 2.0 * H, 2.0 + 4.0 * H),
            Vec2::new(5.0/2.0 + 2.0 * H, 2.0 + 5.0 * H),
        ]),
        seed: 0,
    }];

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(2.0 + H, 3.0/2.0 + 4.0 * H),
            Vec2::new(5.0/2.0 + H, 3.0/2.0 + 3.0 * H),
            Vec2::new(5.0/2.0 + 2.0 * H, 2.0 + 3.0 * H),
            Vec2::new(2.0 + 2.0 * H, 2.0 + 4.0 * H),
        ]),
        seed: 0,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(5.0/2.0 + 2.0 * H, 2.0 + 5.0 * H),
            Vec2::new(7.0/2.0 + 2.0 * H, 2.0 + 5.0 * H),
            Vec2::new(7.0/2.0 + 2.0 * H, 3.0 + 5.0 * H),
            Vec2::new(5.0/2.0 + 2.0 * H, 3.0 + 5.0 * H),
        ]),
        seed: 0,
    });

    // first triangle orbit (seed 1)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(7.0/2.0 + 2.0 * H, 3.0 + 5.0 * H),
            Vec2::new(3.0 + 2.0 * H, 3.0 + 6.0 * H),
            Vec2::new(5.0/2.0 + 2.0 * H, 3.0 + 5.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(2.0 + 2.0 * H, 2.0 + 4.0 * H),
            Vec2::new(3.0 + 2.0 * H, 2.0 + 4.0 * H),
            Vec2::new(5.0/2.0 + 2.0 * H, 2.0 + 5.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0 + 2.0 * H, 2.0 + 4.0 * H),
            Vec2::new(7.0/2.0 + 2.0 * H, 2.0 + 5.0 * H),
            Vec2::new(5.0/2.0 + 2.0 * H, 2.0 + 5.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(5.0/2.0 + 2.0 * H, 2.0 + 3.0 * H),
            Vec2::new(3.0 + 2.0 * H, 2.0 + 4.0 * H),
            Vec2::new(2.0 + 2.0 * H, 2.0 + 4.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0/2.0 + H, 3.0/2.0 + 3.0 * H),
            Vec2::new(5.0/2.0 + H, 3.0/2.0 + 3.0 * H),
            Vec2::new(2.0 + H, 3.0/2.0 + 4.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0/2.0 + H, 3.0/2.0 + 3.0 * H),
            Vec2::new(2.0 + H, 3.0/2.0 + 2.0 * H),
            Vec2::new(5.0/2.0 + H, 3.0/2.0 + 3.0 * H),
        ]),
        seed: 1,
    });

    // second triangle orbit (seed 2)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(5.0/2.0 + H, 5.0/2.0 + 5.0 * H),
            Vec2::new(5.0/2.0 + 2.0 * H, 2.0 + 5.0 * H),
            Vec2::new(5.0/2.0 + 2.0 * H, 3.0 + 5.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(2.0 + H, 5.0/2.0 + 4.0 * H),
            Vec2::new(2.0 + H, 3.0/2.0 + 4.0 * H),
            Vec2::new(2.0 + 2.0 * H, 2.0 + 4.0 * H),
        ]),
        seed: 2,
    });
    LatticeTiling {
        id: "kuniform-n18",
        name: "2-uniform (3⁶; 3².4.3.4)",
        description: "2-uniform tiling (3⁶; 3².4.3.4) — squares and two triangle orbits around 3⁶ vertices (p6m).",
        lattice: [Vec2::new(0.0, 1.0 + S3), Vec2::new((3.0 + S3) / 2.0, (1.0 + S3) / 2.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (3.4.6.4; 3².4.3.4), wallpaper group p6m.
///
/// Wikipedia "k-uniform tilings" list entry `2-uniform n9` (Krotenheerdt's
/// closed list of 20 two-uniform tilings by regular polygons): t = 4 tile
/// orbits (hexagon + squares + two triangle orbits), e = 4 edge orbits.
/// H = √3/2.
///
/// The translation lattice is a 60° hexagonal basis with |b1| = |b2| =
/// 2+√3: b1 = (−(2+√3), 0), b2 = (1+√3/2, −(3/2+√3)); |det| = 6+7√3/2
/// (left-handed basis, handled by the generator). The origin is the center
/// of the hexagon — a genuine 6-fold rotation center (R60..R300 and all
/// six mirrors through it verified numerically), so the declared group is
/// p6m.
///
/// The cell holds 15 tiles: 1 hexagon (seed 0, the largest class), 6
/// squares (seed 1), and 8 equilateral triangles — 6 from the first
/// triangle orbit (seed 2) and 2 from the second (seed 3). Area 3√3/2 +
/// 6·1 + 8·(√3/4) = 6+7√3/2 = |det|.
pub fn kuniform_n9() -> LatticeTiling {
    // hexagon (seed 0, largest class)
    let mut cells: Vec<Cell> = vec![Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-3.0/2.0 - H, -3.0/2.0 - H),
            Vec2::new(-2.0 - H, -3.0/2.0 - 2.0 * H),
            Vec2::new(-3.0/2.0 - H, -3.0/2.0 - 3.0 * H),
            Vec2::new(-1.0/2.0 - H, -3.0/2.0 - 3.0 * H),
            Vec2::new(-H, -3.0/2.0 - 2.0 * H),
            Vec2::new(-1.0/2.0 - H, -3.0/2.0 - H),
        ]),
        seed: 0,
    }];

    // squares (seed 1)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-3.0/2.0 - H, -3.0/2.0 - 3.0 * H),
            Vec2::new(-2.0 - H, -3.0/2.0 - 2.0 * H),
            Vec2::new(-2.0 - 2.0 * H, -2.0 - 2.0 * H),
            Vec2::new(-3.0/2.0 - 2.0 * H, -2.0 - 3.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-3.0/2.0 - 2.0 * H, -1.0 - H),
            Vec2::new(-2.0 - 2.0 * H, -1.0 - 2.0 * H),
            Vec2::new(-2.0 - H, -3.0/2.0 - 2.0 * H),
            Vec2::new(-3.0/2.0 - H, -3.0/2.0 - H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0/2.0 - H, -5.0/2.0 - 3.0 * H),
            Vec2::new(-1.0/2.0 - H, -3.0/2.0 - 3.0 * H),
            Vec2::new(-3.0/2.0 - H, -3.0/2.0 - 3.0 * H),
            Vec2::new(-3.0/2.0 - H, -5.0/2.0 - 3.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-3.0/2.0 - H, -1.0/2.0 - H),
            Vec2::new(-3.0/2.0 - H, -3.0/2.0 - H),
            Vec2::new(-1.0/2.0 - H, -3.0/2.0 - H),
            Vec2::new(-1.0/2.0 - H, -1.0/2.0 - H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0/2.0 - H, -3.0/2.0 - 3.0 * H),
            Vec2::new(-1.0/2.0, -2.0 - 3.0 * H),
            Vec2::new(0.0, -2.0 - 2.0 * H),
            Vec2::new(-H, -3.0/2.0 - 2.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0/2.0 - H, -3.0/2.0 - H),
            Vec2::new(-H, -3.0/2.0 - 2.0 * H),
            Vec2::new(0.0, -1.0 - 2.0 * H),
            Vec2::new(-1.0/2.0, -1.0 - H),
        ]),
        seed: 1,
    });

    // first triangle orbit (seed 2)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-2.0 - 2.0 * H, -1.0 - 2.0 * H),
            Vec2::new(-2.0 - 2.0 * H, -2.0 - 2.0 * H),
            Vec2::new(-2.0 - H, -3.0/2.0 - 2.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-3.0/2.0 - H, -5.0/2.0 - 3.0 * H),
            Vec2::new(-3.0/2.0 - H, -3.0/2.0 - 3.0 * H),
            Vec2::new(-3.0/2.0 - 2.0 * H, -2.0 - 3.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0/2.0 - H, -5.0/2.0 - 3.0 * H),
            Vec2::new(-1.0/2.0, -2.0 - 3.0 * H),
            Vec2::new(-1.0/2.0 - H, -3.0/2.0 - 3.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0/2.0 - H, -5.0/2.0 - 3.0 * H),
            Vec2::new(-1.0/2.0, -3.0 - 3.0 * H),
            Vec2::new(-1.0/2.0, -2.0 - 3.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0/2.0 - H, -1.0/2.0 - H),
            Vec2::new(-1.0/2.0 - H, -3.0/2.0 - H),
            Vec2::new(-1.0/2.0, -1.0 - H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-H, -3.0/2.0 - 2.0 * H),
            Vec2::new(0.0, -2.0 - 2.0 * H),
            Vec2::new(0.0, -1.0 - 2.0 * H),
        ]),
        seed: 2,
    });

    // second triangle orbit (seed 3)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0/2.0 - H, -5.0/2.0 - 3.0 * H),
            Vec2::new(-3.0/2.0 - H, -5.0/2.0 - 3.0 * H),
            Vec2::new(-1.0 - H, -5.0/2.0 - 4.0 * H),
        ]),
        seed: 3,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0/2.0, -2.0 - 3.0 * H),
            Vec2::new(1.0/2.0, -2.0 - 3.0 * H),
            Vec2::new(0.0, -2.0 - 2.0 * H),
        ]),
        seed: 3,
    });
    LatticeTiling {
        id: "kuniform-n9",
        name: "2-uniform (3.4.6.4; 3².4.3.4)",
        description: "2-uniform tiling (3.4.6.4; 3².4.3.4) — hexagons ringed by squares with triangle fills (p6m).",
        lattice: [Vec2::new(-(2.0 + S3), 0.0), Vec2::new(1.0 + S3 / 2.0, -1.5 - S3)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (3.4.6.4; 3³.4²), wallpaper group p6m.
///
/// Wikipedia "k-uniform tilings" list entry `2-uniform n8` (Krotenheerdt's
/// closed list of 20 two-uniform tilings by regular polygons): t = 4 tile
/// orbits (hexagon + squares + two triangle orbits), e = 4 edge orbits.
/// H = √3/2.
///
/// The translation lattice is a 60° hexagonal basis with |b1| = |b2| =
/// 2+√3: b1 = (0, 2+√3), b2 = (−(3/2+√3), −(1+√3/2)); |det| = 6+7√3/2
/// (left-handed basis, handled by the generator). The origin is the center
/// of the hexagon — a genuine 6-fold rotation center (R60..R300 and all
/// six mirrors through it verified numerically), so the declared group is
/// p6m.
///
/// The cell holds 15 tiles: 1 hexagon (seed 0, the largest class), 6
/// squares (seed 1), and 8 equilateral triangles — 6 from the first
/// triangle orbit (seed 2) and 2 from the second (seed 3). Area 3√3/2 +
/// 6·1 + 8·(√3/4) = 6+7√3/2 = |det|.
pub fn kuniform_n8() -> LatticeTiling {
    // hexagon (seed 0, largest class)
    let mut cells: Vec<Cell> = vec![Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0 - 2.0 * H, 1.0 + 2.0 * H),
            Vec2::new(-2.0 - 2.0 * H, 1.0 + 2.0 * H),
            Vec2::new(-5.0/2.0 - 2.0 * H, 1.0 + H),
            Vec2::new(-2.0 - 2.0 * H, 1.0),
            Vec2::new(-1.0 - 2.0 * H, 1.0),
            Vec2::new(-1.0/2.0 - 2.0 * H, 1.0 + H),
        ]),
        seed: 0,
    }];

    // squares (seed 1)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-5.0/2.0 - 2.0 * H, 1.0 + H),
            Vec2::new(-5.0/2.0 - 3.0 * H, 1.0/2.0 + H),
            Vec2::new(-2.0 - 3.0 * H, 1.0/2.0),
            Vec2::new(-2.0 - 2.0 * H, 1.0),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-2.0 - 3.0 * H, 3.0/2.0 + 2.0 * H),
            Vec2::new(-5.0/2.0 - 3.0 * H, 3.0/2.0 + H),
            Vec2::new(-5.0/2.0 - 2.0 * H, 1.0 + H),
            Vec2::new(-2.0 - 2.0 * H, 1.0 + 2.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0 - 2.0 * H, 2.0 + 2.0 * H),
            Vec2::new(-2.0 - 2.0 * H, 2.0 + 2.0 * H),
            Vec2::new(-2.0 - 2.0 * H, 1.0 + 2.0 * H),
            Vec2::new(-1.0 - 2.0 * H, 1.0 + 2.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0 - 2.0 * H, 3.0 + 2.0 * H),
            Vec2::new(-2.0 - 2.0 * H, 3.0 + 2.0 * H),
            Vec2::new(-2.0 - 2.0 * H, 2.0 + 2.0 * H),
            Vec2::new(-1.0 - 2.0 * H, 2.0 + 2.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0/2.0 - H, 3.0/2.0 + H),
            Vec2::new(-1.0 - H, 3.0/2.0 + 2.0 * H),
            Vec2::new(-1.0 - 2.0 * H, 1.0 + 2.0 * H),
            Vec2::new(-1.0/2.0 - 2.0 * H, 1.0 + H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0/2.0 - H, 5.0/2.0 + 3.0 * H),
            Vec2::new(-1.0/2.0 - 2.0 * H, 3.0 + 3.0 * H),
            Vec2::new(-1.0 - 2.0 * H, 3.0 + 2.0 * H),
            Vec2::new(-1.0 - H, 5.0/2.0 + 2.0 * H),
        ]),
        seed: 1,
    });

    // first triangle orbit (seed 2)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-2.0 - 3.0 * H, 5.0/2.0 + 2.0 * H),
            Vec2::new(-2.0 - 4.0 * H, 2.0 + 2.0 * H),
            Vec2::new(-2.0 - 3.0 * H, 3.0/2.0 + 2.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-5.0/2.0 - 3.0 * H, 3.0/2.0 + H),
            Vec2::new(-5.0/2.0 - 3.0 * H, 1.0/2.0 + H),
            Vec2::new(-5.0/2.0 - 2.0 * H, 1.0 + H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-2.0 - 2.0 * H, 2.0 + 2.0 * H),
            Vec2::new(-2.0 - 3.0 * H, 3.0/2.0 + 2.0 * H),
            Vec2::new(-2.0 - 2.0 * H, 1.0 + 2.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-2.0 - 2.0 * H, 3.0 + 2.0 * H),
            Vec2::new(-2.0 - 3.0 * H, 5.0/2.0 + 2.0 * H),
            Vec2::new(-2.0 - 2.0 * H, 2.0 + 2.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0 - 2.0 * H, 1.0 + 2.0 * H),
            Vec2::new(-1.0 - H, 3.0/2.0 + 2.0 * H),
            Vec2::new(-1.0 - 2.0 * H, 2.0 + 2.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0 - 2.0 * H, 2.0 + 2.0 * H),
            Vec2::new(-1.0 - H, 5.0/2.0 + 2.0 * H),
            Vec2::new(-1.0 - 2.0 * H, 3.0 + 2.0 * H),
        ]),
        seed: 2,
    });

    // second triangle orbit (seed 3)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-2.0 - 3.0 * H, 3.0/2.0 + 2.0 * H),
            Vec2::new(-2.0 - 2.0 * H, 2.0 + 2.0 * H),
            Vec2::new(-2.0 - 3.0 * H, 5.0/2.0 + 2.0 * H),
        ]),
        seed: 3,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(-1.0 - H, 5.0/2.0 + 2.0 * H),
            Vec2::new(-1.0 - 2.0 * H, 2.0 + 2.0 * H),
            Vec2::new(-1.0 - H, 3.0/2.0 + 2.0 * H),
        ]),
        seed: 3,
    });
    LatticeTiling {
        id: "kuniform-n8",
        name: "2-uniform (3.4.6.4; 3³.4²)",
        description: "2-uniform tiling (3.4.6.4; 3³.4²) — hexagons ringed by squares with triangle fills, the second p6m 3.4.6.4 form.",
        lattice: [Vec2::new(0.0, 2.0 + S3), Vec2::new(-1.5 - S3, -1.0 - S3 / 2.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
/// 2-uniform tiling (3.4.6.4; 3.4².6), wallpaper group p6m.
///
/// Wikipedia "k-uniform tilings" list entry `2-uniform n5` (Krotenheerdt's
/// closed list of 20 two-uniform tilings by regular polygons): t = 5 tile
/// orbits, e = 5 edge orbits. H = √3/2.
///
/// The translation lattice is a 60° hexagonal basis with |b1| = |b2| =
/// 3+√3: b1 = (3+√3, 0), b2 = (−(3+√3)/2, 3(1+√3)/2); |det| = 9+6√3
/// (right-handed). The origin is a hexagon face center, and the 6-fold
/// center c is the center of the cell's lone #00ff00 hexagon (O-frame
/// (4.732051, 5.464102), the exact O-relative tuple (6,2,4,4)) — R60..R300
/// about c plus all six mirrors through it verified numerically (the
/// Python battery check 5 + m90x_sym_n5.json), so the declared group is
/// p6m.
///
/// The cell holds 18 tiles: 3 hexagons (2 #008000 seed 0, the largest
/// class, 1 #00ff00 seed 1), 9 squares (6 #ff0000 seed 2, 3 #ff8989
/// seed 3), and 6 equilateral triangles (seed 4). Area 3·(3√3/2) + 9·1 +
/// 6·(√3/4) = 9+6√3 = |det|.
///
/// Vertex figures: two vertex orbits, three figure strings — the 3.4.6.4
/// orbit is achiral (figure "3464") while the 3.4.4.6 orbit is CHIRAL:
/// its figure "3446" differs from the mirror orbit's "3644" (100%
/// bidirectional mirror pairing, m90x_dbg7.py). Counts over the ±3×±3
/// patch: 3446 ×266, 3464 ×267, 3644 ×267.
pub fn kuniform_n5() -> LatticeTiling {
    // hexagons, first orbit (seed 0, largest class — 2 of the 3 cell hexagons)
    let mut cells: Vec<Cell> = vec![Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(1.0 + H, 3.0/2.0 + 2.0 * H),
            Vec2::new(2.0 + H, 3.0/2.0 + 2.0 * H),
            Vec2::new(5.0/2.0 + H, 3.0/2.0 + 3.0 * H),
            Vec2::new(2.0 + H, 3.0/2.0 + 4.0 * H),
            Vec2::new(1.0 + H, 3.0/2.0 + 4.0 * H),
            Vec2::new(1.0/2.0 + H, 3.0/2.0 + 3.0 * H),
        ]),
        seed: 0,
    }];

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(1.0 + H, 5.0/2.0 + 4.0 * H),
            Vec2::new(2.0 + H, 5.0/2.0 + 4.0 * H),
            Vec2::new(5.0/2.0 + H, 5.0/2.0 + 5.0 * H),
            Vec2::new(2.0 + H, 5.0/2.0 + 6.0 * H),
            Vec2::new(1.0 + H, 5.0/2.0 + 6.0 * H),
            Vec2::new(1.0/2.0 + H, 5.0/2.0 + 5.0 * H),
        ]),
        seed: 0,
    });

    // hexagons, second orbit (seed 1)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0 + 2.0 * H, 1.0 + 4.0 * H),
            Vec2::new(3.0 + 3.0 * H, 3.0/2.0 + 4.0 * H),
            Vec2::new(3.0 + 3.0 * H, 5.0/2.0 + 4.0 * H),
            Vec2::new(3.0 + 2.0 * H, 3.0 + 4.0 * H),
            Vec2::new(3.0 + H, 5.0/2.0 + 4.0 * H),
            Vec2::new(3.0 + H, 3.0/2.0 + 4.0 * H),
        ]),
        seed: 1,
    });

    // squares, first orbit (seed 2)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(H, 5.0/2.0 + 4.0 * H),
            Vec2::new(1.0/2.0 + H, 5.0/2.0 + 5.0 * H),
            Vec2::new(1.0/2.0, 3.0 + 5.0 * H),
            Vec2::new(0.0, 3.0 + 4.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(1.0 + H, 5.0/2.0 + 4.0 * H),
            Vec2::new(H, 5.0/2.0 + 4.0 * H),
            Vec2::new(H, 3.0/2.0 + 4.0 * H),
            Vec2::new(1.0 + H, 3.0/2.0 + 4.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0 + H, 3.0/2.0 + 4.0 * H),
            Vec2::new(3.0 + H, 5.0/2.0 + 4.0 * H),
            Vec2::new(2.0 + H, 5.0/2.0 + 4.0 * H),
            Vec2::new(2.0 + H, 3.0/2.0 + 4.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(5.0/2.0 + 2.0 * H, 1.0 + 3.0 * H),
            Vec2::new(3.0 + 2.0 * H, 1.0 + 4.0 * H),
            Vec2::new(3.0 + H, 3.0/2.0 + 4.0 * H),
            Vec2::new(5.0/2.0 + H, 3.0/2.0 + 3.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0 + 2.0 * H, 3.0 + 4.0 * H),
            Vec2::new(5.0/2.0 + 2.0 * H, 3.0 + 5.0 * H),
            Vec2::new(5.0/2.0 + H, 5.0/2.0 + 5.0 * H),
            Vec2::new(3.0 + H, 5.0/2.0 + 4.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(7.0/2.0 + 3.0 * H, 3.0/2.0 + 3.0 * H),
            Vec2::new(3.0 + 3.0 * H, 3.0/2.0 + 4.0 * H),
            Vec2::new(3.0 + 2.0 * H, 1.0 + 4.0 * H),
            Vec2::new(7.0/2.0 + 2.0 * H, 1.0 + 3.0 * H),
        ]),
        seed: 2,
    });

    // squares, second orbit (seed 3)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(2.0 + H, 3.0/2.0 + 4.0 * H),
            Vec2::new(2.0 + H, 5.0/2.0 + 4.0 * H),
            Vec2::new(1.0 + H, 5.0/2.0 + 4.0 * H),
            Vec2::new(1.0 + H, 3.0/2.0 + 4.0 * H),
        ]),
        seed: 3,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(2.0 + 2.0 * H, 1.0 + 2.0 * H),
            Vec2::new(5.0/2.0 + 2.0 * H, 1.0 + 3.0 * H),
            Vec2::new(5.0/2.0 + H, 3.0/2.0 + 3.0 * H),
            Vec2::new(2.0 + H, 3.0/2.0 + 2.0 * H),
        ]),
        seed: 3,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(4.0 + 3.0 * H, 3.0/2.0 + 2.0 * H),
            Vec2::new(7.0/2.0 + 3.0 * H, 3.0/2.0 + 3.0 * H),
            Vec2::new(7.0/2.0 + 2.0 * H, 1.0 + 3.0 * H),
            Vec2::new(4.0 + 2.0 * H, 1.0 + 2.0 * H),
        ]),
        seed: 3,
    });

    // triangles (seed 4)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(1.0 + H, 3.0/2.0 + 4.0 * H),
            Vec2::new(H, 3.0/2.0 + 4.0 * H),
            Vec2::new(1.0/2.0 + H, 3.0/2.0 + 3.0 * H),
        ]),
        seed: 4,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(1.0 + H, 5.0/2.0 + 4.0 * H),
            Vec2::new(1.0/2.0 + H, 5.0/2.0 + 5.0 * H),
            Vec2::new(H, 5.0/2.0 + 4.0 * H),
        ]),
        seed: 4,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0/2.0 + H, 3.0/2.0 + H),
            Vec2::new(2.0 + H, 3.0/2.0 + 2.0 * H),
            Vec2::new(1.0 + H, 3.0/2.0 + 2.0 * H),
        ]),
        seed: 4,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0 + H, 3.0/2.0 + 4.0 * H),
            Vec2::new(2.0 + H, 3.0/2.0 + 4.0 * H),
            Vec2::new(5.0/2.0 + H, 3.0/2.0 + 3.0 * H),
        ]),
        seed: 4,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(5.0/2.0 + H, 5.0/2.0 + 5.0 * H),
            Vec2::new(2.0 + H, 5.0/2.0 + 4.0 * H),
            Vec2::new(3.0 + H, 5.0/2.0 + 4.0 * H),
        ]),
        seed: 4,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0 + 2.0 * H, 1.0 + 4.0 * H),
            Vec2::new(5.0/2.0 + 2.0 * H, 1.0 + 3.0 * H),
            Vec2::new(7.0/2.0 + 2.0 * H, 1.0 + 3.0 * H),
        ]),
        seed: 4,
    });
    LatticeTiling {
        id: "kuniform-n5",
        name: "2-uniform (3.4.6.4; 3.4².6)",
        description: "2-uniform tiling (3.4.6.4; 3.4².6) — hexagons ringed by squares with triangle fills, the p6m n5 form (p6m).",
        lattice: [Vec2::new(3.0 + 2.0 * H, 0.0), Vec2::new(-3.0/2.0 - H, 3.0/2.0 + 3.0 * H)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (4.6.12; 3.4.6.4), wallpaper group p6m.
///
/// Wikipedia "k-uniform tilings" list entry `2-uniform n1` (Krotenheerdt's
/// closed list of 20 two-uniform tilings by regular polygons; the
/// 3-4-6-12 tiling): t = 4 tile orbits, e = 4 edge orbits. H = √3/2.
///
/// The translation lattice is a 60° hexagonal basis with |b1| = |b2| =
/// 2+2√3: b1 = (2+2√3, 0), b2 = (1+√3/2, 3+√3/2); |det| = 12+8√3
/// (right-handed). The origin is a hexagon face center; the 6-fold center
/// c is the center of the cell dodecagon (O-frame (4.098076, 2.366025),
/// the exact O-relative tuple (3,3,3,1)) — R60..R300 about c plus all six
/// mirrors through it verified numerically (the Python battery check 5 +
/// m90x_sym_n1.json), so the declared group is p6m.
///
/// The cell holds 12 tiles: 1 dodecagon (seed 0, the largest class),
/// 3 hexagons (seed 1), 6 squares (seed 2), and 2 equilateral triangles
/// (seed 3). Area (6+3√3) + 3·(3√3/2) + 6·1 + 2·(√3/4) = 12+8√3 = |det|.
///
/// Vertex figures: two vertex orbits, three figure strings — the 4.6.12
/// orbit is CHIRAL: its figure "4612" differs from the mirror orbit's
/// "4126" (100% bidirectional mirror pairing, m90x_dbg7.py; the ring
/// labels there are the string-concatenation "1246"/"1264" of the same
/// orbits), while the 3.4.6.4 orbit ("3464") is achiral. Counts over the
/// ±3×±3 patch: 4126 ×266, 4612 ×266, 3464 ×267.
pub fn kuniform_n1() -> LatticeTiling {
    // dodecagon (seed 0, largest class)
    let mut cells: Vec<Cell> = vec![Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(1.0 + 2.0 * H, 2.0 + 2.0 * H),
            Vec2::new(1.0/2.0 + 2.0 * H, 2.0 + H),
            Vec2::new(1.0/2.0 + 2.0 * H, 1.0 + H),
            Vec2::new(1.0 + 2.0 * H, 1.0),
            Vec2::new(1.0 + 3.0 * H, 1.0/2.0),
            Vec2::new(2.0 + 3.0 * H, 1.0/2.0),
            Vec2::new(2.0 + 4.0 * H, 1.0),
            Vec2::new(5.0/2.0 + 4.0 * H, 1.0 + H),
            Vec2::new(5.0/2.0 + 4.0 * H, 2.0 + H),
            Vec2::new(2.0 + 4.0 * H, 2.0 + 2.0 * H),
            Vec2::new(2.0 + 3.0 * H, 5.0/2.0 + 2.0 * H),
            Vec2::new(1.0 + 3.0 * H, 5.0/2.0 + 2.0 * H),
        ]),
        seed: 0,
    }];

    // hexagons (seed 1)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(2.0 + 4.0 * H, 2.0 + 2.0 * H),
            Vec2::new(2.0 + 5.0 * H, 5.0/2.0 + 2.0 * H),
            Vec2::new(2.0 + 5.0 * H, 7.0/2.0 + 2.0 * H),
            Vec2::new(2.0 + 4.0 * H, 4.0 + 2.0 * H),
            Vec2::new(2.0 + 3.0 * H, 7.0/2.0 + 2.0 * H),
            Vec2::new(2.0 + 3.0 * H, 5.0/2.0 + 2.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(5.0/2.0 + 4.0 * H, 1.0 + H),
            Vec2::new(5.0/2.0 + 5.0 * H, 1.0/2.0 + H),
            Vec2::new(5.0/2.0 + 6.0 * H, 1.0 + H),
            Vec2::new(5.0/2.0 + 6.0 * H, 2.0 + H),
            Vec2::new(5.0/2.0 + 5.0 * H, 5.0/2.0 + H),
            Vec2::new(5.0/2.0 + 4.0 * H, 2.0 + H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0 + 6.0 * H, 2.0 + 2.0 * H),
            Vec2::new(3.0 + 7.0 * H, 5.0/2.0 + 2.0 * H),
            Vec2::new(3.0 + 7.0 * H, 7.0/2.0 + 2.0 * H),
            Vec2::new(3.0 + 6.0 * H, 4.0 + 2.0 * H),
            Vec2::new(3.0 + 5.0 * H, 7.0/2.0 + 2.0 * H),
            Vec2::new(3.0 + 5.0 * H, 5.0/2.0 + 2.0 * H),
        ]),
        seed: 1,
    });

    // squares (seed 2)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(1.0/2.0 + H, 1.0/2.0 + H),
            Vec2::new(1.0 + H, 1.0/2.0),
            Vec2::new(1.0 + 2.0 * H, 1.0),
            Vec2::new(1.0/2.0 + 2.0 * H, 1.0 + H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(2.0 + 3.0 * H, 5.0/2.0 + 2.0 * H),
            Vec2::new(2.0 + 3.0 * H, 7.0/2.0 + 2.0 * H),
            Vec2::new(1.0 + 3.0 * H, 7.0/2.0 + 2.0 * H),
            Vec2::new(1.0 + 3.0 * H, 5.0/2.0 + 2.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(2.0 + 4.0 * H, 1.0),
            Vec2::new(2.0 + 5.0 * H, 1.0/2.0),
            Vec2::new(5.0/2.0 + 5.0 * H, 1.0/2.0 + H),
            Vec2::new(5.0/2.0 + 4.0 * H, 1.0 + H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(5.0/2.0 + 4.0 * H, 2.0 + H),
            Vec2::new(5.0/2.0 + 5.0 * H, 5.0/2.0 + H),
            Vec2::new(2.0 + 5.0 * H, 5.0/2.0 + 2.0 * H),
            Vec2::new(2.0 + 4.0 * H, 2.0 + 2.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0 + 5.0 * H, 5.0/2.0 + 2.0 * H),
            Vec2::new(3.0 + 5.0 * H, 7.0/2.0 + 2.0 * H),
            Vec2::new(2.0 + 5.0 * H, 7.0/2.0 + 2.0 * H),
            Vec2::new(2.0 + 5.0 * H, 5.0/2.0 + 2.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(5.0/2.0 + 6.0 * H, 2.0 + H),
            Vec2::new(3.0 + 6.0 * H, 2.0 + 2.0 * H),
            Vec2::new(3.0 + 5.0 * H, 5.0/2.0 + 2.0 * H),
            Vec2::new(5.0/2.0 + 5.0 * H, 5.0/2.0 + H),
        ]),
        seed: 2,
    });

    // triangles (seed 3)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(H, 1.0/2.0),
            Vec2::new(1.0 + H, 1.0/2.0),
            Vec2::new(1.0/2.0 + H, 1.0/2.0 + H),
        ]),
        seed: 3,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(5.0/2.0 + 5.0 * H, 5.0/2.0 + H),
            Vec2::new(3.0 + 5.0 * H, 5.0/2.0 + 2.0 * H),
            Vec2::new(2.0 + 5.0 * H, 5.0/2.0 + 2.0 * H),
        ]),
        seed: 3,
    });
    LatticeTiling {
        id: "kuniform-n1",
        name: "2-uniform (4.6.12; 3.4.6.4)",
        description: "2-uniform tiling (4.6.12; 3.4.6.4) — dodecagons alternating with hexagons, squares and triangles, the p6m 3-4-6-12 form (p6m).",
        lattice: [Vec2::new(2.0 + 4.0 * H, 0.0), Vec2::new(1.0 + 2.0 * H, 3.0 + 2.0 * H)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (3⁶; 3².4.12), wallpaper group p6m.
///
/// Wikipedia "k-uniform tilings" list entry `2-uniform n13` (Krotenheerdt's
/// closed list of 20 two-uniform tilings by regular polygons): t = 4 tile
/// orbits, e = 4 edge orbits. H = √3/2.
///
/// The translation lattice is a 60° hexagonal basis with |b1| = |b2| =
/// 3+√3: b1 = ((3+3√3)/2, (3+√3)/2), b2 = (0, 3+√3); |det| = 9+6√3
/// (right-handed). The origin is a 3⁶ vertex; the 6-fold center c is the
/// center of the cell dodecagon (O-frame (6.830127, 11.830127) — off-ring:
/// the dodecagon-center radius (√6+√2)/2 ∉ Q(√3), float only). R60..R300
/// about c plus all six mirrors through it verified numerically (the Python
/// battery check 5 + m90x_sym_n13.json), so the declared group is p6m.
///
/// The cell holds 16 tiles: 1 dodecagon (seed 0, the largest class),
/// 3 squares (seed 1), and 12 equilateral triangles — 6 from the first
/// triangle orbit (seed 2, #808000) and 6 from the second (seed 3,
/// #ffff00; the two orbits tie at 6/6 per cell, so the seed order is the
/// documented first-appearance order in the cell). Area (6+3√3) + 3·1 +
/// 12·(√3/4) = 9+6√3 = |det|.
///
/// Vertex figures: two vertex orbits, three figure strings — the 3².4.12
/// orbit is CHIRAL: its figure "33412" differs from the mirror orbit's
/// "33124" (100% bidirectional mirror pairing, m90x_dbg7.py; the ring
/// labels there are the string-concatenation "12334"/"12433" of the same
/// orbits), while the 3⁶ orbit ("333333", the 6-fold vertices) is achiral.
/// Counts over the ±3×±3 patch: 33412 ×241, 33124 ×240, 333333 ×84.
pub fn kuniform_n13() -> LatticeTiling {
    // dodecagon (seed 0, largest class)
    let mut cells: Vec<Cell> = vec![Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0 + 5.0 * H, 13.0/2.0 + 4.0 * H),
            Vec2::new(3.0 + 6.0 * H, 7.0 + 4.0 * H),
            Vec2::new(7.0/2.0 + 6.0 * H, 7.0 + 5.0 * H),
            Vec2::new(7.0/2.0 + 6.0 * H, 8.0 + 5.0 * H),
            Vec2::new(3.0 + 6.0 * H, 8.0 + 6.0 * H),
            Vec2::new(3.0 + 5.0 * H, 17.0/2.0 + 6.0 * H),
            Vec2::new(2.0 + 5.0 * H, 17.0/2.0 + 6.0 * H),
            Vec2::new(2.0 + 4.0 * H, 8.0 + 6.0 * H),
            Vec2::new(3.0/2.0 + 4.0 * H, 8.0 + 5.0 * H),
            Vec2::new(3.0/2.0 + 4.0 * H, 7.0 + 5.0 * H),
            Vec2::new(2.0 + 4.0 * H, 7.0 + 4.0 * H),
            Vec2::new(2.0 + 5.0 * H, 13.0/2.0 + 4.0 * H),
        ]),
        seed: 0,
    }];

    // squares (seed 1)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(1.0/2.0 + 2.0 * H, 5.0 + 3.0 * H),
            Vec2::new(1.0/2.0 + 2.0 * H, 4.0 + 3.0 * H),
            Vec2::new(3.0/2.0 + 2.0 * H, 4.0 + 3.0 * H),
            Vec2::new(3.0/2.0 + 2.0 * H, 5.0 + 3.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(2.0 + 3.0 * H, 11.0/2.0 + 4.0 * H),
            Vec2::new(3.0/2.0 + 3.0 * H, 11.0/2.0 + 3.0 * H),
            Vec2::new(3.0/2.0 + 4.0 * H, 5.0 + 3.0 * H),
            Vec2::new(2.0 + 4.0 * H, 5.0 + 4.0 * H),
        ]),
        seed: 1,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(2.0 + 4.0 * H, 7.0 + 4.0 * H),
            Vec2::new(3.0/2.0 + 4.0 * H, 7.0 + 5.0 * H),
            Vec2::new(3.0/2.0 + 3.0 * H, 13.0/2.0 + 5.0 * H),
            Vec2::new(2.0 + 3.0 * H, 13.0/2.0 + 4.0 * H),
        ]),
        seed: 1,
    });

    // triangles, first orbit (seed 2 — the two triangle orbits tie at 6/6 per cell, seed order = first appearance in the cell)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(1.0/2.0 + H, 9.0/2.0 + 3.0 * H),
            Vec2::new(1.0/2.0 + 2.0 * H, 4.0 + 3.0 * H),
            Vec2::new(1.0/2.0 + 2.0 * H, 5.0 + 3.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0/2.0 + 2.0 * H, 7.0 + 5.0 * H),
            Vec2::new(3.0/2.0 + 3.0 * H, 15.0/2.0 + 5.0 * H),
            Vec2::new(3.0/2.0 + 2.0 * H, 8.0 + 5.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0/2.0 + 4.0 * H, 7.0 + 5.0 * H),
            Vec2::new(3.0/2.0 + 3.0 * H, 15.0/2.0 + 5.0 * H),
            Vec2::new(3.0/2.0 + 3.0 * H, 13.0/2.0 + 5.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0/2.0 + 4.0 * H, 8.0 + 5.0 * H),
            Vec2::new(3.0/2.0 + 3.0 * H, 17.0/2.0 + 5.0 * H),
            Vec2::new(3.0/2.0 + 3.0 * H, 15.0/2.0 + 5.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(2.0 + 4.0 * H, 6.0 + 4.0 * H),
            Vec2::new(2.0 + 3.0 * H, 11.0/2.0 + 4.0 * H),
            Vec2::new(2.0 + 4.0 * H, 5.0 + 4.0 * H),
        ]),
        seed: 2,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(2.0 + 4.0 * H, 7.0 + 4.0 * H),
            Vec2::new(2.0 + 3.0 * H, 13.0/2.0 + 4.0 * H),
            Vec2::new(2.0 + 4.0 * H, 6.0 + 4.0 * H),
        ]),
        seed: 2,
    });

    // triangles, second orbit (seed 3)
    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(1.0/2.0 + H, 9.0/2.0 + 3.0 * H),
            Vec2::new(1.0/2.0 + H, 7.0/2.0 + 3.0 * H),
            Vec2::new(1.0/2.0 + 2.0 * H, 4.0 + 3.0 * H),
        ]),
        seed: 3,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(1.0/2.0 + H, 11.0/2.0 + 3.0 * H),
            Vec2::new(1.0/2.0 + H, 9.0/2.0 + 3.0 * H),
            Vec2::new(1.0/2.0 + 2.0 * H, 5.0 + 3.0 * H),
        ]),
        seed: 3,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0/2.0 + 2.0 * H, 5.0 + 3.0 * H),
            Vec2::new(3.0/2.0 + 3.0 * H, 9.0/2.0 + 3.0 * H),
            Vec2::new(3.0/2.0 + 3.0 * H, 11.0/2.0 + 3.0 * H),
        ]),
        seed: 3,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0/2.0 + 3.0 * H, 15.0/2.0 + 5.0 * H),
            Vec2::new(3.0/2.0 + 2.0 * H, 7.0 + 5.0 * H),
            Vec2::new(3.0/2.0 + 3.0 * H, 13.0/2.0 + 5.0 * H),
        ]),
        seed: 3,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(3.0/2.0 + 4.0 * H, 8.0 + 5.0 * H),
            Vec2::new(3.0/2.0 + 3.0 * H, 15.0/2.0 + 5.0 * H),
            Vec2::new(3.0/2.0 + 4.0 * H, 7.0 + 5.0 * H),
        ]),
        seed: 3,
    });

    cells.push(Cell {
        poly: Polygon::from_slice(&[
            Vec2::new(2.0 + 3.0 * H, 11.0/2.0 + 4.0 * H),
            Vec2::new(2.0 + 4.0 * H, 6.0 + 4.0 * H),
            Vec2::new(2.0 + 3.0 * H, 13.0/2.0 + 4.0 * H),
        ]),
        seed: 3,
    });
    LatticeTiling {
        id: "kuniform-n13",
        name: "2-uniform (3⁶; 3².4.12)",
        description: "2-uniform tiling (3⁶; 3².4.12) — a dodecagon ringed by squares and two triangle orbits around 3⁶ vertices (p6m).",
        lattice: [Vec2::new(3.0/2.0 + 3.0 * H, 3.0/2.0 + H), Vec2::new(0.0, 3.0 + 2.0 * H)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (3⁶; 3⁴.6), wallpaper group p6.
///
/// Wikipedia "k-uniform tilings" list entry `2-uniform n20` (Krotenheerdt's
/// closed list of 20 two-uniform tilings by regular polygons): t = 5 tile
/// orbits. The flower construction: take the unit triangular lattice Λ
/// and dissect, at each point of an index-13 sublattice
/// L13 = ⟨B1 = (1,3), B2 = (−3,4)⟩ (lattice coordinates, 60° basis), the
/// 6-face flower around that point into a regular unit hexagon. All
/// remaining unit triangles stay intact. Center-center distances are
/// √13, so flowers never touch (hard-core bound: norm² ≥ 7).
///
/// The translation lattice is ⟨B1, B2⟩ in real coordinates
/// b1 = (5/2, 3H), b2 = (−1, 4H); |b1| = |b2| = √13 at 60°,
/// |det| = 13√3/2 (right-handed). The origin is a hexagon center and the
/// 6-fold point. Cell content: 1 hexagon + 20 triangles (26 lattice
/// faces minus the 6-flower), vertices V = 12, edges E = 33,
/// V − E + F = 0 on the torus.
///
/// Two vertex orbits, 6 points each per cell, pinned by exact modular
/// arithmetic (class k(v) = 4X + 3Y mod 13, kernel = L13): the 3⁶ orbit
/// is classes {2,5,6,7,8,11}, the 3⁴.6 orbit is the hexagon-corner class
/// set {1,3,4,9,10,12}, and R60 acts as k → 4k with each set a single
/// orbit — so the tiling is 2-uniform and A:B = 1:1 (m96b_torus.py).
/// Under G = L13 ⋊ C6 the orbits are: 4 triangle orbits of sizes
/// {6,6,6,2} (seeds 1–4: hexagon-edge triangles; two corner-triangle
/// orbits; far triangles) plus the hexagon = t = 5; 6 edge orbits of
/// sizes {6,6,6,6,6,3}. The Wikipedia parenthetical says e = 7, but the
/// exact torus enumeration gives 6 (that annotation is unreliable — it
/// also lists e = 1 for the snub hexagonal (3.6)², which is impossible
/// since triangle–triangle and triangle–hexagon edges cannot share an
/// orbit); the wiki figure's own four triangle colors reproduce
/// {6,6,6,2}.
///
/// Symmetry: R60–R300 about the origin verified on a 46-radius window;
/// all 6 mirror probes through the hexagon center and every mirror probe
/// through A/B-class points fail (73.8% match through the hexagon
/// center), and the full translation scan finds exactly the six
/// norm-√13 vectors of L13 — the group is p6 (m96b_c13.py).
pub fn kuniform_n20() -> LatticeTiling {
    // Unit-triangle-lattice coordinates: real(v) = (X + Y/2, √3/2 · Y).
    // Hexagon at the origin (seed 0), corners at the 6 unit neighbors.
    let hex_c = [
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
        Vec2::new(-0.5, H),
        Vec2::new(-1.0, 0.0),
        Vec2::new(-0.5, -H),
        Vec2::new(0.5, -H),
    ];
    let mut cells: Vec<Cell> = vec![Cell {
        poly: Polygon::from_slice(&hex_c),
        seed: 0,
    }];

    // 20 triangles: one per translation class, grouped by orbit into the
    // four seeds (first appearance order): seed 1 = hexagon-edge
    // triangles (6), seed 2 and 4 = the two corner-triangle orbits (6
    // each), seed 3 = far triangles (2, the size-2 orbit centered on
    // the 3-fold points).
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(1.0, 0.0), Vec2::new(1.5, H), Vec2::new(0.5, H)]), seed: 1 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(-0.5, 3.0 * H), Vec2::new(0.5, 3.0 * H), Vec2::new(0.0, 4.0 * H)]), seed: 1 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(0.5, 3.0 * H), Vec2::new(1.0, 4.0 * H), Vec2::new(0.0, 4.0 * H)]), seed: 2 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(0.0, 2.0 * H), Vec2::new(1.0, 2.0 * H), Vec2::new(0.5, 3.0 * H)]), seed: 3 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(1.0, 2.0 * H), Vec2::new(1.5, 3.0 * H), Vec2::new(0.5, 3.0 * H)]), seed: 4 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(0.5, H), Vec2::new(1.5, H), Vec2::new(1.0, 2.0 * H)]), seed: 2 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(1.5, H), Vec2::new(2.0, 2.0 * H), Vec2::new(1.0, 2.0 * H)]), seed: 2 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(0.0, 4.0 * H), Vec2::new(1.0, 4.0 * H), Vec2::new(0.5, 5.0 * H)]), seed: 4 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(1.0, 4.0 * H), Vec2::new(1.5, 5.0 * H), Vec2::new(0.5, 5.0 * H)]), seed: 3 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(0.5, 3.0 * H), Vec2::new(1.5, 3.0 * H), Vec2::new(1.0, 4.0 * H)]), seed: 2 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(1.5, 3.0 * H), Vec2::new(2.0, 4.0 * H), Vec2::new(1.0, 4.0 * H)]), seed: 1 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(1.0, 2.0 * H), Vec2::new(2.0, 2.0 * H), Vec2::new(1.5, 3.0 * H)]), seed: 1 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(0.5, 5.0 * H), Vec2::new(1.5, 5.0 * H), Vec2::new(1.0, 6.0 * H)]), seed: 4 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(1.5, 5.0 * H), Vec2::new(2.0, 6.0 * H), Vec2::new(1.0, 6.0 * H)]), seed: 1 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(1.0, 4.0 * H), Vec2::new(2.0, 4.0 * H), Vec2::new(1.5, 5.0 * H)]), seed: 4 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(2.0, 4.0 * H), Vec2::new(2.5, 5.0 * H), Vec2::new(1.5, 5.0 * H)]), seed: 2 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(1.5, 5.0 * H), Vec2::new(2.5, 5.0 * H), Vec2::new(2.0, 6.0 * H)]), seed: 2 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(2.5, 5.0 * H), Vec2::new(3.0, 6.0 * H), Vec2::new(2.0, 6.0 * H)]), seed: 4 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(2.0, 4.0 * H), Vec2::new(3.0, 4.0 * H), Vec2::new(2.5, 5.0 * H)]), seed: 1 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(3.0, 4.0 * H), Vec2::new(3.5, 5.0 * H), Vec2::new(2.5, 5.0 * H)]), seed: 4 });
    LatticeTiling {
        id: "kuniform-n20",
        name: "2-uniform (3⁶; 3⁴.6)₁",
        description: "2-uniform tiling (3⁶; 3⁴.6)₁ — unit triangular lattice with index-13 sublattice hexagon flowers, the chiral p6 form of the two-uniform 3⁴.6 family: no mirrors and five tile orbits (t = 5), unlike ₂ (n19), the achiral p6m index-9 form with mirrors and three tile orbits.",
        lattice: [Vec2::new(5.0 / 2.0, 3.0 * H), Vec2::new(-1.0, 4.0 * H)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (3².6²; 3⁴.6), wallpaper group cmm.
///
/// Wikipedia "k-uniform tilings" list entry `2-uniform n12`
/// (Krotenheerdt's closed list of 20 two-uniform tilings by regular
/// polygons): rows of edge-sharing regular hexagons — consecutive
/// hexagons in a row share a full edge (center distance √3 = twice
/// the apothem) — and the bands between adjacent rows filled with
/// unit triangles. Every hexagon is congruent and all triangles lie
/// in one symmetry orbit (the 2-fold rotation at the origin swaps
/// the two visual triangle pairs modulo lattice), so t = 2; the
/// edges fall into e = 4 orbits, split by adjacency and corner
/// classes: hexagon–hexagon [3².6²|3².6²], hexagon–triangle
/// [3⁴.6|3².6²], triangle–triangle [3⁴.6|3⁴.6] and
/// triangle–triangle [3².6²|3⁴.6] (m97_rust_expect.py).
///
/// In the half-unit frame x = X·√3/2, y = Y/2 (hexagon side 1,
/// H = √3/2, S3 = √3) the hexagon centers are the even rows
/// (2m, 10k) and odd rows (2m+1, 10k+5), so the translation lattice
/// is the centered rectangular lattice Λ = ⟨b1 = (√3, 0), c =
/// (√3/2, 5/2)⟩ — the centering vector of the rectangular cell
/// ⟨b1, b2 = (0, 5)⟩ sits in Λ as c (b2 = −b1 + 2c), which is what
/// makes the group cmm rather than pmm. |det(b1, c)| = 5√3/2 = one
/// hexagon + four triangles: the table is the PRIMITIVE cell of the
/// centered lattice (the running-bond / stepped-hexominoes cmm
/// convention; the battery's 2-hexagon + 8-triangle centered cell is
/// S ⊎ (S + c) mod ℤb1 + ℤb2, with S the five tiles below). Torus
/// census V = 4, E = 9, F = 5, V − E + F = 0; the two vertex orbits
/// are 3².6² (the hex–hex·tri–tri junctions along the shared
/// hexagon edges, reps (√3/2, 1/2), (0, 2)) and 3⁴.6 (four triangles
/// plus one hexagon corner, reps (0, 1), (√3/2, 3/2)), two
/// translation classes each per primitive cell — A:B = 1:1.
///
/// Symmetry: cmm, pinned on a 49-cell patch (m97_rust_expect.py):
/// 8 translation probes (±b1, ±c and the centering-rectangle sides)
/// PASS; all 2-fold probes PASS (origin, b1/2 and their c/b2
/// translates — the two translation classes of 2-fold centers, each
/// hit at three named points), while 2-folds at the valley points
/// (√3/2, 1/2) and (0, 1/2) FAIL (245/245 tiles off); mirrors of both
/// families x = √3·k/2 and y = 5·k/2 PASS (6 probes); R60/R90/R120
/// about every mirror intersection FAIL (no 3-, 4- or 6-fold
/// anywhere). Battery on the emitted table (49 cells): area exact
/// (49·12 + 196·2 = 980 half-unit units = 49·|det|), 406 edge pairs,
/// 0 interior unpaired edges, 0 triple edges, 0 T-junctions,
/// fine-grid coverage exactly-once 1.0000.
pub fn kuniform_n12() -> LatticeTiling {
    // Pointy-top unit hexagon centered at the origin, CCW
    // (corners at 30°, 90°, ..., 330°).
    let hex = Polygon::from_slice(&[
        Vec2::new(-H, 0.5),
        Vec2::new(-H, -0.5),
        Vec2::new(0.0, -1.0),
        Vec2::new(H, -0.5),
        Vec2::new(H, 0.5),
        Vec2::new(0.0, 1.0),
    ]);
    // The four triangles of the primitive cell (seed 1 — they form a
    // single orbit under the group). A-pair: apex up between the
    // hexagon top vertex and the odd-row hexagon; B-pair: apex right
    // between the hexagon side vertex and the next hexagon in the
    // row. The +c translate supplies the odd-band copies, completing
    // the 8-triangle centered cell together with (hex, hex + c).
    let a2 = Polygon::from_slice(&[Vec2::new(0.0, 1.0), Vec2::new(H, 1.5), Vec2::new(0.0, 2.0)]);
    let a1 = Polygon::from_slice(&[Vec2::new(S3, 2.0), Vec2::new(H, 1.5), Vec2::new(S3, 1.0)]);
    let b1t = Polygon::from_slice(&[Vec2::new(H, 1.5), Vec2::new(0.0, 1.0), Vec2::new(H, 0.5)]);
    let b2t = Polygon::from_slice(&[Vec2::new(H, 0.5), Vec2::new(S3, 1.0), Vec2::new(H, 1.5)]);
    LatticeTiling {
        id: "kuniform-n12",
        name: "2-uniform (3².6²; 3⁴.6)",
        description: "2-uniform tiling (3².6²; 3⁴.6) — rows of edge-sharing hexagons with unit-triangle bands between the rows, the cmm two-uniform 3².6²/3⁴.6 form (cmm).",
        lattice: [Vec2::new(S3, 0.0), Vec2::new(H, 2.5)],
        cells: vec![
            Cell { poly: hex, seed: 0 },
            Cell { poly: a2, seed: 1 },
            Cell { poly: a1, seed: 1 },
            Cell { poly: b1t, seed: 1 },
            Cell { poly: b2t, seed: 1 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::Cmm,
        lattice_kind: LatticeKind::CenteredRectangular,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (3⁶; 3⁴.6), wallpaper group p6m.
///
/// Wikipedia "k-uniform tilings" list entry `2-uniform n19` (Krotenheerdt's
/// closed list of 20 two-uniform tilings by regular polygons): t = 3 tile
/// orbits. The flower construction: take the unit triangular lattice Λ
/// and dissect, at each point of the index-9 sublattice
/// L9 = ⟨B1 = (3,0), B2 = (0,3)⟩ (lattice coordinates, 60° basis), the
/// 6-face flower around that point into a regular unit hexagon. All
/// remaining unit triangles stay intact. Center-center distances are 3,
/// so flowers never touch (hard-core bound: norm² ≥ 7); along each
/// center line neighboring flowers leave a one-unit gap spanned by two
/// unit triangles.
///
/// The translation lattice is ⟨B1, B2⟩ in real coordinates
/// b1 = (3, 0), b2 = (3/2, 3H); |b1| = |b2| = 3 at 60°,
/// |det| = 9√3/2 (right-handed). The origin is a hexagon center and the
/// 6-fold point. Cell content: 1 hexagon + 12 triangles (18 lattice
/// faces per cell: the 6-face flower plus 12 intact triangles),
/// vertices V = 8, edges E = 21, V − E + F = 0 on the torus.
///
/// Two vertex orbits, pinned by exact modular arithmetic (class
/// k(v) = v mod 3, kernel = L9): the 3⁶ orbit is the petal-tip classes
/// {(1,1),(2,2)} (a single R60 2-cycle — the tips sit at distance √3
/// from the flower centers and are 3-fold points), the 3⁴.6 orbit is
/// the hexagon-corner class set {(1,0),(0,1),(2,1),(2,0),(0,2),(1,2)}
/// (a single R60 6-cycle) — so the tiling is 2-uniform and A:B = 2:6 =
/// 1:3 per cell (m95x_torus.py). Under the full group G = L9 ⋊ D6 the
/// triangle orbits are the 6 hexagon-edge petals and the 6 gap
/// triangles, plus the hexagon: t = 3; the edge orbits are {6, 12, 3}
/// (hexagon sides; the 12 petal edges — which the mirror merges from
/// two C6 orbits of 6; the 3 Q–Q edges) so e = 3, matching the
/// Wikipedia parenthetical; under the rotation subgroup C6 alone the
/// petal edges split 6+6 and e would be 4.
///
/// Symmetry: R60–R300 about the origin verified on a radius-6 window
/// (181/181 tiles), and — unlike the chiral n20 — both mirror classes
/// pass exactly (axes at 0°/30°/60°/90° through the origin: the lines
/// through the A tips and the lines through the Q–Q edge midpoints;
/// torus-exact tile mapping in m95x_torus.py), and the full translation
/// scan finds exactly 3Λ (min period norm² 9). The group is therefore
/// p6m. The Wikipedia table lists p6 for both 3⁴.6 variants, but only
/// n20 (the t = 5 variant, this module's chiral entry) is genuinely
/// p6: the t = 3 flower is forced by the orbit arithmetic (petals on
/// the six hexagon sides, tips at 3-fold points) and realizes the full
/// dihedral symmetry, so the wiki's p6 label under-states it.
pub fn kuniform_n19() -> LatticeTiling {
    // Unit-triangle-lattice coordinates: real(v) = (X + Y/2, √3/2 · Y).
    // Hexagon at the origin (seed 0), corners at the 6 unit neighbors.
    let hex_c = [
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
        Vec2::new(-0.5, H),
        Vec2::new(-1.0, 0.0),
        Vec2::new(-0.5, -H),
        Vec2::new(0.5, -H),
    ];
    let mut cells: Vec<Cell> = vec![Cell {
        poly: Polygon::from_slice(&hex_c),
        seed: 0,
    }];

    // 12 triangles: the 6 hexagon-edge petals (seed 1) and the 6 gap
    // triangles at the petal tips (seed 2), one per translation class,
    // all CCW.
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(1.5, H), Vec2::new(0.5, H), Vec2::new(1.0, 0.0)]), seed: 1 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(0.5, H), Vec2::new(0.0, 2.0 * H), Vec2::new(-0.5, H)]), seed: 1 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(-0.5, H), Vec2::new(-1.5, H), Vec2::new(-1.0, 0.0)]), seed: 1 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(-1.0, 0.0), Vec2::new(-1.5, -H), Vec2::new(-0.5, -H)]), seed: 1 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(-0.5, -H), Vec2::new(0.0, -2.0 * H), Vec2::new(0.5, -H)]), seed: 1 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(0.5, -H), Vec2::new(1.5, -H), Vec2::new(1.0, 0.0)]), seed: 1 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(1.5, H), Vec2::new(1.0, 0.0), Vec2::new(2.0, 0.0)]), seed: 2 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(1.5, H), Vec2::new(2.5, H), Vec2::new(2.0, 2.0 * H)]), seed: 2 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(1.5, H), Vec2::new(1.0, 2.0 * H), Vec2::new(0.5, H)]), seed: 2 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(0.0, 2.0 * H), Vec2::new(0.5, H), Vec2::new(1.0, 2.0 * H)]), seed: 2 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(0.0, 2.0 * H), Vec2::new(0.5, 3.0 * H), Vec2::new(-0.5, 3.0 * H)]), seed: 2 });
    cells.push(Cell { poly: Polygon::from_slice(&[Vec2::new(0.0, 2.0 * H), Vec2::new(-1.0, 2.0 * H), Vec2::new(-0.5, H)]), seed: 2 });
    LatticeTiling {
        id: "kuniform-n19",
        name: "2-uniform (3⁶; 3⁴.6)₂",
        description: "2-uniform tiling (3⁶; 3⁴.6)₂ — unit triangular lattice with index-9 sublattice hexagon flowers, the achiral p6m form of the two-uniform 3⁴.6 family: mirrors and three tile orbits (t = 3; the wiki table's p6 label under-states the symmetry), unlike ₁ (n20), the chiral p6 index-13 form without mirrors and five tile orbits.",
        lattice: [Vec2::new(3.0, 0.0), Vec2::new(1.5, 3.0 * H)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (4⁴; 3³.4²), wallpaper group cmm.
///
/// Wikipedia "k-uniform tilings" list entry `2-uniform n4` (Krotenheerdt's
/// closed list of 20 two-uniform tilings by regular polygons):
/// `[4⁴; 3³.4²]₁ (t = 2, e = 4)`, group cmm.
///
/// Construction: horizontal square bands two unit squares tall, separated
/// by alternating-triangle bands one triangle (height H) tall; consecutive
/// square bands are staggered by half a unit so the triangle band between
/// them is a single row of alternating up/down triangles sharing full
/// edges with both square bands. The primitive cell holds 2 squares +
/// 2 triangles (area 2 + √3/2 = 2.866025403).
///
/// The translation lattice is b1 = (1, 0), b2 = (1/2, 2 + √3/2),
/// |det| = 2 + √3/2 (right-handed); the full translation scan finds
/// exactly this lattice (min period (1, 0), second period b2). As with
/// `kuniform-n12` the lattice is centered-rectangular: the orthogonal
/// index-2 pair (1, 0), (0, 4 + √3) generates a sublattice whose centering
/// vector b2 it lacks. Torus enumeration over the 2×2-cell quotient
/// (exact surd arithmetic, tku3 battery): F = 4, V = 3 per cell; tile
/// orbits t = 2 (squares 2/cell, triangles 2/cell); edge orbits e = 4
/// (sizes 1 + 2 + 2 + 2 = 7/cell); vertex orbits 4444 (1/cell — the
/// crossing of four squares inside a band) and 33344 (2/cell — square-band
/// / triangle-band junctions), so the tiling is 2-uniform. t and e match
/// the Wikipedia parenthetical exactly.
///
/// Symmetry: the full scan finds R180 (52 centers), horizontal mirrors
/// (16 axes) and vertical mirrors (156 axes), and no rotations of other
/// order — the cmm signature, matching the wiki table.
pub fn kuniform_n4() -> LatticeTiling {
    // Square band column: two stacked unit squares (seed 0).
    let s1 = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ]);
    let s2 = Polygon::from_slice(&[
        Vec2::new(0.0, 1.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(1.0, 2.0),
        Vec2::new(0.0, 2.0),
    ]);
    // Triangle band above the squares (seed 1): up triangle on the square
    // row's top edge, down triangle between two up translates.
    let t_up = Polygon::from_slice(&[
        Vec2::new(0.0, 2.0),
        Vec2::new(1.0, 2.0),
        Vec2::new(0.5, 2.0 + H),
    ]);
    let t_dn = Polygon::from_slice(&[
        Vec2::new(0.5, 2.0 + H),
        Vec2::new(1.0, 2.0),
        Vec2::new(1.5, 2.0 + H),
    ]);
    LatticeTiling {
        id: "kuniform-n4",
        name: "2-uniform (4⁴; 3³.4²)₁",
        description: "2-uniform tiling (4⁴; 3³.4²) — staggered two-square bands alternating with alternating-triangle bands, the cmm two-uniform 4⁴/3³.4² form (cmm).",
        lattice: [Vec2::new(1.0, 0.0), Vec2::new(0.5, 2.0 + H)],
        cells: vec![
            Cell { poly: s1, seed: 0 },
            Cell { poly: s2, seed: 0 },
            Cell { poly: t_up, seed: 1 },
            Cell { poly: t_dn, seed: 1 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::Cmm,
        lattice_kind: LatticeKind::CenteredRectangular,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (3⁶; 3³.4²), wallpaper group pmm.
///
/// Wikipedia "k-uniform tilings" list entry `2-uniform n14` (Krotenheerdt's
/// closed list of 20 two-uniform tilings by regular polygons):
/// `[3⁶; 3³.4²]₁ (t = 3, e = 4)`, group pmm.
///
/// Construction: rows of edge-sharing unit squares, alternating with double
/// triangle strips — two counterposed rows of unit triangles (a strip of
/// up triangles on the strip-bottom line joined down-triangle-to-up-triangle
/// with a strip on the junction line), total strip height √3. Squares sit
/// directly on the strip top line; consecutive square rows share the strip
/// on their other side. The primitive cell holds 1 square + 4 triangles
/// (area 1 + √3 = 2.732050808).
///
/// The translation lattice is the PRIMITIVE one: b1 = (1, 0),
/// b2 = (0, 1 + √3), |det| = 1 + √3 (right-handed); the full translation
/// scan finds exactly ⟨b1, b2⟩ — notably (1, 0) is a period, so the
/// square row is NOT 3-periodic as an early candidate assumed. Torus
/// enumeration over the 2×2-cell quotient (exact surd arithmetic, tku3
/// battery): F = 5, V = 3 per cell; tile orbits t = 3 (square 1/cell and
/// two triangle orbits of 2/cell — {strip1-up, strip2-down} and
/// {strip1-down, strip2-up}, matching the wiki figure's two triangle
/// colors); edge orbits e = 4 (sizes 1 + 1 + 2 + 4 = 8/cell); vertex
/// orbits 3⁶ (1/cell, on the strip-junction line) and 3³.4² (2/cell,
/// the square-row corners), so the tiling is 2-uniform. t and e match the
/// Wikipedia parenthetical exactly.
///
/// Symmetry: the full scan finds R180 (56 centers), horizontal mirrors
/// (56 axes) and vertical mirrors (164 axes), and nothing else — the pmm
/// signature, matching the wiki table.
pub fn kuniform_n14() -> LatticeTiling {
    // Double triangle strip between y = 0 and y = √3 (strip1: 0..H,
    // strip2: H..√3), square row on the strip top line. Seeds: 0 square,
    // 1 = {strip1 up, strip2 down}, 2 = {strip1 down, strip2 up}.
    let st1_up = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, H),
    ]);
    let st1_dn = Polygon::from_slice(&[
        Vec2::new(-0.5, H),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.5, H),
    ]);
    let st2_up = Polygon::from_slice(&[
        Vec2::new(-0.5, H),
        Vec2::new(0.5, H),
        Vec2::new(0.0, S3),
    ]);
    let st2_dn = Polygon::from_slice(&[
        Vec2::new(0.0, S3),
        Vec2::new(0.5, H),
        Vec2::new(1.0, S3),
    ]);
    let sq = Polygon::from_slice(&[
        Vec2::new(0.0, S3),
        Vec2::new(1.0, S3),
        Vec2::new(1.0, 1.0 + S3),
        Vec2::new(0.0, 1.0 + S3),
    ]);
    LatticeTiling {
        id: "kuniform-n14",
        name: "2-uniform (3⁶; 3³.4²)₁",
        description: "2-uniform tiling (3⁶; 3³.4²) — square rows alternating with double triangle strips (two counterposed triangle rows), the pmm two-uniform 3⁶/3³.4² form (pmm).",
        lattice: [Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0 + S3)],
        cells: vec![
            Cell { poly: st1_up, seed: 1 },
            Cell { poly: st1_dn, seed: 2 },
            Cell { poly: st2_up, seed: 2 },
            Cell { poly: st2_dn, seed: 1 },
            Cell { poly: sq, seed: 0 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::Pmm,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (4⁴; 3³.4²), wallpaper group cmm.
///
/// Wikipedia "k-uniform tilings" list entry `2-uniform n3` (Krotenheerdt's
/// closed list of 20 two-uniform tilings by regular polygons):
/// `[4⁴; 3³.4²]₂ (t = 3, e = 5)`, the sibling of `kuniform-n4`.
///
/// The file-index ↔ caption mapping was re-verified anchor-first against
/// the saved wiki page before porting (caption follows the image inside the
/// same table cell): the SVG named n3.svg carries the (3, 5) parenthetical
/// and the three-row band figure; the landed `kuniform-n4` entry is the
/// (2, 4) sibling. Construction: three-square-tall bands (three unit
/// squares stacked edge-to-edge) alternating with one alternating-triangle
/// band one triangle (height H) tall; consecutive bands are staggered by
/// half a unit — forced by the triangle band sharing full edges with both
/// square rows. The primitive cell holds 3 squares + 2 triangles
/// (area 3 + √3/2 = 3.866025404). Seeds mirror the wiki figure's three
/// colors: seed 0 = the two outer rows of a band (one orbit — the band is
/// symmetric under the horizontal mirror through its middle row), seed 1 =
/// the middle row, seed 2 = both triangle rows.
///
/// The translation lattice is b1 = (1, 0), b2 = (1/2, 3 + √3/2),
/// |det| = 3 + √3/2 (right-handed); the translation scan's only periods
/// with |t| ≤ 3.2 are multiples of (1, 0), and b2 closes the cell — the
/// battery's exact coverage is 1.0 (no gaps, no overlaps) over a 6×6-cell
/// patch. As with `kuniform-n4` the lattice is centered-rectangular: the
/// orthogonal index-2 pair (1, 0), (0, 6 + √3) generates a sublattice
/// whose centering vector b2 it lacks. Orbit enumeration over the exact
/// affine action on lattice-orbit classes (exact surd arithmetic, tku4
/// battery — fundamental-domain reduction, immune to the parity artifacts
/// a 2×2-cell torus quotient exhibits for staggered cells): F = 5, V = 4
/// per cell; tile orbits t = 3 (class sizes 1 + 2 + 2 — the middle row
/// 1/cell, outer rows and triangles 2/cell each); edge orbits e = 5
/// (sizes 1 + 2 + 2 + 2 + 2 = 9/cell); vertex orbits 4444 (2/cell — the
/// interior crossings of a band) and 33344 (2/cell — the square-band /
/// triangle-band junctions), so the tiling is 2-uniform. t and e match
/// the Wikipedia parenthetical exactly.
///
/// Symmetry: the full scan finds R180 (72 centers), horizontal mirrors
/// (36 axes) and vertical mirrors (208 axes), and no rotations of other
/// order — the cmm signature.
pub fn kuniform_n3() -> LatticeTiling {
    // Three-square-tall band y = 0..3: outer rows seed 0 (one orbit under
    // the horizontal mirror through the middle row), middle row seed 1.
    // Alternating-triangle band y = 3..3+H, seed 2.
    let r1 = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ]);
    let r2 = Polygon::from_slice(&[
        Vec2::new(0.0, 1.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(1.0, 2.0),
        Vec2::new(0.0, 2.0),
    ]);
    let r3 = Polygon::from_slice(&[
        Vec2::new(0.0, 2.0),
        Vec2::new(1.0, 2.0),
        Vec2::new(1.0, 3.0),
        Vec2::new(0.0, 3.0),
    ]);
    let t_up = Polygon::from_slice(&[
        Vec2::new(0.0, 3.0),
        Vec2::new(1.0, 3.0),
        Vec2::new(0.5, 3.0 + H),
    ]);
    let t_dn = Polygon::from_slice(&[
        Vec2::new(0.0, 3.0),
        Vec2::new(0.5, 3.0 + H),
        Vec2::new(-0.5, 3.0 + H),
    ]);
    LatticeTiling {
        id: "kuniform-n3",
        name: "2-uniform (4⁴; 3³.4²)₂",
        description: "2-uniform tiling (4⁴; 3³.4²) — staggered three-square bands alternating with alternating-triangle bands, the cmm two-uniform 4⁴/3³.4² sibling form (cmm).",
        lattice: [Vec2::new(1.0, 0.0), Vec2::new(0.5, 3.0 + H)],
        cells: vec![
            Cell { poly: r1, seed: 0 },
            Cell { poly: r2, seed: 1 },
            Cell { poly: r3, seed: 0 },
            Cell { poly: t_up, seed: 2 },
            Cell { poly: t_dn, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::Cmm,
        lattice_kind: LatticeKind::CenteredRectangular,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (3⁶; 3³.4²), wallpaper group cmm.
///
/// Wikipedia "k-uniform tilings" list entry `2-uniform n15` (Krotenheerdt's
/// closed list of 20 two-uniform tilings by regular polygons):
/// `[3⁶; 3³.4²]₂ (t = 4, e = 5)`, the sibling of `kuniform-n14` (the
/// saved wiki page's n15.svg carries the (4, 5) parenthetical and the
/// triple-strip figure; the landed `kuniform-n14` is the (3, 4) form).
///
/// Construction: single rows of edge-sharing unit squares alternating with
/// TRIPLE triangle strips — three rows of height H in which every
/// horizontal line carries triangles above AND below sharing the unit edge
/// (back-to-back pairs), unlike `kuniform-n14`'s double strips. The two
/// lines bounding a square row carry a single triangle-orbit of flankers:
/// a down triangle on the square row's bottom edge and an up triangle on
/// its top edge, both on the squares' own x grid. The two inner lines of
/// the strip are offset by half a unit each step. Consecutive square rows
/// are staggered by half a unit. The primitive cell holds 1 square +
/// 6 triangles (area 1 + 3√3/2 = 3.598076211). Seeds mirror the wiki
/// figure's four colors (= the four tile orbits): seed 0 = square,
/// seed 1 = the two flanker triangles, seed 2 = the yellow pair
/// {down on line 1+H, up on line 1+2H}, seed 3 = the cyan pair.
///
/// The translation lattice is b1 = (1, 0), b2 = (1/2, 1 + 3√3/2),
/// |det| = 1 + 3√3/2 (right-handed); the translation scan's periods with
/// |t| ≤ 3.0 are exactly the multiples of (1, 0), and b2 closes the cell —
/// battery coverage 1.0 over a 6×6-cell patch. The lattice is
/// centered-rectangular: the orthogonal index-2 pair (1, 0), (0, 2 + 3√3)
/// generates a sublattice whose centering vector b2 it lacks. Orbit
/// enumeration over the exact affine action on lattice-orbit classes
/// (exact surd arithmetic, tku4 battery): F = 7, V = 4, E = 11 per cell
/// (V − E + F = 0); tile orbits t = 4 (sizes 1 + 2 + 2 + 2, matching the
/// wiki figure's four colors); edge orbits e = 5 (sizes 1 + 2 + 2 + 2 + 4
/// = 11/cell); vertex orbits 3⁶ (2/cell, inside the strips) and 33344
/// (2/cell, the square-row corners), so the tiling is 2-uniform. t and e
/// match the Wikipedia parenthetical exactly.
///
/// Symmetry: the full scan finds R180 (9 centers in the probe window),
/// horizontal mirrors (9 axes) and vertical mirrors (203 axes), and
/// nothing else — with the centered lattice, the cmm signature. The
/// horizontal mirror through a square row's center fuses the flanker pair;
/// the strip's two colored pairs are fused by the same mirror followed by
/// the half-period lattice shift (the wiki figure colors each fused pair
/// identically — verified as exact orbit classes, not assumed from color).
pub fn kuniform_n15() -> LatticeTiling {
    // Square [−1/2, 1/2] × [0, 1] seed 0; flanker triangles seed 1
    // (down on the bottom edge, up on the top edge, squares' own x grid);
    // triple strip y = 1..1+3H with the yellow pair seed 2 and the cyan
    // pair seed 3; the strip's top line 1+3H is the next square's bottom.
    let sq = Polygon::from_slice(&[
        Vec2::new(-0.5, 0.0),
        Vec2::new(0.5, 0.0),
        Vec2::new(0.5, 1.0),
        Vec2::new(-0.5, 1.0),
    ]);
    let f_dn = Polygon::from_slice(&[
        Vec2::new(0.5, 0.0),
        Vec2::new(-0.5, 0.0),
        Vec2::new(0.0, -H),
    ]);
    let f_up = Polygon::from_slice(&[
        Vec2::new(-0.5, 1.0),
        Vec2::new(0.5, 1.0),
        Vec2::new(0.0, 1.0 + H),
    ]);
    let y_dn = Polygon::from_slice(&[
        Vec2::new(1.0, 1.0 + H),
        Vec2::new(0.0, 1.0 + H),
        Vec2::new(0.5, 1.0),
    ]);
    let c_up = Polygon::from_slice(&[
        Vec2::new(0.0, 1.0 + H),
        Vec2::new(1.0, 1.0 + H),
        Vec2::new(0.5, 1.0 + S3),
    ]);
    let c_dn = Polygon::from_slice(&[
        Vec2::new(0.5, 1.0 + S3),
        Vec2::new(-0.5, 1.0 + S3),
        Vec2::new(0.0, 1.0 + H),
    ]);
    let y_up = Polygon::from_slice(&[
        Vec2::new(-0.5, 1.0 + S3),
        Vec2::new(0.5, 1.0 + S3),
        Vec2::new(0.0, 1.0 + 3.0 * H),
    ]);
    LatticeTiling {
        id: "kuniform-n15",
        name: "2-uniform (3⁶; 3³.4²)₂",
        description: "2-uniform tiling (3⁶; 3³.4²) — staggered square rows alternating with triple triangle strips (every horizontal edge shared back-to-back, square rows flanked by one triangle orbit), the cmm two-uniform 3⁶/3³.4² sibling form (cmm).",
        lattice: [Vec2::new(1.0, 0.0), Vec2::new(0.5, 1.0 + 3.0 * H)],
        cells: vec![
            Cell { poly: sq, seed: 0 },
            Cell { poly: f_dn, seed: 1 },
            Cell { poly: f_up, seed: 1 },
            Cell { poly: y_dn, seed: 2 },
            Cell { poly: c_up, seed: 3 },
            Cell { poly: c_dn, seed: 3 },
            Cell { poly: y_up, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::Cmm,
        lattice_kind: LatticeKind::CenteredRectangular,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (3.12.12; 3.4.3.12), wallpaper group p4m.
///
/// Wikipedia "List of k-uniform tilings" entry `2-uniform n2` and the
/// dedicated "3-4-3-12 tiling" article: `[3.12.12; 3.4.3.12] (t = 3,
/// e = 3)`, p4m (4*2).
///
/// Construction: unit dodecagons on a square sublattice of spacing
/// T = 2 + √3, touching flat-edge to flat-edge (their inradius is
/// (2 + √3)/2 = T/2, so neighbors kiss along full unit edges — those
/// contacts carry the 3.12.12 vertices); each square hole bounded by
/// four kissing dodecagons is filled by a unit square plus one unit
/// triangle on each of its four edges (apex = a dodecagon corner),
/// giving the 3.4.3.12 vertices at the square corners. The cell is the
/// cross (square + 4 triangles) centered at the origin plus the
/// dodecagon centered at (T/2, T/2). The cell area is
/// 1 + 4·√3/4 + 3(2 + √3) = 7 + 4√3 = T² = |det|, so the square lattice
/// ⟨(T, 0), (0, T)⟩ is exactly the tiling's translation lattice.
///
/// Symmetry: exact coset audit (Python battery tku5_n2.py): translations
/// mod L give order 8 with point group D4 — reflections across the
/// horizontal and vertical (no glide: mirror lines through the square
/// centers and dodecagon centers) and no true glides ⇒ p4m. Torus
/// orbit census: t = 3 tile orbits (dodecagon, square, 4 triangles),
/// e = 3 edge orbits, 2 vertex orbits (1/cell dodecagon-corner class
/// 3.12.12, 1/cell square-corner class 3.4.3.12 — the origin) — both
/// counts match the Wikipedia parenthetical exactly.
///
/// Pattern seeds by polygon class: dodecagon seed 0, square seed 1,
/// triangles seed 2.
pub fn kuniform_n2() -> LatticeTiling {
    // Unit dodecagon centered at (T/2, T/2), T = 2 + √3; corners are
    // the 12 rotations of the −135° offset −(1+√3)/2·(1,1) about the
    // center — the first corner is (1/2, 1/2). CCW by construction.
    let dod = Polygon::from_slice(&[
        Vec2::new(1.0 / 2.0, 1.0 / 2.0),
        Vec2::new(1.0 / 2.0 + H, 0.0),
        Vec2::new(3.0 / 2.0 + H, 0.0),
        Vec2::new(3.0 / 2.0 + 2.0 * H, 1.0 / 2.0),
        Vec2::new(2.0 + 2.0 * H, 1.0 / 2.0 + H),
        Vec2::new(2.0 + 2.0 * H, 3.0 / 2.0 + H),
        Vec2::new(3.0 / 2.0 + 2.0 * H, 3.0 / 2.0 + 2.0 * H),
        Vec2::new(3.0 / 2.0 + H, 2.0 + 2.0 * H),
        Vec2::new(1.0 / 2.0 + H, 2.0 + 2.0 * H),
        Vec2::new(1.0 / 2.0, 3.0 / 2.0 + 2.0 * H),
        Vec2::new(0.0, 3.0 / 2.0 + H),
        Vec2::new(0.0, 1.0 / 2.0 + H),
    ]);
    // Unit square centered at the origin (the 3.4.3.12 vertex orbit
    // includes its corners, but the canonical 3.4.3.12 vertex is e.g.
    // (1/2, 1/2): triangle + square + triangle + dodecagon).
    let sq = Polygon::from_slice(&[
        Vec2::new(-1.0 / 2.0, -1.0 / 2.0),
        Vec2::new(1.0 / 2.0, -1.0 / 2.0),
        Vec2::new(1.0 / 2.0, 1.0 / 2.0),
        Vec2::new(-1.0 / 2.0, 1.0 / 2.0),
    ]);
    // The four unit triangles on the square's edges, apex A = 1/2 + √3/2
    // toward the adjacent dodecagon-center directions. All CCW.
    let a = 1.0 / 2.0 + H;
    let t_up = Polygon::from_slice(&[
        Vec2::new(-1.0 / 2.0, 1.0 / 2.0),
        Vec2::new(1.0 / 2.0, 1.0 / 2.0),
        Vec2::new(0.0, a),
    ]);
    let t_dn = Polygon::from_slice(&[
        Vec2::new(1.0 / 2.0, -1.0 / 2.0),
        Vec2::new(-1.0 / 2.0, -1.0 / 2.0),
        Vec2::new(0.0, -a),
    ]);
    let t_rt = Polygon::from_slice(&[
        Vec2::new(1.0 / 2.0, 1.0 / 2.0),
        Vec2::new(1.0 / 2.0, -1.0 / 2.0),
        Vec2::new(a, 0.0),
    ]);
    let t_lf = Polygon::from_slice(&[
        Vec2::new(-1.0 / 2.0, -1.0 / 2.0),
        Vec2::new(-1.0 / 2.0, 1.0 / 2.0),
        Vec2::new(-a, 0.0),
    ]);
    LatticeTiling {
        id: "kuniform-n2",
        name: "2-uniform (3.12.12; 3.4.3.12)",
        description: "2-uniform tiling (3.12.12; 3.4.3.12) — dodecagons kissing flat-edge-to-flat-edge on a square lattice with unit squares and edge triangles filling each four-dodecagon hole (p4m).",
        lattice: [Vec2::new(2.0 + S3, 0.0), Vec2::new(0.0, 2.0 + S3)],
        cells: vec![
            Cell { poly: dod, seed: 0 },
            Cell { poly: sq, seed: 1 },
            Cell { poly: t_up, seed: 2 },
            Cell { poly: t_dn, seed: 2 },
            Cell { poly: t_rt, seed: 2 },
            Cell { poly: t_lf, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4m,
        lattice_kind: LatticeKind::Square,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (3³.4²; 3².4.3.4), wallpaper group p4g.
///
/// Wikipedia "List of k-uniform tilings" entry `2-uniform n16` (the
/// "33344-33434 tiling" article's variant 1): `[3³.4²; 3².4.3.4]`,
/// p4g (4\*2), 4-isohedral, 5-isotoxal. (The article's variant 2, the
/// chiral pgg sibling, is `n17` and is NOT this entry.)
///
/// Construction (extracted from the wiki figure 2-uniform_n16.svg and
/// re-derived exactly): rows of edge-sharing unit squares along y with
/// square rows staggered, surrounded by up/down triangle bands — the
/// snub-square-like 3².4.3.4 vertices alternate with the elongated-
/// triangular 3³.4² vertices in a 2:1 ratio (8 + 4 per cell). The
/// translation lattice is b1 = (h, −h), b2 = (h, h) with h = (3 + √3)/2
/// (right-handed, square kind), |det| = 2h² = 6 + 3√3; the cell holds
/// 6 squares + 12 triangles, area 6 + 12·√3/4 = 6 + 3√3 = |det|
/// exactly. Vertex classes per cell: 4 of 3³.4² + 8 of 3².4.3.4.
///
/// Verification (Python battery tku5_n16f/n16k, exact Q(√3) arithmetic
/// over a 7×7-cell emission): cell area = |det| exactly; every interior
/// edge shared by exactly two tiles; vertex-figure census (cyclic
/// words) only 3.3.3.4.4 / 3.3.4.3.4; fine-grid coverage exactly once.
/// Symmetry measured by coset audit over the emission: exactly one
/// isometry coset per D4 linear part (order 8 mod L; periods with
/// |t| ≤ 4.75 are exactly ±b1, ±b2, ±(b1±b2)), with the axis-mirror
/// cosets (x- and y-reflections) pure reflections and the two diagonal
/// cosets TRUE glides (axial shift 7.098 = 3/4 of a diagonal lattice
/// period, no integral lattice shift cancels it) — the p4g signature.
/// Torus orbit census: t = 4 tile orbits (sizes 8 + 4 + 4 + 2 = 18)
/// and e = 5 edge orbits, matching the wiki's 4-isohedral /
/// 5-isotoxal data exactly.
///
/// Seeds by tile orbit (the wiki figure's four colors): seed 0 = the
/// 4-square axis-aligned orbit, seed 1 = the 2-square tilted orbit;
/// seed 2 = the 8-triangle orbit, seed 3 = the 4-triangle orbit.
pub fn kuniform_n16() -> LatticeTiling {
    // Squares, seed 0 (the 4-orbit): the two unit squares stacked at
    // x ∈ [H, 1 + H] and the pair at x ∈ [1 + 2H, 3 + 2H].
    let s0a = Polygon::from_slice(&[
        Vec2::new(H, -1.0 / 2.0),
        Vec2::new(1.0 + H, -1.0 / 2.0),
        Vec2::new(1.0 + H, 1.0 / 2.0),
        Vec2::new(H, 1.0 / 2.0),
    ]);
    let s0b = Polygon::from_slice(&[
        Vec2::new(H, 1.0 / 2.0),
        Vec2::new(1.0 + H, 1.0 / 2.0),
        Vec2::new(1.0 + H, 3.0 / 2.0),
        Vec2::new(H, 3.0 / 2.0),
    ]);
    let s0c = Polygon::from_slice(&[
        Vec2::new(1.0 + 2.0 * H, 0.0),
        Vec2::new(2.0 + 2.0 * H, 0.0),
        Vec2::new(2.0 + 2.0 * H, 1.0),
        Vec2::new(1.0 + 2.0 * H, 1.0),
    ]);
    let s0d = Polygon::from_slice(&[
        Vec2::new(2.0 + 2.0 * H, 0.0),
        Vec2::new(3.0 + 2.0 * H, 0.0),
        Vec2::new(3.0 + 2.0 * H, 1.0),
        Vec2::new(2.0 + 2.0 * H, 1.0),
    ]);
    // Squares, seed 1 (the 2-orbit): the tilted pair closing the
    // diagonal square columns.
    let s1a = Polygon::from_slice(&[
        Vec2::new(3.0 / 2.0 + H, -1.0 / 2.0 - H),
        Vec2::new(3.0 / 2.0 + 2.0 * H, -H),
        Vec2::new(1.0 + 2.0 * H, 0.0),
        Vec2::new(1.0 + H, -1.0 / 2.0),
    ]);
    let s1b = Polygon::from_slice(&[
        Vec2::new(1.0 + H, 3.0 / 2.0),
        Vec2::new(1.0 + 2.0 * H, 1.0),
        Vec2::new(3.0 / 2.0 + 2.0 * H, 1.0 + H),
        Vec2::new(3.0 / 2.0 + H, 3.0 / 2.0 + H),
    ]);
    // Triangles, seed 2 (the 8-orbit).
    let t2a = Polygon::from_slice(&[
        Vec2::new(H, -1.0 / 2.0),
        Vec2::new(H, 1.0 / 2.0),
        Vec2::new(0.0, 0.0),
    ]);
    let t2b = Polygon::from_slice(&[
        Vec2::new(H, 1.0 / 2.0),
        Vec2::new(H, 3.0 / 2.0),
        Vec2::new(0.0, 1.0),
    ]);
    let t2c = Polygon::from_slice(&[
        Vec2::new(1.0 / 2.0 + H, -1.0 / 2.0 - H),
        Vec2::new(3.0 / 2.0 + H, -1.0 / 2.0 - H),
        Vec2::new(1.0 + H, -1.0 / 2.0),
    ]);
    let t2d = Polygon::from_slice(&[
        Vec2::new(1.0 + H, 3.0 / 2.0),
        Vec2::new(3.0 / 2.0 + H, 3.0 / 2.0 + H),
        Vec2::new(1.0 / 2.0 + H, 3.0 / 2.0 + H),
    ]);
    let t2e = Polygon::from_slice(&[
        Vec2::new(1.0 + H, -1.0 / 2.0),
        Vec2::new(1.0 + 2.0 * H, 0.0),
        Vec2::new(1.0 + H, 1.0 / 2.0),
    ]);
    let t2f = Polygon::from_slice(&[
        Vec2::new(1.0 + H, 1.0 / 2.0),
        Vec2::new(1.0 + 2.0 * H, 1.0),
        Vec2::new(1.0 + H, 3.0 / 2.0),
    ]);
    let t2g = Polygon::from_slice(&[
        Vec2::new(3.0 / 2.0 + 2.0 * H, -H),
        Vec2::new(2.0 + 2.0 * H, 0.0),
        Vec2::new(1.0 + 2.0 * H, 0.0),
    ]);
    let t2h = Polygon::from_slice(&[
        Vec2::new(1.0 + 2.0 * H, 1.0),
        Vec2::new(2.0 + 2.0 * H, 1.0),
        Vec2::new(3.0 / 2.0 + 2.0 * H, 1.0 + H),
    ]);
    // Triangles, seed 3 (the 4-orbit).
    let t3a = Polygon::from_slice(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(H, 1.0 / 2.0),
        Vec2::new(0.0, 1.0),
    ]);
    let t3b = Polygon::from_slice(&[
        Vec2::new(1.0 / 2.0 + H, -1.0 / 2.0 - H),
        Vec2::new(1.0 + H, -1.0 / 2.0),
        Vec2::new(H, -1.0 / 2.0),
    ]);
    let t3c = Polygon::from_slice(&[
        Vec2::new(H, 3.0 / 2.0),
        Vec2::new(1.0 + H, 3.0 / 2.0),
        Vec2::new(1.0 / 2.0 + H, 3.0 / 2.0 + H),
    ]);
    let t3d = Polygon::from_slice(&[
        Vec2::new(1.0 + 2.0 * H, 0.0),
        Vec2::new(1.0 + 2.0 * H, 1.0),
        Vec2::new(1.0 + H, 1.0 / 2.0),
    ]);
    LatticeTiling {
        id: "kuniform-n16",
        name: "2-uniform (3³.4²; 3².4.3.4)",
        description: "2-uniform tiling (3³.4²; 3².4.3.4) — staggered square rows and triangle bands in the snub-square-related 4-isohedral form, diagonal glide mirrors (p4g).",
        lattice: [
            Vec2::new(3.0 / 2.0 + H, -(3.0 / 2.0 + H)),
            Vec2::new(3.0 / 2.0 + H, 3.0 / 2.0 + H),
        ],
        cells: vec![
            Cell { poly: s0a, seed: 0 },
            Cell { poly: s0b, seed: 0 },
            Cell { poly: s0c, seed: 0 },
            Cell { poly: s0d, seed: 0 },
            Cell { poly: s1a, seed: 1 },
            Cell { poly: s1b, seed: 1 },
            Cell { poly: t2a, seed: 2 },
            Cell { poly: t2b, seed: 2 },
            Cell { poly: t2c, seed: 2 },
            Cell { poly: t2d, seed: 2 },
            Cell { poly: t2e, seed: 2 },
            Cell { poly: t2f, seed: 2 },
            Cell { poly: t2g, seed: 2 },
            Cell { poly: t2h, seed: 2 },
            Cell { poly: t3a, seed: 3 },
            Cell { poly: t3b, seed: 3 },
            Cell { poly: t3c, seed: 3 },
            Cell { poly: t3d, seed: 3 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4g,
        lattice_kind: LatticeKind::Square,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// 2-uniform tiling (3³.4²; 3².4.3.4)₂, wallpaper group pgg.
///
/// Wikipedia "List of k-uniform tilings" entry `2-uniform n17` (the
/// "33344-33434 tiling" article's variant 2): `[3³.4²; 3².4.3.4]₂`,
/// pgg (22×), 3-isohedral, 6-isotoxal — "triangles in groups of 4,
/// squares in groups of 2". (The article's variant 1, the p4g form,
/// is `n16`, NOT this entry. The wiki figure filename carries a `-1`
/// suffix: 2-uniform_n17-1.svg; the dual is 2-uniform_17_dual.svg.)
///
/// Construction (extracted from the wiki figure 2-uniform_n17-1.svg
/// and re-derived exactly): stair-stepped unit squares and triangles —
/// 4 squares + 8 equilateral triangles per cell (area 4 + 2√3), the
/// 8 triangles in two orbits of 4 (T1–T4, G1–G4). Every vertex is
/// 5-valent; the 3².4.3.4 vertices (8 per cell) and the 3³.4²
/// vertices (4 per cell) run in stair-step rows.
///
/// Translation lattice: b1 = (1/2, 1+√3/2), b2 = (−2−√3, 1)
/// (right-handed, rectangular kind — b1 ⊥ b2 exactly, |b2| = 2|b1|,
/// |det| = 4 + 2√3). The coset audit proves the period lattice is
/// EXACTLY this L: 0 of the 12 square-centroid coset representatives
/// and 0 short shifts pass (a period d has δ = d mod L in the unit
/// square; δ ≠ 0 would translate a square of the emission cell to a
/// square position with corners outside the window — impossible). The
/// emission cell is the true primitive cell (12 tiles/cell; no
/// index-2 superlattice).
///
/// Symmetry (Python battery, exact Q(√3) arithmetic over the ±3×±3
/// emission patch): periods b1 and b2 pass with exact corner
/// projections; four 2-fold centers per cell on the 2×2 half-period
/// grid — fractional (b1,b2) coords (0.590920, 0.864637),
/// (0.090920, 0.364637), and their half-period shifts,
/// 2(c1−c2) = b1+b2 exactly — in exactly two group orbits {c1, c2}
/// and {c1+½b1, c1−½b2}; all Klein-four identities hold on all 495
/// patch vertices; one glide per axis (b1-glide at β0 = 0.385363 mod
/// ½, b2-glide at α0 = 0.340920 mod ½; g² = T(axis), no fixed
/// points); 1097 mirror-bisector candidates (all 15°-multiple
/// directions), 295 full-checked, 0 reflections; no 3/4/6-fold center
/// anywhere (triangle-centroid census 1368 = 944 mult-1 + 12 mult-2 +
/// 412 mult-3, no centroid shared by two different triangles). Torus
/// orbit census: t = 3 tile orbits (4+4+4), e = 6 edge orbits
/// (4+4+4+4+2+2), v = 2 vertex orbits (4+4) — matching the wiki's
/// 3-isohedral / 6-isotoxal data exactly.
///
/// Seeds by tile orbit: seed 0 = the 4-square orbit, seed 1 = the
/// 4-triangle orbit T1–T4, seed 2 = the 4-triangle orbit G1–G4.
pub fn kuniform_n17() -> LatticeTiling {
    // Squares, seed 0 (the 4-orbit): two axis-aligned squares stacked
    // at x ∈ [13/6, 25/6] plus the two tilted squares closing the
    // stair-step.
    let s1 = Polygon::from_slice(&[
        Vec2::new(13.0 / 6.0, 11.0 / 6.0),
        Vec2::new(19.0 / 6.0, 11.0 / 6.0),
        Vec2::new(19.0 / 6.0, 17.0 / 6.0),
        Vec2::new(13.0 / 6.0, 17.0 / 6.0),
    ]);
    let s2 = Polygon::from_slice(&[
        Vec2::new(19.0 / 6.0, 11.0 / 6.0),
        Vec2::new(25.0 / 6.0, 11.0 / 6.0),
        Vec2::new(25.0 / 6.0, 17.0 / 6.0),
        Vec2::new(19.0 / 6.0, 17.0 / 6.0),
    ]);
    let s3 = Polygon::from_slice(&[
        Vec2::new(25.0 / 6.0, 17.0 / 6.0),
        Vec2::new(25.0 / 6.0 + H, 7.0 / 3.0),
        Vec2::new(14.0 / 3.0 + H, 7.0 / 3.0 + H),
        Vec2::new(14.0 / 3.0, 17.0 / 6.0 + H),
    ]);
    let s4 = Polygon::from_slice(&[
        Vec2::new(25.0 / 6.0 + H, 7.0 / 3.0),
        Vec2::new(25.0 / 6.0 + S3, 11.0 / 6.0),
        Vec2::new(14.0 / 3.0 + S3, 11.0 / 6.0 + H),
        Vec2::new(14.0 / 3.0 + H, 7.0 / 3.0 + H),
    ]);
    // Triangles, seed 1 (the 4-orbit): the two up-pointing pairs under
    // the tilted squares and the two down-pointing pairs on the row.
    let t1 = Polygon::from_slice(&[
        Vec2::new(25.0 / 6.0, 11.0 / 6.0),
        Vec2::new(25.0 / 6.0 + H, 4.0 / 3.0),
        Vec2::new(25.0 / 6.0 + H, 7.0 / 3.0),
    ]);
    let t2 = Polygon::from_slice(&[
        Vec2::new(25.0 / 6.0 + H, 4.0 / 3.0),
        Vec2::new(25.0 / 6.0 + S3, 11.0 / 6.0),
        Vec2::new(25.0 / 6.0 + H, 7.0 / 3.0),
    ]);
    let t3 = Polygon::from_slice(&[
        Vec2::new(13.0 / 6.0, 11.0 / 6.0),
        Vec2::new(8.0 / 3.0, 11.0 / 6.0 - H),
        Vec2::new(19.0 / 6.0, 11.0 / 6.0),
    ]);
    let t4 = Polygon::from_slice(&[
        Vec2::new(8.0 / 3.0, 11.0 / 6.0 - H),
        Vec2::new(11.0 / 3.0, 11.0 / 6.0 - H),
        Vec2::new(19.0 / 6.0, 11.0 / 6.0),
    ]);
    // Triangles, seed 2 (the 4-orbit): the stair-step gap triangles
    // one cell below the emission row.
    let g1 = Polygon::from_slice(&[
        Vec2::new(19.0 / 6.0, 5.0 / 6.0 - S3),
        Vec2::new(11.0 / 3.0, 5.0 / 6.0 - H),
        Vec2::new(8.0 / 3.0, 5.0 / 6.0 - H),
    ]);
    let g2 = Polygon::from_slice(&[
        Vec2::new(5.0 / 3.0 - H, 4.0 / 3.0 - H),
        Vec2::new(5.0 / 3.0, 5.0 / 6.0 - H),
        Vec2::new(5.0 / 3.0, 11.0 / 6.0 - H),
    ]);
    let g3 = Polygon::from_slice(&[
        Vec2::new(11.0 / 3.0 + H, 4.0 / 3.0 - H),
        Vec2::new(11.0 / 3.0, 11.0 / 6.0 - H),
        Vec2::new(11.0 / 3.0, 5.0 / 6.0 - H),
    ]);
    let g4 = Polygon::from_slice(&[
        Vec2::new(8.0 / 3.0, 11.0 / 6.0 - H),
        Vec2::new(13.0 / 6.0, 11.0 / 6.0),
        Vec2::new(5.0 / 3.0, 11.0 / 6.0 - H),
    ]);
    LatticeTiling {
        id: "kuniform-n17",
        name: "2-uniform (3³.4²; 3².4.3.4)₂",
        description: "2-uniform tiling (3³.4²; 3².4.3.4)₂ — variant 2 of the 33344-33434 tiling (the pgg sibling of n16): stair-stepped unit squares and triangles on a rectangular period lattice, four 2-fold centers per cell in two group orbits, one glide per axis, no reflections (pgg).",
        lattice: [
            Vec2::new(0.5, 1.0 + H),
            Vec2::new(-2.0 - S3, 1.0),
        ],
        cells: vec![
            Cell { poly: s1, seed: 0 },
            Cell { poly: s2, seed: 0 },
            Cell { poly: s3, seed: 0 },
            Cell { poly: s4, seed: 0 },
            Cell { poly: t1, seed: 1 },
            Cell { poly: t2, seed: 1 },
            Cell { poly: t3, seed: 1 },
            Cell { poly: t4, seed: 1 },
            Cell { poly: g1, seed: 2 },
            Cell { poly: g2, seed: 2 },
            Cell { poly: g3, seed: 2 },
            Cell { poly: g4, seed: 2 },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::Pgg,
        lattice_kind: LatticeKind::Rectangular,
        category: TilingCategory::TwoUniform,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
