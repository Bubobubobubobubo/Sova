use crate::App;
use crate::components::inner_area;
use ratatui::{
    Frame,
    prelude::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
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
