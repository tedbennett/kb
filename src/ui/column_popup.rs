use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::{ColumnFields, ColumnPopupState};

use super::popup::{render_popup, render_text_area};

pub fn render_column_popup<B: Backend>(
    f: &mut Frame<B>,
    title: &str,
    state: &mut ColumnPopupState,
) {
    let frame = render_popup(f, title, 6, None);
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(1)])
        .split(frame);
    render_text_area(f, ColumnFields::Title, &mut state.title, true, sections[0]);
}
