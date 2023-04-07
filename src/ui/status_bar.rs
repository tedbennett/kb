use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::Paragraph,
    Frame,
};

/// Renders the one-line status bar at the bottom of the board
pub fn render_status_bar<B: Backend>(f: &mut Frame<B>, rect: Rect) {
    f.render_widget(
        Paragraph::new("Move Cursor: ↑↓←→ | Create Item: c | Help: Esc")
            .style(Style::default().add_modifier(Modifier::ITALIC | Modifier::DIM))
            .alignment(Alignment::Left),
        rect,
    );
}
