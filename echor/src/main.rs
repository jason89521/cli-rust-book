use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Yu Xuan")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text: Vec<&str> = matches
        .get_many::<String>("text")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect();
    let omit_newline = *matches.get_one::<bool>("omit_newline").unwrap_or(&false);
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending)
}
