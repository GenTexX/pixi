use eframe::egui;

#[derive(Default)]
pub struct PixiApp {

}

impl eframe::App for PixiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pixi");
            ui.label("Editor skeleton running");
        });
    }
}