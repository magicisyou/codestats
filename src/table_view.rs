use tabled::Tabled;

use crate::LanguageStats;
use crate::languages::Language;

#[derive(Tabled)]
#[tabled(rename_all = "Upper Title Case")]
pub struct Data {
    pub icon: &'static str,
    pub language: Language,
    pub files: usize,
    pub lines: usize,
    pub words: usize,
}

impl Data {
    pub fn from(language: Language, statistics: LanguageStats) -> Self {
        Self {
            icon: language.icon(),
            language,
            files: statistics.files,
            lines: statistics.lines,
            words: statistics.words,
        }
    }
}
