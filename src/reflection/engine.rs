use chrono::Utc;

use crate::analysis::patterns::detect_repeated_concepts;
use crate::analysis::themes::detect_emotional_themes;
use crate::models::Answer;
use crate::reflection::templates;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReflectionReport {
    pub insights: Vec<String>,
}

impl ReflectionReport {
    pub fn single(line: impl Into<String>) -> Self {
        Self {
            insights: vec![line.into()],
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ReflectionEngine;

impl ReflectionEngine {
    pub fn generate(
        answers: &[Answer],
        previous_same_question: Option<&Answer>,
    ) -> ReflectionReport {
        let texts: Vec<String> = answers
            .iter()
            .map(|answer| answer.content.clone())
            .collect();
        let mut insights = Vec::new();

        insights.push(templates::answer_history(answers.len()));

        if let Some(previous) = previous_same_question {
            let days = (Utc::now() - previous.timestamp).num_days();
            insights.push(templates::previous_answer(days));
        }

        for concept in detect_repeated_concepts(&texts).into_iter().take(3) {
            insights.push(templates::concept(&concept.term, concept.count));
        }

        for theme in detect_emotional_themes(&texts).into_iter().take(2) {
            insights.push(templates::theme(&theme.theme, theme.count));
        }

        if insights.len() == 1 {
            insights.push(templates::quiet_start());
        }

        ReflectionReport { insights }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn reflection_mentions_repeated_concepts() {
        let answers = vec![
            Answer {
                id: 1,
                question_id: "base-001".to_string(),
                content: "loneliness and fear".to_string(),
                timestamp: Utc::now() - Duration::days(3),
            },
            Answer {
                id: 2,
                question_id: "base-002".to_string(),
                content: "fear of loneliness".to_string(),
                timestamp: Utc::now(),
            },
        ];
        let report = ReflectionEngine::generate(&answers, None);
        assert!(report.insights.iter().any(|line| line.contains("fear")));
    }
}
