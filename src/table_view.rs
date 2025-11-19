use tabled::Tabled;

use crate::languages::Language;

#[derive(Tabled)]
#[tabled(rename_all = "Upper Title Case")]
pub struct Data {
    pub language: Language,
    pub files: usize,
    pub lines: usize,
    pub words: usize,
}
