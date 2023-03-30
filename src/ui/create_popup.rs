use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::model::{PopupFocusState, PopupState};

use super::render_popup;

pub fn render_create_popup<B: Backend>(f: &mut Frame<B>, state: &mut PopupState) {
    let frame = render_popup(f, "Create Item");
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(frame);
    render_text_area(f, PopupFocusState::Title, state, sections[0]);
    render_text_area(f, PopupFocusState::Description, state, sections[1]);
}

fn render_text_area<B: Backend>(
    f: &mut Frame<B>,
    field: PopupFocusState,
    state: &mut PopupState,
    rect: Rect,
) {
    let focussed = field == state.focussed;
    let text_area = match field {
        PopupFocusState::Description => &mut state.description,
        PopupFocusState::Title => &mut state.title,
    };
    let cursor_style = if focussed {
        Style::default().add_modifier(Modifier::REVERSED)
    } else {
        Style::default()
    };
    text_area.set_cursor_style(cursor_style);
    text_area.set_cursor_line_style(Style::default());

    let block_style = if focussed {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
            .fg(Color::Reset)
            .add_modifier(Modifier::DIM)
    };

    let block = Block::default()
        .title(field.title())
        .borders(Borders::ALL)
        .border_style(block_style)
        .border_type(tui::widgets::BorderType::Rounded);
    let inner_rect = block.inner(rect);
    f.render_widget(block, rect);

    if let Some(first) = text_area.lines().first() {
        if first.len() == 0 {
            f.render_widget(
                Paragraph::new(field.placeholder())
                    .style(Style::default().add_modifier(Modifier::DIM | Modifier::ITALIC)),
                inner_rect,
            );
        } else {
            f.render_widget(text_area.widget(), inner_rect);
        }
    }
}
