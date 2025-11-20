mod breakpoint;
mod editor;
mod terminal;

use crate::core::{DebugState, ExternalInterpreter, Interpreter, RunRequest};
use eframe::egui;
use std::sync::{Arc, Mutex};

use breakpoint::BreakpointManager;
use editor::EditorView;
// use terminal::Terminal;

pub struct UiApp {
    editor: EditorView,
    interpreter: Arc<Mutex<Box<dyn Interpreter>>>,
    last_run_output: String,
    last_debug_state: Option<DebugState>,
    debug_session: Option<Box<dyn crate::core::DebugSession>>,
    bp_manager: BreakpointManager,
    // terminal: Terminal,
}

impl eframe::App for UiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_top_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_right_panel(ui);
            self.show_left_panel(ui);
        });
    }
}

impl UiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            editor: EditorView::default(),
            interpreter: Arc::new(Mutex::new(Box::new(ExternalInterpreter::new(
                "F:\\Develop\\esolang\\stk\\cmake-build-debug\\stk.exe".to_string(), // TODO
            )))),
            last_run_output: String::new(),
            last_debug_state: None,
            debug_session: None,
            bp_manager: BreakpointManager::new(),
            // terminal: Terminal::new(),
        }
    }

    fn show_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("EsolangIDE");
                if ui.button("Load File").clicked() {
                    self.editor
                        .set_text(include_str!("../../examples/test_demo.txt")); // TODO
                }
            });
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
        let code = self.editor.get_text();
        let req = RunRequest {
            code,
            input: Vec::new(), // TODO
        };
        match self.interpreter.lock().unwrap().run(req) {
            Ok(res) => {
                self.last_run_output = String::from_utf8_lossy(&res.stdout).to_string();
            }
            Err(e) => {
                self.last_run_output = format!("Run error: {}", e);
            }
        }
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

    fn show_right_panel(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::right("right_panel")
            .resizable(false)
            .max_width(ui.available_width() * 0.25)
            .show_inside(ui, |ui| {
                ui.vertical(|ui| {
                    self.show_controls(ui);
                    ui.add_space(8.0);
                    self.show_debug_state(ui);
                    ui.add_space(8.0);
                    self.show_breakpoints(ui);
                });
            });
    }

    fn show_left_panel(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical(|ui| {
                self.show_editor(ui);
                ui.add_space(8.0);
                self.show_output(ui);
            });
        });
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

    fn show_output(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label("Output");
            ui.vertical(|ui| {
                ui.add_sized(
                    [ui.available_width(), 150.0],
                    egui::TextEdit::multiline(&mut self.last_run_output)
                        .interactive(false)
                        .lock_focus(true),
                );
            });
        });
    }
}
