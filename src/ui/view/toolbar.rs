
use egui::Ui;


#[derive(Default)]
pub struct ToolbarView;

impl ToolbarView {
    pub fn id(&self) -> &'static str {
        "toolbar"
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Hello from Toolbar");
    }
}
