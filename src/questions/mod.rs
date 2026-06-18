pub mod base;
pub mod deep;
pub mod existential;

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::errors::{Error, Result};
use crate::models::Question;

#[derive(Debug, Deserialize)]
struct QuestionFile {
    category: String,
    prompts: Vec<String>,
}

pub fn load_question_bank(path: &Path) -> Result<Vec<Question>> {
    let files = [
        base::file_name(),
        deep::file_name(),
        existential::file_name(),
    ];

    let mut questions = Vec::new();
    for file_name in files {
        let file_path = path.join(file_name);
        questions.extend(load_file(&file_path)?);
    }

    if questions.is_empty() {
        return Err(Error::NoQuestions(path.display().to_string()));
    }

    Ok(questions)
}

fn load_file(path: &PathBuf) -> Result<Vec<Question>> {
    let content = fs::read_to_string(path)?;
    let bank: QuestionFile = serde_json::from_str(&content)?;
    let category = bank.category.trim().to_lowercase();
    let questions = bank
        .prompts
        .into_iter()
        .enumerate()
        .filter_map(|(index, prompt)| {
            let prompt = prompt.trim().to_string();
            if prompt.is_empty() {
                None
            } else {
                Some(Question::new(
                    format!("{category}-{:03}", index + 1),
                    category.clone(),
                    prompt,
                ))
            }
        })
        .collect();
    Ok(questions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_questions_load() {
        let questions = load_question_bank(Path::new(".resources/questions")).unwrap();
        assert!(questions.len() >= 12);
        assert!(questions
            .iter()
            .any(|question| question.category == "existential"));
    }
}
