use crate::data::{SearchCriteria, Section, SectionCollection};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Default)]
pub struct Scheduler {
    query: String,
    section_collection: SectionCollection,
}

impl Scheduler {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let file_path = PathBuf::from("japan").join("test.json");
        let file = File::open(file_path).expect("File did not open");
        let reader = BufReader::new(file);
        let sections: Vec<Section> = serde_json::from_reader(reader).unwrap();
        Scheduler {
            section_collection: SectionCollection::new(sections),
            ..Self::default()
        }
    }
}

impl eframe::App for Scheduler {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Course search: ");
                ui.text_edit_singleline(&mut self.query);
            });

            let criteria = SearchCriteria {
                query: Some(self.query.clone()),
                ..SearchCriteria::default()
            };

            self.section_collection.search(criteria);

            for section in self.section_collection.get_running_sections() {
                ui.horizontal(|ui| {
                    ui.label(&section.to_string());
                });
            }
        });
    }
}
