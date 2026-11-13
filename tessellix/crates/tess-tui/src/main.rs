//! tessellix TUI + CLI.
//!
//! Usage:
//!   tess-tui                       run the TUI
//!   tess-tui list                  list available tilings
//!   tess-tui export --tiling ID    write an SVG of the tiling
//!     --width N --height N --out FILE --scale F --rotation DEG --mode MODE
//!     [--param ID=VALUE ...]       set a tiling parameter (see `list`)

mod app_ui;
mod raster;
mod thumb;

use tess_core::params::{ParamKind, ParamValue};
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

struct ExportArgs {
    tiling: String,
    width: u32,
    height: u32,
    out: String,
    scale: f64,
    rotation: f64,
    mode: ColorMode,
    params: Vec<(String, String)>,
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

fn export(args: &[String]) -> Result<(), String> {
    let mut a = ExportArgs {
        tiling: String::new(),
        width: 1200,
        height: 800,
        out: "tessellix.svg".into(),
        scale: 1.0,
        rotation: 0.0,
        mode: ColorMode::Pattern,
        params: Vec::new(),
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

    let svg = tess_core::svg::render_svg(&result.tiles, &region, a.mode);
    std::fs::write(&a.out, svg).map_err(|e| format!("failed to write {}: {e}", a.out))?;
    println!("wrote {} ({} tiles)", a.out, result.tiles.len());
    Ok(())
}
