use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConceptCount {
    pub term: String,
    pub count: usize,
}

pub fn detect_repeated_concepts(texts: &[String]) -> Vec<ConceptCount> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for text in texts {
        for token in tokenize(text) {
            if is_meaningful(&token) {
                *counts.entry(token).or_default() += 1;
            }
        }
    }

    let mut concepts: Vec<ConceptCount> = counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(term, count)| ConceptCount { term, count })
        .collect();
    concepts.sort_by(|left, right| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| left.term.cmp(&right.term))
    });
    concepts
}

pub fn tokenize(text: &str) -> Vec<String> {
    text.split(|character: char| !character.is_alphanumeric() && character != '\'')
        .filter_map(|token| {
            let normalized = token.trim_matches('\'').to_lowercase();
            if normalized.is_empty() {
                None
            } else {
                Some(normalized)
            }
        })
        .collect()
}

fn is_meaningful(token: &str) -> bool {
    token.len() >= 4 && !STOP_WORDS.contains(&token)
}

const STOP_WORDS: &[&str] = &[
    "about",
    "after",
    "again",
    "also",
    "because",
    "been",
    "being",
    "could",
    "does",
    "from",
    "have",
    "into",
    "just",
    "like",
    "more",
    "most",
    "only",
    "over",
    "really",
    "some",
    "than",
    "that",
    "their",
    "them",
    "then",
    "there",
    "these",
    "they",
    "this",
    "through",
    "today",
    "very",
    "want",
    "were",
    "what",
    "when",
    "where",
    "which",
    "with",
    "would",
    "your",
    "youre",
    "myself",
    "ourselves",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_concepts_are_counted() {
        let concepts = detect_repeated_concepts(&[
            "I felt loneliness and fear".to_string(),
            "Loneliness returned with fear".to_string(),
        ]);
        assert_eq!(concepts[0].term, "fear");
        assert_eq!(concepts[0].count, 2);
    }
}
