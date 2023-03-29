use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, Clear},
    Frame,
};

pub fn render_popup<B: Backend>(f: &mut Frame<B>) -> Rect {
    // Popup takes up 60% of the view's width
    let percentage = 60;
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percentage) / 2),
            Constraint::Percentage(percentage),
            Constraint::Percentage((100 - percentage) / 2),
        ])
        .split(f.size())[1];

    let popup_height = 12;
    let spacing = (f.size().height - popup_height) / 2;
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(spacing),
            Constraint::Length(11),
            Constraint::Length(spacing),
        ])
        .split(layout)[1];

    f.render_widget(Clear, popup_layout);
    f.render_widget(
        Block::default()
            .title("Popup")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        popup_layout,
    );
    popup_layout
}