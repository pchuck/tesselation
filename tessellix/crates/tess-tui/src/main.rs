//! tessellix TUI + CLI.
//!
//! Usage:
//!   tess-tui                       run the TUI
//!   tess-tui list                  list available tilings
//!   tess-tui export --tiling ID    write an SVG of the tiling
//!     --width N --height N --out FILE --scale F --rotation DEG --mode MODE
//!     [--center X,Y]               center the view at world (X, Y)
//!                                  (default: origin; finite values)
//!     [--param ID=VALUE ...]       set a tiling parameter (see `list`)
//!     [--strapwork DEG]            overlay the Hankin PIC strapwork walls
//!                                  at contact angle DEG on the tiling
//!     [--deform NAME PARAMS...]    Islamic parquet deformation (diss.
//!                                  §3.4.1): the contact angle varies as a
//!                                  function of point. NAME is one of
//!                                  linear | radial | sine with a fixed
//!                                  parameter list:
//!                                    linear a b x0 x1   ramp along x: angle
//!                                      a at x0 → b at x1
//!                                    radial a b cx cy r0 r1   ramp along
//!                                      distance from centre (cx, cy):
//!                                      a at r0 → b at r1
//!                                    sine mid amp k   oscillation along
//!                                      x: mid + amp·sin(k·x)
//!                                  REPLACES --strapwork (mutually
//!                                  exclusive). Walls are clamped to the
//!                                  substrate's validated δ-envelope
//!                                  (nearest closed angle — see
//!                                  tess-tilings `envelope_for`); params
//!                                  whose range cannot ever intersect the
//!                                  envelope are a named error, not a
//!                                  silent clamp.
//!     [--decoration STYLE]         render a Kaplan §3.7 decoration style
//!                                  over the strapwork walls instead of the
//!                                  tiling itself: fill | outline |
//!                                  interlace | outline-interlace.
//!                                  Requires --strapwork; --mode is
//!                                  ignored (the decoration paints paper
//!                                  + pattern only). Styles whose wall-map
//!                                  preconditions the contact angle does
//!                                  not meet (Kaplan §3.7.1/§3.7.3) fail
//!                                  with an honest error naming the angle
//!                                  regime — classic contact angles close
//!                                  the weave, generic ones do not.
//!     [--band-w W]                 band width in world units for the
//!                                  decoration (default 0.1 at scale 1)
//!   tess-tui explore --tiling ID  deterministic δ/θ sweep + leaderboard
//!     (docs/briefs/pic-explorer.md). Emits a JSON leaderboard plus an
//!     optional CSV and a top-K SVG gallery.
//!     [--out FILE] [--csv FILE] [--gallery DIR] [--top-k N]
//!     (default: prints the JSON leaderboard to stdout; --top-k defaults
//!     to 10)
//!

mod app_ui;
mod raster;
mod thumb;

use tess_core::geometry::Vec2;
use tess_core::params::{ParamKind, ParamValue};
use tess_core::strapwork::{BuiltInProfile, Decoration, snap_to_envelope};
use tess_core::view::ColorMode;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result: Result<(), String> = match args.first().map(String::as_str) {
        None => app_ui::run().map_err(|e| e.to_string()),
        Some("list") => list(),
        Some("export") => export(&args[1..]),
        Some("explore") => explore(&args[1..]),
        Some(other) => Err(format!(
            "unknown command: {other}\nusage: tess-tui [list | export ... | explore ...]"
        )),
    };
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn list() -> Result<(), String> {
    let registry = tess_tilings::registry();
    let tilings: Vec<&dyn tess_core::Tiling> = registry.iter().collect();
    let idw = tilings.iter().map(|t| t.id().len()).max().unwrap_or(0);
    for t in &tilings {
        println!("{:<idw$} {:<18} {}", t.id(), t.name(), t.description());
    }
    Ok(())
}

#[derive(Debug)]
struct ExportArgs {
    tiling: String,
    width: u32,
    height: u32,
    out: String,
    scale: f64,
    rotation: f64,
    mode: ColorMode,
    params: Vec<(String, String)>,
    strapwork: Option<f64>,
    deform: Option<BuiltInProfile>,
    decoration: Option<Decoration>,
    band_w: f64,
    center: Option<(f64, f64)>,
}

/// Parse a float CLI value, rejecting NaN and infinities: they would
/// otherwise flow into the generators and silently produce empty output.
fn finite(v: &str, what: &str) -> Result<f64, String> {
    let f: f64 = v.parse().map_err(|_| format!("bad {what}: {v}"))?;
    if !f.is_finite() {
        return Err(format!("{what} must be a finite number: {v}"));
    }
    Ok(f)
}

/// Fixed arity + per-parameter names for `--deform` profiles (diss. §3.4.1).
fn deform_arity(name: &str) -> Option<(usize, &'static [&'static str])> {
    match name {
        "linear" => Some((4, &["a", "b", "x0", "x1"])),
        "radial" => Some((6, &["a", "b", "cx", "cy", "r0", "r1"])),
        "sine" => Some((3, &["mid", "amp", "k"])),
        _ => None,
    }
}

/// Parse the `--deform NAME ...` arm: the profile name (already consumed)
/// and its fixed numeric parameter run (scanned up to the next flag token).
/// Errors are honest named errors — never a silent clamp at parse time.
fn parse_deform(name: &str, args: &[String], i: &mut usize) -> Result<BuiltInProfile, String> {
    let (arity, names) = deform_arity(name).ok_or_else(|| {
        format!("unknown --deform profile '{name}': expected linear | radial | sine")
    })?;
    let mut params: Vec<&str> = Vec::new();
    while *i < args.len() && !args[*i].starts_with("--") {
        params.push(args[*i].as_str());
        *i += 1;
    }
    if params.len() != arity {
        return Err(format!(
            "--deform {name}: expected {arity} numeric params ({}), got {}",
            names.join(" "),
            params.len()
        ));
    }
    let mut vals = Vec::with_capacity(arity);
    for (raw, pname) in params.iter().zip(names.iter()) {
        let v: f64 = raw
            .parse()
            .map_err(|_| format!("--deform {name}: bad float '{raw}' (expected {pname})"))?;
        if !v.is_finite() {
            return Err(format!("--deform {name}: {pname} must be a finite float: '{raw}'"));
        }
        vals.push(v);
    }
    match name {
        "linear" => {
            let (a, b, x0, x1) = (vals[0], vals[1], vals[2], vals[3]);
            if x0 == x1 {
                return Err("--deform linear: x0 and x1 must differ (the ramp span)".into());
            }
            Ok(BuiltInProfile::Linear { a, b, x0, x1 })
        }
        "radial" => {
            let (a, b, cx, cy, r0, r1) = (vals[0], vals[1], vals[2], vals[3], vals[4], vals[5]);
            if r0 == r1 {
                return Err("--deform radial: r0 and r1 must differ (the ramp span)".into());
            }
            Ok(BuiltInProfile::Radial {
                a,
                b,
                c: Vec2::new(cx, cy),
                r0,
                r1,
            })
        }
        "sine" => {
            let (mid, amp, k) = (vals[0], vals[1], vals[2]);
            Ok(BuiltInProfile::Sine { mid, amp, k })
        }
        _ => unreachable!("deform_arity already rejected '{name}'"),
    }
}

/// Resolve the parsed deformation profile against the substrate's Phase-A
/// envelope (`tess_tilings::envelope_for`) — the documented clamp policy
/// (diss. §3.4.1):
///
/// * no validated envelope for the substrate → named error (never a guess);
/// * profile nominal range entirely disjoint from the envelope → named
///   error "params outside envelope" — no silent clamp;
/// * otherwise → a closure snapping every point to the nearest closed
///   angle (clamping to the envelope is the correct behaviour).
///
/// Pure: the envelope is passed in so the error policy is unit-testable.
fn resolve_deform<'a>(
    profile: BuiltInProfile,
    envelope: Option<&'a [f64]>,
    substrate_id: &str,
) -> Result<impl Fn(Vec2) -> f64 + 'a, String> {
    let envelope = envelope.ok_or_else(|| {
        format!(
            "no validated δ-envelope for substrate '{substrate_id}' — not a strapwork engine substrate"
        )
    })?;
    let (lo, hi) = profile.nominal_range();
    if !envelope.iter().any(|&e| e >= lo && e <= hi) {
        return Err(format!(
            "--deform profile range {lo:.3}..{hi:.3} is outside the validated δ-envelope [{}] for substrate '{substrate_id}' — params outside envelope",
            envelope
                .iter()
                .map(|e| format!("{e:.1}°"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    Ok(move |q: Vec2| {
        snap_to_envelope(profile.delta(q), envelope)
            .expect("envelope is non-empty and profile parameters are finite")
    })
}

/// Parse + validate export arguments. Kept free of registry or filesystem
/// access so the error paths are unit-testable.
fn parse_export(args: &[String]) -> Result<ExportArgs, String> {
    let mut a = ExportArgs {
        tiling: String::new(),
        width: 1200,
        height: 800,
        out: "tessellix.svg".into(),
        scale: 1.0,
        rotation: 0.0,
        mode: ColorMode::Pattern,
        params: Vec::new(),
        strapwork: None,
        deform: None,
        decoration: None,
        band_w: 0.1,
        center: None,
    };
    let mut i = 0;
    let mut band_w_seen = false;
    let next = |i: &mut usize| -> Result<String, String> {
        if *i >= args.len() {
            return Err(format!(
                "missing value for {}",
                args[(*i).saturating_sub(1)]
            ));
        }
        let v = args[*i].clone();
        *i += 1;
        Ok(v)
    };
    while i < args.len() {
        let key = args[i].as_str();
        i += 1;
        match key {
            "--tiling" => a.tiling = next(&mut i)?,
            "--width" => {
                a.width = next(&mut i)?
                    .parse::<u32>()
                    .map_err(|_| "bad --width".to_string())?
            }
            "--height" => {
                a.height = next(&mut i)?
                    .parse::<u32>()
                    .map_err(|_| "bad --height".to_string())?
            }
            "--out" | "-o" => a.out = next(&mut i)?,
            "--scale" => a.scale = finite(&next(&mut i)?, "--scale")?,
            "--rotation" => a.rotation = finite(&next(&mut i)?, "--rotation")?,
            "--mode" => {
                a.mode = ColorMode::parse(&next(&mut i)?).ok_or_else(|| "bad --mode".to_string())?
            }
            "--strapwork" => a.strapwork = Some(finite(&next(&mut i)?, "--strapwork")?),
            "--deform" => {
                let name = next(&mut i)?;
                a.deform = Some(parse_deform(&name, args, &mut i)?);
            }
            "--decoration" => {
                let v = next(&mut i)?;
                a.decoration = Some(Decoration::parse(&v).ok_or_else(|| {
                    format!(
                        "bad --decoration '{v}': \
                         expected fill | outline | interlace | outline-interlace"
                    )
                })?);
            }
            "--band-w" => {
                band_w_seen = true;
                a.band_w = finite(&next(&mut i)?, "--band-w")?;
            }
            "--center" => {
                let v = next(&mut i)?;
                let (x_s, y_s) = v.split_once(',').ok_or_else(|| {
                    format!("bad --center '{v}': expected X,Y")
                })?;
                a.center = Some((
                    finite(x_s, "--center x")?,
                    finite(y_s, "--center y")?,
                ));
            }
            "--param" => {
                let v = next(&mut i)?;
                let (id, value) = v
                    .split_once('=')
                    .ok_or_else(|| format!("bad --param '{v}': expected ID=VALUE"))?;
                if id.is_empty() || value.is_empty() {
                    return Err(format!("bad --param '{v}': expected ID=VALUE"));
                }
                a.params.push((id.to_string(), value.to_string()));
            }
            other => return Err(format!("unknown option {other}")),
        }
    }
    if a.tiling.is_empty() {
        return Err("--tiling is required (see `tess-tui list`)".into());
    }
    if a.width == 0 || a.height == 0 {
        return Err("--width and --height must be > 0".into());
    }
    if a.scale <= 0.0 {
        return Err("--scale must be a positive number".into());
    }
    if a.strapwork.is_some() && a.deform.is_some() {
        return Err(
            "--strapwork and --deform are mutually exclusive: --deform replaces the constant contact angle"
                .into(),
        );
    }
    if a.decoration.is_some() && a.strapwork.is_none() && a.deform.is_none() {
        return Err(
            "--decoration requires --strapwork DEG or --deform PROFILE (the decoration decorates the strapwork walls)"
                .into(),
        );
    }
    if band_w_seen && a.decoration.is_none() {
        return Err("--band-w requires --decoration".into());
    }
    if a.band_w <= 0.0 {
        return Err("--band-w must be a positive number".into());
    }
    Ok(a)
}

fn export(args: &[String]) -> Result<(), String> {
    let a = parse_export(args)?;

    let registry = tess_tilings::registry();
    let tiling = registry
        .get(&a.tiling)
        .ok_or_else(|| format!("unknown tiling: {}", a.tiling))?;
    let mut params = tess_core::params::ParamSet::default_for(tiling.params());
    params.set(tess_tilings::lattice::SCALE_ID, ParamValue::Float(a.scale));
    params.set(
        tess_tilings::lattice::ROTATION_ID,
        ParamValue::Angle(a.rotation),
    );
    // Generic --param ID=VALUE overrides, resolved against the tiling's
    // self-describing specs (ParamId holds a 'static str, so match on the
    // spec's own id, then set it with the kind-appropriate value).
    for (id, value) in &a.params {
        let spec = tiling
            .params()
            .iter()
            .find(|s| s.id.as_str() == id)
            .ok_or_else(|| {
                let name = a.tiling.clone();
                format!("unknown parameter '{id}' for tiling '{name}' (see `tess-tui list`)")
            })?;
        let v = match spec.kind {
            ParamKind::Float { .. } => {
                ParamValue::Float(finite(value, &format!("value for parameter '{id}'"))?)
            }
            ParamKind::Angle { .. } => {
                ParamValue::Angle(finite(value, &format!("value for parameter '{id}'"))?)
            }
            ParamKind::Int { .. } => ParamValue::Int(
                value
                    .parse()
                    .map_err(|_| format!("bad value for parameter '{id}': {value}"))?,
            ),
            ParamKind::Bool { .. } => match value.as_str() {
                "true" | "on" | "1" => ParamValue::Bool(true),
                "false" | "off" | "0" => ParamValue::Bool(false),
                _ => return Err(format!("bad boolean value for parameter '{id}': {value}")),
            },
        };
        params.set(spec.id, v);
    }

    // View centered at --center (default origin), ~20 world units tall at scale 1.
    let world_h = 20.0 / a.scale.max(1e-9);
    let world_w = world_h * a.width as f64 / a.height as f64;
    let (cx, cy) = a.center.unwrap_or((0.0, 0.0));
    let region = tess_core::BBox::new(
        tess_core::Vec2::new(cx - world_w / 2.0, cy - world_h / 2.0),
        tess_core::Vec2::new(cx + world_w / 2.0, cy + world_h / 2.0),
    );
    let result = tiling.generate(&params, &tess_core::Query { region });
    if let Some(msg) = result.message {
        return Err(msg.into());
    }

    // Hankin PIC strapwork overlay (M57 layer mode): walls at the given
    // contact angle over whatever tiles were generated. Absent flag = no
    // overlay, and the SVG is byte-identical to the pre-M57 export.
    let walls = if let Some(profile) = &a.deform {
        let f = resolve_deform(*profile, tess_tilings::envelope_for(&a.tiling), &a.tiling)?;
        tess_core::strapwork::walls_with_delta(&result.tiles, f)
    } else {
        match a.strapwork {
            Some(delta) => tess_core::strapwork::walls(&result.tiles, delta),
            None => Vec::new(),
        }
    };
    // Decoration styles (Kaplan §3.7) replace the tiling render with
    // paper + decorated walls. The gates run on the view region itself:
    // the coverage contract keeps every unshared host edge (the source
    // of rim artefacts) outside it, so only genuine openings fail. The
    // hairline inset drops vertices that sit exactly on the region edge.
    let svg = match a.decoration {
        Some(style) => {
            let plan = tess_core::strapwork::plan_decoration(
                &walls,
                a.band_w,
                style,
                &region.padding(-1e-9),
            )
            .map_err(|e| e.to_string())?;
            tess_core::svg::render_svg_decorated(&region, &plan)
        }
        None => tess_core::svg::render_svg_with_walls(&result.tiles, &region, a.mode, &walls),
    };
    std::fs::write(&a.out, svg).map_err(|e| format!("failed to write {}: {e}", a.out))?;
    println!("wrote {} ({} tiles)", a.out, result.tiles.len());
    Ok(())
}


// ---------------------------------------------------------------------------
// explore — the deterministic δ/θ sweep (docs/briefs/pic-explorer.md)
// ---------------------------------------------------------------------------

/// Parsed `tess-tui explore` arguments (no registry or filesystem access).
#[derive(Debug)]
struct ExploreArgs {
    tiling: String,
    out: Option<String>,
    csv: Option<String>,
    gallery: Option<String>,
    top_k: usize,
}

/// Parse + validate explore arguments. Pure: named errors, no registry,
/// no filesystem — registry/generation happens in [`explore`].
fn parse_explore(args: &[String]) -> Result<ExploreArgs, String> {
    let mut a = ExploreArgs {
        tiling: String::new(),
        out: None,
        csv: None,
        gallery: None,
        top_k: 10,
    };
    let mut i = 0;
    let next = |i: &mut usize| -> Result<String, String> {
        if *i >= args.len() {
            return Err(format!(
                "missing value for {}",
                args[(*i).saturating_sub(1)]
            ));
        }
        let v = args[*i].clone();
        *i += 1;
        Ok(v)
    };
    while i < args.len() {
        let key = args[i].as_str();
        i += 1;
        match key {
            "--tiling" => a.tiling = next(&mut i)?,
            "--out" => a.out = Some(next(&mut i)?),
            "--csv" => a.csv = Some(next(&mut i)?),
            "--gallery" => a.gallery = Some(next(&mut i)?),
            "--top-k" => {
                let v = next(&mut i)?;
                a.top_k = v.parse::<usize>().map_err(|_| "bad --top-k".to_string())?;
            }
            other => return Err(format!("unknown option {other}")),
        }
    }
    if a.tiling.is_empty() {
        return Err("--tiling is required (see `tess-tui list`)".into());
    }
    if a.top_k == 0 {
        return Err("--top-k must be >= 1".into());
    }
    Ok(a)
}

// --- JSON / CSV builders (pure string builders, byte-deterministic) ------

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// 10-decimal formatting for every finite number in the output (byte-
/// deterministic across runs and platforms); non-finite → null.
fn json_f(x: f64) -> String {
    if x.is_finite() {
        format!("{:.10}", x)
    } else {
        "null".to_string()
    }
}

fn json_num(x: usize) -> String {
    x.to_string()
}

fn json_str_opt(x: &Option<String>) -> String {
    match x {
        Some(s) => json_str(s),
        None => "null".to_string(),
    }
}

fn json_bool_opt(x: Option<bool>) -> String {
    match x {
        Some(b) => b.to_string(),
        None => "null".to_string(),
    }
}

fn json_bool(x: bool) -> String {
    x.to_string()
}

fn json_degree_hist(hist: &std::collections::BTreeMap<usize, usize>) -> String {
    let inner: Vec<String> = hist.iter().map(|(d, n)| format!("{}: {}", json_str(&d.to_string()), json_num(*n))).collect();
    format!("{{{}}}", inner.join(", "))
}

fn json_envelope_angles(angles: &Option<Vec<f64>>) -> String {
    match angles {
        Some(list) => format!(
            "[{}]",
            list.iter().map(|a| json_f(*a)).collect::<Vec<_>>().join(", ")
        ),
        None => "null".to_string(),
    }
}

fn delta_row_json(row: &tess_tilings::sweep::DeltaRow) -> String {
    let f = &row.features;
    let validity = format!(
        "{{\"core_vertices\": {}, \"exit\": {}, \"corner\": {}, \"bad\": {}, \"straight2\": {}, \"joint\": {}}}",
        f.core_vertices, f.exit, f.corner, f.bad, f.straight2, f.joint
    );
    let wall_map = format!(
        "{{\"degree_hist\": {}, \"strand_count\": {}, \"face_count\": {}, \"min_face_area\": {}}}",
        json_degree_hist(&f.degree_hist), f.strand_count, f.face_count, json_f(f.min_face_area)
    );
    let geometry = format!(
        "{{\"total_wall_length\": {}, \"strap_density\": {}, \"crossing_count\": {}, \"min_crossing_angle_deg\": {}, \"crossing_density\": {}}}",
        json_f(f.total_wall_length), json_f(f.strap_density), f.crossing_count,
        json_f(f.min_crossing_angle_deg), json_f(f.crossing_density)
    );
    let symmetry = format!(
        "{{\"translation_invariant\": {}, \"translation_mismatch\": {}, \"mirror_invariant\": {}, \"rotation_invariant\": {}, \"note\": {}, \"fold_note\": {}}}",
        json_bool(f.translation_invariant),
        match f.translation_mismatch {
            usize::MAX => "null".to_string(),
            n => n.to_string(),
        },
        json_bool_opt(f.mirror_invariant),
        json_bool_opt(f.rotation_invariant),
        json_str(&f.symmetry_note),
        json_str_opt(&f.fold_note)
    );
    let feasibility = if f.generation_message.is_some() {
        format!("refused: {}", f.generation_message.as_deref().unwrap())
    } else {
        "ok".to_string()
    };
    let score = match row.score {
        Some(s) => json_f(s),
        None => "null".to_string(),
    };
    format!(
        "{{\"axis\": \"delta\", \"substrate_id\": {}, \"delta_deg\": {}, \"theta_offset_deg\": null, \"rank\": {}, \"score\": {}, \"feasibility\": {}, \"envelope_member\": {}, \"envelope_angles\": {}, \"closure_violation\": {}, \"generation_message\": {}, \"validity\": {}, \"wall_map\": {}, \"geometry\": {}, \"symmetry\": {}}}",
        json_str(&row.substrate_id),
        json_f(row.delta_deg),
        row.rank,
        score,
        json_str(&feasibility),
        match f.envelope_member {
            Some(true) => "true".to_string(),
            Some(false) => "false".to_string(),
            None => "null".to_string(),
        },
        json_envelope_angles(&f.envelope_angles),
        json_str_opt(&f.closure_violation),
        match &f.generation_message {
            Some(m) => json_str(m),
            None => "null".to_string(),
        },
        validity,
        wall_map,
        geometry,
        symmetry
    )
}

fn theta_row_json(row: &tess_tilings::sweep::ThetaRow) -> String {
    let f = &row.features;
    let feasibility = match &f.feasibility_reason {
        Some(reason) => format!("refused: {reason}"),
        None => {
            if f.partition_complete {
                "ok".to_string()
            } else {
                "incomplete_partition".to_string()
            }
        }
    };
    let score = match row.score {
        Some(s) => json_f(s),
        None => "null".to_string(),
    };
    format!(
        "{{\"axis\": \"theta\", \"substrate_id\": {}, \"delta_deg\": null, \"theta_offset_deg\": {}, \"rank\": {}, \"score\": {}, \"feasibility\": {}, \"motifs\": {}, \"partition_complete\": {}, \"max_partition_error\": {}, \"n_cells\": {}, \"min_face_area\": {}, \"total_area\": {}, \"symmetry\": {}, \"feasibility_reason\": {}}}",
        json_str(&row.substrate_id),
        json_f(row.theta_offset_deg),
        row.rank,
        score,
        json_str(&feasibility),
        json_str(&f.motifs),
        json_bool(f.partition_complete),
        json_f(f.max_partition_error),
        f.n_cells,
        json_f(f.min_face_area),
        json_f(f.total_area),
        json_str(&f.symmetry),
        json_str_opt(&f.feasibility_reason)
    )
}

/// The full leaderboard document (byte-deterministic): the fixed
/// region, the documented scores, the sort contract, then both axes.
fn leaderboard_json(outcome: &tess_tilings::sweep::SweepOutcome) -> String {
    let delta_json: Vec<String> = outcome.delta_rows.iter().map(delta_row_json).collect();
    let theta_json: Vec<String> = outcome.theta_rows.iter().map(theta_row_json).collect();
    let refused = outcome
        .delta_rows
        .iter()
        .filter(|r| r.score.is_none())
        .count()
        + outcome
            .theta_rows
            .iter()
            .filter(|r| r.score.is_none())
            .count();
    let closure_violations = outcome
        .delta_rows
        .iter()
        .filter(|r| r.features.closure_violation.is_some())
        .count();
    format!(
        "{{\"schema\": \"pic-explorer-leaderboard/v1\", \"substrate\": {}, \"region\": {{\"min\": [{}, {}], \"max\": [{}, {}]}}, \"sort\": {{\"delta\": \"score asc, delta_deg asc (refused sink, rank 0)\", \"theta\": \"score asc, theta_offset_deg asc (refused sink, rank 0)\"}}, \"score_formula_delta\": \"1000 * (not an envelope member) + 100 * bad + 10 * (exit + corner) + 1/(min_face_area + 1e-9) + 10 * (1 - translation_invariant) + 0.5 * (1 - mirror_invariant, when checked) + 0.5 * (1 - rotation_invariant, when checked); sliver >= 10 when no faces\", \"score_formula_theta\": \"1e6 * (not partition_complete) + 1/(min_face_area + 1e-9)\", \"refused_count\": {}, \"closure_violation_count\": {}, \"delta_rows\": [{}], \"theta_rows\": [{}]}}",
        json_str(&outcome.substrate_id),
        json_f(tess_tilings::sweep::REGION.min.x),
        json_f(tess_tilings::sweep::REGION.min.y),
        json_f(tess_tilings::sweep::REGION.max.x),
        json_f(tess_tilings::sweep::REGION.max.y),
        refused,
        closure_violations,
        delta_json.join(", "),
        theta_json.join(", ")
    )
}

/// Escape a CSV field: wrap in quotes if it contains a comma, quote or
/// newline (inner quotes doubled) — everything else passes through.
fn csv_field(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn csv_f(x: f64) -> String {
    if x.is_finite() {
        format!("{:.10}", x)
    } else {
        String::new()
    }
}

/// One flat row per leaderboard row (null cells for the non-applicable
/// axis). Deterministic byte-for-byte across runs.
fn leaderboard_csv(outcome: &tess_tilings::sweep::SweepOutcome) -> String {
    let mut out = String::from(
        "substrate_id,axis,delta_deg,theta_offset_deg,score,rank,feasibility,closure_envelope_member,closure_violation,core_vertices,exit,corner,bad,straight2,joint,strand_count,face_count,min_face_area,total_wall_length,strap_density,crossing_count,min_crossing_angle_deg,crossing_density,translation_invariant,translation_mismatch,mirror_invariant,rotation_invariant,symmetry,partition_complete,max_partition_error,n_cells,total_area,motifs,degree_hist\n",
    );
    for row in &outcome.delta_rows {
        let f = &row.features;
        let feasibility = if f.generation_message.is_some() {
            format!("refused: {}", f.generation_message.as_deref().unwrap())
        } else {
            "ok".to_string()
        };
        let env = match f.envelope_member {
            Some(true) => "true".to_string(),
            Some(false) => "false".to_string(),
            None => "".to_string(),
        };
        let mismatch = if f.translation_mismatch == usize::MAX {
            String::new()
        } else {
            f.translation_mismatch.to_string()
        };
        let cells = vec![
            csv_field(&row.substrate_id),
            "delta".to_string(),
            csv_f(row.delta_deg),
            String::new(),
            row.score.map(csv_f).unwrap_or_default(),
            row.rank.to_string(),
            csv_field(&feasibility),
            env,
            f.closure_violation.as_deref().map(csv_field).unwrap_or_default(),
            f.core_vertices.to_string(),
            f.exit.to_string(),
            f.corner.to_string(),
            f.bad.to_string(),
            f.straight2.to_string(),
            f.joint.to_string(),
            f.strand_count.to_string(),
            f.face_count.to_string(),
            csv_f(f.min_face_area),
            csv_f(f.total_wall_length),
            csv_f(f.strap_density),
            f.crossing_count.to_string(),
            csv_f(f.min_crossing_angle_deg),
            csv_f(f.crossing_density),
            f.translation_invariant.to_string(),
            mismatch,
            f.mirror_invariant.map(|b| b.to_string()).unwrap_or_default(),
            f.rotation_invariant.map(|b| b.to_string()).unwrap_or_default(),
            csv_field(&f.symmetry_note),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            csv_field(
                &f.degree_hist
                    .iter()
                    .map(|(d, n)| format!("{d}:{n}"))
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        ];
        out.push_str(&cells.join(","));
        out.push('\n');
    }
    for row in &outcome.theta_rows {
        let f = &row.features;
        let feasibility = match &f.feasibility_reason {
            Some(reason) => format!("refused: {reason}"),
            None => {
                if f.partition_complete {
                    "ok".to_string()
                } else {
                    "incomplete_partition".to_string()
                }
            }
        };
        let cells = vec![
            csv_field(&row.substrate_id),
            "theta".to_string(),
            String::new(),
            csv_f(row.theta_offset_deg),
            row.score.map(csv_f).unwrap_or_default(),
            row.rank.to_string(),
            csv_field(&feasibility),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            csv_field(&f.symmetry),
            f.partition_complete.to_string(),
            csv_f(f.max_partition_error),
            f.n_cells.to_string(),
            csv_f(f.total_area),
            csv_field(&f.motifs),
            String::new(),
        ];
        out.push_str(&cells.join(","));
        out.push('\n');
    }
    out
}

/// Render the top-`top_k` gallery rows into `dir`, one SVG per row:
/// δ rows via the decoration-first gallery, θ rows via the cell render.
/// Refused rows are skipped with a note. Returns the number of SVGs written.
fn explore_write_gallery(
    outcome: &tess_tilings::sweep::SweepOutcome,
    dir: &str,
    top_k: usize,
) -> Result<(usize, usize), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("failed to create gallery dir {dir}: {e}"))?;
    let mut written = 0usize;
    let mut skipped = 0usize;
    let mut delta_scored: Vec<&tess_tilings::sweep::DeltaRow> = outcome
        .delta_rows
        .iter()
        .filter(|r| r.rank > 0)
        .collect();
    delta_scored.sort_by(|a, b| a.rank.cmp(&b.rank));
    for row in delta_scored.into_iter().take(top_k) {
        let name = format!("{}-d{:.1}-r{}.svg", row.substrate_id, row.delta_deg, row.rank);
        let path = std::path::Path::new(dir).join(name);
        let svg = tess_tilings::sweep::delta_gallery_svg(
            &outcome.delta_tiles,
            &tess_tilings::sweep::REGION,
            row.delta_deg,
        );
        std::fs::write(&path, svg).map_err(|e| format!("failed to write {}: {e}", path.display()))?;
        written += 1;
    }
    let refused_delta = outcome
        .delta_rows
        .iter()
        .filter(|r| r.rank == 0)
        .count();
    skipped += refused_delta;
    let mut theta_scored: Vec<&tess_tilings::sweep::ThetaRow> = outcome
        .theta_rows
        .iter()
        .filter(|r| r.rank > 0)
        .collect();
    theta_scored.sort_by(|a, b| a.rank.cmp(&b.rank));
    for row in theta_scored.into_iter().take(top_k) {
        let name = format!(
            "{}-t{:.1}-r{}.svg",
            row.substrate_id, row.theta_offset_deg, row.rank
        );
        let path = std::path::Path::new(dir).join(name);
        match tess_tilings::sweep::theta_gallery_svg(&row.substrate_id, row.theta_offset_deg) {
            Ok(svg) => {
                std::fs::write(&path, svg).map_err(|e| format!("failed to write {}: {e}", path.display()))?;
                written += 1;
            }
            Err(_) => skipped += 1,
        }
    }
    let refused_theta = outcome
        .theta_rows
        .iter()
        .filter(|r| r.rank == 0)
        .count();
    skipped += refused_theta;
    Ok((written, skipped))
}

fn explore(args: &[String]) -> Result<(), String> {
    let a = parse_explore(args)?;
    let registry = tess_tilings::registry();
    let tiling = registry
        .get(&a.tiling)
        .ok_or_else(|| format!("unknown tiling: {}", a.tiling))?;
    let outcome = tess_tilings::sweep::run_sweep(tiling, &tess_tilings::sweep::REGION);

    let json = leaderboard_json(&outcome);
    let csv_text = leaderboard_csv(&outcome);

    if let Some(out) = &a.out {
        std::fs::write(out, &json).map_err(|e| format!("failed to write {out}: {e}"))?;
        println!("wrote {out} ({} rows)", outcome.delta_rows.len() + outcome.theta_rows.len());
    } else {
        print!("{}", json);
    }
    if let Some(csv_path) = &a.csv {
        std::fs::write(csv_path, &csv_text).map_err(|e| format!("failed to write {csv_path}: {e}"))?;
        println!("wrote {csv_path}");
    }
    if let Some(dir) = &a.gallery {
        let (written, skipped) = explore_write_gallery(&outcome, dir, a.top_k)?;
        println!("wrote {} gallery SVG(s) into {dir} ({skipped} refused row(s) skipped)", written);
    }
    Ok(())
}

#[cfg(test)]
mod tests_explore {
    use super::{explore, leaderboard_json, parse_explore};

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn explore_parse_requires_tiling() {
        let err = parse_explore(&args(&["--top-k", "3"])).unwrap_err();
        assert!(err.contains("--tiling is required"));
    }

    #[test]
    fn explore_parse_rejects_bad_top_k() {
        let err = parse_explore(&args(&["--tiling", "square", "--top-k", "0"])).unwrap_err();
        assert!(err.contains("--top-k"));
        let err = parse_explore(&args(&["--tiling", "square", "--top-k", "x"])).unwrap_err();
        assert!(err.contains("--top-k"));
    }

    #[test]
    fn explore_parse_rejects_unknown_option() {
        let err = parse_explore(&args(&["--tiling", "square", "--bogus"])).unwrap_err();
        assert!(err.contains("unknown option --bogus"));
    }

    #[test]
    fn explore_rejects_unknown_tiling() {
        let err = explore(&args(&["--tiling", "no-such-tiling"])).unwrap_err();
        assert!(err.contains("unknown tiling: no-such-tiling"));
    }

    #[test]
    fn explore_leaderboard_is_byte_deterministic() {
        let reg = tess_tilings::registry();
        let outcome = tess_tilings::sweep::run_sweep(
            reg.get("square").unwrap(),
            &tess_tilings::sweep::REGION,
        );
        let a = leaderboard_json(&outcome);
        let b = leaderboard_json(&outcome);
        assert_eq!(a, b);
        assert!(a.contains("\"schema\": \"pic-explorer-leaderboard/v1\""));
    }

    #[test]
    fn explore_two_runs_produce_identical_bytes() {
        let tmp = std::env::temp_dir();
        let name = std::process::id().to_string();
        let json_a = tmp.join(format!("tess_explore_a_{name}.json"));
        let json_b = tmp.join(format!("tess_explore_b_{name}.json"));
        let csv_a = tmp.join(format!("tess_explore_a_{name}.csv"));
        let csv_b = tmp.join(format!("tess_explore_b_{name}.csv"));
        let dir_a = tmp.join(format!("tess_explore_gallery_a_{name}"));
        let dir_b = tmp.join(format!("tess_explore_gallery_b_{name}"));
        for (out, csv, dir) in [
            (&json_a, &csv_a, &dir_a),
            (&json_b, &csv_b, &dir_b),
        ] {
            let argv = vec![
                "--tiling".to_string(),
                "square".to_string(),
                "--out".to_string(),
                out.to_string_lossy().to_string(),
                "--csv".to_string(),
                csv.to_string_lossy().to_string(),
                "--gallery".to_string(),
                dir.to_string_lossy().to_string(),
                "--top-k".to_string(),
                "3".to_string(),
            ];
            explore(&argv).expect("explore succeeds");
        }
        assert_eq!(std::fs::read(&json_a).unwrap(), std::fs::read(&json_b).unwrap(), "JSON bytes");
        assert_eq!(std::fs::read(&csv_a).unwrap(), std::fs::read(&csv_b).unwrap(), "CSV bytes");
        let mut a_files: Vec<_> = std::fs::read_dir(&dir_a).unwrap().map(|e| e.unwrap().file_name()).collect();
        let mut b_files: Vec<_> = std::fs::read_dir(&dir_b).unwrap().map(|e| e.unwrap().file_name()).collect();
        a_files.sort();
        b_files.sort();
        assert_eq!(a_files, b_files, "gallery file sets match");
        assert!(!a_files.is_empty(), "gallery must write at least one SVG");
        for f in &a_files {
            assert_eq!(
                std::fs::read(dir_a.join(f)).unwrap(),
                std::fs::read(dir_b.join(f)).unwrap(),
                "gallery bytes for {:?}",
                f
            );
        }
        let _ = std::fs::remove_dir_all(&dir_a);
        let _ = std::fs::remove_dir_all(&dir_b);
        let _ = std::fs::remove_file(&json_a);
        let _ = std::fs::remove_file(&json_b);
        let _ = std::fs::remove_file(&csv_a);
        let _ = std::fs::remove_file(&csv_b);
    }
}

// Doc-vs-code coupling pins for docs/USAGE.md's explore section (the
// audit-X doc-vs-code checks made permanent). Each claim in the explore
// section (flags, defaults, region, grids, schema key, row contract)
// is extracted from the doc — zero-or-one match per anchor, failures
// naming the file + line + claim — and compared against the live parse
// layer, the sweep constants, the leaderboard builder, or the gallery
// writer. No CLI process spawn, no interactive TUI; purely test-only.
#[cfg(test)]
mod tests_explore_contract {
    use super::{explore_write_gallery, leaderboard_json, parse_explore};
    use tess_tilings::sweep::{
        run_sweep, DeltaRow, DELTA_GRID, REGION, SweepFeatures, SweepOutcome, ThetaFeatures,
        ThetaRow, THETA_AXIS_IDS, THETA_GRID,
    };

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    fn usage_doc() -> String {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/USAGE.md");
        std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!("docs/USAGE.md readable from crates/tess-tui ({}): {e}", path.display())
        })
    }

    /// The unique byte position of `anchor`; else panic loudly.
    fn unique_anchor(text: &str, anchor: &str, what: &str) -> usize {
        let positions: Vec<usize> = text.match_indices(anchor).map(|(i, _)| i).collect();
        match positions.len() {
            0 => panic!("{what}: anchor missing: {anchor:?}"),
            1 => positions[0],
            n => panic!("{what}: anchor ambiguous ({n} matches): {anchor:?}"),
        }
    }

    /// 1-based line number of the anchor at `pos`.
    fn anchor_line(text: &str, pos: usize) -> usize {
        text[..pos].chars().filter(|&c| c == '\n').count() + 1
    }

    /// Whitespace-normalized single line containing `pos`.
    fn line_region(text: &str, pos: usize) -> String {
        let start = text[..pos].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let end = text[pos..].find('\n').map(|i| pos + i).unwrap_or(text.len());
        text[start..end]
            .chars()
            .map(|c| if c.is_whitespace() { ' ' } else { c })
            .collect()
    }

    /// The parsed digit run immediately following `marker` on `line`.
    fn digits_after(line: &str, marker: &str, what: &str) -> usize {
        let i = line
            .find(marker)
            .unwrap_or_else(|| panic!("{what}: marker {marker:?} missing on the anchor's line"));
        let after = &line[i + marker.len()..];
        let mut digits = String::new();
        for c in after.chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            } else if !digits.is_empty() {
                break;
            }
        }
        digits.parse().unwrap_or_else(|_| panic!("{what}: no digit after {marker:?}"))
    }

    /// The parsed digit run immediately preceding `suffix` on `line`.
    fn digits_before(line: &str, suffix: &str, what: &str) -> usize {
        let i = line
            .find(suffix)
            .unwrap_or_else(|| panic!("{what}: suffix {suffix:?} missing on the anchor's line"));
        let mut digits = String::new();
        for c in line[..i].chars().rev() {
            if c.is_ascii_digit() {
                digits.push(c);
            } else if !digits.is_empty() {
                break;
            }
        }
        let digits: String = digits.chars().rev().collect();
        digits
            .parse()
            .unwrap_or_else(|_| panic!("{what}: no number before {suffix:?}"))
    }

    /// The (line number, whitespace-normalized line) of a unique anchor,
    /// with a failure message naming the file + line + claim.
    fn anchor_line_for(text: &str, anchor: &str, claim: &str) -> (usize, String) {
        let pos = unique_anchor(text, anchor, &format!("docs/USAGE.md: {claim}"));
        (anchor_line(text, pos), line_region(text, pos))
    }

    /// Split a table line into trimmed cells; cells[1] = flag, cells[2] =
    /// default, cells[3] = meaning.
    fn table_cells(line: &str, what: &str) -> Vec<String> {
        let cells: Vec<String> = line.split('|').map(|c| c.trim().to_string()).collect();
        assert!(
            cells.len() >= 4,
            "{what}: expected flag|default|meaning in {line:?}"
        );
        cells
    }

    /// Resolve a table-row anchor and return (line_no, line, cells).
    /// The 4th element (offset) is unused; the caller keeps the pieces
    /// it needs.
    fn explore_table_row(text: &str, anchor: &str, claim: &str) -> (usize, String, Vec<String>) {
        let (line_no, line) = anchor_line_for(text, anchor, claim);
        let cells = table_cells(&line, &format!("docs/USAGE.md line {line_no} ({claim})"));
        (line_no, line, cells)
    }

    // ---- flag set ------------------------------------------------------

    /// The explore table documents exactly the five flags the parse layer
    /// accepts — every documented flag parses; an unknown option errors.
    #[test]
    fn usage_explore_flag_set_matches_code() {
        let doc = usage_doc();
        let rows = [
            ("any registered substrate", "--tiling"),
            ("write the leaderboard JSON to PATH instead of printing it", "--out"),
            ("write a flat CSV variant", "--csv"),
            ("one SVG per top-`--top-k` ranked row", "--gallery"),
            ("must be ≥ 1", "--top-k"),
        ];
        for (anchor, flag) in rows {
            let (line_no, _line, cells) =
                explore_table_row(&doc, anchor, "docs/USAGE.md: explore flag-set pin");
            let doc_flag = cells[1]
                .trim_start_matches('`')
                .trim_end_matches('`')
                .split(' ')
                .next()
                .unwrap()
                .to_string();
            assert_eq!(
                doc_flag, flag,
                "docs/USAGE.md line {line_no} (flag set: explore row {flag}): documented flag {doc_flag:?}"
            );
        }
        // Each documented flag parses successfully (with a valid tiling).
        for (flag, value) in [
            ("--out", "x"),
            ("--csv", "x"),
            ("--gallery", "x"),
            ("--top-k", "1"),
        ] {
            let argv = args(&["--tiling", "square", flag, value]);
            parse_explore(&argv)
                .unwrap_or_else(|e| panic!("docs/USAGE.md: {flag} must parse ({e})"));
        }
        let argv = args(&["--tiling", "square", "--bogus"]);
        let err = parse_explore(&argv).unwrap_err();
        assert!(
            err.contains("unknown option --bogus"),
            "unknown option must be a named parse error, got {err:?}"
        );
    }

    /// `--tiling ID` is documented as required; the parse layer rejects a
    /// missing `--tiling` with a named error.
    #[test]
    fn usage_tiling_is_required_matches_doc() {
        let doc = usage_doc();
        let (line_no, _line, cells) =
            explore_table_row(&doc, "any registered substrate", "docs/USAGE.md: --tiling required pin");
        assert!(
            cells[2].contains("required"),
            "docs/USAGE.md line {line_no} (--tiling required): default cell {:?} should say required",
            cells[2]
        );
        let err = parse_explore(&args(&["--top-k", "3"])).unwrap_err();
        assert!(
            err.contains("--tiling"),
            "docs/USAGE.md line {line_no} (--tiling required): named error expected, got {err:?}"
        );
    }

    /// `--out` documents a stdout default; the parse layer leaves `out`
    /// as None (the CLI prints the JSON to stdout in that case).
    #[test]
    fn usage_out_default_is_stdout_matches_doc() {
        let doc = usage_doc();
        let (line_no, _line, cells) = explore_table_row(
            &doc,
            "write the leaderboard JSON to PATH instead of printing it",
            "docs/USAGE.md: --out default pin",
        );
        assert!(
            cells[2].contains("stdout"),
            "docs/USAGE.md line {line_no} (--out default): default cell {:?} should say stdout",
            cells[2]
        );
        let a = parse_explore(&args(&["--tiling", "square"]))
            .expect("parse_explore accepts a bare --tiling");
        assert!(
            a.out.is_none(),
            "docs/USAGE.md line {line_no} (--out default stdout): parsed out should be None (stdout), got {:?}",
            a.out
        );
    }

    /// `--csv` and `--gallery` are documented as defaults-off; the parse
    /// layer leaves both as None by default.
    #[test]
    fn usage_csv_gallery_default_off_matches_doc() {
        let doc = usage_doc();
        let (line_no, _line, cells) = explore_table_row(
            &doc,
            "write a flat CSV variant",
            "docs/USAGE.md: --csv default pin",
        );
        assert!(
            cells[2].contains("off"),
            "docs/USAGE.md line {line_no} (--csv default): default cell {:?} should say off",
            cells[2]
        );
        let (line_no2, _line2, cells2) = explore_table_row(
            &doc,
            "one SVG per top-`--top-k` ranked row",
            "docs/USAGE.md: --gallery default pin",
        );
        assert!(
            cells2[2].contains("off"),
            "docs/USAGE.md line {line_no2} (--gallery default): default cell {:?} should say off",
            cells2[2]
        );
        let a = parse_explore(&args(&["--tiling", "square"]))
            .expect("parse_explore accepts a bare --tiling");
        assert!(
            a.csv.is_none() && a.gallery.is_none(),
            "docs/USAGE.md lines {line_no}/{line_no2} (--csv/--gallery default off): expected both None, got {:?}/{:?}",
            a.csv,
            a.gallery
        );
    }

    /// `--top-k` documents default 10 and a must-be-≥ 1 constraint; the
    /// parse layer parses the default, rejects 0/non-numeric with named
    /// errors, and accepts 1.
    #[test]
    fn usage_top_k_default_and_constraint_match_code() {
        let doc = usage_doc();
        let (line_no, line, cells) =
            explore_table_row(&doc, "must be ≥ 1", "docs/USAGE.md: --top-k default/constraint pin");
        let doc_default: usize = cells[2].parse().unwrap_or_else(|_| {
            panic!(
                "docs/USAGE.md line {line_no} (--top-k default): default cell {:?} not parseable",
                cells[2]
            )
        });
        assert!(
            line.contains("must be ≥ 1"),
            "docs/USAGE.md line {line_no} (--top-k constraint): row {line:?} should state >= 1"
        );
        let a = parse_explore(&args(&["--tiling", "square"]))
            .expect("parse_explore accepts a bare --tiling");
        assert_eq!(
            a.top_k, doc_default,
            "docs/USAGE.md line {line_no} (--top-k default): documented {doc_default} vs parsed {}",
            a.top_k
        );
        let err0 = parse_explore(&args(&["--tiling", "square", "--top-k", "0"])).unwrap_err();
        assert!(
            err0.contains("--top-k"),
            "docs/USAGE.md line {line_no} (--top-k constraint): 0 must be a named error, got {err0:?}"
        );
        let errx = parse_explore(&args(&["--tiling", "square", "--top-k", "x"])).unwrap_err();
        assert!(
            errx.contains("--top-k"),
            "docs/USAGE.md line {line_no} (--top-k constraint): non-numeric must be a named error, got {errx:?}"
        );
        let one = parse_explore(&args(&["--tiling", "square", "--top-k", "1"]))
            .expect("--top-k 1 parses");
        assert_eq!(
            one.top_k, 1,
            "docs/USAGE.md line {line_no} (--top-k constraint): 1 must parse as the documented minimum"
        );
    }

    // ---- region --------------------------------------------------------

    /// The region is documented as fixed at 20×20 world units centered on
    /// the origin (±10); the sweep constant must match.
    #[test]
    fn usage_region_claim_matches_region() {
        let doc = usage_doc();
        let (line_no, line) = anchor_line_for(
            &doc,
            "20×20 world units centered on the origin",
            "docs/USAGE.md: region pin",
        );
        let doc_size: usize = digits_before(
            &line,
            "×20 world units",
            &format!("docs/USAGE.md line {line_no} (region size)"),
        );
        let doc_half: usize = digits_after(
            &line,
            "±",
            &format!("docs/USAGE.md line {line_no} (region half-width)"),
        );
        assert_eq!(
            REGION.min.x,
            -(doc_half as f64),
            "docs/USAGE.md line {line_no} (region): min.x should be -{doc_half}, got {}",
            REGION.min.x
        );
        assert_eq!(
            REGION.max.x,
            doc_half as f64,
            "docs/USAGE.md line {line_no} (region): max.x should be {doc_half}, got {}",
            REGION.max.x
        );
        assert_eq!(
            REGION.max.x - REGION.min.x,
            doc_size as f64,
            "docs/USAGE.md line {line_no} (region): width should be {doc_size}, got {}",
            REGION.max.x - REGION.min.x
        );
        assert_eq!(
            REGION.max.y - REGION.min.y,
            doc_size as f64,
            "docs/USAGE.md line {line_no} (region): height should be {doc_size}, got {}",
            REGION.max.y - REGION.min.y
        );
    }

    // ---- grids ----------------------------------------------------------

    /// The δ axis is documented as the 17-point base grid, multiples of
    /// 5° from 5° to 85°; the DELTA_GRID constant must match.
    #[test]
    fn usage_delta_grid_claim_matches_constant() {
        let doc = usage_doc();
        let (line_no, line) = anchor_line_for(&doc, "17-point base grid", "docs/USAGE.md: δ grid pin");
        let doc_n: usize = digits_before(
            &line,
            "-point",
            &format!("docs/USAGE.md line {line_no} (δ grid size)"),
        );
        let step: usize = digits_after(
            &line,
            "multiples of ",
            &format!("docs/USAGE.md line {line_no} (δ grid step)"),
        );
        let start: usize = digits_after(
            &line,
            "from ",
            &format!("docs/USAGE.md line {line_no} (δ grid start)"),
        );
        let end: usize = digits_after(
            &line,
            "to ",
            &format!("docs/USAGE.md line {line_no} (δ grid end)"),
        );
        assert_eq!(
            DELTA_GRID.len(),
            doc_n,
            "docs/USAGE.md line {line_no} (δ grid size): doc says {doc_n} points, constant has {}",
            DELTA_GRID.len()
        );
        assert_eq!(
            DELTA_GRID[0],
            start as f64,
            "docs/USAGE.md line {line_no} (δ grid start): doc says {start}°, constant starts at {}°",
            DELTA_GRID[0]
        );
        assert_eq!(
            DELTA_GRID[DELTA_GRID.len() - 1],
            end as f64,
            "docs/USAGE.md line {line_no} (δ grid end): doc says {end}°, constant ends at {}°",
            DELTA_GRID[DELTA_GRID.len() - 1]
        );
        for w in DELTA_GRID.windows(2) {
            assert_eq!(
                w[1] - w[0],
                step as f64,
                "docs/USAGE.md line {line_no} (δ grid step): doc says step {step}°, found step {}° ({w:?})",
                w[1] - w[0]
            );
        }
    }

    /// The θ axis is documented as 7 rows per substrate, offsets from
    /// −15° to +15°, on the two classic hankin substrates; the constants
    /// must match.
    #[test]
    fn usage_theta_grid_and_substrates_match_constants() {
        let doc = usage_doc();
        let (line_no, line) =
            anchor_line_for(&doc, "7 rows per substrate", "docs/USAGE.md: θ grid pin");
        let doc_n: usize = digits_before(
            &line,
            "rows per substrate",
            &format!("docs/USAGE.md line {line_no} (θ grid size)"),
        );
        assert!(line.contains("−15°"), "docs/USAGE.md line {line_no}: line should state start −15°");
        assert!(line.contains("+15°"), "docs/USAGE.md line {line_no}: line should state end +15°");
        assert!(
            line.contains("hankin-4-8-8-motifs") && line.contains("hankin-3-12-12"),
            "docs/USAGE.md line {line_no}: line should name the two hankin substrates"
        );
        assert_eq!(
            THETA_GRID.len(),
            doc_n,
            "docs/USAGE.md line {line_no} (θ grid size): doc says {doc_n} rows, constant has {}",
            THETA_GRID.len()
        );
        assert_eq!(THETA_GRID[0], -15.0, "docs/USAGE.md line {line_no} (θ grid start): {:?}", THETA_GRID[0]);
        assert_eq!(THETA_GRID[THETA_GRID.len() - 1], 15.0, "docs/USAGE.md line {line_no} (θ grid end): {:?}", THETA_GRID[THETA_GRID.len() - 1]);
        assert_eq!(
            THETA_AXIS_IDS,
            ["hankin-4-8-8-motifs", "hankin-3-12-12"],
            "docs/USAGE.md line {line_no} (θ substrates): θ-axis ids must be the documented two"
        );
    }

    // ---- schema + determinism ------------------------------------------

    /// The output is documented as a `pic-explorer-leaderboard/v1` JSON
    /// document, byte-deterministic across runs; the leaderboard builder
    /// must emit that schema key and produce identical bytes across calls.
    #[test]
    fn usage_schema_key_and_determinism_match_code() {
        let doc = usage_doc();
        let (line_no, line) =
            anchor_line_for(&doc, "pic-explorer-leaderboard/v1", "docs/USAGE.md: schema key pin");
        let schema = line
            .split('`')
            .find(|s| s.contains("pic-explorer-leaderboard"))
            .expect("docs/USAGE.md: schema segment between backticks")
            .trim()
            .to_string();
        assert!(
            line.contains("byte-deterministic"),
            "docs/USAGE.md line {line_no} (byte-determinism): line should claim byte-determinism"
        );
        let reg = tess_tilings::registry();
        let tiling = reg.get("square").expect("square registered");
        let outcome = run_sweep(tiling, &REGION);
        let a = leaderboard_json(&outcome);
        let b = leaderboard_json(&outcome);
        assert_eq!(
            a, b,
            "docs/USAGE.md line {line_no} (byte-determinism): two leaderboard_json calls diverged"
        );
        assert!(
            a.contains(&format!("\"schema\": \"{schema}\"")),
            "docs/USAGE.md line {line_no} (schema key): JSON schema is not {schema:?}"
        );
    }

    // ---- top-k effect ----------------------------------------------------

    /// `--top-k` is documented as gallery rows per axis — its effect is
    /// scoped to the gallery, not the JSON. Behaviorally: the gallery
    /// SVG count is min(top-k, scored per axis), the skipped count is
    /// refused rows (independent of top-k), and the JSON bytes are
    /// unchanged.
    #[test]
    fn usage_top_k_affects_gallery_not_json() {
        let doc = usage_doc();
        let (line_no, line) =
            anchor_line_for(&doc, "gallery rows per axis", "docs/USAGE.md: --top-k effect pin");
        assert!(
            line.contains("gallery"),
            "docs/USAGE.md line {line_no} (--top-k effect): row {line:?} should name the gallery"
        );
        let reg = tess_tilings::registry();
        let tiling = reg.get("square").expect("square registered");
        let outcome = run_sweep(tiling, &REGION);
        let scored = outcome.delta_rows.iter().filter(|r| r.rank > 0).count();
        let refused = outcome.delta_rows.iter().filter(|r| r.rank == 0).count();
        let json = leaderboard_json(&outcome);
        let tmp = std::env::temp_dir();
        let pid = std::process::id();
        let dir1 = tmp.join(format!("tess_explore_contract_topk1_{pid}"));
        let dir5 = tmp.join(format!("tess_explore_contract_topk5_{pid}"));
        let _ = std::fs::remove_dir_all(&dir1);
        let _ = std::fs::remove_dir_all(&dir5);
        let (w1, s1) = explore_write_gallery(&outcome, dir1.to_str().unwrap(), 1)
            .expect("gallery at top-k 1 must succeed");
        let (w5, s5) = explore_write_gallery(&outcome, dir5.to_str().unwrap(), 5)
            .expect("gallery at top-k 5 must succeed");
        assert_eq!(
            w1,
            scored.min(1),
            "docs/USAGE.md line {line_no} (--top-k effect): top-k 1 wrote {w1} SVGs, expected {}",
            scored.min(1)
        );
        assert_eq!(
            w5,
            scored.min(5),
            "docs/USAGE.md line {line_no} (--top-k effect): top-k 5 wrote {w5} SVGs, expected {}",
            scored.min(5)
        );
        assert_eq!(
            s1, refused,
            "docs/USAGE.md line {line_no} (--top-k effect): top-k 1 skipped {s1}, expected {refused} refused rows"
        );
        assert_eq!(
            s5, refused,
            "docs/USAGE.md line {line_no} (--top-k effect): top-k 5 skipped {s5}, expected {refused} refused rows"
        );
        assert!(
            w5 >= w1,
            "docs/USAGE.md line {line_no} (--top-k effect): more top-k must not write fewer SVGs ({w1} vs {w5})"
        );
        // The JSON builder has no top-k input — the JSON bytes are the same
        // across the two gallery writers.
        assert_eq!(
            json,
            leaderboard_json(&outcome),
            "docs/USAGE.md line {line_no} (--top-k effect): JSON must not depend on top-k"
        );
        // Gallery file naming: <substrate>-d<delta>-r<rank>.svg
        let mut names: Vec<String> = std::fs::read_dir(&dir5)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();
        names.sort();
        for n in &names {
            assert!(
                n.starts_with("square-d") && n.ends_with(".svg") && n.contains("-r"),
                "docs/USAGE.md line {line_no} (gallery naming): {n:?} must match <substrate>-d<delta>-r<rank>.svg"
            );
        }
        let _ = std::fs::remove_dir_all(&dir1);
        let _ = std::fs::remove_dir_all(&dir5);
    }

    /// The sort contract is documented as "score ascending, ties by
    /// angle, refused sink with rank 0"; the leaderboard's emitted sort
    /// block must spell the same contract.
    #[test]
    fn usage_sort_contract_wording_matches_leaderboard() {
        let doc = usage_doc();
        let (line_no, line) = anchor_line_for(
            &doc,
            "the sort contract",
            "docs/USAGE.md: sort contract pin",
        );
        assert!(
            line.contains("score ascending")
                && line.contains("ties by angle")
                && line.contains("refused sink with rank 0"),
            "docs/USAGE.md line {line_no} (sort contract): line {line:?} should state score ascending, ties by angle + refused sink with rank 0"
        );
        let reg = tess_tilings::registry();
        let tiling = reg.get("square").expect("square registered");
        let outcome = run_sweep(tiling, &REGION);
        let json = leaderboard_json(&outcome);
        assert!(
            json.contains("\"delta\": \"score asc, delta_deg asc (refused sink, rank 0)\""),
            "docs/USAGE.md line {line_no} (sort contract): JSON sort block must spell score asc, delta_deg asc (refused sink, rank 0), got: {json}"
        );
    }

    // ---- refusal / violation row contract -------------------------------

    /// Refusals and incomplete partitions are documented as named in
    /// `feasibility` (`refused: <reason>` / `incomplete_partition`) and
    /// closure violations as a `closure_violation` finding; the builder
    /// must spell exactly those.
    #[test]
    fn usage_refusal_and_violation_row_contract_matches_builder() {
        let doc = usage_doc();
        let (line_no, line) = anchor_line_for(
            &doc,
            "incomplete_partition",
            "docs/USAGE.md: refusal/violation row contract pin",
        );
        assert!(
            line.contains("refused:"),
            "docs/USAGE.md line {line_no}: line {line:?} should spell the refused refusal format"
        );
        assert!(
            line.contains("incomplete_partition"),
            "docs/USAGE.md line {line_no}: line {line:?} should spell incomplete_partition"
        );
        assert!(
            line.contains("closure_violation"),
            "docs/USAGE.md line {line_no}: line {line:?} should spell closure_violation"
        );

        // δ row with a generation refusal — feasibility spells the reason.
        let mut f = SweepFeatures::default();
        f.generation_message = Some("test-refused-reason");
        let refused_row = DeltaRow {
            substrate_id: "square".to_string(),
            delta_deg: 45.0,
            features: f,
            score: None,
            rank: 0,
        };
        let json = leaderboard_json(&SweepOutcome {
            substrate_id: "square".to_string(),
            delta_rows: vec![refused_row],
            theta_rows: vec![],
            delta_tiles: vec![],
        });
        assert!(
            json.contains("refused: test-refused-reason"),
            "docs/USAGE.md line {line_no} (refusal row contract): refused row must spell the reason, json={json}"
        );

        // δ row with a closure violation — recorded as a closure_violation finding.
        let mut f2 = SweepFeatures::default();
        f2.closure_violation = Some("test-violation".to_string());
        let violation_row = DeltaRow {
            substrate_id: "square".to_string(),
            delta_deg: 90.0,
            features: f2,
            score: Some(0.5),
            rank: 1,
        };
        let json = leaderboard_json(&SweepOutcome {
            substrate_id: "square".to_string(),
            delta_rows: vec![violation_row],
            theta_rows: vec![],
            delta_tiles: vec![],
        });
        assert!(
            json.contains("\"closure_violation\": \"test-violation\""),
            "docs/USAGE.md line {line_no} (violation row contract): violation row must be recorded, json={json}"
        );

        // θ row with an incomplete partition — feasibility spells incomplete_partition.
        let mut tf = ThetaFeatures::default();
        tf.partition_complete = false;
        let theta_row = ThetaRow {
            substrate_id: "hankin-4-8-8-motifs".to_string(),
            theta_offset_deg: 0.0,
            features: tf,
            score: None,
            rank: 0,
        };
        let json = leaderboard_json(&SweepOutcome {
            substrate_id: "hankin-4-8-8-motifs".to_string(),
            delta_rows: vec![],
            theta_rows: vec![theta_row],
            delta_tiles: vec![],
        });
        assert!(
            json.contains("incomplete_partition"),
            "docs/USAGE.md line {line_no} (partition contract): incomplete-partition row must be named, json={json}"
        );
    }
}

// ---------------------------------------------------------------------------
// USAGE.md export-section contract pins (audit IX doc-vs-code made permanent).
// ---------------------------------------------------------------------------
//
// Same discipline as tests_explore_contract: doc claims are extracted from
// docs/USAGE.md — zero-or-one match per anchor, failures naming the file +
// line + claim — and compared against the live parse layer, the SVG writer
// and the tile-budget constant. No CLI process spawn, no interactive TUI;
// purely test-only. Deliberately NOT pinned here:
//   - exact tile counts in the taylor-socolar examples (example-specific
//     narrative, not a contract claim)
//
// `--band-w` "requires `--decoration`" IS pinned
// (usage_band_w_requires_decoration_matches_doc): the doc claim and the
// enforcement are asserted together — parse_export errors with a named parse
// error when `--band-w` is given without `--decoration`.
#[cfg(test)]
mod tests_export_contract {
    use super::{export, parse_export};
    use tess_core::strapwork::Decoration;
    use tess_core::view::ColorMode;
    use tess_tilings::lattice::MAX_TILES;

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    fn usage_doc() -> String {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/USAGE.md");
        std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!(
                "docs/USAGE.md readable from crates/tess-tui ({}): {e}",
                path.display()
            )
        })
    }

    /// The unique byte position of `anchor`; else panic loudly.
    fn unique_anchor(text: &str, anchor: &str, what: &str) -> usize {
        let positions: Vec<usize> = text.match_indices(anchor).map(|(i, _)| i).collect();
        match positions.len() {
            0 => panic!("{what}: anchor missing: {anchor:?}"),
            1 => positions[0],
            n => panic!("{what}: anchor ambiguous ({n} matches): {anchor:?}"),
        }
    }

    /// 1-based line number of the anchor at `pos`.
    fn anchor_line(text: &str, pos: usize) -> usize {
        text[..pos].chars().filter(|&c| c == '\n').count() + 1
    }

    /// Whitespace-normalized single line containing `pos`.
    fn line_region(text: &str, pos: usize) -> String {
        let start = text[..pos].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let end = text[pos..].find('\n').map(|i| pos + i).unwrap_or(text.len());
        text[start..end]
            .chars()
            .map(|c| if c.is_whitespace() { ' ' } else { c })
            .collect()
    }

    /// The parsed digit run immediately following `marker` on `line`.
    fn digits_after(line: &str, marker: &str, what: &str) -> usize {
        let i = line
            .find(marker)
            .unwrap_or_else(|| panic!("{what}: marker {marker:?} missing on the anchor's line"));
        let after = &line[i + marker.len()..];
        let mut digits = String::new();
        for c in after.chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            } else if !digits.is_empty() {
                break;
            }
        }
        digits.parse().unwrap_or_else(|_| panic!("{what}: no digit after {marker:?}"))
    }

    /// The (line number, whitespace-normalized line) of a unique anchor,
    /// with a failure message naming the file + line + claim.
    fn anchor_line_for(text: &str, anchor: &str, claim: &str) -> (usize, String) {
        let pos = unique_anchor(text, anchor, &format!("docs/USAGE.md: {claim}"));
        (anchor_line(text, pos), line_region(text, pos))
    }

    /// Split a table line into trimmed cells; cells[1] = flag, cells[2] =
    /// default, cells[3] = meaning (cells[0] is the empty leading cell).
    fn table_cells(line: &str, what: &str) -> Vec<String> {
        let cells: Vec<String> = line.split('|').map(|c| c.trim().to_string()).collect();
        assert!(
            cells.len() >= 4,
            "{what}: expected flag|default|meaning in {line:?}"
        );
        cells
    }

    /// Resolve a table-row anchor and return (line_no, line, cells).
    fn export_table_row(
        text: &str,
        anchor: &str,
        claim: &str,
    ) -> (usize, String, Vec<String>) {
        let (line_no, line) = anchor_line_for(text, anchor, claim);
        let cells = table_cells(&line, &format!("docs/USAGE.md line {line_no} ({claim})"));
        (line_no, line, cells)
    }

    /// Extract flag names from a flag cell: "`--width` / `--height`" →
    /// ["--width", "--height"]; "`--out`, `-o`" → ["--out", "-o"].
    fn flag_names(cell: &str) -> Vec<&str> {
        let mut out: Vec<&str> = Vec::new();
        for part in cell.split(['/', ',']) {
            let part = part.trim().trim_start_matches('`').trim_end_matches('`').trim();
            if part.is_empty() {
                continue;
            }
            let word = part.split_whitespace().next().unwrap();
            if word.starts_with("--") || word == "-o" {
                out.push(word);
            }
        }
        out
    }

    /// A unique temp SVG path (per test tag), removed on success by the caller.
    fn temp_svg(tag: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("tess_tui_export_contract_{tag}_{}.svg", std::process::id()))
    }

    /// The background rect attributes from a rendered SVG (world units).
    fn background_rect(svg: &str) -> (f64, f64, f64, f64) {
        let i = svg.find("<rect ").unwrap_or_else(|| panic!("SVG has no <rect>: {svg}"));
        let rest = &svg[i..];
        let end = rest.find("/>").unwrap_or(rest.len());
        let tag = &rest[..end];
        let get = |key: &str| -> f64 {
            let needle = format!("{key}=\"");
            let j = tag.find(&needle).unwrap_or_else(|| {
                panic!("rect attribute {key}=\" missing in {tag}")
            });
            let after = &tag[j + needle.len()..];
            let e = after.find('"').expect("rect attribute truncated");
            after[..e].parse().unwrap_or_else(|_| panic!("rect attribute {key} not numeric: {tag}"))
        };
        (get("x"), get("y"), get("width"), get("height"))
    }

    /// Every numeric-looking token in the SVG parses as a finite f64,
    /// and no NaN/inf literal appears anywhere in the document.
    fn all_tokens_finite(svg: &str) -> bool {
        if svg.contains("NaN") || svg.contains("inf") {
            return false;
        }
        let mut tok = String::new();
        let check = |t: &str| -> bool {
            if t.chars().any(|c| c.is_ascii_digit()) {
                match t.parse::<f64>() {
                    Ok(f) => f.is_finite(),
                    Err(_) => false,
                }
            } else {
                true
            }
        };
        for c in svg.chars() {
            if c.is_ascii_digit() || c == '.' || c == '-' || c == '+' || c == 'e' || c == 'E' {
                tok.push(c);
            } else {
                if !tok.is_empty() && !check(&tok) {
                    return false;
                }
                tok.clear();
            }
        }
        if !tok.is_empty() && !check(&tok) {
            return false;
        }
        true
    }

    // ---- flag set -------------------------------------------------------

    /// The export table documents exactly the flags the parse layer accepts;
    /// every documented flag parses; an unknown option errors.
    #[test]
    fn usage_export_flag_set_matches_code() {
        let doc = usage_doc();
        let rows = [
            ("tiling id, see `list` |", "--tiling"),
            ("SVG canvas size in px", "--width"),
            ("uniform scale", "--scale"),
            ("rotation in degrees", "--rotation"),
            ("center the view at world", "--center"),
            ("override a tiling parameter", "--param"),
            ("| `--mode` | pattern |", "--mode"),
            ("Hankin PIC strapwork walls", "--strapwork"),
            ("Islamic parquet deformation", "--deform"),
            ("ornamentation style", "--decoration"),
            ("band width in world units", "--band-w"),
            ("output SVG file", "--out"),
        ];
        let mut documented: Vec<String> = Vec::new();
        for (anchor, _flag) in rows {
            let (line_no, _line, cells) =
                export_table_row(&doc, anchor, "docs/USAGE.md: export flag-set pin");
            let found = flag_names(&cells[1]);
            assert!(
                !found.is_empty(),
                "docs/USAGE.md line {line_no} (flag set): no flag name in cells[1] {:?}",
                cells[1]
            );
            documented.extend(found.into_iter().map(|s| s.to_string()));
        }
        let expected: &[&str] = &[
            "--tiling", "--width", "--height", "--scale", "--rotation", "--center",
            "--param", "--mode", "--strapwork", "--deform", "--decoration",
            "--band-w", "--out", "-o",
        ];
        for flag in expected {
            assert!(
                documented.iter().any(|f| f == flag),
                "docs/USAGE.md: flag {flag:?} missing from the documented set {documented:?}"
            );
        }
        assert_eq!(
            documented.len(),
            expected.len(),
            "docs/USAGE.md: documented flags must match exactly (documented {documented:?} vs expected {expected:?})"
        );

        // Unknown options are named parse errors.
        for other in ["--bogus", "--bogus-two"] {
            let err = parse_export(&args(&["--tiling", "square", other])).unwrap_err();
            assert!(
                err.contains("unknown option"),
                "docs/USAGE.md: unknown option must be a named parse error, got {err:?}"
            );
        }

        // Every documented flag parses successfully (values chosen valid).
        let cases: Vec<(&[&str], &str)> = vec![
            (&["--tiling", "square", "--width", "400", "--height", "300"], "--width/--height"),
            (&["--tiling", "square", "--scale", "2.0"], "--scale"),
            (&["--tiling", "square", "--rotation", "45"], "--rotation"),
            (&["--tiling", "square", "--center", "1,2"], "--center"),
            (&["--tiling", "square", "--param", "scale=2.0"], "--param"),
            (&["--tiling", "square", "--mode", "pattern"], "--mode"),
            (&["--tiling", "square", "--strapwork", "45"], "--strapwork"),
            (&["--tiling", "square", "--deform", "sine", "60", "15", "1"], "--deform"),
            (&["--tiling", "square", "--strapwork", "45", "--decoration", "fill"], "--decoration"),
            (&["--tiling", "square", "--band-w", "0.2", "--strapwork", "45", "--decoration", "fill"], "--band-w"),
            (&["--tiling", "square", "--out", "a.svg"], "--out"),
            (&["--tiling", "square", "-o", "b.svg"], "-o"),
        ];
        for (argv, what) in cases {
            parse_export(&args(argv))
                .unwrap_or_else(|e| panic!("docs/USAGE.md: documented flag {what} must parse: {e}"));
        }

        // --band-w WITHOUT --decoration is a named parse error (the doc's
        // "requires `--decoration`" requirement); it is not silently accepted.
        let err = parse_export(&args(&["--tiling", "square", "--band-w", "0.2"])).unwrap_err();
        assert!(
            err.contains("--band-w requires --decoration"),
            "docs/USAGE.md: --band-w without --decoration must be a named parse error, got {err:?}"
        );
    }

    // ---- requirement claim: --band-w requires --decoration ---------------

    /// The doc states that `--band-w` "requires `--decoration`"; parse_export
    /// enforces it as a named parse error (not silently accepted). Failures
    /// name the file + line + claim.
    #[test]
    fn usage_band_w_requires_decoration_matches_doc() {
        let doc = usage_doc();
        let claim = "docs/USAGE.md: --band-w requires --decoration pin";

        // Doc side: the "(requires `--decoration`)" claim lives on the band-w row.
        let (line_no, _line, cells) = export_table_row(
            &doc,
            "band width in world units",
            claim,
        );
        assert!(
            cells[1].contains("--band-w"),
            "docs/USAGE.md line {line_no} (--band-w row): flag cell {:?} should name --band-w",
            cells[1]
        );
        assert!(
            cells[3].contains("requires `--decoration`"),
            "docs/USAGE.md line {line_no} (--band-w requires --decoration): meaning cell {:?} must claim \"requires `--decoration`\"",
            cells[3]
        );

        // Code side: the claim is enforced — --band-w without --decoration is a
        // named parse error, and the valid combination still parses.
        let err = parse_export(&args(&["--tiling", "square", "--band-w", "0.2"])).unwrap_err();
        assert!(
            err.contains("--band-w requires --decoration"),
            "docs/USAGE.md line {line_no} (--band-w requires --decoration): named parse error expected, got {err:?}"
        );
        parse_export(&args(&["--tiling", "square", "--band-w", "0.2", "--strapwork", "45", "--decoration", "fill"]))
            .unwrap_or_else(|e| panic!("docs/USAGE.md line {line_no} (--band-w requires --decoration): --band-w WITH --decoration must parse: {e}"));
        parse_export(&args(&["--tiling", "square", "--band-w", "0.2", "--deform", "sine", "60", "15", "1", "--decoration", "fill"]))
            .unwrap_or_else(|e| panic!("docs/USAGE.md line {line_no} (--band-w requires --decoration): --band-w WITH --decoration (deform) must parse: {e}"));
    }

    // ---- flags documented as required ------------------------------------

    #[test]
    fn usage_tiling_is_required_matches_export_doc() {
        let doc = usage_doc();
        let (line_no, _line, cells) = export_table_row(
            &doc,
            "tiling id, see `list` |",
            "docs/USAGE.md: --tiling required pin",
        );
        assert!(
            cells[2].contains("required"),
            "docs/USAGE.md line {line_no} (--tiling required): default cell {:?} should say required",
            cells[2]
        );
        let err = parse_export(&args(&["--out", "x"])).unwrap_err();
        assert!(
            err.contains("--tiling"),
            "docs/USAGE.md line {line_no} (--tiling required): named error expected, got {err:?}"
        );
    }

    // ---- documented defaults asserted from parsed values ------------------

    #[test]
    fn usage_export_defaults_match_doc() {
        let doc = usage_doc();
        let defaults = parse_export(&args(&["--tiling", "square"])).expect("defaults parse");

        // --width / --height default 1200 / 800.
        let (line_no, _line, cells) = export_table_row(
            &doc,
            "SVG canvas size in px",
            "docs/USAGE.md: --width/--height default pin",
        );
        let mut it = cells[2].split('/').map(|s| s.trim().parse::<u32>());
        let (dw, dh) = (it.next().unwrap().unwrap(), it.next().unwrap().unwrap());
        assert_eq!(
            defaults.width, dw,
            "docs/USAGE.md line {line_no} (--width default): documented {dw} vs parsed {}",
            defaults.width
        );
        assert_eq!(
            defaults.height, dh,
            "docs/USAGE.md line {line_no} (--height default): documented {dh} vs parsed {}",
            defaults.height
        );

        // --scale default 1.0.
        let (line_no, _line, cells) = export_table_row(
            &doc,
            "uniform scale",
            "docs/USAGE.md: --scale default pin",
        );
        let doc_scale: f64 = cells[2].parse().unwrap_or_else(|_| {
            panic!("docs/USAGE.md line {line_no} (--scale default): cells[2] {:?}", cells[2])
        });
        assert_eq!(
            defaults.scale, doc_scale,
            "docs/USAGE.md line {line_no} (--scale default): documented {doc_scale} vs parsed {}",
            defaults.scale
        );

        // --rotation default 0.
        let (line_no, _line, cells) = export_table_row(
            &doc,
            "rotation in degrees",
            "docs/USAGE.md: --rotation default pin",
        );
        let doc_rot: f64 = cells[2].parse().unwrap_or_else(|_| {
            panic!(
                "docs/USAGE.md line {line_no} (--rotation default): cells[2] {:?}",
                cells[2]
            )
        });
        assert_eq!(
            defaults.rotation, doc_rot,
            "docs/USAGE.md line {line_no} (--rotation default): documented {doc_rot} vs parsed {}",
            defaults.rotation
        );

        // --center default origin (no flag = origin centering).
        let (line_no, _line, cells) = export_table_row(
            &doc,
            "center the view at world",
            "docs/USAGE.md: --center default pin",
        );
        assert!(
            cells[2].contains("origin"),
            "docs/USAGE.md line {line_no} (--center default): default cell {:?} should name origin",
            cells[2]
        );
        assert!(
            defaults.center.is_none(),
            "docs/USAGE.md line {line_no} (--center default): default should be origin (None), got {:?}",
            defaults.center
        );

        // --param default — optional, empty.
        let (line_no, _line, cells) = export_table_row(
            &doc,
            "override a tiling parameter",
            "docs/USAGE.md: --param default pin",
        );
        assert!(
            cells[2].contains("optional"),
            "docs/USAGE.md line {line_no} (--param default): default cell {:?} should say optional",
            cells[2]
        );
        assert!(
            defaults.params.is_empty(),
            "docs/USAGE.md line {line_no} (--param default): default should be empty, got {:?}",
            defaults.params
        );

        // --mode default pattern; the doc lists exactly the five modes the
        // live parser accepts (escaped pipes in the doc mean the raw line
        // is split on " | ", then the meaning cell on " \| ").
        let (line_no, raw_line) = anchor_line_for(
            &doc,
            "| `--mode` | pattern |",
            "docs/USAGE.md: --mode default pin",
        );
        let cells: Vec<&str> = raw_line.split(" | ").collect();
        assert!(
            cells.len() >= 3,
            "docs/USAGE.md line {line_no} (--mode): expected flag|default|meaning in {raw_line:?}"
        );
        assert_eq!(
            cells[1], "pattern",
            "docs/USAGE.md line {line_no} (--mode default): documented default {:?}",
            cells[1]
        );
        assert!(
            matches!(defaults.mode, ColorMode::Pattern),
            "docs/USAGE.md line {line_no} (--mode default): default should be pattern, got {:?}",
            defaults.mode
        );
        let meaning = cells[2].trim_end_matches('|').trim();
        let doc_modes: Vec<&str> = meaning
            .split(" \\| ")
            .map(|c: &str| c.trim().trim_start_matches('`').trim_end_matches('`').trim())
            .filter(|s| !s.is_empty())
            .collect();
        let expected_modes = ["single", "pattern", "proto", "gray", "pattern-gray"];
        assert_eq!(
            doc_modes, expected_modes,
            "docs/USAGE.md line {line_no} (--mode values): documented modes {doc_modes:?}, expected {expected_modes:?}"
        );
        for mode in expected_modes {
            assert!(
                ColorMode::parse(mode).is_some(),
                "docs/USAGE.md line {line_no} (--mode values): mode {mode:?} documented but not parseable"
            );
        }

        // --strapwork default off.
        let (line_no, _line, cells) = export_table_row(
            &doc,
            "Hankin PIC strapwork walls",
            "docs/USAGE.md: --strapwork default pin",
        );
        assert_eq!(
            cells[2], "off",
            "docs/USAGE.md line {line_no} (--strapwork default): documented default {:?}",
            cells[2]
        );
        assert!(
            defaults.strapwork.is_none(),
            "docs/USAGE.md line {line_no} (--strapwork default): default should be off (None), got {:?}",
            defaults.strapwork
        );

        // --deform default off.
        let (line_no, _line, cells) = export_table_row(
            &doc,
            "Islamic parquet deformation",
            "docs/USAGE.md: --deform default pin",
        );
        assert_eq!(
            cells[2], "off",
            "docs/USAGE.md line {line_no} (--deform default): documented default {:?}",
            cells[2]
        );
        assert!(
            defaults.deform.is_none(),
            "docs/USAGE.md line {line_no} (--deform default): default should be off (None), got {:?}",
            defaults.deform
        );

        // --decoration default off. (Deliberately NOT pinning the
        // "requires --decoration" requirement — see the module note.)
        let (line_no, _line, cells) = export_table_row(
            &doc,
            "ornamentation style",
            "docs/USAGE.md: --decoration default pin",
        );
        assert_eq!(
            cells[2], "off",
            "docs/USAGE.md line {line_no} (--decoration default): documented default {:?}",
            cells[2]
        );
        assert!(
            defaults.decoration.is_none(),
            "docs/USAGE.md line {line_no} (--decoration default): default should be off (None), got {:?}",
            defaults.decoration
        );

        // --band-w default 0.1 (the "requires --decoration" requirement is
        // deliberately not pinned here — doc/code mismatch, reported only).
        let (line_no, _line, cells) = export_table_row(
            &doc,
            "band width in world units",
            "docs/USAGE.md: --band-w default pin",
        );
        let doc_band_w: f64 = cells[2].parse().unwrap_or_else(|_| {
            panic!("docs/USAGE.md line {line_no} (--band-w default): cells[2] {:?}", cells[2])
        });
        assert_eq!(
            defaults.band_w, doc_band_w,
            "docs/USAGE.md line {line_no} (--band-w default): documented {doc_band_w} vs parsed {}",
            defaults.band_w
        );

        // --out default tessellix.svg.
        let (line_no, _line, cells) = export_table_row(
            &doc,
            "output SVG file",
            "docs/USAGE.md: --out default pin",
        );
        assert_eq!(
            cells[2], "tessellix.svg",
            "docs/USAGE.md line {line_no} (--out default): documented default {:?}",
            cells[2]
        );
        assert_eq!(
            defaults.out, "tessellix.svg",
            "docs/USAGE.md line {line_no} (--out default): default should be tessellix.svg, got {:?}",
            defaults.out
        );
    }

    // ---- flag behaviors ---------------------------------------------------

    #[test]
    fn usage_param_is_repeatable_matches_doc() {
        let doc = usage_doc();
        let (line_no, line) = anchor_line_for(
            &doc,
            "override a tiling parameter",
            "docs/USAGE.md: --param repeatable pin",
        );
        assert!(
            line.contains("repeatable"),
            "docs/USAGE.md line {line_no} (--param repeatable): claim must say repeatable, line: {line}"
        );
        let a = parse_export(&args(&[
            "--tiling", "square",
            "--param", "scale=2.0",
            "--param", "rotation=90",
        ]))
        .expect("two --param flags parse");
        assert_eq!(a.params.len(), 2, "docs/USAGE.md line {line_no} (--param): two --param flags must each be recorded");
        assert_eq!(a.params[0], ("scale".to_string(), "2.0".to_string()));
        assert_eq!(a.params[1], ("rotation".to_string(), "90".to_string()));
    }

    // ---- the "1.0 → ~20 world units tall" coupling ----------------------

    #[test]
    fn usage_scale_world_units_claim_matches_export() {
        let doc = usage_doc();
        let (line_no, line) = anchor_line_for(
            &doc,
            "world units tall",
            "docs/USAGE.md: --scale world-units pin",
        );
        let doc_h = digits_after(&line, "~", &format!("docs/USAGE.md line {line_no} (world units)"));
        let svg_path = temp_svg("scale_world_units");
        let mut argv = args(&[
            "--tiling", "square", "--width", "1200", "--height", "800",
        ]);
        argv.push("--out".to_string());
        argv.push(svg_path.to_string_lossy().into_owned());
        export(&argv).expect("default export must succeed");
        let svg = std::fs::read_to_string(&svg_path).expect("SVG readable");
        let (_x, _y, w, h) = background_rect(&svg);
        assert!(
            (h - doc_h as f64).abs() < 1e-3,
            "docs/USAGE.md line {line_no} (world units): documented ~{doc_h} world units tall vs emitted height {h}"
        );
        assert!(
            (w / h - 1200.0 / 800.0).abs() < 1e-3,
            "docs/USAGE.md line {line_no} (canvas size in px): SVG ratio {w}x{h} should match 1200/800"
        );
        assert!(
            all_tokens_finite(&svg),
            "docs/USAGE.md line {line_no} (world units): SVG tokens must be finite"
        );
        let _ = std::fs::remove_file(&svg_path);
    }

    // ---- --center: exact translation + finite values ----------------------

    #[test]
    fn usage_center_translation_and_finite_values_match_doc() {
        let doc = usage_doc();
        let (line_no, line) = anchor_line_for(
            &doc,
            "centers the frame at world",
            "docs/USAGE.md: --center translation pin",
        );
        assert!(
            line.contains("translated"),
            "docs/USAGE.md line {line_no} (--center): claim must mention translation, line: {line}"
        );

        // Default-centered vs --center 60,0: the frame is translated exactly
        // by (60, 0); frame size and scale are unchanged.
        let origin = temp_svg("center_origin");
        let centered = temp_svg("center_60_0");
        let base = ["--tiling", "square", "--width", "1200", "--height", "900"];
        let mut argv_a = args(&base);
        argv_a.push("--out".to_string());
        argv_a.push(origin.to_string_lossy().into_owned());
        export(&argv_a).expect("origin-centered export succeeds");
        let mut argv_b = args(&base);
        argv_b.push("--center".to_string());
        argv_b.push("60,0".to_string());
        argv_b.push("--out".to_string());
        argv_b.push(centered.to_string_lossy().into_owned());
        export(&argv_b).expect("--center 60,0 export succeeds");
        let svg_o = std::fs::read_to_string(&origin).expect("origin SVG readable");
        let svg_c = std::fs::read_to_string(&centered).expect("centered SVG readable");
        let (ox, oy, ow, oh) = background_rect(&svg_o);
        let (cx, cy, cw, ch) = background_rect(&svg_c);
        assert!(
            (cx - ox - 60.0).abs() < 1e-3,
            "docs/USAGE.md line {line_no} (--center): rect x should shift exactly by +60 ({ox} → {cx})"
        );
        assert!(
            (cy - oy).abs() < 1e-3,
            "docs/USAGE.md line {line_no} (--center): rect y should be unchanged ({oy} → {cy})"
        );
        assert!(
            (cw - ow).abs() < 1e-3 && (ch - oh).abs() < 1e-3,
            "docs/USAGE.md line {line_no} (--center): frame size must be unchanged ({ow}x{oh} → {cw}x{ch})"
        );
        // The doc's own far-field example: background rect at x = 26.67.
        let example = temp_svg("center_far_field");
        let mut argv_e = args(&["--tiling", "square", "--width", "1200", "--height", "900",
                                 "--scale", "0.4", "--center", "60,0"]);
        argv_e.push("--out".to_string());
        argv_e.push(example.to_string_lossy().into_owned());
        export(&argv_e).expect("far-field --center 60,0 export succeeds");
        let svg_e = std::fs::read_to_string(&example).expect("far-field SVG readable");
        let (ex, _ey, _ew, _eh) = background_rect(&svg_e);
        assert!(
            (ex - 26.67).abs() < 0.05,
            "docs/USAGE.md (far-field rect x): rect should sit at x = 26.67, got {ex}"
        );
        let _ = std::fs::remove_file(&origin);
        let _ = std::fs::remove_file(&centered);
        let _ = std::fs::remove_file(&example);

        // A malformed or non-finite --center is refused, and no SVG is written.
        let nan_path = temp_svg("center_nan");
        let mut argv_nan = args(&["--tiling", "square", "--center", "NaN,0"]);
        argv_nan.push("--out".to_string());
        argv_nan.push(nan_path.to_string_lossy().into_owned());
        let err = export(&argv_nan).expect_err("--center NaN,0 must be refused");
        assert!(
            err.contains("finite"),
            "docs/USAGE.md line {line_no} (--center finite): named parse error expected, got {err:?}"
        );
        assert!(
            !nan_path.exists(),
            "docs/USAGE.md line {line_no} (--center finite): no SVG may be written on a refusal"
        );
    }

    // ---- finite patches beyond their domain ------------------------------

    /// `--center 500,500` on wis-12-fold / golden-bee / girih-decagon-patch
    /// writes a rect-only SVG (one background rect, zero shapes, all tokens
    /// finite, exit 0). The exact tile counts are narrative, not contract.
    #[test]
    fn usage_rect_only_svg_matches_doc() {
        let doc = usage_doc();
        let (line_no, _line) = anchor_line_for(
            &doc,
            "rect-only SVG",
            "docs/USAGE.md: rect-only SVG pin",
        );
        for tiling in ["wis-12-fold", "golden-bee", "girih-decagon-patch"] {
            let svg_path = temp_svg("rect_only");
            let mut argv = args(&[
                "--tiling", tiling,
                "--center", "500,500",
            ]);
            argv.push("--out".to_string());
            argv.push(svg_path.to_string_lossy().into_owned());
            export(&argv)
                .unwrap_or_else(|e| panic!("docs/USAGE.md line {line_no} (rect-only {tiling}): export must succeed, got {e:?}"));
            let svg = std::fs::read_to_string(&svg_path).expect("SVG readable");
            let rects = svg.matches("<rect ").count();
            let paths = svg.matches("<path ").count();
            let lines = svg.matches("<line ").count();
            assert_eq!(
                rects, 1,
                "docs/USAGE.md line {line_no} (rect-only {tiling}): expected exactly one background rect, got {rects}"
            );
            assert_eq!(
                paths, 0,
                "docs/USAGE.md line {line_no} (rect-only {tiling}): expected zero <path> elements, got {paths}"
            );
            assert_eq!(
                lines, 0,
                "docs/USAGE.md line {line_no} (rect-only {tiling}): expected zero <line> elements, got {lines}"
            );
            assert!(
                all_tokens_finite(&svg),
                "docs/USAGE.md line {line_no} (rect-only {tiling}): SVG tokens must be finite"
            );
            let (rx, ry, rw, rh) = background_rect(&svg);
            assert!(
                (rx - (500.0 - rw / 2.0)).abs() < 1e-3 && (ry - (500.0 - rh / 2.0)).abs() < 1e-3,
                "docs/USAGE.md line {line_no} (rect-only {tiling}): rect [{rx},{ry}] {rw}x{rh} should be centered on (500,500)"
            );
            let _ = std::fs::remove_file(&svg_path);
        }
    }

    // ---- rim behavior: partial subset, well-formed, never an error --------

    #[test]
    fn usage_rim_behavior_is_well_formed_never_an_error() {
        let doc = usage_doc();
        let (line_no, _line) = anchor_line_for(
            &doc,
            "still well-formed",
            "docs/USAGE.md: rim behavior pin",
        );
        let svg_path = temp_svg("rim");
        let mut argv = args(&[
            "--tiling", "wis-12-fold",
            "--center", "-14,0",
        ]);
        argv.push("--out".to_string());
        argv.push(svg_path.to_string_lossy().into_owned());
        export(&argv).expect("rim export must succeed (never an error)");
        let svg = std::fs::read_to_string(&svg_path).expect("SVG readable");
        assert!(
            svg.contains("<path"),
            "docs/USAGE.md line {line_no} (rim behavior): rim export must render a partial subset, got {svg}"
        );
        assert!(
            all_tokens_finite(&svg),
            "docs/USAGE.md line {line_no} (rim behavior): SVG tokens must be finite"
        );
        let _ = std::fs::remove_file(&svg_path);
    }

    // ---- tile budget: MAX_TILES constant + named refusal ------------------

    #[test]
    fn usage_max_tiles_budget_matches_doc() {
        let doc = usage_doc();
        let (line_no, _line) = anchor_line_for(
            &doc,
            "Every view is subject to the tile budget",
            "docs/USAGE.md: MAX_TILES pin",
        );
        let pos = doc.match_indices("250,000").next().map(|(i, _)| i).expect("250,000 in doc");
        // The budget constant and the doc's "250,000" must agree.
        let after = &doc[pos..];
        let mut digits = String::new();
        for c in after.chars() {
            if c.is_ascii_digit() || c == ',' {
                digits.push(c);
            } else {
                break;
            }
        }
        let doc_budget: usize = digits.replace(',', "").parse().expect("doc tile budget parses");
        assert_eq!(
            MAX_TILES, doc_budget,
            "docs/USAGE.md line {line_no} (MAX_TILES): documented {doc_budget} vs live constant {MAX_TILES}"
        );

        // Behavior: a view whose window would exceed the budget is refused
        // with the named error and no SVG written.
        let svg_path = temp_svg("max_tiles");
        let mut argv = args(&[
            "--tiling", "square",
            "--scale", "0.01",
        ]);
        argv.push("--out".to_string());
        argv.push(svg_path.to_string_lossy().into_owned());
        let err = export(&argv).expect_err("scale 0.01 must be refused by the tile budget");
        assert!(
            err.contains("too many tiles for this view — zoom in"),
            "docs/USAGE.md (MAX_TILES refusal): named error expected, got {err:?}"
        );
        assert!(
            !svg_path.exists(),
            "docs/USAGE.md (MAX_TILES refusal): no SVG may be written on a refusal"
        );
    }

    // ---- determinism: byte-identical across identical invocations --------

    #[test]
    fn usage_export_is_byte_deterministic_matches_doc() {
        let doc = usage_doc();
        let (line_no, _line) = anchor_line_for(
            &doc,
            "byte-identical across identical invocations",
            "docs/USAGE.md: determinism pin",
        );
        // Two identical invocations must produce identical bytes (the
        // mechanism behind the existing byte-determinism tests).
        let a_path = temp_svg("det_a");
        let b_path = temp_svg("det_b");
        for out in [&a_path, &b_path] {
            let mut argv = args(&["--tiling", "square", "--width", "400", "--height", "300"]);
            argv.push("--out".to_string());
            argv.push(out.to_string_lossy().into_owned());
            export(&argv).expect("determinism run must succeed");
        }
        let bytes_a = std::fs::read(&a_path).expect("SVG A readable");
        let bytes_b = std::fs::read(&b_path).expect("SVG B readable");
        assert_eq!(
            bytes_a, bytes_b,
            "docs/USAGE.md line {line_no} (determinism): identical invocations must be byte-identical"
        );
        let _ = std::fs::remove_file(&a_path);
        let _ = std::fs::remove_file(&b_path);
    }

    // ---- --strapwork/--deform mutual exclusion + the --decoration gate ----

    #[test]
    fn usage_strapwork_deform_exclusion_matches_doc() {
        let doc = usage_doc();
        let (line_no, _line) = anchor_line_for(
            &doc,
            "mutually exclusive with `--strapwork`",
            "docs/USAGE.md: --deform exclusion pin",
        );
        let argv = args(&[
            "--tiling", "square",
            "--strapwork", "45",
            "--deform", "sine", "60", "15", "1",
        ]);
        let err = parse_export(&argv).expect_err("--strapwork + --deform must be refused");
        assert!(
            err.contains("mutually exclusive"),
            "docs/USAGE.md line {line_no} (--deform exclusion): named error expected, got {err:?}"
        );

        // --decoration without an overlay flag is refused (the doc states
        // it requires `--strapwork` or `--deform`, one required).
        let (line_no2, _line2) = anchor_line_for(
            &doc,
            "ornamentation style",
            "docs/USAGE.md: --decoration gate pin",
        );
        let _ = line_no2;
        let argv = args(&["--tiling", "square", "--decoration", "fill"]);
        let err = parse_export(&argv).expect_err("decoration without overlay must be refused");
        assert!(
            err.contains("--decoration"),
            "docs/USAGE.md line {line_no2} (--decoration gate): named error expected, got {err:?}"
        );
    }

    // ---- --decoration styling claims --------------------------------------

    #[test]
    fn usage_decoration_claims_match_doc() {
        let doc = usage_doc();
        let (line_no, line) = anchor_line_for(
            &doc,
            "ignored under `--decoration`",
            "docs/USAGE.md: --mode-ignored-under-decoration pin",
        );
        assert!(
            line.contains("--mode"),
            "docs/USAGE.md line {line_no} (--mode ignored): claim must name --mode, line: {line}"
        );
        // The doc lists exactly the four styles the live parser accepts.
        let (styles_line, styles_line_text, _cells) = export_table_row(
            &doc,
            "ornamentation style",
            "docs/USAGE.md: --decoration styles pin",
        );
        for style in ["fill", "outline", "interlace", "outline-interlace"] {
            assert!(
                Decoration::parse(style).is_some(),
                "docs/USAGE.md line {styles_line} (--decoration styles): style {style:?} documented but not parseable"
            );
        }
        assert!(
            styles_line_text.contains("outline-interlace"),
            "docs/USAGE.md line {styles_line} (--decoration styles): line should list outline-interlace: {styles_line_text}"
        );
        // The --mode-is-ignored claim: the doc promises --mode is ignored
        // under --decoration (behavioral consequence is covered by the
        // existing decoration smoke tests); the claim itself is doc-coupled here.
        let _ = line_no;
    }

    // ---- no z-order claim --------------------------------------------------

    #[test]
    fn usage_no_z_order_claim_documented() {
        let doc = usage_doc();
        for claim in ["z-order", "paint order", "on top"] {
            let n = doc.matches(claim).count();
            assert_eq!(
                n, 0,
                "docs/USAGE.md: unexpected z-order-like claim {claim:?} ({n} match(es))"
            );
        }
    }

    // ---- param bounds/refusal: pins for the resolution entry point --------
    //
    // Registry-wide out-of-range/step/Bool behavior is pinned at the CLI
    // resolution entry point (the `--param` loop inside `export`). The
    // resolution step is parse-only for Float/Angle/Int — it enforces
    // finiteness (Float/Angle) and i64 parsing (Int), never declared bounds
    // or steps — so an out-of-range value flows into the tiling's
    // `generate()`. Only the one tiling with a tiling-level range refusal
    // (glb-binary-n7 `depth`) refuses, and that named refusal is the
    // outermost behavior the CLI surfaces. Nothing here asserts the tiling's
    // own output, which is the tiling-crate's job.

    #[test]
    fn param_bounds_float_angle_step_not_enforced_at_resolution() {
        // scale is 0.1..=50.0 step 0.05; rotation is 0.0..=360.0 step 5.0.
        // All three values below violate the declared bounds or step grid
        // and are accepted by the resolution loop (finiteness/parse only).
        for (param, value) in [
            ("scale", "60"),     // above the 50.0 max
            ("rotation", "720"), // above the 360.0 max
            ("scale", "1.03"),   // off the 0.05 grid (step not enforced)
        ] {
            let out = temp_svg("float_ok");
            let mut argv = args(&[
                "--tiling", "square",
                "--param", &format!("{param}={value}"),
                "--width", "100", "--height", "100",
            ]);
            argv.push("--out".to_string());
            argv.push(out.to_string_lossy().into_owned());
            export(&argv).unwrap_or_else(|e| {
                panic!("out-of-range {param}={value} must be accepted at resolution, got {e:?}")
            });
            assert!(out.exists(), "SVG must be written for {param}={value}");
            let _ = std::fs::remove_file(&out);
        }
    }

    #[test]
    fn param_bounds_float_finiteness_enforced_at_resolution() {
        // NaN and infinity are refused by the finite() helper — the only
        // numeric property the resolution entry point enforces for Float/Angle.
        for (param, value) in [("scale", "nan"), ("rotation", "inf")] {
            let argv = args(&[
                "--tiling", "square",
                "--param", &format!("{param}={value}"),
            ]);
            let err = export(&argv).expect_err("non-finite must be refused");
            assert!(
                err.contains(&format!("value for parameter '{param}' must be a finite number")),
                "error must name the parameter and value: {err}"
            );
        }
    }

    #[test]
    fn param_bounds_bool_accepted_input_set_and_rejection_is_named() {
        // Accepted set, by construction: true|on|1 -> true, false|off|0 -> false.
        for accepted in ["true", "on", "1", "false", "off", "0"] {
            let out = temp_svg("bool_ok");
            let mut argv = args(&[
                "--tiling", "penrose_p2",
                "--param", &format!("star seed={accepted}"),
                "--width", "100", "--height", "100",
            ]);
            argv.push("--out".to_string());
            argv.push(out.to_string_lossy().into_owned());
            export(&argv).unwrap_or_else(|e| {
                panic!("accepted Bool input {accepted:?} must be accepted at resolution, got {e:?}")
            });
            let _ = std::fs::remove_file(&out);
        }
        // Rejects: anything else — named error naming the id and value.
        for rejected in ["yes", "TRUE", "2"] {
            let argv = args(&[
                "--tiling", "penrose_p2",
                "--param", &format!("star seed={rejected}"),
            ]);
            let err = export(&argv).expect_err("non-accepted Bool input must be refused");
            assert_eq!(
                err,
                format!("bad boolean value for parameter 'star seed': {rejected}")
            );
        }
    }

    #[test]
    fn param_bounds_int_out_of_range_refused_by_tiling_not_resolution() {
        // The Int spec (glb-binary-n7 `depth`) is parsed at the resolution
        // entry point with no range check — an out-of-range i64 flows into
        // generate(), which refuses with the named tiling-level refusal.
        let err = export(&args(&[
            "--tiling", "glb-binary-n7",
            "--param", "depth=6", // above the 1..=5 range
            "--width", "100", "--height", "100",
        ]))
        .expect_err("out-of-range depth must be refused by the tiling");
        assert_eq!(err, "unsupported depth — must be 0 (auto) or 1..=5");
        // A non-i64 value is refused at the resolution entry point with the
        // named parse error; no bound is named here.
        let err = export(&args(&[
            "--tiling", "glb-binary-n7",
            "--param", "depth=abc",
            "--width", "100", "--height", "100",
        ]))
        .expect_err("non-i64 depth must be refused at resolution");
        assert_eq!(err, "bad value for parameter 'depth': abc");
    }
}

#[cfg(test)]
mod tests {
    use super::parse_export;

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn decoration_flags_parse() {
        let a = parse_export(&args(&[
            "--tiling",
            "square",
            "--strapwork",
            "45",
            "--decoration",
            "interlace",
            "--band-w",
            "0.2",
        ]))
        .expect("valid decoration flags");
        assert_eq!(a.decoration, Some(tess_core::strapwork::Decoration::Interlace));
        assert_eq!(a.band_w, 0.2);
        assert_eq!(a.strapwork, Some(45.0));
        // Defaults: no decoration, band 0.1.
        let d = parse_export(&args(&["--tiling", "square"])).expect("defaults");
        assert!(d.decoration.is_none());
        assert_eq!(d.band_w, 0.1);
    }

    #[test]
    fn decoration_errors_are_honest_and_pre_generation() {
        let e = parse_export(&args(&["--tiling", "square", "--decoration", "weave"])).unwrap_err();
        assert!(e.contains("bad --decoration"), "{e}");
        assert!(e.contains("fill | outline | interlace | outline-interlace"), "{e}");
        let e = parse_export(&args(&["--tiling", "square", "--decoration", "fill"])).unwrap_err();
        assert!(e.contains("--decoration requires --strapwork"), "{e}");
        let e = parse_export(&args(&[
            "--tiling",
            "square",
            "--strapwork",
            "45",
            "--decoration",
            "fill",
            "--band-w",
            "0",
        ]))
        .unwrap_err();
        assert!(e.contains("--band-w"), "{e}");
        let e = parse_export(&args(&[
            "--tiling",
            "square",
            "--strapwork",
            "45",
            "--decoration",
            "fill",
            "--band-w",
            "nan",
        ]))
        .unwrap_err();
        assert!(e.contains("--band-w"), "{e}");
        // All of these fire inside parse_export — before any registry or
        // filesystem access (unknown tiling `square` never reached).
    }
    // ──────────────────────────────────────────────────────────────────
    // Parquet deformation (22d): --deform parse policy + resolve_deform.
    // ──────────────────────────────────────────────────────────────────

    use super::resolve_deform;
    use tess_core::strapwork::BuiltInProfile;

    #[test]
    fn deform_parse_errors_are_honest_and_pre_generation() {
        // Unknown profile name.
        let e = parse_export(&args(&["--tiling", "square", "--deform", "xyz", "1"])).unwrap_err();
        assert!(e.contains("unknown --deform profile 'xyz'"), "{e}");
        assert!(e.contains("linear | radial | sine"), "{e}");
        // Wrong arity — too few.
        let e = parse_export(&args(&["--tiling", "square", "--deform", "linear", "45", "90", "0"]))
            .unwrap_err();
        assert!(e.contains("--deform linear: expected 4 numeric params"), "{e}");
        assert!(e.contains("a b x0 x1"), "{e}");
        // Wrong arity — too many.
        let e = parse_export(&args(&["--tiling", "square", "--deform", "linear", "45", "90", "0", "1", "2"]))
            .unwrap_err();
        assert!(e.contains("expected 4 numeric params"), "{e}");
        // Bad float.
        let e = parse_export(&args(&["--tiling", "square", "--deform", "linear", "45", "90", "0", "abc"]))
            .unwrap_err();
        assert!(e.contains("bad float 'abc'"), "{e}");
        assert!(e.contains("(expected x1)"), "{e}");
        // Non-finite.
        let e = parse_export(&args(&["--tiling", "square", "--deform", "linear", "45", "90", "0", "nan"]))
            .unwrap_err();
        assert!(e.contains("must be a finite float"), "{e}");
        // Degenerate ramp span.
        let e = parse_export(&args(&["--tiling", "square", "--deform", "linear", "45", "90", "2", "2"]))
            .unwrap_err();
        assert!(e.contains("x0 and x1 must differ"), "{e}");
        let e = parse_export(&args(&[
            "--tiling", "square", "--deform", "radial", "45", "90", "0", "0", "1", "1",
        ]))
        .unwrap_err();
        assert!(e.contains("r0 and r1 must differ"), "{e}");
        // Mutually exclusive with --strapwork.
        let e = parse_export(&args(&[
            "--tiling", "square", "--strapwork", "45", "--deform", "sine", "60", "15", "1",
        ]))
        .unwrap_err();
        assert!(e.contains("mutually exclusive"), "{e}");
        // --decoration + --deform (no --strapwork) parses — the gate accepts
        // either overlay flag.
        let a = parse_export(&args(&[
            "--tiling", "square", "--deform", "sine", "60", "15", "1", "--decoration", "outline",
        ]))
        .expect("--decoration with --deform parses");
        assert_eq!(a.decoration, Some(tess_core::strapwork::Decoration::Outline));
        assert!(a.strapwork.is_none());
    }

    #[test]
    fn resolve_deform_policy() {
        // Unknown substrate: no validated envelope → honest named error.
        let e = resolve_deform(
            BuiltInProfile::Linear { a: 45.0, b: 90.0, x0: 0.0, x1: 1.0 },
            None,
            "unknown-substrate",
        )
        .err()
        .expect("no validated envelope is an error");
        assert!(e.contains("no validated δ-envelope"), "{e}");
        assert!(e.contains("unknown-substrate"), "{e}");

        // Entirely disjoint nominal range → named error, envelope listed.
        let e = resolve_deform(
            BuiltInProfile::Linear { a: 120.0, b: 150.0, x0: 0.0, x1: 1.0 },
            Some(&[45.0, 90.0]),
            "square",
        )
        .err()
        .expect("disjoint range is an error");
        assert!(e.contains("params outside envelope"), "{e}");
        assert!(e.contains("45.0°"), "{e}");
        assert!(e.contains("90.0°"), "{e}");
        assert!(e.contains("outside the validated δ-envelope"), "{e}");

        // Partial overlap: envelope intersects the nominal range → OK,
        // points snap to the nearest closed angle (tie → lower).
        let f = resolve_deform(
            BuiltInProfile::Sine { mid: 75.0, amp: 15.0, k: 1.0 },
            Some(&[60.0, 90.0]),
            "trihexagonal",
        )
        .expect("intersecting envelope");
        assert_eq!(f(tess_core::Vec2::new(0.0, 0.0)), 60.0); // 75° tie → lower
        assert_eq!(f(tess_core::Vec2::new(std::f64::consts::FRAC_PI_2, 0.0)), 90.0); // 90° — 60 vs 90: 30 vs 0

        // Partial overlap, values beyond the interpolation span still snap.
        let f = resolve_deform(
            BuiltInProfile::Linear { a: 45.0, b: 90.0, x0: 0.0, x1: 2.0 },
            Some(&[60.0, 90.0]),
            "trihexagonal",
        )
        .expect("envelope intersects nominal range");
        // At x0: raw 45° → 60 (30 vs 45).
        assert_eq!(f(tess_core::Vec2::new(0.0, 0.0)), 60.0);
        // Mid-span: raw 67.5° → 60 (7.5 vs 22.5).
        assert_eq!(f(tess_core::Vec2::new(1.0, 0.0)), 60.0);
        // Beyond the span (x1 < x): raw 112.5° → 90 (30 vs 22.5).
        assert_eq!(f(tess_core::Vec2::new(3.0, 0.0)), 90.0);
    }

    #[test]
    fn pattern_gray_export_emits_only_gray_fills() {
        // End-to-end through the export path: --mode pattern-gray threads
        // parse_export → render_svg_with_walls → tile_rgb. Every tile fill
        // in the emitted SVG is monochrome and one of PATTERN_GRAY_PALETTE;
        // no hue palette value appears (index-0 light gray is excluded
        // from the hue set — it is deliberately shared). The same tiling
        // exported in pattern mode emits hue fills — the mode is honored
        // through the same code path.
        let tmp = std::env::temp_dir().join(format!("tess_tui_pg_{}.svg", std::process::id()));
        let tmp_pattern = std::env::temp_dir().join(format!("tess_tui_pg_pattern_{}.svg", std::process::id()));
        let base = ["--tiling", "square", "--width", "640", "--height", "640"];
        let mut argv: Vec<String> = base.iter().map(|s| s.to_string()).collect();
        argv.extend([
            "--mode".to_string(),
            "pattern-gray".to_string(),
            "--out".to_string(),
            tmp.to_string_lossy().to_string(),
        ]);
        super::export(&argv).expect("pattern-gray export succeeds");
        let svg = std::fs::read_to_string(&tmp).expect("SVG readable");

        let table = tess_core::paint::PATTERN_GRAY_PALETTE;
        let hue: std::collections::BTreeSet<(u8, u8, u8)> = tess_core::paint::PALETTE
            .iter()
            .skip(1)
            .map(|&c| tess_core::paint::rgb(c))
            .collect();
        let mut gray_fills = std::collections::BTreeSet::new();
        for fragment in svg.split("<path ").skip(1) {
            let Some(i) = fragment.find("fill=\"rgb(") else { continue };
            let rest = &fragment[i + "fill=\"rgb(".len()..];
            let end = rest.find(')').unwrap();
            let v: Vec<u8> = rest[..end]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            assert_eq!(v.len(), 3, "fill must be rgb(r,g,b)");
            assert_eq!(v[0], v[1], "fill not monochrome: {v:?}");
            assert_eq!(v[1], v[2], "fill not monochrome: {v:?}");
            assert!(
                !hue.contains(&(v[0], v[1], v[2])),
                "hue palette value leaked: {v:?}"
            );
            let hex = (v[0] as u32) << 16 | (v[1] as u32) << 8 | v[2] as u32;
            assert!(
                table.contains(&hex),
                "fill {hex:#x} not in PATTERN_GRAY_PALETTE"
            );
            gray_fills.insert(hex);
        }
        assert!(
            gray_fills.len() >= 2,
            "checker-seed square should emit both seed-0 and seed-1 fills: {gray_fills:?}"
        );

        let mut argv_p: Vec<String> = base.iter().map(|s| s.to_string()).collect();
        argv_p.extend([
            "--mode".to_string(),
            "pattern".to_string(),
            "--out".to_string(),
            tmp_pattern.to_string_lossy().to_string(),
        ]);
        super::export(&argv_p).expect("pattern export succeeds");
        let svg_p = std::fs::read_to_string(&tmp_pattern).expect("SVG readable");
        let hue_fills: std::collections::BTreeSet<(u8, u8, u8)> = svg_p
            .split("<path ")
            .skip(1)
            .filter_map(|fragment| {
                let i = fragment.find("fill=\"rgb(")?;
                let rest = &fragment[i + "fill=\"rgb(".len()..];
                let end = rest.find(')').unwrap();
                let v: Vec<u8> = rest[..end]
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                (v.len() == 3).then(|| (v[0], v[1], v[2]))
            })
            .collect();
        assert!(
            hue_fills.iter().any(|&f| f.0 != f.1 || f.1 != f.2),
            "pattern mode must emit the hue palette, not the gray ramp"
        );

        let _ = std::fs::remove_file(&tmp);
        let _ = std::fs::remove_file(&tmp_pattern);
    }

    #[test]
    fn fig_3_7_strip_smoke_is_deterministic() {
        // Canonical Fig. 3.7 shape: square substrate, wide-aspect strip
        // window, linear ramp 45° → 90° across the window (tight stars →
        // open crosses along x).
        let svg_a = std::env::temp_dir().join(format!(
            "tess_tui_smoke_a_{}.svg",
            std::process::id()
        ));
        let svg_b = std::env::temp_dir().join(format!(
            "tess_tui_smoke_b_{}.svg",
            std::process::id()
        ));
        let argv = &[
            "--tiling", "square", "--width", "1600", "--height", "400",
            "--deform", "linear", "45", "90", "-40", "40", "--out",
        ];
        for out in [&svg_a, &svg_b] {
            let mut argv2: Vec<String> = argv.iter().map(|s| s.to_string()).collect();
            argv2.push(out.to_string_lossy().to_string());
            super::export(&argv2).expect("Fig. 3.7 strip export succeeds");
        }
        let svg = std::fs::read_to_string(&svg_a).expect("SVG A readable");
        assert!(svg.contains("<line"), "deformed walls must render as <line> elements");
        let bytes_a = std::fs::read(&svg_a).unwrap();
        let bytes_b = std::fs::read(&svg_b).unwrap();
        assert_eq!(bytes_a, bytes_b, "two runs must produce identical bytes");
        let _ = std::fs::remove_file(&svg_a);
        let _ = std::fs::remove_file(&svg_b);
    }

    #[test]
    fn center_flags_parse() {
        // Default: no flag = origin-centered, no panic.
        let a = parse_export(&args(&["--tiling", "square"])).expect("defaults");
        assert_eq!(a.center, None);
        // Acceptance: world coordinates, negative values, whitespace-free.
        let a = parse_export(&args(&[
            "--tiling",
            "square",
            "--center",
            "60,-17.5",
        ]))
        .expect("valid --center");
        assert_eq!(a.center, Some((60.0, -17.5)));
        // Acceptance: decimal x, zero y.
        let a = parse_export(&args(&[
            "--tiling",
            "square",
            "--center",
            "-33.91,0",
        ]))
        .expect("negative x, zero y");
        assert_eq!(a.center, Some((-33.91, 0.0)));
    }

    #[test]
    fn center_flags_reject_bad_values() {
        // Malformed: missing comma.
        let e = parse_export(&args(&["--tiling", "square", "--center", "60"])).unwrap_err();
        assert!(e.contains("expected X,Y"), "{e}");
        // Malformed: bad x.
        let e = parse_export(&args(&["--tiling", "square", "--center", "abc,0"])).unwrap_err();
        assert!(e.contains("--center x"), "{e}");
        // Malformed: bad y.
        let e = parse_export(&args(&["--tiling", "square", "--center", "0,abc"])).unwrap_err();
        assert!(e.contains("--center y"), "{e}");
        // Non-finite must not flow through (same contract as --scale).
        let e = parse_export(&args(&["--tiling", "square", "--center", "NaN,0"])).unwrap_err();
        assert!(e.contains("must be a finite number"), "{e}");
        let e = parse_export(&args(&["--tiling", "square", "--center", "0,inf"])).unwrap_err();
        assert!(e.contains("must be a finite number"), "{e}");
        // Missing value entirely.
        let e = parse_export(&args(&["--tiling", "square", "--center"])).unwrap_err();
        assert!(e.contains("--center"), "{e}");
    }

    #[test]
    fn center_region_translation_is_byte_consistent() {
        // The region math must translate, not recenter: --center 0,0 is
        // byte-identical to the no-flag default, and --center 60,0 differs.
        let svg_a = std::env::temp_dir().join(format!(
            "tess_tui_center_a_{}.svg",
            std::process::id()
        ));
        let svg_b = std::env::temp_dir().join(format!(
            "tess_tui_center_b_{}.svg",
            std::process::id()
        ));
        let svg_c = std::env::temp_dir().join(format!(
            "tess_tui_center_c_{}.svg",
            std::process::id()
        ));
        let base = &["--tiling", "square", "--width", "400", "--height", "300"];
        let mut argv_a: Vec<String> = base.iter().map(|s| s.to_string()).collect();
        argv_a.push("--out".into());
        argv_a.push(svg_a.to_string_lossy().into());
        super::export(&argv_a).expect("origin-centered export succeeds");
        let mut argv_b: Vec<String> = base.iter().map(|s| s.to_string()).collect();
        argv_b.push("--center".into());
        argv_b.push("0,0".into());
        argv_b.push("--out".into());
        argv_b.push(svg_b.to_string_lossy().into());
        super::export(&argv_b).expect("--center 0,0 export succeeds");
        let mut argv_c: Vec<String> = base.iter().map(|s| s.to_string()).collect();
        argv_c.push("--center".into());
        argv_c.push("60,0".into());
        argv_c.push("--out".into());
        argv_c.push(svg_c.to_string_lossy().into());
        super::export(&argv_c).expect("--center 60,0 export succeeds");
        let bytes_a = std::fs::read(&svg_a).unwrap();
        let bytes_b = std::fs::read(&svg_b).unwrap();
        let bytes_c = std::fs::read(&svg_c).unwrap();
        assert_eq!(bytes_a, bytes_b, "--center 0,0 must be byte-identical to the default");
        assert_ne!(bytes_a, bytes_c, "--center 60,0 must shift the view");
        let _ = std::fs::remove_file(&svg_a);
        let _ = std::fs::remove_file(&svg_b);
        let _ = std::fs::remove_file(&svg_c);
    }
}
