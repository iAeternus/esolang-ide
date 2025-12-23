use egui::UiBuilder;

use crate::{MIN_TERMINAL_HEIGHT, ui::{controller::AppController, state::AppState}};

pub struct CentralPanel;

impl CentralPanel {
    pub fn ui(ctx: &egui::Context, state: &mut AppState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();

            // 代码编辑器
            ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                state.editor_panel.ui(ui)
            });

            // 不点击按钮终端就不展开
            if !state.layout.terminal_visible {
                return;
            }

            // 终端位置
            let min_h = MIN_TERMINAL_HEIGHT;
            let max_h = rect.height() - 40.0;
            state.layout.terminal_height = state.layout.terminal_height.clamp(min_h, max_h);
            let terminal_top = rect.bottom() - state.layout.terminal_height;
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

                let new_height = (state.layout.terminal_height - delta).clamp(min_h, max_h);

                // 达到最小或最大时，完全锁死
                if new_height != state.layout.terminal_height {
                    state.layout.terminal_height = new_height;
                }
            }

            // 终端
            ui.scope_builder(UiBuilder::new().max_rect(terminal_rect), |ui| {
                state.terminal_panel.ui(ui)
            });

            if let Some(input) = state.terminal_panel.take_input() {
                state.terminal_panel.push_output(format!("> {}", input));
                AppController::run_with_input(&state, &input);
            }
        });
    }
}
