mod config;

use std::path::PathBuf;

use ::config::{Config, File};
pub use config::ExtInterpreterConfig;

use crate::EXT_INTERPRETERS_CONFIG_PATH;

pub fn load_config() -> Result<ExtInterpreterConfig, Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    let settings = Config::builder()
        .add_source(File::from(config_path))
        .build()?;
    let config = settings.try_deserialize()?;
    Ok(config)
}

fn get_config_path() -> PathBuf {
    // 首先尝试从可执行文件所在目录加载（打包后）
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let config_path = exe_dir.join(EXT_INTERPRETERS_CONFIG_PATH);
            if config_path.exists() {
                return config_path;
            }
        }
    }

    // 回退到开发环境的项目根目录
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir).join(EXT_INTERPRETERS_CONFIG_PATH)
    } else {
        // 最后尝试当前工作目录
        PathBuf::from(EXT_INTERPRETERS_CONFIG_PATH)
    }
}
