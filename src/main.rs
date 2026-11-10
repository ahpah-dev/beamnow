#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod agent;
mod app;
mod catalog;
mod config;
mod llm;
mod session;
mod theme;
mod tools;
mod views;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("BeamNow")
            .with_inner_size([960.0, 720.0])
            .with_min_inner_size([520.0, 420.0]),
        ..Default::default()
    };
    eframe::run_native(
        "BeamNow",
        options,
        Box::new(|cc| Ok(Box::new(app::BeamApp::new(cc)))),
    )
}
