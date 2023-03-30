use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::model::{PopupFocusState, PopupState};

use super::popup::{render_popup, render_text_area};

pub fn render_item_popup<B: Backend>(f: &mut Frame<B>, title: &str, state: &mut PopupState) {
    let frame = render_popup(f, title, 12, None);
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(frame);
    render_text_area(f, PopupFocusState::Title, state, sections[0]);
    render_text_area(f, PopupFocusState::Description, state, sections[1]);
}
