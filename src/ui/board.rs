use crate::app::board::Board;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Cell, Row as TuiRow, Table},
    Frame,
};
pub fn render_board<B: Backend>(f: &mut Frame<B>, rect: Rect, board: &mut Board) {
    if board.columns.len() == 0 {
        return;
    }

    let width = (100 / board.columns.len()) as u16;
    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(width); board.columns.len()].as_ref())
        .split(rect);

    let is_moving = board.is_moving();
    board.columns.iter_mut().enumerate().for_each(|(i, col)| {
        let selected_style = Style::default().fg(if is_moving {
            Color::LightRed
        } else {
            Color::Green
        });
        let rows = col.rows.iter().map(|row| {
            let height = row.description.chars().filter(|c| *c == '\n').count() + 2;
            let mut text = Text::styled(row.title, Style::default().add_modifier(Modifier::BOLD));
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
                        if board.selected_column == i {
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
