//! Color palettes shared by both UIs.

use crate::tiling::Tile;
use crate::view::ColorMode;

/// 8-color palette. Index 0 is the "single" color.
pub const PALETTE: [u32; 8] = [
    0xD4D4D4, // 0: light gray (single mode)
    0x5B8DEF, // 1: blue
    0xE06C75, // 2: red
    0x98C379, // 3: green
    0xE5C07B, // 4: yellow
    0xC678DD, // 5: purple
    0x56B6C2, // 6: cyan
    0xD19A66, // 7: orange
];

pub const BORDER: u32 = 0x1E1E2E;
pub const BACKGROUND: u32 = 0x282A3A;

/// Monochrome ramp for [`ColorMode::Gray`] (see `crate::view`): 8 even
/// lightness steps (24 apart), indexed by prototile class exactly like
/// the color palette so pattern/proto structure survives the palette
/// swap. Index 0 matches `Single`'s light gray — the dominant class
/// reads light on the dark background in every mode. The darkest step
/// stays above the border color so every tile keeps its outline
/// contrast.
pub const GRAY_PALETTE: [u32; 8] = [
    0xD4D4D4, // 0: light gray (matches Single)
    0xBCBCBC, // 1
    0xA4A4A4, // 2
    0x8C8C8C, // 3
    0x747474, // 4
    0x5C5C5C, // 5
    0x444444, // 6
    0x2C2C2C, // 7
];

/// Grayscale ramp for [`ColorMode::PatternGray`] (see `crate::view`):
/// the Rec.709 luma of each [`PALETTE`] color —
/// `L = 0.2126·R + 0.7152·G + 0.0722·B` on 8-bit sRGB values (not
/// linearized, not Rec.601), rounded half away from zero — indexed by
/// seed exactly like the color palette so pattern structure survives the
/// palette swap. Index 0 matches `Single`'s light gray (and `Gray`'s
/// index 0). Cyan/orange collide at 162 (rounded); cyan is nudged to 163
/// (max |deviation| 0.543 over the table) in preference to orange → 161
/// (0.939), and both still preserve luma order. The table was pinned by a
/// python certificate (see .scratch/pattern-gray/luma_certificate.py).
pub const PATTERN_GRAY_PALETTE: [u32; 8] = [
    0xD4D4D4, // 0: light gray (matches Single / GRAY_PALETTE[0])
    0x898989, // 1: blue  (luma 137.446)
    0x858585, // 2: red   (luma 133.311)
    0xB5B5B5, // 3: green (luma 180.515)
    0xC3C3C3, // 4: yellow (luma 194.884)
    0x909090, // 5: purple (luma 143.875)
    0xA3A3A3, // 6: cyan  (luma 162.457, nudged +1 to keep entries distinct)
    0xA2A2A2, // 7: orange (luma 161.939)
];

/// Warm/earth ramp for [`ColorMode::Role`] (see `crate::view`) — the
/// islamic-family shape-role distinction (star vs. petal vs. background)
/// drawn from a dedicated earth-tone palette instead of the rainbow
/// `PALETTE`, indexed by the same `seed % 8` class. Distinct tones, dark-bg
/// friendly; the table is byte-pinned in the tests below.
pub const ROLE_PALETTE: [u32; 8] = [
    0xE8C170, // 0: sand
    0xD97B56, // 1: clay
    0xC16045, // 2: rust
    0x8C3F38, // 3: deep brick
    0xE0B24F, // 4: ochre
    0xB86730, // 5: copper
    0xD98C73, // 6: terracotta rose
    0x7A4E2D, // 7: umber
];

/// Categorical ramp for [`ColorMode::Metatile`] (see `crate::view`):
/// nine distinct tones covering the hat cluster role (4 classes: H/T/P/F),
/// the cap metatile type (4 classes: 0..=3) and the proto-class fallback
/// (0..=8). Distinct from the rainbow `PALETTE` and the earth `ROLE_PALETTE`;
/// the table is byte-pinned in the tests below.
pub const METATILE_PALETTE: [u32; 9] = [
    0x7ED1A8, // 0: mint
    0xE6647F, // 1: rose
    0xE8A33D, // 2: amber
    0x95CE5F, // 3: lime
    0x4FB4C6, // 4: teal
    0xB46FD9, // 5: violet
    0xE66F5F, // 6: coral
    0xF2D85F, // 7: yellow
    0x7FA8F0, // 8: sky blue
];

/// Polygon shape class: triangle → 0, quadrilateral → 1, … decagon-and-beyond → 7
/// (`min(n - 3, 7)` on the vertex count, degenerate (< 3-vertex) polygons clamp to 0).
pub fn shape_class(poly: &crate::geometry::Polygon) -> usize {
    poly.len().saturating_sub(3).min(7)
}

/// Aperiodic metatile/cluster class (see `crate::view`).
///
/// The `Tile` contract carries only `proto` / `seed` / `poly`, so this
/// keys on the tile signature. Signatures (verified at `hat.rs:903-908`,
/// `spectre.rs:469-474`, `turtle.rs:409-415`, `cap.rs:162`/`1087-1090`):
///
/// * **Hat/spectre/turtle-turtle** (monohedral): `proto == 0`, `seed`
///   in 1..=9 — color by the hat cluster-role bucket (H1-H4 = 1..=4 → 0,
///   T1 = 5 → 1, P1-P2 = 6..=7 → 2, F1-F2 = 8..=9 → 3). Hat and spectre
///   emit the *identical* signature (`proto=0, seed 1..=9`) and the
///   turtle engine uses the same for its turtle-shape tiles, so they
///   cannot be told apart at paint time — one seed→class mapping applies
///   to all three (documented limitation). The same signature is also
///   emitted by reptiles (armchair/chair/sphinx: `seed` = orientation+1,
///   1..=12), pinwheel (`seed` = child-slot+1, 1..=5) and isohedral
///   (`seed` = aspect index, 0..na) tiles, which colors by the same bucket
///   — the mode is not family-scoped and the Tile contract carries no
///   family identity (documented limitation).
/// * **Turtle hat-companion** (`proto == 1`, `seed` in 1..=9 = matching-rule
///   label+1) is *not* disambiguated from **cap** (`proto` = metatile type,
///   `seed` = `SEED_OF[ty]` = `[0, 3, 2, 1][ty]`): the pair
///   `(proto=1, seed=3)` is exactly cap's T *and* turtle's hat-companion
///   with label 2, so both fall through to the proto-class fallback.
/// * **Cap**: `proto` is the metatile type (0..=3); the proto-class
///   fallback below reads it directly. Cap is deliberately NOT matched by
///   the exact `(proto, seed)` pair — `(proto=0, seed=0)` would
///   misclassify every lattice cell-0/seed-0 tile as cap-H.
/// * Everything else: proto-class fallback.
pub fn metatile_class(tile: &Tile) -> usize {
    if tile.proto == 0 && (1..=9).contains(&tile.seed) {
        return match tile.seed {
            1..=4 => 0,
            5 => 1,
            6..=7 => 2,
            _ => 3,
        };
    }
    (tile.proto as usize) % METATILE_PALETTE.len()
}

pub fn tile_rgb(mode: ColorMode, tile: &Tile) -> u32 {
    let idx = match mode {
        ColorMode::Single => 0,
        ColorMode::Pattern => (tile.seed as usize) % PALETTE.len(),
        ColorMode::PatternGray => (tile.seed as usize) % PALETTE.len(),
        ColorMode::Shape => shape_class(&tile.poly),
        ColorMode::Proto => (tile.proto as usize) % PALETTE.len(),
        ColorMode::Gray => (tile.proto as usize) % GRAY_PALETTE.len(),
        ColorMode::Role => (tile.seed as usize) % ROLE_PALETTE.len(),
        ColorMode::Metatile => metatile_class(tile),
    };
    match mode {
        ColorMode::Gray => GRAY_PALETTE[idx],
        ColorMode::PatternGray => PATTERN_GRAY_PALETTE[idx],
        ColorMode::Role => ROLE_PALETTE[idx],
        ColorMode::Metatile => METATILE_PALETTE[idx],
        _ => PALETTE[idx],
    }
}

pub fn rgb(color: u32) -> (u8, u8, u8) {
    (
        ((color >> 16) & 0xFF) as u8,
        ((color >> 8) & 0xFF) as u8,
        (color & 0xFF) as u8,
    )
}

/// Nearest xterm-256 color index (for the TUI).
pub fn rgb_to_256(color: u32) -> u8 {
    let (tr, tg, tb) = rgb(color);
    let mut best: u8 = 0;
    let mut best_d = f64::MAX;
    for i in 0u16..256 {
        let (r, g, b) = xterm_256(i as u8);
        let d = (tr as f64 - r as f64).powi(2)
            + (tg as f64 - g as f64).powi(2)
            + (tb as f64 - b as f64).powi(2);
        if d < best_d {
            best_d = d;
            best = i as u8;
        }
    }
    best
}

fn xterm_256(i: u8) -> (u8, u8, u8) {
    if i < 16 {
        // System colors (approximate).
        const C: [(u8, u8, u8); 16] = [
            (0, 0, 0),
            (160, 0, 0),
            (0, 160, 0),
            (160, 160, 0),
            (0, 0, 160),
            (160, 0, 160),
            (0, 160, 160),
            (192, 192, 192),
            (96, 96, 96),
            (255, 0, 0),
            (0, 255, 0),
            (255, 255, 0),
            (0, 0, 255),
            (255, 0, 255),
            (0, 255, 255),
            (255, 255, 255),
        ];
        C[i as usize]
    } else if i < 232 {
        let c = i - 16;
        let v = |n: u8| -> u8 {
            if n == 0 {
                0
            } else {
                55 + 40 * n
            }
        };
        (v(c / 36), v((c / 6) % 6), v(c % 6))
    } else {
        let g = 8 + 10 * (i - 232);
        (g, g, g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Polygon, Vec2};

    fn tile(proto: u8, seed: u32) -> Tile {
        Tile {
            proto,
            seed,
            poly: Polygon::from_slice(&[
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(0.0, 1.0),
            ]),
        }
    }

    #[test]
    fn nearest_colors_roundtrip() {
        assert_eq!(rgb_to_256(0x000000), 0);
        // 255 white is shared by indices 15 and 231; either is acceptable,
        // but the chosen index must map back to white.
        let w = rgb_to_256(0xFFFFFF);
        assert!(w == 15 || w == 231);
        assert_eq!(xterm_256(w), (255, 255, 255));
        let i = rgb_to_256(0x98C379);
        let (r, g, b) = xterm_256(i);
        assert!((r as i32 - 152).abs() < 40);
        assert!((g as i32 - 195).abs() < 40);
        assert!((b as i32 - 121).abs() < 40);
    }

    #[test]
    fn gray_palette_is_monochrome_ramp() {
        let lum = |c: u32| (c >> 16) & 0xFF;
        let mut prev = 0xFF;
        for &g in GRAY_PALETTE.iter() {
            let (r, gr, b) = rgb(g);
            assert_eq!(r, gr, "gray ramp entry {g:#x} not monochrome");
            assert_eq!(gr, b, "gray ramp entry {g:#x} not monochrome");
            assert!(lum(g) < prev, "gray ramp not strictly decreasing");
            prev = lum(g);
        }
        // Index 0 matches Single's light gray; the darkest step stays
        // above the border color so outlines keep their contrast.
        assert_eq!(GRAY_PALETTE[0], PALETTE[0]);
        assert!(lum(GRAY_PALETTE[7]) > lum(BORDER));
    }

    #[test]
    fn pattern_gray_palette_is_pinned() {
        // The pinned table (certificate: Rec.709 luma of each PALETTE
        // color, rounded half away from zero, cyan nudged +1 to keep
        // entries distinct) — kept byte-for-byte with the certificate.
        const EXPECTED: [u32; 8] = [
            0xD4D4D4, 0x898989, 0x858585, 0xB5B5B5, 0xC3C3C3, 0x909090, 0xA3A3A3, 0xA2A2A2,
        ];
        assert_eq!(PATTERN_GRAY_PALETTE, EXPECTED);
        // Monochrome throughout (r == g == b).
        for &g in PATTERN_GRAY_PALETTE.iter() {
            let (r, gr, b) = rgb(g);
            assert_eq!(r, gr, "pattern-gray entry {g:#x} not monochrome");
            assert_eq!(gr, b, "pattern-gray entry {g:#x} not monochrome");
        }
    }

    #[test]
    fn pattern_gray_palette_all_distinct() {
        for i in 0..PATTERN_GRAY_PALETTE.len() {
            for j in (i + 1)..PATTERN_GRAY_PALETTE.len() {
                assert_ne!(
                    PATTERN_GRAY_PALETTE[i], PATTERN_GRAY_PALETTE[j],
                    "entries {i} and {j} collide"
                );
            }
        }
    }

    #[test]
    fn pattern_gray_palette_within_luma_tolerance() {
        // Rec.709 luma computed in-test from the PALETTE values as f64;
        // each entry must be within 0.55 of it (0.543 over the pinned
        // table, by construction).
        for (i, &c) in PALETTE.iter().enumerate() {
            let (r, g, b) = rgb(c);
            let luma = 0.2126 * f64::from(r) + 0.7152 * f64::from(g) + 0.0722 * f64::from(b);
            let (gr, gg, gb) = rgb(PATTERN_GRAY_PALETTE[i]);
            let actual = f64::from(gr);
            assert!(
                (actual - luma).abs() < 0.55,
                "index {i}: |{actual} - {luma}| too large"
            );
            assert_eq!(gr, gg);
            assert_eq!(gg, gb);
        }
    }

    #[test]
    fn pattern_gray_palette_zero_matches_single_and_gray() {
        assert_eq!(PATTERN_GRAY_PALETTE[0], PALETTE[0]);
        assert_eq!(PATTERN_GRAY_PALETTE[0], GRAY_PALETTE[0]);
    }

    #[test]
    fn pattern_gray_follows_pattern_seed_index() {
        // PatternGray selects by seed exactly like Pattern (seed % 8);
        // only the gray table is substituted.
        for seed in 0u32..64 {
            let expected = PATTERN_GRAY_PALETTE[(seed as usize) % 8];
            for proto in 0u8..8 {
                assert_eq!(
                    tile_rgb(ColorMode::PatternGray, &tile(proto, seed)),
                    expected,
                    "proto must not leak into the pattern-gray selection (seed {seed}, proto {proto})"
                );
            }
        }
        // The proto field must never leak into the selection:
        assert_eq!(
            tile_rgb(ColorMode::PatternGray, &tile(0, 5)),
            tile_rgb(ColorMode::PatternGray, &tile(7, 5))
        );
        // Distinct seeds give distinct colors (the point of the table):
        let colors: std::collections::BTreeSet<u32> = (0..8u32)
            .map(|seed| tile_rgb(ColorMode::PatternGray, &tile(0, seed)))
            .collect();
        assert_eq!(colors.len(), 8, "pattern-gray collapsed distinct seeds");
    }

    #[test]
    fn tile_rgb_pattern_gray_matches_pattern_index() {
        // Selection equivalence proven against the Pattern arm directly:
        // the gray table is substituted at the pattern's exact index.
        for seed in 0u32..64 {
            let pattern_idx = (seed as usize) % PALETTE.len();
            let t = tile(3, seed);
            assert_eq!(
                tile_rgb(ColorMode::PatternGray, &t),
                PATTERN_GRAY_PALETTE[pattern_idx]
            );
        }
    }

    /// Build a polygon with exactly `n` vertices (a regular-ish fan around
    /// the origin). Degenerate (< 3-vertex) polygons are unbuildable here —
    /// see `shape_class_degenerate_clamps_to_zero`.
    fn poly_with_n_vertices(n: usize) -> Polygon {
        let mut pts = Vec::with_capacity(n);
        for i in 0..n {
            let a = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
            pts.push(Vec2::new(a.cos(), a.sin()));
        }
        Polygon::from_slice(&pts)
    }

    fn tile_with_shape(proto: u8, seed: u32, n: usize) -> Tile {
        Tile { proto, seed, poly: poly_with_n_vertices(n) }
    }

    #[test]
    fn role_palette_is_pinned() {
        // The pinned warm/earth ramp — distinct from PALETTE's rainbow.
        const EXPECTED: [u32; 8] = [
            0xE8C170, 0xD97B56, 0xC16045, 0x8C3F38, 0xE0B24F, 0xB86730, 0xD98C73, 0x7A4E2D,
        ];
        assert_eq!(ROLE_PALETTE, EXPECTED);
        // All distinct, and none of them equal a PALETTE tone.
        for i in 0..ROLE_PALETTE.len() {
            for j in (i + 1)..ROLE_PALETTE.len() {
                assert_ne!(ROLE_PALETTE[i], ROLE_PALETTE[j], "entries {i} and {j} collide");
            }
            assert!(!PALETTE.contains(&ROLE_PALETTE[i]));
        }
    }

    #[test]
    fn metatile_palette_is_pinned_and_distinct() {
        // The pinned categorical ramp — 9 entries, distinct from PALETTE
        // and from ROLE_PALETTE.
        const EXPECTED: [u32; 9] = [
            0x7ED1A8, 0xE6647F, 0xE8A33D, 0x95CE5F, 0x4FB4C6, 0xB46FD9, 0xE66F5F, 0xF2D85F, 0x7FA8F0,
        ];
        assert_eq!(METATILE_PALETTE, EXPECTED);
        for i in 0..METATILE_PALETTE.len() {
            for j in (i + 1)..METATILE_PALETTE.len() {
                assert_ne!(METATILE_PALETTE[i], METATILE_PALETTE[j], "entries {i} and {j} collide");
            }
            assert!(!PALETTE.contains(&METATILE_PALETTE[i]));
            assert!(!ROLE_PALETTE.contains(&METATILE_PALETTE[i]));
        }
    }

    #[test]
    fn shape_class_maps_vertex_count() {
        // min(n - 3, 7): triangle→0 … heptagon→4, octagon→5, nonagon→6, 10+→7.
        assert_eq!(shape_class(&poly_with_n_vertices(3)), 0);
        assert_eq!(shape_class(&poly_with_n_vertices(4)), 1);
        assert_eq!(shape_class(&poly_with_n_vertices(5)), 2);
        assert_eq!(shape_class(&poly_with_n_vertices(6)), 3);
        assert_eq!(shape_class(&poly_with_n_vertices(7)), 4);
        assert_eq!(shape_class(&poly_with_n_vertices(8)), 5);
        assert_eq!(shape_class(&poly_with_n_vertices(9)), 6);
        assert_eq!(shape_class(&poly_with_n_vertices(10)), 7);
        assert_eq!(shape_class(&poly_with_n_vertices(16)), 7);
    }

    #[test]
    fn shape_class_degenerate_clamps_to_zero() {
        let empty = Polygon::from_slice(&[]);
        assert_eq!(shape_class(&empty), 0);
        let one_vertex = Polygon::from_slice(&[Vec2::new(0.0, 0.0)]);
        assert_eq!(shape_class(&one_vertex), 0);
    }

    #[test]
    fn role_mode_matches_pattern_seed_index_on_earth_ramp() {
        for seed in 0u32..64 {
            let expected = ROLE_PALETTE[(seed as usize) % 8];
            let t = tile(3, seed); // quad poly
            assert_eq!(tile_rgb(ColorMode::Role, &t), expected);
        }
        // Same selection as Pattern but the ramp is different.
        assert_eq!(
            tile_rgb(ColorMode::Role, &tile(0, 5)),
            ROLE_PALETTE[5 % 8]
        );
        assert_ne!(
            tile_rgb(ColorMode::Role, &tile(0, 5)),
            tile_rgb(ColorMode::Pattern, &tile(0, 5))
        );
    }

    #[test]
    fn metatile_class_hat_cluster_role_buckets() {
        // hat.rs:903-908 — proto=0, seed 1..=9 (H1-H4=1..=4, T1=5, P1-P2=6..=7, F1-F2=8..=9).
        for (seed, expected) in [
            (1u32, 0usize), (2, 0), (3, 0), (4, 0), // H
            (5, 1),                                 // T
            (6, 2), (7, 2),                          // P
            (8, 3), (9, 3),                          // F
        ] {
            let t = Tile { proto: 0, seed, poly: poly_with_n_vertices(3) };
            assert_eq!(metatile_class(&t), expected, "seed {seed}");
        }
    }

    #[test]
    fn metatile_class_cap_metatile_type() {
        // cap.rs:162/1087-1090 — proto = type (0..=3), seed = SEED_OF[ty] = [0,3,2,1].
        for (proto, seed, expected) in [
            (0u8, 0u32, 0usize), // H
            (1, 3, 1),           // T
            (2, 2, 2),           // P
            (3, 1, 3),           // F
        ] {
            let t = Tile { proto, seed, poly: poly_with_n_vertices(3) };
            assert_eq!(metatile_class(&t), expected, "proto {proto}");
        }
    }

    #[test]
    fn metatile_class_falls_back_to_proto() {
        // Non-aperiodic shapes (seed 0, or proto > 3): the proto-class fires.
        for proto in 0u8..9 {
            let t = Tile { proto, seed: 0, poly: poly_with_n_vertices(3) };
            assert_eq!(metatile_class(&t), (proto as usize) % 9);
        }
        // Cap-like signature with a seed NOT matching SEED_OF must not be
        // treated as cap — it follows the hat/spectre seed branch instead.
        let t = Tile { proto: 0, seed: 5, poly: poly_with_n_vertices(3) };
        assert_eq!(metatile_class(&t), 1); // seed-5 → hat T bucket
    }

    #[test]
    fn tile_rgb_shape_follows_polygon_class() {
        // Triangle → class 0 → PALETTE[0]; decagon → class 7 → PALETTE[7].
        let tri = tile_with_shape(0, 0, 3);
        let dec = tile_with_shape(0, 0, 10);
        assert_eq!(tile_rgb(ColorMode::Shape, &tri), PALETTE[0]);
        assert_eq!(tile_rgb(ColorMode::Shape, &dec), PALETTE[7]);
        // The shape class must not leak from seed/proto.
        assert_eq!(
            tile_rgb(ColorMode::Shape, &tile_with_shape(7, 3, 6)),
            tile_rgb(ColorMode::Shape, &tile_with_shape(0, 1, 6))
        );
    }

    #[test]
    fn tile_rgb_metatile_matches_class() {
        // Cap F (proto=3, seed=1) → class 3 → METATILE_PALETTE[3].
        let cap = Tile { proto: 3, seed: 1, poly: poly_with_n_vertices(6) };
        assert_eq!(tile_rgb(ColorMode::Metatile, &cap), METATILE_PALETTE[3]);
        // Hat F1/F2 (proto=0, seed 8/9) → class 3 → same slot.
        let hat = Tile { proto: 0, seed: 8, poly: poly_with_n_vertices(6) };
        assert_eq!(tile_rgb(ColorMode::Metatile, &hat), METATILE_PALETTE[3]);
    }

    #[test]
    fn tile_rgb_gray_follows_proto() {
        assert_eq!(tile_rgb(ColorMode::Gray, &tile(0, 0)), GRAY_PALETTE[0]);
        assert_eq!(tile_rgb(ColorMode::Gray, &tile(3, 0)), GRAY_PALETTE[3]);
        assert_eq!(tile_rgb(ColorMode::Gray, &tile(8, 0)), GRAY_PALETTE[0]);
        assert_ne!(
            tile_rgb(ColorMode::Gray, &tile(0, 0)),
            tile_rgb(ColorMode::Gray, &tile(1, 0))
        );
        // seed must not affect the gray ramp (Gray is proto-indexed).
        assert_eq!(tile_rgb(ColorMode::Gray, &tile(2, 7)), GRAY_PALETTE[2]);
    }
}
