use thiserror::Error;

#[derive(Debug, Error)]
pub enum SysError {
    #[error("Interpreter error: {0}")]
    Interpreter(String),

    #[error("Debug error: {0}")]
    Debug(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}