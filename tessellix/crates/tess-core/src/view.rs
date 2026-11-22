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
}

impl ColorMode {
    pub const ALL: [ColorMode; 4] = [
        ColorMode::Single,
        ColorMode::Pattern,
        ColorMode::Proto,
        ColorMode::Gray,
    ];

    pub fn next(self) -> ColorMode {
        let idx = match self {
            ColorMode::Single => 0,
            ColorMode::Pattern => 1,
            ColorMode::Proto => 2,
            ColorMode::Gray => 3,
        };
        ColorMode::ALL[(idx + 1) % 4]
    }

    pub fn label(self) -> &'static str {
        match self {
            ColorMode::Single => "single",
            ColorMode::Pattern => "pattern",
            ColorMode::Proto => "proto",
            ColorMode::Gray => "gray",
        }
    }

    pub fn parse(s: &str) -> Option<ColorMode> {
        match s {
            "single" => Some(ColorMode::Single),
            "pattern" => Some(ColorMode::Pattern),
            "proto" => Some(ColorMode::Proto),
            "gray" => Some(ColorMode::Gray),
            _ => None,
        }
    }

    /// The mode a small thumbnail should render in. `Single` colors
    /// every tile alike, which makes a thumbnail a featureless slab —
    /// previews fall back to `Proto` (prototile classes), the mode that
    /// reveals structure at thumbnail scale. `Gray` is self-sufficient
    /// (structure without hue) and previews as itself.
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
        assert_eq!(ColorMode::Gray.next(), ColorMode::Single);
        assert_eq!(ColorMode::parse("gray"), Some(ColorMode::Gray));
        assert_eq!(ColorMode::parse("nope"), None);
        assert_eq!(ColorMode::Gray.label(), "gray");
        assert_eq!(ColorMode::Gray.preview_mode(), ColorMode::Gray);
    }
}
