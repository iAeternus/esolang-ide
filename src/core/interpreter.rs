//! 解释器通用接口

use anyhow::Result;
use serde_json::Value;

pub type ByteBuf = Vec<u8>;
pub type CodeLine = usize;

#[derive(Debug, Clone)]
pub struct RunRequest {
    /// 代码
    pub code: String,
    /// 输入
    pub input: ByteBuf,
}

#[derive(Debug, Clone)]
pub struct RunResult {
    pub stdout: ByteBuf,
    pub stderr: ByteBuf,
    pub exit_code: Option<i32>,
    pub metrics: RunMetrics,
}

/// 运行参数，不假定任何特定语言的内存模型或指针
#[derive(Debug, Default, Clone)]
pub struct RunMetrics {
    /// 当前已执行多少指令
    pub step: Option<CodeLine>,
    /// 最大内存使用量
    pub max_memory_bytes: Option<usize>,
    /// 执行耗时（单位：毫秒）
    pub duration_ms: Option<u128>,
}

/// 解释器接口
pub trait Interpreter: Send + Sync {
    /// 运行
    ///
    /// ## Params
    /// - req: 运行请求
    ///
    /// ## Return
    /// 返回运行结果，用Result包装
    fn run(&mut self, req: RunRequest) -> Result<RunResult>;

    /// 开始调试
    ///
    /// ## Params
    /// - code: 代码
    ///
    /// ## Return
    /// 返回Debug会话，用Result封装
    fn start_debug(&mut self, code: String) -> Result<Box<dyn DebugSession + Send + Sync>>;
}

/// Debug会话接口
pub trait DebugSession: Send + Sync {
    /// 单步调试
    ///
    /// ## Return
    /// 返回当前Debug状态
    fn step(&mut self) -> Result<DebugState>;

    /// 继续执行直到遇到断点或程序结束
    ///
    /// ## Params
    /// - breakpoints: 断点列表
    ///
    /// ## Return
    /// 返回执行到断点或程序终止时的Debug状态
    fn resume_until_breakpoint(&mut self, breakpoints: &[CodeLine]) -> Result<DebugState>;

    /// 获取当前调试状态快照
    ///
    /// ## Return
    /// 返回当前Debug状态
    fn current_state(&self) -> DebugState;

    /// 判断程序是否已经执行完毕
    ///
    /// ## Return
    /// true = 程序已终止（PC到达末尾或出错）
    /// false = 程序仍可继续执行
    fn is_terminated(&self) -> bool;
}

/// Debug状态
#[derive(Debug, Clone, Default)]
pub struct DebugState {
    /// 程序计数器，指向当前指令索引
    pub pc: Option<CodeLine>,
    /// 输出
    pub stdout: Vec<u8>,
    /// 是否停止
    pub halted: bool,
    /// 解释器内部状态，任意 JSON
    pub state: Value,
    pub(crate) info: (),
}
