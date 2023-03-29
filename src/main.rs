use std::{error::Error, fs, io};
mod board;
use board::Board;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row as TuiRow, Table},
    Frame, Terminal,
};

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
    board: Board<'a>,
}

impl<'a> App<'a> {
    fn new(board: Board<'a>) -> App<'a> {
        App { board }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.board.down(),
                KeyCode::Up => app.board.up(),
                KeyCode::Left => app.board.left(),
                KeyCode::Right => app.board.right(),
                KeyCode::Esc => app.board.normal_mode(),
                KeyCode::Char('m') => app.board.toggle_mode(),
                _ => {}
            }
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
        Paragraph::new(app.board.title).alignment(Alignment::Center),
        sections[0],
    );

    f.render_widget(Paragraph::new("").alignment(Alignment::Center), sections[0]);

    if app.board.columns.len() == 0 {
        return;
    }

    let width = (100 / app.board.columns.len()) as u16;
    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(width); app.board.columns.len()].as_ref())
        .split(sections[1]);

    let is_moving = app.board.is_moving();
    app.board
        .columns
        .iter_mut()
        .enumerate()
        .for_each(|(i, col)| {
            let selected_style = Style::default().fg(if is_moving {
                Color::LightRed
            } else {
                Color::Green
            });
            let rows = col.rows.iter().map(|row| {
                let height = row.description.chars().filter(|c| *c == '\n').count() + 2;
                let mut text =
                    Text::styled(row.title, Style::default().add_modifier(Modifier::BOLD));
                text.extend(Text::styled(
                    row.description,
                    Style::default().add_modifier(Modifier::ITALIC | Modifier::DIM),
                ));
                let cell = Cell::from(text);
                TuiRow::new(vec![cell])
                    .height(height as u16)
                    .bottom_margin(1)
            });
            let t = Table::new(rows)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(tui::widgets::BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Blue).add_modifier(
                            if app.board.selected_column == i {
                                Modifier::empty()
                            } else {
                                Modifier::DIM
                            },
                        ))
                        .title(col.title)
                        .title_alignment(tui::layout::Alignment::Center),
                )
                .highlight_style(selected_style)
                // .highlight_symbol("│")
                .widths(&[Constraint::Percentage(100)]);
            f.render_stateful_widget(t, rects[i], &mut col.state);
        });
}
