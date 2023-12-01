use std::io::{BufRead, Read};

use clap::{Arg, Command};

use shared::{open, parse_positive_int, MyResult};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(bytes) = config.bytes {
                    let mut handle = file.take(bytes as u64);
                    let mut buffer = vec![0; bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                    }
                    print!("{}", line)
                }
            }
        }
    }

    Ok(())
}

pub fn get_arg() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Yu Xuan")
        .about("Rust head")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input file(s) [default: -]")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Number of lines [default: 10]")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .help("Number of bytes")
                .conflicts_with("lines"),
        )
        .get_matches();

    let files = matches
        .get_many::<String>("files")
        .unwrap_or_default()
        .map(|v| v.into())
        .collect::<Vec<String>>();
    let lines = parse_positive_int(matches.get_one::<String>("lines").unwrap())
        .map_err(|e| format!("illegal line count -- {e}"))?;
    let bytes = matches
        .get_one::<String>("bytes")
        .map(|v| parse_positive_int(v))
        .transpose()
        .map_err(|e| format!("illegal byte count -- {e}"))?;

    Ok(Config {
        files,
        lines,
        bytes,
    })
}
