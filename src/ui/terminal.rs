use eframe::egui;

#[derive(Default)]
pub struct TerminalView {
    output: String,
    input_line: String,
    allow_input: bool,
    pub pending_input: Option<String>,
}

impl TerminalView {
    pub fn request_input(&mut self) {
        self.allow_input = true;
        self.input_line.clear();
        self.pending_input = None;
    }

    pub fn push_output<T: Into<String>>(&mut self, s: T) {
        if !self.output.is_empty() {
            self.output.push('\n');
        }
        self.output.push_str(&s.into());
    }

    pub fn take_input(&mut self) -> Option<String> {
        self.pending_input.take()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(ui.style().visuals.extreme_bg_color)
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Terminal").color(egui::Color32::LIGHT_GRAY));

                    let input_height = 28.0;
                    let output_height = ui.available_height() - input_height - 6.0;

                    // 输出
                    egui::ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .max_height(output_height)
                        .show(ui, |ui| {
                            ui.add_sized(
                                [ui.available_width(), ui.available_height()],
                                egui::TextEdit::multiline(&mut self.output)
                                    .font(egui::TextStyle::Monospace)
                                    .text_color(egui::Color32::from_rgb(220, 220, 220))
                                    .interactive(false)
                                    .frame(false),
                            );
                        });

                    ui.add_space(4.0);

                    // 输入
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(">")
                                .monospace()
                                .color(egui::Color32::LIGHT_GREEN),
                        );

                        let resp = ui.add_sized(
                            [ui.available_width() - 60.0, input_height],
                            egui::TextEdit::singleline(&mut self.input_line)
                                .font(egui::TextStyle::Monospace)
                                .interactive(self.allow_input),
                        );

                        if self.allow_input
                            && resp.lost_focus()
                            && ui.input(|i| i.key_pressed(egui::Key::Enter))
                        {
                            self.commit_input();
                        }

                        if ui
                            .add_enabled(self.allow_input, egui::Button::new("Enter"))
                            .clicked()
                        {
                            self.commit_input();
                        }
                    });
                });
            });
    }

    fn commit_input(&mut self) {
        let s = self.input_line.trim().to_string();
        self.pending_input = Some(s);
        self.allow_input = false;
        self.input_line.clear();
    }
}
