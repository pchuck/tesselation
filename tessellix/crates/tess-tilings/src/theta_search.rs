//! The 24e underdetermined-θ search (docs/ROADMAP.md 24e; the 22h
//! "optimization-based motif correction" tool, scoped): a deterministic
//! search over **documented freedom sets**, scored by a **documented
//! aesthetic objective**, with ranked top-k leaderboards. No RNG, no
//! time, no filesystem — the same determinism policy as [`crate::sweep`]
//! (whose δ/θ sweep machinery this module reuses rather than duplicates).
//!
//! # Freedom sets (all documented, none invented)
//!
//! Three axes, each over a freedom set the repo already documents:
//!
//! * **Contact axis** (wall surface, any registered substrate): the
//!   Kaplan/PIC contact angle δ at every substrate-edge midpoint — the
//!   exact freedom swept by M170's δ axis. The grid is [`crate::sweep`]'s
//!   δ grid (5°..85° multiples of 5° plus the M165 envelope refinement
//!   points, e.g. the 90° square/truncated-square envelope points).
//! * **Two-point axis** (cell surface, hankin two-point substrates): the
//!   two-point contact freedom of the landed Hankin engine
//!   (`families/islamic/hankin.rs`): the contact angle θ_e (here swept
//!   30°..60° in 5° steps, absolute engine angle; the landed classic is
//!   45°) together with the two-point split δ as a **fraction of the
//!   class-0 substrate edge length** (0.0..0.5 in 0.1 steps; the landed
//!   classic is 0.5). This is the "contact position along the edge"
//!   freedom: the split places the two contact points at midpoint ∓
//!   (δ/2)·t̂, so δ = 0 is the one-point (midpoint) regime and δ → edge
//!   length walks the contacts to the corners.
//! * **Motif-θ axis** (cell surface, the two M163 motif substrates): the
//!   per-tile-class design-element angle θ (Kaplan §3.5 via
//!   `hankin_motifs.rs`) — the underdetermined θ of the 22h/Bonner
//!   corrections. M170's θ axis swept ONE offset applied to every class
//!   simultaneously; the freedom is actually per-class, so this axis
//!   sweeps the full per-class offset grid {−10°, −5°, 0°, +5°, +10°}
//!   over k classes (5^k rows: 25 for the 2-class `hankin-4-8-8-motifs`,
//!   125 for the 3-class `hankin-3-12-12`), offsets relative to each
//!   class's classic motif angle. The classic assignment (all offsets 0)
//!   is in-grid.
//!
//! Documented freedom-space size of the default check run
//! ([`CONTACT_CHECK_IDS`] + both hankin axes): 2 substrates × 18 contact
//! rows (17 base grid + the 90° envelope point; 45° is already on the
//! base grid) + 2 × 42 two-point rows + 150 motif-θ rows = **270 rows**.
//! The contact axis accepts any registered substrate; its per-substrate
//! row count is `|delta_grid(id)|` (17 base + envelope refinement).
//!
//! # The objective (exact, pinned in tests)
//!
//! Two components, both computed from geometry the search does not
//! control, mirroring [`crate::sweep`]'s documented-score convention
//! (lower is better, refused rows never compete):
//!
//! * **Band balance** B ∈ (−∞, 1] — inspired by Castéra's band-width
//!   balance analysis of classical patterns (his "correct" band widths
//!   are an open derivation in this repo, see docs/ROADMAP.md); the
//!   exact formula is THIS module's definition, pinned here. Every face
//!   f of the pattern gets the uniform-strip width w_f = 2·|A_f|/P_f
//!   (the width of the equal-area, equal-perimeter uniform strip — the
//!   inradius exactly when the face is tangential). Over the **interior**
//!   faces of the region (contact axis: faces whose vertices are all
//!   STRICTLY inside the open region; cell axes: all faces of the
//!   complete fundamental-domain partition), B = 1 − CV({w_f}) with CV
//!   the population coefficient of variation (std/mean). A pattern whose
//!   bands all have the same width — e.g. every centroid-dual pattern at
//!   δ = 90°, whose faces are congruent — scores B = 1.
//! * **Crossing density** D = (weave-crossing nodes inside the region) /
//!   (region area) — the strand-level crossing census of the wall map.
//!   One subtlety is documented here because it is load-bearing: the
//!   strapwork engine (`tess_core::strapwork::walls`) builds each tile's
//!   rays **split at every same-tile crossing** (the registered
//!   partitions ground truth, see the strapwork module docs), so on the
//!   raw piece list every crossing is a shared *endpoint* and a piece-
//!   level strict-segment census is structurally zero. The crossings
//!   live at the strand level: a weave crossing is a wall-map vertex of
//!   degree ≥ 4 whose darts pair **antiparallel** — the strands pass
//!   straight through — which is exactly the `VKind::Joint` count of
//!   [`tess_core::strapwork::validity_stats`], computed by the same
//!   merged-map/dart-pairing pipeline as the fill plan (audit tolerance
//!   |sin| ≤ 1e-4; bend/termination nodes never count). Denser interlace
//!   = larger D. D is defined only on the wall surface; the cell axes
//!   record it as absent (the M170 θ-axis precedent: cells are an exact
//!   face partition, there is no wall map to census).
//!
//! Composite scores (lower is better):
//! * contact axis: `score = 100·(1 − B) + 1/(1 + D)` — the balance term
//!   dominates (a factor-100 decade, the M170 integrity convention); the
//!   density term breaks ties between equally balanced candidates and
//!   mildly rewards richer weaves. Rows whose wall map admits no fill
//!   faces (non-Eulerian map — the open regimes outside the M165
//!   envelope) or fewer than two interior faces have B undefined and are
//!   REFUSED with an explicit reason (rank 0), exactly like infeasible
//!   rows on the M170 θ axis.
//! * cell axes: `score = 100·(1 − B)`; rows whose partition is
//!   infeasible (engine panic) or fails the area identity (|Σ face
//!   areas − Σ tile areas| > 1e-6) are refused with a reason.
//!
//! # Validation
//!
//! The module's falsifiable check (the brief's Bonner leg is book-gated,
//! so in-repo pins substitute): on the contact axis over the square and
//! truncated-square substrates, the truncated-square δ = 90° row must
//! rank FIRST and its fill faces must be the registered `tetrakis-square`
//! Laves tiling itself (tests
//! `contact_axis_tetrakis_square_dual_top_ranked_at_90` and
//! `..._faces_equal_registered_tetrakis_square` — interior face multiset
//! equality against the registered tiling, reproducing the M170 finding
//! "truncated-square δ=90° IS tetrakis-square" through the 24e objective
//! instead of the M170 sliver score). That half of the check HOLDS: the
//! dual row scores perfect balance B = 1 (every face a tetrakis triangle
//! of uniform width exactly 1/2) and wins the balance decade outright —
//! its δ = 45° competitor is also Eulerian and even has the denser weave
//! (912 X-nodes, D = 2.28, vs 145 = 64 octagon centers with four
//! straight-through strand pairs + 81 square centers with one pair,
//! D = 0.3625), but its faces are wildly mixed (octagon-interior cells
//! against diamond corners, B ≈ 0.27675), so 100·(1−B) ≈ 72.3 swamps the
//! density term.
//!
//! The square half of the draft check — "δ = 90° ranks first there too" —
//! was FALSIFIED by the engine, and the falsification is recorded rather
//! than tuned away (test
//! `contact_axis_square_delta45_wins_balance_tie_on_density`): the draft's
//! hand model assumed the δ = 45° square fill leaves mixed per-tile face
//! widths (diamond + four corner triangles per tile), but the wall map is
//! tile-agnostic — the per-tile inscribed diamonds chain colinearly
//! through every shared edge midpoint into two global diagonal line
//! families whose cells are all congruent 45° diamonds (685 fully inside
//! the view: 361 centred on host lattice points + 324 on host tile
//! centers), so B = 1 exactly as at δ = 90° (361 congruent unit squares
//! of width 1/2). With balance tied at 1, the documented density term
//! decides and the denser weave wins: δ = 45° has an X-node at every
//! shared edge midpoint of the generated patch inside the closed region
//! (21·20 + 20·21 = 840, D = 2.1) against the midline grid's 20×20 = 400
//! host-tile-center nodes (D = 1.0); scores 10/31 ≈ 0.3226 vs 0.5. This
//! matches the M170 leaderboard record, which already listed
//! "square δ=90°/45°" together at the top of the grid. The lesson is
//! documented because it is the objective doing its job: band balance
//! alone cannot distinguish the two perfect weaves, and the density
//! tiebreak selects the richer interlace.
//!
//! Crossing-census closed forms pinned by the tests (all re-derived
//! independently in the Python battery,
//! `.scratch/theta-search-24e/battery_theta_24e.py`). The registered
//! hosts generate a one-ring-larger patch than the region (square:
//! 22×22 = 484 unit tiles covering [−11,11]²; truncated-square: 100
//! octagons at ((i+½)A,(j+½)A) + 81 unit squares at (iA,jA), A = 1+√2,
//! covering ±(4.5A + A/2)): square δ=90° → 400 (one node per host tile
//! center inside the region); square δ=45° → 840 (one node per shared
//! edge midpoint of the patch, closed region inclusive:
//! 21·20 + 20·21); truncated-square δ=90° → 145 (64 octagon centers,
//! four strand pairs each, + 81 square centers, one pair each);
//! truncated-square δ=45° → 912 = 512 octagon-interior web crossings
//! (the octagon's eight midpoint chords form two overlapping inscribed
//! squares crossing twice per edge — 8 per octagon for the 64 octagons
//! centered inside the region; the patch-rim octagons' crossings all
//! fall outside) + 144 octagon–octagon axis-edge midpoints (two 90°
//! strands) + 256 octagon–square diagonal-edge midpoints (octagon and
//! square chords are colinear there: two 90° strands). The ts-90 fill
//! faces count derives too: 49 fully-interior tetrakis cells (4
//! triangles each) + 28 rim cells contributing their single inner
//! triangle = 224 — the same count the registered `tetrakis-square`
//! generates, which is what the multiset test asserts.
//!
//! The Bonner documented-corrections validation leg (brief: "validate
//! against Bonner's documented corrections before trusting it") is
//! **BLOCKED, source-gated**: Bonner 2003 figures are raster-only and no
//! text pins the corrections (24g STOPPED 2026-11-25, see
//! docs/ROADMAP.md). In-repo pins (this module's tests + the M170
//! leaderboard findings) are the validation substitute, as for 24g.
//!
//! The 24b fill freedom (the `medallion::infer::Report` freedom sets) is
//! a documented fourth freedom class deliberately OUT of this module's
//! first scope: its candidates are whole jigsaw completions, and scoring
//! them under this objective means re-running the 24b backtracking
//! enumeration per candidate — staged as the follow-up once this
//! objective layer is validated on the hankin freedoms (the brief's
//! "and/or" allows either freedom class to land first).

use tess_core::geometry::{BBox, Vec2};
use tess_core::strapwork::{plan_decoration, validity_stats, walls, Decoration};
use tess_core::tiling::Query;

use crate::sweep::{bbox_area, delta_grid, params_at_default, REGION};

// ---------------------------------------------------------------------------
// Documented search-space constants
// ---------------------------------------------------------------------------

/// The two-substrate falsifiable check space of the brief.
pub const CONTACT_CHECK_IDS: [&str; 2] = ["square", "truncated-square"];

/// Two-point axis substrates (the landed hankin two-point engines).
pub const TWO_POINT_SUBSTRATES: [&str; 2] = [
    "hankin-square-two-point",
    "hankin-hex-two-point",
];

/// Motif-θ axis substrates (the M163 motif-assignment substrates).
pub const MOTIF_THETA_SUBSTRATES: [&str; 2] = ["hankin-4-8-8-motifs", "hankin-3-12-12"];

/// Two-point contact-angle grid (absolute engine angle θ_e, degrees; the
/// landed classic 45° is in-grid).
pub const TWO_POINT_THETA_GRID: [f64; 7] = [30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0];

/// Two-point split grid as a fraction of the class-0 substrate edge
/// length (the landed classic 0.5 is the grid maximum; 0.0 = midpoint
/// contacts).
pub const TWO_POINT_DELTA_GRID: [f64; 6] = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5];

/// Per-class motif-θ offset grid, degrees, relative to each class's
/// classic angle.
pub const MOTIF_OFFSET_GRID: [f64; 5] = [-10.0, -5.0, 0.0, 5.0, 10.0];

/// Balance-decade weight: one point of (1 − B) costs 100 score points,
/// the M170 integrity-decade convention.
pub const BALANCE_WEIGHT: f64 = 100.0;

/// Band width handed to `plan_decoration` (the Fill style's faces do not
/// depend on it; the argument is the engine's signature).
const PLAN_BAND_W: f64 = 0.1;

/// Partition area-identity tolerance for the cell axes.
const AREA_IDENTITY_TOL: f64 = 1e-6;

// ---------------------------------------------------------------------------
// The objective
// ---------------------------------------------------------------------------

fn shoelace_area(pts: &[Vec2]) -> f64 {
    let n = pts.len();
    let mut acc = 0.0;
    for i in 0..n {
        let a = pts[i];
        let b = pts[(i + 1) % n];
        acc += a.x * b.y - b.x * a.y;
    }
    acc * 0.5
}

fn perimeter(pts: &[Vec2]) -> f64 {
    let n = pts.len();
    let mut acc = 0.0;
    for i in 0..n {
        acc += (pts[(i + 1) % n] - pts[i]).length();
    }
    acc
}

/// Uniform-strip width of one face: w = 2·|A|/P (the objective's band
/// width; equals the inradius exactly for tangential faces).
pub fn face_width(pts: &[Vec2]) -> Option<f64> {
    if pts.len() < 3 {
        return None;
    }
    let per = perimeter(pts);
    if per <= 1e-12 {
        return None;
    }
    Some(2.0 * shoelace_area(pts).abs() / per)
}

/// Band balance over the given faces: B = 1 − CV({w_f}), population
/// std/mean of the face widths. `None` = undefined (fewer than two
/// measurable faces — the objective refuses to rank degenerate rows).
pub fn band_balance_of(faces: &[&[Vec2]]) -> Option<f64> {
    let mut widths = Vec::with_capacity(faces.len());
    for f in faces {
        widths.push(face_width(f)?);
    }
    if widths.len() < 2 {
        return None;
    }
    let mean = widths.iter().sum::<f64>() / widths.len() as f64;
    if !(mean > 1e-12) {
        return None;
    }
    let var = widths.iter().map(|w| (w - mean) * (w - mean)).sum::<f64>() / widths.len() as f64;
    Some(1.0 - var.sqrt() / mean)
}

fn strictly_inside(region: &BBox, p: Vec2) -> bool {
    p.x > region.min.x && p.x < region.max.x && p.y > region.min.y && p.y < region.max.y
}

/// Band balance over a wall-map/fill face list inside `region`: only
/// faces whose vertices are all STRICTLY inside the open region count
/// (rim fragments must not skew the balance), then [`band_balance_of`].
pub fn band_balance(faces: &[Vec<Vec2>], region: &BBox) -> Option<f64> {
    let refs: Vec<&[Vec2]> = faces
        .iter()
        .filter(|f| f.iter().all(|&p| strictly_inside(region, p)))
        .map(|f| f.as_slice())
        .collect();
    band_balance_of(&refs)
}

/// Contact-axis composite score: `100·(1 − B) + 1/(1 + D)`, lower is
/// better; `None` (refused) when the balance is undefined.
pub fn documented_contact_score(balance: Option<f64>, crossing_density: f64) -> Option<f64> {
    let b = balance?;
    Some(BALANCE_WEIGHT * (1.0 - b) + 1.0 / (1.0 + crossing_density))
}

/// Cell-axis composite score: `100·(1 − B)`, lower is better.
pub fn documented_cell_score(balance: Option<f64>) -> Option<f64> {
    let b = balance?;
    Some(BALANCE_WEIGHT * (1.0 - b))
}

// ---------------------------------------------------------------------------
// Contact axis (wall surface; any registered substrate)
// ---------------------------------------------------------------------------

/// One ranked contact-axis row (per-substrate leaderboard).
#[derive(Debug, Clone)]
pub struct ContactRow {
    pub substrate_id: String,
    /// Contact angle at the substrate-edge midpoints, degrees.
    pub delta_deg: f64,
    /// Band balance over the interior fill faces (`None` = refused).
    pub balance: Option<f64>,
    /// Interior faces the balance was computed over (0 when refused).
    pub interior_faces: usize,
    /// Total fill faces found by the wall-map face traversal.
    pub face_count: usize,
    /// Strict wall-pair crossings inside the region (the census).
    ///
    /// Strand-level: the number of degree-≥4 antiparallel-crossing
    /// vertices (`validity_stats().joint`) — see the module docs.
    pub crossing_count: usize,
    /// Crossing nodes / region area.
    pub crossing_density: f64,
    /// Documented composite score; `None` = refused (never competes).
    pub score: Option<f64>,
    /// 1-based rank among scored rows of THIS substrate; 0 for refused.
    pub rank: usize,
    /// Honest refusal reason.
    pub feasibility_reason: Option<String>,
}

/// Run the contact axis for one registered substrate over
/// [`crate::sweep`]'s δ grid (base grid + M165 envelope refinement).
pub fn run_contact_axis(substrate_id: &str, region: &BBox) -> Vec<ContactRow> {
    let reg = crate::registry();
    let Some(tiling) = reg.get(substrate_id) else {
        return Vec::new();
    };
    let grid = delta_grid(substrate_id);
    let params = params_at_default(tiling);
    let result = tiling.generate(&params, &Query { region: *region });
    let mut rows = Vec::with_capacity(grid.len());
    if let Some(msg) = result.message {
        for &delta in &grid {
            rows.push(ContactRow {
                substrate_id: substrate_id.to_string(),
                delta_deg: delta,
                balance: None,
                interior_faces: 0,
                face_count: 0,
                crossing_count: 0,
                crossing_density: 0.0,
                score: None,
                rank: 0,
                feasibility_reason: Some(msg.to_string()),
            });
        }
        return rank_contact_rows(rows);
    }
    let tiles = &result.tiles;
    for &delta in &grid {
        let w = walls(tiles, delta);
        // Strand-level crossing census: the merged wall map's
        // antiparallel-crossing (Joint) vertices inside the region — the
        // same pipeline as the fill plan (see the module docs).
        let count = validity_stats(&w, region).joint;
        let density = count as f64 / bbox_area(region);
        let plan = plan_decoration(&w, PLAN_BAND_W, Decoration::Fill, &region.padding(-1e-9));
        let (balance, interior, face_count, reason) = match plan {
            Err(e) => (None, 0usize, 0usize, Some(format!("fill precondition failed: {e}"))),
            Ok(p) => {
                let face_count = p.faces.len();
                let interior = p
                    .faces
                    .iter()
                    .filter(|(f, _)| f.iter().all(|&pt| strictly_inside(region, pt)))
                    .count();
                match band_balance(&faces_of(&p.faces), region) {
                    Some(b) => (Some(b), interior, face_count, None),
                    None => (
                        None,
                        interior,
                        face_count,
                        Some("band balance undefined (fewer than two interior faces)".to_string()),
                    ),
                }
            }
        };
        let score = documented_contact_score(balance, density);
        rows.push(ContactRow {
            substrate_id: substrate_id.to_string(),
            delta_deg: delta,
            balance,
            interior_faces: interior,
            face_count,
            crossing_count: count,
            crossing_density: density,
            score,
            rank: 0,
            feasibility_reason: reason,
        });
    }
    rank_contact_rows(rows)
}

fn faces_of(faces: &[(Vec<Vec2>, u8)]) -> Vec<Vec<Vec2>> {
    faces.iter().map(|(f, _)| f.clone()).collect()
}

/// Rank one substrate's contact rows: scored rows sorted by score
/// ascending (δ ascending as the tie-break) come first, refused rows
/// sink to the tail in grid order with rank 0 (the M170 sort contract).
fn rank_contact_rows(mut rows: Vec<ContactRow>) -> Vec<ContactRow> {
    let mut scored: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter(|(_, r)| r.score.is_some())
        .map(|(i, _)| i)
        .collect();
    scored.sort_by(|&ia, &ib| {
        let a = &rows[ia];
        let b = &rows[ib];
        a.score
            .unwrap()
            .partial_cmp(&b.score.unwrap())
            .unwrap()
            .then(a.delta_deg.partial_cmp(&b.delta_deg).unwrap())
    });
    let ordered: Vec<ContactRow> = scored
        .iter()
        .enumerate()
        .map(|(pos, &i)| {
            let mut r = rows[i].clone();
            r.rank = pos + 1;
            r
        })
        .chain(
            rows.iter()
                .filter(|r| r.score.is_none())
                .cloned(),
        )
        .collect();
    rows = ordered;
    rows
}

// ---------------------------------------------------------------------------
// Two-point axis (cell surface; islamic-gated hankin substrates)
// ---------------------------------------------------------------------------

/// One ranked two-point row.
#[derive(Debug, Clone)]
pub struct TwoPointRow {
    pub substrate_id: String,
    /// Absolute engine contact angle θ_e, degrees.
    pub theta_e_deg: f64,
    /// Two-point split as a fraction of the class-0 edge length.
    pub delta_frac: f64,
    /// Cells (faces) of the fundamental-domain partition.
    pub n_cells: usize,
    /// Smallest cell area (sliver honesty; 0.0 when refused).
    pub min_cell_area: f64,
    /// |Σ cell areas − Σ tile areas| (the partition identity).
    pub max_area_error: f64,
    /// Band balance over all partition cells (`None` = refused).
    pub balance: Option<f64>,
    pub score: Option<f64>,
    pub rank: usize,
    pub feasibility_reason: Option<String>,
}

/// The class-0 edge length of a two-point substrate (the δ fraction's
/// unit; both landed two-point substrates have one class with unit
/// edges — asserted by the caller's engine round-trip).
fn class0_edge_len(tiles: &[&[Vec2]]) -> f64 {
    (tiles[0][1] - tiles[0][0]).length()
}

/// Run the two-point axis for one hankin two-point substrate: the full
/// [`TWO_POINT_THETA_GRID`] × [`TWO_POINT_DELTA_GRID`] freedom grid
/// through the landed `build_cells` engine.
#[cfg(feature = "islamic")]
pub fn run_two_point_axis(substrate_id: &str) -> Vec<TwoPointRow> {
    use crate::families::islamic::hankin::build_cells;
    use crate::sweep::panic_payload_message;

    let (tiles, _classic_theta) = match substrate_id {
        "hankin-square-two-point" => (
            crate::families::islamic::hankin_square_two_point::tiles(),
            crate::families::islamic::hankin_square_two_point::THETA_E_DEG,
        ),
        "hankin-hex-two-point" => (
            crate::families::islamic::hankin_hex::tiles(),
            crate::families::islamic::hankin_hex::THETA_E_DEG,
        ),
        _ => return Vec::new(),
    };
    let edge0 = class0_edge_len(&tiles);
    let tile_area: f64 = tiles.iter().map(|t| shoelace_area(t).abs()).sum();
    let mut rows = Vec::with_capacity(TWO_POINT_THETA_GRID.len() * TWO_POINT_DELTA_GRID.len());
    for &theta in &TWO_POINT_THETA_GRID {
        for &frac in &TWO_POINT_DELTA_GRID {
            let built = std::panic::catch_unwind(|| build_cells(&tiles, theta, frac * edge0));
            let (cells, max_err, reason) = match built {
                Err(payload) => {
                    rows.push(TwoPointRow {
                        substrate_id: substrate_id.to_string(),
                        theta_e_deg: theta,
                        delta_frac: frac,
                        n_cells: 0,
                        min_cell_area: 0.0,
                        max_area_error: 0.0,
                        balance: None,
                        score: None,
                        rank: 0,
                        feasibility_reason: Some(format!(
                            "hankin engine infeasible: {}",
                            panic_payload_message(&payload)
                        )),
                    });
                    continue;
                }
                Ok((cells, report)) => {
                    let err = {
                        let cell_area: f64 =
                            cells.iter().map(|c| shoelace_area(&c.poly.pts).abs()).sum();
                        (cell_area - tile_area).abs()
                    };
                    let reason = if !report.complete {
                        Some("hankin pairing incomplete".to_string())
                    } else if err > AREA_IDENTITY_TOL {
                        Some(format!("partition area identity failed: {err:.3e}"))
                    } else {
                        None
                    };
                    (cells, err, reason)
                }
            };
            let face_refs: Vec<&[Vec2]> = cells.iter().map(|c| c.poly.pts.as_slice()).collect();
            let min_cell_area = cells
                .iter()
                .map(|c| shoelace_area(&c.poly.pts).abs())
                .fold(f64::INFINITY, f64::min);
            let balance = if reason.is_some() {
                None
            } else {
                match band_balance_of(&face_refs) {
                    Some(b) => Some(b),
                    None => {
                        rows.push(TwoPointRow {
                            substrate_id: substrate_id.to_string(),
                            theta_e_deg: theta,
                            delta_frac: frac,
                            n_cells: cells.len(),
                            min_cell_area: if min_cell_area.is_finite() { min_cell_area } else { 0.0 },
                            max_area_error: max_err,
                            balance: None,
                            score: None,
                            rank: 0,
                            feasibility_reason: Some(
                                "band balance undefined (fewer than two cells)".to_string(),
                            ),
                        });
                        continue;
                    }
                }
            };
            let score = documented_cell_score(balance);
            rows.push(TwoPointRow {
                substrate_id: substrate_id.to_string(),
                theta_e_deg: theta,
                delta_frac: frac,
                n_cells: cells.len(),
                min_cell_area: if min_cell_area.is_finite() { min_cell_area } else { 0.0 },
                max_area_error: max_err,
                balance,
                score,
                rank: 0,
                feasibility_reason: reason,
            });
        }
    }
    rank_two_point_rows(rows)
}

#[cfg(feature = "islamic")]
fn rank_two_point_rows(mut rows: Vec<TwoPointRow>) -> Vec<TwoPointRow> {
    let mut scored: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter(|(_, r)| r.score.is_some())
        .map(|(i, _)| i)
        .collect();
    scored.sort_by(|&ia, &ib| {
        let a = &rows[ia];
        let b = &rows[ib];
        a.score
            .unwrap()
            .partial_cmp(&b.score.unwrap())
            .unwrap()
            .then(a.theta_e_deg.partial_cmp(&b.theta_e_deg).unwrap())
            .then(a.delta_frac.partial_cmp(&b.delta_frac).unwrap())
    });
    rows = scored
        .iter()
        .enumerate()
        .map(|(pos, &i)| {
            let mut r = rows[i].clone();
            r.rank = pos + 1;
            r
        })
        .chain(rows.iter().filter(|r| r.score.is_none()).cloned())
        .collect();
    rows
}

// ---------------------------------------------------------------------------
// Motif-θ axis (cell surface; per-class offsets over the M163 substrates)
// ---------------------------------------------------------------------------

/// One ranked motif-θ row: one offset per tile class.
#[derive(Debug, Clone)]
pub struct MotifThetaRow {
    pub substrate_id: String,
    /// Per-class offsets, degrees, in the substrate's class order.
    pub offsets: Vec<f64>,
    /// Per-class motif spec string (the M170 θ-axis format).
    pub motifs: String,
    pub n_cells: usize,
    pub min_cell_area: f64,
    /// |Σ cell areas − Σ tile areas|.
    pub max_area_error: f64,
    pub balance: Option<f64>,
    pub score: Option<f64>,
    pub rank: usize,
    pub feasibility_reason: Option<String>,
}

/// Every per-class offset tuple over [`MOTIF_OFFSET_GRID`] for k classes,
/// class-0 outermost (deterministic odometer order).
fn offset_tuples(k: usize) -> Vec<Vec<f64>> {
    let mut out: Vec<Vec<f64>> = vec![Vec::new()];
    for _ in 0..k {
        let mut next = Vec::with_capacity(out.len() * MOTIF_OFFSET_GRID.len());
        for base in &out {
            for &g in &MOTIF_OFFSET_GRID {
                let mut v = base.clone();
                v.push(g);
                next.push(v);
            }
        }
        out = next;
    }
    out
}

/// Add `off` degrees to one motif's theta_deg (all four variants).
#[cfg(feature = "islamic")]
fn shift_motif(
    m: &crate::families::islamic::hankin_motifs::Motif,
    off: f64,
) -> crate::families::islamic::hankin_motifs::Motif {
    use crate::families::islamic::hankin_motifs::Motif;
    match *m {
        Motif::Star { n, theta_deg, s } => Motif::Star { n, theta_deg: theta_deg + off, s },
        Motif::Rosette { n, theta_deg, s, h, phi_deg } => Motif::Rosette {
            n,
            theta_deg: theta_deg + off,
            s,
            h,
            phi_deg,
        },
        Motif::ExtendedStar { n, theta_deg, s } => {
            Motif::ExtendedStar { n, theta_deg: theta_deg + off, s }
        }
        Motif::ExtendedRosette { n, theta_deg, s, h, phi_deg } => Motif::ExtendedRosette {
            n,
            theta_deg: theta_deg + off,
            s,
            h,
            phi_deg,
        },
    }
}

/// Run the motif-θ axis for one M163 motif substrate: the full per-class
/// offset grid (5^k rows) through the landed `build_cells_with_motifs`
/// engine, offsets relative to each class's classic angle.
#[cfg(feature = "islamic")]
pub fn run_motif_theta_axis(substrate_id: &str) -> Vec<MotifThetaRow> {
    use crate::families::islamic::hankin_motifs::{build_cells_with_motifs, Motif};
    use crate::sweep::panic_payload_message;

    type BaseMotifs = Vec<Option<Motif>>;
    let (tiles, base): (Vec<&'static [Vec2]>, BaseMotifs) = match substrate_id {
        "hankin-4-8-8-motifs" => (
            crate::families::islamic::hankin_4_8_8::tiles(),
            crate::families::islamic::hankin_4_8_8_motifs::motifs(),
        ),
        "hankin-3-12-12" => (
            crate::families::islamic::hankin_3_12_12::tiles(),
            crate::sweep::classic_motifs_3_12_12(),
        ),
        _ => return Vec::new(),
    };
    let k = tiles.len();
    assert_eq!(k, base.len(), "one classic motif per substrate tile class");
    let tile_area: f64 = tiles.iter().map(|t| shoelace_area(t).abs()).sum();
    let mut rows = Vec::with_capacity(offset_tuples(k).len());
    for offs in offset_tuples(k) {
        let motifs: Vec<Option<Motif>> = base
            .iter()
            .zip(&offs)
            .map(|(m, off)| m.as_ref().map(|motif| shift_motif(motif, *off)))
            .collect();
        let spec = motif_spec(&motifs);
        let built = std::panic::catch_unwind(|| build_cells_with_motifs(&tiles, &motifs));
        let (cells, max_err, reason) = match built {
            Err(payload) => {
                rows.push(MotifThetaRow {
                    substrate_id: substrate_id.to_string(),
                    offsets: offs,
                    motifs: spec,
                    n_cells: 0,
                    min_cell_area: 0.0,
                    max_area_error: 0.0,
                    balance: None,
                    score: None,
                    rank: 0,
                    feasibility_reason: Some(format!(
                        "motif engine infeasible: {}",
                        panic_payload_message(&payload)
                    )),
                });
                continue;
            }
            Ok((cells, report)) => {
                let cell_area: f64 = cells.iter().map(|c| shoelace_area(&c.poly.pts).abs()).sum();
                let err = (cell_area - tile_area).abs();
                let reason = if !report.complete {
                    Some("motif pairing incomplete".to_string())
                } else if err > AREA_IDENTITY_TOL {
                    Some(format!("partition area identity failed: {err:.3e}"))
                } else {
                    None
                };
                (cells, err, reason)
            }
        };
        let face_refs: Vec<&[Vec2]> = cells.iter().map(|c| c.poly.pts.as_slice()).collect();
        let min_cell_area = cells
            .iter()
            .map(|c| shoelace_area(&c.poly.pts).abs())
            .fold(f64::INFINITY, f64::min);
        let balance = if reason.is_some() {
            None
        } else {
            match band_balance_of(&face_refs) {
                Some(b) => Some(b),
                None => {
                    rows.push(MotifThetaRow {
                        substrate_id: substrate_id.to_string(),
                        offsets: offs,
                        motifs: spec,
                        n_cells: cells.len(),
                        min_cell_area: if min_cell_area.is_finite() { min_cell_area } else { 0.0 },
                        max_area_error: max_err,
                        balance: None,
                        score: None,
                        rank: 0,
                        feasibility_reason: Some(
                            "band balance undefined (fewer than two cells)".to_string(),
                        ),
                    });
                    continue;
                }
            }
        };
        let score = documented_cell_score(balance);
        rows.push(MotifThetaRow {
            substrate_id: substrate_id.to_string(),
            offsets: offs,
            motifs: spec,
            n_cells: cells.len(),
            min_cell_area: if min_cell_area.is_finite() { min_cell_area } else { 0.0 },
            max_area_error: max_err,
            balance,
            score,
            rank: 0,
            feasibility_reason: reason,
        });
    }
    rank_motif_rows(rows)
}

/// The M170 θ-axis motif spec format, one class per ` | ` separator.
#[cfg(feature = "islamic")]
fn motif_spec(motifs: &[Option<crate::families::islamic::hankin_motifs::Motif>]) -> String {
    use crate::families::islamic::hankin_motifs::Motif;
    motifs
        .iter()
        .map(|m| match m {
            None => "plain".to_string(),
            Some(Motif::Star { n, theta_deg, s }) => {
                format!("Star(n={n}, θ={theta_deg:.3}°, s={s})")
            }
            Some(Motif::Rosette { n, theta_deg, s, h, phi_deg }) => format!(
                "Rosette(n={n}, θ={theta_deg:.3}°, s={s}, h={h:.2}, φ={phi_deg:.3}°)"
            ),
            Some(Motif::ExtendedStar { n, theta_deg, s }) => {
                format!("ExtendedStar(n={n}, θ={theta_deg:.3}°, s={s})")
            }
            Some(Motif::ExtendedRosette { n, theta_deg, s, h, phi_deg }) => format!(
                "ExtendedRosette(n={n}, θ={theta_deg:.3}°, s={s}, h={h:.2}, φ={phi_deg:.3}°)"
            ),
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

#[cfg(feature = "islamic")]
fn rank_motif_rows(mut rows: Vec<MotifThetaRow>) -> Vec<MotifThetaRow> {
    let mut scored: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter(|(_, r)| r.score.is_some())
        .map(|(i, _)| i)
        .collect();
    scored.sort_by(|&ia, &ib| {
        let a = &rows[ia];
        let b = &rows[ib];
        a.score
            .unwrap()
            .partial_cmp(&b.score.unwrap())
            .unwrap()
            .then(a.offsets.partial_cmp(&b.offsets).unwrap())
    });
    rows = scored
        .iter()
        .enumerate()
        .map(|(pos, &i)| {
            let mut r = rows[i].clone();
            r.rank = pos + 1;
            r
        })
        .chain(rows.iter().filter(|r| r.score.is_none()).cloned())
        .collect();
    rows
}

// ---------------------------------------------------------------------------
// Whole search + leaderboard emission
// ---------------------------------------------------------------------------

/// The full documented search outcome: per-substrate ranked rows for
/// every axis (the M170 per-substrate outcome shape).
#[derive(Debug, Clone)]
pub struct SearchOutcome {
    pub contact_rows: Vec<ContactRow>,
    #[cfg(feature = "islamic")]
    pub two_point_rows: Vec<TwoPointRow>,
    #[cfg(feature = "islamic")]
    pub motif_rows: Vec<MotifThetaRow>,
}

/// Run the documented search: the contact axis over `contact_ids` plus
/// both hankin freedom axes over their documented substrates.
pub fn run_search(contact_ids: &[&str], region: &BBox) -> SearchOutcome {
    let mut contact_rows = Vec::new();
    for id in contact_ids {
        contact_rows.extend(run_contact_axis(id, region));
    }
    SearchOutcome {
        contact_rows,
        #[cfg(feature = "islamic")]
        two_point_rows: TWO_POINT_SUBSTRATES
            .iter()
            .flat_map(|id| run_two_point_axis(id))
            .collect(),
        #[cfg(feature = "islamic")]
        motif_rows: MOTIF_THETA_SUBSTRATES
            .iter()
            .flat_map(|id| run_motif_theta_axis(id))
            .collect(),
    }
}

impl SearchOutcome {
    /// Rows that never competed, across all axes.
    pub fn refused_count(&self) -> usize {
        let mut n = self.contact_rows.iter().filter(|r| r.score.is_none()).count();
        #[cfg(feature = "islamic")]
        {
            n += self
                .two_point_rows
                .iter()
                .filter(|r| r.score.is_none())
                .count()
                + self.motif_rows.iter().filter(|r| r.score.is_none()).count();
        }
        n
    }
}

fn json_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn json_f(v: f64) -> String {
    if v.is_finite() {
        format!("{v}")
    } else {
        "null".to_string()
    }
}

fn json_opt(v: Option<f64>) -> String {
    match v {
        Some(x) => json_f(x),
        None => "null".to_string(),
    }
}

fn json_opt_str(v: &Option<String>) -> String {
    match v {
        Some(s) => json_str(s),
        None => "null".to_string(),
    }
}

fn contact_row_json(r: &ContactRow) -> String {
    format!(
        "{{\"axis\": \"contact\", \"substrate_id\": {}, \"delta_deg\": {}, \"rank\": {}, \"score\": {}, \"balance\": {}, \"interior_faces\": {}, \"face_count\": {}, \"crossing_count\": {}, \"crossing_density\": {}, \"feasibility_reason\": {}}}",
        json_str(&r.substrate_id),
        json_f(r.delta_deg),
        r.rank,
        json_opt(r.score),
        json_opt(r.balance),
        r.interior_faces,
        r.face_count,
        r.crossing_count,
        json_f(r.crossing_density),
        json_opt_str(&r.feasibility_reason),
    )
}

#[cfg(feature = "islamic")]
fn two_point_row_json(r: &TwoPointRow) -> String {
    format!(
        "{{\"axis\": \"two_point\", \"substrate_id\": {}, \"theta_e_deg\": {}, \"delta_frac\": {}, \"rank\": {}, \"score\": {}, \"balance\": {}, \"n_cells\": {}, \"min_cell_area\": {}, \"max_area_error\": {}, \"feasibility_reason\": {}}}",
        json_str(&r.substrate_id),
        json_f(r.theta_e_deg),
        json_f(r.delta_frac),
        r.rank,
        json_opt(r.score),
        json_opt(r.balance),
        r.n_cells,
        json_f(r.min_cell_area),
        json_f(r.max_area_error),
        json_opt_str(&r.feasibility_reason),
    )
}

#[cfg(feature = "islamic")]
fn motif_row_json(r: &MotifThetaRow) -> String {
    let offs = r
        .offsets
        .iter()
        .map(|&o| json_f(o))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "{{\"axis\": \"motif_theta\", \"substrate_id\": {}, \"offsets\": [{}], \"rank\": {}, \"score\": {}, \"balance\": {}, \"motifs\": {}, \"n_cells\": {}, \"min_cell_area\": {}, \"max_area_error\": {}, \"feasibility_reason\": {}}}",
        json_str(&r.substrate_id),
        offs,
        r.rank,
        json_opt(r.score),
        json_opt(r.balance),
        json_str(&r.motifs),
        r.n_cells,
        json_f(r.min_cell_area),
        json_f(r.max_area_error),
        json_opt_str(&r.feasibility_reason),
    )
}

/// The leaderboard document (byte-deterministic): the fixed region, the
/// documented freedom space, the sort contract, the score formulas, and
/// every axis's rows — mirroring the `pic-explorer-leaderboard/v1`
/// conventions (per-row feature vectors, refused sink with rank 0, lower
/// score better) under this module's own schema id. Pure string builder;
/// a CLI surface is a follow-up chore.
pub fn leaderboard_json(outcome: &SearchOutcome) -> String {
    let contact: Vec<String> = outcome.contact_rows.iter().map(contact_row_json).collect();
    #[cfg(feature = "islamic")]
    let tp: Vec<String> = outcome.two_point_rows.iter().map(two_point_row_json).collect();
    #[cfg(feature = "islamic")]
    let mo: Vec<String> = outcome.motif_rows.iter().map(motif_row_json).collect();
    #[cfg(not(feature = "islamic"))]
    let tp: Vec<String> = Vec::new();
    #[cfg(not(feature = "islamic"))]
    let mo: Vec<String> = Vec::new();
    let contact_ids = contact_ids_of(&outcome.contact_rows);
    format!(
        "{{\"schema\": \"theta-search-leaderboard/v1\", \"region\": {{\"min\": [{}, {}], \"max\": [{}, {}]}}, \"substrates\": {{\"contact\": [{}], \"two_point\": [{}], \"motif_theta\": [{}]}}, \"sort\": {{\"contact\": \"score asc, delta_deg asc (refused sink, rank 0)\", \"two_point\": \"score asc, theta_e_deg asc, delta_frac asc (refused sink, rank 0)\", \"motif_theta\": \"score asc, offsets lexicographic (refused sink, rank 0)\"}}, \"score_formula_contact\": \"{} * (1 - band_balance) + 1/(1 + crossing_density); band_balance = 1 - CV(2*abs(area)/perimeter over interior fill faces); refused rows (no fill faces / < 2 interior faces) sink with rank 0\", \"score_formula_cells\": \"{} * (1 - band_balance) over the complete fundamental-domain partition; refused rows (engine infeasible / area identity failed) sink with rank 0\", \"freedom_space\": {{\"contact\": \"|delta_grid(substrate)| rows per substrate (17 base + M165 envelope refinement)\", \"two_point\": \"|THETA grid| * |DELTA grid| = 7 * 6 = 42 rows per substrate\", \"motif_theta\": \"5^k rows per substrate (k = tile classes) = 25 + 125\"}}, \"refused_count\": {}, \"contact_rows\": [{}], \"two_point_rows\": [{}], \"motif_theta_rows\": [{}]}}",
        json_f(REGION.min.x),
        json_f(REGION.min.y),
        json_f(REGION.max.x),
        json_f(REGION.max.y),
        contact_ids.join(", "),
        TWO_POINT_SUBSTRATES.iter().map(|s| json_str(s)).collect::<Vec<_>>().join(", "),
        MOTIF_THETA_SUBSTRATES.iter().map(|s| json_str(s)).collect::<Vec<_>>().join(", "),
        BALANCE_WEIGHT,
        BALANCE_WEIGHT,
        outcome.refused_count(),
        contact.join(", "),
        tp.join(", "),
        mo.join(", "),
    )
}

fn contact_ids_of(rows: &[ContactRow]) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for r in rows {
        if !seen.contains(&r.substrate_id) {
            seen.push(r.substrate_id.clone());
        }
    }
    seen.into_iter().map(|s| json_str(&s)).collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use tess_core::tiling::Tile;

    fn registry_tiles(id: &str, region: &BBox) -> Vec<Tile> {
        let reg = crate::registry();
        let tiling = reg.get(id).expect("registered");
        let params = params_at_default(tiling);
        tiling.generate(&params, &Query { region: *region }).tiles
    }

    /// Interior (all vertices strictly inside) polygon list.
    fn interior_polys(polys: &[Vec<Vec2>], region: &BBox) -> Vec<Vec<Vec2>> {
        polys
            .iter()
            .filter(|f| f.iter().all(|&p| strictly_inside(region, p)))
            .cloned()
            .collect()
    }

    #[test]
    fn band_balance_hand_pinned() {
        // Two squares: width 2·1/4 = 0.5 and 2·4/8 = 1.0 →
        // mean 0.75, population std 0.25, CV = 1/3, B = 2/3.
        let sq1 = vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0)];
        let sq2 = vec![Vec2::new(0.0, 0.0), Vec2::new(2.0, 0.0), Vec2::new(2.0, 2.0), Vec2::new(0.0, 2.0)];
        let b = band_balance_of(&[&sq1, &sq2]).unwrap();
        assert!((b - (2.0 / 3.0)).abs() < 1e-12, "B = {b}");
        // Congruent faces → CV = 0 → B = 1 exactly.
        let sq3 = vec![Vec2::new(5.0, 5.0), Vec2::new(6.0, 5.0), Vec2::new(6.0, 6.0), Vec2::new(5.0, 6.0)];
        let b1 = band_balance_of(&[&sq1, &sq3]).unwrap();
        assert!((b1 - 1.0).abs() < 1e-12, "B = {b1}");
        // Fewer than two faces → undefined (refused).
        assert!(band_balance_of(&[&sq1]).is_none());
        // The uniform-strip width equals the inradius for tangential faces:
        // the 0.5-square's width is 0.25.
        let half = vec![Vec2::new(0.0, 0.0), Vec2::new(0.5, 0.0), Vec2::new(0.5, 0.5), Vec2::new(0.0, 0.5)];
        assert!((face_width(&half).unwrap() - 0.25).abs() < 1e-12);
    }

    /// Drop vertices collinear with their neighbours (relative cross
    /// tolerance): canonicalizes fill-face polygons that carry junction
    /// midpoints on otherwise straight sides, so a hexagon-annotated
    /// triangle compares equal to the registered 3-corner triangle.
    fn canonical_pts(pts: &[Vec2]) -> Vec<Vec2> {
        let mut cur: Vec<Vec2> = pts.to_vec();
        for _ in 0..3 {
            let n = cur.len();
            if n < 3 {
                break;
            }
            let mut next: Vec<Vec2> = Vec::with_capacity(n);
            for i in 0..n {
                let (p, q, r) = (cur[(i + n - 1) % n], cur[i], cur[(i + 1) % n]);
                let e1 = q - p;
                let e2 = r - q;
                let scale = e1.length() * e2.length();
                if scale > 0.0 && e1.cross(e2).abs() / scale > 1e-9 {
                    next.push(q);
                }
            }
            if next.len() == cur.len() {
                break;
            }
            cur = next;
        }
        cur
    }

    #[test]
    fn contact_axis_square_delta45_wins_balance_tie_on_density() {
        // Hand-checked sample (independently cross-checked in Python,
        // scratch/theta-search-24e/battery_theta_24e.py). BOTH classic
        // angles close into face tilings on the square host, and BOTH
        // achieve perfect band balance — a fact the hand model in the
        // brief draft disputed (it assumed δ = 45° leaves mixed
        // octagon/triangle face widths per tile) until the strand-merge
        // census settled it: the wall map knows nothing about tile
        // boundaries, and at δ = 45° the per-tile inscribed diamonds
        // chain colinearly through every shared edge midpoint into two
        // global diagonal line families whose cells are ALL congruent
        // 45° diamonds (diagonals 1 × 1, area 1/2, width √2/4):
        // 685 of them lie fully inside the view (361 centred on host
        // lattice points + 324 on host tile centers), so B = 1. At
        // δ = 90° (the midline grid) the faces are 361 congruent unit
        // squares (width 1/2), B = 1 too. With balance tied, the
        // documented density term decides: the δ = 45° weave has an
        // X-node at EVERY shared edge midpoint of the generated patch
        // that lies in the closed region — 21·20 + 20·21 = 840,
        // D = 2.1 — while the midline grid crosses only at the 20×20
        // host tile centers, D = 1.0. Composite: 100·(1−B) ≈ 0 for
        // both, so score(45°) = 1/(1+2.1) = 10/31 ≈ 0.3226 beats
        // score(90°) = 1/2 = 0.5 and the denser weave ranks first.
        let rows = run_contact_axis("square", &REGION);
        assert_eq!(rows.len(), 18, "17 base grid + the 90° envelope point");
        let row45 = rows
            .iter()
            .find(|r| (r.delta_deg - 45.0).abs() < 1e-6)
            .expect("45° row");
        assert_eq!(row45.rank, 1, "the balanced denser weave ranks first");
        assert!(row45.balance.unwrap() > 1.0 - 1e-9, "B = {:?}", row45.balance);
        assert_eq!(row45.crossing_count, 840, "21·20 + 20·21 edge-midpoint X-nodes");
        assert!((row45.crossing_density - 2.1).abs() < 1e-9, "D = {}", row45.crossing_density);
        assert!((row45.score.unwrap() - 10.0 / 31.0).abs() < 1e-9, "score = {:?}", row45.score);
        assert_eq!(row45.face_count, 685, "361 lattice-centred + 324 tile-centred diamonds");
        let row90 = rows
            .iter()
            .find(|r| (r.delta_deg - 90.0).abs() < 1e-6)
            .expect("90° row");
        assert_eq!(row90.rank, 2, "the midline dual ranks second on this host");
        assert!(row90.balance.unwrap() > 1.0 - 1e-9, "B = {:?}", row90.balance);
        assert_eq!(row90.crossing_count, 400, "20×20 host tile centers");
        assert!((row90.crossing_density - 1.0).abs() < 1e-9, "D = {}", row90.crossing_density);
        assert!((row90.score.unwrap() - 0.5).abs() < 1e-9, "score = {:?}", row90.score);
        assert_eq!(row90.face_count, 361, "19×19 unit squares of width 1/2");
        assert!(row45.score.unwrap() < row90.score.unwrap());
        // Generic angles are open regimes: the fill precondition fails →
        // refused with a reason, recorded not dropped (rank 0). Their
        // strand-level census is still recorded (D = 6.1 at δ = 25°,
        // battery-cross-checked).
        let row25 = rows
            .iter()
            .find(|r| (r.delta_deg - 25.0).abs() < 1e-6)
            .expect("25° row");
        assert!(row25.score.is_none());
        assert!(row25
            .feasibility_reason
            .as_deref()
            .unwrap_or_default()
            .contains("fill precondition"));
        assert_eq!(row25.rank, 0);
        assert_eq!(row25.crossing_count, 2440);
        assert!((row25.crossing_density - 6.1).abs() < 1e-9);
        assert_eq!(rows.iter().filter(|r| r.score.is_some()).count(), 2);
    }

    #[test]
    fn contact_axis_tetrakis_square_dual_top_ranked_at_90() {
        // The module's falsifiable check (brief §24e asks the search to be
        // validated against in-repo pins since the Bonner leg is
        // book-gated): on the truncated-square substrate the search must
        // rank the δ = 90° centroid-dual row first. At δ = 90° the wall
        // map is the centroid-dual decomposition: weave-crossing nodes =
        // 64 octagon centers (four straight-through strand pairs each:
        // vertical, horizontal and both diagonal chords) + 81 square
        // centers (one pair: the two diagonal chords) = 145,
        // D = 145/400 = 0.3625; all faces
        // congruent tetrakis triangles (uniform width exactly 1/2,
        // B = 1), score = 1/(1 + 0.3625) = 80/109. The δ = 45° row (the
        // pic-4-8-8 regime) is ALSO Eulerian and scored, but its faces
        // are wildly mixed (octagon-interior cells vs diamond corners):
        // B ≈ 0.27675 → the balance term 100·(1−B) ≈ 72.32 swamps
        // everything (its weave is in fact denser: 912 X-nodes,
        // D = 2.28, so the density term alone would favour it — the
        // balance decade is what the dual wins on, matching the M170
        // leaderboard finding that ts δ=90° heads the grid).
        let rows = run_contact_axis("truncated-square", &REGION);
        assert_eq!(rows.len(), 18);
        let row = rows
            .iter()
            .find(|r| (r.delta_deg - 90.0).abs() < 1e-6)
            .expect("90° row");
        assert_eq!(row.rank, 1, "tetrakis dual must rank first: {:?}", row);
        assert!(row.balance.unwrap() > 1.0 - 1e-9, "B = {:?}", row.balance);
        assert!((row.crossing_density - 145.0 / 400.0).abs() < 1e-9, "D = {}", row.crossing_density);
        assert!((row.score.unwrap() - 80.0 / 109.0).abs() < 1e-9, "score = {:?}", row.score);
        let row45 = rows
            .iter()
            .find(|r| (r.delta_deg - 45.0).abs() < 1e-6)
            .expect("45° row");
        assert_eq!(row45.rank, 2);
        // 912 = 512 octagon-interior web crossings (8 per interior
        // octagon — the two inscribed squares of its midpoints cross
        // twice per edge — 4 per rim, 2 per corner) + 144
        // octagon–octagon axis-edge midpoints (two 45° strands) + 256
        // octagon–square diagonal-edge midpoints (the octagon's and the
        // square's chords are colinear there: two 90° strands).
        assert_eq!(row45.crossing_count, 912);
        assert!((row45.crossing_density - 2.28).abs() < 1e-9, "D = {}", row45.crossing_density);
        assert!((row45.balance.unwrap() - 0.2767532525240787).abs() < 1e-9,
            "B45 = {:?}", row45.balance);
        assert!((row45.score.unwrap() - 72.62955279637262).abs() < 1e-6,
            "score45 = {:?}", row45.score);
        assert!(row45.score.unwrap() > row.score.unwrap());
    }

    #[test]
    fn tetrakis_90_faces_equal_registered_tetrakis_square() {
        // The M170 finding reproduced through the 24e objective: the
        // δ = 90° fill faces of the truncated-square host are the
        // registered `tetrakis-square` tiling — interior multiset
        // equality of (collinear-canonical vertex-average, area, corner
        // count). The fill faces carry junction midpoints on their
        // sides (the wall map's degree-2 pass-through nodes), so the
        // polygons are canonicalized before comparison.
        let region = REGION;
        let tiles = registry_tiles("truncated-square", &region);
        let w = walls(&tiles, 90.0);
        let plan = plan_decoration(&w, PLAN_BAND_W, Decoration::Fill, &region.padding(-1e-9))
            .expect("90° map is fill-decomposable");
        let faces: Vec<Vec<Vec2>> = plan.faces.into_iter().map(|(f, _)| f).collect();
        let got = interior_polys(&faces, &region);

        let tet = registry_tiles("tetrakis-square", &region);
        let want: Vec<Vec<Vec2>> = tet.iter().map(|t| t.poly.pts.clone()).collect();
        let want = interior_polys(&want, &region);

        assert_eq!(got.len(), want.len(), "interior face counts");
        assert!(got.len() > 100, "a real patch, not a degenerate window");

        let key = |f: &Vec<Vec2>| -> (i64, i64, i64, usize) {
            let c = canonical_pts(f);
            let n = c.len() as f64;
            let cx = (c.iter().map(|p| p.x).sum::<f64>() / n * 1e6).round() as i64;
            let cy = (c.iter().map(|p| p.y).sum::<f64>() / n * 1e6).round() as i64;
            let a = (shoelace_area(&c).abs() * 1e6).round() as i64;
            (cx, cy, a, c.len())
        };
        let mut got_map: BTreeMap<(i64, i64, i64, usize), usize> = BTreeMap::new();
        for f in &got {
            *got_map.entry(key(f)).or_insert(0) += 1;
        }
        let mut want_map: BTreeMap<(i64, i64, i64, usize), usize> = BTreeMap::new();
        for f in &want {
            *want_map.entry(key(f)).or_insert(0) += 1;
        }
        assert_eq!(got_map, want_map, "face multiset equality");
        // And the balance identity behind the check: every tetrakis
        // triangle's uniform width is exactly 1/2.
        let widths: Vec<f64> = want.iter().filter_map(|f| face_width(f)).collect();
        assert!((widths.iter().sum::<f64>() / widths.len() as f64 - 0.5).abs() < 1e-9);
    }

    #[cfg(feature = "islamic")]
    #[test]
    fn two_point_axis_sizes_and_classic_pin() {
        // 7 θ_e × 6 δ_frac = 42 rows per substrate, 84 total.
        let sq = run_two_point_axis("hankin-square-two-point");
        assert_eq!(sq.len(), 42);
        let hex = run_two_point_axis("hankin-hex-two-point");
        assert_eq!(hex.len(), 42);
        // The classic (θ_e = 45°, δ = 0.5) row is complete and ranked;
        // its cell count is the landed engine pin (4 triangles + 2
        // pentagons + 1 decagon = 7 faces, the EXPECTED_0 face lens).
        let classic = sq
            .iter()
            .find(|r| (r.theta_e_deg - 45.0).abs() < 1e-9 && (r.delta_frac - 0.5).abs() < 1e-9)
            .expect("classic row");
        assert!(classic.feasibility_reason.is_none(), "{:?}", classic);
        assert_eq!(classic.n_cells, 7);
        assert!(classic.balance.unwrap() < 1.0, "mixed widths at the classic star");
        // Every row is either scored or carries an explicit reason.
        for r in sq.iter().chain(hex.iter()) {
            assert!(r.score.is_some() || r.feasibility_reason.is_some(), "{r:?}");
        }
        // The midpoint regime (δ = 0) is in-grid and feasible at 45°.
        let mid = sq
            .iter()
            .find(|r| (r.theta_e_deg - 45.0).abs() < 1e-9 && r.delta_frac.abs() < 1e-9)
            .expect("midpoint row");
        assert!(mid.score.is_some(), "{mid:?}");
    }

    #[cfg(feature = "islamic")]
    #[test]
    fn motif_theta_axis_sizes_and_classic_pin() {
        // 5² = 25 rows for the 2-class substrate, 5³ = 125 for the
        // 3-class substrate — the documented freedom-space sizes.
        let m48 = run_motif_theta_axis("hankin-4-8-8-motifs");
        assert_eq!(m48.len(), 25);
        let m312 = run_motif_theta_axis("hankin-3-12-12");
        assert_eq!(m312.len(), 125);
        // The classic assignment (offsets all 0) is in-grid and feasible.
        let classic = m48
            .iter()
            .find(|r| r.offsets.iter().all(|&o| o.abs() < 1e-9))
            .expect("classic row");
        assert!(classic.feasibility_reason.is_none(), "{:?}", classic);
        assert!(classic.score.is_some());
        assert!(classic.motifs.contains("Rosette(n=8"));
        // Every row is either scored or carries an explicit reason.
        for r in m48.iter().chain(m312.iter()) {
            assert!(r.score.is_some() || r.feasibility_reason.is_some(), "{r:?}");
        }
    }

    #[test]
    fn leaderboard_json_is_byte_deterministic() {
        // Two full runs over the documented check space → byte-identical
        // leaderboard documents (the M170 two-run byte-identity bar).
        let ids: Vec<&str> = CONTACT_CHECK_IDS.to_vec();
        let a = leaderboard_json(&run_search(&ids, &REGION));
        let b = leaderboard_json(&run_search(&ids, &REGION));
        assert_eq!(a, b, "byte identity across runs");
        assert!(a.contains("\"schema\": \"theta-search-leaderboard/v1\""));
        assert!(a.contains("\"refused_count\": "));
        // The documented freedom space of the default run: 36 contact
        // rows + 84 two-point rows + 150 motif-θ rows = 270.
        let count_axis = |needle: &str| a.matches(&format!("\"axis\": \"{needle}\"")).count();
        assert_eq!(count_axis("contact"), 36);
        #[cfg(feature = "islamic")]
        {
            assert_eq!(count_axis("two_point"), 84);
            assert_eq!(count_axis("motif_theta"), 150);
        }
        // The pinned check rows appear with their ranks.
        assert!(a.contains("\"delta_deg\": 90, \"rank\": 1"));
    }

    #[test]
    fn contact_refusals_are_honest_and_sunk() {
        let rows = run_contact_axis("truncated-square", &REGION);
        let refused: Vec<&ContactRow> = rows.iter().filter(|r| r.score.is_none()).collect();
        assert!(!refused.is_empty(), "generic angles must be refused");
        for r in refused {
            assert_eq!(r.rank, 0);
            assert!(r.feasibility_reason.is_some(), "{r:?}");
        }
        // Refused rows never precede scored rows in the leaderboard.
        let first_refused = rows.iter().position(|r| r.score.is_none());
        let last_scored = rows.iter().rposition(|r| r.score.is_some());
        if let (Some(fr), Some(ls)) = (first_refused, last_scored) {
            assert!(fr > ls, "refused rows must sink");
        }
    }
}