use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::Frame;

use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines = app
        .recent_answer_lines()
        .into_iter()
        .map(|line| ListItem::new(Line::from(format!("answer | {line}"))))
        .collect::<Vec<_>>();

    if lines.is_empty() {
        lines.push(ListItem::new(Line::from("No activity yet.")));
    }

    frame.render_widget(
        List::new(lines)
            .block(Block::default().title("Timeline").borders(Borders::ALL))
            .style(Style::default().fg(Color::White)),
        area,
    );
}
