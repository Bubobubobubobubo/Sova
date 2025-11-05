use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Style, Stylize}, widgets::{StatefulWidget, Widget}};
use sova_core::schedule::{ActionTiming, SchedulerMessage};
use tui_textarea::TextArea;

use crate::{app::AppState, event::AppEvent};

#[derive(Default)]
pub struct EditWidget {
    text_area: TextArea<'static>
}

impl EditWidget {

    pub fn open(&mut self, state: &AppState) {
        let Some(frame) = state.selected_frame() else {
            return;
        };
        let content = frame.script().content();
        self.text_area = content.lines().into();
        self.text_area.set_line_number_style(Style::default().dark_gray());
    }

    pub fn get_help() -> &'static str {
        "\
        C-S: Upload
        "
    }

    pub fn process_event(&mut self, state: &mut AppState, event: KeyEvent) { 
        match event.code {
            KeyCode::Char('s' | 'S') if event.modifiers == KeyModifiers::CONTROL => {
                let Some(frame) = state.selected_frame() else {
                    return;
                };
                let content = self.text_area.lines().join("\n");
                let (line_id, frame_id) = state.selected;
                state.events.send(
                    SchedulerMessage::SetScript(
                        line_id, 
                        frame_id, 
                        frame.script().lang().to_owned(), 
                        content,
                        ActionTiming::Immediate
                    ).into()
                );
                state.events.send(
                    AppEvent::Positive("Sent script".to_owned())
                );
            } 
            _ => { 
                self.text_area.input(event);
            }
        }
    }

}

impl StatefulWidget for &EditWidget {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        use Constraint::*;
        let layout = Layout::vertical([Min(0), Length(2)]);
        let [main_area, tools_area] = layout.areas(area);
        self.text_area.render(main_area, buf);
    }
}