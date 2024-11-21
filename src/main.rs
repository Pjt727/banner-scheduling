#![allow(dead_code)]
mod banner;
mod data;
mod db;
mod ui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "egui",
        native_options,
        Box::new(|cc| Ok(Box::new(ui::Scheduler::new(cc)))),
    );
}
