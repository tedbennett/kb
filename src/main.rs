use std::{error::Error, fs, io};
mod app;
mod terminal;
use app::args::Args;
use app::model::{Model, Popup};
mod ui;
use app::board::Board;
use clap::Parser;
use crossterm::event::{self, Event};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame, Terminal,
};
use ui::{render_board, render_column_popup, render_dialog, render_item_popup, render_status_bar};

const BOARD_FILENAME: &str = "kb.json";

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    // setup terminal
    let mut terminal = terminal::init()?;

    // parse board from file
    let filename = args.filename.unwrap_or(BOARD_FILENAME.to_string());
    let file = fs::read_to_string(&filename)?;
    let board = Board::from_file(&file, &filename)?;
    // create app and run it
    let app = App::new(board);
    let res = run_app(&mut terminal, app);

    // cleanup - restore terminal
    terminal::reset(&mut terminal)?;
    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

struct App<'a> {
    pub model: Model<'a>,
}

impl<'a> App<'a> {
    fn new(board: Board<'a>) -> App<'a> {
        App {
            model: Model::new(board),
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            app.model.on_keypress(key);
        }
        if app.model.quit {
            return Ok(());
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.size());

    f.render_widget(
        Paragraph::new(app.model.board.title).alignment(Alignment::Center),
        sections[0],
    );

    render_status_bar(f, sections[2]);
    render_board(f, sections[1], &mut app.model.board);
    match &mut app.model.popup {
        Popup::CreateRow(state) => render_item_popup(f, "Create Item", state),
        Popup::EditRow(state) => render_item_popup(f, "Edit Item", state),
        Popup::DeleteRow(state) => render_dialog(f, "Delete Item?", state),
        Popup::CreateColumn(state) => render_column_popup(f, "Create Column", state),
        Popup::EditColumn(state) => render_column_popup(f, "Edit Column", state),
        Popup::DeleteColumn(state) => render_dialog(f, "Delete Column?", state),
        _ => {}
    };
}
