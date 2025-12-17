
use egui::Ui;

#[derive(Default)]
pub struct LayersView;

impl LayersView {
    pub fn id(&self) -> &'static str {
        "layers"
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Hello from Layers");
    }
}
