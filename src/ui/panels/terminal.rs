use eframe::egui;

#[derive(Default)]
pub struct TerminalPanel {
    output: String,
    input_line: String,
    allow_input: bool,
    pending_input: Option<String>,
}

impl TerminalPanel {
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
        let visuals = ui.style().visuals.clone();

        egui::Frame::new()
            .fill(visuals.panel_fill)
            .stroke(egui::Stroke::new(
                1.0,
                visuals.widgets.noninteractive.bg_stroke.color,
            ))
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                // ===== Header =====
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("TERMINAL")
                            .monospace()
                            .size(11.0)
                            .color(visuals.weak_text_color()),
                    );
                    ui.separator();
                });

                ui.add_space(4.0);

                // 输出区域
                let input_height = 22.0;
                let output_height = ui.available_height() - input_height - 6.0;

                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .max_height(output_height)
                    .show(ui, |ui| {
                        ui.add_sized(
                            [ui.available_width(), ui.available_height()],
                            egui::TextEdit::multiline(&mut self.output)
                                .font(egui::TextStyle::Monospace)
                                .text_color(egui::Color32::from_rgb(210, 210, 210))
                                .interactive(false)
                                .frame(false),
                        );
                    });

                ui.add_space(4.0);

                // 输入区域
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(">").monospace().size(14.0).color(
                        if self.allow_input {
                            egui::Color32::LIGHT_GREEN
                        } else {
                            visuals.weak_text_color()
                        },
                    ));

                    let resp = ui.add_sized(
                        [ui.available_width() - 56.0, input_height],
                        egui::TextEdit::singleline(&mut self.input_line)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .interactive(self.allow_input),
                    );

                    if self.allow_input
                        && resp.lost_focus()
                        && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        self.commit_input();
                    }

                    if ui
                        .add_enabled(
                            self.allow_input,
                            egui::Button::new(egui::RichText::new("Enter").monospace().size(12.0)),
                        )
                        .clicked()
                    {
                        self.commit_input();
                    }
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
