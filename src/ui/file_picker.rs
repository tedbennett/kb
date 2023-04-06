#![allow(dead_code)]
use tui::{
    backend::Backend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Cell, Row, Table},
    Frame,
};

use crate::app::FilePickerState;

use super::popup::render_popup;

pub fn render_file_picker<B: Backend>(f: &mut Frame<B>, state: &mut FilePickerState) {
    let popup = render_popup(f, "Select Board", 20, None);
    let rows = state.files.iter().map(|row| {
        let text = Text::styled(row, Style::default());
        let cell = Cell::from(text);
        Row::new(vec![cell]).height(1)
    });
    let t = Table::new(rows)
        // .block(
        //     Block::default()
        //         .borders(Borders::ALL)
        //         .border_type(tui::widgets::BorderType::Rounded)
        //         .border_style(Style::default().fg(Color::Blue))
        //         .title("Select Board")
        //         .title_alignment(tui::layout::Alignment::Center),
        // )
        .highlight_style(
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .widths(&[Constraint::Percentage(100)]);
    f.render_stateful_widget(t, popup, &mut state.state);
}
