//! Viewport (pan/zoom state) and color modes.

use crate::geometry::{BBox, Vec2};

/// How tiles are colored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorMode {
    #[default]
    Single,
    /// Color by the tile's `seed` (pattern chosen by the generator).
    Pattern,
    /// Color by prototile index.
    Proto,
    /// Monochrome variant of `Proto`: the same prototile-class indices
    /// drawn from a gray lightness ramp instead of the color palette —
    /// the print/e-ink mode (structure without hue).
    Gray,
    /// Rec.709 grayscale variant of `Pattern`: the same pattern-indexed
    /// selection (`seed % 8`) drawn from a gray lightness ramp instead of
    /// the color palette — pattern structure without hue. Appended after
    /// `Gray` (not before) so the existing ALL-index pins stay stable.
    PatternGray,
    /// Color by the polygon shape class derived from the tile's polygon:
    /// the vertex count mapped into the palette index space (triangle → 0,
    /// quadrilateral → 1, … decagon-and-beyond → 7). Polygonal shape, NOT
    /// the prototile class — the geometric counterpart of `Proto`, shared by
    /// every family. Appended after `PatternGray` so the existing ALL-index
    /// pins stay stable.
    Shape,
    /// Color by the shape role already encoded in the tile's `seed` — the
    /// islamic-family star/petal/background distinction (and the same seed
    /// for any family whose seeds carry a class) — drawn from a dedicated
    /// warm/earth ramp (`paint::ROLE_PALETTE`) instead of the rainbow
    /// palette. Appended after `Shape` so the existing ALL-index pins stay
    /// stable.
    Role,
    /// Color by the aperiodic metatile / cluster classification: cap
    /// metatile type (0..=3), hat cluster role (H/T/P/F, seeds 1..=9),
    /// turtle + spectre matching-rule label (0..=8), with a proto-
    /// class fallback for non-aperiodic tiles. Appended last so the
    /// existing ALL-index pins stay stable.
    Metatile,
    /// Color by the tile's geometric orientation: the polygon's dominant
    /// undirected axis (total edge length per axis direction, the
    /// argmax), quantized into a small set of 45-degree buckets — 0°
    /// (axis-aligned, horizontal), 45° (diagonal), 90° (axis-aligned,
    /// vertical), 135° (anti-diagonal) — drawn from a dedicated
    /// 4-entry ramp. The geometric counterpart of `Shape`: it encodes
    /// *how* the tile is laid, not which shape it is.
    Orientation,
    /// Color by the tile's substitution level (`Tile::level`): the
    /// hierarchy depth tracked by substitution engines — the number of
    /// refinement/expansion rounds that produced the tile. Level 0
    /// (non-hierarchical, or the level-0 seed plane of a fractalizing
    /// family) gets a dedicated neutral tone; levels 1..=8 get ordered
    /// ramp steps, and deeper levels cycle the 8 steps (level-1 mod 8),
    /// so adjacent levels always land on distinct tones. The uniform
    /// engines pick one expansion depth per view, so the mode reads the
    /// patch's position in the family's hierarchy and steps as the view
    /// crosses substitution octaves; the irreptile engines (golden bee,
    /// golden triangle) show genuinely nested levels at once. Appended
    /// last so the existing ALL-index pins stay stable.
    Level,
}

impl ColorMode {
    pub const ALL: [ColorMode; 10] = [
        ColorMode::Single,
        ColorMode::Pattern,
        ColorMode::Proto,
        ColorMode::Gray,
        ColorMode::PatternGray,
        ColorMode::Shape,
        ColorMode::Role,
        ColorMode::Metatile,
        ColorMode::Orientation,
        ColorMode::Level,
    ];

    pub fn next(self) -> ColorMode {
        let idx = match self {
            ColorMode::Single => 0,
            ColorMode::Pattern => 1,
            ColorMode::Proto => 2,
            ColorMode::Gray => 3,
            ColorMode::PatternGray => 4,
            ColorMode::Shape => 5,
            ColorMode::Role => 6,
            ColorMode::Metatile => 7,
            ColorMode::Orientation => 8,
            ColorMode::Level => 9,
        };
        ColorMode::ALL[(idx + 1) % 10]
    }

    pub fn label(self) -> &'static str {
        match self {
            ColorMode::Single => "single",
            ColorMode::Pattern => "pattern",
            ColorMode::Proto => "proto",
            ColorMode::Gray => "gray",
            ColorMode::PatternGray => "pattern-gray",
            ColorMode::Shape => "shape",
            ColorMode::Role => "role",
            ColorMode::Metatile => "metatile",
            ColorMode::Orientation => "orientation",
            ColorMode::Level => "level",
        }
    }

    pub fn parse(s: &str) -> Option<ColorMode> {
        match s {
            "single" => Some(ColorMode::Single),
            "pattern" => Some(ColorMode::Pattern),
            "proto" => Some(ColorMode::Proto),
            "gray" => Some(ColorMode::Gray),
            "pattern-gray" => Some(ColorMode::PatternGray),
            "shape" => Some(ColorMode::Shape),
            "role" => Some(ColorMode::Role),
            "metatile" => Some(ColorMode::Metatile),
            "orientation" => Some(ColorMode::Orientation),
            "level" => Some(ColorMode::Level),
            _ => None,
        }
    }

    /// The mode a small thumbnail should render in. `Single` colors
    /// every tile alike, which makes a thumbnail a featureless slab —
    /// previews fall back to `Proto` (prototile classes), the mode that
    /// reveals structure at thumbnail scale. `Gray` and `PatternGray` are
    /// self-sufficient (structure without hue) and preview as themselves.
    pub fn preview_mode(self) -> ColorMode {
        match self {
            ColorMode::Single => ColorMode::Proto,
            other => other,
        }
    }
}

/// Camera state. `zoom` is screen pixels per world unit.
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub center: Vec2,
    pub zoom: f64,
}

impl Viewport {
    pub fn new(zoom: f64) -> Self {
        Self {
            center: Vec2::ZERO,
            zoom,
        }
    }

    /// World-space box covered by a viewport of `w` x `h` pixels.
    pub fn world_bbox(&self, w: u32, h: u32) -> BBox {
        let hw = w as f64 / (2.0 * self.zoom);
        let hh = h as f64 / (2.0 * self.zoom);
        BBox::new(
            self.center - Vec2::new(hw, hh),
            self.center + Vec2::new(hw, hh),
        )
    }

    /// Screen-space pan delta (y-axis points down, as in UIs).
    pub fn pan_screen(&mut self, dx_px: f64, dy_px: f64) {
        self.center.x -= dx_px / self.zoom;
        self.center.y += dy_px / self.zoom;
    }

    /// Zoom by `factor`. If `anchor` (a world point, e.g. under the cursor)
    /// is given it stays fixed on screen; otherwise the center stays fixed.
    pub fn zoom_by(&mut self, factor: f64, anchor: Option<Vec2>) {
        let factor = factor.clamp(1e-4, 1e4);
        if let Some(a) = anchor {
            self.center = a - (a - self.center) / factor;
        }
        self.zoom = (self.zoom * factor).clamp(1e-6, 1e6);
    }

    /// World point under a screen position (y-axis down).
    pub fn screen_to_world(&self, sx: f64, sy: f64, w: f64, h: f64) -> Vec2 {
        Vec2::new(
            self.center.x + (sx - w / 2.0) / self.zoom,
            self.center.y - (sy - h / 2.0) / self.zoom,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bbox_centered() {
        let v = Viewport::new(10.0);
        let b = v.world_bbox(100, 50);
        assert!((b.min.x - (-5.0)).abs() < 1e-12);
        assert!((b.max.x - 5.0).abs() < 1e-12);
        assert!((b.min.y - (-2.5)).abs() < 1e-12);
        assert!((b.max.y - 2.5).abs() < 1e-12);
    }

    #[test]
    fn zoom_anchor_stays() {
        let mut v = Viewport::new(10.0);
        let a = Vec2::new(1.0, 2.0);
        let before = |v: &Viewport| v.screen_to_world(64.0, 32.0, 100.0, 50.0);
        // anchor is at screen (64, 32) currently:
        v.center = a - Vec2::new((64.0 - 50.0) / 10.0, -(32.0 - 25.0) / 10.0);
        let p = before(&v);
        v.zoom_by(2.0, Some(a));
        let q = before(&v);
        assert!((p.x - q.x).abs() < 1e-9);
        assert!((p.y - q.y).abs() < 1e-9);
    }

    #[test]
    fn color_mode_cycle_parse_label_preview() {
        assert_eq!(ColorMode::Single.next(), ColorMode::Pattern);
        assert_eq!(ColorMode::Pattern.next(), ColorMode::Proto);
        assert_eq!(ColorMode::Proto.next(), ColorMode::Gray);
        assert_eq!(ColorMode::Gray.next(), ColorMode::PatternGray);
        assert_eq!(ColorMode::PatternGray.next(), ColorMode::Shape);
        assert_eq!(ColorMode::Shape.next(), ColorMode::Role);
        assert_eq!(ColorMode::Role.next(), ColorMode::Metatile);
        assert_eq!(ColorMode::Metatile.next(), ColorMode::Orientation);
        assert_eq!(ColorMode::Orientation.next(), ColorMode::Level);
        assert_eq!(ColorMode::Level.next(), ColorMode::Single);
        assert_eq!(ColorMode::parse("gray"), Some(ColorMode::Gray));
        assert_eq!(ColorMode::parse("pattern-gray"), Some(ColorMode::PatternGray));
        assert_eq!(ColorMode::parse("shape"), Some(ColorMode::Shape));
        assert_eq!(ColorMode::parse("role"), Some(ColorMode::Role));
        assert_eq!(ColorMode::parse("metatile"), Some(ColorMode::Metatile));
        assert_eq!(ColorMode::parse("orientation"), Some(ColorMode::Orientation));
        assert_eq!(ColorMode::parse("level"), Some(ColorMode::Level));
        assert_eq!(ColorMode::parse("nope"), None);
        assert_eq!(ColorMode::Gray.label(), "gray");
        assert_eq!(ColorMode::PatternGray.label(), "pattern-gray");
        assert_eq!(ColorMode::Shape.label(), "shape");
        assert_eq!(ColorMode::Role.label(), "role");
        assert_eq!(ColorMode::Metatile.label(), "metatile");
        assert_eq!(ColorMode::Orientation.label(), "orientation");
        assert_eq!(ColorMode::Level.label(), "level");
        assert_eq!(
            ColorMode::parse(ColorMode::PatternGray.label()),
            Some(ColorMode::PatternGray)
        );
        assert_eq!(
            ColorMode::parse(ColorMode::Metatile.label()),
            Some(ColorMode::Metatile)
        );
        assert_eq!(
            ColorMode::parse(ColorMode::Orientation.label()),
            Some(ColorMode::Orientation)
        );
        assert_eq!(
            ColorMode::parse(ColorMode::Level.label()),
            Some(ColorMode::Level)
        );
        assert_eq!(ColorMode::Gray.preview_mode(), ColorMode::Gray);
        assert_eq!(ColorMode::PatternGray.preview_mode(), ColorMode::PatternGray);
        // The structure modes (Shape, Role, Metatile, Orientation, Level)
        // preview as themselves (like Gray/PatternGray) — thumbnails keep
        // their structure and are NOT switched onto a legacy mode.
        assert_eq!(ColorMode::Shape.preview_mode(), ColorMode::Shape);
        assert_eq!(ColorMode::Role.preview_mode(), ColorMode::Role);
        assert_eq!(ColorMode::Metatile.preview_mode(), ColorMode::Metatile);
        assert_eq!(ColorMode::Orientation.preview_mode(), ColorMode::Orientation);
        assert_eq!(ColorMode::Level.preview_mode(), ColorMode::Level);
    }

    #[test]
    fn color_mode_all_len_is_ten() {
        assert_eq!(ColorMode::ALL.len(), 10);
        assert_eq!(ColorMode::ALL[4], ColorMode::PatternGray);
        assert_eq!(ColorMode::ALL[5], ColorMode::Shape);
        assert_eq!(ColorMode::ALL[6], ColorMode::Role);
        assert_eq!(ColorMode::ALL[7], ColorMode::Metatile);
        assert_eq!(ColorMode::ALL[8], ColorMode::Orientation);
        assert_eq!(ColorMode::ALL[9], ColorMode::Level);
    }
}
