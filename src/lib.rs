use ignore::Walk;
use std::collections::HashMap;
use tabled::{Table, settings::Style};

mod language_stats;
mod languages;
mod table_view;

use language_stats::LanguageStats;
use languages::Language;

pub struct Analyzer {
    root: String,
    result: HashMap<Language, LanguageStats>,
}

impl Analyzer {
    pub fn new(root: String) -> Self {
        Self {
            root,
            result: HashMap::new(),
        }
    }

    pub fn analyze(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for entry in Walk::new(&self.root) {
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

    pub fn show_table(&self) {
        let mut data = vec![];
        for (language, stat) in &self.result {
            data.push(table_view::Data {
                language: language.clone(),
                files: stat.files,
                lines: stat.lines,
                words: stat.words,
            });
        }

        let mut table = Table::new(data);
        table.with(Style::sharp());
        println!("{}", table);
    }
}
