//! 外部解释器调用

use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::core::{DebugSession, Interpreter, RunRequest, RunResult, interpreter::RunMetrics};
use anyhow::Result;
use tempfile::NamedTempFile;

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
        // 代码临时文件
        let mut code_file = NamedTempFile::new()?;
        code_file.write_all(req.code.as_bytes())?;
        let code_path = code_file.path().to_str().unwrap().to_string();

        // 数据临时文件
        let mut input_file = NamedTempFile::new()?;
        input_file.write_all(&req.input)?;
        let input_path = input_file.path().to_str().unwrap().to_string();

        let child = Command::new(&self.exe_path)
            .arg(&code_path) // 代码文件路径
            .arg(&input_path) // 输入文件路径
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = child.wait_with_output()?;

        // 临时文件会在离开作用域后自动删除

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_call_external_interpreter_with_files() -> anyhow::Result<()> {
        // Given
        let mut ei = ExternalInterpreter::new(
            "F:\\Develop\\esolang\\stk\\cmake-build-debug\\stk.exe".to_string(),
        );
        let req = RunRequest {
            code: "INPUT INPUT SUB OUTPUT".to_string(),
            input: "15 10".as_bytes().to_vec(),
        };

        // When
        let res = ei.run(req)?;

        // Then
        println!("stdout: {:?}", String::from_utf8_lossy(&res.stdout));
        println!("stderr: {:?}", String::from_utf8_lossy(&res.stderr));
        println!("exit_code: {:?}", res.exit_code);

        let output = String::from_utf8_lossy(&res.stdout);
        assert!(output.trim().contains("-5"));
        assert_eq!(res.exit_code, Some(0));

        Ok(())
    }
}
