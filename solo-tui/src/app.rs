use crate::event::{AppEvent, Event, EventHandler};
use crossbeam_channel::{Receiver, Sender};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use sova_core::schedule::{SchedulerMessage, SovaNotification};

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub events: EventHandler,
    pub sched_iface: Sender<SchedulerMessage>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(sched_iface: Sender<SchedulerMessage>, sched_update: Receiver<SovaNotification>) -> Self {
        App {
            running: false,
            events: EventHandler::new(),
            sched_iface,
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event)
                    if key_event.kind == crossterm::event::KeyEventKind::Press =>
                {
                    self.handle_key_event(key_event)?
                }
                _ => {}
            }
            Event::App(app_event) => match app_event {
                AppEvent::NextPage => todo!(),
                AppEvent::PreviousPage => todo!(),
                AppEvent::Quit => self.quit(),
            }
            Event::SchedulerControl(msg) => {
                let _ = self.sched_iface.send(msg);
            }
            Event::SchedulerNotification(notif) => {
                self.handle_notification(notif)?
            }
        }
        Ok(())
    }

    pub fn handle_notification(&mut self, notif: SovaNotification) -> color_eyre::Result<()> {
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            // KeyCode::Right => self.events.send(AppEvent::Increment),
            // KeyCode::Left => self.events.send(AppEvent::Decrement),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
