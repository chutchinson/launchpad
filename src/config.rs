use serde_derive::Deserialize;

use std::path::Path;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub profile: Vec<ProfileConfig>
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct BrowserConfig {
    pub incognito: Option<bool>,
    pub distinct: Option<bool>
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ProfileConfig {
    pub target: Option<String>,
    pub url: Option<String>,
    pub display: Option<usize>,
    pub kiosk: Option<bool>,
    pub window: Option<WindowConfig>,
    pub browser: Option<BrowserConfig>
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct WindowConfig {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub x: Option<u32>,
    pub y: Option<u32>
}

impl Config {
    pub fn load<P: AsRef<Path>>(filename: P) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(filename)?;
        let config = toml::from_str(&contents)?;
        return Ok(config)
    }
}