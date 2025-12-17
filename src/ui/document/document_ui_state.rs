use eframe::egui;
use egui::Vec2;

#[derive(Default)]
pub struct DocumentUiState {
    pub pan: egui::Vec2,
    pub zoom: f32,
}

impl DocumentUiState {
    pub fn new() -> Self {
        Self {
            pan: Vec2::new(0.0, 0.0),
            zoom: 0.0,
        }
    }
}
