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
    let mut print_suffix = "\n";

    if let Ok(Some(values)) = matches.try_get_many::<String>("text") {
        for val in values {
            text_values.push(val.to_string());
        }
    }

    let newline_flag = matches.get_flag("omit_newlines");
    if newline_flag {
        print_suffix = " ";
    }

    println!("{}{}", text_values.join(" "), print_suffix);
}
