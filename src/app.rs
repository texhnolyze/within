use ansi_term::Colour::Yellow;
use clap::{App as ClapApp, Arg, ArgMatches};

use std::path::PathBuf;

use crate::errors::handle_error;

pub struct App<'a> {
    matches: ArgMatches<'a>,
}

pub struct Config<'a> {
    pub command: &'a str,
    pub command_arguments: Vec<&'a str>,
    pub directories: Vec<PathBuf>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            matches: Self::matches(),
        }
    }

    pub fn config(&self) -> Config {
        let command_values: Vec<&str> = self.matches.values_of("command").unwrap().collect();
        let directories: Vec<PathBuf> = self
            .matches
            .values_of("directory")
            .unwrap()
            .filter_map(into_path)
            .collect();

        Config {
            command: command_values[0],
            command_arguments: command_values[1..].to_vec(),
            directories,
        }
    }

    fn matches() -> ArgMatches<'a> {
        Self::build_app().get_matches()
    }

    fn build_app() -> ClapApp<'a, 'a> {
        ClapApp::new("within")
            .version("0.1.0")
            .author("texhnolyze <joegriepen@gmail.com>")
            .about("Runs a command within list of given directories")
            .arg(
                Arg::with_name("directory")
                    .value_name("DIRECTORY")
                    .help("Sets DIRECTORY to execute COMMAND in")
                    .required(true)
                    .multiple(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("command")
                    .value_name("COMMAND")
                    .help("COMMAND to execute in DIRECTORY")
                    .required(true)
                    .raw(true),
            )
            .help_message("Print this help message.")
            .version_message("Show version information.")
            .after_help(
                "EXAMPLES:\n    within ~/test1 ~/test2 -- ls\n    within ~/documents/* -- git add",
            )
    }
}



fn into_path(path_string: &str) -> Option<PathBuf> {
    let path = PathBuf::from(path_string).canonicalize();

    match path {
        Ok(path) => {
            if path.is_dir() {
                Some(path)
            } else {
                eprintln!(
                    "{}: {} is not a directory",
                    Yellow.paint("[within warning]"),
                    path_string
                );
                None
            }
        }
        Err(e) => {
            handle_error(e, format!("{} does not exist", path_string));
            None
        }
    }
}
