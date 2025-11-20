use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use super::Language;

pub struct LanguageStats {
    pub files: usize,
    pub lines: usize,
    pub words: usize,
}

impl LanguageStats {
    pub fn get(file_path: PathBuf) -> Option<(Language, Self)> {
        if let Some(language) = Language::from_path(&file_path) {
            let (lines, words) =
                Self::count_lines_and_words(file_path).expect("Counting lines and words failed");

            return Some((
                language,
                Self {
                    files: 1,
                    lines,
                    words,
                },
            ));
        }

        None
    }

    fn count_lines_and_words(path: PathBuf) -> Result<(usize, usize), io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut lines = 0;
        let mut words = 0;

        for line_result in reader.lines() {
            if let Ok(line) = line_result {
                words += line
                    .split_whitespace()
                    .filter(|word| !word.is_empty())
                    .count();
            }
            lines += 1;
        }

        Ok((lines, words))
    }

    pub fn combine(&mut self, other: &Self) {
        self.files += 1;
        self.lines += other.lines;
        self.words += other.words;
    }
}
