mod config;

use ::config::{Config, File};
pub use config::ExtInterpreterConfig;

use crate::EXT_INTERPRETERS_CONFIG_PATH;

pub fn load_config() -> Result<ExtInterpreterConfig, Box<dyn std::error::Error>> {
    let settings = Config::builder()
        .add_source(File::with_name(EXT_INTERPRETERS_CONFIG_PATH))
        .build()?;

    let config: ExtInterpreterConfig = settings.try_deserialize()?;
    Ok(config)
}