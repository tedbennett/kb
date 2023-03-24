//! ## Demo
//!
//! `Demo` shows how to use tui-realm in a real case

extern crate tuirealm;

use tuirealm::application::PollStrategy;

use tuirealm::{AttrValue, Attribute, Update};
// -- internal
mod app;
mod components;
use app::model::Model;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
// Let's define the messages handled by our app. NOTE: it must derive `PartialEq`
#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    MoveItem(Direction),
    ChangeColumn(Direction),
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    ToDo,
    InProgress,
    Done,
}

impl Id {
    pub fn cycle(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Up | Direction::Down => self.to_owned(),
            Direction::Right => match self {
                Id::ToDo => Id::InProgress,
                Id::InProgress => Id::Done,
                Id::Done => Id::ToDo,
            },
            Direction::Left => match self {
                Id::ToDo => Id::Done,
                Id::InProgress => Id::ToDo,
                Id::Done => Id::InProgress,
            },
        }
    }
}

fn main() {
    // Setup model
    let mut model = Model::default();
    // Enter alternate screen
    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();
    // Main loop
    // NOTE: loop until quit; quit is set in update if AppClose is received from counter
    while !model.quit {
        // Tick
        match model.app.tick(PollStrategy::Once) {
            Err(err) => {
                assert!(model
                    .app
                    .attr(
                        &Id::ToDo,
                        Attribute::Text,
                        AttrValue::String(format!("Application error: {}", err)),
                    )
                    .is_ok());
            }
            Ok(messages) if messages.len() > 0 => {
                // NOTE: redraw if at least one msg has been processed
                model.redraw = true;
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            _ => {}
        }
        // Redraw
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }
    // Terminate terminal
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
}
