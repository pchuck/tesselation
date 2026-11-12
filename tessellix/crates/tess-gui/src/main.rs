//! tessellix GUI (egui/eframe).

mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 640.0])
            .with_title("tessellix"),
        ..Default::default()
    };
    eframe::run_native(
        "tessellix",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(ui::GuiApp::new()))
        }),
    )
}
