use anyhow::Result;
use eframe::{NativeOptions, run_native};
use esolang_ide::ui;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), eframe::Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

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
