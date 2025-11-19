use tabled::Tabled;

use crate::LanguageStats;
use crate::languages::Language;

#[derive(Tabled)]
#[tabled(rename_all = "Upper Title Case")]
pub struct Data {
    pub language: Language,
    pub files: usize,
    pub lines: usize,
    pub words: usize,
}

impl Data {
    pub fn from(language: Language, statistics: LanguageStats) -> Self {
        Self {
            language,
            files: statistics.files,
            lines: statistics.lines,
            words: statistics.words,
        }
    }
}
