use std::rc::Rc;
use ratatui::{
    Frame,
    layout::{Layout, Constraint, Rect, Direction},
    widgets::{ListItem, List, Paragraph, ListState},
    style::{Style, Color, Modifier},
    text::{Line, Span}
};

use crate::filemanager::FileManager;

pub fn draw(frame: &mut Frame, fm: &mut FileManager) {
    let chunks: Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1)
        ])
        .split(frame.area());

    let title: Paragraph = Paragraph::new(format!("{}", fm.current_path.display()))
        .style(Style::default().bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD));
    frame.render_widget(title, chunks[0]);

    let entries: Vec<ListItem> = fm.entries.iter().enumerate().map(| (i, entry) | {
        let style = if i == fm.selected {
            Style::default().bg(Color::Cyan).fg(Color::Black)
        } else {
            Style::default().bg(Color::Black).fg(Color::Cyan)
        };

        let display_name = format!("{} {}",
                if entry.is_dir { "D" } else { "F" },
                entry.name.to_string_lossy()
        );

        ListItem::new(Line::from(Span::styled(display_name, style)))
    }).collect();

    let list: List<'_> = List::new(entries);

    let mut list_state = ListState::default();
    list_state.select(Some(fm.selected));

    frame.render_stateful_widget(list, chunks[1], &mut list_state);

    let terminal_width: usize = chunks[2].width as usize;

    let margin: usize = 2;
    let available_width: usize = terminal_width.saturating_sub(margin * 2);

    let parts: [(&str, &str); 4] = [
        ("[↑/↓]", "Navigation"),
        ("[ENTER]", "Open"),
        ("[BACKSPACE]", "Go back"),
        ("[Q]", "Quit"),
    ];

    let total_text_length: usize = parts.iter()
        .map(|(key, _)| key.len() + 2)  
        .sum::<usize>() + 
        parts.iter().map(|(_, val)| val.len()).sum::<usize>();
        
    let spaces_needed: usize = available_width.saturating_sub(total_text_length);
    let space_between: usize = if parts.len() > 1 {
        spaces_needed / (parts.len() - 1)
    } else {
        0
    };

    let mut help_text: String = String::new();
    help_text.push_str(&" ".repeat(margin));

    for (i, (key, val)) in parts.iter().enumerate() {
        if i > 0 {
            help_text.push_str(&" ".repeat(space_between));
        }
        help_text.push_str(&format!("{} - {}", key, val));
    }

    help_text.push_str(&" ".repeat(margin));

    let help: Paragraph = Paragraph::new(help_text)
        .style(Style::default().bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD));
    frame.render_widget(help, chunks[2]);
}