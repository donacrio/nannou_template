use serde::Deserialize;
use std::path::Path;

pub fn load(path: String) -> Config {
    match Config::new(path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Encountered error: {}.\nProcess exited with code 1", err);
            std::process::exit(1);
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub window: WindowConfig,
}

impl Config {
    pub fn new(path: String) -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::from(Path::new(&path)))
            .build()?
            .try_deserialize()
    }
}

#[derive(Debug, Deserialize)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub background_color: BackgroundColorConfig,
}

#[derive(Debug, Deserialize)]
pub struct BackgroundColorConfig {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}
