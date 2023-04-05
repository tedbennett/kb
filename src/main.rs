use std::path::Path;
mod error;
use color_eyre::Report;
use std::fs;
mod app;
mod terminal;
use app::args::{Cli, Commands};
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

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    let board = parse_board(args)?;
    let mut terminal = terminal::init()?;

    let res = run_app(&mut terminal, board);

    // cleanup - restore terminal
    terminal::reset(&mut terminal)?;
    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn get_boolean_input(msg: &str) -> bool {
    loop {
        let mut buf = String::new();
        println!("{}", msg);
        if std::io::stdin().read_line(&mut buf).is_ok() {
            match buf.to_lowercase().trim() {
                "n" => return false,
                "" | "y" => return true,
                _ => {}
            }
        }
    }
}

fn get_full_filename(filename: &Option<String>) -> color_eyre::Result<String> {
    match filename {
        Some(f) => {
            let path = Path::new(".kb");
            if !path.exists() || !path.is_dir() {
                if get_boolean_input(".kb directory not found. Create one? Y/n ") {
                    _ = fs::create_dir(&path);
                } else {
                    return Err(Report::msg("Failed to find .kb directory"));
                }
            }
            Ok(format!(".kb/{}.json", f))
        }
        None => Ok(BOARD_FILENAME.to_string()),
    }
}

fn parse_board<'a>(args: Cli) -> color_eyre::Result<Board> {
    match &args.command {
        Some(Commands::New(arg)) => {
            let filename = get_full_filename(&arg.filename)?;
            // Make sure file does not already exist
            if Path::new(&filename).exists() {
                return Err(Report::msg("File already exists"));
            }
            Board::create(&filename)
        }
        None => {
            let filename = get_full_filename(&args.filename)?;
            println!("{}", &filename);
            let file = fs::read_to_string(&filename)?;
            Board::from_file(file, filename)
        }
    }
}

struct App<'a> {
    pub model: Model<'a>,
}

impl<'a> App<'a> {
    fn new(board: Board) -> App<'a> {
        App {
            model: Model::new(board),
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, board: Board) -> color_eyre::Result<()> {
    // create app and run it
    let mut app = App::new(board);
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
    let title = app.model.board.title.clone();
    f.render_widget(
        Paragraph::new(title).alignment(Alignment::Center),
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
