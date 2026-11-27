//! Pavers family: paver bonds and the wallpaper-catalog completion batch (M78-M85):
//! the 14 bond/course/grid tables
//!
//! Owned here: module declarations, `register` (the family's registry
//! slice, in within-tab order), `classification` arms, the family's
//! lattice-culling entries and the family census pin. The family doc is
//! `docs/families/pavers.md`.

use tess_core::Registry;
use tess_core::classify::TilingClass;

pub mod basketweave;
pub mod herringbone;
pub mod running_bond;
pub mod stepped_hexominoes;
pub mod trapezoid_courses;
pub mod glide_hexagon_strips;
pub mod shifted_trapezoid_courses;
pub mod wavy_courses;
pub mod chiral_cut_honeycomb;
pub mod split_triangle_grid;
pub mod alcove_cut_grid;
pub mod stack_bond;
pub mod herringbone_45;
pub mod basketweave_3;

/// The family's registry slice, in within-tab order.
pub fn register(mut r: Registry) -> Registry {
        r = r.register(basketweave::basketweave());
        r = r.register(herringbone::herringbone());
        r = r.register(running_bond::running_bond());
        r = r.register(stepped_hexominoes::stepped_hexominoes());
        r = r.register(trapezoid_courses::trapezoid_courses());
        r = r.register(glide_hexagon_strips::glide_hexagon_strips());
        r = r.register(shifted_trapezoid_courses::shifted_trapezoid_courses());
        r = r.register(wavy_courses::wavy_courses());
        r = r.register(chiral_cut_honeycomb::chiral_cut_honeycomb());
        r = r.register(split_triangle_grid::split_triangle_grid());
        r = r.register(alcove_cut_grid::alcove_cut_grid());
        r = r.register(stack_bond::stack_bond());
        r = r.register(herringbone_45::herringbone_45());
        r = r.register(basketweave_3::basketweave_3());
    r
}

/// The family's classification arms (see `crate::classify` for the
/// composer and the shared constructor helpers).
pub(crate) fn classification(id: &str) -> Option<TilingClass> {
    // Arms are verbatim from the pre-split table (they yield
    // `TilingClass`); an unknown id returns None so the composer can
    // fall through to later families.
    Some(match id {
        "stepped-hexominoes" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("staircase hexomino"),
            cell_content: Some("1 hexomino per cell"),
            ..TilingClass::EMPTY
        },
        "trapezoid-courses" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("two incongruent trapezoids"),
            cell_content: Some("4 trapezoids per cell"),
            ..TilingClass::EMPTY
        },
        "glide-hexagon-strips" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("fat + waist hexagons"),
            cell_content: Some("2 hexagons per cell"),
            ..TilingClass::EMPTY
        },
        "shifted-trapezoid-courses" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("two incongruent trapezoids"),
            cell_content: Some("8 trapezoids per cell"),
            ..TilingClass::EMPTY
        },
        "wavy-courses" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("two incongruent triangles"),
            cell_content: Some("4 triangles per cell"),
            ..TilingClass::EMPTY
        },
        "split-triangle-grid" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("inner triangle + 3 corner quads"),
            cell_content: Some("8 tiles per cell"),
            ..TilingClass::EMPTY
        },
        "chiral-cut-honeycomb" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("chiral cut hexagon"),
            cell_content: Some("3 tiles per cell"),
            chirality: Some("homochiral"),
            ..TilingClass::EMPTY
        },
        "alcove-cut-grid" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("alcove-cut triangles"),
            cell_content: Some("18 tiles per cell"),
            ..TilingClass::EMPTY
        },
        "running-bond" => TilingClass {
            families: &["Arrangement"],
            method: Some("paver bond"),
            tile_set: Some("2:1 dominoes"),
            cell_content: Some("1 domino per cell"),
            ..TilingClass::EMPTY
        },
        "stack-bond" => TilingClass {
            families: &["Arrangement"],
            method: Some("paver bond"),
            tile_set: Some("2:1 dominoes"),
            cell_content: Some("1 domino per cell"),
            ..TilingClass::EMPTY
        },
        "herringbone-45" => TilingClass {
            families: &["Arrangement"],
            method: Some("paver bond"),
            tile_set: Some("2:1 dominoes"),
            cell_content: Some("2 dominoes per cell"),
            ..TilingClass::EMPTY
        },
        "basketweave-3" => TilingClass {
            families: &["Arrangement"],
            method: Some("paver bond"),
            tile_set: Some("2:1 dominoes"),
            cell_content: Some("15 dominoes per cell"),
            ..TilingClass::EMPTY
        },
        "basketweave" => TilingClass {
            families: &["Arrangement"],
            method: Some("paver bond"),
            tile_set: Some("2:1 dominoes"),
            cell_content: Some("4 dominoes per cell"),
            ..TilingClass::EMPTY
        },
        "herringbone" => TilingClass {
            families: &["Arrangement", "Wallpaper"],
            method: Some("paver bond"),
            tile_set: Some("2:1 dominoes"),
            cell_content: Some("2 dominoes per cell"),
            ..TilingClass::EMPTY
        },
        _ => return None,
    })
}

/// This family's entries for the global lattice-culling cross-check
/// (`lattice_culling_tests` in `lib.rs`).
#[cfg(test)]
pub(crate) fn lattice_tilings() -> Vec<(&'static str, crate::lattice::LatticeTiling)> {
    vec![
            ("basketweave", basketweave::basketweave()),
            ("herringbone", herringbone::herringbone()),
            ("running-bond", running_bond::running_bond()),
            ("stepped-hexominoes", stepped_hexominoes::stepped_hexominoes()),
            ("trapezoid-courses", trapezoid_courses::trapezoid_courses()),
            ("glide-hexagon-strips", glide_hexagon_strips::glide_hexagon_strips()),
            ("shifted-trapezoid-courses", shifted_trapezoid_courses::shifted_trapezoid_courses()),
            ("wavy-courses", wavy_courses::wavy_courses()),
            ("chiral-cut-honeycomb", chiral_cut_honeycomb::chiral_cut_honeycomb()),
            ("split-triangle-grid", split_triangle_grid::split_triangle_grid()),
            ("alcove-cut-grid", alcove_cut_grid::alcove_cut_grid()),
            ("stack-bond", stack_bond::stack_bond()),
            ("herringbone-45", herringbone_45::herringbone_45()),
            ("basketweave-3", basketweave_3::basketweave_3()),
    ]
}

#[cfg(test)]
mod census {
    use super::*;
    use std::collections::BTreeSet;

    /// The family's registry ids — by construction also its
    /// classification arm ids. Update alongside any membership change
    /// (this test fails on drift in either direction: a tiling
    /// registered but not classified, or classified but not
    /// registered).
    const IDS: &[&str] = &[
        "stepped-hexominoes",
        "trapezoid-courses",
        "glide-hexagon-strips",
        "shifted-trapezoid-courses",
        "wavy-courses",
        "split-triangle-grid",
        "chiral-cut-honeycomb",
        "alcove-cut-grid",
        "running-bond",
        "stack-bond",
        "herringbone-45",
        "basketweave-3",
        "basketweave",
        "herringbone",
    ];

    #[test]
    fn family_census_matches() {
        let r = register(Registry::new());
        assert_eq!(
            r.len(),
            14,
            "pavers family census — update when adding a tiling to this family"
        );
        let have: BTreeSet<&str> = r.iter().map(|t| t.id()).collect();
        let want: BTreeSet<&str> = IDS.iter().copied().collect();
        assert_eq!(
            have, want,
            "pavers family membership drift: register() vs IDS"
        );
        for id in IDS {
            assert!(
                classification(id).is_some(),
                "{id} has no classification arm in this family"
            );
        }
    }
}

/// Brute-force cross-check of [`crate::lattice::LatticeTiling`]'s
/// index-range culling, the one part of the generator that could silently
/// drop (or invent) tiles at the edges of the query region: the per-cell
/// offsets that pad the index ranges are measured in lattice coordinates
/// (cell vertices mapped through `u(p) = A⁻¹ p`), and getting that wrong
/// for a sheared or tall cell would produce gaps that sample-based
/// coverage tests can miss. This test re-emits every tile from the public
/// table data over a wide fixed index window — no range math at all — and
/// requires exactly the same tile set as `generate`.
/// Exact wallpaper-group probe for the paver data tables.
///
/// All paver geometry is integral, and every candidate isometry
/// `g = (M, v)` we want to test has `M` with {0, ±1} entries and `v` a
/// half-integer vector, so membership can be tested exactly in
/// *doubled* coordinates (world point `p` ↔ `P = 2p`). A candidate is
/// a symmetry iff `M` maps the lattice basis into the domino-origin
/// lattice `L`, and for every unit-cell domino the image domino's
/// origin `t` (the min corner of the image, i.e. `M·o + c0 + v` with
/// `c0` the min corner of `M([0,w]×[0,h])`) lands in that kind's
/// origin-coset set `O(kind)` — the rigorous coset check of the
/// Python spike. All coordinates below are doubled: the lattice basis,
/// the cell origins/extents (`kind` 0 = horizontal, 1 = vertical),
/// and the translations `V = 2v`.
#[cfg(all(test, feature = "pavers"))]
fn pavers_exact_is_sym(
    m: [i64; 4], // M = [[m[0], m[1]], [m[2], m[3]]]
    v: [i64; 2], // doubled translation V = 2v
    basis: &[(i64, i64); 2],
    cells: &[(i64, i64, i64, i64, u8)],
    in_lattice: impl Fn(i64, i64) -> bool,
    in_origin: impl Fn(u8, i64, i64) -> bool,
) -> bool {
    let mv = |p: (i64, i64)| (m[0] * p.0 + m[1] * p.1, m[2] * p.0 + m[3] * p.1);
    // M must map the lattice to itself; all candidates have det ±1, so
    // checking the basis vectors suffices.
    for &(bx, by) in basis {
        let (x, y) = mv((bx, by));
        if !in_lattice(x, y) {
            return false;
        }
    }
    for &(ox, oy, w, h, _kind) in cells {
        let cs = [mv((0, 0)), mv((w, 0)), mv((0, h)), mv((w, h))];
        let c0x = cs.iter().map(|c| c.0).min().unwrap();
        let c0y = cs.iter().map(|c| c.1).min().unwrap();
        let dx = cs.iter().map(|c| c.0).max().unwrap() - c0x;
        let dy = cs.iter().map(|c| c.1).max().unwrap() - c0y;
        let kind2 = u8::from(dx == 2 && dy == 4); // image domino kind: 2×4 (doubled) = V
        let t = (
            m[0] * ox + m[1] * oy + c0x + v[0],
            m[2] * ox + m[3] * oy + c0y + v[1],
        );
        if !in_origin(kind2, t.0, t.1) {
            return false;
        }
    }
    true
}

/// Exact shape-set wallpaper-group probe for the rotated (non
/// axis-aligned) paver data tables.
///
/// `pavers_exact_is_sym` keys dominoes by their axis-aligned bbox,
/// which breaks once a cell is rotated 45°. This probe works with
/// bare vertex sets instead: a candidate isometry `g = (M, v)` is a
/// symmetry iff `M` maps the lattice basis into the tiling lattice
/// and, for every source shape `S` in the cell list, the image
/// `M·S + v` equals some target shape `T` translated by a lattice
/// vector `p` — `p` is determined as the min-corner anchor (unique:
/// a bounded shape has no nonzero self-translation). All coordinates
/// are the table's integer P-coords, where the lattice basis, the
/// shape vertices, and the translations are all integral.
#[cfg(all(test, feature = "pavers"))]
fn shape_set_is_sym(
    m: [i64; 4], // M = [[m[0], m[1]], [m[2], m[3]]]
    v: [i64; 2],
    basis: &[(i64, i64); 2],
    shapes: &[[[i64; 2]; 6]],
    in_lattice: impl Fn(i64, i64) -> bool,
) -> bool {
    let mv = |p: [i64; 2]| [
        m[0] * p[0] + m[1] * p[1],
        m[2] * p[0] + m[3] * p[1],
    ];
    // M must map the lattice to itself; all candidates have det ±1, so
    // checking the basis vectors suffices.
    for &(bx, by) in basis {
        let q = mv([bx, by]);
        if !in_lattice(q[0], q[1]) {
            return false;
        }
    }
    for s in shapes {
        let img: Vec<[i64; 2]> = s
            .iter()
            .map(|p| {
                let q = mv(*p);
                [q[0] + v[0], q[1] + v[1]]
            })
            .collect();
        let img_minx = img.iter().map(|p| p[0]).min().unwrap();
        let img_miny = img.iter().map(|p| p[1]).min().unwrap();
        let mut matched = false;
        for t in shapes {
            let t_minx = t.iter().map(|p| p[0]).min().unwrap();
            let t_miny = t.iter().map(|p| p[1]).min().unwrap();
            let px = img_minx - t_minx;
            let py = img_miny - t_miny;
            if !in_lattice(px, py) {
                continue;
            }
            let mut a = img.clone();
            let mut b: Vec<[i64; 2]> =
                t.iter().map(|p| [p[0] + px, p[1] + py]).collect();
            a.sort_unstable();
            b.sort_unstable();
            if a == b {
                matched = true;
                break;
            }
        }
        if !matched {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod pavers_tests {
    use super::pavers_exact_is_sym;
    use super::shape_set_is_sym;
    use crate::tests::{check_covers, check_det, check_rotated};
    use crate::basketweave;
    use crate::herringbone;
    use crate::glide_hexagon_strips;
    use crate::running_bond;
    use crate::shifted_trapezoid_courses;
    use crate::stepped_hexominoes;
    use crate::trapezoid_courses;
    use crate::wavy_courses;
    use crate::alcove_cut_grid;
    use crate::chiral_cut_honeycomb;
    use crate::split_triangle_grid;
    use crate::stack_bond;
    use crate::herringbone_45;
    use crate::basketweave_3;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::tiling::{Query, Tiling};

    /// Candidate matrices (row-major) used by the paver group probes.
    const M_ROT4: [i64; 4] = [0, -1, 1, 0];
    const M_ROT2: [i64; 4] = [-1, 0, 0, -1];
    const M_MIRX: [i64; 4] = [-1, 0, 0, 1]; // mirror of x = c: (2c−x, y)
    const M_MIRY: [i64; 4] = [1, 0, 0, -1]; // mirror of y = c: (x, 2c−y)
    const M_DIAG: [i64; 4] = [0, 1, 1, 0]; // glide/mirror of y = x + c
    const M_ANTI: [i64; 4] = [0, -1, -1, 0]; // glide/mirror of y = −x + c

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn basketweave_covers_region() {
        check_covers(
            &basketweave::basketweave(),
            "basketweave",
            BBox::new(v2(-3.0, -3.0), v2(11.0, 9.0)),
        );
    }

    #[test]
    fn basketweave_rotated_scaled() {
        check_rotated(
            &basketweave::basketweave(),
            "basketweave(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn basketweave_deterministic() {
        check_det(
            &basketweave::basketweave(),
            "basketweave",
            BBox::new(v2(-2.0, -2.0), v2(8.0, 6.0)),
        );
    }

    /// Lattice-membership / origin-coset predicates for the basketweave
    /// table, in doubled coordinates. L = {x,y even, x+y ≡ 0 (mod 4)};
    /// H origins = L ∪ (L + (0,1)), V origins = L + (2,0) ∪ L + (3,0).
    fn bw_in_lattice(x: i64, y: i64) -> bool {
        x.rem_euclid(4) == 0 && y.rem_euclid(4) == 0 && (x + y).rem_euclid(8) == 0
    }

    fn bw_in_origin(kind: u8, x: i64, y: i64) -> bool {
        match kind {
            0 => bw_in_lattice(x, y) || bw_in_lattice(x, y - 2),
            _ => bw_in_lattice(x - 4, y) || bw_in_lattice(x - 6, y),
        }
    }

    #[test]
    fn basketweave_wallpaper_group_is_p4g() {
        // Unit-cell dominoes in doubled coordinates: H (0,0), H (0,1),
        // V (2,0), V (3,0); kind 0 = H, 1 = V.
        let basis = [(4, 4), (4, -4)];
        let cells: [(i64, i64, i64, i64, u8); 4] =
            [(0, 0, 4, 2, 0), (0, 2, 4, 2, 0), (4, 0, 2, 4, 1), (6, 0, 2, 4, 1)];
        let sym = |m: [i64; 4], v: [i64; 2]| {
            pavers_exact_is_sym(m, v, &basis, &cells, bw_in_lattice, bw_in_origin)
        };

        // 4-folds at every even-even point; 2-folds at odd-odd;
        // mirrors at the odd axes; six glide families.
        let expected = [
            (M_ROT4, [0, 0], "rot4(0,0)"),
            (M_ROT4, [4, -4], "rot4(2,0)"),
            (M_ROT4, [4, 4], "rot4(0,2)"),
            (M_ROT4, [8, 0], "rot4(2,2)"),
            (M_ROT4, [8, -8], "rot4(4,0)"),
            (M_ROT4, [8, 8], "rot4(0,4)"),
            (M_ROT4, [12, -4], "rot4(4,2)"),
            (M_ROT4, [16, 0], "rot4(4,4)"),
            (M_ROT2, [4, 4], "rot2(1,1)"),
            (M_ROT2, [12, 12], "rot2(3,3)"),
            (M_ROT2, [4, 12], "rot2(1,3)"),
            (M_ROT2, [12, 4], "rot2(3,1)"),
            (M_MIRX, [4, 0], "mir_x(1)"),
            (M_MIRX, [12, 0], "mir_x(3)"),
            (M_MIRY, [0, 4], "mir_y(1)"),
            (M_MIRY, [0, 12], "mir_y(3)"),
            (M_DIAG, [0, 4], "glide(y, x+2)"),
            (M_DIAG, [4, 0], "glide(y+2, x)"),
            (M_ANTI, [4, 0], "glide(2-y, -x)"),
            (M_ANTI, [8, 4], "glide(4-y, 2-x)"),
            (M_MIRX, [0, 4], "glide(-x, y+2)"),
            (M_MIRY, [4, 0], "glide(x+2, -y)"),
        ];
        for (m, v, label) in expected {
            assert!(
                sym(m, v),
                "{label} must be a symmetry of the p4g basketweave"
            );
        }

        // The rest of the candidate families are absent: this is what
        // pins p4g (vs p4m / cmm / pgg).
        let forbidden = [
            // diagonal mirrors, integer and half-integer offsets
            (M_DIAG, [0, 0], "mir_d(0)"),
            (M_DIAG, [-2, 2], "mir_d(1)"),
            (M_DIAG, [-4, 4], "mir_d(2)"),
            (M_DIAG, [-6, 6], "mir_d(3)"),
            (M_DIAG, [-8, 8], "mir_d(4)"),
            (M_ANTI, [0, 0], "mir_a(0)"),
            (M_ANTI, [2, 2], "mir_a(1)"),
            (M_ANTI, [4, 4], "mir_a(2)"),
            (M_ANTI, [6, 6], "mir_a(3)"),
            (M_ANTI, [8, 8], "mir_a(4)"),
            (M_DIAG, [-1, 1], "mir_dh(1/2)"),
            (M_DIAG, [-3, 3], "mir_dh(3/2)"),
            (M_DIAG, [-5, 5], "mir_dh(5/2)"),
            (M_DIAG, [-7, 7], "mir_dh(7/2)"),
            (M_DIAG, [-9, 9], "mir_dh(9/2)"),
            (M_ANTI, [1, 1], "mir_ah(1/2)"),
            (M_ANTI, [3, 3], "mir_ah(3/2)"),
            (M_ANTI, [5, 5], "mir_ah(5/2)"),
            (M_ANTI, [7, 7], "mir_ah(7/2)"),
            (M_ANTI, [9, 9], "mir_ah(9/2)"),
            // even-axis and half-integer mirrors
            (M_MIRX, [0, 0], "mir_x(0)"),
            (M_MIRX, [8, 0], "mir_x(2)"),
            (M_MIRX, [16, 0], "mir_x(4)"),
            (M_MIRY, [0, 0], "mir_y(0)"),
            (M_MIRY, [0, 8], "mir_y(2)"),
            (M_MIRY, [0, 16], "mir_y(4)"),
            (M_MIRX, [2, 0], "mir_xhalf(1/2)"),
            (M_MIRX, [6, 0], "mir_xhalf(3/2)"),
            (M_MIRX, [10, 0], "mir_xhalf(5/2)"),
            (M_MIRX, [14, 0], "mir_xhalf(7/2)"),
            (M_MIRX, [18, 0], "mir_xhalf(9/2)"),
            (M_MIRY, [0, 2], "mir_yhalf(1/2)"),
            (M_MIRY, [0, 6], "mir_yhalf(3/2)"),
            (M_MIRY, [0, 10], "mir_yhalf(5/2)"),
            (M_MIRY, [0, 14], "mir_yhalf(7/2)"),
            (M_MIRY, [0, 18], "mir_yhalf(9/2)"),
            // 4-folds away from even-even centers
            (M_ROT4, [4, 0], "rot4(1,1)"),
            (M_ROT4, [12, 0], "rot4(3,3)"),
            (M_ROT4, [8, 4], "rot4(1,3)"),
            (M_ROT4, [8, -4], "rot4(3,1)"),
            (M_ROT4, [2, 0], "rot4(1/2,1/2)"),
            (M_ROT4, [2, -2], "rot4(0,1)"),
            // odd-axis vertical/horizontal glides (T(2,0), T(0,2) are
            // not tiling translations)
            (M_MIRX, [4, 4], "glide(2-x, y+2)"),
            (M_MIRY, [4, 4], "glide(x+2, 2-y)"),
        ];
        for (m, v, label) in forbidden {
            assert!(
                !sym(m, v),
                "{label} must NOT be a symmetry of the basketweave"
            );
        }
    }

    #[test]
    fn herringbone_covers_region() {
        check_covers(
            &herringbone::herringbone(),
            "herringbone",
            BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
        );
    }

    #[test]
    fn herringbone_rotated_scaled() {
        check_rotated(
            &herringbone::herringbone(),
            "herringbone(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn herringbone_deterministic() {
        check_det(
            &herringbone::herringbone(),
            "herringbone",
            BBox::new(v2(-4.0, -4.0), v2(8.0, 4.0)),
        );
    }

    /// Lattice-membership / origin-coset predicates for the herringbone
    /// table, in doubled coordinates. L = ⟨(1,3),(2,2)⟩ = {(x,y): y−x
    /// even, 3x−y ≡ 0 (mod 4)}; H origins = L, V origins = L + (2,0).
    fn hb_in_lattice(x: i64, y: i64) -> bool {
        (y - x).rem_euclid(4) == 0 && (3 * x - y).rem_euclid(8) == 0
    }

    fn hb_in_origin(kind: u8, x: i64, y: i64) -> bool {
        match kind {
            0 => hb_in_lattice(x, y),
            _ => hb_in_lattice(x - 4, y),
        }
    }

    #[test]
    fn herringbone_wallpaper_group_is_pgg() {
        // Unit-cell dominoes in doubled coordinates: H (0,0), V (2,0);
        // kind 0 = H, 1 = V.
        let basis = [(2, 6), (4, 4)];
        let cells: [(i64, i64, i64, i64, u8); 2] =
            [(0, 0, 4, 2, 0), (4, 0, 2, 4, 1)];
        let sym = |m: [i64; 4], v: [i64; 2]| {
            pavers_exact_is_sym(m, v, &basis, &cells, hb_in_lattice, hb_in_origin)
        };

        // 16 2-fold centers mod 4, two perpendicular glide families.
        let expected = [
            (M_ROT2, [0, 6], "rot2(0,3/2)"),
            (M_ROT2, [0, 14], "rot2(0,7/2)"),
            (M_ROT2, [4, 2], "rot2(1,1/2)"),
            (M_ROT2, [4, 10], "rot2(1,5/2)"),
            (M_ROT2, [2, 4], "rot2(1/2,1)"),
            (M_ROT2, [2, 12], "rot2(1/2,3)"),
            (M_ROT2, [8, 6], "rot2(2,3/2)"),
            (M_ROT2, [8, 14], "rot2(2,7/2)"),
            (M_ROT2, [12, 2], "rot2(3,1/2)"),
            (M_ROT2, [12, 10], "rot2(3,5/2)"),
            (M_ROT2, [6, 0], "rot2(3/2,0)"),
            (M_ROT2, [6, 8], "rot2(3/2,2)"),
            (M_ROT2, [10, 4], "rot2(5/2,1)"),
            (M_ROT2, [10, 12], "rot2(5/2,3)"),
            (M_ROT2, [14, 0], "rot2(7/2,0)"),
            (M_ROT2, [14, 8], "rot2(7/2,2)"),
            // diagonal glide family (axes y = x + c)
            (M_DIAG, [2, 2], "glide(y+1, x+1)"),
            (M_DIAG, [0, 4], "glide(y, x+2)"),
            (M_DIAG, [-2, 6], "glide(y-1, x+3)"),
            (M_DIAG, [-4, 8], "glide(y-2, x+4)"),
            // anti-diagonal glide family (axes y = -x + c)
            (M_ANTI, [2, 0], "glide(1-y, -x)"),
            (M_ANTI, [-2, -4], "glide(-1-y, -x-2)"),
        ];
        for (m, v, label) in expected {
            assert!(
                sym(m, v),
                "{label} must be a symmetry of the pgg herringbone"
            );
        }

        // No mirrors of any family, no 4-folds, no off-family glides,
        // and 2-folds only at the 16 listed centers: this is what
        // pins pgg (vs cmm / p4g / p4m).
        let forbidden = [
            // horizontal/vertical mirrors, integer and half-integer axes
            (M_MIRX, [0, 0], "mir_x(0)"),
            (M_MIRX, [4, 0], "mir_x(1)"),
            (M_MIRX, [8, 0], "mir_x(2)"),
            (M_MIRX, [12, 0], "mir_x(3)"),
            (M_MIRX, [16, 0], "mir_x(4)"),
            (M_MIRY, [0, 0], "mir_y(0)"),
            (M_MIRY, [0, 4], "mir_y(1)"),
            (M_MIRY, [0, 8], "mir_y(2)"),
            (M_MIRY, [0, 12], "mir_y(3)"),
            (M_MIRY, [0, 16], "mir_y(4)"),
            (M_MIRX, [2, 0], "mir_xhalf(1/2)"),
            (M_MIRX, [6, 0], "mir_xhalf(3/2)"),
            (M_MIRX, [10, 0], "mir_xhalf(5/2)"),
            (M_MIRX, [14, 0], "mir_xhalf(7/2)"),
            (M_MIRX, [18, 0], "mir_xhalf(9/2)"),
            (M_MIRY, [0, 2], "mir_yhalf(1/2)"),
            (M_MIRY, [0, 6], "mir_yhalf(3/2)"),
            (M_MIRY, [0, 10], "mir_yhalf(5/2)"),
            (M_MIRY, [0, 14], "mir_yhalf(7/2)"),
            (M_MIRY, [0, 18], "mir_yhalf(9/2)"),
            // diagonal mirrors, integer and half-integer offsets
            (M_DIAG, [0, 0], "mir_d(0)"),
            (M_DIAG, [-2, 2], "mir_d(1)"),
            (M_DIAG, [-4, 4], "mir_d(2)"),
            (M_DIAG, [-6, 6], "mir_d(3)"),
            (M_DIAG, [-8, 8], "mir_d(4)"),
            (M_DIAG, [-1, 1], "mir_dh(1/2)"),
            (M_DIAG, [-3, 3], "mir_dh(3/2)"),
            (M_DIAG, [-5, 5], "mir_dh(5/2)"),
            (M_DIAG, [-7, 7], "mir_dh(7/2)"),
            (M_ANTI, [0, 0], "mir_a(0)"),
            (M_ANTI, [2, 2], "mir_a(1)"),
            (M_ANTI, [4, 4], "mir_a(2)"),
            (M_ANTI, [6, 6], "mir_a(3)"),
            (M_ANTI, [8, 8], "mir_a(4)"),
            (M_ANTI, [1, 1], "mir_ah(1/2)"),
            (M_ANTI, [3, 3], "mir_ah(3/2)"),
            (M_ANTI, [5, 5], "mir_ah(5/2)"),
            (M_ANTI, [7, 7], "mir_ah(7/2)"),
            // off-family glides
            (M_ANTI, [2, 2], "glide(1-y, 1-x)"),
            (M_MIRX, [0, 2], "glide(-x, y+1)"),
            (M_MIRY, [2, 0], "glide(x+1, -y)"),
            // no 4-folds
            (M_ROT4, [0, 0], "rot4(0,0)"),
            (M_ROT4, [4, 0], "rot4(1,1)"),
            (M_ROT4, [8, 0], "rot4(2,2)"),
            // 2-folds only at the 16 expected centers
            (M_ROT2, [0, 0], "rot2(0,0)"),
            (M_ROT2, [4, 4], "rot2(1,1)"),
            (M_ROT2, [0, 4], "rot2(0,1)"),
            (M_ROT2, [4, 12], "rot2(1,3)"),
            (M_ROT2, [8, 4], "rot2(2,1)"),
            (M_ROT2, [12, 12], "rot2(3,3)"),
            (M_ROT2, [0, 10], "rot2(0,5/2)"),
            (M_ROT2, [2, 2], "rot2(1/2,1/2)"),
            (M_ROT2, [6, 6], "rot2(3/2,3/2)"),
            (M_ROT2, [10, 10], "rot2(5/2,5/2)"),
            (M_ROT2, [14, 14], "rot2(7/2,7/2)"),
        ];
        for (m, v, label) in forbidden {
            assert!(
                !sym(m, v),
                "{label} must NOT be a symmetry of the herringbone"
            );
        }
    }

    #[test]
    fn running_bond_covers_region() {
        check_covers(
            &running_bond::running_bond(),
            "running-bond",
            BBox::new(v2(-4.0, -4.0), v2(8.0, 6.0)),
        );
    }

    #[test]
    fn running_bond_rotated_scaled() {
        check_rotated(
            &running_bond::running_bond(),
            "running-bond(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn running_bond_deterministic() {
        check_det(
            &running_bond::running_bond(),
            "running-bond",
            BBox::new(v2(-3.0, -3.0), v2(9.0, 5.0)),
        );
    }

    /// Lattice-membership / origin-set predicates for the running-bond
    /// table, in doubled coordinates. L = 2ℤ²; domino origins = L ∪
    /// (L + (1,1)) (all horizontal — there are no vertical dominoes).
    fn rb_in_lattice(x: i64, y: i64) -> bool {
        x.rem_euclid(4) == 0 && y.rem_euclid(4) == 0
    }

    fn rb_in_origin(kind: u8, x: i64, y: i64) -> bool {
        match kind {
            0 => rb_in_lattice(x, y) || rb_in_lattice(x - 2, y - 2),
            _ => false, // no vertical dominoes in the running bond
        }
    }

    #[test]
    fn running_bond_wallpaper_group_is_cmm() {
        // Unit-cell dominoes in doubled coordinates: H (0,0),
        // H (1,1); kind 0 = H.
        let basis = [(4, 0), (0, 4)];
        let cells: [(i64, i64, i64, i64, u8); 2] =
            [(0, 0, 4, 2, 0), (2, 2, 4, 2, 0)];
        let sym = |m: [i64; 4], v: [i64; 2]| {
            pavers_exact_is_sym(m, v, &basis, &cells, rb_in_lattice, rb_in_origin)
        };

        // Mirrors at the integer x-axes and half-integer y-axes;
        // 2-folds at (k, l+1/2) on the mirrors and at (k+1/2, l)
        // between the rows; two glide families per direction.
        let expected = [
            (M_MIRX, [0, 0], "mir_x(0)"),
            (M_MIRX, [4, 0], "mir_x(1)"),
            (M_MIRX, [8, 0], "mir_x(2)"),
            (M_MIRX, [12, 0], "mir_x(3)"),
            (M_MIRX, [16, 0], "mir_x(4)"),
            (M_MIRY, [0, 2], "mir_yhalf(1/2)"),
            (M_MIRY, [0, 6], "mir_yhalf(3/2)"),
            (M_MIRY, [0, 10], "mir_yhalf(5/2)"),
            (M_MIRY, [0, 14], "mir_yhalf(7/2)"),
            (M_MIRY, [0, 18], "mir_yhalf(9/2)"),
            // 2-folds on the mirrors: (k, l+1/2)
            (M_ROT2, [0, 2], "rot2(0,1/2)"),
            (M_ROT2, [0, 6], "rot2(0,3/2)"),
            (M_ROT2, [0, 10], "rot2(0,5/2)"),
            (M_ROT2, [0, 14], "rot2(0,7/2)"),
            (M_ROT2, [4, 2], "rot2(1,1/2)"),
            (M_ROT2, [4, 6], "rot2(1,3/2)"),
            (M_ROT2, [4, 10], "rot2(1,5/2)"),
            (M_ROT2, [4, 14], "rot2(1,7/2)"),
            (M_ROT2, [8, 2], "rot2(2,1/2)"),
            (M_ROT2, [8, 6], "rot2(2,3/2)"),
            (M_ROT2, [8, 10], "rot2(2,5/2)"),
            (M_ROT2, [8, 14], "rot2(2,7/2)"),
            (M_ROT2, [12, 2], "rot2(3,1/2)"),
            (M_ROT2, [12, 6], "rot2(3,3/2)"),
            (M_ROT2, [12, 10], "rot2(3,5/2)"),
            (M_ROT2, [12, 14], "rot2(3,7/2)"),
            (M_ROT2, [16, 2], "rot2(4,1/2)"),
            (M_ROT2, [16, 6], "rot2(4,3/2)"),
            (M_ROT2, [16, 10], "rot2(4,5/2)"),
            (M_ROT2, [16, 14], "rot2(4,7/2)"),
            // 2-folds between the rows: (k+1/2, l)
            (M_ROT2, [2, 0], "rot2(1/2,0)"),
            (M_ROT2, [2, 4], "rot2(1/2,1)"),
            (M_ROT2, [2, 8], "rot2(1/2,2)"),
            (M_ROT2, [2, 12], "rot2(1/2,3)"),
            (M_ROT2, [2, 16], "rot2(1/2,4)"),
            (M_ROT2, [6, 0], "rot2(3/2,0)"),
            (M_ROT2, [6, 4], "rot2(3/2,1)"),
            (M_ROT2, [6, 8], "rot2(3/2,2)"),
            (M_ROT2, [6, 12], "rot2(3/2,3)"),
            (M_ROT2, [6, 16], "rot2(3/2,4)"),
            (M_ROT2, [10, 0], "rot2(5/2,0)"),
            (M_ROT2, [10, 4], "rot2(5/2,1)"),
            (M_ROT2, [10, 8], "rot2(5/2,2)"),
            (M_ROT2, [10, 12], "rot2(5/2,3)"),
            (M_ROT2, [10, 16], "rot2(5/2,4)"),
            (M_ROT2, [14, 0], "rot2(7/2,0)"),
            (M_ROT2, [14, 4], "rot2(7/2,1)"),
            (M_ROT2, [14, 8], "rot2(7/2,2)"),
            (M_ROT2, [14, 12], "rot2(7/2,3)"),
            (M_ROT2, [14, 16], "rot2(7/2,4)"),
            // two glide families per direction
            (M_MIRY, [2, 0], "glide(x+1, -y)"),
            (M_MIRY, [2, 8], "glide(x+1, 4-y)"),
            (M_MIRX, [2, 2], "glide(1-x, y+1)"),
            (M_MIRX, [6, 2], "glide(3-x, y+1)"),
        ];
        for (m, v, label) in expected {
            assert!(
                sym(m, v),
                "{label} must be a symmetry of the cmm running bond"
            );
        }

        // No diagonal mirrors, no 4-folds, no integer-axis y-mirrors,
        // no off-axis glides, and 2-folds only at the two center
        // families: this is what pins cmm (vs cmm' / pmm / p4g).
        let forbidden = [
            // diagonal mirrors, integer and half-integer offsets
            (M_DIAG, [0, 0], "mir_d(0)"),
            (M_DIAG, [-2, 2], "mir_d(1)"),
            (M_DIAG, [-4, 4], "mir_d(2)"),
            (M_DIAG, [-6, 6], "mir_d(3)"),
            (M_DIAG, [-8, 8], "mir_d(4)"),
            (M_DIAG, [-1, 1], "mir_dh(1/2)"),
            (M_DIAG, [-3, 3], "mir_dh(3/2)"),
            (M_DIAG, [-5, 5], "mir_dh(5/2)"),
            (M_DIAG, [-7, 7], "mir_dh(7/2)"),
            (M_ANTI, [0, 0], "mir_a(0)"),
            (M_ANTI, [2, 2], "mir_a(1)"),
            (M_ANTI, [4, 4], "mir_a(2)"),
            (M_ANTI, [6, 6], "mir_a(3)"),
            (M_ANTI, [8, 8], "mir_a(4)"),
            (M_ANTI, [1, 1], "mir_ah(1/2)"),
            (M_ANTI, [3, 3], "mir_ah(3/2)"),
            (M_ANTI, [5, 5], "mir_ah(5/2)"),
            (M_ANTI, [7, 7], "mir_ah(7/2)"),
            // integer-axis y-mirrors and half-integer x-mirrors
            (M_MIRY, [0, 0], "mir_y(0)"),
            (M_MIRY, [0, 4], "mir_y(1)"),
            (M_MIRY, [0, 8], "mir_y(2)"),
            (M_MIRY, [0, 12], "mir_y(3)"),
            (M_MIRY, [0, 16], "mir_y(4)"),
            (M_MIRX, [2, 0], "mir_xhalf(1/2)"),
            (M_MIRX, [6, 0], "mir_xhalf(3/2)"),
            (M_MIRX, [10, 0], "mir_xhalf(5/2)"),
            (M_MIRX, [14, 0], "mir_xhalf(7/2)"),
            (M_MIRX, [18, 0], "mir_xhalf(9/2)"),
            // 2-folds off the two center families
            (M_ROT2, [0, 0], "rot2(0,0)"),
            (M_ROT2, [4, 4], "rot2(1,1)"),
            (M_ROT2, [8, 8], "rot2(2,2)"),
            (M_ROT2, [12, 12], "rot2(3,3)"),
            (M_ROT2, [16, 16], "rot2(4,4)"),
            (M_ROT2, [2, 2], "rot2(1/2,1/2)"),
            (M_ROT2, [6, 6], "rot2(3/2,3/2)"),
            (M_ROT2, [10, 10], "rot2(5/2,5/2)"),
            (M_ROT2, [14, 14], "rot2(7/2,7/2)"),
            // no 4-folds
            (M_ROT4, [0, 0], "rot4(0,0)"),
            (M_ROT4, [4, 0], "rot4(1,1)"),
            // off-axis glides
            (M_MIRY, [2, 2], "glide(x+1, 1-y)"),
            (M_MIRX, [0, 2], "glide(-x, y+1)"),
        ];
        for (m, v, label) in forbidden {
            assert!(
                !sym(m, v),
                "{label} must NOT be a symmetry of the running bond"
            );
        }
    }

    /// Stack-bond (M114) standard trio.
    #[test]
    fn stack_bond_covers_region() {
        check_covers(
            &stack_bond::stack_bond(),
            "stack-bond",
            BBox::new(v2(-3.0, -2.0), v2(9.0, 6.0)),
        );
    }

    #[test]
    fn stack_bond_rotated_scaled() {
        check_rotated(
            &stack_bond::stack_bond(),
            "stack-bond(rot30,scale0.7)",
            BBox::new(v2(-4.0, -4.0), v2(6.0, 6.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn stack_bond_deterministic() {
        check_det(
            &stack_bond::stack_bond(),
            "stack-bond",
            BBox::new(v2(-2.0, -1.0), v2(8.0, 5.0)),
        );
    }

    /// Lattice-membership / origin-set predicates for the stack-bond
    /// table, in doubled coordinates. L = <(4,0),(0,2)>; domino
    /// origins = L (all horizontal — there are no vertical dominoes).
    fn sb_in_lattice(x: i64, y: i64) -> bool {
        x.rem_euclid(4) == 0 && y.rem_euclid(2) == 0
    }

    fn sb_in_origin(kind: u8, x: i64, y: i64) -> bool {
        match kind {
            0 => sb_in_lattice(x, y),
            _ => false, // no vertical dominoes in the stack bond
        }
    }

    #[test]
    fn stack_bond_wallpaper_group_is_pmm() {
        // Unit-cell domino in doubled coordinates: H (0,0);
        // kind 0 = H.
        let basis = [(4, 0), (0, 2)];
        let cells: [(i64, i64, i64, i64, u8); 1] = [(0, 0, 4, 2, 0)];
        let sym = |m: [i64; 4], v: [i64; 2]| {
            pavers_exact_is_sym(m, v, &basis, &cells, sb_in_lattice, sb_in_origin)
        };

        // Mirrors at the integer x-axes and the half-integer
        // y-axes (row boundaries and row midlines); 2-fold centers at
        // every mirror intersection (k, n/2).
        let expected = [
            (M_MIRX, [0, 0], "mir_x(0)"),
            (M_MIRX, [4, 0], "mir_x(1)"),
            (M_MIRX, [8, 0], "mir_x(2)"),
            (M_MIRX, [-4, 0], "mir_x(-1)"),
            (M_MIRX, [-8, 0], "mir_x(-2)"),
            (M_MIRY, [0, 0], "mir_y(0)"),
            (M_MIRY, [0, 2], "mir_y(1/2)"),
            (M_MIRY, [0, 4], "mir_y(1)"),
            (M_MIRY, [0, 6], "mir_y(3/2)"),
            (M_MIRY, [0, -2], "mir_y(-1/2)"),
            (M_MIRY, [0, -4], "mir_y(-1)"),
            // 2-folds at the mirror intersections (k, n/2)
            (M_ROT2, [0, 0], "rot2(0,0)"),
            (M_ROT2, [4, 0], "rot2(1,0)"),
            (M_ROT2, [0, 2], "rot2(0,1/2)"),
            (M_ROT2, [4, 2], "rot2(1,1/2)"),
            (M_ROT2, [8, 4], "rot2(2,1)"),
            (M_ROT2, [-4, 6], "rot2(-1,3/2)"),
            (M_ROT2, [0, -2], "rot2(0,-1/2)"),
            (M_ROT2, [12, -4], "rot2(3,-1)"),
        ];
        for (m, v, label) in expected {
            assert!(
                sym(m, v),
                "{label} must be a symmetry of the pmm stack bond"
            );
        }

        // No half-integer x-mirrors, no y-mirrors off y = n/2, no
        // 2-folds off the (k, n/2) grid, no 4-folds, no diagonal
        // mirrors, no glides: this is what pins pmm (vs cmm / p4m /
        // pgg).
        let forbidden = [
            // half-integer x-axes are not mirrors
            (M_MIRX, [2, 0], "mir_xhalf(1/2)"),
            (M_MIRX, [6, 0], "mir_xhalf(3/2)"),
            (M_MIRX, [-2, 0], "mir_xhalf(-1/2)"),
            // y-mirrors only at y = n/2
            (M_MIRY, [0, 1], "mir_y(1/4)"),
            (M_MIRY, [0, 3], "mir_y(3/4)"),
            (M_MIRY, [0, 5], "mir_y(5/4)"),
            // 2-folds only at the (k, n/2) intersections
            (M_ROT2, [2, 0], "rot2(1/2,0)"),
            (M_ROT2, [2, 2], "rot2(1/2,1/2)"),
            (M_ROT2, [6, 2], "rot2(3/2,1/2)"),
            (M_ROT2, [0, 1], "rot2(0,1/4)"),
            (M_ROT2, [4, 1], "rot2(1,1/4)"),
            // no 4-folds
            (M_ROT4, [0, 0], "rot4(0,0)"),
            (M_ROT4, [4, 0], "rot4(1,0)"),
            (M_ROT4, [0, 2], "rot4(0,1/2)"),
            // no diagonal mirrors
            (M_DIAG, [0, 0], "mir_d(0)"),
            (M_DIAG, [4, 0], "mir_d(1)"),
            (M_DIAG, [2, 2], "mir_d(1/2)"),
            (M_ANTI, [0, 0], "mir_a(0)"),
            (M_ANTI, [4, 0], "mir_a(1)"),
            (M_ANTI, [2, 2], "mir_a(1/2)"),
            // no glides (the rigid sibling of the running bond's cmm)
            (M_MIRX, [2, 2], "glide(1-x, y+1)"),
            (M_MIRY, [2, 0], "glide(x+1, -y)"),
        ];
        for (m, v, label) in forbidden {
            assert!(
                !sym(m, v),
                "{label} must NOT be a symmetry of the stack bond"
            );
        }
    }

    /// Herringbone-45 (M115) standard trio.
    #[test]
    fn herringbone_45_covers_region() {
        check_covers(
            &herringbone_45::herringbone_45(),
            "herringbone-45",
            BBox::new(v2(-4.0, -4.0), v2(8.0, 8.0)),
        );
    }

    #[test]
    fn herringbone_45_rotated_scaled() {
        check_rotated(
            &herringbone_45::herringbone_45(),
            "herringbone-45(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn herringbone_45_deterministic() {
        check_det(
            &herringbone_45::herringbone_45(),
            "herringbone-45",
            BBox::new(v2(-2.0, -2.0), v2(8.0, 6.0)),
        );
    }

    /// Lattice-membership predicate for the herringbone-45 table, in
    /// its integer P-coords: L'' = <(2,0),(0,4)> = { x even,
    /// y ≡ 0 (mod 4) }; world = (√2/2)·P.
    fn h45_in_lattice(x: i64, y: i64) -> bool {
        x.rem_euclid(2) == 0 && y.rem_euclid(4) == 0
    }

    #[test]
    fn herringbone_45_wallpaper_group_is_pgg() {
        // Cell shapes in integer P-coords (world = (√2/2)·P), each in
        // cyclic order; the shapes are rotated hexagons, so the probe
        // uses the vertex-set criterion rather than the axis-aligned
        // bbox probe.
        let basis = [(2, 0), (0, 4)];
        let shapes: [[[i64; 2]; 6]; 2] = [
            [[0, 0], [1, 1], [2, 2], [1, 3], [0, 2], [-1, 1]],
            [[2, 2], [3, 3], [2, 4], [1, 5], [0, 4], [1, 3]],
        ];
        let sym = |m: [i64; 4], v: [i64; 2]| {
            shape_set_is_sym(m, v, &basis, &shapes, h45_in_lattice)
        };

        // pgg: translations L''; 2-folds at v ≡ (1,3) mod (2,4);
        // vertical glide axes (MIRX, v ≡ (0,2) mod (2,4)); horizontal
        // glide axes (MIRY, v ≡ (1,1) mod (2,4)); no mirrors, no
        // 4-folds, no diagonal glides.
        let expected = [
            (M_ROT2, [1, 3], "rot2 center (1/2,3/2)"),
            (M_ROT2, [3, 3], "rot2 center (3/2,3/2)"),
            (M_ROT2, [-1, 3], "rot2 center (-1/2,3/2)"),
            (M_ROT2, [1, 7], "rot2 center (1/2,7/2)"),
            (M_ROT2, [1, -1], "rot2 center (1/2,-1/2)"),
            (M_ROT2, [3, -1], "rot2 center (3/2,-1/2)"),
            (M_ROT2, [-1, -1], "rot2 center (-1/2,-1/2)"),
            (M_MIRX, [0, 2], "glide x=0, shift 1"),
            (M_MIRX, [2, 2], "glide x=1, shift 1"),
            (M_MIRX, [0, 6], "glide x=0, shift 3"),
            (M_MIRX, [-2, 2], "glide x=-1, shift 1"),
            (M_MIRX, [0, -2], "glide x=0, shift -1"),
            (M_MIRY, [1, 1], "glide y=1/2, shift 1/2"),
            (M_MIRY, [3, 1], "glide y=1/2, shift 3/2"),
            (M_MIRY, [-1, 1], "glide y=1/2, shift -1/2"),
            (M_MIRY, [1, 5], "glide y=5/2, shift 1/2"),
            (M_MIRY, [1, -3], "glide y=-3/2, shift 1/2"),
            (M_MIRY, [3, 5], "glide y=5/2, shift 3/2"),
        ];
        for (m, v, label) in expected {
            assert!(sym(m, v), "{label} must be a symmetry of the 45° herringbone");
        }

        // No pure mirrors of any family (pgg is chiral): the MIRX
        // mirror lines (v_y = 0 mod 4) and MIRY mirror lines (v_x
        // even) are all absent, as are off-family glides, 4-folds,
        // and diagonal glides.
        let forbidden = [
            (M_MIRX, [0, 0], "mir x=0"),
            (M_MIRX, [2, 0], "mir x=1"),
            (M_MIRX, [4, 0], "mir x=2"),
            (M_MIRY, [0, 0], "mir y=0"),
            (M_MIRY, [0, 4], "mir y=2"),
            (M_MIRY, [2, 2], "mir y=1"),
            // 2-folds off the (1,3) mod (2,4) family
            (M_ROT2, [0, 0], "rot2(0,0)"),
            (M_ROT2, [2, 0], "rot2(1,0)"),
            (M_ROT2, [0, 4], "rot2(0,2)"),
            (M_ROT2, [1, 1], "rot2(1/2,1/2)"),
            (M_ROT2, [2, 2], "rot2(1,1)"),
            (M_ROT2, [1, 5], "rot2(1/2,5/2)"),
            // no 4-folds
            (M_ROT4, [0, 0], "rot4(0,0)"),
            (M_ROT4, [2, 0], "rot4(1,0)"),
            (M_ROT4, [1, 3], "rot4 at a 2-fold"),
            (M_ROT4, [0, 4], "rot4(0,2)"),
            // no diagonal mirrors or glides
            (M_DIAG, [0, 0], "glide/mir y=x"),
            (M_DIAG, [2, 0], "glide/mir y=x-1"),
            (M_DIAG, [1, 1], "glide/mir y=x+1/2"),
            (M_ANTI, [0, 0], "glide/mir y=-x"),
            (M_ANTI, [2, 0], "glide/mir y=-x+1"),
            (M_ANTI, [1, 1], "glide/mir y=-x+1/2"),
        ];
        for (m, v, label) in forbidden {
            assert!(
                !sym(m, v),
                "{label} must NOT be a symmetry of the 45° herringbone"
            );
        }
    }

    /// Basketweave-three (M116) standard trio.
    #[test]
    fn basketweave_3_covers_region() {
        check_covers(
            &basketweave_3::basketweave_3(),
            "basketweave-3",
            BBox::new(v2(-4.0, -4.0), v2(10.0, 9.0)),
        );
    }

    #[test]
    fn basketweave_3_rotated_scaled() {
        check_rotated(
            &basketweave_3::basketweave_3(),
            "basketweave-3(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn basketweave_3_deterministic() {
        check_det(
            &basketweave_3::basketweave_3(),
            "basketweave-3",
            BBox::new(v2(-2.0, -2.0), v2(12.0, 8.0)),
        );
    }

    /// Lattice-membership / origin-set predicates for the basketweave-3
    /// table, in doubled coordinates. The declared cell is
    /// ⟨(12,0),(0,10)⟩; H-domino origins sit at x ≡ 0 (mod 4), y ≡
    /// 0/2/4 (mod 10), V-domino origins at x even, y ≡ 6 (mod 10).
    fn bw3_in_lattice(x: i64, y: i64) -> bool {
        x.rem_euclid(12) == 0 && y.rem_euclid(10) == 0
    }

    fn bw3_in_origin(kind: u8, x: i64, y: i64) -> bool {
        match kind {
            0 => x.rem_euclid(4) == 0 && matches!(y.rem_euclid(10), 0 | 2 | 4),
            1 => x.rem_euclid(2) == 0 && y.rem_euclid(10) == 6,
            _ => false,
        }
    }

    #[test]
    fn basketweave_3_wallpaper_group_is_pmm() {
        const M_I: [i64; 4] = [1, 0, 0, 1];
        const M_ROT4_270: [i64; 4] = [0, 1, -1, 0]; // R270
        // The 15 split dominoes of one 6×5 cell in doubled coordinates:
        // 9 H dominoes (4×2, kind 0) at x ∈ {0,4,8}, y ∈ {0,2,4}, and 6
        // V dominoes (2×4, kind 1) at y = 6, x even.
        let basis = [(12, 0), (0, 10)];
        let mut cells: Vec<(i64, i64, i64, i64, u8)> = Vec::with_capacity(15);
        for oy in [0i64, 2, 4] {
            for ox in [0i64, 4, 8] {
                cells.push((ox, oy, 4, 2, 0));
            }
        }
        for ox in (0..12).step_by(2) {
            cells.push((ox, 6, 2, 4, 1));
        }
        let sym = |m: [i64; 4], v: [i64; 2]| {
            pavers_exact_is_sym(m, v, &basis, &cells, bw3_in_lattice, bw3_in_origin)
        };

        // pmm: translations on the true lattice <(2,0),(0,5)> (doubled:
        // v_x ≡ 0 mod 4, v_y ≡ 0 mod 10 — finer than the declared cell,
        // which is an index-3 superlattice); mirrors at every vertical
        // axis x = k; mirrors at the horizontal axes y = 2.5k + 1.5
        // (doubled: v_y ≡ 6 mod 10); 2-folds at every mirror
        // intersection (k, 2.5k' + 1.5).
        let expected = [
            (M_I, [0, 0], "T(0,0)"),
            (M_I, [4, 0], "T(2,0)"),
            (M_I, [8, 0], "T(4,0)"),
            (M_I, [12, 0], "T(6,0) — declared cell"),
            (M_I, [0, 10], "T(0,5) — declared cell"),
            (M_I, [4, 10], "T(2,5)"),
            (M_I, [-4, 0], "T(-2,0)"),
            (M_I, [-12, 10], "T(-6,5)"),
            (M_MIRX, [0, 0], "mir_x(0)"),
            (M_MIRX, [4, 0], "mir_x(1)"),
            (M_MIRX, [8, 0], "mir_x(2)"),
            (M_MIRX, [-4, 0], "mir_x(-1)"),
            (M_MIRX, [0, 10], "mir_x(0) + T(0,5)"),
            (M_MIRX, [4, 10], "mir_x(1) + T(0,5)"),
            (M_MIRY, [0, 6], "mir_y(3/2)"),
            (M_MIRY, [4, 6], "mir_y(3/2) + T(2,0)"),
            (M_MIRY, [8, 6], "mir_y(3/2) + T(4,0)"),
            (M_MIRY, [0, 16], "mir_y(4)"),
            (M_MIRY, [4, 16], "mir_y(4) + T(2,0)"),
            (M_MIRY, [12, 16], "mir_y(4) + T(6,0)"),
            (M_MIRY, [0, -4], "mir_y(-1)"),
            (M_ROT2, [0, 6], "rot2(0,3/2)"),
            (M_ROT2, [4, 6], "rot2(1,3/2)"),
            (M_ROT2, [8, 16], "rot2(2,4)"),
            (M_ROT2, [12, 16], "rot2(3,4)"),
            (M_ROT2, [0, -4], "rot2(0,-1)"),
            (M_ROT2, [-4, 6], "rot2(-1,3/2)"),
        ];
        for (m, v, label) in expected {
            assert!(
                sym(m, v),
                "{label} must be a symmetry of the basketweave-3"
            );
        }

        // No half-integer x-mirrors, no y-mirrors off the 2.5k+1.5
        // family, no essential glides (a half-period shift along a
        // mirror carries the even-x horizontal courses to odd x, and
        // shifts the vertical walls off the integer x-axes), no
        // 2-folds off the mirror intersections, no 4-folds, no
        // diagonal mirrors.
        let forbidden = [
            // half-integer x-axes are not mirrors
            (M_MIRX, [2, 0], "mir_xhalf(1/2)"),
            (M_MIRX, [6, 0], "mir_xhalf(3/2)"),
            // y-mirrors only at y = 2.5k + 1.5
            (M_MIRY, [0, 0], "mir_y(0)"),
            (M_MIRY, [4, 0], "mir_y(0) + T(2,0)"),
            (M_MIRY, [0, 10], "mir_y(5/2)"),
            (M_MIRY, [4, 10], "mir_y(5/2) + T(2,0)"),
            // glides along the mirrors (half-period shifts)
            (M_MIRX, [0, 5], "glide x=0, shift 5/2"),
            (M_MIRX, [4, 5], "glide x=1, shift 5/2"),
            (M_MIRY, [2, 6], "glide y=3/2, shift 1"),
            (M_MIRY, [2, 16], "glide y=4, shift 1"),
            // mirrors shifted by a non-lattice amount
            (M_MIRX, [0, 6], "mir_x(0) + T(0,3)"),
            (M_MIRX, [4, 6], "mir_x(1) + T(0,3)"),
            // 2-folds only at the mirror intersections
            (M_ROT2, [0, 0], "rot2(0,0)"),
            (M_ROT2, [4, 0], "rot2(1,0)"),
            (M_ROT2, [0, 10], "rot2(0,5/2)"),
            (M_ROT2, [2, 6], "rot2(1/2,3/2)"),
            // no 4-folds
            (M_ROT4, [0, 0], "rot4(0,0)"),
            (M_ROT4, [4, 0], "rot4(1,0)"),
            (M_ROT4_270, [0, 0], "rot4_270(0,0)"),
            (M_ROT4_270, [12, 10], "rot4_270(3,5/2)"),
            // no diagonal mirrors or glides
            (M_DIAG, [0, 0], "mir_d(0)"),
            (M_DIAG, [4, 6], "mir_d(3/2)"),
            (M_ANTI, [0, 0], "mir_a(0)"),
            (M_ANTI, [0, 16], "mir_a(4)"),
        ];
        for (m, v, label) in forbidden {
            assert!(
                !sym(m, v),
                "{label} must NOT be a symmetry of the basketweave-3"
            );
        }
    }

    #[test]
    fn stepped_hexominoes_covers_region() {
        check_covers(
            &stepped_hexominoes::stepped_hexominoes(),
            "stepped-hexominoes",
            BBox::new(v2(-4.0, -4.0), v2(12.0, 10.0)),
        );
    }

    #[test]
    fn stepped_hexominoes_rotated_scaled() {
        check_rotated(
            &stepped_hexominoes::stepped_hexominoes(),
            "stepped-hexominoes(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn stepped_hexominoes_deterministic() {
        check_det(
            &stepped_hexominoes::stepped_hexominoes(),
            "stepped-hexominoes",
            BBox::new(v2(-3.0, -3.0), v2(9.0, 5.0)),
        );
    }

    /// Lattice membership for the stepped-hexominoes table:
    /// Λ = ⟨(3,1),(0,2)⟩ = {(3i, i + 2j) : i, j ∈ ℤ}.
    fn sh_in_lat(x: i64, y: i64) -> bool {
        x.rem_euclid(3) == 0 && (y - x / 3).rem_euclid(2) == 0
    }

    /// The prototile's 12 vertex set (the CCW corners of the emitted
    /// polygon, four of them 180° collinear — the shape test runs on
    /// the unit-edge data).
    const SH_VERTS: [(i64, i64); 12] = [
        (0, 0), (1, 0), (1, 1), (2, 1), (3, 1), (3, 2),
        (3, 3), (2, 3), (1, 3), (1, 2), (0, 2), (0, 1),
    ];

    /// Does M normalize Λ? Every candidate here is in GL(2, ℤ)
    /// (|det| = 1), so both basis images in Λ imply M(Λ) = Λ.
    fn sh_normalizes(m: [i64; 4]) -> bool {
        sh_in_lat(3 * m[0] + m[1], 3 * m[2] + m[3]) && sh_in_lat(2 * m[1], 2 * m[3])
    }

    /// M(S) = S + t for a unique t (the prototile is a bounded
    /// polygon, hence has no nontrivial self-translation)? Returns t,
    /// found by aligning the lexicographic-min vertices and verified
    /// by sorted set comparison.
    fn sh_shape(m: [i64; 4]) -> Option<(i64, i64)> {
        let mut mv: Vec<(i64, i64)> = SH_VERTS
            .iter()
            .map(|&(x, y)| (m[0] * x + m[1] * y, m[2] * x + m[3] * y))
            .collect();
        let min = |v: &[(i64, i64)]| *v.iter().min().unwrap();
        let t = (min(&mv).0 - min(&SH_VERTS).0, min(&mv).1 - min(&SH_VERTS).1);
        let mut moved: Vec<(i64, i64)> = SH_VERTS
            .iter()
            .map(|&(x, y)| (x + t.0, y + t.1))
            .collect();
        mv.sort_unstable();
        moved.sort_unstable();
        (mv == moved).then_some(t)
    }

    /// The exact p1 criterion: g = (M, v) is a symmetry of the tiling
    /// iff M normalizes Λ, M(S) = S + t for some t ∈ Λ, and v ∈ Λ
    /// (one orbit per cell, so the shape match is translation-exact:
    /// the image of the origin tile must land on a lattice tile).
    fn sh_is_sym(m: [i64; 4], v: [i64; 2]) -> bool {
        sh_normalizes(m)
            && sh_shape(m).is_some_and(|t| sh_in_lat(t.0, t.1))
            && sh_in_lat(v[0], v[1])
    }

    #[test]
    fn stepped_hexominoes_wallpaper_group_is_p1() {
        const M_ROT4_270: [i64; 4] = [0, 1, -1, 0]; // R270
        // The seven non-identity D4 candidates, as
        // (M, normalizes Λ, M(S) = S + t). R90/R270/Md/Ma fail the
        // normalizer; R180/Mx/My normalize but break the asymmetric
        // staircase shape.
        let table: [([i64; 4], bool, bool); 7] = [
            (M_ROT4, false, false),     // R90
            (M_ROT2, true, false),      // R180
            (M_ROT4_270, false, false), // R270
            (M_MIRX, true, false),      // mirrors of x = c
            (M_MIRY, true, false),      // mirrors of y = c
            (M_DIAG, false, false),     // mirrors/glides of y = x + c
            (M_ANTI, false, false),     // mirrors/glides of y = −x + c
        ];
        for (m, norm, shape) in table {
            assert_eq!(sh_normalizes(m), norm, "normalizer of {m:?}");
            assert_eq!(sh_shape(m).is_some(), shape, "shape of {m:?}");
        }

        // Every isometry of the tiling is a lattice translation:
        // (I, v) is a symmetry for every v in a wide Λ window, and no
        // non-identity M is a symmetry for any v.
        let window: Vec<[i64; 2]> = (
            -2i64..=2
        )
            .flat_map(|i| {
                (-3i64..=3).map(move |j| {
                    [
                        3 * i,
                        i + 2 * j,
                    ]
                })
            })
            .collect();
        for v in &window {
            assert!(
                sh_is_sym([1, 0, 0, 1], *v),
                "translation {v:?} must be a symmetry"
            );
        }
        for (m, _, _) in table {
            for v in &window {
                assert!(
                    !sh_is_sym(m, *v),
                    "({m:?}, {v:?}) must NOT be a symmetry"
                );
            }
        }

        // Positive sanity at the two basis vectors, negative sanity
        // just off the lattice — including (3,0), which is absent from
        // Λ, so the width-3 vertical bands are not congruent by a
        // tiling symmetry.
        assert!(sh_is_sym([1, 0, 0, 1], [3, 1]));
        assert!(sh_is_sym([1, 0, 0, 1], [0, 2]));
        assert!(!sh_is_sym([1, 0, 0, 1], [0, 1]));
        assert!(!sh_is_sym([1, 0, 0, 1], [3, 0]));
        assert!(!sh_in_lat(3, 0));
    }

    /// Trapezoid-courses (M79) standard trio.
    #[test]
    fn trapezoid_courses_covers_region() {
        check_covers(
            &trapezoid_courses::trapezoid_courses(),
            "trapezoid-courses",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn trapezoid_courses_rotated_scaled() {
        check_rotated(
            &trapezoid_courses::trapezoid_courses(),
            "trapezoid-courses(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn trapezoid_courses_deterministic() {
        check_det(
            &trapezoid_courses::trapezoid_courses(),
            "trapezoid-courses",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Glide-hexagon-strips (M80) standard trio.
    #[test]
    fn glide_hexagon_strips_covers_region() {
        check_covers(
            &glide_hexagon_strips::glide_hexagon_strips(),
            "glide-hexagon-strips",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn glide_hexagon_strips_rotated_scaled() {
        check_rotated(
            &glide_hexagon_strips::glide_hexagon_strips(),
            "glide-hexagon-strips(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn glide_hexagon_strips_deterministic() {
        check_det(
            &glide_hexagon_strips::glide_hexagon_strips(),
            "glide-hexagon-strips",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Shifted-trapezoid-courses (M81) standard trio.
    #[test]
    fn shifted_trapezoid_courses_covers_region() {
        check_covers(
            &shifted_trapezoid_courses::shifted_trapezoid_courses(),
            "shifted-trapezoid-courses",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn shifted_trapezoid_courses_rotated_scaled() {
        check_rotated(
            &shifted_trapezoid_courses::shifted_trapezoid_courses(),
            "shifted-trapezoid-courses(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn shifted_trapezoid_courses_deterministic() {
        check_det(
            &shifted_trapezoid_courses::shifted_trapezoid_courses(),
            "shifted-trapezoid-courses",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Wavy-courses (M82) standard trio.
    #[test]
    fn wavy_courses_covers_region() {
        check_covers(
            &wavy_courses::wavy_courses(),
            "wavy-courses",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn wavy_courses_rotated_scaled() {
        check_rotated(
            &wavy_courses::wavy_courses(),
            "wavy-courses(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn wavy_courses_deterministic() {
        check_det(
            &wavy_courses::wavy_courses(),
            "wavy-courses",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Chiral-cut-honeycomb (M83) standard trio.
    #[test]
    fn chiral_cut_honeycomb_covers_region() {
        check_covers(
            &chiral_cut_honeycomb::chiral_cut_honeycomb(),
            "chiral-cut-honeycomb",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn chiral_cut_honeycomb_rotated_scaled() {
        check_rotated(
            &chiral_cut_honeycomb::chiral_cut_honeycomb(),
            "chiral-cut-honeycomb(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn chiral_cut_honeycomb_deterministic() {
        check_det(
            &chiral_cut_honeycomb::chiral_cut_honeycomb(),
            "chiral-cut-honeycomb",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Split-triangle-grid (M84) standard trio.
    #[test]
    fn split_triangle_grid_covers_region() {
        check_covers(
            &split_triangle_grid::split_triangle_grid(),
            "split-triangle-grid",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn split_triangle_grid_rotated_scaled() {
        check_rotated(
            &split_triangle_grid::split_triangle_grid(),
            "split-triangle-grid(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn split_triangle_grid_deterministic() {
        check_det(
            &split_triangle_grid::split_triangle_grid(),
            "split-triangle-grid",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Alcove-cut-grid (M85) standard trio.
    #[test]
    fn alcove_cut_grid_covers_region() {
        check_covers(
            &alcove_cut_grid::alcove_cut_grid(),
            "alcove-cut-grid",
            BBox::new(v2(-5.0, -3.0), v2(11.0, 7.0)),
        );
    }

    #[test]
    fn alcove_cut_grid_rotated_scaled() {
        check_rotated(
            &alcove_cut_grid::alcove_cut_grid(),
            "alcove-cut-grid(rot30,scale0.7)",
            BBox::new(v2(-5.0, -5.0), v2(5.0, 5.0)),
            0.7,
            30.0,
        );
    }

    #[test]
    fn alcove_cut_grid_deterministic() {
        check_det(
            &alcove_cut_grid::alcove_cut_grid(),
            "alcove-cut-grid",
            BBox::new(v2(-4.0, -3.0), v2(10.0, 6.0)),
        );
    }

    /// Lattice membership for the trapezoid-courses table, in doubled
    /// (×10, integer) coordinates: Λ10 = ⟨(20,0),(0,10)⟩ =
    /// {(20i, 10j) : i, j ∈ ℤ}.
    fn tc_in_lat(x: i64, y: i64) -> bool {
        x.rem_euclid(20) == 0 && y.rem_euclid(10) == 0
    }

    /// The four cell tiles in ×10 coordinates, CCW:
    /// P (6/10 area, slant rising to the right), mP (its mirror in
    /// x = 1), P' (4/10 area, slant rising), mP' (its mirror in x = 1).
    const TC_TILES: [[(i64, i64); 4]; 4] = [
        [(0, 3), (10, 5), (10, 10), (0, 10)],
        [(10, 5), (20, 3), (20, 10), (10, 10)],
        [(0, 0), (10, 0), (10, 5), (0, 3)],
        [(10, 0), (20, 0), (20, 3), (10, 5)],
    ];

    /// Does M normalize Λ10? Every candidate here is in GL(2, ℤ)
    /// (|det| = 1), so both basis images in Λ10 imply M(Λ10) = Λ10.
    fn tc_normalizes(m: [i64; 4]) -> bool {
        tc_in_lat(20 * m[0], 20 * m[2]) && tc_in_lat(10 * m[1], 10 * m[3])
    }

    /// The image of tile `t` under g = (M, v), as a sorted 4-point set.
    fn tc_image(t: &[(i64, i64); 4], m: [i64; 4], v: [i64; 2]) -> [(i64, i64); 4] {
        let mut out: Vec<(i64, i64)> = t
            .iter()
            .map(|&(x, y)| (m[0] * x + m[1] * y + v[0], m[2] * x + m[3] * y + v[1]))
            .collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 4] = [(0, 0); 4];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    fn tc_sorted(t: &[(i64, i64); 4]) -> [(i64, i64); 4] {
        let mut out = *t;
        out.sort_unstable();
        out
    }

    /// Is the sorted set `a` a translate of the sorted set `u` by a
    /// vector of Λ10? Tries every (a_i, u_j) anchor pair; a match on
    /// the first pair fixes λ and the full-set check confirms it.
    fn tc_translate_of(a: &[(i64, i64); 4], u: &[(i64, i64); 4]) -> bool {
        (0..4).into_iter().flat_map(|i| (0..4).map(move |j| (a[i].0 - u[j].0, a[i].1 - u[j].1))).any(|(dx, dy)| {
            tc_in_lat(dx, dy)
                && (0..4)
                    .all(|k| (a[k].0, a[k].1) == (u[k].0 + dx, u[k].1 + dy))
        })
    }

    /// Does g = (M, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for a 4-tile
    /// cell (with M normalizing Λ10, the image of the full tiling is
    /// then exactly the tiling)?
    fn tc_maps_cells(m: [i64; 4], v: [i64; 2]) -> bool {
        TC_TILES.iter().all(|t| {
            let a = tc_image(t, m, v);
            TC_TILES
                .iter()
                .any(|u| tc_translate_of(&a, &tc_sorted(u)))
        })
    }

    /// The exact pm criterion: g = (M, v) is a symmetry of the tiling
    /// iff M normalizes Λ10 and it maps every cell tile onto a cell
    /// tile plus a lattice translate.
    fn tc_is_sym(m: [i64; 4], v: [i64; 2]) -> bool {
        tc_normalizes(m) && tc_maps_cells(m, v)
    }

    /// Exact pm probe: mirrors at x = integer are symmetries; half-turns,
    /// horizontal mirrors/glides, the 90° rotations, the diagonal and
    /// anti-diagonal mirrors/glides, off-lattice translations, and the
    /// half-integer vertical mirrors are all not — mirroring the full
    /// Python battery (`m79_verify.py`: 15 positive / 53 negative).
    #[test]
    fn trapezoid_courses_wallpaper_group_is_pm() {
        // Structural sanity: the mirror x = 1 pairs the two chiral
        // trapezoids exactly, with no translate at all — P ↔ mP and
        // P' ↔ mP' (hence x = 1 is a genuine mirror line of the tiling).
        assert_eq!(tc_image(&TC_TILES[0], M_MIRX, [20, 0]), tc_sorted(&TC_TILES[1]));
        assert_eq!(tc_image(&TC_TILES[1], M_MIRX, [20, 0]), tc_sorted(&TC_TILES[0]));
        assert_eq!(tc_image(&TC_TILES[2], M_MIRX, [20, 0]), tc_sorted(&TC_TILES[3]));
        assert_eq!(tc_image(&TC_TILES[3], M_MIRX, [20, 0]), tc_sorted(&TC_TILES[2]));

        // Positive: the lattice translations (a window of Λ10).
        for i in -1..=1 {
            for j in -1..=1 {
                assert!(
                    tc_is_sym([1, 0, 0, 1], [20 * i, 10 * j]),
                    "translation ({}i, {}j) must be a symmetry",
                    i,
                    j
                );
            }
        }
        // Positive: the vertical mirror lines x = k (k = −1..2), and
        // their composites with lattice translations.
        for c in -1i64..=2 {
            assert!(tc_is_sym(M_MIRX, [20 * c, 0]), "mirror x = {c} must be a symmetry");
        }
        assert!(tc_is_sym(M_MIRX, [0, 10]), "mirror x=0 + (0,1)");
        assert!(tc_is_sym(M_MIRX, [20, -10]), "mirror x=1 + (0,−1)");

        // Negative: off-lattice translations.
        for v in [
            [10, 0],
            [0, 5],
            [5, 5],
            [20, 5],
            [10, 10],
        ] {
            assert!(!tc_is_sym([1, 0, 0, 1], v), "off-lattice translation {v:?}");
        }
        // Negative: half-integer vertical mirrors (x = k + 1/2).
        for c in [-10i64, 10, 30] {
            assert!(!tc_is_sym(M_MIRX, [c, 0]), "mirror x = {c}/10 must not be");
        }
        // Negative: half-turns about the 18 candidate centers from the
        // Python battery (every center mod Λ/2 that the pattern could
        // plausibly carry), given in exact ×10 integer coordinates:
        // half-turn g(p) = 2c − p has v = 2c, so v×10 = (2cx10, 2cy10).
        for (cx10, cy10) in [
            (10, 5),
            (5, 5),
            (15, 5),
            (0, 5),
            (5, 0),
            (10, 0),
            (0, 0),
            (20, 0),
            (0, 10),
            (20, 10),
            (10, 10),
            (5, 10),
            (15, 10),
            (20, 5),
            (0, 3),
            (10, 7),
            (15, 3),
            (5, 7),
        ] {
            let v = [2 * cx10, 2 * cy10];
            assert!(
                !tc_is_sym(M_ROT2, v),
                "half-turn about ({cx10},{cy10})/10 must not be a symmetry"
            );
        }
        // Negative: horizontal mirrors and horizontal glides
        // ((x, 2c−y) + (s, 0) for s ∈ {0, 1, 2}, c ∈ {0, 1/2, 1}).
        for s in [0i64, 10, 20] {
            for c in [0i64, 10, 20] {
                assert!(!tc_is_sym(M_MIRY, [s, c]), "horizontal mirror/glide ({s},{c})");
            }
        }
        // Negative: 90° rotations (R90 and R270 about candidate centers;
        // neither normalizes Λ10 in the first place).
        const M_ROT4_270: [i64; 4] = [0, 1, -1, 0];
        for v in [[0, 0], [20, 10], [20, 0], [0, 10]] {
            assert!(!tc_is_sym(M_ROT4, v), "rot90 at {v:?}");
            assert!(!tc_is_sym(M_ROT4_270, v), "rot270 at {v:?}");
        }
        // Negative: diagonal and anti-diagonal mirrors/glides.
        for v in [[0, 0], [5, 5], [10, 10]] {
            assert!(!tc_is_sym(M_DIAG, v), "diagonal mirror/glide at {v:?}");
        }
        for v in [[0, 0], [10, 10], [15, 15]] {
            assert!(!tc_is_sym(M_ANTI, v), "anti-diagonal mirror/glide at {v:?}");
        }
        // Negative: vertical glides (mirror x = c + half-period shift s).
        for c in [0i64, 20] {
            for s in [5i64, 15] {
                assert!(!tc_is_sym(M_MIRX, [c, s]), "vertical glide (x={c}/10, shift {s}/10)");
            }
        }

        // Sanity: the two prototile classes are incongruent — different
        // area and different squared-edge multisets (the structural
        // reason half-turns and horizontal mirrors cannot be symmetries).
        let area10 = |t: &[(i64, i64); 4]| -> i64 {
            (0..4)
                .map(|i| t[i].0 * t[(i + 1) % 4].1 - t[(i + 1) % 4].0 * t[i].1)
                .sum::<i64>()
        };
        assert_ne!(area10(&TC_TILES[0]), area10(&TC_TILES[2]), "P and P' areas differ");
    }

    /// Lattice membership for the glide-hexagon-strips table, in ×20
    /// (integer) coordinates: Λ20 = ⟨(20,0),(0,40)⟩ = {(20i, 40j) :
    /// i, j ∈ ℤ}.
    fn gh_in_lat(x: i64, y: i64) -> bool {
        x.rem_euclid(20) == 0 && y.rem_euclid(40) == 0
    }

    /// The four cell tiles in ×20 coordinates, CCW, in cell order
    /// [A, T, B, U]: A/T the fat hexagons (strips 0/1), B/U the waist
    /// hexagons (strips 0/1).
    const GH_TILES: [[(i64, i64); 6]; 4] = [
        [(0, 0), (10, 0), (14, 8), (10, 20), (0, 20), (-5, 10)],
        [(-10, 40), (-15, 30), (-10, 20), (0, 20), (4, 32), (0, 40)],
        [(10, 0), (20, 0), (15, 10), (20, 20), (10, 20), (14, 8)],
        [(0, 40), (4, 32), (0, 20), (10, 20), (5, 30), (10, 40)],
    ];

    /// Does M normalize Λ20? Every candidate here is in GL(2, ℤ)
    /// (|det| = 1), so both basis images in Λ20 imply M(Λ20) = Λ20.
    fn gh_normalizes(m: [i64; 4]) -> bool {
        gh_in_lat(20 * m[0], 20 * m[2]) && gh_in_lat(40 * m[1], 40 * m[3])
    }

    /// The image of tile `t` under g = (M, v), as a sorted 6-point set.
    fn gh_image(t: &[(i64, i64); 6], m: [i64; 4], v: [i64; 2]) -> [(i64, i64); 6] {
        let mut out: Vec<(i64, i64)> = t
            .iter()
            .map(|&(x, y)| (m[0] * x + m[1] * y + v[0], m[2] * x + m[3] * y + v[1]))
            .collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 6] = [(0, 0); 6];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    /// The sorted 6-point set `t + (dx, dy)`.
    fn gh_shifted(t: &[(i64, i64); 6], dx: i64, dy: i64) -> [(i64, i64); 6] {
        let mut out: Vec<(i64, i64)> = t.iter().map(|&(x, y)| (x + dx, y + dy)).collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 6] = [(0, 0); 6];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    fn gh_sorted(t: &[(i64, i64); 6]) -> [(i64, i64); 6] {
        let mut out = *t;
        out.sort_unstable();
        out
    }

    /// Is the sorted set `a` a translate of the sorted set `u` by a
    /// vector of Λ20? Tries every (a_i, u_j) anchor pair; a match on
    /// the first pair fixes λ and the full-set check confirms it.
    fn gh_translate_of(a: &[(i64, i64); 6], u: &[(i64, i64); 6]) -> bool {
        (0..6)
            .into_iter()
            .flat_map(|i| (0..6).map(move |j| (a[i].0 - u[j].0, a[i].1 - u[j].1)))
            .any(|(dx, dy)| {
                gh_in_lat(dx, dy)
                    && (0..6)
                        .all(|k| (a[k].0, a[k].1) == (u[k].0 + dx, u[k].1 + dy))
            })
    }

    /// Does g = (M, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for a 4-tile
    /// cell (with M normalizing Λ20, the image of the full tiling is
    /// then exactly the tiling)?
    fn gh_maps_cells(m: [i64; 4], v: [i64; 2]) -> bool {
        GH_TILES.iter().all(|t| {
            let a = gh_image(t, m, v);
            GH_TILES.iter().any(|u| gh_translate_of(&a, &gh_sorted(u)))
        })
    }

    /// The exact pg criterion: g = (M, v) is a symmetry of the tiling
    /// iff M normalizes Λ20 and it maps every cell tile onto a cell
    /// tile plus a lattice translate.
    fn gh_is_sym(m: [i64; 4], v: [i64; 2]) -> bool {
        gh_normalizes(m) && gh_maps_cells(m, v)
    }

    /// Exact pg probe: the lattice translations and the single glide
    /// family g·Λ are symmetries; every off-lattice translation coset,
    /// every non-glide horizontal mirror/glide coset, all 16 vertical
    /// mirror/glide cosets, all 16 half-turn cosets, and every
    /// non-normalizing linear part (diagonal and anti-diagonal
    /// mirrors/glides, 90°/270° rotations) are not — mirroring the
    /// full Python battery (`m80_verify.py`: 74 positive candidates,
    /// 62 exact-coset negatives, 6 non-normalizing parts).
    #[test]
    fn glide_hexagon_strips_wallpaper_group_is_pg() {
        // Structural sanity: the glide (M_MIRY, (1/2, 0)) — ×20,
        // (10, 0) — is a symmetry and swaps the strips: g(A) = T +
        // (20,−40), g(T) = A + (0,−40), g(B) = U + (20,−40), g(U) =
        // B + (0,−40), exact vertex-set equality. The strip-1 finger
        // at (4,32)/20 = (1/5,8/5) is the glide image of the strip-0
        // finger at (14,8)/20 = (7/10,2/5) — its x-position is forced
        // by the glide.
        assert_eq!(gh_image(&GH_TILES[0], M_MIRY, [10, 0]), gh_shifted(&GH_TILES[1], 20, -40));
        assert_eq!(gh_image(&GH_TILES[1], M_MIRY, [10, 0]), gh_shifted(&GH_TILES[0], 0, -40));
        assert_eq!(gh_image(&GH_TILES[2], M_MIRY, [10, 0]), gh_shifted(&GH_TILES[3], 20, -40));
        assert_eq!(gh_image(&GH_TILES[3], M_MIRY, [10, 0]), gh_shifted(&GH_TILES[2], 0, -40));

        // Positive: the 9 lattice translations (a window of Λ20).
        for i in -1..=1 {
            for j in -1..=1 {
                assert!(
                    gh_is_sym([1, 0, 0, 1], [20 * i, 40 * j]),
                    "translation ({}i, {}j) must be a symmetry",
                    i,
                    j
                );
            }
        }
        // Positive: the 9 glide samples (M_MIRY, (10 + 20i, 40j)) —
        // the family g·Λ (glide 1/2, axes y = integer).
        for i in -1..=1 {
            for j in -1..=1 {
                assert!(
                    gh_is_sym(M_MIRY, [10 + 20 * i, 40 * j]),
                    "glide ((1/2 + {}i), {}j) must be a symmetry",
                    i,
                    j
                );
            }
        }

        // Complete coset enumeration: for each of I, M_MIRY, M_MIRX,
        // M_ROT2, all 16 v-cosets mod Λ20 (x20 ∈ {0,5,10,15}, y20 ∈
        // {0,10,20,30}). Exactly one coset of I (the identity) and
        // exactly one of M_MIRY (the glide coset (10,0)) are
        // positive; all 16 of M_MIRX and all 16 of M_ROT2 are
        // negative — 64 checks, 2 pos / 62 neg.
        for (mi, m) in [
            [1, 0, 0, 1],
            M_MIRY,
            M_MIRX,
            M_ROT2,
        ]
        .into_iter()
        .enumerate()
        {
            for &x20 in &[0i64, 5, 10, 15] {
                for &y20 in &[0i64, 10, 20, 30] {
                    let expected = match mi {
                        0 => x20 == 0 && y20 == 0,
                        1 => x20 == 10 && y20 == 0,
                        _ => false,
                    };
                    assert_eq!(
                        gh_is_sym(m, [x20, y20]),
                        expected,
                        "coset ({x20},{y20}) of linear part {mi}"
                    );
                }
            }
        }

        // Non-normalizing linear parts: R90, R270, diagonal and
        // anti-diagonal mirror/glide linear parts send (20,0) to
        // (0,±20) ∉ Λ20, so no such isometry acts at any center.
        const M_ROT4_270: [i64; 4] = [0, 1, -1, 0];
        for m in [M_ROT4, M_ROT4_270, M_DIAG, M_ANTI] {
            assert!(!gh_normalizes(m), "part {m:?} must not normalize Λ20");
            for v in [[0, 0], [20, 20], [20, 0], [0, 40], [10, 20]] {
                assert!(!gh_is_sym(m, v), "{m:?} at {v:?}");
            }
        }

        // Sanity: the ×20 cell tiles are CCW with doubled areas 580,
        // 580, 220, 220 (areas 290, 290, 110, 110 = 29/40, 29/40,
        // 11/40, 11/40 times 20²), summing to 800 = 20² · det Λ.
        let gh_area2 = |t: &[(i64, i64); 6]| -> i64 {
            (0..6)
                .map(|i| t[i].0 * t[(i + 1) % 6].1 - t[(i + 1) % 6].0 * t[i].1)
                .sum::<i64>()
        };
        assert_eq!(gh_area2(&GH_TILES[0]), 580, "A ×20 doubled area");
        assert_eq!(gh_area2(&GH_TILES[1]), 580, "T ×20 doubled area");
        assert_eq!(gh_area2(&GH_TILES[2]), 220, "B ×20 doubled area");
        assert_eq!(gh_area2(&GH_TILES[3]), 220, "U ×20 doubled area");
        assert_eq!(
            gh_area2(&GH_TILES[0])
                + gh_area2(&GH_TILES[1])
                + gh_area2(&GH_TILES[2])
                + gh_area2(&GH_TILES[3]),
            1600,
            "cell doubled areas sum to 2·20²·det"
        );
    }

    /// Lattice membership for the shifted-trapezoid-courses table, in
    /// ×10 (integer) coordinates: Λ10 = ⟨(20,0),(0,20)⟩ = {(20i, 20j)
    /// : i, j ∈ ℤ}.
    fn st_in_lat(x: i64, y: i64) -> bool {
        x.rem_euclid(20) == 0 && y.rem_euclid(20) == 0
    }

    /// The eight cell tiles in ×10 coordinates, CCW, in cell order
    /// [P, mP, P', mP', R0, R1, R2, R3]: row 0 (y ∈ [0,10]) is the
    /// trapezoid-course row; row 1 (y ∈ [10,20]) is its reflection
    /// F(x,y) = (x, 2−y) shifted by (1,0). R2, R3 straddle the cell's
    /// right edge (x up to 30).
    const ST_TILES: [[(i64, i64); 4]; 8] = [
        [(0, 3), (10, 5), (10, 10), (0, 10)],
        [(10, 5), (20, 3), (20, 10), (10, 10)],
        [(0, 0), (10, 0), (10, 5), (0, 3)],
        [(10, 0), (20, 0), (20, 3), (10, 5)],
        [(10, 10), (20, 10), (20, 15), (10, 17)],
        [(10, 17), (20, 15), (20, 20), (10, 20)],
        [(20, 10), (30, 10), (30, 17), (20, 15)],
        [(20, 15), (30, 17), (30, 20), (20, 20)],
    ];

    /// Does M normalize Λ10? Every candidate here is in GL(2, ℤ)
    /// (|det| = 1), so both basis images in Λ10 imply M(Λ10) = Λ10.
    fn st_normalizes(m: [i64; 4]) -> bool {
        st_in_lat(20 * m[0], 20 * m[2]) && st_in_lat(20 * m[1], 20 * m[3])
    }

    /// The image of tile `t` under g = (M, v), as a sorted 4-point set.
    fn st_image(t: &[(i64, i64); 4], m: [i64; 4], v: [i64; 2]) -> [(i64, i64); 4] {
        let mut out: Vec<(i64, i64)> = t
            .iter()
            .map(|&(x, y)| (m[0] * x + m[1] * y + v[0], m[2] * x + m[3] * y + v[1]))
            .collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 4] = [(0, 0); 4];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    fn st_sorted(t: &[(i64, i64); 4]) -> [(i64, i64); 4] {
        let mut out = *t;
        out.sort_unstable();
        out
    }

    /// The sorted 4-point set `t + (dx, dy)`.
    fn st_shifted(t: &[(i64, i64); 4], dx: i64, dy: i64) -> [(i64, i64); 4] {
        let mut out: Vec<(i64, i64)> = t.iter().map(|&(x, y)| (x + dx, y + dy)).collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 4] = [(0, 0); 4];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    /// Is the sorted set `a` a translate of the sorted set `u` by a
    /// vector of Λ10? Tries every (a_i, u_j) anchor pair; a match on
    /// the first pair fixes λ and the full-set check confirms it.
    fn st_translate_of(a: &[(i64, i64); 4], u: &[(i64, i64); 4]) -> bool {
        (0..4)
            .into_iter()
            .flat_map(|i| (0..4).map(move |j| (a[i].0 - u[j].0, a[i].1 - u[j].1)))
            .any(|(dx, dy)| {
                st_in_lat(dx, dy)
                    && (0..4)
                        .all(|k| (a[k].0, a[k].1) == (u[k].0 + dx, u[k].1 + dy))
            })
    }

    /// Does g = (M, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for an 8-tile
    /// cell (with M normalizing Λ10, the image of the full tiling is
    /// then exactly the tiling)?
    fn st_maps_cells(m: [i64; 4], v: [i64; 2]) -> bool {
        ST_TILES.iter().all(|t| {
            let a = st_image(t, m, v);
            ST_TILES.iter().any(|u| st_translate_of(&a, &st_sorted(u)))
        })
    }

    /// The exact pmg criterion: g = (M, v) is a symmetry of the tiling
    /// iff M normalizes Λ10 and it maps every cell tile onto a cell
    /// tile plus a lattice translate.
    fn st_is_sym(m: [i64; 4], v: [i64; 2]) -> bool {
        st_normalizes(m) && st_maps_cells(m, v)
    }

    /// Exact pmg probe: the lattice translations, the vertical mirror
    /// lines x = k, the glide family g·Λ (g(x,y) = (x+1, −y), glide 1,
    /// axes y = integer), and the half-turns about (m+1/2, n) — for
    /// ALL m, n ∈ ℤ, including the even-y centers — are symmetries;
    /// the complete 128-coset enumeration (all eight D4 linear parts
    /// that normalize Λ10 × 16 cosets) shows no other isometry coset
    /// is positive, and every named negative from the Python battery
    /// (`m81_verify.py`: 48 positive candidates, 128-coset
    /// enumeration 4 pos / 124 neg) is killed.
    #[test]
    fn shifted_trapezoid_courses_wallpaper_group_is_pmg() {
        // Structural sanity: the mirror x = 1 pairs the chiral
        // trapezoids within each row (P ↔ mP, P' ↔ mP'), with no
        // translate at all.
        assert_eq!(st_image(&ST_TILES[0], M_MIRX, [20, 0]), st_sorted(&ST_TILES[1]));
        assert_eq!(st_image(&ST_TILES[1], M_MIRX, [20, 0]), st_sorted(&ST_TILES[0]));
        assert_eq!(st_image(&ST_TILES[2], M_MIRX, [20, 0]), st_sorted(&ST_TILES[3]));
        assert_eq!(st_image(&ST_TILES[3], M_MIRX, [20, 0]), st_sorted(&ST_TILES[2]));
        // The glide (M_MIRY, (1, 2)) — ×10, (10, 20): (x, 2−y) — maps
        // row 0 onto row 1 exactly: P → R0, P' → R1, mP → R2,
        // mP' → R3 (no translate). The same family on the axis y = 0,
        // (x, −y) — ×10, (10, 0) — maps P → R0 + (0,−2).
        assert_eq!(st_image(&ST_TILES[0], M_MIRY, [10, 20]), st_sorted(&ST_TILES[4]));
        assert_eq!(st_image(&ST_TILES[2], M_MIRY, [10, 20]), st_sorted(&ST_TILES[5]));
        assert_eq!(st_image(&ST_TILES[1], M_MIRY, [10, 20]), st_sorted(&ST_TILES[6]));
        assert_eq!(st_image(&ST_TILES[3], M_MIRY, [10, 20]), st_sorted(&ST_TILES[7]));
        assert_eq!(st_image(&ST_TILES[0], M_MIRY, [10, 0]), st_shifted(&ST_TILES[4], 0, -20));
        // The half-turn about (1/2, 1) — ×10, v = (10, 20) — maps P
        // to R2 + (−2,0); the EVEN-Y centre (1/2, 0) — v = (10, 0) —
        // maps P to R2 + (−2,−2) (the even-y centers are centers too:
        // four per 2×2 cell, (m+1/2, n) for all m, n).
        assert_eq!(
            st_image(&ST_TILES[0], M_ROT2, [10, 20]),
            st_shifted(&ST_TILES[6], -20, 0)
        );
        assert_eq!(
            st_image(&ST_TILES[0], M_ROT2, [10, 0]),
            st_shifted(&ST_TILES[6], -20, -20)
        );

        // Positive: the 9 lattice translations (a window of Λ10).
        for i in -1..=1 {
            for j in -1..=1 {
                assert!(
                    st_is_sym([1, 0, 0, 1], [20 * i, 20 * j]),
                    "translation ({}i, {}j) must be a symmetry",
                    i,
                    j
                );
            }
        }
        // Positive: the vertical mirror lines x = k (k = −1..2), and
        // their composites with lattice translations.
        for c in -1i64..=2 {
            assert!(st_is_sym(M_MIRX, [20 * c, 0]), "mirror x = {c} must be a symmetry");
        }
        assert!(st_is_sym(M_MIRX, [0, 20]), "mirror x=0 + (0,2)");
        assert!(st_is_sym(M_MIRX, [20, -20]), "mirror x=1 + (0,−2)");
        // Positive: the 9 glide samples (M_MIRY, (10 + 20i, 20j)) —
        // the family g·Λ, (x+1+2i, −y+2j) (glide 1, axes y = integer).
        for i in -1..=1 {
            for j in -1..=1 {
                assert!(
                    st_is_sym(M_MIRY, [10 + 20 * i, 20 * j]),
                    "glide (1 + {}i, {}j) must be a symmetry",
                    i,
                    j
                );
            }
        }
        // Positive: the 9 half-turns (M_ROT2, (10 + 20m, 20n)) —
        // centers (1/2 + m, n), ALL integers m, n.
        for m in -1..=1 {
            for n in -1..=1 {
                assert!(
                    st_is_sym(M_ROT2, [10 + 20 * m, 20 * n]),
                    "half-turn ({} + 1/2, {}j) must be a symmetry",
                    m,
                    n
                );
            }
        }

        // Complete coset enumeration: for each of the eight D4 linear
        // parts that normalize Λ10 (a square-metric lattice of side
        // 20), all 16 v-cosets mod Λ10 (x10, y10 ∈ {0,5,10,15}).
        // Exactly one coset of I (the identity), one of M_MIRX (the
        // mirrors x = k), one of M_MIRY (the glide coset (10,0)) and
        // one of M_ROT2 (the half-turn coset (10,0)) are positive; all
        // 16 cosets of the four D4-extra parts (rot90, rot270,
        // diagonal, anti-diagonal) are negative — 128 checks, 4 pos /
        // 124 neg (no p4/p4m/p4g/cmm upgrade).
        for (pi, m) in [
            [1, 0, 0, 1],
            M_MIRX,
            M_MIRY,
            M_ROT2,
            M_ROT4,
            [0, 1, -1, 0],
            M_DIAG,
            M_ANTI,
        ]
        .into_iter()
        .enumerate()
        {
            for &x10 in &[0i64, 5, 10, 15] {
                for &y10 in &[0i64, 5, 10, 15] {
                    let expected = match pi {
                        0 => x10 == 0 && y10 == 0,
                        1 => x10 == 0 && y10 == 0,
                        2 | 3 => x10 == 10 && y10 == 0,
                        _ => false,
                    };
                    assert_eq!(
                        st_is_sym(m, [x10, y10]),
                        expected,
                        "coset ({x10},{y10}) of linear part {pi}"
                    );
                }
            }
        }

        // Named negatives (from the Python battery): off-lattice
        // translations.
        for v in [[10, 0], [0, 10], [10, 10], [5, 0]] {
            assert!(!st_is_sym([1, 0, 0, 1], v), "off-lattice translation {v:?}");
        }
        // Horizontal mirrors: y = 0 is the unshifted flip F (lands P
        // at R0 + (−1,0), half a period off); y = 1/2 likewise.
        assert!(!st_is_sym(M_MIRY, [0, 0]), "horizontal mirror y = 0 must not be");
        assert!(!st_is_sym(M_MIRY, [0, 10]), "horizontal mirror y = 1/2 must not be");
        // The glide-1/2 (x + 1/2, −y): right parity, wrong slant
        // endpoint image.
        assert!(!st_is_sym(M_MIRY, [5, 0]), "glide (1/2, axis y = 0) must not be");
        // Vertical mirrors/glides off the x = k family.
        assert!(!st_is_sym(M_MIRX, [10, 0]), "vertical mirror x = 1/2 must not be");
        assert!(!st_is_sym(M_MIRX, [0, 10]), "vertical glide (x = 0, slide (0,1))");
        assert!(!st_is_sym(M_MIRX, [10, 10]), "vertical glide (x = 1/2, slide (0,1))");
        // Off-set half-turn centers: (0,0), (1/2,1/2), (0,1/2),
        // (1,1/2) — v = 2·center ×10.
        for v in [[0, 0], [10, 10], [0, 10], [20, 10]] {
            assert!(!st_is_sym(M_ROT2, v), "off-set half-turn v = {v:?}");
        }

        // Sanity: the ×10 cell tiles are CCW with doubled areas 120,
        // 120, 80, 80, 120, 80, 120, 80 (areas 6/10, 6/10, 4/10, 4/10,
        // 6/10, 4/10, 6/10, 4/10), summing to 800 = 10² · det Λ.
        let st_area2 = |t: &[(i64, i64); 4]| -> i64 {
            (0..4)
                .map(|i| t[i].0 * t[(i + 1) % 4].1 - t[(i + 1) % 4].0 * t[i].1)
                .sum::<i64>()
        };
        let areas: Vec<i64> = ST_TILES.iter().map(st_area2).collect();
        assert_eq!(
            areas,
            vec![120, 120, 80, 80, 120, 80, 120, 80],
            "cell doubled areas (P, mP, P', mP', R0, R1, R2, R3)"
        );
        assert_eq!(areas.iter().sum::<i64>(), 800, "cell doubled areas sum to 10²·det");
    }

    /// Lattice membership for the wavy-courses table, in ×3 (integer)
    /// coordinates: Λ3 = ⟨(3,3),(6,0)⟩ = {(x,y) : y ≡ 0 (mod 3), x−y ≡
    /// 0 (mod 6)} — the x+y-even lattice of ℤ², det 18.
    fn wc_in_lat(x: i64, y: i64) -> bool {
        y.rem_euclid(3) == 0 && (x - y).rem_euclid(6) == 0
    }

    /// The four cell tiles in ×3 coordinates, CCW, in cell order
    /// [T_low, T_high, T_low', T_high']: column 0 (even parity) and
    /// column 1 (odd parity) of course 0.
    const WC_TILES: [[(i64, i64); 3]; 4] = [
        [(0, 0), (3, 1), (3, 3)],
        [(0, 0), (3, 3), (0, 4)],
        [(3, 1), (6, 0), (3, 3)],
        [(3, 3), (6, 0), (6, 4)],
    ];

    /// Does M normalize Λ3? Every candidate here is in GL(2, ℤ)
    /// (|det| = 1), so both basis images in Λ3 imply M(Λ3) = Λ3 (the
    /// lattice is square-metric, so all eight D4 parts normalize it).
    fn wc_normalizes(m: [i64; 4]) -> bool {
        wc_in_lat(3 * m[0] + 3 * m[1], 3 * m[2] + 3 * m[3])
            && wc_in_lat(6 * m[0], 6 * m[2])
    }

    /// The image of tile `t` under g = (M, v), as a sorted 3-point set.
    fn wc_image(t: &[(i64, i64); 3], m: [i64; 4], v: [i64; 2]) -> [(i64, i64); 3] {
        let mut out: Vec<(i64, i64)> = t
            .iter()
            .map(|&(x, y)| (m[0] * x + m[1] * y + v[0], m[2] * x + m[3] * y + v[1]))
            .collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 3] = [(0, 0); 3];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    fn wc_sorted(t: &[(i64, i64); 3]) -> [(i64, i64); 3] {
        let mut out = *t;
        out.sort_unstable();
        out
    }

    /// The sorted 3-point set `t + (dx, dy)`.
    fn wc_shifted(t: &[(i64, i64); 3], dx: i64, dy: i64) -> [(i64, i64); 3] {
        let mut out: Vec<(i64, i64)> = t.iter().map(|&(x, y)| (x + dx, y + dy)).collect();
        out.sort_unstable();
        let mut arr: [(i64, i64); 3] = [(0, 0); 3];
        for (i, p) in out.iter().enumerate() {
            arr[i] = *p;
        }
        arr
    }

    /// Is the sorted set `a` a translate of the sorted set `u` by a
    /// vector of Λ3? Tries every (a_i, u_j) anchor pair; a match on the
    /// first pair fixes λ and the full-set check confirms it.
    fn wc_translate_of(a: &[(i64, i64); 3], u: &[(i64, i64); 3]) -> bool {
        (0..3)
            .into_iter()
            .flat_map(|i| (0..3).map(move |j| (a[i].0 - u[j].0, a[i].1 - u[j].1)))
            .any(|(dx, dy)| {
                wc_in_lat(dx, dy)
                    && (0..3).all(|k| (a[k].0, a[k].1) == (u[k].0 + dx, u[k].1 + dy))
            })
    }

    /// Does g = (M, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for a 4-tile
    /// cell (with M normalizing Λ3, the image of the full tiling is
    /// then exactly the tiling)?
    fn wc_maps_cells(m: [i64; 4], v: [i64; 2]) -> bool {
        WC_TILES.iter().all(|t| {
            let a = wc_image(t, m, v);
            WC_TILES.iter().any(|u| wc_translate_of(&a, &wc_sorted(u)))
        })
    }

    /// The exact cm criterion: g = (M, v) is a symmetry of the tiling
    /// iff M normalizes Λ3 and it maps every cell tile onto a cell
    /// tile plus a lattice translate.
    fn wc_is_sym(m: [i64; 4], v: [i64; 2]) -> bool {
        wc_normalizes(m) && wc_maps_cells(m, v)
    }

    /// Exact cm probe: the lattice translations, the vertical mirror
    /// lines x = k, and the glide family g_j·Λ (g_j(x,y) = (2j+1−x,
    /// y+1), glide (0,1), half-integer axes) are symmetries; the
    /// complete 144-coset enumeration (all eight D4 linear parts that
    /// normalize the square-metric Λ3 × 18 cosets, x3 ∈ {0..5},
    /// y3 ∈ {0..2}) shows no other isometry coset is positive —
    /// exactly the identity coset and the mirror/glide coset (2 pos /
    /// 142 neg; the glide representatives (6j+3,3) lie in the same
    /// coset as the mirrors (6j,0) because (3,3) ∈ Λ3) — and every
    /// named negative from the Python battery (`m82_verify.py`) is
    /// killed.
    #[test]
    fn wavy_courses_wallpaper_group_is_cm() {
        // Structural sanity: the mirror x = 0 (M_MIRX, v = (0,0))
        // swaps the even-column tiles with the odd-column tiles of
        // course 0 — T_low ↔ T_low' + (−2,0), T_high ↔ T_high' +
        // (−2,0), and back (C1 stays C1, C2 stays C2 — no class
        // mixing).
        assert_eq!(wc_image(&WC_TILES[0], M_MIRX, [0, 0]), wc_shifted(&WC_TILES[2], -6, 0));
        assert_eq!(wc_image(&WC_TILES[1], M_MIRX, [0, 0]), wc_shifted(&WC_TILES[3], -6, 0));
        assert_eq!(wc_image(&WC_TILES[2], M_MIRX, [0, 0]), wc_shifted(&WC_TILES[0], -6, 0));
        assert_eq!(wc_image(&WC_TILES[3], M_MIRX, [0, 0]), wc_shifted(&WC_TILES[1], -6, 0));
        // The glide g_0(x,y) = (1−x, y+1) — ×3, (M_MIRX, (3,3)) — maps
        // T_low → T_low' + (−1,1) and T_high → T_high' + (−1,1); its
        // square is the lattice translation (0,2).
        assert_eq!(wc_image(&WC_TILES[0], M_MIRX, [3, 3]), wc_shifted(&WC_TILES[2], -3, 3));
        assert_eq!(wc_image(&WC_TILES[1], M_MIRX, [3, 3]), wc_shifted(&WC_TILES[3], -3, 3));
        // The centering translation (1,1) — ×3, (3,3) — is a symmetry.
        assert!(wc_is_sym([1, 0, 0, 1], [3, 3]), "centering (1,1) must be a symmetry");

        // Positive: the 25 lattice translations (a window of Λ3).
        for i in -2..=2 {
            for j in -2..=2 {
                assert!(
                    wc_is_sym([1, 0, 0, 1], [3 * i + 6 * j, 3 * i]),
                    "translation ({}i + {}j·(2,0)) must be a symmetry",
                    i,
                    j
                );
            }
        }
        // Positive: the vertical mirror lines x = k (k = −2..2), v =
        // (6k, 0) in ×3, and their composites with lattice
        // translations.
        for c in -2i64..=2 {
            assert!(
                wc_is_sym(M_MIRX, [6 * c, 0]),
                "mirror x = {c} must be a symmetry"
            );
        }
        assert!(wc_is_sym(M_MIRX, [0, 6]), "mirror x=0 + (0,2): (0,2) in Λ");
        assert!(wc_is_sym(M_MIRX, [6, -6]), "mirror x=1 + (0,−2)");
        // Positive: the glide samples g_j (j = −1..1) — ×3, v =
        // (6j+3, 3) — axes x = j + 1/2, glide (0,1).
        for j in -1..=1 {
            assert!(
                wc_is_sym(M_MIRX, [6 * j + 3, 3]),
                "glide axis x = {}+1/2 must be a symmetry",
                j
            );
        }

        // Complete coset enumeration: for each of the eight D4 linear
        // parts that normalize Λ3 (a square-metric lattice), all 18
        // v-cosets mod Λ3 (x3 ∈ 0..5, y3 ∈ 0..2). For a fixed part m
        // that normalizes Λ3, the group elements with part m form
        // either nothing or a single coset of Λ3, so one rep per coset
        // is the complete census. Exactly one coset of I (the
        // identity) and one of M_MIRX (the mirrors x = k together with
        // the glides (6j+3,3) — (3,3) ∈ Λ3 puts them in ONE coset) are
        // positive; all 18 cosets of the other six parts are negative
        // — 144 checks, 2 pos / 142 neg (no p2/cmm/p4 upgrade).
        const M_ROT4_270: [i64; 4] = [0, 1, -1, 0];
        for (pi, m) in [
            [1, 0, 0, 1],
            M_MIRX,
            M_MIRY,
            M_ROT2,
            M_ROT4,
            M_ROT4_270,
            M_DIAG,
            M_ANTI,
        ]
        .into_iter()
        .enumerate()
        {
            for x3 in 0..6 {
                for y3 in 0..3 {
                    let expected = match pi {
                        0 => x3 == 0 && y3 == 0,
                        1 => x3 == 0 && y3 == 0,
                        _ => false,
                    };
                    assert_eq!(
                        wc_is_sym(m, [x3, y3]),
                        expected,
                        "coset ({x3},{y3}) of linear part {pi}"
                    );
                }
            }
        }

        // Named negatives (from the Python battery): off-lattice
        // translations.
        for v in [[3, 0], [0, 3], [3, 1], [0, 1]] {
            assert!(!wc_is_sym([1, 0, 0, 1], v), "off-lattice translation {v:?}");
        }
        // Vertical mirror off the x = k family.
        assert!(!wc_is_sym(M_MIRX, [3, 0]), "vertical mirror x = 1/2 must not be");
        // Vertical glides with the wrong slide: the integer-axis
        // glide (0,1) and the half-integer-axis glide (0,2) are NOT
        // symmetries (the glide coset is the one containing (3,3)).
        assert!(!wc_is_sym(M_MIRX, [0, 3]), "vertical glide (x = 0, slide (0,1))");
        assert!(!wc_is_sym(M_MIRX, [3, 6]), "vertical glide (x = 1/2, slide (0,2))");
        // Horizontal mirrors and the horizontal glide.
        assert!(!wc_is_sym(M_MIRY, [0, 0]), "horizontal mirror y = 0 must not be");
        assert!(!wc_is_sym(M_MIRY, [0, 3]), "horizontal mirror y = 1/2 must not be");
        assert!(!wc_is_sym(M_MIRY, [3, 3]), "horizontal glide (axis y = 1/2) must not be");
        // Half-turns about the thirds-grid centers the pattern could
        // plausibly carry — v = 2·center in ×3.
        for (cx, cy) in [
            (0, 0),
            (3, 3),
            (0, 3),
            (3, 1),
            (3, 0),
            (6, 0),
            (3, 2),
            (6, 4),
            (0, 2),
            (6, 3),
            (9, 0),
            (6, 6),
        ] {
            assert!(!wc_is_sym(M_ROT2, [2 * cx, 2 * cy]), "half-turn about ({cx},{cy})/3");
        }
        // Extra D4 parts: spot checks (all 18 cosets each already
        // proven negative by the enumeration).
        for m in [M_ROT4, M_ROT4_270, M_DIAG, M_ANTI] {
            for v in [[0, 0], [3, 3], [6, 6], [3, 0], [0, 3]] {
                assert!(!wc_is_sym(m, v), "extra D4 part {m:?} at {v:?}");
            }
        }

        // Sanity: the ×3 cell tiles are CCW with doubled areas 6, 12,
        // 6, 12 (areas 1/3, 2/3, 1/3, 2/3), summing to 36 = 9 · 2 ·
        // det Λ, and the two congruence classes are incongruent (C1
        // carries the 2/3 edge — squared 4 ×9 — where C2 carries the
        // 4/3 edge — squared 16 ×9).
        let wc_area2 = |t: &[(i64, i64); 3]| -> i64 {
            (0..3)
                .map(|i| t[i].0 * t[(i + 1) % 3].1 - t[(i + 1) % 3].0 * t[i].1)
                .sum::<i64>()
        };
        let areas: Vec<i64> = WC_TILES.iter().map(wc_area2).collect();
        assert_eq!(
            areas,
            vec![6, 12, 6, 12],
            "cell doubled areas (T_low, T_high, T_low', T_high')"
        );
        assert_eq!(areas.iter().sum::<i64>(), 36, "cell doubled areas sum to 9·2·det");
        let wc_sq_edges = |t: &[(i64, i64); 3]| -> Vec<i64> {
            let mut e: Vec<i64> = (0..3)
                .map(|i| {
                    let dx = t[i].0 - t[(i + 1) % 3].0;
                    let dy = t[i].1 - t[(i + 1) % 3].1;
                    dx * dx + dy * dy
                })
                .collect();
            e.sort_unstable();
            e
        };
        assert_eq!(
            wc_sq_edges(&WC_TILES[0]),
            vec![4, 10, 18],
            "C1 squared edges ×9 (2/3, sqrt(10)/3, sqrt(2))"
        );
        assert_eq!(
            wc_sq_edges(&WC_TILES[1]),
            vec![10, 16, 18],
            "C2 squared edges ×9 (4/3 edge present)"
        );
    }

    // ============ chiral-cut-honeycomb (M83, p3) ============

    /// The ten distinct cell vertices, ×320, in first-encounter
    /// tile order (O, W0, V0, V1, V2, W2, V3, V4, W4, V5): the
    /// p3 probe candidate set.
    const P3_VERTS: [[i64; 4]; 10] = [
        [0, 0, 0, 0],
        [-40, 128, 160, 0],
        [0, 320, 320, 0],
        [0, 640, 0, 0],
        [0, 320, -320, 0],
        [20, 176, -144, -20],
        [0, -320, -320, 0],
        [0, -640, 0, 0],
        [20, -304, -16, 20],
        [0, -320, 320, 0],
    ];

    /// The three cell tiles, ×320, CCW, 6 vertices each: tile k =
    /// [O, W_{2k}, V_{2k}, V_{2k+1}, V_{2k+2}, W_{2k+4}]. R120 about
    /// O cycles tile0 -> tile1 -> tile2 exactly (no translate).
    const P3_TILES: [[[i64; 4]; 6]; 3] = [
        [
            [0, 0, 0, 0],
            [-40, 128, 160, 0],
            [0, 320, 320, 0],
            [0, 640, 0, 0],
            [0, 320, -320, 0],
            [20, 176, -144, -20],
        ],
        [
            [0, 0, 0, 0],
            [20, 176, -144, -20],
            [0, 320, -320, 0],
            [0, -320, -320, 0],
            [0, -640, 0, 0],
            [20, -304, -16, 20],
        ],
        [
            [0, 0, 0, 0],
            [20, -304, -16, 20],
            [0, -640, 0, 0],
            [0, -320, 320, 0],
            [0, 320, 320, 0],
            [-40, 128, 160, 0],
        ],
    ];

    /// Off-lattice translations that are NOT symmetries (×320):
    /// (√3,0) [the vertex coset], a2/2, (1,0), (3,0), (1/2,0),
    /// (√3, 1+√3) [off-lattice in both slots].
    const P3_TRANSLATION_NEGS: [[i64; 4]; 6] = [
        [0, 0, 320, 0],
        [0, 480, 160, 0],
        [320, 0, 0, 0],
        [960, 0, 0, 0],
        [160, 0, 0, 0],
        [0, 320, 320, 320],
    ];

    /// The 11 D6 linear parts (non-identity), exact ×320, as the
    /// INTEGER matrix 2L (row-major 4×4) in (A,B,C,D) coords: applying
    /// the map to a ×320 point p is (2L·p)/2, exact division on the
    /// vertex set. Re-derived from exact cos/sin values and cross-
    /// checked against the hx_engine D6 definitions in m83_verify.py.
    const P3_LIDENT: [[i64; 4]; 4] = [
        [2, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 2],
    ];
    const P3_LROT60: [[i64; 4]; 4] = [
        [1, 0, 0, -3],
        [0, 1, 3, 0],
        [0, -1, 1, 0],
        [1, 0, 0, 1],
    ];
    const P3_LROT120: [[i64; 4]; 4] = [
        [-1, 0, 0, -3],
        [0, -1, 3, 0],
        [0, -1, -1, 0],
        [1, 0, 0, -1],
    ];
    const P3_LROT180: [[i64; 4]; 4] = [
        [-2, 0, 0, 0],
        [0, -2, 0, 0],
        [0, 0, -2, 0],
        [0, 0, 0, -2],
    ];
    const P3_LROT240: [[i64; 4]; 4] = [
        [-1, 0, 0, 3],
        [0, -1, -3, 0],
        [0, 1, -1, 0],
        [-1, 0, 0, -1],
    ];
    const P3_LROT300: [[i64; 4]; 4] = [
        [1, 0, 0, 3],
        [0, 1, -3, 0],
        [0, 1, 1, 0],
        [-1, 0, 0, 1],
    ];
    const P3_LREFL0: [[i64; 4]; 4] = [
        [2, 0, 0, 0],
        [0, -2, 0, 0],
        [0, 0, 2, 0],
        [0, 0, 0, -2],
    ];
    const P3_LREFL30: [[i64; 4]; 4] = [
        [1, 0, 0, 3],
        [0, -1, 3, 0],
        [0, 1, 1, 0],
        [1, 0, 0, -1],
    ];
    const P3_LREFL60: [[i64; 4]; 4] = [
        [-1, 0, 0, 3],
        [0, 1, 3, 0],
        [0, 1, -1, 0],
        [1, 0, 0, 1],
    ];
    const P3_LREFL90: [[i64; 4]; 4] = [
        [-2, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 0, -2, 0],
        [0, 0, 0, 2],
    ];
    const P3_LREFL120: [[i64; 4]; 4] = [
        [-1, 0, 0, -3],
        [0, 1, -3, 0],
        [0, -1, -1, 0],
        [-1, 0, 0, 1],
    ];
    const P3_LREFL150: [[i64; 4]; 4] = [
        [1, 0, 0, -3],
        [0, -1, -3, 0],
        [0, -1, 1, 0],
        [-1, 0, 0, -1],
    ];

    /// Lattice membership in ×320 integer coordinates:
    /// Λ320 = ⟨(0,0,640,0), (0,960,320,0)⟩, i.e. A = D = 0,
    /// B ≡ 0 (mod 960), C − B/3 ≡ 0 (mod 640).
    fn p3_in_lat(v: [i64; 4]) -> bool {
        v[0] == 0
            && v[3] == 0
            && v[1].rem_euclid(960) == 0
            && (v[2] - v[1] / 3).rem_euclid(640) == 0
    }

    /// Coupled-floor coset reduction for points with A = D = 0:
    /// i-coord = (3C−B)/1920, j-coord = B/960, canonical iff both lie
    /// in [0,1)². The two floors are COUPLED (the i-floor depends on
    /// B): decoupled floors misplace representatives.
    fn p3_reduce320(v: [i64; 4]) -> [i64; 4] {
        let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
        let fj = b.div_euclid(960);
        let bm = b - 960 * fj;
        let fi = (3 * c - b).div_euclid(1920);
        let cm = c - 320 * fj - 640 * fi;
        [a, bm, cm, d]
    }

    /// Apply g = (L, v) to a ×320 point: L is given as the integer
    /// matrix 2L (basis images have denominator ≤ 2, so the division
    /// is exact on the vertex set).
    fn p3_apply(l2: &[[i64; 4]; 4], v: [i64; 4], p: [i64; 4]) -> [i64; 4] {
        let mut out = [0i64; 4];
        for i in 0..4 {
            let s = (0..4).map(|j| l2[i][j] * p[j]).sum::<i64>();
            assert!(s % 2 == 0, "2L·p must be even, got {s}");
            out[i] = s / 2 + v[i];
        }
        out
    }

    /// Does g = (L, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for the
    /// 3-tile cell?
    fn p3_maps_cells(l2: &[[i64; 4]; 4], v: [i64; 4]) -> bool {
        for t in P3_TILES {
            let mut gt: Vec<[i64; 4]> = t.iter().map(|p| p3_apply(l2, v, *p)).collect();
            gt.sort_unstable();
            let mut ok = false;
            for u in P3_TILES {
                let mut us = u.to_vec();
                us.sort_unstable();
                'anchors: for i in 0..6 {
                    for j in 0..6 {
                        let lam = [
                            gt[i][0] - us[j][0],
                            gt[i][1] - us[j][1],
                            gt[i][2] - us[j][2],
                            gt[i][3] - us[j][3],
                        ];
                        if !p3_in_lat(lam) {
                            continue;
                        }
                        let mut all = true;
                        for k in 0..6 {
                            if gt[k][0] != us[k][0] + lam[0]
                                || gt[k][1] != us[k][1] + lam[1]
                                || gt[k][2] != us[k][2] + lam[2]
                                || gt[k][3] != us[k][3] + lam[3]
                            {
                                all = false;
                                break;
                            }
                        }
                        if all {
                            ok = true;
                            break 'anchors;
                        }
                    }
                }
                if ok {
                    break;
                }
            }
            if !ok {
                return false;
            }
        }
        true
    }

    /// Does the linear part normalize the lattice? (Both basis images
    /// in Λ; finite index with |det| = 1 then gives equality.)
    fn p3_normalizes(l2: &[[i64; 4]; 4]) -> bool {
        p3_in_lat(p3_apply(l2, [0; 4], [0, 0, 640, 0]))
            && p3_in_lat(p3_apply(l2, [0; 4], [0, 960, 320, 0]))
    }

    /// The exact p3 criterion: g = (L, v) is a symmetry of the tiling
    /// iff L normalizes Λ320 and it maps every cell tile onto a cell
    /// tile plus a lattice translate.
    fn p3_is_sym(l2: &[[i64; 4]; 4], v: [i64; 4]) -> bool {
        p3_normalizes(l2) && p3_maps_cells(l2, v)
    }

    /// Exact p3 probe (×320 integer scheme, mirroring `m83_verify.py`):
    /// positives are exactly the translations (25/25 in a ±2 window)
    /// and the zero coset of rot120 and of rot240 — everything else is
    /// negative across the full candidate census (the HX-STUDY battery
    /// numbers: 18/27/18/27/18 tested cosets for the five rotations,
    /// 30 for each of the six reflections), six off-lattice
    /// translations are killed, and the three rot120 centre orbits
    /// (hex centre, even corner, odd corner — p3's "333" fingerprint,
    /// all genuinely 3-fold) reduce from the closed form
    /// c(i,j) = (I − R120)⁻¹(i·a1 + j·a2) = (√3·i, i + 2j),
    /// ×320: (0, 320(i+2j), 320i, 0).
    #[test]
    fn chiral_cut_honeycomb_wallpaper_group_is_p3() {
        // Structural sanity: R120 cycles the cell tiles exactly about
        // O (no translate) and R240 = R120².
        let im = |l2: &[[i64; 4]; 4], t: &[[i64; 4]; 6]| -> Vec<[i64; 4]> {
            let mut v: Vec<[i64; 4]> = t.iter().map(|p| p3_apply(l2, [0; 4], *p)).collect();
            v.sort_unstable();
            v
        };
        let sorted = |t: &[[i64; 4]; 6]| -> Vec<[i64; 4]> {
            let mut v = t.to_vec();
            v.sort_unstable();
            v
        };
        assert_eq!(
            im(&P3_LROT120, &P3_TILES[0]),
            sorted(&P3_TILES[1]),
            "R120(tile0) == tile1 exactly"
        );
        assert_eq!(
            im(&P3_LROT240, &P3_TILES[0]),
            sorted(&P3_TILES[2]),
            "R240(tile0) == tile2 exactly"
        );
        assert_eq!(im(&P3_LIDENT, &P3_TILES[0]), sorted(&P3_TILES[0]));

        // Positive: the 25 lattice translations of the ±2 window.
        for i in -2i64..=2 {
            for j in -2i64..=2 {
                assert!(
                    p3_is_sym(&P3_LIDENT, [0, 960 * j, 640 * i + 320 * j, 0]),
                    "lattice translation ({i},{j}) must be a symmetry"
                );
            }
        }
        // Negative: the six off-lattice translations.
        for v in P3_TRANSLATION_NEGS {
            assert!(
                !p3_is_sym(&P3_LIDENT, v),
                "off-lattice {v:?} must not be a symmetry"
            );
        }

        // Full candidate census for the 11 non-identity D6 parts:
        // candidates = {q − L·p : p, q in the 10-vertex set}, reduced
        // to one canonical rep per coset. Only the zero coset of
        // rot120 and of rot240 is positive; the other nine parts are
        // entirely negative (no 60°/180°/300° rotations, no mirror in
        // any of the six families, no glides — p3, not p6 or any
        // pmm-family upgrade).
        const PARTS: [(&str, &[[i64; 4]; 4], usize, bool); 11] = [
            ("rot60", &P3_LROT60, 18, false),
            ("rot120", &P3_LROT120, 27, true),
            ("rot180", &P3_LROT180, 18, false),
            ("rot240", &P3_LROT240, 27, true),
            ("rot300", &P3_LROT300, 18, false),
            ("refl0", &P3_LREFL0, 30, false),
            ("refl30", &P3_LREFL30, 30, false),
            ("refl60", &P3_LREFL60, 30, false),
            ("refl90", &P3_LREFL90, 30, false),
            ("refl120", &P3_LREFL120, 30, false),
            ("refl150", &P3_LREFL150, 30, false),
        ];
        for (name, l2, tested, has_pos) in PARTS {
            assert!(p3_normalizes(l2), "{name}: linear part must normalize the lattice");
            let mut cands: Vec<[i64; 4]> = Vec::new();
            for p in P3_VERTS {
                let mp = p3_apply(l2, [0; 4], p);
                for q in P3_VERTS {
                    let v = p3_reduce320([
                        q[0] - mp[0],
                        q[1] - mp[1],
                        q[2] - mp[2],
                        q[3] - mp[3],
                    ]);
                    if !cands.contains(&v) {
                        cands.push(v);
                    }
                }
            }
            assert_eq!(cands.len(), tested, "{name}: tested coset count");
            let mut pos = 0;
            for v in &cands {
                let sym = p3_is_sym(l2, *v);
                if sym {
                    pos += 1;
                }
                assert_eq!(
                    sym,
                    has_pos && *v == [0, 0, 0, 0],
                    "{name}: candidate {v:?}"
                );
            }
            assert_eq!(pos, usize::from(has_pos), "{name}: positive cosets");
        }

        // The three rot120 centre orbits mod the lattice: hex centre,
        // even corner, odd corner — all three genuinely 3-fold.
        let mut orb: Vec<[i64; 4]> = Vec::new();
        for i in -3i64..=3 {
            for j in -3i64..=3 {
                let v = p3_reduce320([0, 320 * (i + 2 * j), 320 * i, 0]);
                if !orb.contains(&v) {
                    orb.push(v);
                }
            }
        }
        orb.sort_unstable();
        let want_orb: [[i64; 4]; 3] = [[0, 0, 0, 0], [0, 320, 320, 0], [0, 640, 640, 0]];
        assert_eq!(orb, want_orb.to_vec(), "rot120 centre orbits mod lattice");
        // Each orbit rep c satisfies c − R120·c ∈ Λ (R120 about c is
        // a symmetry).
        for c in want_orb {
            let d = p3_apply(&P3_LROT120, [0; 4], c);
            assert!(
                p3_in_lat([c[0] - d[0], c[1] - d[1], c[2] - d[2], c[3] - d[3]]),
                "orbit rep {c:?}: c − R120·c in Λ"
            );
        }

        // Cell arithmetic ×320²: doubled area per tile (0, 409600) =
        // 4√3·320², sum (0, 1228800) = 2·det·320², and the
        // squared-edge multiset 2×(409600,0), 2×(94784,−12800),
        // 2×(115264,12800) identical across the three tiles (one
        // congruence class).
        let mut areas: Vec<(i64, i64)> = Vec::new();
        let mut edges: Vec<Vec<(i64, i64)>> = Vec::new();
        for t in P3_TILES {
            let mut area = (0i64, 0i64);
            let mut es: Vec<(i64, i64)> = Vec::new();
            for i in 0..6 {
                let (u, w) = (t[i], t[(i + 1) % 6]);
                area.0 += u[0] * w[1] + 3 * u[2] * w[3] - w[0] * u[1] - 3 * w[2] * u[3];
                area.1 += u[0] * w[3] + u[2] * w[1] - w[0] * u[3] - w[2] * u[1];
                let (da, db, dc, dd) = (w[0] - u[0], w[1] - u[1], w[2] - u[2], w[3] - u[3]);
                es.push((
                    da * da + db * db + 3 * dc * dc + 3 * dd * dd,
                    2 * da * dc + 2 * db * dd,
                ));
            }
            es.sort_unstable();
            edges.push(es.clone());
            areas.push(area);
        }
        assert_eq!(areas, vec![(0, 409600); 3], "doubled tile areas ×320²");
        assert_eq!(
            areas[0].0 + areas[1].0 + areas[2].0,
            0,
            "real part of doubled cell area"
        );
        assert_eq!(
            areas[0].1 + areas[1].1 + areas[2].1,
            1_228_800,
            "sum = 2·det·320²"
        );
        let want_edges: Vec<(i64, i64)> = vec![
            (94784, -12800),
            (94784, -12800),
            (115264, 12800),
            (115264, 12800),
            (409600, 0),
            (409600, 0),
        ];
        for e in &edges {
            assert_eq!(*e, want_edges, "squared-edge multiset per tile");
        }
    }

    // ============ split-triangle-grid (M84, p31m) ============

    /// The ten distinct cell vertices, ×240, in first-encounter tile
    /// order: the p31m probe candidate set.
    const P31M_VERTS: [[i64; 4]; 10] = [
        [0, 0, 0, 0],
        [144, -6, 0, 32],
        [216, 3, -3, 152],
        [240, 0, 0, 240],
        [360, 3, 3, 56],
        [384, 6, 0, 208],
        [456, -3, -3, 88],
        [480, 0, 0, 0],
        [600, -3, 3, 184],
        [720, 0, 0, 240],
    ];

    /// The eight cell tiles, ×240, CCW, JSON order: tile 0 the inner
    /// equilateral triangle of the up-triangle U, tiles 1–3 its corner
    /// quads (one per edge), tile 4 the inner triangle of the
    /// down-triangle D (the mirror image of tile 0), tiles 5–7 its
    /// corner quads. Tiles 0 and 4 are triangles (3 real vertices; the
    /// 4th slot is padding [0,0,0,0]).
    const P31M_TILE_VERTS: [[[i64; 4]; 4]; 8] = [
        [
            [144, -6, 0, 32],
            [360, 3, 3, 56],
            [216, 3, -3, 152],
            [0, 0, 0, 0],
        ],
        [
            [0, 0, 0, 0],
            [480, 0, 0, 0],
            [360, 3, 3, 56],
            [144, -6, 0, 32],
        ],
        [
            [480, 0, 0, 0],
            [240, 0, 0, 240],
            [216, 3, -3, 152],
            [360, 3, 3, 56],
        ],
        [
            [240, 0, 0, 240],
            [0, 0, 0, 0],
            [144, -6, 0, 32],
            [216, 3, -3, 152],
        ],
        [
            [456, -3, -3, 88],
            [600, -3, 3, 184],
            [384, 6, 0, 208],
            [0, 0, 0, 0],
        ],
        [
            [384, 6, 0, 208],
            [600, -3, 3, 184],
            [720, 0, 0, 240],
            [240, 0, 0, 240],
        ],
        [
            [600, -3, 3, 184],
            [456, -3, -3, 88],
            [480, 0, 0, 0],
            [720, 0, 0, 240],
        ],
        [
            [456, -3, -3, 88],
            [384, 6, 0, 208],
            [240, 0, 0, 240],
            [480, 0, 0, 0],
        ],
    ];
    const P31M_TILE_LEN: [usize; 8] = [3, 4, 4, 4, 3, 4, 4, 4];

    /// Off-lattice translations that are NOT symmetries, ×240: (1,0),
    /// (0,1), (1/2,0), (1/2, √3/2), (√3,0).
    const P31M_TRANSLATION_NEGS: [[i64; 4]; 5] = [
        [240, 0, 0, 0],
        [0, 240, 0, 0],
        [120, 0, 0, 0],
        [120, 0, 0, 120],
        [0, 0, 240, 0],
    ];

    /// Lattice membership in ×240 integer coordinates:
    /// Λ240 = ⟨(480,0,0,0), (240,0,0,240)⟩, i.e. B = C = 0,
    /// D ≡ 0 (mod 240), A − D ≡ 0 (mod 480).
    fn p31m_in_lat(v: [i64; 4]) -> bool {
        v[1] == 0
            && v[2] == 0
            && v[3].rem_euclid(240) == 0
            && (v[0] - v[3]).rem_euclid(480) == 0
    }

    /// Coset reduction for ×240 points: j = ⌊D/240⌉ (div_euclid),
    /// subtract j·a2, then i on A−240j mod 480; canonical reps have
    /// A ∈ [0,480), D ∈ [0,240) (B, C pass through — Λ has no B, C
    /// content).
    fn p31m_reduce(v: [i64; 4]) -> [i64; 4] {
        let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
        let j = d.div_euclid(240);
        let dm = d - 240 * j;
        let am = a - 240 * j;
        let i = am.div_euclid(480);
        [am - 480 * i, b, c, dm]
    }

    /// Apply g = (L, v) to a ×240 point: L is given as the integer
    /// matrix 2L (basis images have denominator ≤ 2, so the division
    /// is exact on the vertex set and the centre reps).
    fn p31m_apply(l2: &[[i64; 4]; 4], v: [i64; 4], p: [i64; 4]) -> [i64; 4] {
        let mut out = [0i64; 4];
        for i in 0..4 {
            let s = (0..4).map(|j| l2[i][j] * p[j]).sum::<i64>();
            assert!(s % 2 == 0, "2L·p must be even, got {s}");
            out[i] = s / 2 + v[i];
        }
        out
    }

    /// Does g = (L, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for the
    /// 8-tile cell? Tiles 0 and 4 are triangles, the other six quads,
    /// so the vertex count of the target must match before anchor
    /// matching.
    fn p31m_maps_cells(l2: &[[i64; 4]; 4], v: [i64; 4]) -> bool {
        for t in 0..8 {
            let n = P31M_TILE_LEN[t];
            let mut gt: Vec<[i64; 4]> = (0..n)
                .map(|k| p31m_apply(l2, v, P31M_TILE_VERTS[t][k]))
                .collect();
            gt.sort_unstable();
            let mut ok = false;
            'tiles: for u in 0..8 {
                if P31M_TILE_LEN[u] != n {
                    continue;
                }
                let mut us: Vec<[i64; 4]> =
                    (0..n).map(|k| P31M_TILE_VERTS[u][k]).collect();
                us.sort_unstable();
                'anchors: for i in 0..n {
                    for j in 0..n {
                        let lam = [
                            gt[i][0] - us[j][0],
                            gt[i][1] - us[j][1],
                            gt[i][2] - us[j][2],
                            gt[i][3] - us[j][3],
                        ];
                        if !p31m_in_lat(lam) {
                            continue;
                        }
                        let mut all = true;
                        for k in 0..n {
                            if gt[k][0] != us[k][0] + lam[0]
                                || gt[k][1] != us[k][1] + lam[1]
                                || gt[k][2] != us[k][2] + lam[2]
                                || gt[k][3] != us[k][3] + lam[3]
                            {
                                all = false;
                                break;
                            }
                        }
                        if all {
                            ok = true;
                            break 'anchors;
                        }
                    }
                }
                if ok {
                    break 'tiles;
                }
            }
            if !ok {
                return false;
            }
        }
        true
    }

    /// Does the linear part normalize the lattice? (Both basis images
    /// in Λ; finite index with |det| = 1 then gives equality.)
    fn p31m_normalizes(l2: &[[i64; 4]; 4]) -> bool {
        p31m_in_lat(p31m_apply(l2, [0; 4], [480, 0, 0, 0]))
            && p31m_in_lat(p31m_apply(l2, [0; 4], [240, 0, 0, 240]))
    }

    /// The exact p31m criterion: g = (L, v) is a symmetry of the
    /// tiling iff L normalizes Λ240 and it maps every cell tile onto a
    /// cell tile plus a lattice translate.
    fn p31m_is_sym(l2: &[[i64; 4]; 4], v: [i64; 4]) -> bool {
        p31m_normalizes(l2) && p31m_maps_cells(l2, v)
    }

    /// Is centre c on one of the tiling's mirror lines? The positive
    /// mirror cosets are exactly the zero cosets of the three grid-line
    /// families refl0/refl60/refl120 (the probe below), whose mirror
    /// lines all pass through the origin, so c lies on a mirror iff
    /// c − L·c ∈ Λ for one of the three linear parts.
    fn p31m_on_mirror(c: [i64; 4]) -> bool {
        for l2 in [&P3_LREFL0, &P3_LREFL60, &P3_LREFL120] {
            let d = p31m_apply(l2, [0; 4], c);
            if p31m_in_lat([c[0] - d[0], c[1] - d[1], c[2] - d[2], c[3] - d[3]]) {
                return true;
            }
        }
        false
    }

    /// Exact p31m probe (×240 integer scheme, mirroring
    /// `m84_verify.py`): positives are exactly the translations
    /// (25/25 in a ±2 window) and the zero coset of rot120, rot240
    /// and the three grid-line mirror families refl0/refl60/refl120 —
    /// everything else negative across the full candidate census
    /// (43 tested cosets for the five positive parts, 28 for the six
    /// negative parts: no 60°/180°/300° rotation, so not p6 or p6m;
    /// no 30°/90°/150° mirror family, no glides). Five off-lattice
    /// translations are killed. The three rot120 centre orbits — grid
    /// vertices, up-triangle centroids, down-triangle centroids —
    /// reduce from the closed form c(i,j) ×240 =
    /// (240i, 0, 0, 80(i+2j)), and the on-mirror incidence is
    /// [grid vertex: on, centroids: off, off]: exactly ONE of the three
    /// orbits on mirrors, which is p31m's fingerprint — p3m1 would put
    /// all three on mirrors.
    #[test]
    fn split_triangle_grid_wallpaper_group_is_p31m() {
        // Positive: the 25 lattice translations of the ±2 window.
        for i in -2i64..=2 {
            for j in -2i64..=2 {
                assert!(
                    p31m_is_sym(&P3_LIDENT, [480 * i + 240 * j, 0, 0, 240 * j]),
                    "lattice translation ({i},{j}) must be a symmetry"
                );
            }
        }
        // Negative: the five off-lattice translations.
        for v in P31M_TRANSLATION_NEGS {
            assert!(
                !p31m_is_sym(&P3_LIDENT, v),
                "off-lattice {v:?} must not be a symmetry"
            );
        }

        // Full candidate census for the 11 non-identity D6 parts:
        // candidates = {q − L·p : p, q in the 10-vertex set}, reduced
        // to one canonical rep per coset. Only the zero coset is
        // positive, and only for the five p31m parts (rot120, rot240,
        // refl0, refl60, refl120).
        const PARTS: [(&str, &[[i64; 4]; 4], usize, bool); 11] = [
            ("rot60", &P3_LROT60, 28, false),
            ("rot120", &P3_LROT120, 43, true),
            ("rot180", &P3_LROT180, 28, false),
            ("rot240", &P3_LROT240, 43, true),
            ("rot300", &P3_LROT300, 28, false),
            ("refl0", &P3_LREFL0, 43, true),
            ("refl30", &P3_LREFL30, 28, false),
            ("refl60", &P3_LREFL60, 43, true),
            ("refl90", &P3_LREFL90, 28, false),
            ("refl120", &P3_LREFL120, 43, true),
            ("refl150", &P3_LREFL150, 28, false),
        ];
        for (name, l2, tested, has_pos) in PARTS {
            assert!(
                p31m_normalizes(l2),
                "{name}: linear part must normalize the lattice"
            );
            let mut cands: Vec<[i64; 4]> = Vec::new();
            for p in P31M_VERTS {
                let mp = p31m_apply(l2, [0; 4], p);
                for q in P31M_VERTS {
                    let v = p31m_reduce([
                        q[0] - mp[0],
                        q[1] - mp[1],
                        q[2] - mp[2],
                        q[3] - mp[3],
                    ]);
                    if !cands.contains(&v) {
                        cands.push(v);
                    }
                }
            }
            assert_eq!(cands.len(), tested, "{name}: tested coset count");
            let mut pos = 0;
            for v in &cands {
                let sym = p31m_is_sym(l2, *v);
                if sym {
                    pos += 1;
                }
                assert_eq!(
                    sym,
                    has_pos && *v == [0, 0, 0, 0],
                    "{name}: candidate {v:?}"
                );
            }
            assert_eq!(pos, usize::from(has_pos), "{name}: positive cosets");
        }

        // The three rot120 centre orbits mod the lattice: grid
        // vertices, up-triangle centroids, down-triangle centroids
        // (closed form c(i,j) ×240 = (240i, 0, 0, 80(i+2j))).
        let mut orb: Vec<[i64; 4]> = Vec::new();
        for i in -3i64..=3 {
            for j in -3i64..=3 {
                let v = p31m_reduce([240 * i, 0, 0, 80 * (i + 2 * j)]);
                if !orb.contains(&v) {
                    orb.push(v);
                }
            }
        }
        orb.sort_unstable();
        let want_orb: [[i64; 4]; 3] = [[0, 0, 0, 0], [0, 0, 0, 160], [240, 0, 0, 80]];
        assert_eq!(orb, want_orb.to_vec(), "rot120 centre orbits mod lattice");
        // Each orbit rep c satisfies c − R120·c ∈ Λ (R120 about c is
        // a symmetry).
        for c in want_orb {
            let d = p31m_apply(&P3_LROT120, [0; 4], c);
            assert!(
                p31m_in_lat([c[0] - d[0], c[1] - d[1], c[2] - d[2], c[3] - d[3]]),
                "orbit rep {c:?}: c − R120·c in Λ"
            );
        }
        // Incidence — the p31m/p3m1 discriminator: exactly one of the
        // three centre orbits lies on a mirror (the grid-vertex orbit);
        // both centroid orbits are off every mirror. In p3m1 all three
        // orbits would be on mirrors.
        assert!(
            p31m_on_mirror([0, 0, 0, 0]),
            "grid-vertex centres lie on the grid-line mirrors"
        );
        assert!(!p31m_on_mirror([240, 0, 0, 80]), "up-centroid centres off every mirror");
        assert!(
            !p31m_on_mirror([0, 0, 0, 160]),
            "down-centroid centres off every mirror"
        );

        // Cell arithmetic ×240²: doubled area (re, im) per tile — the
        // two inner triangles (2592, 24246), the six corner quads
        // (−864, 30318); sum (0, 230400) = 2·det·240² (det = 2√3).
        // Squared-edge multiset: two congruence classes — triangles
        // (48492,1728) × 3, quads (23844,−384) × 2, (48492,1728),
        // (230400,0).
        let mut areas: Vec<(i64, i64)> = Vec::new();
        let mut edges: Vec<Vec<(i64, i64)>> = Vec::new();
        for (t, n) in P31M_TILE_VERTS.iter().zip(P31M_TILE_LEN.iter()) {
            let mut area = (0i64, 0i64);
            let mut es: Vec<(i64, i64)> = Vec::new();
            for i in 0..*n {
                let (u, w) = (t[i], t[(i + 1) % n]);
                area.0 += u[0] * w[1] + 3 * u[2] * w[3] - w[0] * u[1] - 3 * w[2] * u[3];
                area.1 += u[0] * w[3] + u[2] * w[1] - w[0] * u[3] - w[2] * u[1];
                let (da, db, dc, dd) = (
                    w[0] - u[0],
                    w[1] - u[1],
                    w[2] - u[2],
                    w[3] - u[3],
                );
                es.push((
                    da * da + db * db + 3 * dc * dc + 3 * dd * dd,
                    2 * da * dc + 2 * db * dd,
                ));
            }
            es.sort_unstable();
            edges.push(es.clone());
            areas.push(area);
        }
        let want_areas: [(i64, i64); 8] = [
            (2592, 24246),
            (-864, 30318),
            (-864, 30318),
            (-864, 30318),
            (2592, 24246),
            (-864, 30318),
            (-864, 30318),
            (-864, 30318),
        ];
        assert_eq!(areas, want_areas.to_vec(), "doubled tile areas ×240²");
        assert_eq!(
            areas.iter().map(|a| a.0).sum::<i64>(),
            0,
            "real part of doubled cell area"
        );
        assert_eq!(
            areas.iter().map(|a| a.1).sum::<i64>(),
            230_400,
            "sum = 2·det·240² (det 2√3)"
        );
        let want_tri: Vec<(i64, i64)> = vec![(48492, 1728); 3];
        let want_quad: Vec<(i64, i64)> = vec![
            (23844, -384),
            (23844, -384),
            (48492, 1728),
            (230400, 0),
        ];
        for (e, n) in edges.iter().zip(P31M_TILE_LEN.iter()) {
            let want = if *n == 3 { &want_tri } else { &want_quad };
            assert_eq!(
                *e, *want,
                "squared-edge multiset (two congruence classes: achiral triangles, chiral quads)"
            );
        }
    }

    // ============ alcove-cut-grid (M85, p3m1) ============

    /// The 13 distinct cell vertices, ×126, sorted by (A, D): the
    /// p3m1 probe candidate set.
    const P3M1_VERTS: [[i64; 4]; 13] = [
        [0, 0, 0, 0],
        [126, 0, 0, -126],
        [126, 0, 0, 126],
        [144, 0, 0, -56],
        [144, 0, 0, 56],
        [222, 0, 0, -82],
        [222, 0, 0, 82],
        [252, 0, 0, 0],
        [378, 0, 0, -126],
        [378, 0, 0, 126],
        [390, 0, 0, -26],
        [390, 0, 0, 26],
        [504, 0, 0, 0],
    ];

    /// The 18 cell tiles, ×126, CCW, JSON order: each small triangle
    /// (U0, D0, U1, D1, U2, D2 of the side-2 cell) cut into three
    /// triangles from its alcove point. All 18 tiles are triangles.
    const P3M1_TILE_VERTS: [[[i64; 4]; 3]; 18] = [
        [
            [144, 0, 0, 56],
            [0, 0, 0, 0],
            [252, 0, 0, 0],
        ],
        [
            [144, 0, 0, 56],
            [252, 0, 0, 0],
            [126, 0, 0, 126],
        ],
        [
            [144, 0, 0, 56],
            [126, 0, 0, 126],
            [0, 0, 0, 0],
        ],
        [
            [144, 0, 0, -56],
            [126, 0, 0, -126],
            [252, 0, 0, 0],
        ],
        [
            [144, 0, 0, -56],
            [252, 0, 0, 0],
            [0, 0, 0, 0],
        ],
        [
            [144, 0, 0, -56],
            [0, 0, 0, 0],
            [126, 0, 0, -126],
        ],
        [
            [222, 0, 0, 82],
            [126, 0, 0, 126],
            [252, 0, 0, 0],
        ],
        [
            [222, 0, 0, 82],
            [252, 0, 0, 0],
            [378, 0, 0, 126],
        ],
        [
            [222, 0, 0, 82],
            [378, 0, 0, 126],
            [126, 0, 0, 126],
        ],
        [
            [222, 0, 0, -82],
            [378, 0, 0, -126],
            [252, 0, 0, 0],
        ],
        [
            [222, 0, 0, -82],
            [252, 0, 0, 0],
            [126, 0, 0, -126],
        ],
        [
            [222, 0, 0, -82],
            [126, 0, 0, -126],
            [378, 0, 0, -126],
        ],
        [
            [390, 0, 0, 26],
            [378, 0, 0, 126],
            [252, 0, 0, 0],
        ],
        [
            [390, 0, 0, 26],
            [252, 0, 0, 0],
            [504, 0, 0, 0],
        ],
        [
            [390, 0, 0, 26],
            [504, 0, 0, 0],
            [378, 0, 0, 126],
        ],
        [
            [390, 0, 0, -26],
            [504, 0, 0, 0],
            [252, 0, 0, 0],
        ],
        [
            [390, 0, 0, -26],
            [252, 0, 0, 0],
            [378, 0, 0, -126],
        ],
        [
            [390, 0, 0, -26],
            [378, 0, 0, -126],
            [504, 0, 0, 0],
        ],
    ];

    /// Off-lattice translations that are NOT symmetries, ×126: (1,0),
    /// (0,1), (1/2,0), (1/2, √3/2), (√3,0).
    const P3M1_TRANSLATION_NEGS: [[i64; 4]; 5] = [
        [126, 0, 0, 0],
        [0, 126, 0, 0],
        [63, 0, 0, 0],
        [63, 0, 0, 63],
        [0, 0, 126, 0],
    ];

    /// Lattice membership in ×126 integer coordinates:
    /// Λ126 = ⟨(378,0,0,−126), (378,0,0,126)⟩, i.e. B = C = 0,
    /// A ≡ 0 (mod 378), D ≡ 0 (mod 126), (D/126) − (A/378) ≡ 0 (mod 2).
    fn p3m1_in_lat(v: [i64; 4]) -> bool {
        v[1] == 0
            && v[2] == 0
            && v[0].rem_euclid(378) == 0
            && v[3].rem_euclid(126) == 0
            && ((v[3] / 126) - (v[0] / 378)).rem_euclid(2) == 0
    }

    /// Coset reduction for ×126 points: i = ⌊(A+3D)/756⌉,
    /// j = ⌊(A−3D)/756⌉ (div_euclid, matching Python `//`); canonical
    /// reps have A ∈ [0,756), D ∈ [−126,126) (B, C pass through — Λ
    /// has no B, C content).
    fn p3m1_reduce(v: [i64; 4]) -> [i64; 4] {
        let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
        let i = (a + 3 * d).div_euclid(756);
        let j = (a - 3 * d).div_euclid(756);
        [a - 378 * (i + j), b, c, d - 126 * (i - j)]
    }

    /// Apply g = (L, v) to a ×126 point: L is given as the integer
    /// matrix 2L (basis images have denominator ≤ 2, so the division
    /// is exact on the vertex set and the centre reps).
    fn p3m1_apply(l2: &[[i64; 4]; 4], v: [i64; 4], p: [i64; 4]) -> [i64; 4] {
        let mut out = [0i64; 4];
        for i in 0..4 {
            let s = (0..4).map(|j| l2[i][j] * p[j]).sum::<i64>();
            assert!(s % 2 == 0, "2L·p must be even, got {s}");
            out[i] = s / 2 + v[i];
        }
        out
    }

    /// Does g = (L, v) map every cell tile onto a cell tile plus a
    /// lattice translate — the exact symmetry criterion for the
    /// 18-tile cell? All 18 tiles are triangles (3 real vertices
    /// each, no padding).
    fn p3m1_maps_cells(l2: &[[i64; 4]; 4], v: [i64; 4]) -> bool {
        for gtile in &P3M1_TILE_VERTS {
            let mut gt: Vec<[i64; 4]> = (0..3)
                .map(|k| p3m1_apply(l2, v, gtile[k]))
                .collect();
            gt.sort_unstable();
            let mut ok = false;
            'tiles: for utile in &P3M1_TILE_VERTS {
                let mut us: Vec<[i64; 4]> = (0..3).map(|k| utile[k]).collect();
                us.sort_unstable();
                'anchors: for i in 0..3 {
                    for j in 0..3 {
                        let lam = [
                            gt[i][0] - us[j][0],
                            gt[i][1] - us[j][1],
                            gt[i][2] - us[j][2],
                            gt[i][3] - us[j][3],
                        ];
                        if !p3m1_in_lat(lam) {
                            continue;
                        }
                        let mut all = true;
                        for k in 0..3 {
                            if gt[k][0] != us[k][0] + lam[0]
                                || gt[k][1] != us[k][1] + lam[1]
                                || gt[k][2] != us[k][2] + lam[2]
                                || gt[k][3] != us[k][3] + lam[3]
                            {
                                all = false;
                                break;
                            }
                        }
                        if all {
                            ok = true;
                            break 'anchors;
                        }
                    }
                }
                if ok {
                    break 'tiles;
                }
            }
            if !ok {
                return false;
            }
        }
        true
    }

    /// Does the linear part normalize the lattice? (Both basis images
    /// in Λ; finite index with |det| = 1 then gives equality.)
    fn p3m1_normalizes(l2: &[[i64; 4]; 4]) -> bool {
        p3m1_in_lat(p3m1_apply(l2, [0; 4], [378, 0, 0, -126]))
            && p3m1_in_lat(p3m1_apply(l2, [0; 4], [378, 0, 0, 126]))
    }

    /// The exact p3m1 criterion: g = (L, v) is a symmetry of the
    /// tiling iff L normalizes Λ126 and it maps every cell tile onto a
    /// cell tile plus a lattice translate.
    fn p3m1_is_sym(l2: &[[i64; 4]; 4], v: [i64; 4]) -> bool {
        p3m1_normalizes(l2) && p3m1_maps_cells(l2, v)
    }

    /// Is centre c on one of the tiling's mirror lines? The positive
    /// mirror cosets are exactly the zero cosets of the three grid-line
    /// families refl0/refl60/refl120 (the probe below), whose mirror
    /// lines all pass through the origin, so c lies on a mirror iff
    /// c − L·c ∈ Λ for one of the three linear parts.
    fn p3m1_on_mirror(c: [i64; 4]) -> bool {
        for l2 in [&P3_LREFL0, &P3_LREFL60, &P3_LREFL120] {
            let d = p3m1_apply(l2, [0; 4], c);
            if p3m1_in_lat([c[0] - d[0], c[1] - d[1], c[2] - d[2], c[3] - d[3]]) {
                return true;
            }
        }
        false
    }

    /// Exact p3m1 probe (×126 integer scheme, mirroring
    /// `m85_emit.py` from the HX-STUDY handoff): positives are exactly
    /// the translations (25/25 in a ±2 window) and the zero coset of
    /// rot120, rot240 and the three grid-line mirror families
    /// refl0/refl60/refl120 — everything else negative across the full
    /// candidate census (69 tested cosets for the five positive parts,
    /// 42 for the six negative parts: no 60°/180°/300° rotation, so
    /// not p6 or p6m; no 30°/90°/150° mirror family, no glides). Five
    /// off-lattice translations are killed. The three rot120 centre
    /// orbits — the (0,0), (2,0), (4,0) grid-vertex orbits — reduce
    /// from the closed form c(i,j) ×126 = (126(2i+j), 0, 0, 126j),
    /// and the on-mirror incidence is [T, T, T]: all three orbits on
    /// mirrors, the *333 fingerprint — M84's p31m probe pins the
    /// contrast ([T, F, F]: one orbit on, two off).
    #[test]
    fn alcove_cut_grid_wallpaper_group_is_p3m1() {
        // Positive: the 25 lattice translations of the ±2 window.
        for i in -2i64..=2 {
            for j in -2i64..=2 {
                assert!(
                    p3m1_is_sym(&P3_LIDENT, [378 * (i + j), 0, 0, 126 * (j - i)]),
                    "lattice translation ({i},{j}) must be a symmetry"
                );
            }
        }
        // Negative: the five off-lattice translations.
        for v in P3M1_TRANSLATION_NEGS {
            assert!(
                !p3m1_is_sym(&P3_LIDENT, v),
                "off-lattice {v:?} must not be a symmetry"
            );
        }

        // Full candidate census for the 11 non-identity D6 parts:
        // candidates = {q − L·p : p, q in the 13-vertex set}, reduced
        // to one canonical rep per coset. Only the zero coset is
        // positive, and only for the five p3m1 parts (rot120, rot240,
        // refl0, refl60, refl120).
        const PARTS: [(&str, &[[i64; 4]; 4], usize, bool); 11] = [
            ("rot60", &P3_LROT60, 42, false),
            ("rot120", &P3_LROT120, 69, true),
            ("rot180", &P3_LROT180, 42, false),
            ("rot240", &P3_LROT240, 69, true),
            ("rot300", &P3_LROT300, 42, false),
            ("refl0", &P3_LREFL0, 69, true),
            ("refl30", &P3_LREFL30, 42, false),
            ("refl60", &P3_LREFL60, 69, true),
            ("refl90", &P3_LREFL90, 42, false),
            ("refl120", &P3_LREFL120, 69, true),
            ("refl150", &P3_LREFL150, 42, false),
        ];
        for (name, l2, tested, has_pos) in PARTS {
            assert!(
                p3m1_normalizes(l2),
                "{name}: linear part must normalize the lattice"
            );
            let mut cands: Vec<[i64; 4]> = Vec::new();
            for p in P3M1_VERTS {
                let mp = p3m1_apply(l2, [0; 4], p);
                for q in P3M1_VERTS {
                    let v = p3m1_reduce([
                        q[0] - mp[0],
                        q[1] - mp[1],
                        q[2] - mp[2],
                        q[3] - mp[3],
                    ]);
                    if !cands.contains(&v) {
                        cands.push(v);
                    }
                }
            }
            assert_eq!(cands.len(), tested, "{name}: tested coset count");
            let mut pos = 0;
            for v in &cands {
                let sym = p3m1_is_sym(l2, *v);
                if sym {
                    pos += 1;
                }
                assert_eq!(
                    sym,
                    has_pos && *v == [0, 0, 0, 0],
                    "{name}: candidate {v:?}"
                );
            }
            assert_eq!(pos, usize::from(has_pos), "{name}: positive cosets");
        }

        // The three rot120 centre orbits mod the lattice: the (0,0),
        // (2,0), (4,0) grid-vertex orbits (closed form c(i,j) ×126 =
        // (126(2i+j), 0, 0, 126j)).
        let mut orb: Vec<[i64; 4]> = Vec::new();
        for i in -3i64..=3 {
            for j in -3i64..=3 {
                let v = p3m1_reduce([126 * (2 * i + j), 0, 0, 126 * j]);
                if !orb.contains(&v) {
                    orb.push(v);
                }
            }
        }
        orb.sort_unstable();
        let want_orb: [[i64; 4]; 3] = [[0, 0, 0, 0], [252, 0, 0, 0], [504, 0, 0, 0]];
        assert_eq!(orb, want_orb.to_vec(), "rot120 centre orbits mod lattice");
        // Each orbit rep c satisfies c − R120·c ∈ Λ (R120 about c is
        // a symmetry).
        for c in want_orb {
            let d = p3m1_apply(&P3_LROT120, [0; 4], c);
            assert!(
                p3m1_in_lat([c[0] - d[0], c[1] - d[1], c[2] - d[2], c[3] - d[3]]),
                "orbit rep {c:?}: c − R120·c in Λ"
            );
        }
        // Incidence — the p3m1/p31m discriminator: all THREE centre
        // orbits lie on mirrors, the *333 fingerprint. In p31m (M84's
        // split-triangle-grid probe) exactly one of the three orbits
        // is on a mirror: [T, T, T] here vs [T, F, F] there.
        for c in want_orb {
            assert!(
                p3m1_on_mirror(c),
                "orbit rep {c:?}: all three centre orbits on mirrors (p3m1)"
            );
        }

        // Cell arithmetic ×126²: doubled area (re, im) per tile — the
        // three congruence classes (membership
        // [0,1,2,1,0,2,1,0,2,0,1,2,0,1,2,1,0,2]): class 0 (6 tiles)
        // (0, 14112), class 1 (6 tiles) (0, 6552), class 2 (6 tiles)
        // (0, 11088); sum (0, 190512) = 2·det·126² (det = 6√3).
        // Squared-edge multiset: three congruence classes — class 0
        // {(21072,0), (30144,0), (63504,0)}, class 1
        // {(15024,0), (21072,0), (63504,0)}, class 2
        // {(15024,0), (30144,0), (63504,0)} (achiral as sets: each
        // class is mirror-closed under y = 0).
        let mut areas: Vec<(i64, i64)> = Vec::new();
        let mut edges: Vec<Vec<(i64, i64)>> = Vec::new();
        for t in P3M1_TILE_VERTS.iter() {
            let mut area = (0i64, 0i64);
            let mut es: Vec<(i64, i64)> = Vec::new();
            for i in 0..3 {
                let (u, w) = (t[i], t[(i + 1) % 3]);
                area.0 += u[0] * w[1] + 3 * u[2] * w[3] - w[0] * u[1] - 3 * w[2] * u[3];
                area.1 += u[0] * w[3] + u[2] * w[1] - w[0] * u[3] - w[2] * u[1];
                let (da, db, dc, dd) = (
                    w[0] - u[0],
                    w[1] - u[1],
                    w[2] - u[2],
                    w[3] - u[3],
                );
                es.push((
                    da * da + db * db + 3 * dc * dc + 3 * dd * dd,
                    2 * da * dc + 2 * db * dd,
                ));
            }
            es.sort_unstable();
            edges.push(es.clone());
            areas.push(area);
        }
        let want_areas: [(i64, i64); 18] = [
            (0, 14112), (0, 6552), (0, 11088), (0, 6552), (0, 14112), (0, 11088),
            (0, 6552), (0, 14112), (0, 11088), (0, 14112), (0, 6552), (0, 11088),
            (0, 14112), (0, 6552), (0, 11088), (0, 6552), (0, 14112), (0, 11088),
        ];
        assert_eq!(areas, want_areas.to_vec(), "doubled tile areas ×126²");
        assert_eq!(
            areas.iter().map(|a| a.0).sum::<i64>(),
            0,
            "real part of doubled cell area"
        );
        assert_eq!(
            areas.iter().map(|a| a.1).sum::<i64>(),
            190_512,
            "sum = 2·det·126² (det 6√3)"
        );
        const MEMBERSHIP: [usize; 18] = [0, 1, 2, 1, 0, 2, 1, 0, 2, 0, 1, 2, 0, 1, 2, 1, 0, 2];
        let want_edges: [Vec<(i64, i64)>; 3] = [
            vec![(21072, 0), (30144, 0), (63504, 0)],
            vec![(15024, 0), (21072, 0), (63504, 0)],
            vec![(15024, 0), (30144, 0), (63504, 0)],
        ];
        for (e, cls) in edges.iter().zip(MEMBERSHIP.iter()) {
            assert_eq!(
                *e, want_edges[*cls],
                "squared-edge multiset (three congruence classes, achiral as sets)"
            );
        }
    }

    /// M112: `SeedPattern::Rows` — the tile at lattice index (i, j)
    /// sits at world y = j on the centered-rectangular lattice
    /// <(2,0),(1,1)>, so the row-index parity is exactly the course
    /// parity: every generated tile's seed must equal the parity of
    /// its row, and a bounded view must show both parities.
    #[test]
    fn running_bond_seed_rows_are_course_parity() {
        let t = running_bond::running_bond();
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            // Default params (scale 1, rotation 0): a domino of course
            // j occupies y = [j, j+1], so its aabb min.y is j exactly.
            let j = tile.poly.aabb().min.y.round() as i64;
            let want = (((j % 2) + 2) % 2) as u32;
            assert_eq!(tile.seed, want, "seed at course {j} must equal the course parity");
            seen[tile.seed as usize] = true;
        }
        assert!(seen[0] && seen[1], "both courses must appear");
    }

    /// M112: single hexomino per cell, single orbit — the shipped seed
    /// is the decorative `SeedPattern::Checker` lattice parity, so a
    /// bounded view must show both parities (seeds stay in {0, 1}).
    #[test]
    fn stepped_hexominoes_seed_is_decorative_checkerboard() {
        let t = stepped_hexominoes::stepped_hexominoes();
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            assert!(tile.seed < 2, "checkerboard seed must stay in {{0, 1}}");
            seen[tile.seed as usize] = true;
        }
        assert!(seen[0] && seen[1], "both lattice-parity classes must appear");
    }

    /// M112: the three tiles of the cell are R120 images of one chiral
    /// class (a single orbit) — the shipped seed is the decorative
    /// `SeedPattern::Checker` lattice parity, so a bounded view must
    /// show both parities (seeds stay in {0, 1}).
    #[test]
    fn chiral_cut_honeycomb_seed_is_decorative_checkerboard() {
        let t = chiral_cut_honeycomb::chiral_cut_honeycomb();
        let p = ParamSet::default_for(t.params());
        let r = t.generate(&p, &Query { region: BBox::new(v2(-6.0, -6.0), v2(6.0, 6.0)) });
        assert!(r.message.is_none(), "{:?}", r.message);
        let mut seen = [false; 2];
        for tile in &r.tiles {
            assert!(tile.seed < 2, "checkerboard seed must stay in {{0, 1}}");
            seen[tile.seed as usize] = true;
        }
        assert!(seen[0] && seen[1], "both lattice-parity classes must appear");
    }
}

