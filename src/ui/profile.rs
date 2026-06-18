use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let profile = &app.profile;
    let lines = vec![
        Line::from(format!("First run: {}", profile.first_run)),
        Line::from(format!("Total days: {}", profile.total_days)),
        Line::from(format!("Total answers: {}", profile.total_answers)),
        Line::from(format!("Total memories: {}", profile.total_memories)),
        Line::from(format!(
            "Total journal entries: {}",
            profile.total_journal_entries
        )),
        Line::from(format!("Total capsules: {}", profile.total_capsules)),
        Line::from(format!("Database: {}", app.config.db_path.display())),
    ];
    frame.render_widget(
        Paragraph::new(lines)
            .block(Block::default().title("Profile").borders(Borders::ALL))
            .style(Style::default().fg(Color::White)),
        area,
    );
}
