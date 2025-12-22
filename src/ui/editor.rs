use eframe::egui;
use egui_code_editor::ColorTheme;

#[derive(Default)]
pub struct EditorView {
    text: String,
}

// Gruvbox Dark
const GRUVBOX_DARK: ColorTheme = ColorTheme {
    name: "Gruvbox Dark",
    dark: true,
    bg: "#282828",
    cursor: "#a89984",
    selection: "#504945",
    comments: "#928374",
    functions: "#b8bb26",
    keywords: "#fb4934",
    literals: "#ebdbb2",
    numerics: "#d3869b",
    punctuation: "#fe8019",
    strs: "#8ec07c",
    types: "#fabd2f",
    special: "#83a598",
};

impl EditorView {
    pub fn set_text(&mut self, s: &str) {
        self.text = s.to_string();
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            let available_height = ui.available_height().max(0.0);

            let font_size = 15.0;
            let line_height = font_size * 1.35;
            let rows = ((available_height / line_height).floor() as usize).max(1) + 1;

            egui_code_editor::CodeEditor::default()
                .id_source("code_editor")
                .with_syntax(egui_code_editor::Syntax::rust())
                .with_numlines(true)
                .with_theme(GRUVBOX_DARK)
                .with_fontsize(font_size)
                .with_rows(rows)
                .auto_shrink(false)
                .show(ui, &mut self.text);
        });
    }
}
