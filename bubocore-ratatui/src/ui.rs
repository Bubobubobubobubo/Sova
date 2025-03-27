use crate::app::{App, Mode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Clear, Paragraph},
};
use std::time::Instant;

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
        Mode::Editor => draw_editor(frame, app, main_area),
        Mode::Grid => draw_grid(frame, app, main_area),
        Mode::Options => draw_options(frame, app, main_area),
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

fn draw_editor(frame: &mut Frame, app: &App, area: Rect) {
    // Create the main horizontal layout with 80%/20% split
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(area);

    // Editor area (left side - 80%)
    let editor_area = chunks[0];
    let editor = Block::default()
        .title("Editor")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    // Draw the editor box
    frame.render_widget(editor.clone(), editor_area);

    // Here you would render the actual text content inside the editor area
    // This is a placeholder, you'll need to implement text rendering based on your app state
    let editor_content = Paragraph::new(Text::from(app.editor_data.content.clone()))
        .style(Style::default())
        .block(Block::default());

    // Use inner area of the block to place the text content
    let editor_text_area = inner_area(editor_area);
    frame.render_widget(editor_content, editor_text_area);

    // Info panel (right side - 20%)
    let info_area = chunks[1];
    let info_panel = Block::default()
        .title("Info")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    frame.render_widget(info_panel.clone(), info_area);

    // Example info content
    let info_content = Paragraph::new(Text::from(format!(
        "Cursor: ({}, {})\nLines: {}",
        app.editor_data.cursor_position.0,
        app.editor_data.cursor_position.1,
        app.editor_data.line_count
    )))
    .style(Style::default());

    let info_text_area = inner_area(info_area);
    frame.render_widget(info_content, info_text_area);
}

fn draw_options(frame: &mut Frame, app: &App, area: Rect) {
    // Create a horizontal layout with 60%/40% split
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Left side - Log box (60% width)
    let log_area = main_chunks[0];
    let log_block = Block::default()
        .title("Log")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    frame.render_widget(log_block, log_area);

    // Example log content
    let log_content = Paragraph::new(Text::from("System log entries will appear here..."))
        .style(Style::default())
        .block(Block::default());

    let log_text_area = inner_area(log_area);
    frame.render_widget(log_content, log_text_area);

    // Right side - Three equal boxes (Devices, Peers, Options)
    let right_side = main_chunks[1];
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(right_side);

    // Devices box (top)
    let devices_block = Block::default()
        .title("Devices")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    frame.render_widget(devices_block, right_chunks[0]);

    // Show connected devices
    let devices_content = if app.state.devices.is_empty() {
        String::from("No devices connected")
    } else {
        app.state.devices.join("\n")
    };

    let devices_text = Paragraph::new(Text::from(devices_content))
        .style(Style::default())
        .block(Block::default());

    let devices_text_area = inner_area(right_chunks[0]);
    frame.render_widget(devices_text, devices_text_area);

    // Peers box (middle)
    let peers_block = Block::default()
        .title("Peers")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    frame.render_widget(peers_block, right_chunks[1]);

    // Show connected peers
    let peers_content = if app.state.peers.is_empty() {
        String::from("No peers connected")
    } else {
        app.state.peers.join("\n")
    };

    let peers_text = Paragraph::new(Text::from(peers_content))
        .style(Style::default())
        .block(Block::default());

    let peers_text_area = inner_area(right_chunks[1]);
    frame.render_widget(peers_text, peers_text_area);

    // Options box (bottom)
    let options_block = Block::default()
        .title("Options")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    frame.render_widget(options_block, right_chunks[2]);

    // Example options content
    let options_content = Paragraph::new(Text::from("IDK what to do :))))"))
        .style(Style::default())
        .block(Block::default());

    let options_text_area = inner_area(right_chunks[2]);
    frame.render_widget(options_content, options_text_area);
}

fn draw_grid(frame: &mut Frame, _app: &App, area: Rect) {
    // Create a centered block
    let block = Block::default()
        .title("Grid")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    frame.render_widget(block, area);

    // Example grid content
    let grid_content = Paragraph::new(Text::from("Idk what to do :)))) "))
        .style(Style::default())
        .block(Block::default());

    let grid_area = inner_area(area);
    frame.render_widget(grid_content, grid_area);
}

fn draw_bottom_bar(frame: &mut Frame, app: &App, area: Rect) {
    // Get the current mode text
    let mode_text = match app.screen_state.mode {
        Mode::Editor => "EDITOR",
        Mode::Grid => "GRID",
        Mode::Options => "OPTIONS",
    };

    // Combine the mode with the status message
    let status_text = format!("{} | {}", mode_text, app.status_message);

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

// Helper function to get the inner area of a block, accounting for borders
fn inner_area(area: Rect) -> Rect {
    let inner = area;
    Rect {
        x: inner.x + 1,
        y: inner.y + 1,
        width: inner.width.saturating_sub(2),
        height: inner.height.saturating_sub(2),
    }
}
