use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use directories::ProjectDirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
    pub audio_engine: bool,
    pub sample_rate: u32,
    pub block_size: u32,
    pub buffer_size: u32,
    pub max_audio_buffers: u32,
    pub max_voices: u32,
    pub output_device: Option<String>,
    pub osc_port: u16,
    pub osc_host: String,
    pub timestamp_tolerance_ms: u64,
    pub audio_files_location: String,
    pub audio_priority: u8,
    pub relay: Option<String>,
    pub instance_name: String,
    pub relay_token: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            ip: "0.0.0.0".to_string(),
            port: 8080,
            audio_engine: false,
            sample_rate: 44100,
            block_size: 512,
            buffer_size: 1024,
            max_audio_buffers: 2048,
            max_voices: 128,
            output_device: None,
            osc_port: 12345,
            osc_host: "127.0.0.1".to_string(),
            timestamp_tolerance_ms: 1000,
            audio_files_location: "./samples".to_string(),
            audio_priority: 80,
            relay: None,
            instance_name: "local".to_string(),
            relay_token: None,
        }
    }
}

fn get_config_path() -> Result<PathBuf, String> {
    let proj_dirs = ProjectDirs::from("com", "sova", "sova")
        .ok_or("Failed to resolve config directory")?;

    Ok(proj_dirs.config_dir().join("config.json"))
}

pub async fn load_config() -> Result<ServerConfig, String> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        let default_config = ServerConfig::default();
        save_config(&default_config).await?;
        return Ok(default_config);
    }

    let content = fs::read_to_string(&config_path)
        .await
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: ServerConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config file: {}", e))?;

    Ok(config)
}

pub async fn save_config(config: &ServerConfig) -> Result<(), String> {
    let config_path = get_config_path()?;

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_path, json)
        .await
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

pub fn get_config_file_path() -> Result<String, String> {
    let path = get_config_path()?;
    Ok(path.to_string_lossy().to_string())
}
