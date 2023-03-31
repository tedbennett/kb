use std::{error::Error, fs, io};
mod app;
use app::model::Model;
mod ui;
use app::board::Board;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame, Terminal,
};
use ui::{render_board, render_column_popup, render_dialog, render_item_popup, render_status_bar};

const BOARD_FILENAME: &str = "kanban.json";

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // parse board from file
    let file = fs::read_to_string(BOARD_FILENAME)
        .expect(&format!("Failed to find board file: {BOARD_FILENAME}"));
    let board = Board::from_file(&file, &BOARD_FILENAME)
        .expect(&format!("Failed to parse board at: {BOARD_FILENAME}"));
    // create app and run it
    let app = App::new(board);
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

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
    if app.model.is_creating() {
        render_item_popup(f, "Create Item", &mut app.model.popup);
    }

    if app.model.is_editing() {
        render_item_popup(f, "Edit Item", &mut app.model.popup);
    }

    if app.model.is_deleting() {
        render_dialog(f, "Delete Item?", &mut app.model.dialog);
    }
    if app.model.is_creating_column() {
        render_column_popup(f, "Create Column", &mut app.model.column);
    }
    if app.model.is_editing_column() {
        render_column_popup(f, "Edit Column", &mut app.model.column);
    }
    if app.model.is_deleting_column() {
        render_dialog(f, "Delete Column?", &mut app.model.dialog);
    }
}
