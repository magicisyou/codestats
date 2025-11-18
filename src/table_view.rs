use tabled::Tabled;

use super::file_types::Language;

#[derive(Tabled)]
pub struct Data {
    pub language: Language,
    pub files: usize,
    pub lines: usize,
    pub words: usize,
}
