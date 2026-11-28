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

/// Dedicated 9-entry ramp for [`ColorMode::Level`] (see `crate::view`):
/// index 0 is the neutral tone for level 0 (non-hierarchical tiles and
/// the level-0 seed plane of fractalizing families); indices 1..=8 are
/// an ordered spectral progression (chartreuse → mint → teal → sky →
/// indigo → purple → magenta → rose) read as "deeper into the
/// hierarchy". Deeper levels cycle the eight steps (index `1 + (level -
/// 1) % 8`), so adjacent levels always land on distinct tones. Engine
/// level ranges measured from the shipped engines: golden bee and
/// golden triangle carry per-tile lineage depths up to `2 * MAX_DEPTH =
/// 34`; the uniform engines peak at their per-view depth caps — 40
/// (chair-family `.min(40)`), 24 (penrose `STEP_CAP`), 16 (stampfli /
/// fractalizing), 20 (Ammann-Beenker / shield), 12 (golden rhomboid),
/// 8 (morphic patches), 6 (fibonacci), 5 (turtle / spectre / GLB).
/// Clamping would collapse everything from 8 on onto one tone, whereas
/// aliasing needs levels a full 8 hierarchy steps (≈ 8 zoom octaves, or
/// 8 scale generations) apart in one view — never adjacent. Distinct
/// from every other ramp; the table is byte-pinned in the tests below.
pub const LEVEL_PALETTE: [u32; 9] = [
    0xB9B7C9, // 0: neutral lavender-gray — non-hierarchical / seed level
    0xCDE86B, // 1: chartreuse — level 1
    0x67D489, // 2: mint — level 2
    0x4FC0A6, // 3: teal — level 3
    0x4AA8D8, // 4: sky — level 4
    0x5B7CE8, // 5: indigo — level 5
    0x8A6FE8, // 6: purple — level 6
    0xB75FC8, // 7: magenta — level 7
    0xD94F8A, // 8: rose — level 8 (and every 8th deeper level)
];

/// Substitution-level class of a tile: 0 stays 0 (neutral,
/// non-hierarchical); levels 1..=8 map to ramp slots 1..=8; deeper
/// levels cycle the 8 hierarchy steps (`1 + (level - 1) % 8`) so
/// adjacent levels are always distinct — see [`LEVEL_PALETTE`] for the
/// clamp-vs-cycle design evidence.
pub fn level_class(tile: &Tile) -> usize {
    if tile.level == 0 {
        0
    } else {
        1 + ((tile.level as usize - 1) % (LEVEL_PALETTE.len() - 1))
    }
}

/// Dedicated 4-entry ramp for [`ColorMode::Orientation`] (see `crate::view`):
/// one tone per dominant-axis bucket — 0: axis-aligned (horizontal),
/// 1: diagonal (45°), 2: axis-aligned (vertical), 3: anti-diagonal
/// (135°). Chosen to be visually distinct from the rainbow `PALETTE`
/// and from the `GRAY`/`PATTERN_GRAY`/`ROLE`/`METATILE` ramps; the
/// table is byte-pinned in the tests below.
pub const ORIENTATION_PALETTE: [u32; 4] = [
    0x3EC1D3, // 0: bright teal — axis-aligned (horizontal)
    0xE95FA6, // 1: hot pink — diagonal (45°)
    0xF2DD5D, // 2: bright yellow — axis-aligned (vertical)
    0xA66BE0, // 3: violet — anti-diagonal (135°)
];

/// Dominant undirected axis of a polygon, in degrees in [0, 180): the
/// angle (undirected, so θ and θ+180 are the same axis) of the direction
/// carrying the most total edge length (degenerate edges skipped).
/// This is a robust choice over the first edge angle — it is invariant
/// to vertex ordering and to collinear mid-side vertices (the 2:1
/// dominoes with split long sides, e.g. `herringbone_45`'s 6-vertex
/// polygons, still read their long-side direction). A direction tie
/// (e.g. a square, no orientation) resolves to the higher angle; an
/// empty/degenerate polygon returns 0.0.
pub fn orientation_axis_deg(poly: &crate::geometry::Polygon) -> f64 {
    let pts = &poly.pts;
    let n = pts.len();
    if n < 2 {
        return 0.0;
    }
    // Total edge length per undirected axis direction, at 0.2°
    // quantization (0.2° matches the resolution at which the bucket
    // quantize below operates).
    let mut totals: std::collections::BTreeMap<i32, f64> = std::collections::BTreeMap::new();
    let mut any = false;
    for i in 0..n {
        let e = pts[(i + 1) % n] - pts[i];
        let len = (e.x * e.x + e.y * e.y).sqrt();
        if len < 1e-12 {
            continue;
        }
        any = true;
        let (dx, dy) = (e.x / len, e.y / len);
        let mut a = dy.atan2(dx);
        // Fold the direction into [0, PI) — the undirected axis.
        if a < 0.0 {
            a += std::f64::consts::PI;
        }
        if a >= std::f64::consts::PI {
            a -= std::f64::consts::PI;
        }
        let key = (a * 1800.0 / std::f64::consts::PI).round() as i32;
        *totals.entry(key).or_insert(0.0) += len;
    }
    if !any {
        return 0.0;
    }
    let (&best_key, _) = totals
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .expect("totals non-empty");
    // key = round(degrees * 10); the documented tie-break is the higher
    // axis (BTreeMap iterates ascending, so max_by's last-maximum wins).
    best_key as f64 / 10.0
}

/// Orientation class of a tile: its dominant-axis angle quantized to
/// 4 buckets at 45° spacing — 0: axis-aligned (horizontal), 1: diagonal
/// (45°), 2: axis-aligned (vertical), 3: anti-diagonal (135°). Distinct
/// from `Metatile`'s aperiodic classification and from `Shape`'s vertex
/// count: this encodes *how* the tile is oriented, not which shape it
/// is. Degenerate polygons (or a direction tie) bucket to 0.
///
/// Note on reptiles: `armchair` (M128) seeds `orientation + 1` in 1..=4 —
/// the same four rotations of one shape. Under this mode the four
/// orientations collapse onto the two axis-aligned buckets (0° and 90°),
/// because 0° and 180° (and 90° and 270°) are the same undirected axis;
/// the mode does not distinguish an armchair at 0° from one at 180°.
/// The seed is still emitted by the generator and `Pattern` mode still
/// distinguishes all four classes.
pub fn orientation_class(poly: &crate::geometry::Polygon) -> usize {
    let deg = orientation_axis_deg(poly);
    ((deg / 45.0).round() as i32).rem_euclid(4) as usize
}

/// Quantizer shared by [`shape_signature`]: round to a 1e-6 grid. Engine
/// noise on emitted coordinates and derived angles stays at the 1e-12
/// relative level (far below one quantum); genuinely different shapes
/// differ by orders of magnitude more, so equal signature ⇔ same shape
/// at tolerance while distinct shapes never merge.
fn shape_quant(v: f64) -> i64 {
    (v * 1.0e6).round() as i64
}

/// Geometric shape signature of a polygon: the canonical representative
/// of its **similarity class** (equal up to translation, rotation,
/// uniform scale, and reflection — orientation and position are not
/// shape; mirror images are the same shape seen from behind).
///
/// Construction (robust at tolerance):
///
/// 1. drop consecutive duplicate vertices (and a repeated closing point);
/// 2. merge collinear mid-side vertices (turn within 1e-9 rad of 0° with
///    a forward dot product) to a fixpoint — the 2:1 dominoes the engines
///    emit both plain (4 vertices) and with split long sides (6 vertices,
///    e.g. `herringbone_45`) share one signature; zero-length edges drop
///    the vertex too;
/// 3. per vertex, the true interior angle (reflex-aware: the signed turn
///    measured against the polygon's winding from the signed area) and
///    per edge the length divided by the maximum post-merge edge length
///    (scale invariance; the maximum is subdivision-invariant, so the
///    normalizer cannot drift when a side happens to carry mid-side
///    vertices — a mean over raw edges would);
/// 4. quantize both to a 1e-6 grid; take the lexicographic minimum over
///    all 2n dihedral traversals (every start vertex, both directions) —
///    cyclic shifts and vertex order cannot change the signature, and
///    reversing the traversal (a reflection) is explicitly enumerated;
/// 5. flatten to `[n, angle_1, len_1, …]`.
///
/// The result is a complete invariant for the vertex loop up to
/// similarity: unlike a sorted angle/edge multiset it preserves which
/// angles are adjacent to which edges. Degenerate polygons (< 3 vertices
/// before or after cleaning, or zero total edge length) return `[0]`,
/// which no real polygon produces (`n ≥ 3` fills the first slot).
///
/// See [`shape_class`] for the palette mapping and the
/// `shape_separation.rs` sweep for the registry-wide certification.
pub fn shape_signature(poly: &crate::geometry::Polygon) -> Vec<i64> {
    // 1. consecutive duplicate / repeated-closing vertices.
    let mut pts: Vec<crate::geometry::Vec2> = Vec::new();
    for &p in &poly.pts {
        if let Some(&last) = pts.last() {
            if (p - last).length() < 1e-12 {
                continue;
            }
        }
        pts.push(p);
    }
    while pts.len() > 1 && (pts[0] - *pts.last().unwrap()).length() < 1e-12 {
        pts.pop();
    }
    if pts.len() < 3 {
        return vec![0];
    }
    let edge_len = |i: usize| -> f64 {
        let n = pts.len();
        (pts[(i + 1) % n] - pts[i]).length()
    };
    let mean = (0..pts.len()).map(edge_len).sum::<f64>() / pts.len() as f64;
    if !(mean > 0.0) {
        return vec![0];
    }
    // 2. merge collinear mid-side vertices (and drop zero-length edges),
    //    to fixpoint.
    loop {
        let n = pts.len();
        if n < 3 {
            return vec![0];
        }
        let mut drop = None;
        for i in 0..n {
            let u = pts[i] - pts[(i + n - 1) % n];
            let v = pts[(i + 1) % n] - pts[i];
            let (lu, lv) = (u.length(), v.length());
            if lu < 1e-9 * mean || lv < 1e-9 * mean {
                drop = Some(i);
                break;
            }
            let cross = u.x * v.y - u.y * v.x;
            let dot = u.x * v.x + u.y * v.y;
            if dot > 0.0 && cross.abs() <= SHAPE_COLLINEAR_TOL * lu * lv {
                drop = Some(i);
                break;
            }
        }
        match drop {
            Some(i) => {
                pts.remove(i);
                if pts.len() < 3 {
                    return vec![0];
                }
            }
            None => break,
        }
    }
    let n = pts.len();
    // 3. winding from the signed area; lengths normalized by the post-
    //    merge maximum (subdivision- and similarity-invariant).
    let mut max_len = 0.0f64;
    for i in 0..n {
        max_len = max_len.max((pts[(i + 1) % n] - pts[i]).length());
    }
    if !(max_len > 0.0) {
        return vec![0];
    }
    let mut sa = 0.0;
    for i in 0..n {
        let p = pts[i];
        let r = pts[(i + 1) % n];
        sa += p.x * r.y - r.x * p.y;
    }
    let s: f64 = if sa >= 0.0 { 1.0 } else { -1.0 };
    let mut angles = vec![0i64; n];
    let mut lens = vec![0i64; n];
    for i in 0..n {
        let u = pts[i] - pts[(i + n - 1) % n];
        let v = pts[(i + 1) % n] - pts[i];
        let turn = (u.x * v.y - u.y * v.x).atan2(u.x * v.x + u.y * v.y);
        angles[i] = shape_quant(std::f64::consts::PI - s * turn);
        lens[i] = shape_quant(v.length() / max_len);
    }
    // 4. dihedral-canonical lexicographic minimum over 2n traversals.
    let mut best: Option<Vec<i64>> = None;
    for j in 0..n {
        let fwd: Vec<i64> = (0..n)
            .flat_map(|k| [angles[(j + k) % n], lens[(j + k) % n]])
            .collect();
        let rev: Vec<i64> = (0..n)
            .flat_map(|k| {
                let v = (j + n - k) % n;
                [angles[v], lens[(v + n - 1) % n]]
            })
            .collect();
        for cand in [fwd, rev] {
            if best.as_ref().is_none_or(|b| cand < *b) {
                best = Some(cand);
            }
        }
    }
    let mut sig = Vec::with_capacity(1 + 2 * n);
    sig.push(n as i64);
    sig.extend(best.unwrap());
    sig
}

/// Collinearity tolerance (radians, cross-product form) for the mid-side
/// vertex merge in [`shape_signature`] — far above engine noise (≈ 1e-12
/// relative), far below any real corner.
const SHAPE_COLLINEAR_TOL: f64 = 1e-9;

/// Position weights (mod 8) folding a [`shape_signature`] into an 8-slot
/// palette index: per canonical position `k`, the low 3 bits of the angle
/// entry weight `SHAPE_SLOT_WEIGHTS[k % 24]` and the next 3 bits weight
/// `SHAPE_SLOT_WEIGHTS[24 + k % 24]`. Derived by constraint solving
/// (walkSAT over the registry's co-occurring shape classes, the
/// `shape_separation.rs` sweep) so every tiling with ≤ 8 classes in its
/// default preview window colors its classes injectively; re-derived if
/// the sweep test ever reports a collision.
const SHAPE_SLOT_WEIGHTS: [u64; 48] = [
    7, 5, 7, 5, 7, 1, 2, 4, 0, 5, 3, 7, 4, 7, 6, 2, 3, 0, 5, 4, 0, 0, 4, 2, 7, 4, 1, 7, 6, 5, 7,
    1, 1, 1, 5, 6, 4, 0, 7, 3, 3, 6, 2, 3, 7, 3, 7, 2,
];

/// Polygon shape class: the palette index of the tile's similarity class
/// ([`shape_signature`]) — geometrically distinct shapes (a square vs a
/// 45° rhombus, a thick vs a thin Penrose rhombus) always land on
/// distinct colors, while the same shape in different positions,
/// orientations, scales, and mirrorings shares one color. The old key
/// (M192) was the raw vertex count, which collapsed exactly such
/// same-count, different-shape tiles onto one color — the user-reported
/// Ammann-Beenker bug (squares and rhombi all blue).
///
/// The signature folds into the 8-slot palette through
/// [`SHAPE_SLOT_WEIGHTS`]. With at most 8 classes in view the mapping is
/// injective — certified registry-wide by the `shape_separation.rs`
/// sweep — and one catalogued tiling (`girih-star-cross`, whose
/// per-instance girih straps realize 536 genuinely distinct similarity
/// classes in the default window) exceeds the palette and shares tones
/// by construction. (A generic hash into 8 slots cannot achieve this:
/// the registry's 332 co-occurrence constraints expect ~40 hash
/// collisions — e.g. plain FNV-1a collided in 45 tilings, with
/// splitmix64 avalanching in 19, `star-cross` among them.)
pub fn shape_class(poly: &crate::geometry::Polygon) -> usize {
    let sig = shape_signature(poly);
    let mut acc: u64 = 0;
    for (k, v) in sig.iter().enumerate() {
        let v = *v as u64;
        acc = acc
            .wrapping_add(SHAPE_SLOT_WEIGHTS[k % 24] * (v & 7))
            .wrapping_add(SHAPE_SLOT_WEIGHTS[24 + k % 24] * ((v >> 3) & 7));
    }
    (acc & 7) as usize
}

/// Aperiodic metatile/cluster class (see `crate::view`).
///
/// The `Tile` contract carries no shape-classification label, so this
/// keys on the tile signature. (Family-tagged tiles never reach a
/// distinguishing read here in practice: the Landing A
/// [`family_slot`] rule overrides this class for tagged tilings in
/// `Metatile` mode.) Signatures (verified at `hat.rs:903-908`,
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
///   — the mode is not family-scoped and untagged tiles carry no family
///   identity (documented limitation).
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

/// Uniform / Two-Uniform polygon-type slot (Landing A palette rule).
///
/// The D6 census over all 31 tagged cards (11 archimedean + 20 kuniform,
/// default preview windows) shows exactly five regular polygon types,
/// keyed by vertex count in [`shape_signature`]: triangle (3), square
/// (4), hexagon (6), octagon (8), dodecagon (12) — union size 5 ≤ 8.
/// Raw [`shape_class`] is NOT injective across that union (octagon and
/// hexagon both fold to slot 3; they merely never co-occur in one
/// view, which is why M215 per-view injectivity holds), so the rule
/// pins five distinct slots and keeps the palette at 8:
/// triangle 7, square 5, hexagon 3, octagon 1, dodecagon 0 — matching
/// shape_class wherever shape_class already reads the type cleanly.
/// Any unexpected shape falls back to `shape_class` (documented
/// collision risk; none observed registry-wide).
fn uniform_polygon_slot(poly: &crate::geometry::Polygon) -> usize {
    let sig = shape_signature(poly);
    match sig.first().copied().unwrap_or(0) {
        3 => 7,
        4 => 5,
        6 => 3,
        8 => 1,
        12 => 0,
        _ => shape_class(poly),
    }
}

/// Penrose family slot: pinned table, the pair `(proto, vertex count)`
/// across the four cards, except that the golden-triangle half-shapes
/// key on SHAPE: they are similarity-identical across `penrose` (rhomb
/// halves) and `penrose_p2` (kite/dart halves) — the acute golden
/// triangle is simultaneously the half-kite and the half-of-thin-rhomb
/// — so proto alone would give one shape two colors. A golden gnomon
/// is recognised by its 108° angle (1884956 in signature units).
///
/// * thick quads (fat rhomb `penrose` p0, kite `penrose_p2` p0) → 0;
/// * thin quads (thin rhomb `penrose` p1, dart `penrose_p2` p1,
///   diamond `penrose-p1` p3 — the diamond IS the thin rhomb, one
///   genuinely shared shape type across cards) → 1;
/// * acute golden triangle (and half-kite) → 2; golden gnomon (and
///   half-dart) → 3;
/// * `penrose-p1`: regular pentagon p0 → 4, decagram star p1 → 5,
///   boat p2 → 6 (with diamond → 1 the card reads {4,5,6,1});
/// * `penrose-pentagon` (5 pentagon types): 0 → 4, 1 → 2, 2 → 5,
///   3 → 5 (types 2/3 are a mirrored pair — identical
///   mirror-invariant signature — so one color), 4 → 6.
///
/// Every card sees pairwise-distinct colors for its distinct shapes.
/// Cross-card color sharing occurs only between DIFFERENT types
/// (e.g. the P1 regular pentagon and pentagon type 1 both read 4),
/// which the one-color-per-type constraint permits.
fn penrose_slot(tile: &Tile) -> usize {
    let sig = shape_signature(&tile.poly);
    let n = sig.first().copied().unwrap_or(0);
    if n == 3 {
        return if sig.contains(&1_884_956) { 3 } else { 2 };
    }
    match (tile.proto, n) {
        (0, 4) => 0,
        (1, 4) | (3, 4) => 1,
        (0, 5) => 4,
        (1, 10) => 5,
        (2, 7) => 6,
        (1, 5) => 2,
        (2, 5) | (3, 5) => 5,
        (4, 5) => 6,
        _ => shape_class(&tile.poly),
    }
}

/// Andritz family slot (bowtie-hexagon cards): proto 0 = bowtie hexagon
/// → 6, proto 1 = regular hexagon → 3 (the regular-hexagon color shared
/// with the Uniform tabs), proto 2 = regular decagon → 2 (a type absent
/// from the Uniform union; slot kept distinct within every card view).
fn andritz_slot(tile: &Tile) -> usize {
    match tile.proto {
        0 => 6,
        1 => 3,
        2 => 2,
        _ => shape_class(&tile.poly),
    }
}

/// The per-family palette rule (Landing A): a fixed identity-mode slot
/// for family-tagged tiles, `None` (leave the mode's own decision)
/// otherwise.
///
/// Overrides ONLY the four identity modes — `Pattern`, `Proto`, `Role`,
/// `Metatile` — and only when `tile.family != FamilyTag::None`;
/// `Single`/`Gray`/`Shape`/`Orientation`/`Level`/`PatternGray` never
/// consult it, and untagged tilings are byte-identical everywhere.
/// `Isohedral`/`Hat`/`Turtle`/`Spectre` carry no rule arm yet (tag is
/// identity-only this landing). All slots are < 8, inside every
/// identity-mode index space (8-entry palettes; `Metatile` 9).
///
/// This is the single source of truth shared by
/// [`tile_rgb`] and raster's `color_index` (M192 sync).
pub fn family_slot(mode: ColorMode, tile: &Tile) -> Option<usize> {
    match mode {
        ColorMode::Pattern | ColorMode::Proto | ColorMode::Role | ColorMode::Metatile => {}
        _ => return None,
    }
    match tile.family {
        crate::tiling::FamilyTag::Uniform | crate::tiling::FamilyTag::TwoUniform => Some(uniform_polygon_slot(&tile.poly)),
        crate::tiling::FamilyTag::Penrose => Some(penrose_slot(tile)),
        crate::tiling::FamilyTag::Andritz => Some(andritz_slot(tile)),
        crate::tiling::FamilyTag::None
        | crate::tiling::FamilyTag::Isohedral
        | crate::tiling::FamilyTag::Hat
        | crate::tiling::FamilyTag::Turtle
        | crate::tiling::FamilyTag::Spectre => None,
    }
}

pub fn tile_rgb(mode: ColorMode, tile: &Tile) -> u32 {
    let idx = match family_slot(mode, tile) {
        Some(s) => s,
        None => match mode {
            ColorMode::Single => 0,
            ColorMode::Pattern => (tile.seed as usize) % PALETTE.len(),
            ColorMode::PatternGray => (tile.seed as usize) % PALETTE.len(),
            ColorMode::Shape => shape_class(&tile.poly),
            ColorMode::Proto => (tile.proto as usize) % PALETTE.len(),
            ColorMode::Gray => (tile.proto as usize) % GRAY_PALETTE.len(),
            ColorMode::Role => (tile.seed as usize) % ROLE_PALETTE.len(),
            ColorMode::Metatile => metatile_class(tile),
            ColorMode::Orientation => orientation_class(&tile.poly),
            ColorMode::Level => level_class(tile),
        },
    };
    match mode {
        ColorMode::Gray => GRAY_PALETTE[idx],
        ColorMode::PatternGray => PATTERN_GRAY_PALETTE[idx],
        ColorMode::Role => ROLE_PALETTE[idx],
        ColorMode::Metatile => METATILE_PALETTE[idx],
        ColorMode::Orientation => ORIENTATION_PALETTE[idx],
        ColorMode::Level => LEVEL_PALETTE[idx],
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
        Tile { family: crate::tiling::FamilyTag::None, reflected: false,
            proto,
            seed,
            level: 0,
            poly: Polygon::from_slice(&[
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(0.0, 1.0),
            ]),
        }
    }

    /// Unit square with an explicit substitution level (level-mode tests).
    fn tile_lv(proto: u8, seed: u32, level: u8) -> Tile {
        Tile { family: crate::tiling::FamilyTag::None, reflected: false, level, ..tile(proto, seed) }
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
        Tile { family: crate::tiling::FamilyTag::None, reflected: false, proto, seed, level: 0, poly: poly_with_n_vertices(n) }
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
    fn shape_signature_distinguishes_regular_polygons_by_geometry() {
        // Under the M-reclassification key every regular n-gon is its own
        // similarity class (the old vertex-count key also separated these;
        // the point of the new key is that same-count different-shape
        // polygons separate too — see the tests below). The palette
        // mapping may alias two classes only when > 8 classes coexist;
        // the class function stays in palette range for all inputs.
        let sigs: Vec<Vec<i64>> = (3..16).map(|n| shape_signature(&poly_with_n_vertices(n))).collect();
        for i in 0..sigs.len() {
            for j in (i + 1)..sigs.len() {
                assert_ne!(sigs[i], sigs[j], "regular {}-gon and {}-gon share a signature", i + 3, j + 3);
            }
            assert!(shape_class(&poly_with_n_vertices(i + 3)) < PALETTE.len());
        }
        // Equal signature ⇔ equal class, always.
        for n in 3..16 {
            let p = poly_with_n_vertices(n);
            assert_eq!(shape_class(&p), shape_class(&p));
        }
    }

    #[test]
    fn shape_signature_degenerate_inputs_share_the_zero_signature() {
        for pts in [
            vec![],
            vec![Vec2::new(1.0, 2.0)],
            vec![Vec2::new(1.0, 2.0), Vec2::new(4.0, 6.0)],
            // Zero-area sliver: all three points identical.
            vec![Vec2::new(0.5, 0.5), Vec2::new(0.5, 0.5), Vec2::new(0.5, 0.5)],
        ] {
            let p = Polygon::from_slice(&pts);
            assert_eq!(shape_signature(&p), vec![0], "degenerate input {pts:?}");
        }
        // Degenerate polygons share one class (not necessarily 0 — the
        // class is the signature's weighted palette fold).
        let a = Polygon::from_slice(&[]);
        let b = Polygon::from_slice(&[Vec2::new(0.0, 0.0)]);
        assert_eq!(shape_class(&a), shape_class(&b));
    }

    /// Rotate `pts` by `deg`, scale by `k`, translate by `(tx, ty)`, and
    /// optionally mirror in x — the similarity group a shape class must
    /// be blind to.
    fn similar_poly(pts: &[[f64; 2]], deg: f64, k: f64, tx: f64, ty: f64, mirror: bool) -> Polygon {
        let a = deg.to_radians();
        let (sa, ca) = a.sin_cos();
        let mut out: Vec<Vec2> = pts
            .iter()
            .map(|p| Vec2::new(p[0] * ca - p[1] * sa, p[0] * sa + p[1] * ca))
            .map(|p| Vec2::new(p.x * k + tx, p.y * k + ty))
            .collect();
        if mirror {
            for p in &mut out {
                p.x = -p.x;
            }
        }
        Polygon::from_slice(&out)
    }

    #[test]
    fn shape_class_is_similarity_invariant() {
        // The unit square through every similarity transform: same class.
        let sq = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let base = shape_signature(&rect_poly(&sq));
        for &(deg, k, tx, ty, mirror) in &[
            (0.0f64, 1.0, 0.0, 0.0, false),
            (37.0, 1.0, 0.0, 0.0, false),
            (180.0, 1.0, 0.0, 0.0, false),
            (0.0, 7.5, 0.0, 0.0, false),
            (123.0, 0.001, 0.0, 0.0, false),
            (0.0, 1.0, 1.0e4, -2.5e4, false),
            (45.0, 1.0, 0.0, 0.0, true),
            (0.0, 1.0, 0.0, 0.0, true),
        ] {
            let p = similar_poly(&sq, deg, k, tx, ty, mirror);
            assert_eq!(shape_signature(&p), base, "similarity transform ({deg},{k},{tx},{ty},mirror={mirror}) changed the square's signature");
            assert_eq!(shape_class(&p), shape_class(&rect_poly(&sq)));
        }
        // Vertex-order invariance: a cyclic shift and a reversal (mirror
        // traversal) of the same vertex list read the same signature.
        let shifted = rect_poly(&[[1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [0.0, 0.0]]);
        assert_eq!(shape_signature(&shifted), base);
        let reversed = rect_poly(&[[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]]);
        assert_eq!(shape_signature(&reversed), base);
    }

    #[test]
    fn shape_class_is_tolerance_robust() {
        // Coordinate jitter far ABOVE engine noise but far BELOW one
        // quantum (1e-6) must not split a class.
        let sq = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let base = shape_signature(&rect_poly(&sq));
        let jitter = |v: f64, i: usize, j: usize| -> f64 {
            // deterministic pseudo-jitter of |δ| ≤ 1e-10
            let d = ((i * 7 + j * 13) % 5) as f64 - 2.0;
            v + d * 5e-11
        };
        for k in 0..3 {
            let mut pts: Vec<Vec2> = Vec::new();
            for (i, p) in sq.iter().enumerate() {
                pts.push(Vec2::new(jitter(p[0], i, k), jitter(p[1], k, i)));
            }
            assert_eq!(shape_signature(&Polygon::from_slice(&pts)), base, "jitter pass {k} split the square class");
        }
    }

    #[test]
    fn shape_class_separates_ammann_beenker_square_and_rhombus() {
        // The user-reported bug: under the old vertex-count key both the
        // AB square and the 45° rhombus were class 1 (4 vertices) and
        // painted the same blue. Their similarity classes differ in
        // angles (90° vs 45°/135°), so the new key must separate them —
        // while every rotated/mirrored copy stays in its own class.
        let square = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        // 45° rhombus: unit edges, corners at 45° and 135°.
        let rhombus = [
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0 + 45f64.to_radians().cos(), 45f64.to_radians().sin()],
            [45f64.to_radians().cos(), 45f64.to_radians().sin()],
        ];
        let s = shape_signature(&rect_poly(&square));
        let r = shape_signature(&rect_poly(&rhombus));
        assert_ne!(s, r, "square and 45° rhombus must not share a shape class");
        assert_ne!(shape_class(&rect_poly(&square)), shape_class(&rect_poly(&rhombus)));
        // Rotated/mirrored copies keep each class (orientation is not shape).
        let rhombus_rot = similar_poly(&rhombus, 72.0, 1.0, 3.0, -4.0, false);
        assert_eq!(shape_signature(&rhombus_rot), r);
        let square_mirror = similar_poly(&square, 0.0, 2.0, 0.0, 0.0, true);
        assert_eq!(shape_signature(&square_mirror), s);
    }

    #[test]
    fn shape_class_separates_penrose_rhombi_but_not_their_orientations() {
        // Thick (72°/108°) and thin (36°/144°) Penrose rhombi are
        // different shapes; the same rhombus at any rotation is one shape.
        let mk = |half: f64| -> Polygon {
            // unit-edge rhombus with interior angles `half` and π−half
            let a = half.to_radians();
            rect_poly(&[
                [0.0, 0.0],
                [1.0, 0.0],
                [1.0 + a.cos(), a.sin()],
                [a.cos(), a.sin()],
            ])
        };
        let thick = mk(72.0);
        let thin = mk(36.0);
        assert_ne!(shape_signature(&thick), shape_signature(&thin), "72° and 36° rhombi must not share a shape class");
        assert_ne!(shape_class(&thick), shape_class(&thin));
        // Two orientations of the thick rhombus (e.g. the P3 rotations):
        // same class.
        let thick_rot = similar_poly(
            &[[0.0, 0.0], [1.0, 0.0], [1.0 + 72f64.to_radians().cos(), 72f64.to_radians().sin()], [72f64.to_radians().cos(), 72f64.to_radians().sin()]],
            144.0,
            1.0,
            0.0,
            0.0,
            false,
        );
        assert_eq!(shape_signature(&thick_rot), shape_signature(&thick));
        // And a square (90° rhombus) is a third class.
        let square = mk(90.0);
        assert_ne!(shape_signature(&square), shape_signature(&thick));
        assert_ne!(shape_signature(&square), shape_signature(&thin));
    }

    #[test]
    fn shape_class_merges_mid_side_split_polygons() {
        // The herringbone precedent: the 2:1 domino emitted plain (4
        // vertices) and with both long sides split at the midpoint (6
        // vertices) is ONE shape. The old key called them classes 1 and 3.
        let plain = [[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]];
        let split = [
            [0.0, 0.0],
            [1.0, 0.0],
            [2.0, 0.0],
            [2.0, 1.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ];
        assert_eq!(shape_signature(&rect_poly(&split)), shape_signature(&rect_poly(&plain)), "mid-side split must not split the class");
        // A rectangle 1×2 and a square are different shapes even though
        // both are 4-gons; 1×2 and 2×1 are the same shape.
        let sq = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        assert_ne!(shape_signature(&rect_poly(&plain)), shape_signature(&rect_poly(&sq)));
        let tall = [[0.0, 0.0], [1.0, 0.0], [1.0, 2.0], [0.0, 2.0]];
        assert_eq!(shape_signature(&rect_poly(&tall)), shape_signature(&rect_poly(&plain)));
    }

    #[test]
    fn shape_class_reflects_reflex_angles() {
        // Reflex-awareness: the unit L-hexagon (one 270° corner) and the
        // equiangular-ish convex counterpart share edge multisets but not
        // canonical (angle, edge) sequences.
        let stair = rect_poly(&[[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [1.0, 1.0], [1.0, 2.0], [0.0, 2.0]]);
        let regular6 = poly_with_n_vertices(6);
        assert_ne!(shape_signature(&stair), shape_signature(&regular6));
        // The stair is one class under rotation/mirroring of itself.
        let stair_m = similar_poly(
            &[[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [1.0, 1.0], [1.0, 2.0], [0.0, 2.0]],
            90.0,
            1.0,
            0.0,
            0.0,
            true,
        );
        assert_eq!(shape_signature(&stair_m), shape_signature(&stair));
    }

    #[test]
    fn tile_rgb_shape_follows_polygon_class() {
        // The palette color is the class's palette-fold slot; distinct
        // shapes get distinct colors and the class must not leak from
        // seed/proto.
        let tri = tile_with_shape(0, 0, 3);
        let dec = tile_with_shape(0, 0, 10);
        assert_eq!(tile_rgb(ColorMode::Shape, &tri), PALETTE[shape_class(&tri.poly)]);
        assert_eq!(tile_rgb(ColorMode::Shape, &dec), PALETTE[shape_class(&dec.poly)]);
        assert_ne!(tile_rgb(ColorMode::Shape, &tri), tile_rgb(ColorMode::Shape, &dec));
        // The shape class must not leak from seed/proto.
        assert_eq!(
            tile_rgb(ColorMode::Shape, &tile_with_shape(7, 3, 6)),
            tile_rgb(ColorMode::Shape, &tile_with_shape(0, 1, 6))
        );
        // A square and a 45° rhombus (the AB bug) read different colors
        // even with identical seed/proto.
        let sq = Tile { family: crate::tiling::FamilyTag::None, reflected: false, proto: 1, seed: 2, level: 0, poly: rect_poly(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]) };
        let rh = Tile { family: crate::tiling::FamilyTag::None, reflected: false, proto: 1, seed: 2, level: 0, poly: rect_poly(&[[0.0, 0.0], [1.0, 0.0], [1.0 + 45f64.to_radians().cos(), 45f64.to_radians().sin()], [45f64.to_radians().cos(), 45f64.to_radians().sin()]]) };
        assert_ne!(tile_rgb(ColorMode::Shape, &sq), tile_rgb(ColorMode::Shape, &rh));
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
            let t = Tile { family: crate::tiling::FamilyTag::None, reflected: false, proto: 0, seed, level: 0, poly: poly_with_n_vertices(3) };
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
            let t = Tile { family: crate::tiling::FamilyTag::None, reflected: false, proto, seed, level: 0, poly: poly_with_n_vertices(3) };
            assert_eq!(metatile_class(&t), expected, "proto {proto}");
        }
    }

    #[test]
    fn metatile_class_falls_back_to_proto() {
        // Non-aperiodic shapes (seed 0, or proto > 3): the proto-class fires.
        for proto in 0u8..9 {
            let t = Tile { family: crate::tiling::FamilyTag::None, reflected: false, proto, seed: 0, level: 0, poly: poly_with_n_vertices(3) };
            assert_eq!(metatile_class(&t), (proto as usize) % 9);
        }
        // Cap-like signature with a seed NOT matching SEED_OF must not be
        // treated as cap — it follows the hat/spectre seed branch instead.
        let t = Tile { family: crate::tiling::FamilyTag::None, reflected: false, proto: 0, seed: 5, level: 0, poly: poly_with_n_vertices(3) };
        assert_eq!(metatile_class(&t), 1); // seed-5 → hat T bucket
    }

    #[test]
    fn tile_rgb_metatile_matches_class() {
        // Cap F (proto=3, seed=1) → class 3 → METATILE_PALETTE[3].
        let cap = Tile { family: crate::tiling::FamilyTag::None, reflected: false, proto: 3, seed: 1, level: 0, poly: poly_with_n_vertices(6) };
        assert_eq!(tile_rgb(ColorMode::Metatile, &cap), METATILE_PALETTE[3]);
        // Hat F1/F2 (proto=0, seed 8/9) → class 3 → same slot.
        let hat = Tile { family: crate::tiling::FamilyTag::None, reflected: false, proto: 0, seed: 8, level: 0, poly: poly_with_n_vertices(6) };
        assert_eq!(tile_rgb(ColorMode::Metatile, &hat), METATILE_PALETTE[3]);
    }

    /// Build a polygon from a point list (a `rect_poly`-ish helper, named
    /// `rect_poly` to avoid clashing with the existing `tile()` helper).
    fn rect_poly(pts: &[[f64; 2]]) -> Polygon {
        Polygon::from_slice(&pts.iter().map(|p| Vec2::new(p[0], p[1])).collect::<Vec<_>>())
    }

    #[test]
    fn orientation_palette_is_pinned_and_distinct() {
        // The pinned 4-entry ramp (one tone per dominant-axis bucket).
        const EXPECTED: [u32; 4] = [0x3EC1D3, 0xE95FA6, 0xF2DD5D, 0xA66BE0];
        assert_eq!(ORIENTATION_PALETTE, EXPECTED);
        // All distinct, and none of them equal to any entry of the
        // other ramps — the orientation table is its own ramp.
        for i in 0..ORIENTATION_PALETTE.len() {
            for j in (i + 1)..ORIENTATION_PALETTE.len() {
                assert_ne!(ORIENTATION_PALETTE[i], ORIENTATION_PALETTE[j], "entries {i} and {j} collide");
            }
            for &other in PALETTE.iter()
                .chain(GRAY_PALETTE.iter())
                .chain(PATTERN_GRAY_PALETTE.iter())
                .chain(ROLE_PALETTE.iter())
                .chain(METATILE_PALETTE.iter())
                .chain(LEVEL_PALETTE.iter())
            {
                assert_ne!(ORIENTATION_PALETTE[i], other, "orientation entry {i} collides with another ramp");
            }
        }
    }

    #[test]
    fn level_palette_is_pinned_and_distinct() {
        // The pinned 9-entry level ramp (neutral + 8 hierarchy steps).
        const EXPECTED: [u32; 9] = [
            0xB9B7C9, 0xCDE86B, 0x67D489, 0x4FC0A6, 0x4AA8D8, 0x5B7CE8, 0x8A6FE8, 0xB75FC8,
            0xD94F8A,
        ];
        assert_eq!(LEVEL_PALETTE, EXPECTED);
        // All distinct, and none of them equal to any entry of the
        // other ramps — the level table is its own ramp.
        for i in 0..LEVEL_PALETTE.len() {
            for j in (i + 1)..LEVEL_PALETTE.len() {
                assert_ne!(LEVEL_PALETTE[i], LEVEL_PALETTE[j], "entries {i} and {j} collide");
            }
            for &other in PALETTE.iter()
                .chain(GRAY_PALETTE.iter())
                .chain(PATTERN_GRAY_PALETTE.iter())
                .chain(ROLE_PALETTE.iter())
                .chain(METATILE_PALETTE.iter())
                .chain(ORIENTATION_PALETTE.iter())
            {
                assert_ne!(LEVEL_PALETTE[i], other, "level entry {i} collides with another ramp");
            }
        }
    }

    #[test]
    fn level_class_maps_and_cycles() {
        // Level 0 is its own neutral class; levels 1..=8 map to ramp
        // slots 1..=8; deeper levels cycle the 8 steps (1 + (l-1) % 8),
        // so adjacent levels are always distinct and the whole ramp is
        // reachable from levels 0..=8 alone.
        for l in 0u8..=8 {
            assert_eq!(level_class(&tile_lv(0, 0, l)) as u32, l as u32, "level {l}");
        }
        for &(l, expected) in &[(9u8, 1usize), (16, 8), (17, 1), (34, 2), (255, 7)] {
            assert_eq!(level_class(&tile_lv(0, 0, l)), expected, "level {l}");
        }
        // Adjacent levels never share a class across the full u8 range.
        for l in 0u8..255 {
            assert_ne!(
                level_class(&tile_lv(0, 0, l)),
                level_class(&tile_lv(0, 0, l + 1)),
                "levels {l} and {} collide",
                l + 1
            );
        }
    }

    #[test]
    fn level_mode_reads_the_level_field_only() {
        // tile_rgb(Level) is the field read straight through the ramp:
        // proto/seed must not leak, and the tone equals LEVEL_PALETTE.
        for l in 0u8..=34 {
            let c = level_class(&tile_lv(3, 7, l));
            assert_eq!(tile_rgb(ColorMode::Level, &tile_lv(3, 7, l)), LEVEL_PALETTE[c], "level {l}");
        }
    }

    #[test]
    fn orientation_axis_and_class_bucket() {
        // Horizontal domino: long sides at 0° → axis ~0°, bucket 0.
        let h = rect_poly(&[[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
        assert_eq!(orientation_class(&h), 0);
        assert!(orientation_axis_deg(&h).abs() < 0.2);
        // Vertical domino: long sides at 90° → axis ~90°, bucket 2.
        let v = rect_poly(&[[0.0, 0.0], [0.0, 2.0], [1.0, 2.0], [1.0, 0.0]]);
        assert_eq!(orientation_class(&v), 2);
        assert!((orientation_axis_deg(&v) - 90.0).abs() < 0.2);
        // 45°-laid domino (slope +1 long sides) → axis ~45°, bucket 1.
        let d = rect_poly(&[[0.0, 0.0], [2.0, 2.0], [1.0, 3.0], [-1.0, 1.0]]);
        assert_eq!(orientation_class(&d), 1);
        assert!((orientation_axis_deg(&d) - 45.0).abs() < 0.2);
        // Anti-diagonal domino (slope -1 long sides) → axis ~135°, bucket 3.
        let ad = rect_poly(&[[0.0, 2.0], [2.0, 0.0], [3.0, 1.0], [1.0, 3.0]]);
        assert_eq!(orientation_class(&ad), 3);
        assert!((orientation_axis_deg(&ad) - 135.0).abs() < 0.2);
        // A square: no orientation — any stable bucket is fine; pin the
        // documented tie-break (higher axis wins; square resolves to
        // 90° → bucket 2) so the behavior is pinned, not incidental.
        let s = rect_poly(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
        assert_eq!(orientation_class(&s), 2);
    }

    #[test]
    fn orientation_is_vertex_order_invariant() {
        // The same polygon, vertex order permuted, reads the same
        // axis/bucket — the derivation is not "first edge angle".
        let a = rect_poly(&[[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
        let b = rect_poly(&[[2.0, 1.0], [0.0, 1.0], [0.0, 0.0], [2.0, 0.0]]);
        assert_eq!(orientation_axis_deg(&a), orientation_axis_deg(&b));
        assert_eq!(orientation_class(&a), orientation_class(&b));
    }

    #[test]
    fn tile_rgb_orientation_follows_axis_not_seed_or_proto() {
        // Same quad polygon at 0° vs 90° → distinct colors.
        let h = rect_poly(&[[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
        let v = rect_poly(&[[0.0, 0.0], [0.0, 2.0], [1.0, 2.0], [1.0, 0.0]]);
        let h_tile = Tile { family: crate::tiling::FamilyTag::None, reflected: false, proto: 0, seed: 0, level: 0, poly: h };
        let v_tile = Tile { family: crate::tiling::FamilyTag::None, reflected: false, proto: 0, seed: 0, level: 0, poly: v };
        assert_eq!(tile_rgb(ColorMode::Orientation, &h_tile), ORIENTATION_PALETTE[0]);
        assert_eq!(tile_rgb(ColorMode::Orientation, &v_tile), ORIENTATION_PALETTE[2]);
        assert_ne!(
            tile_rgb(ColorMode::Orientation, &h_tile),
            tile_rgb(ColorMode::Orientation, &v_tile)
        );
        // seed/proto must not leak into the selection.
        let a = Tile { family: crate::tiling::FamilyTag::None, reflected: false, proto: 0, seed: 3, level: 0, poly: rect_poly(&[[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]) };
        let b = Tile { family: crate::tiling::FamilyTag::None, reflected: false, proto: 7, seed: 1, level: 0, poly: rect_poly(&[[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]) };
        assert_eq!(tile_rgb(ColorMode::Orientation, &a), tile_rgb(ColorMode::Orientation, &b));
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
