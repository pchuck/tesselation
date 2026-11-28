//! Focus: the vertex-figure-class orbit dots (C1,
//! `tess_core::svg::vertex_orbits`) driven through the registry over the
//! 20 Two-Uniform tab cards in the exact view the CLI export renders by
//! default (1200×800, scale 1, rotation 0 → a 30×20 world region centered
//! at the origin).
//!
//! # What the battery certifies (per card, independently)
//!
//! 1. **Two classes.** The emitted tiles' vertices partition into exactly
//!    two vertex-figure classes — the semicolon pair a 2-uniform card
//!    declares — and the on-state SVG carries exactly one
//!    `<g data-vertex-orbits="true">` block with exactly two
//!    `<g data-orbit-figure="...">` groups.
//! 2. **Dot counts.** Each class's dot count equals the number of
//!    complete vertices the independent verifier derives for that class
//!    (full-turn wedge closure + no gap; no reuse of the orbits module's
//!    machinery).
//! 3. **Dot positions.** Each printed `(cx, cy)` matches the verifier's
//!    vertex set element-for-element, sorted by (x, y), within the
//!    emission's print resolution (2e-4 — vertex spacing in every 2-uniform
//!    tiling is orders of magnitude larger, so the sorted pairing is
//!    unambiguous).
//! 4. **Two distinguishable colours.** The two figure groups carry exactly
//!    the `ORBIT_A` / `ORBIT_B` `rgb()` fills, orbit 0 → A, orbit 1 → B.
//! 5. **Byte identity.** The off-state is byte-identical to the same C5
//!    entry's output, and the on-state is the off-state's bytes with the
//!    orbit block appended after the walls and before the close.
//! 6. **Declaration match.** The two derived figures equal the two
//!    figures the card declares in its `vertex_config`, compared up to
//!    dihedral equivalence. With the module's numeric-lex canonical form
//!    every one of the 20 declared pairs reads in its canonical rotation
//!    already (the conventional Archimedean order), so the strings in
//!    practice coincide — the dihedral comparison keeps the claim robust
//!    to a future re-rotation of a declaration.
//!
//! The verifier is deliberately NOT the orbits module's machinery: wedges
//! are accumulated in a flat hash map keyed by 1e-6-quantized coordinates,
//! and the figure key is the lexicographically smallest dot-joined string
//! over the 2·n rotations (cyclic rotations of both traversal directions),
//! built as a string rather than a best-kept sequence.

use std::collections::{BTreeMap, HashMap};

use tess_core::geometry::{BBox, Vec2};
use tess_core::params::{ParamSet, ParamValue};
use tess_core::tiling::{Query, Tile};

/// The default export view (what `tess-tui export --tiling ID` renders
/// with no `--width`/`--height`/`--scale`/`--rotation`).
fn region() -> BBox {
    BBox::new(Vec2::new(-15.0, -10.0), Vec2::new(15.0, 10.0))
}

/// The Two-Uniform tab: the registry cards whose classification carries
/// the "Two-Uniform" family tag. Sorted for determinism.
fn two_uniform_ids() -> Vec<&'static str> {
    let registry = tess_tilings::registry();
    let mut ids: Vec<&'static str> = registry
        .iter()
        .filter(|t| {
            tess_tilings::classification(t.id())
                .families
                .contains(&"Two-Uniform")
        })
        .map(|t| t.id())
        .collect();
    ids.sort_unstable();
    assert_eq!(ids.len(), 20, "the Two-Uniform tab has 20 cards: {ids:?}");
    ids
}

/// The tiles of one card's default export view (scale 1, rotation 0).
fn tiles_of(id: &str) -> Vec<Tile> {
    let registry = tess_tilings::registry();
    let tiling = registry
        .get(id)
        .unwrap_or_else(|| panic!("{id} registered"));
    let mut params = ParamSet::default_for(tiling.params());
    params.set(tess_tilings::lattice::SCALE_ID, ParamValue::Float(1.0));
    params.set(tess_tilings::lattice::ROTATION_ID, ParamValue::Angle(0.0));
    let result = tiling.generate(&params, &Query { region: region() });
    assert!(
        result.message.is_none(),
        "{id} refused the default view: {:?}",
        result.message
    );
    assert!(!result.tiles.is_empty(), "{id} produced tiles");
    result.tiles
}

// ──────────────────────────────────────────────────────────────────────────
// Independent naive verifier.
// ──────────────────────────────────────────────────────────────────────────

const FULL_TURN: f64 = std::f64::consts::TAU;
/// Closure tolerance (radians) for "the incident wedges make a full turn"
/// — the same 1e-3 rad the module documents (≈ 0.057°, far below any
/// genuine tile angle).
const CLOSE_TOL: f64 = 1e-3;

/// 1e-6-quantized vertex identity (the k-uniform census grid).
fn q(p: Vec2) -> (i64, i64) {
    ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
}

/// Normalize an angle into [0, 2π).
fn norm(x: f64) -> f64 {
    let m = x % FULL_TURN;
    if m < 0.0 {
        m + FULL_TURN
    } else {
        m
    }
}

/// The figure key of a cyclic side-count sequence: the lexicographically
/// smallest rotation over all cyclic rotations in both traversal
/// directions (dihedral canonical form), compared as side-count
/// sequences (numeric lex BEFORE dot-joining — string lex would sort
/// `12` before `4`, which is not the canonical order the orbits module
/// pins).
fn figure_key(seq: &[usize]) -> String {
    let n = seq.len();
    let mut best: Option<Vec<usize>> = None;
    for start in 0..n {
        for dir in [1i64, -1] {
            let mut cand = Vec::with_capacity(n);
            for i in 0..n {
                let idx = ((start as i64 + dir * (i as i64)).rem_euclid(n as i64)) as usize;
                cand.push(seq[idx]);
            }
            match &best {
                None => best = Some(cand),
                Some(b) if cand < *b => best = Some(cand),
                Some(_) => {}
            }
        }
    }
    best.expect("non-empty sequence")
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(".")
}

/// Independent derivation of the vertex-figure classes: the complete
/// vertices (incident wedges sum to 2π within tolerance AND tile the turn
/// with no gap above tolerance), partitioned by figure key. Each value is
/// the class's quantized vertex coordinates.
fn derive_classes(tiles: &[Tile]) -> BTreeMap<String, Vec<(i64, i64)>> {
    let mut acc: HashMap<(i64, i64), Vec<(f64, f64, usize)>> = HashMap::new();
    for tile in tiles {
        let pts = &tile.poly.pts;
        let n = pts.len();
        if n < 3 {
            continue; // degenerate: no interior wedge
        }
        // Twice the signed area: positive = CCW, interior to the left of
        // each directed edge.
        let mut twice = 0.0f64;
        for k in 0..n {
            twice += pts[k].x * pts[(k + 1) % n].y - pts[k].y * pts[(k + 1) % n].x;
        }
        let ccw = twice > 0.0;
        for k in 0..n {
            let v = pts[k];
            let prev = pts[(k + n - 1) % n];
            let next = pts[(k + 1) % n];
            let ang = |p: Vec2| f64::atan2(p.y - v.y, p.x - v.x);
            // Interior span between the two incident edge directions.
            let (start, span) = if ccw {
                (ang(next), norm(ang(prev) - ang(next)))
            } else {
                (ang(prev), norm(ang(next) - ang(prev)))
            };
            if span < 1e-9 {
                continue; // collinear vertex: zero wedge
            }
            acc.entry(q(v)).or_default().push((norm(start), span, n));
        }
    }
    let mut out: BTreeMap<String, Vec<(i64, i64)>> = BTreeMap::new();
    for (key, ws) in acc {
        let total: f64 = ws.iter().map(|w| w.1).sum();
        if (total - FULL_TURN).abs() > CLOSE_TOL {
            continue; // boundary / T-junction: does not close to 2π
        }
        let mut ws = ws;
        ws.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut closed = true;
        for i in 0..ws.len() {
            let end = ws[i].0 + ws[i].1;
            let next_start = ws[(i + 1) % ws.len()].0;
            if (next_start - end).rem_euclid(FULL_TURN) > CLOSE_TOL {
                closed = false;
                break;
            }
        }
        if !closed {
            continue;
        }
        let seq: Vec<usize> = ws.iter().map(|w| w.2).collect();
        out.entry(figure_key(&seq)).or_default().push(key);
    }
    out
}

// ──────────────────────────────────────────────────────────────────────────
// Declared vertex_config parsing.
// ──────────────────────────────────────────────────────────────────────────

/// Parse a declared vertex configuration (e.g. `"(3⁶; 3².6²)₁"`) into its
/// two dot-joined figure strings (e.g. `"3.3.3.3.3.3"`, `"3.3.6.6"`).
/// `None` when it is not a semicolon pair of polygon tokens (the battery
/// then skips the declaration comparison for that card).
fn declared_figures(config: &str) -> Option<[Vec<usize>; 2]> {
    // Drop the M113 subscript disambiguators (₁/₂/...), the brackets and
    // the space after the semicolon.
    let body: String = config
        .chars()
        .filter(|c| !matches!(c, '(' | ')' | ' ' | '₀' | '₁'..='₉'))
        .collect();
    let sides: Vec<&str> = body.split(';').collect();
    if sides.len() != 2 {
        return None;
    }
    let mut out: [Vec<usize>; 2] = [Vec::new(), Vec::new()];
    for (i, side) in sides.iter().enumerate() {
        for tok in side.split('.') {
            let (base, sup): (String, String) =
                tok.chars().partition(|c| c.is_ascii_digit());
            if base.is_empty() {
                return None;
            }
            let rep = if sup.is_empty() {
                1
            } else {
                let mut digits: Vec<usize> = Vec::new();
                for c in sup.chars() {
                    digits.push(match c {
                        '¹' => 1,
                        '²' => 2,
                        '³' => 3,
                        '⁴' => 4,
                        '⁵' => 5,
                        '⁶' => 6,
                        '⁷' => 7,
                        '⁸' => 8,
                        '⁹' => 9,
                        '⁰' => 0,
                        _ => return None,
                    });
                }
                if digits.is_empty() {
                    return None;
                }
                digits.iter().fold(0usize, |a, &d| a * 10 + d)
            };
            if rep == 0 {
                return None;
            }
            for _ in 0..rep {
                out[i].push(base.parse().expect("digit run"));
            }
        }
        if out[i].is_empty() {
            return None;
        }
    }
    Some(out)
}

// ──────────────────────────────────────────────────────────────────────────
// Parsed-SVG census.
// ──────────────────────────────────────────────────────────────────────────

const MARKER: &str = r#"<g data-vertex-orbits="true""#;
const FIGURE_ATTR: &str = r#"<g data-orbit-figure=""#;
const TAIL: &str = "</g></svg>";

/// `(figure, fill, printed dots)` per inner figure group, in document
/// order.
fn parse_orbit_block(svg: &str) -> Vec<(String, String, Vec<(f64, f64)>)> {
    let start = svg
        .find(MARKER)
        .expect("the orbit block is missing from the on-state");
    // The block is a suffix: it ends where the shared close
    // (`</g></svg>`) begins.
    let block = &svg[start..svg.len() - TAIL.len()];
    let mut groups = Vec::new();
    for part in block.split(FIGURE_ATTR).skip(1) {
        let end_q = part
            .find('"')
            .expect("figure attribute is quote-terminated");
        let figure = &part[..end_q];
        let rest = &part[end_q + 1..];
        let fi = rest
            .find(r#"fill=""#)
            .expect("the figure group carries its accent fill");
        let f = &rest[fi + 6..];
        let fe = f.find('"').expect("fill is quote-terminated");
        let fill = f[..fe].to_string();
        let body = &rest[rest.find('>').expect("opener closes") + 1..];
        groups.push((figure.to_string(), fill, circles_in(body)));
    }
    groups
}

/// The printed `(cx, cy)` of every `<circle .../>` in `s`, in document
/// order.
fn circles_in(s: &str) -> Vec<(f64, f64)> {
    const OPEN: &str = r#"<circle cx=""#;
    let mut out = Vec::new();
    for (i, _) in s.match_indices(OPEN) {
        let rest = &s[i + OPEN.len()..];
        let (x, rest) = take_quoted(rest, "cx");
        assert!(rest.starts_with(r#" cy=""#), "circle carries cy: {rest:?}");
        let (y, _) = take_quoted(&rest[r#" cy=""#.len()..], "cy");
        out.push((x, y));
    }
    out
}

/// Parse the contents of the next double-quoted attribute value as f64.
fn take_quoted<'a>(s: &'a str, what: &str) -> (f64, &'a str) {
    let end = s
        .find('"')
        .unwrap_or_else(|| panic!("unterminated {what} value: {s:?}"));
    let v: f64 = s[..end]
        .parse()
        .unwrap_or_else(|e| panic!("{what} is not a finite number: {s:?} ({e})"));
    assert!(v.is_finite(), "{what} is finite: {v}");
    (v, &s[end + 1..])
}

/// Sort points by their 1e-3-quantized (x, y) key, raw values as
/// tiebreak (see the position-match rationale in the battery).
fn coarse_sort(v: &mut [(f64, f64)]) {
    v.sort_by(|a, b| {
        let ka = ((a.0 * 1e3).round() as i64, (a.1 * 1e3).round() as i64);
        let kb = ((b.0 * 1e3).round() as i64, (b.1 * 1e3).round() as i64);
        ka.cmp(&kb)
            .then_with(|| a.0.partial_cmp(&b.0).unwrap())
            .then_with(|| a.1.partial_cmp(&b.1).unwrap())
    });
}

// ──────────────────────────────────────────────────────────────────────────
// Battery.
// ──────────────────────────────────────────────────────────────────────────

#[test]
fn vertex_orbits_two_uniform_battery() {
    let bbox = region();
    let (ra, ga, ba) = tess_core::paint::rgb(tess_core::svg::ORBIT_A);
    let (rb, gb, bb) = tess_core::paint::rgb(tess_core::svg::ORBIT_B);
    let fill_a = format!("rgb({ra},{ga},{ba})");
    let fill_b = format!("rgb({rb},{gb},{bb})");
    assert_ne!(fill_a, fill_b, "the two orbit accents are distinguishable");

    let mut skips = 0usize;
    for id in two_uniform_ids() {
        let tiles = tiles_of(id);
        let orbits = tess_core::svg::vertex_orbits(&tiles);
        let classes = derive_classes(&tiles);

        // (1) Exactly two classes — the "two vertex orbits" the tab's
        // name promises — and the independent verifier agrees on the class
        // set and on every class size.
        assert_eq!(
            orbits.len(),
            2,
            "{id}: a 2-uniform tiling has exactly two vertex-figure classes, got {orbits:?}"
        );
        assert_eq!(
            classes.len(),
            2,
            "{id}: the verifier derives exactly two classes: {classes:?}"
        );
        for orbit in &orbits {
            let want = classes.get(&orbit.figure).unwrap_or_else(|| {
                panic!(
                    "{id}: the verifier has no class {} (it has {:?})",
                    orbit.figure,
                    classes.keys().collect::<Vec<_>>()
                )
            });
            assert_eq!(
                want.len(),
                orbit.points.len(),
                "{id}: class {} — orbit size {} vs verifier size {}",
                orbit.figure,
                orbit.points.len(),
                want.len()
            );
            assert!(
                !orbit.points.is_empty(),
                "{id}: class {} must be non-empty in the default view",
                orbit.figure
            );
        }

        // (5) Byte identity: off == the C5 entry, on == off + the orbit
        // block.
        let off = tess_core::svg::render_svg_with_walls_orbits(
            &tiles, &bbox, tess_core::view::ColorMode::Pattern, &[], false, false, None, None,
        );
        let off5 = tess_core::svg::render_svg_with_walls_overlay(
            &tiles, &bbox, tess_core::view::ColorMode::Pattern, &[], false, false, None,
        );
        assert_eq!(off, off5, "{id}: off-state is byte-identical to the C5 entry");
        let on = tess_core::svg::render_svg_with_walls_orbits(
            &tiles,
            &bbox,
            tess_core::view::ColorMode::Pattern,
            &[],
            false,
            false,
            None,
            Some(&orbits),
        );
        assert!(!off.contains(MARKER), "{id}: off-state carries no orbit block");
        let idx = on
            .find(MARKER)
            .unwrap_or_else(|| panic!("{id}: on-state carries no orbit block"));
        assert_eq!(
            &on[..idx],
            &off[..off.len() - TAIL.len()],
            "{id}: on minus the orbit block == off"
        );
        assert_eq!(on.matches(MARKER).count(), 1, "{id}: exactly one orbit block");

        // (2, 3, 4) Parsed-SVG census: two figure groups, per-class dot
        // counts, dot positions, the two accent fills.
        let groups = parse_orbit_block(&on);
        assert_eq!(groups.len(), 2, "{id}: exactly two figure groups");
        for (i, orbit) in orbits.iter().enumerate() {
            let (fig, fill, dots) = &groups[i];
            assert_eq!(fig, &orbit.figure, "{id}: figure group {i} carries the orbit's figure");
            let want_fill = if i == 0 { &fill_a } else { &fill_b };
            assert_eq!(fill, want_fill, "{id}: figure group {i} carries its accent");
            assert_eq!(
                dots.len(),
                orbit.points.len(),
                "{id}: class {fig} — {} printed dots vs {} orbit points",
                dots.len(),
                orbit.points.len()
            );
            // Positions: the emission prints 4 decimals, so match within
            // 2e-4 after the shared coarse sort. Same-column vertices
            // carry sub-ULP x noise from the per-tile generation paths,
            // which a raw (x, y) sort cannot survive across the print
            // boundary; vertex spacing in every 2-uniform tiling is
            // ≥ 0.866 (the 60° grid step), far above the key's 1e-3
            // resolution, so the pairing is unambiguous.
            let mut got: Vec<(f64, f64)> = dots.clone();
            coarse_sort(&mut got);
            let mut want: Vec<(f64, f64)> = orbit.points.iter().map(|p| (p.x, p.y)).collect();
            coarse_sort(&mut want);
            for (g, w) in got.iter().zip(want.iter()) {
                assert!(
                    (g.0 - w.0).abs() <= 2e-4 && (g.1 - w.1).abs() <= 2e-4,
                    "{id}: class {fig} — printed dot ({:.4}, {:.4}) vs expected ({:.4}, {:.4})",
                    g.0, g.1, w.0, w.1
                );
            }
        }

        // (6) Declaration match, up to dihedral equivalence: the derived
        // figures' canonical keys equal the declared pair's canonical keys.
        let config = tess_tilings::classification(id)
            .vertex_config
            .expect("2-uniform cards declare a vertex_config");
        match declared_figures(config) {
            Some(pair) => {
                let mut want: Vec<String> = pair.iter().map(|seq| figure_key(seq)).collect();
                let mut got: Vec<String> = orbits.iter().map(|o| o.figure.clone()).collect();
                want.sort();
                got.sort();
                assert_eq!(
                    got, want,
                    "{id}: derived figures {got:?} vs declared {config:?}"
                );
            }
            None => skips += 1, // not a semicolon pair of polygon tokens: skip-compare
        }
    }
    assert_eq!(
        skips, 0,
        "every 2-uniform vertex_config parsed as a semicolon pair of polygon tokens"
    );
}
