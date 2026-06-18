use crate::models::Profile;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Statistics {
    pub answers: usize,
    pub memories: usize,
    pub journals: usize,
    pub capsules: usize,
}

impl Statistics {
    pub fn from_profile(profile: &Profile) -> Self {
        Self {
            answers: profile.total_answers as usize,
            memories: profile.total_memories as usize,
            journals: profile.total_journal_entries as usize,
            capsules: profile.total_capsules as usize,
        }
    }
}

pub fn profile_summary(profile: &Profile) -> String {
    let statistics = Statistics::from_profile(profile);
    format!(
        "{} days | {} answers | {} memories | {} journal entries | {} capsules",
        profile.total_days,
        statistics.answers,
        statistics.memories,
        statistics.journals,
        statistics.capsules
    )
}
