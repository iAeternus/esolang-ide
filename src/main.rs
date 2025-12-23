#![windows_subsystem = "windows"]
use std::error::Error;

use anyhow::Result;
use eframe::{NativeOptions, run_native};
use egui::{FontData, FontDefinitions, FontFamily};
use esolang_ide::{
    MIN_EDITOR_HEIGHT, MIN_LEFT_PANEL_WIDTH, MIN_TERMINAL_HEIGHT, UiApp, load_config,
};

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;

    let min_width = MIN_LEFT_PANEL_WIDTH + 300.0;
    let min_height = 40.0 + MIN_EDITOR_HEIGHT + MIN_TERMINAL_HEIGHT;

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([min_width, min_height]),
        ..Default::default()
    };

    run_native(
        "EsolangIDE",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(UiApp::new(config, cc)))
        }),
    )?;

    Ok(())
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // 中文字体
    fonts.font_data.insert(
        "chinese".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/NotoSansSC-Medium.ttf")).into(),
    );

    // 只把中文字体添加为 fallback 到 monospace（编辑器/终端）
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("chinese".to_owned());

    // 只把中文字体添加为 fallback 到 proportional（UI文字）
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .push("chinese".to_owned());

    ctx.set_fonts(fonts);
}
