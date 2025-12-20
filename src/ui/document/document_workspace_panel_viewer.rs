use egui_dock::TabViewer;

use crate::ui::{
    document::document_workspace::Panel,
    view::{
        canvas::Canvas, color_palette::ColorPaletteView, layers::LayersView, toolbar::ToolbarView,
    },
};

pub struct DocumentWorkspacePanelViewer<'a> {
    pub color_palette: &'a mut ColorPaletteView,
    pub layers: &'a mut LayersView,
    pub toolbar: &'a mut ToolbarView,
    pub canvas: &'a mut Canvas,
}

impl<'a> TabViewer for DocumentWorkspacePanelViewer<'a> {
    type Tab = Panel;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Panel::Palette => self.color_palette.id().into(),
            Panel::Layers => self.layers.id().into(),
            Panel::Toolbar => self.toolbar.id().into(),
            Panel::Canvas => self.canvas.id().into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Panel::Palette => {
                self.color_palette.ui(ui);
            }
            Panel::Layers => {
                self.layers.ui(ui);
            }
            Panel::Toolbar => {
                self.toolbar.ui(ui);
            }
            Panel::Canvas => {
                self.toolbar.ui(ui);
            }
        }
    }
}
