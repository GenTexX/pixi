use egui::Ui;


#[derive(Default)]
pub struct Canvas;

impl Canvas {
    pub fn id(&self) -> &'static str {
        "canvas"
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Hello from Canvas!");
    }
}

