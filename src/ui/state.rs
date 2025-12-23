use crate::{
    ExtInterpreterConfig,
    core::{DebugSession, DebugState, ExternalInterpreter, WorkerHandle},
    ui::{
        breakpoint::BreakpointPanel, editor::EditorPanel, layout::LayoutState,
        terminal::TerminalPanel,
    },
};

pub struct AppState {
    pub bp_panel: BreakpointPanel,
    pub editor_panel: EditorPanel,
    pub terminal_panel: TerminalPanel,
    pub layout: LayoutState,

    pub available_languages: Vec<(String, String)>,
    pub selected_language: Option<String>,

    pub last_run_output: String,
    pub last_debug_state: Option<DebugState>,
    pub debug_session: Option<Box<dyn DebugSession>>,

    pub worker: WorkerHandle,
    pub config: ExtInterpreterConfig,
}

impl AppState {
    pub fn new(config: ExtInterpreterConfig) -> Self {
        let available_languages = config.available_languages();
        let default_language = available_languages.get(0).map(|(id, _)| id.clone());
        let initial_interp =
            ExternalInterpreter::with_language_id(&config, default_language.as_ref());
        let worker = WorkerHandle::spawn(Box::new(initial_interp));

        Self {
            bp_panel: BreakpointPanel::new(),
            editor_panel: EditorPanel::default(),
            terminal_panel: TerminalPanel::default(),
            layout: LayoutState::default(),
            available_languages,
            selected_language: default_language,
            last_run_output: String::new(),
            last_debug_state: None,
            debug_session: None,
            worker,
            config,
        }
    }
}
