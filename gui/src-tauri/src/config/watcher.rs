use anyhow::Result;
use notify::{Config as NotifyConfig, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use super::loader::ConfigLoader;
use super::types::ConfigUpdateEvent;

pub fn start_watcher(app_handle: AppHandle) -> Result<()> {
    std::thread::spawn(move || {
        if let Err(e) = watch_config_file(app_handle) {
            eprintln!("Config watcher error: {}", e);
        }
    });

    Ok(())
}

fn watch_config_file(app_handle: AppHandle) -> Result<()> {
    let loader = ConfigLoader::new()?;
    let config_path = loader.config_path().clone();

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        tx,
        NotifyConfig::default()
            .with_poll_interval(Duration::from_secs(2))
    )?;

    watcher.watch(&config_path, RecursiveMode::NonRecursive)?;

    println!("Watching config file: {:?}", config_path);

    for res in rx {
        match res {
            Ok(_event) => {
                match loader.load() {
                    Ok(config) => {
                        let event = ConfigUpdateEvent {
                            editor: config.editor,
                            appearance: config.appearance,
                        };

                        if let Err(e) = app_handle.emit("config-update", &event) {
                            eprintln!("Failed to emit config-update event: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to reload config: {}", e);
                    }
                }
            }
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }

    Ok(())
}
