use eframe::egui;
use egui_code_editor::{CodeEditor, ColorTheme, Completer, Syntax};

#[derive(Default)]
pub struct EditorView {
    text: String,
}

// Gruvbox 深色主题
const GRUVBOX_DARK: ColorTheme = ColorTheme {
    name: "Gruvbox Dark",
    dark: true,
    bg: "#282828",          // bg0
    cursor: "#a89984",      // bg4 / gray
    selection: "#504945",   // bg2
    comments: "#928374",    // gray
    functions: "#b8bb26",   // green
    keywords: "#fb4934",    // red
    literals: "#ebdbb2",    // fg1
    numerics: "#d3869b",    // purple
    punctuation: "#fe8019", // orange
    strs: "#8ec07c",        // aqua
    types: "#fabd2f",       // yellow
    special: "#83a598",     // blue
};

// Gruvbox 浅色主题
const GRUVBOX_LIGHT: ColorTheme = ColorTheme {
    name: "Gruvbox Light",
    dark: false,
    bg: "#fbf1c7",          // bg0
    cursor: "#7c6f64",      // gray
    selection: "#b57614",   // yellow (作为选中背景)
    comments: "#7c6f64",    // gray
    functions: "#79740e",   // green
    keywords: "#9d0006",    // red
    literals: "#282828",    // fg
    numerics: "#8f3f71",    // purple
    punctuation: "#af3a03", // orange
    strs: "#427b58",        // aqua
    types: "#b57614",       // yellow
    special: "#076678",     // blue
};

impl EditorView {
    pub fn set_text(&mut self, s: &str) {
        self.text = s.to_string();
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        CodeEditor::default()
            .id_source("code_editor")
            .with_syntax(Syntax::rust())
            .with_numlines(true)
            .with_theme(GRUVBOX_DARK)
            .with_rows(32)
            .show_with_completer(ui, &mut self.text, &mut Completer::default()); // TODO
    }
}
