use serde::Deserialize;
use std::collections::HashMap;

/// 外部解释器配置
#[derive(Debug, Deserialize, Clone)]
pub struct ExtInterpreterConfig {
    pub interpreters: HashMap<String, ExtInterpreter>,
}

/// 外部解释器
#[derive(Debug, Deserialize, Clone)]
pub struct ExtInterpreter {
    pub name: String,                 // 名称
    pub exe_path: String,             // 可执行文件路径
    pub file_extensions: Vec<String>, // 支持的文件扩展名
}

impl ExtInterpreterConfig {
    pub fn interpreters(&self) -> Vec<ExtInterpreter> {
        self.interpreters.values().cloned().collect()
    }

    pub fn language_names(&self) -> Vec<String> {
        self.interpreters
            .values()
            .map(|ei| ei.name.clone())
            .collect()
    }

    pub fn available_languages(&self) -> Vec<(String, String)> {
        self.interpreters
            .iter()
            .map(|(k, v)| (k.clone(), v.name.clone()))
            .collect()
    }

    /// 根据语言ID获取外部解释器信息
    ///
    /// ## Params
    /// - language_id: 配置项中的 [interpreters.xxx]
    pub fn get(&self, language_id: &str) -> Option<&ExtInterpreter> {
        self.interpreters.get(language_id)
    }

    /// 检查外部解释器是否可用 TODO: 这里需要一种手段检测是否可用，而非简单的判断配置是否正确，类似ping
    pub fn is_available(&self, language_id: &str) -> bool {
        self.get(language_id)
            .map_or(false, |info| !info.exe_path.is_empty())
    }
}
