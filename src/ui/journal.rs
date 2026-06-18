use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(6)])
        .split(area);

    let entries = app
        .journals()
        .into_iter()
        .map(|entry| {
            ListItem::new(Line::from(format!(
                "{} | {}",
                entry.timestamp.format("%Y-%m-%d"),
                entry.content.replace('\n', " ")
            )))
        })
        .collect::<Vec<_>>();
    frame.render_widget(
        List::new(entries)
            .block(
                Block::default()
                    .title("Journal Entries")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White)),
        chunks[0],
    );

    frame.render_widget(
        Paragraph::new(format!(
            "{}\n\nUse memory: title | content to preserve a named memory.",
            app.input
        ))
        .block(
            Block::default()
                .title("New Journal Entry")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false }),
        chunks[1],
    );
}
