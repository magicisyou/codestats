use ignore::Walk;
use std::{collections::HashMap, env};
use tabled::settings::Style;

mod file_types;
mod table_view;

use file_types::{Language, LanguageStats};

#[derive(Default)]
struct Analyzer {
    result: HashMap<Language, LanguageStats>,
}

impl Analyzer {
    fn analyze(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // let walker = WalkDir::new(path).into_iter();
        // for entry in walker {
        for entry in Walk::new(path) {
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

    fn get_data(&self) -> Vec<table_view::Data> {
        let mut data = vec![];
        for (language, stat) in &self.result {
            data.push(table_view::Data {
                language: language.clone(),
                files: stat.files,
                lines: stat.lines,
                words: stat.words,
            });
        }
        data
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

    // analyzer.show_statistics();
    let mut table = tabled::Table::new(analyzer.get_data());
    table.with(Style::modern());
    println!("{}", table);
}
