use crate::{
    ExtInterpreterConfig,
    ui::{
        panels::CentralPanel, controller::AppController, panels::LeftPanel,
        state::AppState, panels::TopPanel,
    },
};
use eframe::egui;

pub struct UiApp {
    pub state: AppState,
}

impl eframe::App for UiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        AppController::handle_worker_msg(&mut self.state);
        TopPanel::ui(&mut self.state, ctx);
        if self.state.layout.show_left_panel {
            LeftPanel::ui(ctx, &mut self.state);
        }
        CentralPanel::ui(ctx, &mut self.state);
        ctx.request_repaint();
    }
}

impl UiApp {
    pub fn new(config: ExtInterpreterConfig, _cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: AppState::new(config),
        }
    }
}
