use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(buf) => {
                let mut line_num = 0usize;
                let mut lines: Vec<String> = vec![];
                for line_result in buf.lines() {
                    let line = line_result?;
                    line_num += 1;
                    if config.number_lines {
                        lines.push(format!("{:>6}\t{}", line_num, line));
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            line_num -= 1;
                            lines.push("".into());
                        } else {
                            lines.push(format!("{:>6}\t{}", line_num, line))
                        }
                    } else {
                        lines.push(format!("{}", line))
                    }
                }

                let result = lines.join("\n");
                print!("{}", result)
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Yu Xuan")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let number_lines = matches.is_present("number_lines");
    let number_nonblank_lines = matches.is_present("number_nonblank_lines");

    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        f => Ok(Box::new(BufReader::new(File::open(f)?))),
    }
}
