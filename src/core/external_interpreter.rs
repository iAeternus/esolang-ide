//! 外部解释器调用

use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::core::{DebugSession, Interpreter, RunRequest, RunResult, interpreter::RunMetrics};
use anyhow::Result;

/// 外部解释器
pub struct ExternalInterpreter {
    /// 解释器exe路径
    pub exe_path: String,
}

impl ExternalInterpreter {
    pub fn new(exe_path: String) -> Self {
        Self { exe_path }
    }
}

impl Interpreter for ExternalInterpreter {
    fn run(&mut self, req: RunRequest) -> Result<RunResult> {
        let mut child = Command::new(&self.exe_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // 将 input 写入 stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(&req.input)?;
        }

        let output = child.wait_with_output()?;

        Ok(RunResult {
            stdout: output.stdout,
            stderr: output.stderr,
            exit_code: Some(output.status.code().unwrap_or(-1)),
            metrics: RunMetrics::default(),
        })
    }

    fn start_debug(&mut self, _code: String) -> Result<Box<dyn DebugSession + Send + Sync>> {
        anyhow::bail!("Debug not implemented for external interpreter yet")
    }
}
