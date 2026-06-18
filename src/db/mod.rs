pub mod migrations;
pub mod schema;

use std::fs;
use std::path::Path;

use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rusqlite::{params, Connection, OptionalExtension};

use crate::errors::{Error, Result};
use crate::models::{Answer, Capsule, Journal, Memory, Profile, Question};
use crate::types::{from_db_time, to_db_time};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;",
        )?;
        migrations::run(&conn)?;
        Ok(Self { conn })
    }

    pub fn upsert_questions(&mut self, questions: &[Question]) -> Result<()> {
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO questions (id, category, prompt, created_at)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(id) DO UPDATE SET
                    category = excluded.category,
                    prompt = excluded.prompt",
            )?;
            for question in questions {
                stmt.execute(params![
                    question.id,
                    question.category,
                    question.prompt,
                    to_db_time(question.created_at),
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn questions(&self) -> Result<Vec<Question>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, category, prompt, created_at
             FROM questions
             ORDER BY category, id",
        )?;
        let rows = stmt.query_map([], |row| {
            let created_at: String = row.get(3)?;
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, created_at))
        })?;

        let mut questions = Vec::new();
        for row in rows {
            let (id, category, prompt, created_at): (String, String, String, String) = row?;
            questions.push(Question {
                id,
                category,
                prompt,
                created_at: from_db_time(&created_at)?,
            });
        }
        Ok(questions)
    }

    pub fn random_question(&self) -> Result<Question> {
        let questions = self.questions()?;
        questions
            .choose(&mut thread_rng())
            .cloned()
            .ok_or_else(|| Error::NoQuestions("database".to_string()))
    }

    pub fn save_answer(&self, question_id: &str, content: &str) -> Result<Answer> {
        let timestamp = Utc::now();
        self.conn.execute(
            "INSERT INTO answers (question_id, content, timestamp) VALUES (?1, ?2, ?3)",
            params![question_id, content.trim(), to_db_time(timestamp)],
        )?;
        self.mark_started()?;
        Ok(Answer {
            id: self.conn.last_insert_rowid(),
            question_id: question_id.to_string(),
            content: content.trim().to_string(),
            timestamp,
        })
    }

    pub fn recent_answers(&self, limit: usize) -> Result<Vec<Answer>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, question_id, content, timestamp
             FROM answers
             ORDER BY timestamp DESC
             LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            let timestamp: String = row.get(3)?;
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, timestamp))
        })?;
        let mut answers = Vec::new();
        for row in rows {
            let (id, question_id, content, timestamp): (i64, String, String, String) = row?;
            answers.push(Answer {
                id,
                question_id,
                content,
                timestamp: from_db_time(&timestamp)?,
            });
        }
        Ok(answers)
    }

    pub fn all_answers(&self) -> Result<Vec<Answer>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, question_id, content, timestamp
             FROM answers
             ORDER BY timestamp ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            let timestamp: String = row.get(3)?;
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, timestamp))
        })?;
        let mut answers = Vec::new();
        for row in rows {
            let (id, question_id, content, timestamp): (i64, String, String, String) = row?;
            answers.push(Answer {
                id,
                question_id,
                content,
                timestamp: from_db_time(&timestamp)?,
            });
        }
        Ok(answers)
    }

    pub fn previous_answer_for_question(
        &self,
        question_id: &str,
        exclude_id: i64,
    ) -> Result<Option<Answer>> {
        let row = self
            .conn
            .query_row(
                "SELECT id, question_id, content, timestamp
                 FROM answers
                 WHERE question_id = ?1 AND id != ?2
                 ORDER BY timestamp DESC
                 LIMIT 1",
                params![question_id, exclude_id],
                |row| {
                    let timestamp: String = row.get(3)?;
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        timestamp,
                    ))
                },
            )
            .optional()?;

        row.map(|(id, question_id, content, timestamp)| {
            Ok(Answer {
                id,
                question_id,
                content,
                timestamp: from_db_time(&timestamp)?,
            })
        })
        .transpose()
    }

    pub fn save_journal(&self, content: &str) -> Result<Journal> {
        let timestamp = Utc::now();
        self.conn.execute(
            "INSERT INTO journals (content, timestamp) VALUES (?1, ?2)",
            params![content.trim(), to_db_time(timestamp)],
        )?;
        self.mark_started()?;
        Ok(Journal {
            id: self.conn.last_insert_rowid(),
            content: content.trim().to_string(),
            timestamp,
        })
    }

    pub fn recent_journals(&self, limit: usize) -> Result<Vec<Journal>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content, timestamp FROM journals ORDER BY timestamp DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            let timestamp: String = row.get(2)?;
            Ok((row.get(0)?, row.get(1)?, timestamp))
        })?;
        let mut journals = Vec::new();
        for row in rows {
            let (id, content, timestamp): (i64, String, String) = row?;
            journals.push(Journal {
                id,
                content,
                timestamp: from_db_time(&timestamp)?,
            });
        }
        Ok(journals)
    }

    pub fn save_memory(&self, title: &str, content: &str) -> Result<Memory> {
        let timestamp = Utc::now();
        self.conn.execute(
            "INSERT INTO memories (title, content, timestamp) VALUES (?1, ?2, ?3)",
            params![title.trim(), content.trim(), to_db_time(timestamp)],
        )?;
        self.mark_started()?;
        Ok(Memory {
            id: self.conn.last_insert_rowid(),
            title: title.trim().to_string(),
            content: content.trim().to_string(),
            timestamp,
        })
    }

    pub fn create_capsule(
        &self,
        title: &str,
        content: &str,
        unlock_date: DateTime<Utc>,
    ) -> Result<Capsule> {
        let created_at = Utc::now();
        self.conn.execute(
            "INSERT INTO capsules (title, content, unlock_date, created_at, opened)
             VALUES (?1, ?2, ?3, ?4, 0)",
            params![
                title.trim(),
                content.trim(),
                to_db_time(unlock_date),
                to_db_time(created_at)
            ],
        )?;
        self.mark_started()?;
        Ok(Capsule {
            id: self.conn.last_insert_rowid(),
            title: title.trim().to_string(),
            content: content.trim().to_string(),
            unlock_date,
            created_at,
            opened: false,
        })
    }

    pub fn capsules(&self) -> Result<Vec<Capsule>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, content, unlock_date, created_at, opened
             FROM capsules
             ORDER BY unlock_date ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            let unlock_date: String = row.get(3)?;
            let created_at: String = row.get(4)?;
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                unlock_date,
                created_at,
                row.get::<_, i64>(5)?,
            ))
        })?;
        let mut capsules = Vec::new();
        for row in rows {
            let (id, title, content, unlock_date, created_at, opened): (
                i64,
                String,
                String,
                String,
                String,
                i64,
            ) = row?;
            capsules.push(Capsule {
                id,
                title,
                content,
                unlock_date: from_db_time(&unlock_date)?,
                created_at: from_db_time(&created_at)?,
                opened: opened != 0,
            });
        }
        Ok(capsules)
    }

    pub fn profile(&self) -> Result<Profile> {
        let first_run =
            self.conn
                .query_row("SELECT first_run FROM profile WHERE id = 1", [], |row| {
                    let value: i64 = row.get(0)?;
                    Ok(value != 0)
                })?;

        let total_answers = self.count("answers")?;
        let total_memories = self.count("memories")?;
        let total_journal_entries = self.count("journals")?;
        let total_capsules = self.count("capsules")?;
        let total_days: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM (
                SELECT substr(timestamp, 1, 10) AS day FROM answers
                UNION
                SELECT substr(timestamp, 1, 10) AS day FROM memories
                UNION
                SELECT substr(timestamp, 1, 10) AS day FROM journals
                UNION
                SELECT substr(created_at, 1, 10) AS day FROM capsules
             )",
            [],
            |row| row.get(0),
        )?;

        Ok(Profile {
            first_run,
            total_days,
            total_answers,
            total_memories,
            total_journal_entries,
            total_capsules,
        })
    }

    fn count(&self, table: &str) -> Result<i64> {
        let sql = format!("SELECT COUNT(*) FROM {table}");
        Ok(self.conn.query_row(&sql, [], |row| row.get(0))?)
    }

    fn mark_started(&self) -> Result<()> {
        self.conn
            .execute("UPDATE profile SET first_run = 0 WHERE id = 1", [])?;
        Ok(())
    }
}
