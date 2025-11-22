mod config;

use config::loader::ConfigLoader;
use config::types::{Config, ConfigUpdateEvent};
use config::watcher;
use tauri::Emitter;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_config() -> Result<Config, String> {
    let loader = ConfigLoader::new()
        .map_err(|e| e.to_string())?;

    loader.load_or_create()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_config_content() -> Result<String, String> {
    let loader = ConfigLoader::new()
        .map_err(|e| e.to_string())?;

    std::fs::read_to_string(loader.config_path())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn save_config_content(content: String) -> Result<(), String> {
    use config::validation::Validate;

    let mut config: Config = toml::from_str(&content)
        .map_err(|e| format!("Invalid TOML syntax: {}", e))?;

    config.validate();

    let loader = ConfigLoader::new()
        .map_err(|e| e.to_string())?;

    std::fs::write(loader.config_path(), content)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            match ConfigLoader::new().and_then(|l| l.load_or_create()) {
                Ok(config) => {
                    let event = ConfigUpdateEvent {
                        editor: config.editor,
                        appearance: config.appearance,
                    };
                    let _ = app.emit("config-update", &event);
                }
                Err(e) => {
                    eprintln!("Failed to load initial config: {}. Using defaults.", e);
                    let _ = app.emit("config-update", &ConfigUpdateEvent {
                        editor: config::types::EditorConfig::default(),
                        appearance: config::types::AppearanceConfig::default(),
                    });
                }
            }

            watcher::start_watcher(app.handle().clone())?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_config, get_config_content, save_config_content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
