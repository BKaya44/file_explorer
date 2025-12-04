use eframe::egui;

mod app;
mod model;
mod services;
mod ui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 700.0])
            .with_title("File Explorer"),
        ..Default::default()
    };

    eframe::run_native(
        "File Explorer",
        options,
        Box::new(|_cc| Ok(Box::new(app::FileExplorerApp::default()))),
    )
}
