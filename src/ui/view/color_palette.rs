
use egui::Ui;

#[derive(Default)]
pub struct ColorPaletteView;

impl ColorPaletteView {
    pub fn id(&self) -> &'static str {
        "color_palette"
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Hello from Palette");
    }
}
