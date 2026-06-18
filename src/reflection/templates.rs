pub fn concept(term: &str, count: usize) -> String {
    format!("You mentioned {term} {count} times.")
}

pub fn theme(theme: &str, count: usize) -> String {
    format!("You frequently reference {theme}; it appears {count} times in your stored writing.")
}

pub fn answer_history(total: usize) -> String {
    match total {
        0 => "Your archive is waiting for its first answer.".to_string(),
        1 => "This is the first answer in this part of your archive.".to_string(),
        _ => format!("Your archive now holds {total} answers."),
    }
}

pub fn previous_answer(days_ago: i64) -> String {
    match days_ago {
        0 => "You answered this question earlier today.".to_string(),
        1 => "You answered this question differently yesterday.".to_string(),
        days if days >= 730 => {
            let years = days / 365;
            format!("You answered this differently {years} years ago.")
        }
        days => format!("You answered this question differently {days} days ago."),
    }
}

pub fn quiet_start() -> String {
    "A pattern needs time to become visible. This answer is now part of the record.".to_string()
}
