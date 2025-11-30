use clap::{Command, Arg};
use std::error::Error;``

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn run() -> MyResult {
    let matches = Command::new("catr")
        .version("1.0")
        .author("compressedonion")
        .arg(
            Arg::new("files")
                .help("input files")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .help("number nonempty lines")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("number_lines"),
        )
        .arg(
            Arg::new("number_lines")
                .short('b')
                .help("number all lines")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("number_nonblank_lines"),
        )
        .get_matches();



    Ok(Config {
        
    })
}

