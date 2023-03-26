use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
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

struct ListItem<'a> {
    title: &'a str,
    description: &'a str,
}
struct Column<'a> {
    title: &'a str,
    items: Vec<ListItem<'a>>,
}
struct App<'a> {
    state: TableState,
    columns: Vec<Column<'a>>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        let mut state = TableState::default();
        state.select(Some(0));
        App {
            state,
            columns: vec![
                Column {
                    title: "To Do",
                    items: vec![ListItem {
                        title: "List Item 1",
                        description: "This is a description\nhello",
                    }],
                },
                Column {
                    title: "Done",
                    items: vec![ListItem {
                        title: "List Item 2",
                        description: "This is a description\nhello",
                    }],
                },
            ],
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.columns.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.columns.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let width = (100 / app.columns.len()) as u16;
    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(width); app.columns.len()].as_ref())
        .split(f.size());

    app.columns.iter().enumerate().for_each(|(i, col)| {
        let selected_style = Style::default().fg(Color::LightGreen);
        let rows = col.items.iter().map(|item| {
            let height = item.description.chars().filter(|c| *c == '\n').count() + 2;
            let mut text = Text::styled(item.title, Style::default().add_modifier(Modifier::BOLD));
            text.extend(Text::styled(
                item.description,
                Style::default().add_modifier(Modifier::ITALIC | Modifier::DIM),
            ));
            let cell = Cell::from(text);
            Row::new(vec![cell]).height(height as u16)
        });
        let t = Table::new(rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(tui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Blue))
                    .title(col.title)
                    .title_alignment(tui::layout::Alignment::Center),
            )
            .highlight_style(selected_style)
            .highlight_symbol("│")
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(30),
                Constraint::Min(10),
            ]);
        f.render_stateful_widget(t, rects[i], &mut app.state);
    });
}
