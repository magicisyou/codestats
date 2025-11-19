use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
};

use super::Language;

pub struct LanguageStats {
    pub files: usize,
    pub lines: usize,
    pub words: usize,
}

impl LanguageStats {
    pub fn get(file_path: &str) -> Option<(Language, Self)> {
        if !Self::is_file(file_path) || Self::is_dot_file(file_path) {
            return None;
        }

        if let Some(extension) = file_path.split('.').next_back() {
            let language = Language::from_extension(extension);
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

    fn count_lines_and_words(path: &str) -> Result<(usize, usize), io::Error> {
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

    fn is_file(path: &str) -> bool {
        let path = Path::new(path);
        match fs::metadata(path) {
            Ok(metadata) => metadata.is_file(),
            Err(_) => false,
        }
    }

    fn is_dot_file(path: &str) -> bool {
        if path.starts_with('.') {
            return true;
        }
        false
    }
}
