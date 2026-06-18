use chrono::Utc;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(6), Constraint::Length(7)])
        .split(area);

    let now = Utc::now();
    let capsules = app
        .capsules()
        .into_iter()
        .map(|capsule| {
            let state = if capsule.is_unlocked(now) {
                format!("unlocked: {}", capsule.content.replace('\n', " "))
            } else {
                format!("locked until {}", capsule.unlock_date.format("%Y-%m-%d"))
            };
            ListItem::new(Line::from(format!("{} | {}", capsule.title, state)))
        })
        .collect::<Vec<_>>();
    frame.render_widget(
        List::new(capsules)
            .block(Block::default().title("Capsules").borders(Borders::ALL))
            .style(Style::default().fg(Color::White)),
        chunks[0],
    );

    let help = format!(
        "{}\n\nFormat: title | message | 30d, 1y, 5y, or YYYY-MM-DD. Tab cycles default unlock: {}",
        app.input, app.capsule_unlock
    );
    frame.render_widget(
        Paragraph::new(help)
            .block(Block::default().title("New Capsule").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: false }),
        chunks[1],
    );
}
