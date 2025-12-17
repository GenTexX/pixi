use eframe::egui;

pub mod view;
pub mod document;
pub mod doc_id;

pub struct UiCtx<'a> {
    pub egui: &'a egui::Context,
}