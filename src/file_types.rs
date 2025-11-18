use std::{
    fmt,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum Language {
    C,
    Cpp,
    Python,
    Rust,
    Others,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::C => write!(f, "C"),
            Self::Cpp => write!(f, "C++"),
            Self::Python => write!(f, "Python"),
            Self::Rust => write!(f, "Rust"),
            Self::Others => write!(f, "others"),
        }
    }
}

impl Language {
    fn from_extension(extension: &str) -> Self {
        match extension {
            "c" | "h" => Self::C,
            "cpp" | "hpp" => Self::Cpp,
            "py" => Self::Python,
            "rs" => Self::Rust,
            _ => Self::Others,
        }
    }
}

pub struct LanguageStats {
    pub files: usize,
    pub lines: usize,
    pub words: usize,
}

impl LanguageStats {
    pub fn get(filename: &str) -> Option<(Language, Self)> {
        if !Self::is_file(filename) || Self::is_dot_file(filename) {
            return None;
        }
        let extension = filename.split('.').next_back();

        if let Some(ext) = extension {
            let language = Language::from_extension(ext);
            let (lines, words) =
                Self::count_lines_and_words(filename).expect("Counting lines and words failed");

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
            let line = line_result?;
            lines += 1;

            words += line
                .split_whitespace()
                .filter(|word| !word.is_empty())
                .count();
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
