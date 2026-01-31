mod models;
mod storage;
mod app;

use crate::app::NmptmApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "nmptm - Nutrition & Task Management",
        options,
        Box::new(|cc| Ok(Box::new(NmptmApp::new(cc)))),
    )
}

