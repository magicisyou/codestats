use std::{env, path::PathBuf};

use codestats::Analyzer;

struct Config {
    root: PathBuf,
}

impl Config {
    fn generate() -> Self {
        let args: Vec<String> = env::args().collect();
        let root = if args.len() < 2 {
            env::current_dir().expect("Err: pwd determination failed")
        } else {
            PathBuf::from(&args[1])
                .canonicalize()
                .expect("Path canonicalize failed")
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
