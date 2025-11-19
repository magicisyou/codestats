use std::env;

use codestats::Analyzer;

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

    let mut analyzer = Analyzer::new(config.root);

    if let Err(e) = analyzer.analyze() {
        eprintln!("Err: {e}");
    } else {
        analyzer.show_table();
    }
}
