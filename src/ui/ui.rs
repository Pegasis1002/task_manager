use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::app::Task;

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, tasks: &[Task]) {
    let size = f.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Task Manager")
        .style(Style::default().fg(Color::Blue));

    let items: Vec<ListItem> = tasks.iter().map(|task| {
        let content = if task.completed {
            format!("{} (completed)", task.description)
        } else {
            task.description.clone()
        };
        ListItem::new(content)
    }).collect();

    let list = List::new(items)
        .block(block)
        .style(Style::default().fg(Color::White));

    f.render_widget(list, size);
}

