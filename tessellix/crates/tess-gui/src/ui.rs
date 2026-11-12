//! GUI application (egui).

use tess_core::paint;
use tess_core::params::{ParamKind, ParamValue};
use tess_core::view::ColorMode;

pub struct GuiApp {
    pub app: tess_core::App,
    /// Cached side-panel metadata: (tiling id, params fingerprint,
    /// symmetry, periodic basis). Regenerated only when the tiling or its
    /// parameter values change — never per frame — so panel rendering
    /// cannot trigger view-sized tile generation (e.g. Hirschhorn growth).
    meta: Option<Meta>,
}

/// Region-independent metadata shown in the side panel.
struct Meta {
    key: String,
    symmetry: tess_core::SymmetryMeta,
    periodic: Option<[tess_core::Vec2; 2]>,
}

/// Layout fingerprint of a polygon's vertex list: equal exactly when two
/// vertex lists are direct-similar images of each other with the index
/// correspondence i↔i — same polygon up to translation, uniform scale,
/// and rotation, starting at the same structural vertex and traversed
/// the same way. This is the condition under which one triangulation's
/// index triples are valid for the other polygon, so it is what the
/// GUI's per-prototile triangulation cache must key on.
///
/// Computed from the complex ratios `(v_i − v_0) / (v_1 − v_0)` for
/// i ≥ 2 (invariant under translation/rotation/scale; quantized to
/// 1e-6 and FNV-1a-hashed). Cyclic shifts start the ratios at a
/// different vertex and mirrors conjugate them (imaginary part flips),
/// so both get distinct fingerprints — their index triples must not be
/// shared.
fn layout_fingerprint(pts: &[tess_core::Vec2]) -> u64 {
    let quant = |v: f64| -> i64 {
        if v.is_finite() {
            (v * 1.0e6).round() as i64
        } else {
            i64::MAX
        }
    };
    let mut h = 0xcbf2_9ce4_8422_2325_u64;
    let fold = |h: &mut u64, q: i64| {
        for b in q.to_le_bytes() {
            *h = (*h ^ b as u64).wrapping_mul(0x100_0000_01b3);
        }
    };
    fold(&mut h, pts.len() as i64);
    if pts.len() < 3 {
        return h;
    }
    let (ax, ay) = (pts[1].x - pts[0].x, pts[1].y - pts[0].y);
    let den = ax * ax + ay * ay;
    if den <= 0.0 || den.is_nan() {
        // Degenerate first edge (the generators never emit one): fall
        // back to raw quantized coordinates — still deterministic.
        for p in pts {
            fold(&mut h, quant(p.x));
            fold(&mut h, quant(p.y));
        }
        return h;
    }
    for p in &pts[2..] {
        let (bx, by) = (p.x - pts[0].x, p.y - pts[0].y);
        // (b + i·b') / (a + i·a'): real and imaginary parts of the
        // complex quotient — a pure rotation cancels, so rotated copies
        // of one layout hash equal while cyclic shifts do not.
        fold(&mut h, quant((bx * ax + by * ay) / den));
        fold(&mut h, quant((by * ax - bx * ay) / den));
    }
    h
}

impl GuiApp {
    pub fn new() -> Self {
        Self {
            app: tess_core::App::new(tess_tilings::registry(), 60.0),
            meta: None,
        }
    }

    /// The cached metadata for the current tiling + parameters, refreshing
    /// it on a miss. The cache key is the tiling id plus every current
    /// parameter value. The probe region is a tiny box at the origin,
    /// which every generator covers instantly (aperiodic patches grow
    /// from the origin) — so a refresh can never kick off expensive
    /// generation, and panning the view never touches this path.
    fn metadata(&mut self) -> &Meta {
        let mut key = self.app.selected_id().to_string();
        for spec in self.app.tiling().params() {
            let v = self
                .app
                .param_value(spec.id)
                .unwrap_or_else(|| spec.default_value());
            key.push_str(&format!("|{}={:?}", spec.id, v));
        }
        if self.meta.as_ref().is_none_or(|m| m.key != key) {
            let r = self.app.result_region(tess_core::BBox::new(
                tess_core::Vec2::new(-0.5, -0.5),
                tess_core::Vec2::new(0.5, 0.5),
            ));
            self.meta = Some(Meta {
                key,
                symmetry: r.symmetry,
                periodic: r.periodic,
            });
        }
        self.meta.as_ref().unwrap()
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("titlebar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("tessellix");
                ui.separator();
                ui.label(self.app.tiling().name());
            });
        });

        self.draw_side_panel(ctx);
        self.draw_canvas(ctx);
    }
}

impl GuiApp {
    fn draw_side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("panel")
            .exact_width(272.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("filter");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.text_edit_singleline(&mut self.app.search);
                    });
                });
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Tilings");
                    // Owned snapshot of the grouped list: sections in
                    // canonical category order (categories with no
                    // matches are omitted by the grouping helper).
                    // While the search filter is active the matches
                    // render flat — they are usually scattered across
                    // categories, and collapsing sections would
                    // fragment the result list.
                    let searching = !self.app.search.trim().is_empty();
                    let sections: Vec<(tess_core::TilingCategory, Vec<(String, String)>)> = self
                        .app
                        .visible_grouped()
                        .into_iter()
                        .map(|(cat, tilings)| {
                            (
                                cat,
                                tilings
                                    .into_iter()
                                    .map(|t| (t.id().to_string(), t.name().to_string()))
                                    .collect(),
                            )
                        })
                        .collect();
                    let selected_id = self.app.selected_id().to_string();
                    let mut clicked: Option<String> = None;
                    for (cat, tilings) in &sections {
                        let mut entries = |ui: &mut egui::Ui| {
                            for (id, name) in tilings {
                                if ui
                                    .selectable_label(
                                        *id == selected_id,
                                        format!("{name}  ({id})"),
                                    )
                                    .clicked()
                                {
                                    clicked = Some(id.clone());
                                }
                            }
                        };
                        if searching {
                            entries(ui);
                        } else {
                            // Open-state is remembered per section in
                            // egui's memory across frames/switches;
                            // default_open only applies the first time.
                            egui::CollapsingHeader::new(format!("{} ({})", cat, tilings.len()))
                                .default_open(true)
                                .show(ui, entries);
                        }
                    }
                    if let Some(id) = clicked {
                        self.app.select_id(&id);
                    }
                    ui.separator();
                    self.draw_params(ui);
                    ui.separator();
                    ui.label("Color mode");
                    ui.horizontal(|ui| {
                        for m in [ColorMode::Single, ColorMode::Pattern, ColorMode::Proto] {
                            if ui
                                .selectable_label(self.app.color_mode == m, m.label())
                                .clicked()
                            {
                                self.app.color_mode = m;
                            }
                        }
                    });
                    ui.separator();
                    let m = self.metadata();
                    ui.label(format!("wallpaper group: {}", m.symmetry.group));
                    ui.label(format!("lattice: {}", m.symmetry.lattice));
                    if let Some([l1, l2]) = m.periodic {
                        ui.label(format!(
                            "basis: ({:.2},{:.2}) ({:.2},{:.2})",
                            l1.x, l1.y, l2.x, l2.y
                        ));
                    }
                    if let Some(n) = m.symmetry.notes {
                        ui.label(n);
                    }
                });
            });
    }
}

impl GuiApp {
    fn draw_canvas(&mut self, ctx: &egui::Context) {
        let (painter, response, rect) = egui::CentralPanel::default()
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let resp = ui.interact(rect, ui.id().with("canvas"), egui::Sense::drag());
                (ui.painter_at(rect), resp, rect)
            })
            .inner;

        // egui's zoom_delta() is 1.0 when there is no zoom this frame
        // (0.0 is NOT the neutral value — that check would run every frame).
        let z = ctx.input(|i| i.zoom_delta());
        if z != 1.0 {
            let anchor = response.hover_pos().map(|p| {
                self.app.viewport.screen_to_world(
                    (p.x - rect.min.x) as f64,
                    (p.y - rect.min.y) as f64,
                    rect.width() as f64,
                    rect.height() as f64,
                )
            });
            self.app.zoom(z as f64, anchor);
        }
        if response.dragged() {
            let d = response.drag_delta();
            self.app.pan(d.x as f64, d.y as f64);
        }

        let w = rect.width().max(1.0) as u32;
        let h = rect.height().max(1.0) as u32;
        let result = self.app.result(w, h);

        let (br, bg_, bb) = paint::rgb(paint::BORDER);
        let (ar, ag, ab) = paint::rgb(paint::BACKGROUND);
        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(ar, ag, ab));

        if let Some(msg) = result.message {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                msg,
                egui::FontId::proportional(16.0),
                egui::Color32::from_rgb(224, 108, 117),
            );
            return;
        }

        let zoom = self.app.viewport.zoom as f32;
        let center = self.app.viewport.center;
        let to_screen = |p: tess_core::Vec2| {
            egui::pos2(
                rect.center().x + (p.x - center.x) as f32 * zoom,
                rect.center().y - (p.y - center.y) as f32 * zoom,
            )
        };

        let border = egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(br, bg_, bb));
        // One ear-clipped triangulation per (proto, vertex-count, layout)
        // triple: the index triples transfer between two copies of a
        // prototile only when their vertex lists are *directly similar
        // with the index correspondence i↔i* — same polygon up to
        // translation, rotation, and uniform scale, starting at the same
        // structural vertex, traversed the same way. The first two key
        // components handle penrose, whose unpaired boundary triangles
        // (3 vertices) share proto 0/1 with 4-vertex rhombi; reusing a
        // 4-vertex triangulation on a 3-vertex mesh yields an
        // out-of-range index (epaint asserts in Mesh::is_valid → crash),
        // and reusing a 3-vertex one on a rhombus leaves it half
        // unfilled (dark background shows through). The layout
        // fingerprint handles everything else: the chair tiling emits
        // four rotation classes that all share proto 0 and 6 vertices,
        // but whose vertex lists start at different structural corners —
        // reusing one class's triangulation on another overfilled the
        // notch (concave reflex vertex) and the fill bled across the
        // tile walls into the neighboring chairs. The fingerprint is
        // invariant under translation/rotation/scale but changes under
        // cyclic shifts and mirrors, so every genuinely distinct vertex
        // layout gets its own triangulation. (egui's `convex_polygon`
        // fans the fill from vertex 0, which overfills concave tiles
        // such as the star-cross 16-gons; the mesh fill + line-loop
        // outline below is concave-safe.)
        let mut tri_cache: std::collections::HashMap<(u8, usize, u64), Vec<[u32; 3]>> =
            std::collections::HashMap::new();
        for tile in &result.tiles {
            let pts = &tile.poly.pts;
            if pts.len() < 3 {
                continue;
            }
            let screen: Vec<egui::Pos2> = pts.iter().map(|p| to_screen(*p)).collect();
            let (r, g, b) = paint::rgb(paint::tile_rgb(self.app.color_mode, tile));
            let color = egui::Color32::from_rgb(r, g, b);
            let tris = tri_cache
                .entry((tile.proto, pts.len(), layout_fingerprint(pts)))
                .or_insert_with(|| tile.poly.triangulate());
            let mut mesh = egui::Mesh::with_texture(egui::TextureId::default());
            for p in &screen {
                mesh.colored_vertex(*p, color);
            }
            for t in tris {
                mesh.add_triangle(t[0], t[1], t[2]);
            }
            painter.add(egui::Shape::Mesh(mesh));
            painter.add(egui::Shape::closed_line(screen, border));
        }
    }

    fn draw_params(&mut self, ui: &mut egui::Ui) {
        ui.label("Parameters");
        let specs = self.app.tiling().params();
        if specs.is_empty() {
            ui.label("(none)");
            return;
        }
        for spec in specs {
            let cur = self
                .app
                .param_value(spec.id)
                .unwrap_or_else(|| spec.default_value());
            let next = ui
                .horizontal(|ui| {
                    ui.label(spec.label);
                    match spec.kind {
                        ParamKind::Float { min, max, step, .. } => {
                            let mut f = cur.as_f64().unwrap_or(0.0);
                            let changed = ui
                                .add(egui::DragValue::new(&mut f).range(min..=max).speed(step))
                                .changed();
                            changed.then_some(ParamValue::Float(f))
                        }
                        ParamKind::Angle { min, max, step, .. } => {
                            let mut f = cur.as_f64().unwrap_or(0.0);
                            let changed = ui
                                .add(egui::DragValue::new(&mut f).range(min..=max).speed(step))
                                .changed();
                            changed.then_some(ParamValue::Angle(f))
                        }
                        ParamKind::Int { min, max, step, .. } => {
                            let mut i = cur.as_i64().unwrap_or(0);
                            let changed = ui
                                .add(
                                    egui::DragValue::new(&mut i)
                                        .range(min..=max)
                                        .speed(step as f64),
                                )
                                .changed();
                            changed.then_some(ParamValue::Int(i))
                        }
                        ParamKind::Bool { default } => {
                            let mut b = cur.as_bool().unwrap_or(default);
                            let changed = ui.checkbox(&mut b, "").changed();
                            changed.then_some(ParamValue::Bool(b))
                        }
                    }
                })
                .inner;
            if let Some(v) = next {
                self.app.set_param_value(spec.id, v);
            }
        }
    }
}

// The canvas fills each tile with a mesh whose index triples come from
// a triangulation cached per prototile. Penrose is the only tiling that
// emits 3-vertex tiles: a Robinson triangle whose twin was culled at
// the view boundary (penrose.rs culls children per refinement step).
// Such a triangle shares its proto (0/1) with 4-vertex rhombi, so the
// cache must key on the vertex count as well: reusing a 4-vertex
// triangulation on a 3-vertex mesh yields an out-of-range index (the
// tessellator then panics — the crash-on-resize), and reusing a
// 3-vertex one on a rhombus leaves half of it unfilled (the dark
// sliver that reads as "dark-gray tiles" in mono mode).
//
// The metadata tests lock the side-panel contract: the probe region is a
// tiny box at the origin and must produce tiles for every registered
// tiling, and panel metadata must come from the cache, not from
// view-sized generation.
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::layout_fingerprint;

    #[test]
    fn penrose_cache_valid_for_all_tiles() {
        let mut app = tess_core::App::new(tess_tilings::registry(), 60.0);
        app.select_id("penrose");
        let result = app.result(800, 600);
        assert!(result.message.is_none(), "{:?}", result.message);
        let mut cache: HashMap<(u8, usize, u64), Vec<[u32; 3]>> = HashMap::new();
        let mut n3 = 0usize;
        for tile in &result.tiles {
            let n = tile.poly.pts.len();
            if n == 3 {
                n3 += 1;
            }
            let tris = cache
                .entry((tile.proto, n, layout_fingerprint(&tile.poly.pts)))
                .or_insert_with(|| tile.poly.triangulate());
            for [a, b, c] in tris {
                assert!(
                    (*a as usize) < n && (*b as usize) < n && (*c as usize) < n,
                    "proto {} tile with {n} vertices got out-of-range cached index",
                    tile.proto
                );
            }
        }
        assert!(
            n3 > 0,
            "expected 3-vertex boundary triangles in the default penrose view, got 0"
        );
    }

    #[test]
    fn metadata_probe_covers_every_tiling() {
        let mut app = tess_core::App::new(tess_tilings::registry(), 60.0);
        let probe = tess_core::BBox::new(
            tess_core::Vec2::new(-0.5, -0.5),
            tess_core::Vec2::new(0.5, 0.5),
        );
        let ids: Vec<&'static str> = app.registry().iter().map(|t| t.id()).collect();
        for id in ids {
            app.select_id(id);
            let r = app.result_region(probe);
            assert!(
                !r.tiles.is_empty(),
                "{id}: metadata probe region produced no tiles"
            );
        }
    }

    #[test]
    fn metadata_cache_tracks_tiling_switch() {
        let mut gui = super::GuiApp::new();
        let before = gui.metadata().symmetry.group;
        gui.app.select_id("penrose");
        let after = gui.metadata().symmetry.group;
        assert_ne!(before, after, "metadata did not refresh on tiling switch");
    }

    /// The canvas fills each tile from a triangulation cached per
    /// (proto, vertex-count) — the exact cache `draw_canvas` builds.
    /// Simulate it for EVERY registered tiling at the default view and
    /// assert the cached triangulation is complete (n − 2 triangles) and
    /// valid (indices in range). This guards the whole class of
    /// "some tiles render unfilled" bugs: a cache entry computed from a
    /// polygon of a different shape, or a broken triangulation, shows up
    /// here as a wrong triangle count or an out-of-range index.
    /// A cached triangulation is only valid for a tile whose vertex list is
/// directly similar (i↔i) to the tile that seeded the cache entry. The
/// failure mode this guards against — the chair bug — is invisible to
/// index-bounds and triangle-count checks: a fan triangulation computed
/// for one orientation, applied to a rotated copy whose vertex 0 sits
/// at a different structural corner, still yields n−2 in-range
/// triangles but *overfills the concave notch*, and the fill bleeds
/// across the tile walls. Overfill shows up as Σ|triangle areas| >
/// polygon area, so every cache HIT is area-checked here; fresh
/// entries are triangulated from the tile itself and covered by the
/// tess-core triangulate tests.
#[test]
    fn tri_cache_valid_for_all_tilings() {
        let mut app = tess_core::App::new(tess_tilings::registry(), 60.0);
        let ids: Vec<&'static str> = app.registry().iter().map(|t| t.id()).collect();
        for id in ids {
            app.select_id(id);
            let result = app.result(800, 600);
            assert!(result.message.is_none(), "{id}: {:?}", result.message);
            assert!(!result.tiles.is_empty(), "{id}: no tiles at default view");
            let mut cache: HashMap<(u8, usize, u64), Vec<[u32; 3]>> = HashMap::new();
            for tile in &result.tiles {
                let n = tile.poly.pts.len();
                assert!(n >= 3, "{id}: {n}-vertex tile");
                let (tris, fresh) = match cache
                    .entry((tile.proto, n, layout_fingerprint(&tile.poly.pts)))
                {
                    std::collections::hash_map::Entry::Occupied(e) => {
                        (e.into_mut(), false)
                    }
                    std::collections::hash_map::Entry::Vacant(e) => {
                        (e.insert(tile.poly.triangulate()), true)
                    }
                };
                assert_eq!(tris.len(), n - 2, "{id}: {n}-gon triangulation size");
                for [a, b, c] in tris.iter() {
                    assert!(
                        (*a as usize) < n && (*b as usize) < n && (*c as usize) < n,
                        "{id}: out-of-range index for proto {} {n}-gon",
                        tile.proto
                    );
                }
                if !fresh {
                    // Cached reuse: the seeded triangulation must still
                    // cover exactly this tile's polygon.
                    let pa = tile.poly.area();
                    let ta: f64 = tris
                        .iter()
                        .map(|[a, b, c]| {
                            let p = &tile.poly.pts[*a as usize];
                            let q = &tile.poly.pts[*b as usize];
                            let r = &tile.poly.pts[*c as usize];
                            ((q.x - p.x) * (r.y - p.y)
                                - (q.y - p.y) * (r.x - p.x))
                                .abs()
                                * 0.5
                        })
                        .sum();
                    assert!(
                        (ta - pa).abs() <= 1e-6 * pa,
                        "{id}: cached triangulation for proto {} {n}-gon \
                         covers {ta} != polygon {pa} (misaligned vertex \
                         lists — fill would bleed across tile walls)",
                        tile.proto
                    );
                }
            }
        }
    }

    /// The layout fingerprint must distinguish exactly the cases where
    /// sharing a triangulation is valid: rotation/translation copies
    /// hash equal (index correspondence survives), while cyclic shifts
    /// and mirrors hash differently (they do not).
    #[test]
    fn layout_fingerprint_matches_only_aligned_copies() {
        let v2 = |x: f64, y: f64| tess_core::Vec2::new(x, y);
        // A concave L-shape (the chair prototile).
        let a = [
            v2(0.0, 0.0),
            v2(2.0, 0.0),
            v2(2.0, 1.0),
            v2(1.0, 1.0),
            v2(1.0, 2.0),
            v2(0.0, 2.0),
        ];
        // 90° rotation + translation: every vertex maps i↔i — same key.
        let rot: Vec<tess_core::Vec2> =
            a.iter().map(|p| v2(-p.y + 5.0, p.x + 3.0)).collect();
        assert_eq!(
            layout_fingerprint(&a),
            layout_fingerprint(&rot),
            "rotation/translation copies must share a cache entry"
        );
        // Uniform scale: same key.
        let scaled: Vec<tess_core::Vec2> = a.iter().map(|p| v2(p.x * 3.0, p.y * 3.0)).collect();
        assert_eq!(layout_fingerprint(&a), layout_fingerprint(&scaled));
        // Cyclic shift of the same polygon: vertex 0 sits at a different
        // structural corner — must NOT share (this is exactly the chair
        // failure mode).
        let mut shifted = a[1..].to_vec();
        shifted.push(a[0]);
        assert_ne!(
            layout_fingerprint(&a),
            layout_fingerprint(&shifted),
            "cyclically shifted lists must get their own triangulation"
        );
        // Mirror: conjugate ratios — must not share.
        let mirrored: Vec<tess_core::Vec2> = a.iter().map(|p| v2(p.x, -p.y)).collect();
        assert_ne!(layout_fingerprint(&a), layout_fingerprint(&mirrored));
    }
}
