use rusty_link::{AblLink, SessionState};
use std::error::Error;
use std::time::{Duration, Instant};

/// The current screen that the user is on:
/// - Editor: The user is editing a script.
/// - Grid: The user is browsing the grid of scripts.
/// - Options: The user is viewing the options menu.
pub enum Mode {
    Editor,
    Grid,
    Options,
}

pub struct Flash {
    pub is_flashing: bool,
    pub flash_start: Option<Instant>,
    pub flash_duration: Duration,
    pub flash_elapsed: Duration,
}

pub struct ScreenState {
    pub mode: Mode,
    pub flash: Flash,
}

pub struct EditorData {
    pub content: String,
    pub line_count: usize,
    pub cursor_position: (u16, u16),
}

pub struct ServerState {
    pub is_connected: bool,
    pub peers: Vec<String>,
    pub devices: Vec<String>,
}

pub struct App {
    pub screen_state: ScreenState,
    pub editor_data: EditorData,
    pub state: ServerState,
    pub status_message: String,
    pub link_client: AblLink,
}

impl App {
    pub fn new() -> App {
        let app = App {
            screen_state: ScreenState {
                mode: Mode::Editor,
                flash: Flash {
                    is_flashing: false,
                    flash_start: None,
                    flash_duration: Duration::from_micros(200_000),
                    flash_elapsed: Duration::from_secs(0),
                },
            },
            editor_data: EditorData {
                content: String::new(),
                line_count: 0,
                cursor_position: (0, 0),
            },
            state: ServerState {
                is_connected: false,
                peers: Vec::new(),
                devices: Vec::new(),
            },
            status_message: String::from("Welcome!"),
            link_client: AblLink::new(120.),
        };
        app.link_client.enable(true);
        app
    }

    pub fn set_content(&mut self, content: String) {
        self.editor_data.content = content;
        self.editor_data.line_count = self.editor_data.content.lines().count().max(1);
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) {
        self.editor_data.cursor_position = (x, y);
    }

    pub fn set_status_message(&mut self, message: String) {
        self.status_message = message;
    }

    pub fn set_flash_duration(&mut self, microseconds: u64) {
        self.screen_state.flash.flash_duration = Duration::from_micros(microseconds);
    }

    pub fn send_content(&self) -> Result<(), Box<dyn Error>> {
        // TODO: I probably should do something!
        Ok(())
    }
}
