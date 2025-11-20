use std::error::Error;

use anyhow::Result;
use eframe::{NativeOptions, run_native};
use esolang_ide::{UiApp, load_config};

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };

    run_native(
        "EsolangIDE",
        options,
        Box::new(|cc| Ok(Box::new(UiApp::new(config, cc)))),
    )?;

    Ok(())
}
