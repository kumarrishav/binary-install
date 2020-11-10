use clap::{App, Arg, SubCommand};
use console::Emoji;
use std::env;
use std::io::{self, Read};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("c")
                .short("c")
                .multiple(true)
                .help("Counts the number of -c arguments"),
        )
        .subcommand(SubCommand::with_name("echo").about("echo stdin to stdout"))
        .get_matches();

    let c_num = matches.occurrences_of("c");

    if c_num == 0 {
        eprintln!("{} you didn't pass any -c arguments ", Emoji("🥺 ", ""))
    } else {
        println!("{} you passed {} -c arguments!", Emoji("🥰 ", ""), c_num);
    }

    if let Some(_) = matches.subcommand_matches("echo") {
        let mut buf = String::new();
        let mut stdin = io::stdin();
        stdin.read_to_string(&mut buf).unwrap();
        println!("{}", buf);
    }
}
