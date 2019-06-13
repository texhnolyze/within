use rayon::prelude::*;

use std::io;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

use crate::app::Config;
use crate::errors::handle_error;

pub struct Runner {}

impl Runner {
    pub fn execute_commands(config: &Config) -> io::Result<()> {
        (0..config.directories.len())
            .into_par_iter()
            .for_each(|directory_index| Self::run_comamnd_for_directory(&config, directory_index));

        Ok(())
    }

    fn run_comamnd_for_directory(config: &Config, directory_index: usize) {
        let working_directory = config.directories.get(directory_index).unwrap();
        let command_output = Self::run_command(config, working_directory);

        match command_output {
            Ok(output) => print!("{}", Self::format_command_output(output, working_directory)),
            Err(e) => handle_error(e, format!("{} is not a valid command", &config.command)),
        }
    }

    fn run_command(config: &Config, working_dir: &PathBuf) -> io::Result<(Output)> {
        return Command::new(&config.command)
            .current_dir(working_dir)
            .args(&config.command_arguments)
            .stderr(Stdio::inherit())
            .output();
    }

    fn format_command_output(command_output: Output, working_directory: &PathBuf) -> String {
        let command_stdout = String::from_utf8_lossy(&command_output.stdout);

        return command_stdout
            .lines()
            .map(|line| format!("{}: {} \n", working_directory.to_string_lossy(), line))
            .collect();
    }
}
