use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, style::{Color, Modifier, Style, Stylize}, text::Text, widgets::{Cell, HighlightSpacing, Row, ScrollbarState, StatefulWidget, Table, TableState}};
use sova_core::{device_map::DeviceMap, protocol::{DeviceDirection, DeviceInfo}};

use crate::app::AppState;

#[derive(Debug, Default)]
pub struct DevicesWidget {
    state: TableState,
    scroll_state: ScrollbarState,
}

impl DevicesWidget {

    pub fn process_event(&mut self, state: &mut AppState, event: KeyEvent) {
        match event.code {
            KeyCode::Up => self.state.select_previous(),
            KeyCode::Down => self.state.select_next(),
            _ => ()
        }
    }

}

impl StatefulWidget for &mut DevicesWidget {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let header_style = Style::default()
            .fg(Color::White)
            .bg(Color::Magenta)
            .bold();
        let selected_row_style = Style::default()
            .fg(Color::White)
            .bg(Color::LightMagenta)
            .bold();
        let header = [ "Name", "I/O", "Kind", "Connected", "Slot", "Address"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let mut longest_name = 5;
        let rows : Vec<Row> = state.devices.iter().map(|dev| {
            let name = Cell::from(format!("\n{}", dev.name));
            longest_name = std::cmp::max(dev.name.len() as u16, longest_name);
            let io = Cell::from(match dev.direction {
                DeviceDirection::Output => "\nO",
                DeviceDirection::Input => "\nI",
            });
            let kind = Cell::from(format!("\n{}", dev.kind));
            let connected = Cell::from(format!("\n{}", dev.is_connected));
            let slot = Cell::from(format!("\n{}", dev.slot_id.as_ref().map(ToString::to_string).unwrap_or_default()));
            let addr = Cell::from(format!("\n{}", dev.address.clone().unwrap_or_default()));
            Row::new([name, io, kind, connected, slot, addr]).height(3)
                //.style(Style::new().fg(self.colors.row_fg).bg(color))
        }).collect();
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(longest_name + 1),
                Constraint::Length(3),
                Constraint::Length(12),
                Constraint::Length(10),
                Constraint::Length(5),
                Constraint::Min(0),
            ],
        )
            .header(header)
            .row_highlight_style(selected_row_style)
            .highlight_symbol(Text::from(vec![
                "".into(),
                bar.into(),
                "".into(),
            ]))
            .highlight_spacing(HighlightSpacing::Always);
        t.render(area, buf, &mut self.state);
    }
}