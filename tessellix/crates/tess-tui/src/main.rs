//! tessellix TUI + CLI.
//!
//! Usage:
//!   tess-tui                       run the TUI
//!   tess-tui list                  list available tilings
//!   tess-tui export --tiling ID    write an SVG of the tiling
//!     --width N --height N --out FILE --scale F --rotation DEG --mode MODE
//!     [--param ID=VALUE ...]       set a tiling parameter (see `list`)
//!     [--strapwork DEG]            overlay the Hankin PIC strapwork walls
//!                                  at contact angle DEG on the tiling
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

use tess_core::params::{ParamKind, ParamValue};
use tess_core::strapwork::Decoration;
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
    decoration: Option<Decoration>,
    band_w: f64,
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
        decoration: None,
        band_w: 0.1,
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
    if a.decoration.is_some() && a.strapwork.is_none() {
        return Err(
            "--decoration requires --strapwork DEG (the decoration decorates the strapwork walls)"
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

    // View centered at the origin, ~20 world units tall at scale 1.
    let world_h = 20.0 / a.scale.max(1e-9);
    let world_w = world_h * a.width as f64 / a.height as f64;
    let region = tess_core::BBox::new(
        tess_core::Vec2::new(-world_w / 2.0, -world_h / 2.0),
        tess_core::Vec2::new(world_w / 2.0, world_h / 2.0),
    );
    let result = tiling.generate(&params, &tess_core::Query { region });
    if let Some(msg) = result.message {
        return Err(msg.into());
    }

    // Hankin PIC strapwork overlay (M57 layer mode): walls at the given
    // contact angle over whatever tiles were generated. Absent flag = no
    // overlay, and the SVG is byte-identical to the pre-M57 export.
    let walls = match a.strapwork {
        Some(delta) => tess_core::strapwork::walls(&result.tiles, delta),
        None => Vec::new(),
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
}
