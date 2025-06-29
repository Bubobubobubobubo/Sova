use crate::app::App;
use tui_textarea::{CursorMove, Input, Key};

use super::super::Mode;

pub fn handle_insert_mode(app: &mut App, input: Input) -> bool {
    match input {
        Input { key: Key::Esc, .. } | Input { key: Key::Char('c'), ctrl: true, .. } => {
            app.editor.textarea.move_cursor(CursorMove::Back);
            app.editor.vim_state.set_mode(Mode::Normal, &mut app.editor.textarea);
        }
        _ => {
            app.editor.textarea.input(input);
        }
    }
    
    true
}