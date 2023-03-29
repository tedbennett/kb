use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    widgets::Paragraph,
    Frame,
};

/// Renders the one-line status bar at the bottom of the board
pub fn render_status_bar<B: Backend>(f: &mut Frame<B>, rect: Rect) {
    let subrects = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rect);

    f.render_widget(
        Paragraph::new("Move Cursor: ↑↓←→ | Switch Mode: M")
            .style(Style::default().add_modifier(Modifier::ITALIC | Modifier::DIM))
            .alignment(Alignment::Left),
        subrects[0],
    );
    f.render_widget(
        Paragraph::new("NORMAL")
            .style(Style::default().add_modifier(Modifier::ITALIC | Modifier::DIM))
            .alignment(Alignment::Right),
        subrects[1],
    );
}
