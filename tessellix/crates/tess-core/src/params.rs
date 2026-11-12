//! Typed, self-describing tiling parameters.
//!
//! Every tiling declares its parameters via [`ParamSpec`] so that UIs can
//! discover and edit them generically — adding a new tiling never requires
//! touching UI code.

use std::collections::BTreeMap;
use std::fmt;

/// Stable identifier for a parameter within a tiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParamId(&'static str);

impl ParamId {
    pub const fn new(s: &'static str) -> Self {
        Self(s)
    }

    pub fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for ParamId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// The kind and constraints of a parameter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParamKind {
    Float { min: f64, max: f64, step: f64, default: f64 },
    Int { min: i64, max: i64, step: i64, default: i64 },
    Bool { default: bool },
    /// Angle in degrees.
    Angle { min: f64, max: f64, step: f64, default: f64 },
}

impl ParamKind {
    pub fn default_value(self) -> ParamValue {
        match self {
            ParamKind::Float { default, .. } => ParamValue::Float(default),
            ParamKind::Int { default, .. } => ParamValue::Int(default),
            ParamKind::Bool { default } => ParamValue::Bool(default),
            ParamKind::Angle { default, .. } => ParamValue::Angle(default),
        }
    }
}

/// A concrete parameter value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParamValue {
    Float(f64),
    Int(i64),
    Bool(bool),
    Angle(f64),
}

impl ParamValue {
    pub fn as_f64(self) -> Option<f64> {
        match self {
            ParamValue::Float(v) | ParamValue::Angle(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_i64(self) -> Option<i64> {
        match self {
            ParamValue::Int(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_bool(self) -> Option<bool> {
        match self {
            ParamValue::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Human-readable value for UI status lines.
    pub fn display(self) -> String {
        match self {
            ParamValue::Float(v) => format_smart(v),
            ParamValue::Int(v) => v.to_string(),
            ParamValue::Bool(v) => if v { "on" } else { "off" }.to_string(),
            ParamValue::Angle(v) => format!("{:.0}°", v),
        }
    }
}

/// Format a float with sensible precision (avoids 0.30000000000000004).
pub fn format_smart(v: f64) -> String {
    if v.fract().abs() < 1e-9 {
        format!("{:.0}", v)
    } else if (v * 100.0).fract().abs() < 1e-6 {
        format!("{:.2}", v)
    } else if (v * 1000.0).fract().abs() < 1e-5 {
        format!("{:.3}", v)
    } else {
        format!("{:.4}", v)
    }
}

/// Declaration of a tiling parameter, enough for UIs to render an editor.
#[derive(Debug, Clone, Copy)]
pub struct ParamSpec {
    pub id: ParamId,
    pub label: &'static str,
    pub kind: ParamKind,
}

impl ParamSpec {
    pub const fn float(
        id: ParamId,
        label: &'static str,
        min: f64,
        max: f64,
        step: f64,
        default: f64,
    ) -> Self {
        Self { id, label, kind: ParamKind::Float { min, max, step, default } }
    }

    pub const fn int(
        id: ParamId,
        label: &'static str,
        min: i64,
        max: i64,
        step: i64,
        default: i64,
    ) -> Self {
        Self { id, label, kind: ParamKind::Int { min, max, step, default } }
    }

    pub const fn bool(id: ParamId, label: &'static str, default: bool) -> Self {
        Self { id, label, kind: ParamKind::Bool { default } }
    }

    pub const fn angle(
        id: ParamId,
        label: &'static str,
        min: f64,
        max: f64,
        step: f64,
        default: f64,
    ) -> Self {
        Self { id, label, kind: ParamKind::Angle { min, max, step, default } }
    }

    pub fn default_value(self) -> ParamValue {
        self.kind.default_value()
    }

    /// Advance a value by `dir` steps (dir = -1, 0 or +1), keeping it in range.
    /// The result is snapped to the grid `min + k * step`.
    pub fn step_by(&self, v: ParamValue, dir: i32) -> ParamValue {
        if dir == 0 {
            return v;
        }
        match (self.kind, v) {
            (ParamKind::Float { min, max, step, .. }, ParamValue::Float(x)) => {
                let y = snap_to_grid(x + dir as f64 * step, min, step).clamp(min, max);
                ParamValue::Float(y)
            }
            (ParamKind::Angle { min, max, step, .. }, ParamValue::Angle(x)) => {
                let range = max - min;
                let mut y = snap_to_grid(x + dir as f64 * step, min, step);
                while y < min {
                    y += range;
                }
                while y >= max {
                    y -= range;
                }
                ParamValue::Angle(y)
            }
            (ParamKind::Int { min, max, step, .. }, ParamValue::Int(x)) => {
                let y = (x + dir as i64 * step).clamp(min, max);
                ParamValue::Int(y)
            }
            (ParamKind::Bool { .. }, ParamValue::Bool(x)) => ParamValue::Bool(!x),
            // Mismatched kinds: leave unchanged.
            _ => v,
        }
    }
}

fn snap_to_grid(v: f64, min: f64, step: f64) -> f64 {
    if step <= 0.0 {
        return v;
    }
    min + ((v - min) / step).round() * step
}

/// The current values of a tiling's parameters.
#[derive(Debug, Clone, Default)]
pub struct ParamSet {
    inner: BTreeMap<ParamId, ParamValue>,
}

impl ParamSet {
    /// An empty set (reads always fall back to defaults).
    pub const EMPTY: Self = Self { inner: BTreeMap::new() };

    /// A set filled with every spec's default value.
    pub fn default_for(specs: &[ParamSpec]) -> Self {
        Self {
            inner: specs.iter().map(|s| (s.id, s.default_value())).collect(),
        }
    }

    pub fn get(&self, id: ParamId) -> Option<ParamValue> {
        self.inner.get(&id).copied()
    }

    pub fn set(&mut self, id: ParamId, v: ParamValue) {
        self.inner.insert(id, v);
    }

    pub fn float(&self, id: ParamId, default: f64) -> f64 {
        self.get(id).and_then(|v| v.as_f64()).unwrap_or(default)
    }

    pub fn int(&self, id: ParamId, default: i64) -> i64 {
        self.get(id).and_then(|v| v.as_i64()).unwrap_or(default)
    }

    pub fn bool(&self, id: ParamId, default: bool) -> bool {
        self.get(id).and_then(|v| v.as_bool()).unwrap_or(default)
    }

    pub fn angle(&self, id: ParamId, default: f64) -> f64 {
        self.get(id).and_then(|v| v.as_f64()).unwrap_or(default)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ParamId, &ParamValue)> {
        self.inner.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values_are_explicit() {
        assert_eq!(
            ParamSpec::float(ParamId::new("s"), "s", 1.0, 2.0, 0.5, 1.5).default_value(),
            ParamValue::Float(1.5)
        );
        assert_eq!(
            ParamSpec::angle(ParamId::new("r"), "r", 0.0, 360.0, 5.0, 90.0).default_value(),
            ParamValue::Angle(90.0)
        );
        assert_eq!(
            ParamSpec::int(ParamId::new("i"), "i", -2, 2, 1, 1).default_value(),
            ParamValue::Int(1)
        );
        assert_eq!(
            ParamSpec::bool(ParamId::new("b"), "b", false).default_value(),
            ParamValue::Bool(false)
        );
        assert_eq!(
            ParamSpec::bool(ParamId::new("b2"), "b2", true).default_value(),
            ParamValue::Bool(true)
        );
    }

    #[test]
    fn angle_wraps() {
        let spec = ParamSpec::angle(ParamId::new("rot"), "rot", 0.0, 360.0, 5.0, 0.0);
        assert_eq!(spec.step_by(ParamValue::Angle(355.0), 1), ParamValue::Angle(0.0));
        assert_eq!(spec.step_by(ParamValue::Angle(5.0), -1), ParamValue::Angle(0.0));
        assert_eq!(spec.step_by(ParamValue::Angle(2.5), -1), ParamValue::Angle(355.0));
        // Off-grid values (e.g. from a slider) land on the nearest grid point.
        assert_eq!(spec.step_by(ParamValue::Angle(357.5), 1), ParamValue::Angle(5.0));
    }

    #[test]
    fn float_clamps() {
        let spec = ParamSpec::float(ParamId::new("s"), "s", 1.0, 2.0, 0.5, 1.0);
        assert_eq!(spec.step_by(ParamValue::Float(1.5), 1), ParamValue::Float(2.0));
        assert_eq!(spec.step_by(ParamValue::Float(2.0), 1), ParamValue::Float(2.0));
    }

    #[test]
    fn set_get_roundtrip() {
        let mut set = ParamSet::default();
        set.set(ParamId::new("x"), ParamValue::Float(1.25));
        assert_eq!(set.float(ParamId::new("x"), 0.0), 1.25);
        assert_eq!(set.float(ParamId::new("missing"), 7.0), 7.0);
    }
}
