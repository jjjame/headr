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
                .action(ArgAction::Append)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .help("Print the first K lines instead of the first 10")
                .short('n')
                .long("lines")
                .default_value("10")
                .num_args(1),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .help("print the first K bytes of each file")
                .short('c')
                .long("bytes")
                .default_value("10")
                .num_args(1),
        )
        .get_matches();

    let files = args
        .get_many::<String>("files")
        .ok_or("argument not found")?
        .filter_map(|s: &String| s.parse::<String>().ok())
        .collect();

    let lines = args
        .get_one::<String>("lines")
        .ok_or("shouldnt happen")?
        .parse::<usize>()?;

    let bytes = args
        .get_one::<String>("bytes")
        .ok_or("shouldnt happen")?
        .parse::<usize>()?;

    Ok(Config {
        files,
        lines,
        bytes: Some(bytes),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // a zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string())
}
