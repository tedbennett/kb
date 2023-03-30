use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::model::{PopupFocusState, PopupState};

pub fn render_popup<B: Backend>(
    f: &mut Frame<B>,
    title: &str,
    height: u16,
    width: Option<u16>,
) -> Rect {
    let constraints = {
        if let Some(width) = width {
            [
                Constraint::Length((f.size().width - width) / 2),
                Constraint::Length(width),
                Constraint::Length((f.size().width - width) / 2),
            ]
        } else {
            // Popup takes up 60% of the view's width
            let percentage = 60;
            [
                Constraint::Percentage((100 - percentage) / 2),
                Constraint::Percentage(percentage),
                Constraint::Percentage((100 - percentage) / 2),
            ]
        }
    };
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(f.size())[1];

    let spacing = (f.size().height - height) / 2;
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(spacing),
            Constraint::Length(height),
            Constraint::Length(spacing),
        ])
        .split(layout)[1];

    f.render_widget(Clear, popup_layout);
    let popup = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let popup_inner = popup.inner(popup_layout);
    f.render_widget(popup, popup_layout);
    popup_inner
}

pub fn render_text_area<B: Backend>(
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
