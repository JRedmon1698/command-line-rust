use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("1.0")
        .author("compressedonion")
        .arg(
            Arg::new("text")
                .help("input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newlines")
                .short('n')
                .help("do not print new lines")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let mut text_values: Vec<String> = Vec::new();

    if let Ok(Some(values)) = matches.try_get_many::<String>("text") {
        for val in values {
            text_values.push(val.to_string());
        }
    }

    let omit_newlines = matches.get_flag("omit_newlines");

    let print_suffix = if omit_newlines { " " } else { "\n" };

    println!("{}{}", text_values.join(" "), print_suffix);
}
