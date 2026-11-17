//! Symmetry metadata: the 17 wallpaper groups and lattice types.

use std::fmt;

/// The 17 wallpaper groups (international notation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperGroup {
    P1, P2,
    Pm, Pg, Cm, Pmm,
    Pgg, Cmm, Pmg,
    P4, P4m, P4g,
    P3, P3m1, P31m,
    P6, P6m,
}

impl WallpaperGroup {
    pub const ALL: [WallpaperGroup; 17] = [
        WallpaperGroup::P1, WallpaperGroup::P2,
        WallpaperGroup::Pm, WallpaperGroup::Pg, WallpaperGroup::Cm, WallpaperGroup::Pmm,
        WallpaperGroup::Pgg, WallpaperGroup::Cmm, WallpaperGroup::Pmg,
        WallpaperGroup::P4, WallpaperGroup::P4m, WallpaperGroup::P4g,
        WallpaperGroup::P3, WallpaperGroup::P3m1, WallpaperGroup::P31m,
        WallpaperGroup::P6, WallpaperGroup::P6m,
    ];
}

impl fmt::Display for WallpaperGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            WallpaperGroup::P1 => "p1",
            WallpaperGroup::P2 => "p2",
            WallpaperGroup::Pm => "pm",
            WallpaperGroup::Pg => "pg",
            WallpaperGroup::Cm => "cm",
            WallpaperGroup::Pmm => "pmm",
            WallpaperGroup::Pgg => "pgg",
            WallpaperGroup::Cmm => "cmm",
            WallpaperGroup::Pmg => "pmg",
            WallpaperGroup::P4 => "p4",
            WallpaperGroup::P4m => "p4m",
            WallpaperGroup::P4g => "p4g",
            WallpaperGroup::P3 => "p3",
            WallpaperGroup::P3m1 => "p3m1",
            WallpaperGroup::P31m => "p31m",
            WallpaperGroup::P6 => "p6",
            WallpaperGroup::P6m => "p6m",
        };
        f.write_str(s)
    }
}

/// Lattice type of a periodic tiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LatticeKind {
    Oblique,
    Rectangular,
    /// Centered rectangular lattice (crystallographically `cmm`-family
    /// groups): a rectangular lattice with an extra point at the cell
    /// center.
    CenteredRectangular,
    Square,
    Hexagonal,
    /// Not a lattice tiling (e.g. aperiodic tilings).
    None,
}

impl fmt::Display for LatticeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LatticeKind::Oblique => "oblique",
            LatticeKind::Rectangular => "rectangular",
            LatticeKind::CenteredRectangular => "centered",
            LatticeKind::Square => "square",
            LatticeKind::Hexagonal => "hexagonal",
            LatticeKind::None => "none",
        };
        f.write_str(s)
    }
}

/// Symmetry facts a tiling can report about itself.
#[derive(Debug, Clone, Copy)]
pub struct SymmetryMeta {
    pub group: WallpaperGroup,
    pub lattice: LatticeKind,
    pub notes: Option<&'static str>,
}

impl SymmetryMeta {
    pub fn new(group: WallpaperGroup, lattice: LatticeKind) -> Self {
        Self { group, lattice, notes: None }
    }

    pub fn with_notes(mut self, notes: &'static str) -> Self {
        self.notes = Some(notes);
        self
    }
}

impl fmt::Display for SymmetryMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} / {}", self.group, self.lattice)?;
        if let Some(n) = self.notes {
            write!(f, " ({n})")?;
        }
        Ok(())
    }
}
