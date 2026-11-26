//! The 24h PIC explorer (docs/briefs/pic-explorer.md): a deterministic
//! δ/θ sweep over the registered lattice substrates with a programmatic
//! leaderboard and a ranked per-candidate gallery. No RNG, no time, no
//! filesystem in this module — those live in the CLI layer.
//!
//! Two axes, both deterministic:
//!
//! * **δ axis** — every registered substrate with a fixed lattice basis
//!   ([`Tiling::lattice_basis`] returns `Some`), contact angle δ ∈
//!   {5°, …, 85°} (multiples of 5°) plus the exact validated envelope
//!   points for the four envelope-bearing substrates (the 90° square
//!   envelope point is outside the base grid and is added by refinement).
//!   Every candidate is scored and recorded, INCLUDING open-strand
//!   regimes — the M165 envelope is a closed-angle point set, not a
//!   requirement for rendering (open weaves are legitimate design
//!   space). A closed weave at a NON-envelope δ (exits/corners/bad all
//!   zero) would contradict the M165 envelope table and is recorded as
//!   `closure_violation` — an honest finding, never silently dropped.
//! * **θ axis** (`islamic` feature) — per-substrate motif-angle offsets
//!   ∈ {−15°, …, +15°} relative to each class's classic motif angle, on
//!   the two hankin substrates that carry a classic motif assignment
//!   (`hankin-4-8-8-motifs`: the landed M163 motifs; `hankin-3-12-12`:
//!   the canonical Kaplan `Water Rings.tap` reading, Star 45°/s=1 per
//!   class — see [`classic_motifs_3_12_12`]). ONE offset is applied
//!   to every class's classic `theta_deg` (a documented interpretation
//!   of "±{5,10,15}° per-class offsets": a single per-substrate offset,
//!   not a per-class combination grid — 7 rows per substrate). Feasibility
//!   is panic-caught ([`hankin_motifs::build_cells_with_motifs`]);
//!   infeasible candidates are honest failure rows, never skipped.
//!
//! Features per δ candidate (never hidden): envelope membership +
//! envelope angles, validity stats (core vertices / exit / corner /
//! bad / straight2 / joint), wall-map features (degree histogram,
//! strand count, face count, minimum face area), strapwork geometry
//! (total wall length, strap density, strict crossing count + acute
//! angle, crossing density), symmetry probes (fold trick via lattice
//! translation, measured mirror + measured rotation where the group
//! claims them), generation message, closure-violation guard, and the
//! documented score. The score deliberately is NOT aesthetic: it
//! separates closed weaves from open regimes, then ranks structural
//! integrity, slivers and symmetry-invariance (module docs for
//! [`documented_score`]).

use std::collections::{BTreeMap, BTreeSet, HashMap};

use tess_core::geometry::{BBox, Vec2};
use tess_core::params::{ParamSet, ParamValue};
use tess_core::strapwork::{
    plan_decoration, validity_stats, wall_map_features, Decoration, Wall,
};
use tess_core::tiling::{Query, Tiling, Tile};
use tess_core::symmetry::{SymmetryMeta, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::envelope::envelope_for;
use crate::lattice::{SCALE_ID, ROTATION_ID};

/// The fixed patch window every sweep runs at: 20×20 world units
/// centered on the origin at scale 1 (the M165 CLI convention).
pub const REGION: BBox = BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0));

/// δ grid: multiples of 5° from 5° to 85° (the brief's base grid).
pub const DELTA_GRID: [f64; 17] = [
    5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0, 75.0, 80.0,
    85.0,
];

/// Per-substrate θ offsets on the hankin θ axis, in degrees.
pub const THETA_GRID: [f64; 7] = [-15.0, -10.0, -5.0, 0.0, 5.0, 10.0, 15.0];

/// The 2-class hankin substrates the θ axis targets (exact list).
pub const THETA_AXIS_IDS: [&str; 2] = ["hankin-4-8-8-motifs", "hankin-3-12-12"];

/// Decoration band width for the gallery (world units).
pub const GALLERY_BAND_W: f64 = 0.1;

// ---------------------------------------------------------------------------
// Feature vectors (never hidden — every row serializes these fields)
// ---------------------------------------------------------------------------

/// The full per-candidate δ-axis feature vector.
#[derive(Debug, Clone, Default)]
pub struct SweepFeatures {
    /// `None` = no validated envelope for this substrate;
    /// `Some(true)` = this δ is a validated closed angle;
    /// `Some(false)` = not an envelope member.
    pub envelope_member: Option<bool>,
    /// The validated closed angles, if this substrate has an envelope.
    pub envelope_angles: Option<Vec<f64>>,
    /// Region-gated validity stats over core vertices.
    pub core_vertices: usize,
    pub exit: usize,
    pub corner: usize,
    pub bad: usize,
    pub straight2: usize,
    pub joint: usize,
    /// Degree histogram over core vertices.
    pub degree_hist: BTreeMap<usize, usize>,
    pub strand_count: usize,
    pub face_count: usize,
    /// Smallest absolute face area (sliver metric); 0.0 if no faces.
    pub min_face_area: f64,
    pub total_wall_length: f64,
    /// Total wall length / region area ("strap density").
    pub strap_density: f64,
    /// Strict wall-pair crossings (intersection point inside region).
    pub crossing_count: usize,
    /// Acute crossing angle in degrees; 0.0 if no crossings.
    pub min_crossing_angle_deg: f64,
    /// Crossing count / region area ("crossing density").
    pub crossing_density: f64,
    /// Fold trick: lattice-translation invariance of the wall multiset
    /// on the overlap of the region and its lattice-vector shift.
    pub translation_invariant: bool,
    /// Number of wall-multiset mismatches (0 == invariant).
    pub translation_mismatch: usize,
    /// Mirror check (mirror-claiming groups only). `None` = not checked.
    pub mirror_invariant: Option<bool>,
    /// Rotation check (rotation-claiming groups only; axes at the origin).
    /// `None` = not checked.
    pub rotation_invariant: Option<bool>,
    /// Human-readable record of which symmetry checks ran / why not.
    pub symmetry_note: String,
    /// Honest reason the fold trick was not performed.
    pub fold_note: Option<String>,
    /// Set when generation was refused (e.g. too many tiles).
    pub generation_message: Option<&'static str>,
    /// M165 envelope-table guard: a closed weave at a non-envelope δ.
    pub closure_violation: Option<String>,
}

/// The full per-candidate θ-axis feature vector. Cells are an exact face
/// partition (the θ axis has no wall map — motifs operate on cells), so
/// there is no wall-map / crossing metric on this axis.
#[derive(Debug, Clone, Default)]
pub struct ThetaFeatures {
    /// Per-class motif spec, e.g.
    /// `Rosette(n=8, θ=22.500°, s=3, h=1.00, φ=0.000°) | ExtendedRosette(n=4, θ=67.500°, s=2, h=1.00, φ=0.000°)`.
    pub motifs: String,
    /// Whether the partition closed exactly (per-tile area identity).
    pub partition_complete: bool,
    /// max |Σ face area − tile area| across classes (exact identity at 1e-9).
    pub max_partition_error: f64,
    /// Number of generated cells (faces).
    pub n_cells: usize,
    /// Smallest cell-polygon area (sliver metric on the θ axis).
    pub min_face_area: f64,
    /// Total cell-polygon area.
    pub total_area: f64,
    /// Declared symmetry of the source tiling (display string).
    pub symmetry: String,
    /// Honest feasibility reason when the candidate was infeasible.
    pub feasibility_reason: Option<String>,
}

/// One ranked δ-axis leaderboard row.
#[derive(Debug, Clone)]
pub struct DeltaRow {
    pub substrate_id: String,
    pub delta_deg: f64,
    pub features: SweepFeatures,
    /// Documented composite score (lower is better); `None` = never competes (refused).
    pub score: Option<f64>,
    /// 1-based rank among scored rows; 0 for refused rows.
    pub rank: usize,
}

/// One ranked θ-axis leaderboard row.
#[derive(Debug, Clone)]
pub struct ThetaRow {
    pub substrate_id: String,
    /// Offset applied to every class's classic motif angle, in degrees.
    pub theta_offset_deg: f64,
    pub features: ThetaFeatures,
    /// Documented composite score; `None` = never competes (infeasible).
    pub score: Option<f64>,
    /// 1-based rank among scored rows; 0 for infeasible rows.
    pub rank: usize,
}

/// A single row from either axis, for the leaderboard writer.
#[derive(Debug, Clone)]
pub enum Row {
    Delta(DeltaRow),
    Theta(ThetaRow),
}

impl Row {
    pub fn substrate_id(&self) -> &str {
        match self {
            Row::Delta(r) => &r.substrate_id,
            Row::Theta(r) => &r.substrate_id,
        }
    }
    /// 1-based rank (per axis, per substrate); 0 = refused/infeasible.
    pub fn rank(&self) -> usize {
        match self {
            Row::Delta(r) => r.rank,
            Row::Theta(r) => r.rank,
        }
    }
    pub fn score(&self) -> Option<f64> {
        match self {
            Row::Delta(r) => r.score,
            Row::Theta(r) => r.score,
        }
    }
}

/// The full sweep over one substrate (both axes).
#[derive(Debug, Clone, Default)]
pub struct SweepOutcome {
    pub substrate_id: String,
    pub delta_rows: Vec<DeltaRow>,
    pub theta_rows: Vec<ThetaRow>,
    /// The δ-axis base tiles, reused by the gallery renderer (the θ axis
    /// renders its own cells instead). Empty when generation was refused.
    pub delta_tiles: Vec<Tile>,
}

impl SweepOutcome {
    /// Rows in leaderboard order: δ rows (scored, rank order; refused
    /// sink at the bottom), then θ rows (same rule).
    pub fn rows(&self) -> Vec<Row> {
        let mut out: Vec<Row> = Vec::with_capacity(self.delta_rows.len() + self.theta_rows.len());
        for r in &self.delta_rows {
            out.push(Row::Delta(r.clone()));
        }
        for r in &self.theta_rows {
            out.push(Row::Theta(r.clone()));
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn params_at_default(tiling: &dyn Tiling) -> ParamSet {
    let mut params = ParamSet::default_for(tiling.params());
    params.set(SCALE_ID, ParamValue::Float(1.0));
    params.set(ROTATION_ID, ParamValue::Angle(0.0));
    params
}

fn bbox_area(r: &BBox) -> f64 {
    r.size().x * r.size().y
}

fn shift_region(r: &BBox, t: Vec2) -> BBox {
    BBox::new(r.min + t, r.max + t)
}

fn region_intersection(a: &BBox, b: &BBox) -> BBox {
    BBox::new(
        Vec2::new(a.min.x.max(b.min.x), a.min.y.max(b.min.y)),
        Vec2::new(a.max.x.min(b.max.x), a.max.y.min(b.max.y)),
    )
}

fn wall_mid(w: &Wall) -> Vec2 {
    (w.a + w.b) * 0.5
}

fn wall_aabb(w: &Wall) -> (f64, f64, f64, f64) {
    (
        w.a.x.min(w.b.x),
        w.a.y.min(w.b.y),
        w.a.x.max(w.b.x),
        w.a.y.max(w.b.y),
    )
}

// ---------------------------------------------------------------------------
// Wall-map features: strict crossing detection (spatial hash, never O(w²))
// ---------------------------------------------------------------------------

/// Strict segment intersection: returns (acute angle, point) if the two
/// segments cross strictly (not at endpoints).
fn strict_segment_crossing(w1: &Wall, w2: &Wall) -> Option<(f64, Vec2)> {
    let d1 = w1.b - w1.a;
    let d2 = w2.b - w2.a;
    let den = d1.cross(d2);
    if den.abs() < 1e-12 {
        return None; // parallel
    }
    let q = w2.a - w1.a;
    let t = q.cross(d2) / den;
    let u = q.cross(d1) / den;
    if t <= 1e-6 || t >= 1.0 - 1e-6 || u <= 1e-6 || u >= 1.0 - 1e-6 {
        return None;
    }
    let l1 = d1.length();
    let l2 = d2.length();
    if l1 < 1e-12 || l2 < 1e-12 {
        return None;
    }
    let c = (d1.dot(d2)).abs() / (l1 * l2);
    if c > 1.0 + 1e-9 {
        return None;
    }
    let angle = c.clamp(0.0, 1.0).acos().to_degrees();
    let point = w1.a + d1 * t;
    Some((angle, point))
}

/// Strict wall-pair crossings whose intersection point lands inside the
/// region. Uniform spatial hash (bin size = 2 × max wall length): only
/// nearby wall-pairs are tested — O(w · neighborhood), never O(w²).
fn crossing_features(walls: &[Wall], region: &BBox) -> (usize, f64) {
    let n = walls.len();
    if n < 2 {
        return (0, 0.0);
    }
    let max_len = walls
        .iter()
        .map(|w| w.length())
        .fold(0.0_f64, f64::max)
        .max(1e-6);
    let bin_size = 2.0 * max_len;
    let mut bins: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
    for (i, w) in walls.iter().enumerate() {
        let mid = wall_mid(w);
        let k = (
            (mid.x / bin_size).floor() as i64,
            (mid.y / bin_size).floor() as i64,
        );
        bins.entry(k).or_default().push(i);
    }
    let mut count = 0usize;
    let mut min_angle = f64::INFINITY;
    for (i, w) in walls.iter().enumerate() {
        let mid = wall_mid(w);
        let bx = (mid.x / bin_size).floor() as i64;
        let by = (mid.y / bin_size).floor() as i64;
        let mut seen: BTreeSet<usize> = BTreeSet::new();
        let mut candidates: Vec<usize> = Vec::new();
        for dx in -1..=1i64 {
            for dy in -1..=1i64 {
                if let Some(list) = bins.get(&(bx + dx, by + dy)) {
                    for &j in list {
                        if j > i && seen.insert(j) {
                            candidates.push(j);
                        }
                    }
                }
            }
        }
        for j in candidates {
            let w2 = &walls[j];
            // Cheap AABB reject.
            let (x1, y1, x2, y2) = wall_aabb(w);
            let (x1b, y1b, x2b, y2b) = wall_aabb(w2);
            if x2 < x1b || x1 > x2b || y2 < y1b || y1 > y2b {
                continue;
            }
            if let Some((angle, point)) = strict_segment_crossing(w, w2) {
                if region.contains_point(point) {
                    count += 1;
                    if angle < min_angle {
                        min_angle = angle;
                    }
                }
            }
        }
    }
    (count, if min_angle.is_finite() { min_angle } else { 0.0 })
}

// ---------------------------------------------------------------------------
// Symmetry checks (fold trick + measured rotation/mirror)
// ---------------------------------------------------------------------------

const WALL_KEY_GRID: f64 = 1e6;

type WallKey = ((i64, i64), (i64, i64));

fn wall_key(a: Vec2, b: Vec2) -> WallKey {
    let ka = (
        (a.x * WALL_KEY_GRID).round() as i64,
        (a.y * WALL_KEY_GRID).round() as i64,
    );
    let kb = (
        (b.x * WALL_KEY_GRID).round() as i64,
        (b.y * WALL_KEY_GRID).round() as i64,
    );
    if ka <= kb {
        (ka, kb)
    } else {
        (kb, ka)
    }
}

fn multiset_of(walls: &[Wall], region: &BBox) -> BTreeMap<WallKey, usize> {
    let mut m: BTreeMap<WallKey, usize> = BTreeMap::new();
    for w in walls {
        let mid = wall_mid(w);
        if !region.contains_point(mid) {
            continue;
        }
        *m.entry(wall_key(w.a, w.b)).or_insert(0) += 1;
    }
    m
}

fn multiset_equal(a: &BTreeMap<WallKey, usize>, b: &BTreeMap<WallKey, usize>) -> bool {
    if a == b {
        return true;
    }
    let keys: BTreeSet<&WallKey> = a.keys().chain(b.keys()).collect();
    for k in keys {
        let da = a.get(k).copied().unwrap_or(0) as i64;
        let db = b.get(k).copied().unwrap_or(0) as i64;
        if da != db {
            return false;
        }
    }
    true
}

fn multiset_mismatch(a: &BTreeMap<WallKey, usize>, b: &BTreeMap<WallKey, usize>) -> usize {
    let keys: BTreeSet<&WallKey> = a.keys().chain(b.keys()).collect();
    let mut total = 0usize;
    for k in keys {
        let da = a.get(k).copied().unwrap_or(0) as i64;
        let db = b.get(k).copied().unwrap_or(0) as i64;
        total += (da - db).unsigned_abs() as usize;
    }
    total
}

/// Mirror check (x-axis reflection about the origin; the patch is
/// symmetric about the origin, so the region is invariant). Only the
/// wall midpoints inside the region are compared.
fn mirror_invariance(region: &BBox, tiles: &[Tile], delta_deg: f64) -> (bool, Option<String>) {
    if region.min.x != -region.max.x || region.min.y != -region.max.y {
        return (
            false,
            Some("mirror: patch is not symmetric about the origin".to_string()),
        );
    }
    let w1 = tess_core::strapwork::walls(tiles, delta_deg);
    let wrefl: Vec<Wall> = w1
        .iter()
        .map(|w| Wall {
            a: Vec2::new(w.a.x, -w.a.y),
            b: Vec2::new(w.b.x, -w.b.y),
        })
        .collect();
    let m1 = multiset_of(&w1, region);
    let m2 = multiset_of(&wrefl, region);
    (multiset_equal(&m1, &m2), None)
}

/// Rotation check (about the origin, angle from the declared group).
/// For 60°/120°, the square patch is not rotation-symmetric, so the
/// comparison is restricted to the overlap of the rotated and unrotated
/// patches.
fn rotation_invariance(
    region: &BBox,
    tiles: &[Tile],
    delta_deg: f64,
    angle_deg: f64,
) -> (bool, Option<String>) {
    if region.min.x != -region.max.x || region.min.y != -region.max.y {
        return (
            false,
            Some("rotation: patch is not symmetric about the origin".to_string()),
        );
    }
    let rad = angle_deg.to_radians();
    let (c, s) = (rad.cos(), rad.sin());
    let rotate = |p: Vec2| Vec2::new(p.x * c - p.y * s, p.x * s + p.y * c);
    let w1 = tess_core::strapwork::walls(tiles, delta_deg);
    let wrot: Vec<Wall> = w1
        .iter()
        .map(|w| Wall {
            a: rotate(w.a),
            b: rotate(w.b),
        })
        .collect();
    let rot_min = rotate(region.min);
    let rot_max = rotate(region.max);
    let region_rot = BBox::new(
        Vec2::new(rot_min.x.min(rot_max.x), rot_min.y.min(rot_max.y)),
        Vec2::new(rot_min.x.max(rot_max.x), rot_min.y.max(rot_max.y)),
    );
    let overlap = region_intersection(region, &region_rot);
    if overlap.size().x <= 0.0 || overlap.size().y <= 0.0 {
        return (true, Some("rotation: zero-area overlap".to_string()));
    }
    let m1 = multiset_of(&w1, &overlap);
    let m2 = multiset_of(&wrot, &overlap);
    (multiset_equal(&m1, &m2), None)
}

// ---------------------------------------------------------------------------
// Declared-symmetry helpers (conservative tables — see module docs)
// ---------------------------------------------------------------------------

/// Mirrors claimed by the declared wallpaper group (conservative table:
/// groups with at least one true (non-glide) reflection axis).
fn claimed_mirror(meta: Option<&SymmetryMeta>) -> bool {
    match meta.map(|m| m.group) {
        Some(WallpaperGroup::Pm)
        | Some(WallpaperGroup::Cm)
        | Some(WallpaperGroup::Pmm)
        | Some(WallpaperGroup::Cmm)
        | Some(WallpaperGroup::Pmg)
        | Some(WallpaperGroup::P3m1)
        | Some(WallpaperGroup::P31m)
        | Some(WallpaperGroup::P4m)
        | Some(WallpaperGroup::P6m) => true,
        _ => false,
    }
}

/// Rotation angle claimed by the declared group, restricted to groups
/// whose rotation axes sit at lattice points (unambiguously at the origin
/// of the patch): 90° for P4/P4m/P4g, 60° for P6/P6m, 120° for
/// P3/P3m1/P31m, 180° for P2/Pmm/Cmm. Groups with 2-fold axes at cell
/// centers/midpoints (cm, pmg, pgg) or unspecified axes are not checked.
fn claimed_rotation(meta: Option<&SymmetryMeta>) -> Option<f64> {
    match meta.map(|m| m.group) {
        Some(WallpaperGroup::P4) | Some(WallpaperGroup::P4m) | Some(WallpaperGroup::P4g) => {
            Some(90.0)
        }
        Some(WallpaperGroup::P6) | Some(WallpaperGroup::P6m) => Some(60.0),
        Some(WallpaperGroup::P3) | Some(WallpaperGroup::P3m1) | Some(WallpaperGroup::P31m) => {
            Some(120.0)
        }
        Some(WallpaperGroup::P2) | Some(WallpaperGroup::Pmm) | Some(WallpaperGroup::Cmm) => {
            Some(180.0)
        }
        _ => None,
    }
}

fn symmetry_note(meta: Option<&SymmetryMeta>, mirror_ok: Option<bool>, rotation: Option<f64>, rotation_ok: Option<bool>) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(meta) = meta {
        parts.push(meta.group.to_string());
    } else {
        parts.push("no declared symmetry".to_string());
        return parts.join("; ");
    }
    match mirror_ok {
        Some(true) => parts.push("mirror-x holds".to_string()),
        Some(false) => parts.push("mirror-x fails".to_string()),
        None => parts.push("mirror not claimed".to_string()),
    }
    match rotation {
        Some(a) => parts.push(format!(
            "rot{} {}",
            a.to_string(),
            if rotation_ok.unwrap_or(false) {
                "holds"
            } else {
                "fails"
            }
        )),
        None => parts.push("rotation not claimed".to_string()),
    }
    parts.join("; ")
}

// ---------------------------------------------------------------------------
// Scoring (documented composite score — lower is better)
// ---------------------------------------------------------------------------

/// Compose the δ-axis score. Deliberately NOT an aesthetic objective:
/// this separates closed weaves from open regimes, then ranks by
/// structural integrity, slivers and symmetry-invariance:
///
/// ```text
/// score = 1000.0 · (NOT an envelope member)
///       + 100.0 · bad
///       + 10.0 · (exit + corner)
///       + 1/(min_face_area + 1e-9)          (sliver; 10.0 if no faces)
///       + 10.0 · (1 − translation_invariant)
///       + 0.5 · (1 − mirror_invariant)      [when checked]
///       + 0.5 · (1 − rotation_invariant)    [when checked]
/// ```
fn documented_score(f: &SweepFeatures) -> f64 {
    let non_envelope = match f.envelope_member {
        Some(true) => 0.0,
        _ => 1000.0,
    };
    let integrity = 100.0 * f.bad as f64 + 10.0 * (f.exit + f.corner) as f64;
    let sliver = if f.face_count == 0 {
        10.0
    } else {
        1.0 / (f.min_face_area + 1e-9)
    };
    let sym = 10.0 * if f.translation_invariant { 0.0 } else { 1.0 };
    let mirror = match f.mirror_invariant {
        Some(true) => 0.0,
        Some(false) => 0.5,
        None => 0.0,
    };
    let rotation = match f.rotation_invariant {
        Some(true) => 0.0,
        Some(false) => 0.5,
        None => 0.0,
    };
    non_envelope + integrity + sliver + sym + mirror + rotation
}

/// θ-axis score: infeasible rows never compete (score None, rank 0);
/// incomplete partitions compete with a 1e6 penalty behind complete rows;
/// complete rows rank by the sliver metric.
fn documented_theta_score(complete: bool, min_face_area: f64) -> f64 {
    let integrity = if complete { 0.0 } else { 1e6 };
    let sliver = 1.0 / (min_face_area + 1e-9);
    integrity + sliver
}

// ---------------------------------------------------------------------------
// δ axis
// ---------------------------------------------------------------------------

/// The δ grid for one substrate: the base grid plus the exact validated
/// envelope points (added even when outside 5°–85° — e.g. the 90° square
/// envelope point — unless already on the grid, so the refined angles
/// are scored).
fn delta_grid(substrate_id: &str) -> Vec<f64> {
    let mut grid = DELTA_GRID.to_vec();
    if let Some(env) = envelope_for(substrate_id) {
        for &a in env {
            if !grid.iter().any(|&x| (x - a).abs() < 1e-6) {
                grid.push(a);
            }
        }
    }
    grid.sort_by(|a, b| a.partial_cmp(b).unwrap());
    grid
}

/// Per-substrate setup: δ-independent generation (once per substrate, not
/// per δ), plus the fold-trick shift and shifted-region tiles (once).
struct SubstrateBase {
    tiles: Vec<Tile>,
    message: Option<&'static str>,
    /// (l1_world, shifted-region tiles, shifted-region).
    fold: Option<(Vec2, Vec<Tile>, BBox)>,
    /// Honest reason the fold trick was not performed.
    fold_note: Option<String>,
}

impl SubstrateBase {
    fn build(tiling: &dyn Tiling, region: &BBox) -> Self {
        let params = params_at_default(tiling);
        let result = tiling.generate(&params, &Query { region: *region });
        let tiles = result.tiles;
        let message = result.message;

        let mut fold: Option<(Vec2, Vec<Tile>, BBox)> = None;
        let mut fold_note: Option<String> = None;
        if message.is_some() {
            fold_note = Some("fold: generation refused (base tiles unavailable)".to_string());
        } else {
            let shift = result
                .periodic
                .map(|p| p[0])
                .or_else(|| tiling.lattice_basis().map(|b| b[0]));
            match shift {
                Some(l1) if l1.length() >= 1e-12 => {
                    let region2 = shift_region(region, l1);
                    let r2 = tiling.generate(&params, &Query { region: region2 });
                    if r2.message.is_some() {
                        fold_note =
                            Some("fold: shifted-region generation refused".to_string());
                    } else {
                        fold = Some((l1, r2.tiles, region2));
                    }
                }
                // No lattice basis (or degenerate basis vector): the fold is
                // not applicable — nothing to report, the caller stays honest.
                _ => {}
            }
        }
        SubstrateBase {
            tiles,
            message,
            fold,
            fold_note,
        }
    }
}

/// The per-δ row. `base` is the δ-independent per-substrate value reused
/// across the grid; `delta_deg` is the only varying input.
fn sweep_delta_row(
    tiling: &dyn Tiling,
    base: &SubstrateBase,
    delta_deg: f64,
    region: &BBox,
) -> DeltaRow {
    let substrate_id = tiling.id().to_string();
    let env_angles = envelope_for(&substrate_id).map(|e| e.to_vec());
    let envelope_member = env_angles
        .as_ref()
        .map(|e| e.iter().any(|&a| (a - delta_deg).abs() < 1e-6));

    let mut features = SweepFeatures {
        envelope_member,
        envelope_angles: env_angles,
        symmetry_note: String::new(),
        ..Default::default()
    };

    if let Some(msg) = base.message {
        features.generation_message = Some(msg);
        return DeltaRow {
            substrate_id,
            delta_deg,
            features,
            score: None,
            rank: 0,
        };
    }

    let tiles = &base.tiles;
    let walls = tess_core::strapwork::walls(tiles, delta_deg);
    let stats = validity_stats(&walls, region);
    let wf = wall_map_features(&walls, region);
    let area = bbox_area(region);
    let total_wall_length: f64 = walls.iter().map(|w| w.length()).sum();
    let (crossing_count, min_crossing_angle_deg) = crossing_features(&walls, region);

    // Symmetry checks.
    let declared = tiling.declared_symmetry();
    let mut mirror_ok: Option<bool> = None;
    let mut rotation_ok: Option<bool> = None;
    let mut fold_note: Option<String> = None;
    let translation_invariant: bool;
    let mut translation_mismatch = 0usize;

    if let Some((l1, tiles2, region2)) = &base.fold {
        let overlap = region_intersection(region, region2);
        if overlap.size().x > 0.0 && overlap.size().y > 0.0 {
            let w2 = tess_core::strapwork::walls(tiles2, delta_deg);
            let w2_back: Vec<Wall> = w2
                .iter()
                .map(|w| Wall {
                    a: w.a - *l1,
                    b: w.b - *l1,
                })
                .collect();
            let m1 = multiset_of(&walls, &overlap);
            let m2 = multiset_of(&w2_back, &overlap);
            translation_mismatch = multiset_mismatch(&m1, &m2);
            translation_invariant = translation_mismatch == 0;
        } else {
            // Zero-area overlap: the fold cannot run — claim nothing.
            fold_note = Some("fold: zero-area overlap".to_string());
            translation_invariant = false;
            translation_mismatch = usize::MAX;
        }
    } else {
        fold_note = base.fold_note.clone().or_else(|| {
            // Fold skipped: honest note. Without the fold the row is not
            // translation-checked — it must NOT claim invariance.
            Some("fold: skipped — translation not checked".to_string())
        });
        translation_invariant = false;
    }

    if claimed_mirror(declared.as_ref()) {
        let (ok, note) = mirror_invariance(region, tiles, delta_deg);
        mirror_ok = Some(ok);
        if let Some(n) = note {
            fold_note = fold_note.or(Some(n));
        }
    }
    if let Some(angle_deg) = claimed_rotation(declared.as_ref()) {
        let (ok, note) = rotation_invariance(region, tiles, delta_deg, angle_deg);
        rotation_ok = Some(ok);
        if let Some(n) = note {
            fold_note = fold_note.or(Some(n));
        }
    }

    features.symmetry_note = symmetry_note(declared.as_ref(), mirror_ok, claimed_rotation(declared.as_ref()), rotation_ok);

    // M165 envelope-table guard: closed weave at a non-envelope δ.
    let closure_violation = match (envelope_member, stats.exit, stats.corner, stats.bad) {
        (Some(false), 0, 0, 0) => Some(format!(
            "closed weave at non-envelope δ {delta_deg}° (exits=0, corners=0, bad=0) — contradicts the M165 envelope table"
        )),
        _ => None,
    };

    features.core_vertices = stats.core_vertices;
    features.exit = stats.exit;
    features.corner = stats.corner;
    features.bad = stats.bad;
    features.straight2 = stats.straight2;
    features.joint = stats.joint;
    features.degree_hist = wf.degree_hist;
    features.strand_count = wf.strand_count;
    features.face_count = wf.face_count;
    features.min_face_area = wf.min_face_area;
    features.total_wall_length = total_wall_length;
    features.strap_density = total_wall_length / area;
    features.crossing_count = crossing_count;
    features.min_crossing_angle_deg = min_crossing_angle_deg;
    features.crossing_density = crossing_count as f64 / area;
    features.translation_invariant = translation_invariant;
    features.translation_mismatch = translation_mismatch;
    features.mirror_invariant = mirror_ok;
    features.rotation_invariant = rotation_ok;
    features.fold_note = fold_note;
    features.closure_violation = closure_violation;

    let score = documented_score(&features);

    DeltaRow {
        substrate_id,
        delta_deg,
        features,
        score: Some(score),
        rank: 0,
    }
}

/// Sort + rank δ rows (scored first, ascending score, ties by δ;
/// refused rows sink to the bottom with rank 0).
pub fn rank_delta_rows(rows: Vec<DeltaRow>) -> Vec<DeltaRow> {
    let mut scored: Vec<DeltaRow> = rows.iter().filter(|r| r.score.is_some()).cloned().collect();
    let mut refused: Vec<DeltaRow> = rows.iter().filter(|r| r.score.is_none()).cloned().collect();
    scored.sort_by(|a, b| {
        let sa = a.score.unwrap();
        let sb = b.score.unwrap();
        sa.partial_cmp(&sb)
            .unwrap()
            .then(a.delta_deg.partial_cmp(&b.delta_deg).unwrap())
    });
    for (i, r) in scored.iter_mut().enumerate() {
        r.rank = i + 1;
    }
    for r in refused.iter_mut() {
        r.rank = 0;
    }
    scored.extend(refused);
    scored
}

/// Run the δ axis for one substrate (grid + envelope refinement points).
pub fn run_delta_axis(tiling: &dyn Tiling, region: &BBox) -> Vec<DeltaRow> {
    let grid = delta_grid(tiling.id());
    let base = SubstrateBase::build(tiling, region);
    let mut rows = Vec::with_capacity(grid.len());
    for &delta in &grid {
        rows.push(sweep_delta_row(tiling, &base, delta, region));
    }
    rank_delta_rows(rows)
}

// ---------------------------------------------------------------------------
// θ axis (islamic-gated)
// ---------------------------------------------------------------------------

#[cfg(feature = "islamic")]
use crate::families::islamic::hankin_motifs::{build_cells_with_motifs, Motif};
#[cfg(feature = "islamic")]
use crate::families::islamic::hankin_4_8_8::tiles as hankin_4_8_8_tiles;
#[cfg(feature = "islamic")]
use crate::families::islamic::hankin_4_8_8_motifs::motifs as hankin_4_8_8_base_motifs;
#[cfg(feature = "islamic")]
use crate::families::islamic::hankin_3_12_12::tiles as hankin_3_12_12_tiles;

/// The classic (offset-0) motif assignment for `hankin-4-8-8-motifs`:
/// the landed M163 motifs (octagon Rosette 22.5°/s=3, diamond
/// ExtendedRosette 67.5°/s=2), used verbatim.
#[cfg(feature = "islamic")]
fn classic_motifs_4_8_8() -> Vec<Option<Motif>> {
    hankin_4_8_8_base_motifs()
}

/// The classic (offset-0) motif assignment for `hankin-3-12-12` — the
/// **canonical Kaplan reading**, pinned by his own saved Taprats design
/// `Water Rings.tap` (tiling `3.12^2`, one of the 24 official Taprats
/// Example Designs; file parsed exactly, 2026-11-25): the dodecagon feature
/// carries the classic `[12/3]s=1` star figure — chords joining successive
/// edge-midpoint contacts, each leaving its contact at exactly 45° to the
/// tile edge (dodecagon-class Star(12, 45°, s=1) in this vocabulary: the
/// d_12-replicated figure equals the `Motif::Star` arm network as a segment
/// multiset, 24/24) — and the triangle feature's saved ExplicitFigure is
/// exactly the 45° arm continuation (both triangle classes Star(3, 45°,
/// s=1): 6 arms cut at first crossings, vertex + edge-multiset match to
/// 1e-15, both orientations). The reading also agrees with the registered
/// M99 `hankin-3-12-12` design angle (θK = 45°, "classical shining
/// twelve-pointed star"). Feasibility is panic-caught and reported honestly
/// per row.
#[cfg(feature = "islamic")]
fn classic_motifs_3_12_12() -> Vec<Option<Motif>> {
    vec![
        Some(Motif::Star { n: 12, theta_deg: 45.0, s: 1 }),
        Some(Motif::Star { n: 3, theta_deg: 45.0, s: 1 }),
        Some(Motif::Star { n: 3, theta_deg: 45.0, s: 1 }),
    ]
}

/// The tiles (cell space) for a θ-axis substrate: each class's polygon.
#[cfg(feature = "islamic")]
fn theta_axis_tiles(substrate_id: &str) -> Option<Vec<&'static [Vec2]>> {
    match substrate_id {
        "hankin-4-8-8-motifs" => Some(hankin_4_8_8_tiles()),
        "hankin-3-12-12" => Some(hankin_3_12_12_tiles()),
        _ => None,
    }
}

/// The classic motifs (offset 0) for a θ-axis substrate.
#[cfg(feature = "islamic")]
fn classic_motifs(substrate_id: &str) -> Option<Vec<Option<Motif>>> {
    match substrate_id {
        "hankin-4-8-8-motifs" => Some(classic_motifs_4_8_8()),
        "hankin-3-12-12" => Some(classic_motifs_3_12_12()),
        _ => None,
    }
}

/// Add `offset_deg` to every class's classic motif theta_deg (one
/// per-substrate offset applied to every class — the documented
/// interpretation of "±{5,10,15}° per-class offsets").
#[cfg(feature = "islamic")]
fn shifted_motifs(motifs: &[Option<Motif>], offset_deg: f64) -> Vec<Option<Motif>> {
    motifs
        .iter()
        .map(|m| {
            m.as_ref().map(|motif| match motif {
                Motif::Star { n, theta_deg, s } => Motif::Star {
                    n: *n,
                    theta_deg: theta_deg + offset_deg,
                    s: *s,
                },
                Motif::Rosette {
                    n,
                    theta_deg,
                    s,
                    h,
                    phi_deg,
                } => Motif::Rosette {
                    n: *n,
                    theta_deg: theta_deg + offset_deg,
                    s: *s,
                    h: *h,
                    phi_deg: *phi_deg,
                },
                Motif::ExtendedStar {
                    n,
                    theta_deg,
                    s,
                } => Motif::ExtendedStar {
                    n: *n,
                    theta_deg: theta_deg + offset_deg,
                    s: *s,
                },
                Motif::ExtendedRosette {
                    n,
                    theta_deg,
                    s,
                    h,
                    phi_deg,
                } => Motif::ExtendedRosette {
                    n: *n,
                    theta_deg: theta_deg + offset_deg,
                    s: *s,
                    h: *h,
                    phi_deg: *phi_deg,
                },
            })
        })
        .collect()
}

#[cfg(feature = "islamic")]
fn motif_spec(m: &Option<Motif>) -> String {
    match m {
        None => "None".to_string(),
        Some(Motif::Star { n, theta_deg, s }) => {
            format!("Star(n={n}, θ={theta_deg:.3}°, s={s})")
        }
        Some(Motif::Rosette { n, theta_deg, s, h, phi_deg }) => format!(
            "Rosette(n={n}, θ={theta_deg:.3}°, s={s}, h={h:.2}, φ={phi_deg:.3}°)"
        ),
        Some(Motif::ExtendedStar { n, theta_deg, s }) => format!(
            "ExtendedStar(n={n}, θ={theta_deg:.3}°, s={s})"
        ),
        Some(Motif::ExtendedRosette { n, theta_deg, s, h, phi_deg }) => format!(
            "ExtendedRosette(n={n}, θ={theta_deg:.3}°, s={s}, h={h:.2}, φ={phi_deg:.3}°)"
        ),
    }
}

/// The per-offset θ row (panic-caught feasibility).
#[cfg(feature = "islamic")]
fn theta_row(substrate_id: &str, offset_deg: f64) -> ThetaRow {
    let Some(tiles) = theta_axis_tiles(substrate_id) else {
        return ThetaRow {
            substrate_id: substrate_id.to_string(),
            theta_offset_deg: offset_deg,
            features: ThetaFeatures {
                feasibility_reason: Some(
                    "θ axis not applicable to this substrate (not a hankin motif target)".to_string(),
                ),
                ..Default::default()
            },
            score: None,
            rank: 0,
        };
    };
    let Some(base_motifs) = classic_motifs(substrate_id) else {
        return ThetaRow {
            substrate_id: substrate_id.to_string(),
            theta_offset_deg: offset_deg,
            features: ThetaFeatures {
                feasibility_reason: Some("no classic motif assignment".to_string()),
                ..Default::default()
            },
            score: None,
            rank: 0,
        };
    };
    let motifs = shifted_motifs(&base_motifs, offset_deg);
    let spec = motifs
        .iter()
        .map(motif_spec)
        .collect::<Vec<_>>()
        .join(" | ");

    let result = std::panic::catch_unwind(|| build_cells_with_motifs(&tiles, &motifs));
    let (cells, report) = match result {
        Ok(ok) => ok,
        Err(e) => {
            let msg = panic_payload_message(&e);
            return ThetaRow {
                substrate_id: substrate_id.to_string(),
                theta_offset_deg: offset_deg,
                features: ThetaFeatures {
                    motifs: spec,
                    feasibility_reason: Some(format!("infeasible: {msg}")),
                    ..Default::default()
                },
                score: None,
                rank: 0,
            };
        }
    };

    let max_partition_error = report
        .per_tile
        .iter()
        .map(|t| (t.area_sum - t.tile_area).abs())
        .fold(0.0_f64, f64::max);
    let n_cells = cells.len();
    let min_face_area = cells
        .iter()
        .map(|c| c.poly.area())
        .fold(f64::INFINITY, f64::min);
    let min_face_area = if min_face_area.is_finite() { min_face_area } else { 0.0 };
    let total_area: f64 = cells.iter().map(|c| c.poly.area()).sum();
    let complete = report.complete;
    let declared = crate::registry()
        .get(substrate_id)
        .and_then(|t| t.declared_symmetry())
        .map(|m| m.to_string())
        .unwrap_or_else(|| "no declared symmetry".to_string());

    let score = documented_theta_score(complete, min_face_area);

    ThetaRow {
        substrate_id: substrate_id.to_string(),
        theta_offset_deg: offset_deg,
        features: ThetaFeatures {
            motifs: spec,
            partition_complete: complete,
            max_partition_error,
            n_cells,
            min_face_area,
            total_area,
            symmetry: declared,
            feasibility_reason: None,
        },
        score: Some(score),
        rank: 0,
    }
}

#[cfg(feature = "islamic")]
fn panic_payload_message(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = payload.downcast_ref::<&'static str>() {
        s.to_string()
    } else {
        "panic (unknown payload)".to_string()
    }
}

#[cfg(feature = "islamic")]
fn rank_theta_rows(rows: Vec<ThetaRow>) -> Vec<ThetaRow> {
    let mut scored: Vec<ThetaRow> = rows.iter().filter(|r| r.score.is_some()).cloned().collect();
    let mut refused: Vec<ThetaRow> = rows.iter().filter(|r| r.score.is_none()).cloned().collect();
    scored.sort_by(|a, b| {
        let sa = a.score.unwrap();
        let sb = b.score.unwrap();
        sa.partial_cmp(&sb)
            .unwrap()
            .then(a.theta_offset_deg.partial_cmp(&b.theta_offset_deg).unwrap())
    });
    for (i, r) in scored.iter_mut().enumerate() {
        r.rank = i + 1;
    }
    for r in refused.iter_mut() {
        r.rank = 0;
    }
    scored.extend(refused);
    scored
}

/// Run the θ axis for one substrate (grid + each offset panic-caught).
/// Returns an empty vec for substrates not on the θ axis.
#[cfg(feature = "islamic")]
pub fn run_theta_axis(substrate_id: &str) -> Vec<ThetaRow> {
    if !THETA_AXIS_IDS.contains(&substrate_id) {
        return Vec::new();
    }
    let mut rows = Vec::with_capacity(THETA_GRID.len());
    for &offset in &THETA_GRID {
        rows.push(theta_row(substrate_id, offset));
    }
    rank_theta_rows(rows)
}

/// Run the sweep over one substrate (both axes).
pub fn run_sweep(tiling: &dyn Tiling, region: &BBox) -> SweepOutcome {
    let substrate_id = tiling.id().to_string();
    let base = SubstrateBase::build(tiling, region);
    let grid = delta_grid(&substrate_id);
    let mut rows = Vec::with_capacity(grid.len());
    for &delta in &grid {
        rows.push(sweep_delta_row(tiling, &base, delta, region));
    }
    let delta_rows = rank_delta_rows(rows);
    #[cfg(feature = "islamic")]
    let theta_rows = run_theta_axis(&substrate_id);
    #[cfg(not(feature = "islamic"))]
    let theta_rows = Vec::new();
    SweepOutcome {
        substrate_id,
        delta_rows,
        theta_rows,
        delta_tiles: base.tiles.clone(),
    }
}

// ---------------------------------------------------------------------------
// Gallery renderers (pure string builders; the CLI writes the files)
// ---------------------------------------------------------------------------

/// Render the δ-axis gallery SVG for one row: try each decoration style
/// (fill → interlace → outline-interlace → outline) over the walls, with
/// an honest gate-honesty fallback to the plain wall rendering.
pub fn delta_gallery_svg(tiles: &[Tile], region: &BBox, delta_deg: f64) -> String {
    let walls = tess_core::strapwork::walls(tiles, delta_deg);
    let region_render = *region;
    let region_pad = region.padding(-1e-9);
    for style in [
        Decoration::Fill,
        Decoration::Interlace,
        Decoration::OutlineInterlace,
        Decoration::Outline,
    ] {
        if let Ok(plan) = plan_decoration(&walls, GALLERY_BAND_W, style, &region_pad) {
            return tess_core::svg::render_svg_decorated(&region_render, &plan);
        }
    }
    tess_core::svg::render_svg_with_walls(tiles, &region_render, ColorMode::Pattern, &walls)
}

/// Render the θ-axis gallery SVG for one row: cells as synthesized tiles
/// over the cell bounding box.
#[cfg(feature = "islamic")]
pub fn theta_gallery_svg(substrate_id: &str, offset_deg: f64) -> Result<String, String> {
    let Some(tiles) = theta_axis_tiles(substrate_id) else {
        return Err("θ axis not applicable to this substrate".to_string());
    };
    let Some(base_motifs) = classic_motifs(substrate_id) else {
        return Err("no classic motif assignment".to_string());
    };
    let motifs = shifted_motifs(&base_motifs, offset_deg);
    let result = std::panic::catch_unwind(|| build_cells_with_motifs(&tiles, &motifs));
    let (cells, _report) = result.map_err(|e| format!("infeasible: {}", panic_payload_message(&e)))?;
    if cells.is_empty() {
        return Err("no cells (empty partition)".to_string());
    }
    let tile_polys: Vec<Tile> = cells
        .iter()
        .map(|c| Tile {
            proto: 0,
            poly: c.poly.clone(),
            seed: c.seed,
        })
        .collect();
    let mut bb = BBox::new(
        Vec2::new(f64::INFINITY, f64::INFINITY),
        Vec2::new(f64::NEG_INFINITY, f64::NEG_INFINITY),
    );
    for t in &tile_polys {
        for p in &t.poly.pts {
            if p.x < bb.min.x {
                bb.min.x = p.x;
            }
            if p.y < bb.min.y {
                bb.min.y = p.y;
            }
            if p.x > bb.max.x {
                bb.max.x = p.x;
            }
            if p.y > bb.max.y {
                bb.max.y = p.y;
            }
        }
    }
    if bb.min.x.is_infinite() {
        return Err("no cells (empty partition)".to_string());
    }
    Ok(tess_core::svg::render_svg(&tile_polys, &bb, ColorMode::Pattern))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fetch the landed square substrate (the registry is cheap to build).
    fn with_square<F: FnOnce(&dyn Tiling)>(f: F) {
        let reg = crate::registry();
        let tiling = reg.get("square").expect("square registered");
        f(tiling)
    }

    fn row_for<'a>(rows: &'a [DeltaRow], delta_deg: f64) -> &'a DeltaRow {
        rows.iter()
            .find(|r| (r.delta_deg - delta_deg).abs() < 1e-6)
            .unwrap_or_else(|| panic!("row for {delta_deg}° exists"))
    }

    #[test]
    fn square_45_is_envelope_member_closed_weave() {
        // Hand-computed spot pin: the square substrate at its canonical
        // 45° closed contact angle (M165 envelope {45°, 90°}) renders as a
        // closed weave — zero exits/corners/bad, an envelope-member row,
        // with a non-empty wall map, and an envelope-member score strictly
        // better than a non-envelope row.
        with_square(|tiling| {
            let rows = run_delta_axis(tiling, &REGION);
            let row = row_for(&rows, 45.0);
            assert_eq!(row.features.envelope_member, Some(true));
            assert!(row.features.envelope_angles.as_ref().unwrap().contains(&45.0));
            assert_eq!(row.features.exit, 0);
            assert_eq!(row.features.corner, 0);
            assert_eq!(row.features.bad, 0);
            assert!(row.features.core_vertices > 0);
            assert!(row.features.strand_count >= 1);
            assert!(row.features.face_count >= 1);
            assert!(row.features.min_face_area > 0.0);
            assert!(row.features.closure_violation.is_none());
            assert!(
                row.features.translation_invariant,
                "lattice translation fold invariance"
            );
            let row25 = row_for(&rows, 25.0);
            assert!(
                row.score.unwrap() < row25.score.unwrap(),
                "envelope member must beat non-envelope: {:?} vs {:?}",
                row.score,
                row25.score
            );
        });
    }

    #[test]
    fn square_90_envelope_point_is_refined_and_member() {
        // 90° is outside the base 5–85° grid: the per-substrate envelope
        // refinement must add it and mark it as an envelope member.
        with_square(|tiling| {
            let rows = run_delta_axis(tiling, &REGION);
            let row = row_for(&rows, 90.0);
            assert_eq!(row.features.envelope_member, Some(true));
            assert_eq!(row.features.exit, 0);
            assert_eq!(row.features.corner, 0);
        });
    }

    #[test]
    fn square_25_is_open_regime_recorded_not_dropped() {
        // 25° is a non-envelope angle: the row must exist, be recorded,
        // and honestly report the open regime (no claim of closure).
        with_square(|tiling| {
            let rows = run_delta_axis(tiling, &REGION);
            let row = row_for(&rows, 25.0);
            assert_eq!(row.features.envelope_member, Some(false));
            let profile = row.features.exit + row.features.corner + row.features.bad;
            assert!(
                profile > 0,
                "25° on the square substrate is an open regime (exit/corner/bad > 0)"
            );
        });
    }

    #[test]
    fn strap_density_is_wall_length_over_region_area() {
        // Independent recomputation of the density feature (1e-9).
        with_square(|tiling| {
            let rows = run_delta_axis(tiling, &REGION);
            let row = row_for(&rows, 45.0);
            let expected = row.features.total_wall_length / bbox_area(&REGION);
            assert!((row.features.strap_density - expected).abs() < 1e-9);
            let expected_crossing_density =
                row.features.crossing_count as f64 / bbox_area(&REGION);
            assert!((row.features.crossing_density - expected_crossing_density).abs() < 1e-9);
        });
    }

    #[test]
    fn crossing_detector_counts_strict_interior_crossings_only() {
        // Hand-built walls: two segments crossing strictly in the interior.
        let walls = vec![
            Wall { a: Vec2::new(-2.0, -2.0), b: Vec2::new(2.0, 2.0) },
            Wall { a: Vec2::new(-2.0, 2.0), b: Vec2::new(2.0, -2.0) },
        ];
        let region = BBox::new(Vec2::new(-3.0, -3.0), Vec2::new(3.0, 3.0));
        let (count, angle) = crossing_features(&walls, &region);
        assert_eq!(count, 1);
        assert!((angle - 90.0).abs() < 1e-9);
        // Endpoints sharing an endpoint: not a strict crossing.
        let shared = vec![
            Wall { a: Vec2::new(0.0, 0.0), b: Vec2::new(2.0, 2.0) },
            Wall { a: Vec2::new(0.0, 0.0), b: Vec2::new(-2.0, 2.0) },
        ];
        let (count2, _) = crossing_features(&shared, &region);
        assert_eq!(count2, 0);
        // Parallel walls: no crossing.
        let parallel = vec![
            Wall { a: Vec2::new(-2.0, 0.0), b: Vec2::new(2.0, 0.0) },
            Wall { a: Vec2::new(-2.0, 1.0), b: Vec2::new(2.0, 1.0) },
        ];
        let (count3, _) = crossing_features(&parallel, &region);
        assert_eq!(count3, 0);
    }

    #[test]
    fn rank_delta_rows_sorts_and_sinks_refused() {
        let rows = vec![
            DeltaRow {
                substrate_id: "square".to_string(),
                delta_deg: 30.0,
                features: SweepFeatures::default(),
                score: Some(5.0),
                rank: 0,
            },
            DeltaRow {
                substrate_id: "square".to_string(),
                delta_deg: 45.0,
                features: SweepFeatures::default(),
                score: Some(2.0),
                rank: 0,
            },
            DeltaRow {
                substrate_id: "square".to_string(),
                delta_deg: 60.0,
                features: SweepFeatures::default(),
                score: None, // refused
                rank: 0,
            },
        ];
        let rows = rank_delta_rows(rows);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].delta_deg, 45.0, "best score first");
        assert_eq!(rows[0].rank, 1);
        assert_eq!(rows[1].delta_deg, 30.0, "second best next");
        assert_eq!(rows[1].rank, 2);
        assert_eq!(rows[2].score, None, "refused sinks");
        assert_eq!(rows[2].rank, 0);
    }

    #[test]
    fn sweep_is_deterministic_across_runs() {
        with_square(|tiling| {
            let a = run_delta_axis(tiling, &REGION);
            let b = run_delta_axis(tiling, &REGION);
            assert_eq!(a.len(), b.len());
            for (ra, rb) in a.iter().zip(b.iter()) {
                assert_eq!(ra.delta_deg, rb.delta_deg);
                assert_eq!(ra.score, rb.score);
                assert_eq!(ra.rank, rb.rank);
                assert_eq!(ra.features.core_vertices, rb.features.core_vertices);
                assert_eq!(ra.features.face_count, rb.features.face_count);
                assert_eq!(ra.features.crossing_count, rb.features.crossing_count);
                assert_eq!(ra.features.translation_invariant, rb.features.translation_invariant);
            }
        });
    }

    #[cfg(feature = "islamic")]
    #[test]
    fn theta_axis_not_applicable_to_square() {
        // The θ axis targets only the two hankin substrates — anything
        // else must return an empty vec (honest "not applicable").
        assert!(run_theta_axis("square").is_empty());
    }

    #[cfg(feature = "islamic")]
    #[test]
    fn theta_axis_4_8_8_refutes_nothing_records_offset_spec() {
        // The M163 classic assignment is known-feasible at offset 0, so
        // all 7 rows (including offsets from ±5, ±10, ±15) must render
        // rows; at least the offset-0 row must be complete.
        let rows = run_theta_axis("hankin-4-8-8-motifs");
        assert_eq!(rows.len(), THETA_GRID.len());
        let row0 = rows.iter().find(|r| (r.theta_offset_deg).abs() < 1e-9).unwrap();
        assert!(row0.features.partition_complete, "offset-0 classic must partition: {:?}", row0.features.feasibility_reason);
        assert!(row0.score.is_some());
        let row_neg = rows.iter().find(|r| (r.theta_offset_deg - (-15.0)).abs() < 1e-9).unwrap();
        assert!(row_neg.features.motifs.contains("7.500°"), "octagon spec: {}", row_neg.features.motifs);
        assert!(row_neg.features.motifs.contains("52.500°"), "diamond spec: {}", row_neg.features.motifs);
    }

    #[cfg(feature = "islamic")]
    #[test]
    fn theta_axis_3_12_12_records_feasibility_honestly() {
        // The canonical assignment (dodecagon Star 45°/s=1, triangles Star
        // 45°/s=1 — pinned by Kaplan's saved `Water Rings.tap`) need not be
        // feasible at every offset; the test does not assume feasibility, it
        // only requires honest accounting: every row exists and carries an
        // explicit feasibility reason or a partition result.
        let rows = run_theta_axis("hankin-3-12-12");
        assert_eq!(rows.len(), THETA_GRID.len());
        for r in &rows {
            assert!(r.features.motifs.contains("Star(n=12"));
            match r.score {
                Some(_) => {
                    assert!(!r.features.partition_complete || r.features.max_partition_error < 1e-6);
                }
                None => assert!(
                    r.features.feasibility_reason.is_some(),
                    "infeasible row carries a reason"
                ),
            }
        }
    }

    #[cfg(feature = "islamic")]
    #[test]
    fn theta_axis_is_deterministic_across_runs() {
        for id in THETA_AXIS_IDS {
            let a = run_theta_axis(id);
            let b = run_theta_axis(id);
            for (ra, rb) in a.iter().zip(b.iter()) {
                assert_eq!(ra.theta_offset_deg, rb.theta_offset_deg);
                assert_eq!(ra.score, rb.score);
                assert_eq!(ra.rank, rb.rank);
                assert_eq!(ra.features.motifs, rb.features.motifs);
                assert_eq!(ra.features.partition_complete, rb.features.partition_complete);
            }
        }
    }

    #[test]
    fn gallery_svg_renders_and_is_deterministic() {
        // Hand-built wall set: the gallery must render SOMETHING (honest
        // fallback) and be byte-identical across runs.
        with_square(|tiling| {
            let params = params_at_default(tiling);
            let result = tiling.generate(&params, &Query { region: REGION });
            let svg_a = delta_gallery_svg(&result.tiles, &REGION, 45.0);
            let svg_b = delta_gallery_svg(&result.tiles, &REGION, 45.0);
            assert!(svg_a.contains("<svg"), "gallery must emit an <svg> document");
            assert!(
                svg_a.contains("<line") || svg_a.contains("<polygon") || svg_a.contains("<path"),
                "gallery must emit wall/fill geometry"
            );
            assert_eq!(svg_a, svg_b, "gallery render must be byte-deterministic");
        });
    }
}
