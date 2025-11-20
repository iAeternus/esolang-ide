mod breakpoint;
mod editor;
mod terminal;

use crate::{
    ExtInterpreterConfig,
    core::{DebugSession, DebugState, ExternalInterpreter, Interpreter, RunRequest},
};
use eframe::egui;
use std::sync::{Arc, Mutex};

use breakpoint::BreakpointManager;
use editor::EditorView;
use terminal::Terminal;

pub struct UiApp {
    editor: EditorView,
    interpreter: Arc<Mutex<Box<dyn Interpreter>>>,
    last_run_output: String,
    last_debug_state: Option<DebugState>,
    debug_session: Option<Box<dyn DebugSession>>,
    bp_manager: BreakpointManager,
    terminal: Terminal,
    config: ExtInterpreterConfig,
    available_languages: Vec<(String, String)>,
    selected_language: Option<String>,
}

impl eframe::App for UiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_top_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_left_panel(ui);
            self.show_right_panel(ui);
        });
    }
}

impl UiApp {
    pub fn new(config: ExtInterpreterConfig, _cc: &eframe::CreationContext<'_>) -> Self {
        let available_languages = config.available_languages();
        let default_language = available_languages.get(0).map(|(id, _)| id.clone());
        let interpreter = Self::create_interpreter(&config, default_language.as_ref());

        Self {
            editor: EditorView::default(),
            interpreter: Arc::new(Mutex::new(interpreter)),
            last_run_output: String::new(),
            last_debug_state: None,
            debug_session: None,
            bp_manager: BreakpointManager::new(),
            terminal: Terminal::new(),
            config: config.clone(),
            available_languages,
            selected_language: default_language,
        }
    }

    fn show_left_panel(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .max_width(ui.available_width() * 0.25)
            .show_inside(ui, |ui| {
                ui.vertical(|ui| {
                    self.show_language_selector(ui);
                    ui.add_space(8.0);
                    self.show_controls(ui);
                    ui.add_space(8.0);
                    self.show_debug_state(ui);
                    ui.add_space(8.0);
                    self.show_breakpoints(ui);
                });
            });
    }

    fn show_right_panel(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical(|ui| {
                self.show_editor(ui);
                ui.add_space(8.0);
                self.terminal.ui(ui);

                if let Some(input) = self.terminal.take_input() {
                    self.terminal.push_output(format!("> {}", input));
                    self.run_with_input(input);
                }
            });
        });
    }

    fn create_interpreter(
        config: &ExtInterpreterConfig,
        language_id: Option<&String>,
    ) -> Box<dyn Interpreter> {
        language_id
            .and_then(|id| config.get(id))
            .map(|cfg| {
                Box::new(ExternalInterpreter::new(cfg.exe_path.clone())) as Box<dyn Interpreter>
            })
            .unwrap_or_else(|| Box::new(ExternalInterpreter::new("".to_string())))
    }

    fn show_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("EsolangIDE");
                self.show_load_file_button(ui);
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
                        self.editor.set_text(&content);
                        self.terminal
                            .push_output(format!("Loaded file: {}", path.display()));
                    }
                    Err(e) => {
                        self.terminal
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

            let old_language = self.selected_language.clone();
            let mut current_language = self.selected_language.clone();

            let selected_text = current_language
                .as_ref()
                .and_then(|lang_id| {
                    self.available_languages
                        .iter()
                        .find(|(id, _)| id == lang_id)
                        .map(|(_, name)| name.clone())
                })
                .unwrap_or_else(|| "Select language".to_string());

            egui::ComboBox::from_id_salt("language_selector")
                .width(150.0)
                .selected_text(selected_text)
                .show_ui(ui, |ui| {
                    for (lang_id, lang_name) in &self.available_languages {
                        ui.selectable_value(
                            &mut current_language,
                            Some(lang_id.clone()),
                            lang_name,
                        );
                    }
                });

            if current_language != old_language {
                self.selected_language = current_language;
                self.update_interpreter();
            }
        });
    }

    fn update_interpreter(&mut self) {
        let new_interpreter =
            Self::create_interpreter(&self.config, self.selected_language.as_ref());

        if let Ok(mut guard) = self.interpreter.lock() {
            *guard = new_interpreter;
        }

        if let Some(lang_id) = &self.selected_language {
            self.terminal
                .push_output(format!("Updated interpreter to: {}", lang_id));
        }
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
            self.start_debug_session();
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
        self.terminal.push_output("Please input:");
        self.terminal.request_input();
    }

    fn start_debug_session(&mut self) {
        let code = self.editor.get_text();
        match self.interpreter.lock().unwrap().start_debug(code) {
            Ok(session) => {
                self.debug_session = Some(session);
                self.last_debug_state = self.debug_session.as_ref().map(|s| s.current_state());
            }
            Err(e) => {
                self.last_run_output = format!("Start debug error: {}", e);
            }
        }
    }

    fn step_debug(&mut self) {
        if let Some(session) = self.debug_session.as_mut() {
            match session.step() {
                Ok(state) => {
                    self.last_debug_state = Some(state);
                }
                Err(e) => {
                    self.last_run_output = format!("Step error: {}", e);
                }
            }
        }
    }

    fn resume_debug(&mut self) {
        if let Some(session) = self.debug_session.as_mut() {
            match session.resume_until_breakpoint(&self.bp_manager.breakpoints()) {
                Ok(state) => {
                    self.last_debug_state = Some(state);
                }
                Err(e) => {
                    self.last_run_output = format!("Resume error: {}", e);
                }
            }
        }
    }

    fn stop_debug(&mut self) {
        self.debug_session = None;
        self.last_debug_state = None;
    }

    fn show_debug_state(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.label("Debug State (JSON)");
            if let Some(state) = &self.last_debug_state {
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
        self.bp_manager.ui(ui);
    }

    fn show_editor(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label("Editor");
            let available_height = ui.available_height() - 180.0;
            ui.vertical(|ui| {
                ui.set_height(available_height.max(100.0));
                self.editor.ui(ui);
            });
        });
    }

    fn run_with_input(&mut self, input: String) {
        let code = self.editor.get_text();

        let req = RunRequest {
            code,
            input: input.into_bytes(),
        };

        match self.interpreter.lock().unwrap().run(req) {
            Ok(res) => {
                self.terminal
                    .push_output(String::from_utf8_lossy(&res.stdout).to_string());
            }
            Err(e) => {
                self.terminal.push_output(format!("Run error: {}", e));
            }
        }
    }
}
