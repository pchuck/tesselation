//! M203 off-state anchor (design doc `docs/medallion-third-layer-design.md`
//! §4.1/§7 A4): the third-layer additions must be *inert* when unused.
//!
//! Pins, byte-identically on synthetic substrates (tess-core has no
//! registry access — grids are built inline):
//!
//! * `walls` delegating through `walls_layered` with an empty layer is
//!   byte-identical to `walls_with_delta` at a constant angle;
//! * `plan_decoration_medallion(.., None)` is identical to
//!   `plan_decoration` (both the success plans and the error strings);
//! * the §4.4 named-record message variant appears ONLY on the
//!   interlace-style open-end gate, ONLY when the witness lies on the
//!   layer, and ONLY as a suffix — the base messages stay byte-identical.

const S3: f64 = 1.732_050_807_568_877_2;

use tess_core::geometry::{BBox, Polygon, Vec2};
use tess_core::strapwork::{
    plan_decoration, plan_decoration_medallion, walls, walls_layered, walls_with_delta, Decoration,
    MedallionContext, Wall,
};
use tess_core::tiling::Tile;

fn square_grid(n: i32) -> Vec<Tile> {
    let mut out = Vec::new();
    for i in 0..n {
        for j in 0..n {
            let (x, y) = (i as f64, j as f64);
            let poly = Polygon::from_slice(&[
                Vec2::new(x, y),
                Vec2::new(x + 1.0, y),
                Vec2::new(x + 1.0, y + 1.0),
                Vec2::new(x, y + 1.0),
            ]);
            out.push(Tile {
                proto: 0,
                poly,
                seed: ((i + j) % 2) as u32,
                level: 0,
            });
        }
    }
    out
}

fn hex_grid() -> Vec<Tile> {
    let mut out = Vec::new();
    for col in -3..=3i32 {
        for row in -3..=3i32 {
            let cx = col as f64 * 1.5;
            let cy = row as f64 * S3 + if col % 2 != 0 { S3 / 2.0 } else { 0.0 };
            let pts: Vec<Vec2> = (0..6)
                .map(|k| {
                    let a = std::f64::consts::FRAC_PI_3 * k as f64;
                    Vec2::new(cx + a.cos(), cy + a.sin())
                })
                .collect();
            out.push(Tile {
                proto: 0,
                poly: Polygon::from_slice(&pts),
                seed: ((col + row).rem_euclid(2)) as u32,
                level: 0,
            });
        }
    }
    out
}

fn triangle_grid(n: i32) -> Vec<Tile> {
    let mut out = Vec::new();
    for i in 0..n {
        for j in 0..n {
            let (x, y) = (i as f64, j as f64);
            let up = Polygon::from_slice(&[
                Vec2::new(x, y),
                Vec2::new(x + 1.0, y),
                Vec2::new(x + 0.5, y + S3 / 2.0),
            ]);
            let dn = Polygon::from_slice(&[
                Vec2::new(x + 1.0, y),
                Vec2::new(x + 1.5, y + S3 / 2.0),
                Vec2::new(x + 0.5, y + S3 / 2.0),
            ]);
            out.push(Tile {
                proto: 0,
                poly: up,
                seed: 0,
                level: 0,
            });
            out.push(Tile {
                proto: 1,
                poly: dn,
                seed: 1,
                level: 0,
            });
        }
    }
    out
}

fn substrates() -> Vec<(&'static str, Vec<Tile>)> {
    vec![
        ("squares", square_grid(6)),
        ("hexes", hex_grid()),
        ("triangles", triangle_grid(5)),
    ]
}

/// `walls` → `walls_layered(empty)` → `walls_with_delta(const)` is the
/// byte-identity chain: every link compares bitwise (`Wall: PartialEq`
/// over the raw f64s).
#[test]
fn walls_delegation_is_byte_identical() {
    for (name, tiles) in substrates() {
        for delta in [11.37f64, 30.0, 45.0, 60.0, 72.0] {
            let via_layered = walls_layered(&tiles, delta, &[]);
            let via_delta = walls_with_delta(&tiles, |_| delta);
            let plain = walls(&tiles, delta);
            assert_eq!(plain, via_layered, "{name}@{delta}: walls vs walls_layered(empty)");
            assert_eq!(plain, via_delta, "{name}@{delta}: walls vs walls_with_delta");
            // Bit-level too: NaN-free f64s compare through to_bits.
            let bits = |ws: &Vec<Wall>| {
                ws.iter()
                    .flat_map(|w| [w.a.x.to_bits(), w.a.y.to_bits(), w.b.x.to_bits(), w.b.y.to_bits()])
                    .collect::<Vec<u64>>()
            };
            assert_eq!(bits(&plain), bits(&via_layered), "{name}@{delta}: bits");
        }
    }
}

/// The gate planner with no medallion context is identical to the
/// plain planner — identical plans on the passing angles and
/// byte-identical error strings on the failing ones.
#[test]
fn plan_decoration_medallion_none_is_plan_decoration() {
    let region = BBox::new(Vec2::new(-1.0, -1.0), Vec2::new(5.5, 5.5));
    for (name, tiles) in substrates() {
        for delta in [30.0f64, 45.0, 60.0] {
            let ws = walls(&tiles, delta);
            for style in [
                Decoration::Fill,
                Decoration::Outline,
                Decoration::Interlace,
                Decoration::OutlineInterlace,
            ] {
                let plain = plan_decoration(&ws, 0.15, style, &region);
                let delegated = plan_decoration_medallion(&ws, 0.15, style, &region, None);
                match (plain, delegated) {
                    (Ok(a), Ok(b)) => assert_eq!(a, b, "{name}@{delta} {style:?}"),
                    (Err(a), Err(b)) => assert_eq!(
                        a.to_string(),
                        b.to_string(),
                        "{name}@{delta} {style:?} error strings"
                    ),
                    _ => panic!("{name}@{delta} {style:?}: ok/err divergence under None"),
                }
            }
        }
    }
}

/// The §4.4 variant: a tiny open map whose witness (0, 0) is exactly
/// on / off the layer geometry, and the fill gate that must stay
/// unnamed even when the witness lies on the layer (design §4.4: the
/// odd-degree message is "unchanged code"; only the open-end gate
/// gains the variant).
#[test]
fn named_variant_fires_only_on_open_end_witness_on_layer() {
    let ws = vec![
        Wall {
            a: Vec2::new(0.0, 0.0),
            b: Vec2::new(1.0, 0.0),
        },
        Wall {
            a: Vec2::new(0.0, 0.0),
            b: Vec2::new(0.0, 1.0),
        },
    ];
    let region = BBox::new(Vec2::new(-0.5, -0.5), Vec2::new(1.5, 1.5));
    let on_layer = MedallionContext {
        record: "TEST-RECORD",
        absorption: "TEST-PATH",
        suppressed: &[],
        extra: &[Wall {
            a: Vec2::new(0.0, 0.0),
            b: Vec2::new(0.5, 0.5),
        }],
    };
    let off_layer = MedallionContext {
        record: "TEST-RECORD",
        absorption: "TEST-PATH",
        suppressed: &[],
        extra: &[Wall {
            a: Vec2::new(7.0, 7.0),
            b: Vec2::new(8.0, 8.0),
        }],
    };
    let plain = plan_decoration(&ws, 0.1, Decoration::Interlace, &region)
        .expect_err("two open ends must fail the weave gate");
    let plain_msg = plain.to_string();

    let named = plan_decoration_medallion(&ws, 0.1, Decoration::Interlace, &region, Some(&on_layer))
        .expect_err("still open ends");
    let named_msg = named.to_string();
    assert!(
        named_msg.starts_with(&plain_msg),
        "named message must extend the byte-identical base message"
    );
    assert!(named_msg.contains("TEST-RECORD"), "{named_msg}");
    assert!(named_msg.contains("TEST-PATH"), "{named_msg}");
    assert!(named_msg.contains("law (L)"), "{named_msg}");

    let uninvolved =
        plan_decoration_medallion(&ws, 0.1, Decoration::Interlace, &region, Some(&off_layer))
            .expect_err("still open ends");
    assert_eq!(
        uninvolved.to_string(),
        plain_msg,
        "witness off the layer must keep the plain message byte-identically"
    );

    // The suppressed-hull-rim route: the witness (0, 0) lies on the
    // rim edge of a suppressed tile.
    let rim_tile = Tile {
        proto: 0,
        poly: Polygon::from_slice(&[
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(0.0, 2.0),
        ]),
        seed: 9,
        level: 0,
    };
    let rim_ctx = MedallionContext {
        record: "RIM-RECORD",
        absorption: "RIM-PATH",
        suppressed: &[rim_tile],
        extra: &[],
    };
    let named_rim =
        plan_decoration_medallion(&ws, 0.1, Decoration::Interlace, &region, Some(&rim_ctx))
            .expect_err("still open ends");
    assert!(
        named_rim.to_string().contains("RIM-RECORD"),
        "a witness on a suppressed hull rim must name the record"
    );

    // The fill gate stays byte-identical even with the witness on the
    // layer (§4.4: "odd-degree witness error, unchanged code").
    let fill_plain = plan_decoration(&ws, 0.1, Decoration::Fill, &region)
        .err()
        .map(|e| e.to_string());
    let fill_med = plan_decoration_medallion(&ws, 0.1, Decoration::Fill, &region, Some(&on_layer))
        .err()
        .map(|e| e.to_string());
    assert_eq!(fill_plain, fill_med, "fill gate messages must not diverge");
    if let Some(msg) = fill_med {
        assert!(!msg.contains("medallion"), "fill must stay unnamed: {msg}");
    }
}
