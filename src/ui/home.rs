use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::analysis::statistics::profile_summary;
use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(7), Constraint::Length(5)])
        .split(area);

    let logo = Paragraph::new(app.logo.as_str())
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::Cyan))
        .wrap(Wrap { trim: false });
    frame.render_widget(logo, chunks[0]);

    let summary = vec![
        Line::from(vec![Span::styled(
            "Where will you spend eternity?",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from(profile_summary(&app.profile)),
        Line::from("Press q to answer a question, j to journal, c for capsules."),
    ];
    let paragraph = Paragraph::new(summary)
        .block(Block::default().title("Home").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    frame.render_widget(paragraph, chunks[1]);
}
