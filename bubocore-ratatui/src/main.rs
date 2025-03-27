#[macro_use]
extern crate rocket;
use app::{App, Mode};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::io::stderr;
use std::time::{Duration, Instant};
use std::{error::Error, io};
use ui::ui;

mod app;
mod components;
mod ui;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let _res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    let _ = rocket::build().mount("/", routes![index]);
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    // Approximativement 60 images par seconde
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // Événements qui ne sont pas KeyEventKind::Press
                    continue;
                }

                let screen = &mut app.screen_state;
                match screen.mode {
                    Mode::Splash => match key.code {
                        KeyCode::Enter => {
                            screen.mode = Mode::Editor;
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(true);
                        }
                        _ => {}
                    },
                    Mode::Editor => match key.code {
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(true);
                        }
                        KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            ui::flash_screen(app);
                            match app.send_content() {
                                Ok(_) => {
                                    app.set_status_message(String::from(
                                        "Content sent successfully!",
                                    ));
                                }
                                Err(e) => {
                                    app.set_status_message(format!("Error sending content: {}", e));
                                }
                            }
                        }
                        KeyCode::F(1) => {
                            screen.mode = Mode::Editor;
                        }
                        KeyCode::F(2) => {
                            screen.mode = Mode::Grid;
                        }
                        KeyCode::F(3) => {
                            screen.mode = Mode::Options;
                        }
                        KeyCode::Tab => {
                            screen.mode = Mode::Grid;
                        }
                        _ => {
                            app.editor_data.textarea.input(key);
                            // Update content string if needed
                            app.set_content(app.editor_data.textarea.lines().join("\n"));
                        }
                    },
                    Mode::Grid => match key.code {
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(true);
                        }
                        KeyCode::F(1) => {
                            screen.mode = Mode::Editor;
                        }
                        KeyCode::F(2) => {
                            screen.mode = Mode::Grid;
                        }
                        KeyCode::F(3) => {
                            screen.mode = Mode::Options;
                        }
                        KeyCode::Tab => {
                            screen.mode = Mode::Options;
                        }
                        _ => {}
                    },
                    Mode::Options => match key.code {
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(true);
                        }
                        KeyCode::F(1) => {
                            screen.mode = Mode::Editor;
                        }
                        KeyCode::F(2) => {
                            screen.mode = Mode::Grid;
                        }
                        KeyCode::F(3) => {
                            screen.mode = Mode::Options;
                        }
                        KeyCode::Tab => {
                            screen.mode = Mode::Editor;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
                // TODO: éventuellement on peut appeler une fonction ici pour faire du refresh!
            }
        }
    }
}
