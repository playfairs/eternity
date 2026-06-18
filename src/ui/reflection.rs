use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::Frame;

use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let items = app
        .reflection
        .insights
        .iter()
        .map(|insight| ListItem::new(Line::from(insight.as_str())))
        .collect::<Vec<_>>();
    let list = List::new(items)
        .block(Block::default().title("Reflection").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    frame.render_widget(list, area);
}
