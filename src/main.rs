use anyhow::Result;
use eframe::{NativeOptions, run_native};
use esolang_ide::ui;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };

    run_native(
        "EsolangIDE",
        options,
        Box::new(|cc| Ok(Box::new(ui::UiApp::new(cc)))),
    )
}
