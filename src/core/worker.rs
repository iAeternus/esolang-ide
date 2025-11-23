use crate::core::{DebugSession, Interpreter, RunRequest, RunResult};
use anyhow::Result;
use std::sync::mpsc::{Receiver, Sender, channel};

pub enum WorkerMsg {
    /// 运行
    Run(RunRequest),
    /// 更新解释器
    UpdateInterpreter(Box<dyn crate::core::Interpreter + Send>),
    /// 开始Debug
    StartDebug(String),
    /// 退出工作线程
    Shutdown,
}

pub enum UiMsg {
    /// 运行完成
    RunFinished(RunResult),
    /// 运行错误
    RunError(String),
    /// 解释器已更换
    InterpreterUpdated,
    /// Debug已开始
    DebugStarted(Result<Box<dyn DebugSession + Send + Sync>>),
}
pub struct WorkerHandle {
    pub to_worker: Sender<WorkerMsg>,
    pub from_worker: Receiver<UiMsg>,
}

/// 启动工作线程，初始 interpreter 由调用者提供
pub fn start_worker(mut interp: Box<dyn Interpreter + Send>) -> WorkerHandle {
    let (tx_worker, rx_worker) = channel::<WorkerMsg>();
    let (tx_ui, rx_ui) = channel::<UiMsg>();

    std::thread::spawn(move || {
        while let Ok(msg) = rx_worker.recv() {
            match msg {
                WorkerMsg::Run(req) => {
                    // 运行解释器
                    match interp.run(req) {
                        Ok(res) => {
                            let _ = tx_ui.send(UiMsg::RunFinished(res));
                        }
                        Err(e) => {
                            let _ = tx_ui.send(UiMsg::RunError(format!("{}", e)));
                        }
                    }
                }

                WorkerMsg::UpdateInterpreter(new_interp) => {
                    // 更换解释器，会drop掉旧解释器
                    interp = new_interp;
                    let _ = tx_ui.send(UiMsg::InterpreterUpdated);
                }

                WorkerMsg::StartDebug(code) => {
                    let result = interp.start_debug(code);
                    let _ = tx_ui.send(UiMsg::DebugStarted(result));
                }

                WorkerMsg::Shutdown => {
                    break;
                }
            }
        }
    });

    WorkerHandle {
        to_worker: tx_worker,
        from_worker: rx_ui,
    }
}
