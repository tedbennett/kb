use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Text,
    widgets::{List, ListItem},
    Frame,
};

use super::popup::render_popup;

pub fn render_help_popup<B: Backend>(f: &mut Frame<B>) {
    let items = vec![
        ("Navigate", "↑↓←→"),
        ("Move Item", "⇧ ↑↓←→"),
        ("Create Item", "c"),
        ("Edit Item", "e"),
        ("Delete Item", "d"),
        ("Create Column", "⇧c"),
        ("Edit Column", "⇧e"),
        ("Delete Column", "⇧d"),
    ];
    let max_cmd_width = 6;
    let frame = render_popup(f, "Help", items.len() as u16 + 4, None);
    let sections = Layout::default()
        .margin(1)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(max_cmd_width)])
        .split(frame);
    let descriptions: Vec<ListItem> = items
        .iter()
        .map(|(description, _)| ListItem::new(Text::from(*description)))
        .collect();
    let description_list = List::new(descriptions);
    f.render_widget(description_list, sections[0]);

    let commands: Vec<ListItem> = items
        .iter()
        .map(|(_, command)| {
            ListItem::new(Text::styled(
                format!("{:>6}", command),
                Style::default().add_modifier(Modifier::BOLD),
            ))
        })
        .collect();
    let commands_list = List::new(commands);
    f.render_widget(commands_list, sections[1]);
}
