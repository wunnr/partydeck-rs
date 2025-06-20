use crate::paths::*;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PartyConfig {
    pub force_sdl: bool,
    pub render_scale: i32,
    pub gamescope_sdl_backend: bool,
    pub proton_version: String,
    #[serde(default)]
    pub vertical_two_player: bool,
}

pub fn load_cfg() -> PartyConfig {
    let path = PATH_PARTY.join("settings.json");

    if let Ok(file) = File::open(path) {
        if let Ok(config) = serde_json::from_reader::<_, PartyConfig>(BufReader::new(file)) {
            return config;
        }
    }

    // Return default settings if file doesn't exist or has error
    PartyConfig {
        force_sdl: false,
        render_scale: 100,
        gamescope_sdl_backend: true,
        proton_version: "".to_string(),
        vertical_two_player: false,
    }
}

pub fn save_cfg(config: &PartyConfig) -> Result<(), Box<dyn Error>> {
    let path = PATH_PARTY.join("settings.json");
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, config)?;
    Ok(())
}
