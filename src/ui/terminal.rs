use eframe::egui;

pub struct Terminal {
    /// 程序运行 / debug 输出区（不可编辑）
    output: String,
    /// 用户输入（单行）
    input_line: String,
    /// 是否允许输入（在点击 Run 后开启）
    allow_input: bool,
    /// 当用户点击 Input 按钮时，将输入发送到 UI 层
    pub pending_input: Option<String>,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            input_line: String::new(),
            allow_input: false,
            pending_input: None,
        }
    }

    /// 是否允许输入
    pub fn request_input(&mut self) {
        self.allow_input = true;
        self.input_line.clear();
        self.pending_input = None;
    }

    /// 终端添加输出文本
    pub fn push_output<T: Into<String>>(&mut self, s: T) {
        let new_content = s.into();
        if !self.output.is_empty() && !self.output.ends_with('\n') {
            self.output.push('\n');
        }
        self.output.push_str(&new_content);
    }

    /// 清除输出
    pub fn clear_output(&mut self) {
        self.output.clear();
    }

    /// UI 层消费输入（仅消费一次）
    pub fn take_input(&mut self) -> Option<String> {
        self.pending_input.take()
    }

    /// 重置输入状态
    pub fn reset(&mut self) {
        self.allow_input = false;
        self.input_line.clear();
        self.pending_input = None;
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label("Output");
            let output_height = 100.0;
            let available_width = ui.available_width();
            egui::ScrollArea::vertical()
                .max_height(output_height)
                .show(ui, |ui| {
                    ui.add_sized(
                        [available_width, ui.available_height()],
                        egui::TextEdit::multiline(&mut self.output)
                            .font(egui::TextStyle::Monospace)
                            .interactive(false)
                            .lock_focus(true)
                            .frame(false),
                    );
                });

            ui.add_space(6.0);

            ui.horizontal(|ui| {
                ui.label(">");
                let input_field = ui.add_sized(
                    [ui.available_width() - 80.0, 24.0],
                    egui::TextEdit::singleline(&mut self.input_line).interactive(self.allow_input),
                );
                if self.allow_input
                    && input_field.lost_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                {
                    self.commit_input();
                }
                if ui
                    .add_enabled(self.allow_input, egui::Button::new("Input"))
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
        // 输入后不可再编辑，等待下一次点击 Run 按钮
        self.allow_input = false;
        self.input_line.clear();
    }
}
