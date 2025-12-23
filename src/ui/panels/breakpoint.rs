use std::collections::BTreeSet;

use crate::core::CodeLine;

#[derive(Default)]
pub struct BreakpointPanel {
    breakpoints: BTreeSet<CodeLine>,
    add_input: String,
}

impl BreakpointPanel {
    pub fn new() -> Self {
        Self {
            breakpoints: BTreeSet::new(),
            add_input: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.set_max_width(ui.available_width());
            ui.label("Breakpoints");

            self.show_add_breakpoint(ui);
            ui.add_space(6.0);
            self.show_breakpoint_list(ui);
        });
    }

    pub fn show_add_breakpoint(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Add bp idx:");
            let text_width = ui.available_width() - 50.0;
            ui.add(egui::TextEdit::singleline(&mut self.add_input).desired_width(text_width));
            if ui.button("Add").clicked() {
                if let Ok(idx) = self.add_input.trim().parse::<usize>() {
                    self.breakpoints.insert(idx);
                    self.add_input.clear();
                }
            }
        });
    }

    pub fn show_breakpoint_list(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label("Current breakpoints");
            let mut to_remove = Vec::new();

            ui.vertical(|ui| {
                if self.breakpoints.is_empty() {
                    ui.label("No breakpoints set");
                } else {
                    for &bp in &self.breakpoints {
                        ui.horizontal(|ui| {
                            ui.add_sized([40.0, 20.0], egui::Label::new(format!("{}", bp)));
                            if ui.small_button("×").clicked() {
                                to_remove.push(bp);
                            }
                        });
                    }
                }
            });

            for bp in to_remove {
                self.breakpoints.remove(&bp);
            }
        });
    }

    pub fn breakpoints(&self) -> Vec<CodeLine> {
        self.breakpoints.iter().cloned().collect()
    }
}
