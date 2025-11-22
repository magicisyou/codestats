use spinners::{Spinner, Spinners};
use std::{env, path::PathBuf, sync::mpsc, thread};

use codestats::{Analyzer, Config};

#[derive(Default)]
struct ArgParser {
    root: Option<String>,
    help: bool,
    show_version: bool,
    hide_icons: bool,
    invalid_args: bool,
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
            arg_parser.invalid_args = true;
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

fn check_arguements(arg_parser: &ArgParser) {
    if arg_parser.help || arg_parser.show_version || arg_parser.invalid_args {
        if arg_parser.invalid_args {
            println!("Invalid arguements!");
            print_help_message();
        } else {
            if arg_parser.help {
                print_help_message();
            }
            if arg_parser.show_version {
                show_version();
            }
        }
        std::process::exit(0);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg_parser = ArgParser::parse();

    let config = arg_parser.get_config()?;
    check_arguements(&arg_parser);

    let (tx, rx) = mpsc::channel();

    let mut spinner = Spinner::new(Spinners::Dots, "".into());
    thread::spawn(move || {
        let mut analyzer = Analyzer::new(config);
        match analyzer.analyze() {
            Ok(_) => {
                if let Err(e) = tx.send(analyzer) {
                    eprintln!("Err: {e}");
                }
            }
            Err(e) => eprintln!("Err: {e}"),
        }
    });

    if let Ok(analyzer) = rx.recv() {
        spinner.stop_with_symbol("");
        analyzer.show_table();
    }
    Ok(())
}
