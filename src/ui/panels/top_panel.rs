use crate::ui::state::AppState;

pub struct TopPanel;

impl TopPanel {
    pub fn ui(state: &mut AppState, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("☰").clicked() {
                    state.layout.show_left_panel = !state.layout.show_left_panel;
                }

                ui.heading("EsolangIDE");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 展开终端按钮
                    if ui.button("🖥").clicked() {
                        state.layout.terminal_visible = !state.layout.terminal_visible;
                    }

                    draw_load_file_button(state, ui);
                });
            });
        });
    }
}

fn draw_load_file_button(state: &mut AppState, ui: &mut egui::Ui) {
    if ui.button("Load File").clicked() {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("All files", &["*"])
            .pick_file()
        {
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    state.editor_panel.set_text(&content);
                    state
                        .terminal_panel
                        .push_output(format!("Loaded file: {}", path.display()));
                }
                Err(e) => {
                    state
                        .terminal_panel
                        .push_output(format!("Error loading file: {}", e));
                }
            }
        }
    }
}
