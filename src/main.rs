use std::{collections::HashMap, env};
use walkdir::WalkDir;

mod file_types;

use file_types::{Language, LanguageStats};

#[derive(Default)]
struct Analyzer {
    result: HashMap<Language, LanguageStats>,
}

impl Analyzer {
    fn analyze(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let walker = WalkDir::new(path).into_iter();
        for entry in walker {
            if let Some(file_name) = entry?.path().to_str()
                && let Some((language, stat)) = LanguageStats::get(file_name)
            {
                self.result
                    .entry(language)
                    .and_modify(|s| {
                        s.combine(&stat);
                    })
                    .or_insert(stat);
            }
        }
        Ok(())
    }

    fn show_statistics(&self) {
        for (language, stat) in &self.result {
            println!(
                "{:?}: {}: {}: {}",
                language, stat.files, stat.lines, stat.words
            );
        }
    }
}

struct Config {
    root: String,
}

impl Config {
    fn generate() -> Self {
        let args: Vec<String> = env::args().collect();
        let root = if args.len() < 2 {
            env::current_dir()
                .expect("pwd dont found")
                .into_os_string()
                .into_string()
                .expect("Path to string failed")
        } else {
            args[1].to_string()
        };

        Self { root }
    }
}

fn main() {
    let config = Config::generate();

    let mut analyzer = Analyzer::default();

    if let Err(e) = analyzer.analyze(&config.root) {
        eprintln!("Err: {e}");
    }

    analyzer.show_statistics();
}
