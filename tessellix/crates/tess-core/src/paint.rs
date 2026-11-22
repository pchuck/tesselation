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

pub fn tile_rgb(mode: ColorMode, tile: &Tile) -> u32 {
    let idx = match mode {
        ColorMode::Single => 0,
        ColorMode::Pattern => (tile.seed as usize) % PALETTE.len(),
        ColorMode::Proto => (tile.proto as usize) % PALETTE.len(),
        ColorMode::Gray => (tile.proto as usize) % GRAY_PALETTE.len(),
    };
    match mode {
        ColorMode::Gray => GRAY_PALETTE[idx],
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
