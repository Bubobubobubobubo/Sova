use crate::App;
use ratatui::{
    Frame,
    prelude::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(frame: &mut Frame, _app: &App, area: Rect) {
    // Création d'un bloc central
    let block = Block::default()
        .title("Grid")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    frame.render_widget(block, area);

    // On affiche n'importe quoi
    let grid_content = Paragraph::new(Text::from("Idk what to do :)))) "))
        .style(Style::default())
        .block(Block::default());

    let grid_area = inner_area(area);
    frame.render_widget(grid_content, grid_area);
}

fn inner_area(area: Rect) -> Rect {
    let inner = area;
    Rect {
        x: inner.x + 1,
        y: inner.y + 1,
        width: inner.width.saturating_sub(2),
        height: inner.height.saturating_sub(2),
    }
}
