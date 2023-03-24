//! ## Model
//!
//! app model

use crate::components::List;

use super::{Id, Msg};

use std::time::Duration;
use tuirealm::terminal::TerminalBridge;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::{event::NoUserEvent, Application, EventListenerCfg, Update};

pub struct Model {
    /// Application
    pub app: Application<Id, Msg, NoUserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: TerminalBridge,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
        }
    }
}

impl Model {
    pub fn view(&mut self) {
        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(33), // ToDo
                            Constraint::Percentage(33), // InProgress
                            Constraint::Percentage(33), // Done
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                self.app.view(&Id::ToDo, f, chunks[0]);
                self.app.view(&Id::InProgress, f, chunks[1]);
                self.app.view(&Id::Done, f, chunks[2]);
            })
            .is_ok());
    }

    fn init_app() -> Application<Id, Msg, NoUserEvent> {
        // Setup application
        // NOTE: NoUserEvent is a shorthand to tell tui-realm we're not going to use any custom user event
        // NOTE: the event listener is configured to use the default crossterm input listener and to raise a Tick event each second
        // which we will use to update the clock

        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10)),
        );
        assert!(app
            .mount(Id::ToDo, Box::new(List::new("To Do", 0)), Vec::default())
            .is_ok());
        assert!(app
            .mount(
                Id::InProgress,
                Box::new(List::new("In Progress", 0)),
                Vec::default()
            )
            .is_ok());
        assert!(app
            .mount(Id::Done, Box::new(List::new("Done", 0)), Vec::new())
            .is_ok());
        // Active letter counter
        assert!(app.active(&Id::ToDo).is_ok());
        app
    }
}

// Let's implement Update for model

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            // Set redraw
            self.redraw = true;
            // Match message
            match msg {
                Msg::AppClose => {
                    self.quit = true; // Terminate
                    None
                }
                Msg::MoveItem(_) => None,
                Msg::ChangeColumn(dir) => {
                    if let Some(focussed) = self.app.focus() {
                        let next = focussed.cycle(&dir);
                        self.app.active(&next).unwrap()
                    }
                    None
                }
            }
        } else {
            None
        }
    }
}
