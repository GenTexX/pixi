use egui::Ui;
use egui_dock::{DockArea, DockState, Style};

use crate::{
    model::document::Document,
    ui::{
        doc_id::{DocId}, document::{
            document_ui_state::DocumentUiState,
            document_workspace_panel_viewer::DocumentWorkspacePanelViewer,
        }, view::{color_palette::ColorPaletteView, layers::LayersView, toolbar::ToolbarView}
    },
};

pub struct DocumentWorkspace {
    pub state: DocumentUiState,
    pub document: Document,
    dock_state: DockState<Panel>,
    doc_id: DocId,
    color_palette: ColorPaletteView,
    layers: LayersView,
    toolbar: ToolbarView,
}

pub enum Panel {
    Palette,
    Layers,
    Toolbar,
}

impl DocumentWorkspace {
    pub fn new(doc_id: DocId) -> Self {
        Self {
            state: DocumentUiState::new(),
            dock_state: DockState::new(vec![Panel::Palette, Panel::Layers, Panel::Toolbar]),
            doc_id: doc_id,
            document: Document::untitled(),
            color_palette: ColorPaletteView::default(),
            layers: LayersView::default(),
            toolbar: ToolbarView::default(),
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let unique = self as *const _ as usize;

        ui.push_id(unique, |ui| {
            let mut style = Style::from_egui(ui.style().as_ref());
            style.tab_bar.bg_fill = egui::Color32::from_rgb(20, 20, 20);
            style.tab.tab_body.corner_radius = egui::CornerRadius::ZERO;
            
            let dock_id = egui::Id::new(("workspace_dock", u64::from(self.doc_id)));
            
            let mut viewer = DocumentWorkspacePanelViewer {
                color_palette: &mut self.color_palette,
                layers: &mut self.layers,
                toolbar: &mut self.toolbar,
            };

            DockArea::new(&mut self.dock_state)
                .id(egui::Id::new(dock_id))
                .style(style)
                .show_leaf_collapse_buttons(false)
                .show_leaf_close_all_buttons(false)
                .show_add_popup(true)
                .show_close_buttons(false)
                .show_inside(ui, &mut viewer);
        });
    }
    
}
