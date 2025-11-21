use std::{env, path::PathBuf};

use codestats::{Analyzer, Config};

#[derive(Default)]
struct ArgParser {
    root: Option<String>,
    help: bool,
    show_version: bool,
    hide_icons: bool,
    too_many_args: bool,
}

impl ArgParser {
    fn parse() -> Self {
        let mut args: Vec<String> = env::args().collect();
        let mut arg_parser = Self::default();

        args.retain(|arg| {
            if arg == &String::from("--help") {
                arg_parser.help = true;
                false
            } else if arg == &String::from("--version") {
                arg_parser.show_version = true;
                false
            } else if arg == &String::from("--no-icons") {
                arg_parser.hide_icons = true;
                false
            } else {
                true
            }
        });

        if args.len() == 2 {
            arg_parser.root = Some(args[1].clone());
        } else if args.len() > 2 {
            arg_parser.too_many_args = true;
        }

        arg_parser
    }

    fn get_config(&self) -> std::io::Result<Config> {
        match &self.root {
            None => Ok(Config {
                root: env::current_dir()?,
                icons: !self.hide_icons,
            }),
            Some(r) => Ok(Config {
                root: PathBuf::from(r).canonicalize()?,
                icons: !self.hide_icons,
            }),
        }
    }
}

fn print_help_message() {
    println!("codestats ");
    println!("codestats [PATH]");
    println!("codestats --help");
    println!("codestats --version");
    println!("codestats --no-icons");
}

fn show_version() {
    println!("codestats v0.1.0");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg_parser = ArgParser::parse();

    if arg_parser.too_many_args {
        println!("Too many arguements! Only one project path is accepted");
        print_help_message();
        std::process::exit(0);
    }

    if arg_parser.help || arg_parser.show_version {
        if arg_parser.help {
            print_help_message();
        }
        if arg_parser.show_version {
            show_version();
        }
        std::process::exit(0);
    }

    let config = arg_parser.get_config()?;

    let mut analyzer = Analyzer::new(config);
    analyzer.analyze()?;
    analyzer.show_table();
    Ok(())
}
