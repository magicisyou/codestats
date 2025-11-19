use ignore::Walk;
use std::collections::HashMap;
use tabled::{Table, settings::Style};

mod language_stats;
mod languages;
mod table_view;

use language_stats::LanguageStats;
use languages::Language;
use table_view::Data;

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
            if let Some(file_path) = entry?.path().to_str()
                && let Some((language, statistics)) = LanguageStats::get(file_path)
            {
                self.result
                    .entry(language)
                    .and_modify(|existing_statistics| {
                        existing_statistics.combine(&statistics);
                    })
                    .or_insert(statistics);
            }
        }
        Ok(())
    }

    pub fn show_table(self) {
        let data: Vec<Data> = self
            .result
            .into_iter()
            .map(|(language, statistics)| Data::from(language, statistics))
            .collect();

        let mut table = Table::new(&data);
        table.with(Style::rounded());

        println!("{}", table);
    }
}
