use crate::{
    core::{ExternalInterpreter, FromWorkerMsg, RunRequest, ToWorkerMsg},
    ui::state::AppState,
};

pub struct AppController;

impl AppController {
    /// 接收工作线程执行结果，并处理消息
    pub fn handle_worker_msg(state: &mut AppState) {
        while let Ok(msg) = state.worker.try_recv_from() {
            match msg {
                FromWorkerMsg::RunFinished(result) => {
                    state
                        .terminal_panel
                        .push_output(String::from_utf8_lossy(&result.stdout).to_string());
                }
                FromWorkerMsg::RunError(err) => {
                    state
                        .terminal_panel
                        .push_output(format!("Run error: {}", err));
                }
                FromWorkerMsg::DebugStarted(session) => match session {
                    Ok(session) => {
                        state.debug_session = Some(session);
                        state.last_debug_state =
                            state.debug_session.as_ref().map(|s| s.current_state());
                    }
                    Err(e) => {
                        state.last_run_output = format!("Start debug error: {}", e);
                    }
                },
                FromWorkerMsg::InterpreterUpdated => {
                    if let Some(lang) = &state.selected_language {
                        state
                            .terminal_panel
                            .push_output(format!("The interpreter has been changed to: {}.", lang));
                    }
                }
                FromWorkerMsg::WorkerShutdown => {
                    todo!() // TODO: 处理工作线程返回消息
                }
            }
        }
    }

    // pub fn spawn_worker(config: &ExtInterpreterConfig) -> WorkerHandle {
    //     let available_languages = config.available_languages();
    //     let default_language = available_languages.get(0).map(|(id, _)| id.clone());
    //     let initial_interp =
    //         ExternalInterpreter::with_language_id(&config, default_language.as_ref());
    //     WorkerHandle::spawn(Box::new(initial_interp))
    // }

    pub fn update_interpreter(state: &AppState, language_id: Option<&String>) {
        let new_interp = ExternalInterpreter::with_language_id(&state.config, language_id);
        let _ = state
            .worker
            .send_to(ToWorkerMsg::UpdateInterpreter(Box::new(new_interp)));
    }

    pub fn start_debug_session(state: &AppState) {
        let code = state.editor_panel.get_text();
        let _ = state.worker.send_to(ToWorkerMsg::StartDebug(code));
    }

    pub fn run_with_input(state: &AppState, input: &str) {
        let req = RunRequest {
            code: state.editor_panel.get_text(),
            input: input.into(),
        };

        let _ = state.worker.send_to(ToWorkerMsg::Run(req));
    }
}
