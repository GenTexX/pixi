use crate::ui::document::document_workspace::DocumentWorkspace;
use eframe::egui;
use egui_dock::TabViewer;

pub struct PixiDocumentTabViewer<'a> {
    pub documents: &'a mut Vec<DocumentWorkspace>,
}

impl<'a> TabViewer for PixiDocumentTabViewer<'a> {
    type Tab = usize;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        self.documents
            .get(*tab)
            .map(|d| d.document.title.clone())
            .unwrap_or_else(|| "Missing doc".into())
            .into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        if let Some(open) = self.documents.get_mut(*tab) {
            open.ui(ui);
        } else {
            ui.label("Document not found.");
        }
    }
}
