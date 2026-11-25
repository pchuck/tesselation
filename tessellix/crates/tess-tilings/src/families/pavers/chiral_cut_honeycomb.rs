//! Chiral-cut honeycomb (M83) — the first **p3** wallpaper-group
//! realizer in the catalog, expressed as a [`LatticeTiling`] data table.
//!
//! At a glance this reads as the rhombille tiling (hexagons cut into
//! three by a 3-arm star); it is deliberately a *perturbed* rhombille:
//! straight centre cuts would realize p6m (the existing `rhombille`
//! entry), so each cut is bent by ±2.83° at a single bend point. The
//! bend breaks every reflection while preserving the 120° rotations —
//! that is the entire difference between this tiling and rhombille,
//! and the source of the slight unevenness of the cut lines.
//!
//! # The pattern
//!
//! Start with a regular hexagon of circumradius 2 (side 2, area `6√3`)
//! centred at `O = (0,0)`, with vertices `V_k` at angles
//! `30° + 60°·k`. A chiral three-armed "Y" is cut from the alternating
//! vertices `V0, V2, V4` toward the centre. Each arm bends once: arm 0
//! bends at `W0 = mid(V0, O) + δ` with `δ = (−1/8, −1/10)`, and the
//! other bends are its 120° rotations, `W2 = R120(W0)`,
//! `W4 = R120²(W0)` — the offset `δ` is defined once and rotates WITH
//! each arm. The sign of `δ` matters: the opposite sign flips the bend
//! the other way and makes the pieces non-star-shaped.
//!
//! The cut splits the hexagon into three 6-gon tiles, each anchored at
//! `O` and walking one arm out, two hex edges, and the next arm back:
//!
//! * tile 0: `O, W0, V0, V1, V2, W2`
//! * tile 1: `O, W2, V2, V3, V4, W4`
//! * tile 2: `O, W4, V4, V5, V0, W0`
//!
//! Each tile has area `2√3`, so the three fill the hexagon exactly
//! (`3 · 2√3 = 6√3 = det Λ`). The rotation R120 about `O` maps
//! tile 0 → tile 1 → tile 2 exactly (no translate), and all three
//! belong to one congruence class — a CHIRAL one: the mirror image is
//! not directly congruent to the class, so the tiling distinguishes
//! left from right.
//!
//! # Edge-to-edge note
//!
//! The partition is edge-to-edge: every tile edge is either a hexagon
//! side, a cut arm segment (outer or inner), and every edge is shared
//! by exactly two tiles. The battery window check (interior edges all
//! paired exactly twice, no T-junctions) confirms 450 interior edges,
//! 0 singles, 0 bad.
//!
//! # Vertex stars
//!
//! Four vertex types, all summing to 360°:
//!
//! * the hex centre `O` and the odd hex corners `V1, V3, V5`: three
//!   tiles meet with angles `120° × 3`;
//! * the even hex corners `V0, V2, V4` (where the cut arms leave the
//!   hexagon): SIX tiles meet — each incident hexagon contributes two
//!   tiles flanking its arm — with angles `≈58.8079° × 3` +
//!   `≈61.1921° × 3`;
//! * the bend vertices `W_k`: two tiles meet with angles
//!   `≈177.1677°` and `≈182.8323°` (the bend-edge directions' dot
//!   product is `−1559/1600` exactly; the cosine itself is that
//!   divided by `|W0V0|·|W0O|`, ≈ −0.9988).
//!
//! # Wallpaper group
//!
//! **p3** — 120° and 240° rotations plus translations, and nothing
//! else. The chiral cut breaks the hexagon's 6-fold symmetry at the
//! centres down to 3-fold and kills every reflection. The 3-fold
//! centres fall into three orbits under the lattice: the hex centres
//! `O`, the even corners, and the odd corners — all three genuinely
//! 3-fold (the "333" fingerprint of p3, as opposed to p6's single
//! 6-fold orbit). The mirror image of the tiling is a DIFFERENT
//! tiling (same group, opposite handedness) — p3 is chiral.
//!
//! # Bravais kind
//!
//! **Hexagonal** (primitive): `Λ = ⟨(2√3, 0), (√3, 3)⟩`,
//! `det Λ = 6√3` — the hexagon's own lattice, so the cell is one
//! hexagon (three tiles).
//!
//! # Seeds and protos
//!
//! One chiral congruence class per cell: the three tiles are R120
//! images of each other about O — a single orbit, with no
//! orbit-distinct coloring to ship.  The shipped seed is therefore
//! *decorative*: `SeedPattern::Checker` colors lattice sites by the
//! parity of `(i + j)` on the hexagonal basis — a lattice-parity
//! coloring, not an orbit class.  (Without it, Pattern coloring would
//! be monochrome: the pieces are all "the same tile" up to rotation,
//! and that is exactly what p3 is about.)
//!
//! # Verification
//!
//! Coords are machine-emitted (`m83_emit.py` → `m83_emit_rust.inc`,
//! parsed back to the HX-STUDY cell JSON exactly, no hand-typed
//! coordinates). The full HX-STUDY battery was reproduced before
//! porting (`m83_verify.py`): exact-Fraction battery matches the JSON
//! on every number (area sum `6√3 = det`, e2e 450/0/0, stars 170/0,
//! coverage 4096/0/0, positives only at rot120/rot240 zero coset,
//! translations 25/25, six off-lattice negatives 0/6, three rot120
//! centre orbits), AND the exact-integer ×320 probe that the Rust test
//! below mirrors (lattice `a1×320 = (0,0,640,0)`,
//! `a2×320 = (0,960,320,0)`; 11 integer `2L` matrices; coupled-floor
//! coset reduction). Rust: the standard trio, the exact-integer p3
//! group probe (`chiral_cut_honeycomb_wallpaper_group_is_p3`), and the
//! brute-force culling cross-check
//! (`lattice_culling_matches_brute_force`).

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// √3/2 — the y-coordinate of a side-1 unit hexagon's top vertex.
const H: f64 = 0.8660254037844386; // √3/2

fn poly(pts: &[Vec2]) -> Polygon {
    Polygon::from_slice(pts)
}

/// Tile 0 of the cell: `[O, W0, V0, V1, V2, W2]`, CCW, anchored at the
/// hex centre `O = (0,0)`.
fn piece0() -> Polygon {
    poly(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(-1.0 / 8.0 + H, 2.0 / 5.0),
        Vec2::new(2.0 * H, 1.0),
        Vec2::new(0.0, 2.0),
        Vec2::new(-2.0 * H, 1.0),
        Vec2::new(1.0 / 16.0 - 9.0 * H / 10.0, 11.0 / 20.0 - 1.0 * H / 8.0),
    ])
}

/// Tile 1 of the cell: `[O, W2, V2, V3, V4, W4]` — the image of
/// [`piece0`] under R120 about `O`.
fn piece1() -> Polygon {
    poly(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0 / 16.0 - 9.0 * H / 10.0, 11.0 / 20.0 - 1.0 * H / 8.0),
        Vec2::new(-2.0 * H, 1.0),
        Vec2::new(-2.0 * H, -1.0),
        Vec2::new(0.0, -2.0),
        Vec2::new(1.0 / 16.0 - 1.0 * H / 10.0, -19.0 / 20.0 + 1.0 * H / 8.0),
    ])
}

/// Tile 2 of the cell: `[O, W4, V4, V5, V0, W0]` — the image of
/// [`piece0`] under R240 about `O`.
fn piece2() -> Polygon {
    poly(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0 / 16.0 - 1.0 * H / 10.0, -19.0 / 20.0 + 1.0 * H / 8.0),
        Vec2::new(0.0, -2.0),
        Vec2::new(2.0 * H, -1.0),
        Vec2::new(2.0 * H, 1.0),
        Vec2::new(-1.0 / 8.0 + H, 2.0 / 5.0),
    ])
}

/// Chiral-cut honeycomb — a side-2 hexagon (area `6√3`) cut by a
/// chiral three-armed Y from alternating vertices to the centre, per
/// primitive hexagonal cell `⟨(2√3,0),(√3,3)⟩` (det `6√3`); three
/// tiles, one chiral congruence class, group p3.
pub fn chiral_cut_honeycomb() -> LatticeTiling {
    // One chiral congruence class per cell: all three tiles are
    // R120 images of each other about O, so they share seed 0.
    let cells = vec![
        Cell {
            poly: piece0(),
            seed: 0,
        },
        Cell {
            poly: piece1(),
            seed: 0,
        },
        Cell {
            poly: piece2(),
            seed: 0,
        },
    ];

    LatticeTiling {
        id: "chiral-cut-honeycomb",
        name: "Chiral-cut honeycomb",
        description: "Side-2 hexagon (area 6√3 = det Λ) cut by a chiral three-armed Y from alternating vertices V0,V2,V4 to the centre, each arm bending once at Wk = mid(Vk,O) + rotated delta (delta = (-1/8,-1/10), defined once, rotating with the arm); three tiles per hexagonal cell <(2√3,0),(√3,3)>, one chiral congruence class, edge-to-edge. The chiral cut breaks the 6-fold to 3-fold and kills all reflections: 120°/240° rotations about three centre orbits (hex centres, even corners, odd corners — all 3-fold) plus translations only, the catalog's first p3 realizer (M83, HX-STUDY handoff).",
        lattice: [Vec2::new(4.0 * H, 0.0), Vec2::new(2.0 * H, 3.0)],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P3,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Wallpaper,
        secondary: &[],
        seed_pattern: SeedPattern::Checker,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
