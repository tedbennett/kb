use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Paragraph,
    Frame,
};

use crate::app::{DialogFields, DialogState, PopupFields};

use super::popup::render_popup;

pub fn render_dialog<B: Backend>(f: &mut Frame<B>, message: &str, state: &mut DialogState) {
    let frame = render_popup(f, "", 7, Some(30));

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1); 5])
        .split(frame);

    f.render_widget(
        Paragraph::new(message).alignment(Alignment::Center),
        sections[1],
    );

    let button_sections = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50); 2])
        .split(sections[3]);
    f.render_widget(
        button_widget(state.focussed == DialogFields::Cancel, DialogFields::Cancel),
        button_sections[0],
    );
    f.render_widget(
        button_widget(
            state.focussed == DialogFields::Confirm,
            DialogFields::Confirm,
        ),
        button_sections[1],
    );
}

pub fn button_widget<F: PopupFields>(focussed: bool, field: F) -> Paragraph<'static> {
    let style = if focussed {
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Magenta)
    } else {
        Style::default().bg(Color::DarkGray)
    };

    Paragraph::new(Span::styled(field.title().to_string(), style)).alignment(Alignment::Center)
}
