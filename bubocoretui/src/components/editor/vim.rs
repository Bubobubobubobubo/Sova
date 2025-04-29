use ratatui::prelude::*;
use std::fmt;
use tui_textarea::Input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VimMode {
    Normal,
    Insert,
    Visual,
    Operator(char),
    Command,        // New mode for entering commands like :1
    SearchForward,  // New mode for typing forward search query
    SearchBackward, // New mode for typing backward search query
}

impl VimMode {
    // Helper to get a title string for the block
    pub fn title_string(&self) -> String {
        match self {
            Self::Normal => "NORMAL".to_string(),
            Self::Insert => "INSERT".to_string(),
            Self::Visual => "VISUAL".to_string(),
            Self::Operator(c) => format!("OPERATOR({})", c),
            Self::Command => "COMMAND".to_string(), // Title for Command mode
            Self::SearchForward => "SEARCH".to_string(), // Title for Search modes
            Self::SearchBackward => "SEARCH".to_string(),
        }
    }

    // Helper to get cursor style (copied from example)
    pub fn cursor_style(&self) -> Style {
        let color = match self {
            Self::Normal => Color::Reset,
            Self::Insert => Color::LightBlue,
            Self::Visual => Color::LightYellow,
            Self::Operator(_) => Color::LightGreen,
            Self::Command => Color::Yellow, // Cursor style for Command mode
            Self::SearchForward => Color::LightMagenta, // Cursor style for Search modes
            Self::SearchBackward => Color::LightMagenta,
        };
        Style::default().fg(color).add_modifier(Modifier::REVERSED)
    }
}

impl fmt::Display for VimMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.title_string())
    }
}

// How the Vim emulation state transitions
#[derive(Debug, Clone, PartialEq, Eq)] // Removed Copy
pub enum VimTransition {
    Nop(Option<String>), // No operation / state change (optional status message)
    Mode(VimMode, Option<String>), // Switch to a new mode (optional status message)
    Pending(Input),      // Waiting for the next key (e.g., after 'g')
                         // Quit is handled by the main editor Esc logic now
}

// State of Vim emulation
#[derive(Debug, Clone)]
pub struct VimState {
    pub mode: VimMode,
    pub pending: Input,             // For multi-key sequences like 'gg'
    pub replace_pending: bool,      // Flag for 'r' command
    pub command_buffer: String, // Buffer for command mode input
}

impl VimState {
    pub fn new() -> Self {
        Self {
            mode: VimMode::Normal,
            pending: Input::default(),
            replace_pending: false,
            command_buffer: String::new(),
        }
    }

    // Helper to update state with pending input
    pub fn set_pending(&mut self, pending: Input) {
        self.pending = pending;
        self.replace_pending = false; 
        self.command_buffer.clear();
    }

    // Helper to reset pending input
    pub fn clear_pending(&mut self) {
        self.pending = Input::default();
        self.replace_pending = false; 
    }

    // Helper to set Vim mode
    pub fn set_mode(&mut self, mode: VimMode) {
        self.mode = mode;
        self.pending = Input::default();
        self.replace_pending = false; 
        if !matches!(
            mode,
            VimMode::Command | VimMode::SearchForward | VimMode::SearchBackward
        ) {
            self.command_buffer.clear();
        }
    }

    pub fn set_replace_pending(&mut self) {
        self.pending = Input::default(); 
        self.replace_pending = true;
        self.command_buffer.clear(); 
    }
}