use std::collections::HashMap;

use crate::analysis::patterns::tokenize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeCount {
    pub theme: String,
    pub count: usize,
}

pub fn detect_emotional_themes(texts: &[String]) -> Vec<ThemeCount> {
    let mut counts: HashMap<&'static str, usize> = HashMap::new();
    for text in texts {
        for token in tokenize(text) {
            for (theme, words) in THEME_KEYWORDS {
                if words.contains(&token.as_str()) {
                    *counts.entry(theme).or_default() += 1;
                }
            }
        }
    }

    let mut themes: Vec<ThemeCount> = counts
        .into_iter()
        .map(|(theme, count)| ThemeCount {
            theme: theme.to_string(),
            count,
        })
        .collect();
    themes.sort_by(|left, right| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| left.theme.cmp(&right.theme))
    });
    themes
}

const THEME_KEYWORDS: &[(&str, &[&str])] = &[
    (
        "loneliness",
        &["alone", "lonely", "loneliness", "isolated", "unseen"],
    ),
    (
        "fear",
        &["afraid", "fear", "feared", "anxious", "worry", "dread"],
    ),
    (
        "grief",
        &["grief", "loss", "lost", "mourning", "gone", "absence"],
    ),
    (
        "belonging",
        &["home", "belong", "belonging", "family", "friend", "held"],
    ),
    (
        "memory",
        &[
            "remember",
            "remembered",
            "memory",
            "forgotten",
            "forget",
            "erase",
        ],
    ),
    (
        "hope",
        &["hope", "future", "become", "heal", "light", "possible"],
    ),
    (
        "love",
        &["love", "loved", "tender", "care", "devotion", "heart"],
    ),
    (
        "change",
        &[
            "change",
            "changed",
            "becoming",
            "growth",
            "leaving",
            "returning",
        ],
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_theme_keywords() {
        let themes = detect_emotional_themes(&["I fear being forgotten".to_string()]);
        assert!(themes.iter().any(|theme| theme.theme == "fear"));
        assert!(themes.iter().any(|theme| theme.theme == "memory"));
    }
}
