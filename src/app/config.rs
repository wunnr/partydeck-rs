use crate::paths::*;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum PadFilterType {
    All,
    NoSteamInput,
    OnlySteamInput,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PartyConfig {
    pub force_sdl: bool,
    pub render_scale: i32,
    pub enable_kwin_script: bool,
    pub gamescope_sdl_backend: bool,
    pub kbm_support: bool,
    pub proton_version: String,
    #[serde(default)]
    pub vertical_two_player: bool,
    pub pad_filter_type: PadFilterType,
    #[serde(default)]
    pub enable_cpu_affinity: bool,
    #[serde(default)]
    pub cpu_affinity_pattern: String,
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
        enable_kwin_script: true,
        gamescope_sdl_backend: true,
        kbm_support: true,
        proton_version: "".to_string(),
        vertical_two_player: false,
        pad_filter_type: PadFilterType::NoSteamInput,
        enable_cpu_affinity: false,
        cpu_affinity_pattern: "0,1,8,9;2,3,10,11;4,5,12,13;6,7,14,15".to_string(),
    }
}

pub fn save_cfg(config: &PartyConfig) -> Result<(), Box<dyn Error>> {
    let path = PATH_PARTY.join("settings.json");
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, config)?;
    Ok(())
}
