use super::types::Config;
use std::path::Path;
use std::fs;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load() -> Config {
        // Try to load from standard locations
        let config_paths = [
            "ccline.json",
            "ccline.toml", 
            "config.json",
            "config.toml",
        ];
        
        for path in &config_paths {
            if let Ok(config) = Self::load_from_path(path) {
                return config;
            }
        }
        
        // Return default config if no config file found
        Config::default()
    }
    
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        
        let config = if let Some(ext) = path.extension() {
            match ext.to_str() {
                Some("json") => serde_json::from_str(&content)?,
                Some("toml") => toml::from_str(&content)?,
                _ => return Err("Unsupported config file format".into()),
            }
        } else {
            // Try JSON first, then TOML
            serde_json::from_str(&content)
                .or_else(|_| toml::from_str(&content))?
        };
        
        Ok(config)
    }
}