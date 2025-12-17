
use eframe::egui;

use crate::ui::UiCtx;

#[derive(Default)]
pub struct MenuBar;

impl MenuBar {
    pub fn id(&self) -> &'static str {
        "menu_bar"
    }

    pub fn ui(&mut self, ui_ctx: &mut UiCtx<'_>) {
        egui::TopBottomPanel::top(self.id()).show(ui_ctx.egui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    ui.button("New");
                    ui.button("Open…");
                    ui.button("Save");
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Pixi v0.1");
                });
            });
        });
    }
}
