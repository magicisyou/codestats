use ignore::Walk;
use std::{collections::HashMap, path::PathBuf};
use tabled::{
    Table,
    settings::{
        Color, Remove, Style,
        object::{Columns, Rows},
        style::LineText,
    },
};

mod language_stats;
mod languages;
mod table_view;

use language_stats::LanguageStats;
use languages::Language;
use table_view::Data;

pub struct Config {
    pub root: PathBuf,
    pub icons: bool,
}

pub struct Analyzer {
    config: Config,
    result: HashMap<Language, LanguageStats>,
}

impl Analyzer {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            result: HashMap::new(),
        }
    }

    pub fn analyze(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for entry in Walk::new(&self.config.root) {
            let file_path = entry?.into_path();
            if let Some((language, statistics)) = LanguageStats::get(file_path) {
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

        let style = Style::rounded();
        let mut table = Table::new(&data);

        if !self.config.icons {
            table.with(Remove::column(Columns::first()));
        }

        table.with(style).modify(Rows::first(), Color::FG_GREEN);

        if let Some(filename) = self.config.root.file_name() {
            table.with(
                LineText::new(format!("{}", filename.display()), Rows::first())
                    .offset(2)
                    .color(Color::BOLD),
            );
        }

        println!("{}", table);
    }
}
