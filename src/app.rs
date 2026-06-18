use std::fs;
use std::io::{self, Stdout};
use std::time::Duration as StdDuration;

use chrono::{Duration, NaiveDate, TimeZone, Utc};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::config::AppConfig;
use crate::db::Database;
use crate::errors::{Error, Result};
use crate::models::{Capsule, Journal, Profile, Question};
use crate::questions::load_question_bank;
use crate::reflection::{ReflectionEngine, ReflectionReport};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    Question,
    Reflection,
    Journal,
    Capsule,
    Profile,
    Timeline,
}

pub struct App {
    pub config: AppConfig,
    pub logo: String,
    pub db: Database,
    pub screen: Screen,
    pub current_question: Question,
    pub input: String,
    pub capsule_unlock: String,
    pub reflection: ReflectionReport,
    pub profile: Profile,
    pub status: String,
    pub running: bool,
}

impl App {
    pub fn initialize() -> Result<Self> {
        let config = AppConfig::local();
        fs::create_dir_all(&config.data_dir)?;

        let logo = fs::read_to_string(&config.logo_path)
            .unwrap_or_else(|_| "Eternity\nWhere will you spend eternity?".to_string());

        let mut db = Database::open(&config.db_path)?;
        let questions = load_question_bank(&config.questions_dir)?;
        db.upsert_questions(&questions)?;
        let current_question = db.random_question()?;
        let profile = db.profile()?;

        Ok(Self {
            config,
            logo,
            db,
            screen: Screen::Home,
            current_question,
            input: String::new(),
            capsule_unlock: "30d".to_string(),
            reflection: ReflectionReport::single("Answer a question to begin seeing patterns."),
            profile,
            status: "Ready".to_string(),
            running: true,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = TerminalSession::start()?;
        while self.running {
            terminal.draw(|frame| crate::ui::draw(frame, self))?;
            if event::poll(StdDuration::from_millis(200))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key)?;
                }
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.running = false;
            return Ok(());
        }

        match self.screen {
            Screen::Question => self.handle_question_key(key),
            Screen::Journal => self.handle_journal_key(key),
            Screen::Capsule => self.handle_capsule_key(key),
            _ => self.handle_navigation_key(key),
        }
    }

    fn handle_navigation_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('h') | KeyCode::Esc => self.screen = Screen::Home,
            KeyCode::Char('q') => self.screen = Screen::Question,
            KeyCode::Char('r') => self.screen = Screen::Reflection,
            KeyCode::Char('j') => self.screen = Screen::Journal,
            KeyCode::Char('c') => self.screen = Screen::Capsule,
            KeyCode::Char('p') => {
                self.refresh_profile()?;
                self.screen = Screen::Profile;
            }
            KeyCode::Char('t') => self.screen = Screen::Timeline,
            KeyCode::Char('n') => self.next_question()?,
            KeyCode::Char('x') => self.running = false,
            _ => {}
        }
        Ok(())
    }

    fn handle_question_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => self.screen = Screen::Home,
            KeyCode::Enter => self.save_answer()?,
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Char('n') if self.input.is_empty() => self.next_question()?,
            KeyCode::Char(character) => self.input.push(character),
            _ => {}
        }
        Ok(())
    }

    fn handle_journal_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => self.screen = Screen::Home,
            KeyCode::Enter if !self.input.trim().is_empty() => {
                if let Some((title, content)) = parse_memory_entry(&self.input) {
                    self.db.save_memory(title, content)?;
                    self.status = "Memory saved".to_string();
                } else {
                    self.db.save_journal(&self.input)?;
                    self.status = "Journal entry saved".to_string();
                }
                self.input.clear();
                self.refresh_profile()?;
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Char(character) => self.input.push(character),
            _ => {}
        }
        Ok(())
    }

    fn handle_capsule_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => self.screen = Screen::Home,
            KeyCode::Enter => self.save_capsule_from_input()?,
            KeyCode::Tab => {
                if self.capsule_unlock == "30d" {
                    self.capsule_unlock = "1y".to_string();
                } else if self.capsule_unlock == "1y" {
                    self.capsule_unlock = "5y".to_string();
                } else {
                    self.capsule_unlock = "30d".to_string();
                }
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Char(character) => self.input.push(character),
            _ => {}
        }
        Ok(())
    }

    fn save_answer(&mut self) -> Result<()> {
        let content = self.input.trim().to_string();
        if content.is_empty() {
            self.status = "Write something before saving.".to_string();
            return Ok(());
        }

        let saved = self.db.save_answer(&self.current_question.id, &content)?;
        let answers = self.db.all_answers()?;
        let previous = self
            .db
            .previous_answer_for_question(&saved.question_id, saved.id)?;
        self.reflection = ReflectionEngine::generate(&answers, previous.as_ref());
        self.input.clear();
        self.refresh_profile()?;
        self.status = "Answer saved".to_string();
        self.screen = Screen::Reflection;
        Ok(())
    }

    fn save_capsule_from_input(&mut self) -> Result<()> {
        let parts: Vec<&str> = self.input.splitn(3, '|').map(str::trim).collect();
        if parts.len() < 2 || parts[0].is_empty() || parts[1].is_empty() {
            self.status = "Capsule format: title | message | 30d".to_string();
            return Ok(());
        }
        let unlock_spec = parts.get(2).copied().unwrap_or(&self.capsule_unlock);
        let unlock_date = parse_unlock_date(unlock_spec)?;
        self.db.create_capsule(parts[0], parts[1], unlock_date)?;
        self.input.clear();
        self.refresh_profile()?;
        self.status = "Capsule created".to_string();
        Ok(())
    }

    fn next_question(&mut self) -> Result<()> {
        self.current_question = self.db.random_question()?;
        self.input.clear();
        self.screen = Screen::Question;
        self.status = "New question loaded".to_string();
        Ok(())
    }

    fn refresh_profile(&mut self) -> Result<()> {
        self.profile = self.db.profile()?;
        Ok(())
    }

    pub fn journals(&self) -> Vec<Journal> {
        self.db.recent_journals(8).unwrap_or_default()
    }

    pub fn capsules(&self) -> Vec<Capsule> {
        self.db.capsules().unwrap_or_default()
    }

    pub fn recent_answer_lines(&self) -> Vec<String> {
        self.db
            .recent_answers(8)
            .unwrap_or_default()
            .into_iter()
            .map(|answer| {
                format!(
                    "{} | {}",
                    answer.timestamp.format("%Y-%m-%d"),
                    answer.content.replace('\n', " ")
                )
            })
            .collect()
    }
}

fn parse_unlock_date(spec: &str) -> Result<chrono::DateTime<Utc>> {
    let trimmed = spec.trim();
    let now = Utc::now();
    match trimmed {
        "" | "30d" | "30 days" => Ok(now + Duration::days(30)),
        "1y" | "1 year" => Ok(now + Duration::days(365)),
        "5y" | "5 years" => Ok(now + Duration::days(365 * 5)),
        custom => {
            let date = NaiveDate::parse_from_str(custom, "%Y-%m-%d")?;
            let naive = date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| Error::InvalidInput(format!("invalid unlock date: {custom}")))?;
            Ok(Utc.from_utc_datetime(&naive))
        }
    }
}

fn parse_memory_entry(input: &str) -> Option<(&str, &str)> {
    let body = input.trim().strip_prefix("memory:")?.trim();
    let (title, content) = body.split_once('|')?;
    let title = title.trim();
    let content = content.trim();
    if title.is_empty() || content.is_empty() {
        None
    } else {
        Some((title, content))
    }
}

struct TerminalSession {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalSession {
    fn start() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    fn draw<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut ratatui::Frame),
    {
        self.terminal.draw(f)?;
        Ok(())
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}
