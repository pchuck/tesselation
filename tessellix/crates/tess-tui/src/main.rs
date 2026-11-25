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
        Some(other) => Err(format!(
            "unknown command: {other}\nusage: tess-tui [list | export ...]"
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
            "--band-w" => a.band_w = finite(&next(&mut i)?, "--band-w")?,
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
