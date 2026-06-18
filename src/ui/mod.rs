pub mod capsule;
pub mod home;
pub mod journal;
pub mod profile;
pub mod question;
pub mod reflection;
pub mod timeline;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use ratatui::Frame;

use crate::app::{App, Screen};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    draw_nav(frame, chunks[0], app.screen);
    match app.screen {
        Screen::Home => home::render(frame, chunks[1], app),
        Screen::Question => question::render(frame, chunks[1], app),
        Screen::Reflection => reflection::render(frame, chunks[1], app),
        Screen::Journal => journal::render(frame, chunks[1], app),
        Screen::Capsule => capsule::render(frame, chunks[1], app),
        Screen::Profile => profile::render(frame, chunks[1], app),
        Screen::Timeline => timeline::render(frame, chunks[1], app),
    }
    draw_status(frame, chunks[2], app);
}

fn draw_nav(frame: &mut Frame, area: Rect, screen: Screen) {
    let titles = [
        "Home",
        "Question",
        "Reflection",
        "Journal",
        "Capsule",
        "Profile",
        "Timeline",
    ]
    .into_iter()
    .map(Line::from)
    .collect::<Vec<_>>();
    let selected = match screen {
        Screen::Home => 0,
        Screen::Question => 1,
        Screen::Reflection => 2,
        Screen::Journal => 3,
        Screen::Capsule => 4,
        Screen::Profile => 5,
        Screen::Timeline => 6,
    };
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::BOTTOM))
        .select(selected)
        .style(Style::default().fg(Color::Gray))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(tabs, area);
}

fn draw_status(frame: &mut Frame, area: Rect, app: &App) {
    let text = format!(
        " {}  |  h home  q question  r reflection  j journal  c capsule  p profile  t timeline  Ctrl-C exit ",
        app.status
    );
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::TOP))
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(paragraph, area);
}
