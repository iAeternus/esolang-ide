use crate::{
    ExtInterpreterConfig, MIN_LEFT_PANEL_WIDTH, MIN_TERMINAL_HEIGHT,
    core::{
        DebugSession, DebugState, ExternalInterpreter, FromWorkerMsg, Interpreter, RunRequest,
        ToWorkerMsg, WorkerHandle,
    },
    ui::{
        breakpoint::BreakpointPanel, controller::AppController, editor::EditorPanel,
        layout::LayoutState, state::AppState, terminal::TerminalPanel,
    },
};
use eframe::egui;
use egui::{UiBuilder, scroll_area::State};

pub struct UiApp {
    pub state: AppState,
}

impl eframe::App for UiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        AppController::handle_worker_msg(&mut self.state);
        self.show_top_panel(ctx);
        if self.state.layout.show_left_panel {
            self.show_left_panel(ctx);
        }
        self.show_central_panel(ctx);
        ctx.request_repaint();
    }
}

impl UiApp {
    pub fn new(config: ExtInterpreterConfig, _cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: AppState::new(config),
        }
    }

    fn show_left_panel(&mut self, ctx: &egui::Context) {
        let window_width = ctx.input(|i| i.content_rect().width());

        // 最大宽度：保证中央区至少还能放下编辑器
        let max_width = window_width - 300.0; // 给 CentralPanel 留最小空间

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .min_width(MIN_LEFT_PANEL_WIDTH)
            .max_width(max_width.max(MIN_LEFT_PANEL_WIDTH))
            .default_width(self.state.layout.left_panel_width)
            .show(ctx, |ui| {
                let actual = ui.available_width();
                self.state.layout.left_panel_width = actual.clamp(MIN_LEFT_PANEL_WIDTH, max_width);

                ui.vertical(|ui| {
                    self.show_language_selector(ui);
                    ui.separator();
                    self.show_controls(ui);
                    ui.separator();
                    self.show_debug_state(ui);
                    ui.separator();
                    self.show_breakpoints(ui);
                });
            });
    }

    fn show_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();

            // 代码编辑器
            ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                self.state.editor_panel.ui(ui)
            });

            // 不点击按钮终端就不展开
            if !self.state.layout.terminal_visible {
                return;
            }

            // 终端位置
            let min_h = MIN_TERMINAL_HEIGHT;
            let max_h = rect.height() - 40.0;
            self.state.layout.terminal_height =
                self.state.layout.terminal_height.clamp(min_h, max_h);
            let terminal_top = rect.bottom() - self.state.layout.terminal_height;
            let terminal_rect =
                egui::Rect::from_min_max(egui::pos2(rect.left(), terminal_top), rect.max);

            // 拖动热点区域（不可见）
            const HOT_ZONE_H: f32 = 6.0;

            let hot_rect = egui::Rect::from_min_max(
                egui::pos2(rect.left(), terminal_top - HOT_ZONE_H * 0.5),
                egui::pos2(rect.right(), terminal_top + HOT_ZONE_H * 0.5),
            );

            let id = ui.id().with("terminal_top_hotzone");
            let response = ui.interact(hot_rect, id, egui::Sense::drag());

            // 分割线
            if response.hovered() || response.dragged() {
                ui.painter().line_segment(
                    [
                        egui::pos2(rect.left(), terminal_top),
                        egui::pos2(rect.right(), terminal_top),
                    ],
                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                );

                ctx.set_cursor_icon(egui::CursorIcon::ResizeVertical);
            }

            // 拖拽逻辑
            if response.dragged() {
                let delta = response.drag_delta().y;

                let new_height = (self.state.layout.terminal_height - delta).clamp(min_h, max_h);

                // 达到最小或最大时，完全锁死
                if new_height != self.state.layout.terminal_height {
                    self.state.layout.terminal_height = new_height;
                }
            }

            // 终端
            ui.scope_builder(UiBuilder::new().max_rect(terminal_rect), |ui| {
                self.state.terminal_panel.ui(ui)
            });

            if let Some(input) = self.state.terminal_panel.take_input() {
                self.state
                    .terminal_panel
                    .push_output(format!("> {}", input));
                AppController::run_with_input(&self.state, &input);
            }
        });
    }

    fn show_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("☰").clicked() {
                    self.state.layout.show_left_panel = !self.state.layout.show_left_panel;
                }

                ui.heading("EsolangIDE");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 展开终端按钮
                    if ui.button("🖥").clicked() {
                        self.state.layout.terminal_visible = !self.state.layout.terminal_visible;
                    }

                    self.show_load_file_button(ui);
                });
            });
        });
    }

    fn show_load_file_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Load File").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("All files", &["*"])
                .pick_file()
            {
                match std::fs::read_to_string(&path) {
                    Ok(content) => {
                        self.state.editor_panel.set_text(&content);
                        self.state
                            .terminal_panel
                            .push_output(format!("Loaded file: {}", path.display()));
                    }
                    Err(e) => {
                        self.state
                            .terminal_panel
                            .push_output(format!("Error loading file: {}", e));
                    }
                }
            }
        }
    }

    fn show_language_selector(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.label("Language:");

            let old_language = self.state.selected_language.clone();
            let mut current_language = self.state.selected_language.clone();

            let selected_text = current_language
                .as_ref()
                .and_then(|lang_id| {
                    self.state
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
                    for (lang_id, lang_name) in &self.state.available_languages {
                        ui.selectable_value(
                            &mut current_language,
                            Some(lang_id.clone()),
                            lang_name,
                        );
                    }
                });

            if current_language != old_language {
                self.state.selected_language = current_language;
                AppController::update_interpreter(
                    &self.state,
                    self.state.selected_language.as_ref(),
                );
            }
        });
    }

    fn show_controls(&mut self, ui: &mut egui::Ui) {
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

                self.handle_control_clicks(
                    run_clicked,
                    debug_clicked,
                    step_clicked,
                    resume_clicked,
                    stop_clicked,
                );
            });
        });
    }

    fn handle_control_clicks(
        &mut self,
        run_clicked: bool,
        debug_clicked: bool,
        step_clicked: bool,
        resume_clicked: bool,
        stop_clicked: bool,
    ) {
        if run_clicked {
            self.run_code();
        }

        if debug_clicked {
            AppController::start_debug_session(&self.state);
        }

        if step_clicked {
            self.step_debug();
        }

        if resume_clicked {
            self.resume_debug();
        }

        if stop_clicked {
            self.stop_debug();
        }
    }

    fn run_code(&mut self) {
        self.state.terminal_panel.push_output("Please input:");
        self.state.terminal_panel.request_input();
    }

    fn step_debug(&mut self) {
        if let Some(session) = self.state.debug_session.as_mut() {
            match session.step() {
                Ok(state) => {
                    self.state.last_debug_state = Some(state);
                }
                Err(e) => {
                    self.state.last_run_output = format!("Step error: {}", e);
                }
            }
        }
    }

    fn resume_debug(&mut self) {
        if let Some(session) = self.state.debug_session.as_mut() {
            match session.resume_until_breakpoint(&self.state.bp_panel.breakpoints()) {
                Ok(state) => {
                    self.state.last_debug_state = Some(state);
                }
                Err(e) => {
                    self.state.last_run_output = format!("Resume error: {}", e);
                }
            }
        }
    }

    fn stop_debug(&mut self) {
        self.state.debug_session = None;
        self.state.last_debug_state = None;
    }

    fn show_debug_state(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.label("Debug State (JSON)");
            if let Some(state) = &self.state.last_debug_state {
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

    fn show_breakpoints(&mut self, ui: &mut egui::Ui) {
        self.state.bp_panel.ui(ui);
    }
}
