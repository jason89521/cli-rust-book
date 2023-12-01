use std::io::BufRead;

use clap::{Arg, ArgAction, Command};

use shared::{open, MyResult};

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
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Yu Xuan")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input files")
                .num_args(1..),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let files = matches
        .get_many::<String>("files")
        .unwrap_or_default()
        .map(|v| v.into())
        .collect::<Vec<String>>();
    let number_lines = *matches.get_one::<bool>("number_lines").unwrap_or(&false);
    let number_nonblank_lines = *matches
        .get_one::<bool>("number_nonblank_lines")
        .unwrap_or(&false);

    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}
