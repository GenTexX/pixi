use crate::{
    app::pixi_document_tab_viewer::PixiDocumentTabViewer,
    ui::{
        UiCtx, doc_id::DocId, document::document_workspace::DocumentWorkspace, view::menu_bar::MenuBar
    },
};
use eframe::egui;
use egui::{Color32, CornerRadius, Frame, Margin, Shadow, Stroke};
use egui_dock::{DockArea, DockState, Style};

pub struct PixiApp {
    menu_bar: MenuBar,

    dock_state: DockState<usize>,

    next_doc_id: u64,
    documents: Vec<DocumentWorkspace>,
}

impl PixiApp {
    pub fn new() -> Self {
        let debug_doc = DocumentWorkspace::new(DocId(0));
        let debug_doc2 = DocumentWorkspace::new(DocId(1));

        Self {
            menu_bar: MenuBar::default(),
            dock_state: DockState::new(vec![0, 1]),
            next_doc_id: 0,
            documents: vec![debug_doc, debug_doc2],
        }
    }
}

impl Default for PixiApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for PixiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut ui_ctx = UiCtx { egui: ctx };
        self.menu_bar.ui(&mut ui_ctx);

        egui::CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                },
                fill: Color32::from_rgb(10, 10, 10),
                stroke: Stroke {
                    width: 1.0,
                    color: Color32::from_rgb(10, 10, 10),
                },
                corner_radius: CornerRadius {
                    nw: 0,
                    ne: 0,
                    sw: 0,
                    se: 0,
                },
                outer_margin: Margin {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                },
                shadow: Shadow::default(),
            })
            .show(ctx, |ui| {
                let mut style = Style::from_egui(ui.style().as_ref());
                style.tab_bar.bg_fill = Color32::from_rgb(20, 20, 20);
                style.tab.tab_body.corner_radius = CornerRadius {
                    nw: 0,
                    ne: 0,
                    sw: 0,
                    se: 0,
                };

                let mut viewer = PixiDocumentTabViewer {
                    documents: &mut self.documents,
                };

                DockArea::new(&mut self.dock_state)
                    .id(egui::Id::new("root_dock"))
                    .style(style)
                    .show_leaf_collapse_buttons(false)
                    .show_inside(ui, &mut viewer);
            });
    }
}
