//! 外部解释器调用

use std::os::windows::process::CommandExt;
use std::sync::mpsc;
use std::thread;
use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::core::{DebugSession, Interpreter, RunRequest, RunResult, interpreter::RunMetrics};
use crate::{CREATE_NO_WINDOW, ExtInterpreterConfig};
use anyhow::Result;
use tempfile::NamedTempFile;

enum StreamMsg {
    Stdout(Vec<u8>),
    Stderr(Vec<u8>),
}

/// 外部解释器
#[derive(Debug)]
pub struct ExternalInterpreter {
    /// 解释器exe路径
    pub exe_path: String,
}

impl ExternalInterpreter {
    pub fn new(exe_path: String) -> Self {
        Self { exe_path }
    }

    pub fn with_language_id(config: &ExtInterpreterConfig, language_id: Option<&String>) -> Self {
        language_id
            .and_then(|id| config.get(id))
            .map(|cfg| ExternalInterpreter::new(cfg.exe_path.clone()))
            .unwrap_or_else(|| ExternalInterpreter::new("".to_string()))
    }
}

impl Interpreter for ExternalInterpreter {
    fn run(&mut self, req: &RunRequest) -> Result<RunResult> {
        // // 代码临时文件
        // let code_path = create_temp_file_with_content(req.code.as_bytes())?;
        // // 输入临时文件
        // let input_path = create_temp_file_with_content(&req.input)?;
        // 代码临时文件
        let mut code_file = NamedTempFile::new()?;
        code_file.write_all(req.code.as_bytes())?;
        let code_path = code_file.path().to_string_lossy().to_string();

        // 输入临时文件
        let mut input_file = NamedTempFile::new()?;
        input_file.write_all(&req.input)?;
        let input_path = input_file.path().to_string_lossy().to_string();

        let mut child = spawn_process(&self.exe_path, &code_path, &input_path)?;
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let stdin_thread = spawn_stdin_writer(stdin, req.input.clone());
        let (tx, rx) = mpsc::channel();
        let out_thread = spawn_output_reader(stdout, true, tx.clone());
        let err_thread = spawn_output_reader(stderr, false, tx);

        let (out, err, code) = collect_process_output(&mut child, &rx)?;

        let _ = stdin_thread.join();
        let _ = out_thread.join();
        let _ = err_thread.join();

        Ok(RunResult {
            stdout: out,
            stderr: err,
            exit_code: Some(code),
            metrics: RunMetrics::default(),
        })
    }

    fn start_debug(&mut self, _code: String) -> Result<Box<dyn DebugSession + Send + Sync>> {
        anyhow::bail!("Debug not implemented for external interpreter yet")
    }
}

fn create_temp_file_with_content<C: AsRef<[u8]>>(content: C) -> Result<String> {
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(content.as_ref())?;
    Ok(temp_file.path().to_string_lossy().to_string())
}

fn spawn_process(exe: &str, code: &str, input: &str) -> Result<std::process::Child> {
    Ok(Command::new(exe)
        .arg(code)
        .arg(input)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()?)
}

fn spawn_stdin_writer(
    mut stdin: std::process::ChildStdin,
    input: Vec<u8>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        if !input.is_empty() {
            let _ = stdin.write_all(&input);
            let _ = stdin.flush();
        }
    })
}

fn spawn_output_reader(
    mut stream: impl std::io::Read + Send + 'static,
    is_stdout: bool,
    tx: mpsc::Sender<StreamMsg>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut buf = [0u8; 4096];

        loop {
            match stream.read(&mut buf) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    let msg = if is_stdout {
                        StreamMsg::Stdout(buf[..n].to_vec())
                    } else {
                        StreamMsg::Stderr(buf[..n].to_vec())
                    };
                    if tx.send(msg).is_err() {
                        break; // 接收方断开时，退出线程以避免死锁
                    }
                }
                Err(_) => break,
            }
        }
    })
}

fn collect_process_output(
    child: &mut std::process::Child,
    rx: &mpsc::Receiver<StreamMsg>,
) -> Result<(Vec<u8>, Vec<u8>, i32)> {
    let mut out_buf = Vec::new();
    let mut err_buf = Vec::new();

    loop {
        match rx.try_recv() {
            Ok(StreamMsg::Stdout(bytes)) => out_buf.extend_from_slice(&bytes),
            Ok(StreamMsg::Stderr(bytes)) => err_buf.extend_from_slice(&bytes),

            Err(mpsc::TryRecvError::Empty) => {
                /*
                TODO:
                子进程退出并不代表 stdout/stderr 已经全部读完
                reader thread 可能还在读缓冲区数据
                try_recv 只能获得 已经发送到 channel 的消息
                但 reader thread 可能还没来得及 send（因为 OS pipe 有缓冲，reader 正在 sleep 或者正在读）
                最后几 KB 输出会丢失
                reader thread 在 join 阶段已经无法把消息发送给 tx，因为主线程 rx 早就 return 退出函数，丢失了引用
                 */
                if let Some(status) = child.try_wait()? {
                    let code = status.code().unwrap_or(-1);

                    while let Ok(msg) = rx.try_recv() {
                        match msg {
                            StreamMsg::Stdout(b) => out_buf.extend_from_slice(&b),
                            StreamMsg::Stderr(b) => err_buf.extend_from_slice(&b),
                        }
                    }

                    return Ok((out_buf, err_buf, code));
                }

                std::thread::sleep(std::time::Duration::from_millis(5));
            }

            Err(mpsc::TryRecvError::Disconnected) => {
                let status = child.wait()?;
                return Ok((out_buf, err_buf, status.code().unwrap_or(-1)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_call_external_interpreter_with_files() -> anyhow::Result<()> {
        // Given
        let mut ei = ExternalInterpreter::new(
            "F:\\Develop\\esolang\\StackStackStack\\bin\\ststst\\ststst.exe".to_string(),
        );
        let req = RunRequest {
            code: "INPUT INPUT SUB OUTPUT".to_string(),
            input: "15 10".as_bytes().to_vec(),
        };

        // When
        let res = ei.run(&req)?;

        // Then
        println!("stdout: {:?}", String::from_utf8_lossy(&res.stdout));
        println!("stderr: {:?}", String::from_utf8_lossy(&res.stderr));
        println!("exit_code: {:?}", res.exit_code);

        let out = String::from_utf8_lossy(&res.stdout);
        assert!(out.trim().contains("-5"));
        assert_eq!(res.exit_code, Some(0));

        Ok(())
    }
}
