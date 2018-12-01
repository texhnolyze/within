use rayon::prelude::*;

use std::borrow::Cow;
use std::io;
use std::process::{Command, Output, Stdio};

use app::Config;
use errors::handle_error;

pub struct Runner {}

impl Runner {
    pub fn execute_commands(config: Config) -> io::Result<()> {
        (0..config.directories.len())
            .into_par_iter()
            .for_each(|dir_index| {
                let working_dir = config.directories.get(dir_index).unwrap();
                let command_output = Command::new(&config.command)
                    .current_dir(working_dir)
                    .args(&config.command_arguments)
                    .stderr(Stdio::inherit())
                    .output();

                match command_output {
                    Ok(output) => Self::print(output, working_dir.to_string_lossy()),
                    Err(e) => {
                        handle_error(e, format!("{} is not a valid command", &config.command))
                    }
                }
            });

        Ok(())
    }

    fn print(command_output: Output, working_dir: Cow<str>) {
        let command_stdout = String::from_utf8_lossy(&command_output.stdout);

        let output: String = command_stdout
            .lines()
            .map(|line| format!("{}: {} \n", working_dir, line))
            .collect();

        print!("{}", output);
    }
}
