use crate::app::{App, Mode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Clear, Paragraph},
};
use std::time::Instant;

use crate::components::{editor, grid, options, splash};

pub fn flash_screen(app: &mut App) {
    app.screen_state.flash.is_flashing = true;
    app.screen_state.flash.flash_start = Some(Instant::now());
}

pub fn ui(frame: &mut Frame, app: &mut App) {
    let flash = &mut app.screen_state.flash;
    if flash.is_flashing {
        if let Some(start_time) = flash.flash_start {
            if start_time.elapsed() > flash.flash_duration {
                flash.is_flashing = false;
                flash.flash_start = None;
            }
        }
    }
    // Create a vertical layout with main content and bottom bar
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    let main_area = main_layout[0];
    let bottom_bar = main_layout[1];

    match app.screen_state.mode {
        Mode::Splash => splash::draw(frame, app, main_area),
        Mode::Editor => editor::draw(frame, app, main_area),
        Mode::Grid => grid::draw(frame, app, main_area),
        Mode::Options => options::draw(frame, app, main_area),
    }

    draw_bottom_bar(frame, app, bottom_bar);

    let flash = &mut app.screen_state.flash;
    if flash.is_flashing {
        frame.render_widget(Clear, frame.area());
        frame.render_widget(
            Block::default().style(Style::default().bg(Color::White)),
            frame.area(),
        );
    }
}

fn draw_bottom_bar(frame: &mut Frame, app: &App, area: Rect) {
    // Get the current mode text
    let mode_text = match app.screen_state.mode {
        Mode::Editor => "EDITOR",
        Mode::Grid => "GRID",
        Mode::Options => "OPTIONS",
        Mode::Splash => "SPLASH",
    };

    // Combine the mode with the status message
    let status_text = format!("[ {} ] | {}", mode_text, app.status_message);

    // Calculate available width to determine if we need to truncate the status message
    let available_width = area.width as usize;
    let combined_text = if status_text.len() + 3 <= available_width {
        // We have room for both status and key hints
        format!("{}", status_text)
    } else if status_text.len() + 3 < available_width {
        // Only room for status
        status_text
    } else {
        // Need to truncate status
        format!("{}...", &status_text[0..available_width.saturating_sub(3)])
    };

    // Create the bottom bar with inverted colors for visibility
    let bottom_bar = Paragraph::new(Text::from(combined_text))
        .style(Style::default().bg(Color::White).fg(Color::Black));

    frame.render_widget(bottom_bar, area);
}
