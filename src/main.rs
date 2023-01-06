use clap::{App, Arg};
use colored::*;

fn main() {
    let matches = App::new("String checker")
        .arg(
            Arg::with_name("string")
                .help("The string to split and search for")
                .required(true)
                .index(1),
        )
        .get_matches();

    let string = matches.value_of("string").unwrap();

    let strings = ["apple", "banana", "cherry"];

    for word in string.split_whitespace() {
        if strings.contains(&word) {
            println!("'{}' found in list", word.bright_red().on_black().bold());
        } else {
            println!("'{}' not found in list", word.blue().on_bright_green().bold());
        }
    }
}
