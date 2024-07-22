use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SoundPackConfig {
    pub id: String,
    pub name: String,
    pub key_define_type: String,
    pub includes_numpad: bool,
    pub sound: String,
    pub defines: HashMap<String, [u64; 2]>,
}

impl SoundPackConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(path)?;
        println!("Config: {}", config_str);
        let config: SoundPackConfig = toml::from_str(&config_str)?;
        Ok(config)
    }
}
