use crate::core::{DebugSessionBox, InterpreterBox, RunRequest, RunResult};
use anyhow::Result;
use log::info;
use std::sync::mpsc::{Receiver, SendError, Sender, TryRecvError, channel};

// #[derive(Debug)]
pub enum ToWorkerMsg {
    /// 运行
    Run(RunRequest),
    /// 更新解释器
    UpdateInterpreter(InterpreterBox),
    /// 开始Debug
    StartDebug(String),
    /// 退出工作线程
    Shutdown,
}

// #[derive(Debug)]
pub enum FromWorkerMsg {
    /// 运行完成
    RunFinished(RunResult),
    /// 运行错误
    RunError(String),
    /// 解释器已更换
    InterpreterUpdated,
    /// Debug已开始
    DebugStarted(Result<DebugSessionBox>),
    /// 工作线程已退出
    WorkerShutdown,
}

/// 工作线程处理器
/// 
/// ## 线程结构
/// 主线程（UI）<br>
/// 工作线程（worker）<br>
///    ├─ stdin writer (I/O thread) <br>
///    ├─ stdout reader (I/O thread) <br>
///    └─ stderr reader (I/O thread) 
pub struct WorkerHandle {
    pub to_worker: Sender<ToWorkerMsg>,
    pub from_worker: Receiver<FromWorkerMsg>,
}

impl WorkerHandle {
    /// 启动工作线程，初始 interpreter 由调用者提供
    pub fn spawn(interpreter: InterpreterBox) -> Self {
        let (tx_worker, rx_worker) = channel::<ToWorkerMsg>();
        let (tx_ui, rx_ui) = channel::<FromWorkerMsg>();

        std::thread::spawn(move || {
            let mut worker = Worker::new(interpreter, tx_ui);

            while let Ok(msg) = rx_worker.recv() {
                // debug!("Worker received: {:?}", msg);
                if !worker.handle_message(msg) {
                    break;
                }
            }
            info!("Worker thread shutting down");
        });

        WorkerHandle {
            to_worker: tx_worker,
            from_worker: rx_ui,
        }
    }

    pub fn send_to(&self, msg: ToWorkerMsg) -> Result<(), SendError<ToWorkerMsg>> {
        self.to_worker.send(msg)
    }

    pub fn try_recv_from(&self) -> Result<FromWorkerMsg, TryRecvError> {
        self.from_worker.try_recv()
    }
}

struct Worker {
    interpreter: InterpreterBox,
    ui_tx: Sender<FromWorkerMsg>,
}

impl Worker {
    fn new(interpreter: InterpreterBox, ui_tx: Sender<FromWorkerMsg>) -> Self {
        Self { interpreter, ui_tx }
    }

    fn handle_message(&mut self, msg: ToWorkerMsg) -> bool {
        match msg {
            ToWorkerMsg::Run(req) => self.handle_run(&req),
            ToWorkerMsg::UpdateInterpreter(new_interp) => {
                self.handle_update_interpreter(new_interp)
            }
            ToWorkerMsg::StartDebug(code) => self.handle_start_debug(code),
            ToWorkerMsg::Shutdown => {
                self.handle_shutdown();
                return false; // 停止循环
            }
        }
        true // 继续循环
    }

    // 处理运行请求
    fn handle_run(&mut self, req: &RunRequest) {
        match self.interpreter.run(&req) {
            Ok(res) => {
                let _ = self.ui_tx.send(FromWorkerMsg::RunFinished(res));
            }
            Err(e) => {
                let _ = self.ui_tx.send(FromWorkerMsg::RunError(format!("{}", e)));
            }
        }
    }

    // 处理解释器更新
    fn handle_update_interpreter(&mut self, new_interp: InterpreterBox) {
        self.interpreter = new_interp;
        let _ = self.ui_tx.send(FromWorkerMsg::InterpreterUpdated);
    }

    // 处理调试开始
    fn handle_start_debug(&mut self, code: String) {
        let result = self.interpreter.start_debug(code);
        let _ = self.ui_tx.send(FromWorkerMsg::DebugStarted(result));
    }

    // 处理退出工作线程
    fn handle_shutdown(&self) {
        let _ = self.ui_tx.send(FromWorkerMsg::WorkerShutdown);
    }
}
