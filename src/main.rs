use crate::app::pixi_app::PixiApp;

mod app;
mod ui;
mod model;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Pixi",
        options,
        Box::new(|_cc| Ok(Box::new(PixiApp::default()))),
    )
}