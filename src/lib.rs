use clap::{Arg, ArgAction, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let args = Command::new("headr")
        .version("0.1")
        .author("jjjame")
        .about("rust head")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("list of files")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .help("Print the first K lines instead of the first 10")
                .short('n')
                .long("lines")
                .num_args(1),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .help("print the first K bytes of each file")
                .short('c')
                .long("bytes")
                .num_args(1),
        )
        .get_matches();

    let files = args
        .get_many::<String>("files")
        .ok_or("argument not found")?
        .filter_map(|s: &String| s.parse::<String>().ok())
        .collect();

    Ok(Config {
        files,
        lines,
        bytes,
    })
}
