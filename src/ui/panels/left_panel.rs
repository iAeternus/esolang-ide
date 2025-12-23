use crate::{
    MIN_LEFT_PANEL_WIDTH,
    ui::{controller::AppController, state::AppState},
};

pub struct LeftPanel;

impl LeftPanel {
    pub fn ui(ctx: &egui::Context, state: &mut AppState) {
        let window_width = ctx.input(|i| i.content_rect().width());

        // 最大宽度：保证中央区至少还能放下编辑器
        let max_width = window_width - 300.0; // 给 CentralPanel 留最小空间

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .min_width(MIN_LEFT_PANEL_WIDTH)
            .max_width(max_width.max(MIN_LEFT_PANEL_WIDTH))
            .default_width(state.layout.left_panel_width)
            .show(ctx, |ui| {
                let actual = ui.available_width();
                state.layout.left_panel_width = actual.clamp(MIN_LEFT_PANEL_WIDTH, max_width);

                ui.vertical(|ui| {
                    draw_language_selector(ui, state);
                    ui.separator();
                    draw_controls(ui, state);
                    ui.separator();
                    draw_debug_state(ui, state);
                    ui.separator();
                    draw_breakpoints(ui, state);
                });
            });
    }
}

fn draw_language_selector(ui: &mut egui::Ui, state: &mut AppState) {
    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label("Language:");

        let old_language = state.selected_language.clone();
        let mut current_language = state.selected_language.clone();

        let selected_text = current_language
            .as_ref()
            .and_then(|lang_id| {
                state
                    .available_languages
                    .iter()
                    .find(|(id, _)| id == lang_id)
                    .map(|(_, name)| name.clone())
            })
            .unwrap_or_else(|| "Select language".to_string());

        egui::ComboBox::from_id_salt("language_selector")
            .width(150.0)
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                for (lang_id, lang_name) in &state.available_languages {
                    ui.selectable_value(&mut current_language, Some(lang_id.clone()), lang_name);
                }
            });

        if current_language != old_language {
            state.selected_language = current_language;
            AppController::update_interpreter(&state, state.selected_language.as_ref());
        }
    });
}

fn draw_controls(ui: &mut egui::Ui, state: &mut AppState) {
    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label("Controls");
        ui.vertical(|ui| {
            let run_clicked = ui
                .add_sized([ui.available_width(), 30.0], egui::Button::new("Run"))
                .clicked();
            let debug_clicked = ui
                .add_sized(
                    [ui.available_width(), 30.0],
                    egui::Button::new("Start Debug"),
                )
                .clicked();
            let step_clicked = ui
                .add_sized([ui.available_width(), 30.0], egui::Button::new("Step"))
                .clicked();
            let resume_clicked = ui
                .add_sized([ui.available_width(), 30.0], egui::Button::new("Resume"))
                .clicked();
            let stop_clicked = ui
                .add_sized(
                    [ui.available_width(), 30.0],
                    egui::Button::new("Stop Debug"),
                )
                .clicked();

            handle_control_clicks(
                run_clicked,
                debug_clicked,
                step_clicked,
                resume_clicked,
                stop_clicked,
                state,
            );
        });
    });
}

fn handle_control_clicks(
    run_clicked: bool,
    debug_clicked: bool,
    step_clicked: bool,
    resume_clicked: bool,
    stop_clicked: bool,
    state: &mut AppState,
) {
    if run_clicked {
        run_code(state);
    }

    if debug_clicked {
        AppController::start_debug_session(state);
    }

    if step_clicked {
        step_debug(state);
    }

    if resume_clicked {
        resume_debug(state);
    }

    if stop_clicked {
        stop_debug(state);
    }
}

fn run_code(state: &mut AppState) {
    state.terminal_panel.push_output("Please input:");
    state.terminal_panel.request_input();
}

fn step_debug(state: &mut AppState) {
    if let Some(session) = state.debug_session.as_mut() {
        match session.step() {
            Ok(debug_state) => {
                state.last_debug_state = Some(debug_state);
            }
            Err(e) => {
                state.last_run_output = format!("Step error: {}", e);
            }
        }
    }
}

fn resume_debug(state: &mut AppState) {
    if let Some(session) = state.debug_session.as_mut() {
        match session.resume_until_breakpoint(&state.bp_panel.breakpoints()) {
            Ok(debug_state) => {
                state.last_debug_state = Some(debug_state);
            }
            Err(e) => {
                state.last_run_output = format!("Resume error: {}", e);
            }
        }
    }
}

fn stop_debug(state: &mut AppState) {
    state.debug_session = None;
    state.last_debug_state = None;
}

fn draw_debug_state(ui: &mut egui::Ui, state: &mut AppState) {
    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label("Debug State (JSON)");
        if let Some(state) = &state.last_debug_state {
            let mut s = serde_json::to_string_pretty(&state.info)
                .unwrap_or_else(|_| "<failed to serialize>".to_string());
            ui.add_sized(
                [ui.available_width(), 200.0],
                egui::TextEdit::multiline(&mut s),
            );
        } else {
            ui.label("No debug state");
        }
    });
}

fn draw_breakpoints(ui: &mut egui::Ui, state: &mut AppState) {
    state.bp_panel.ui(ui);
}
