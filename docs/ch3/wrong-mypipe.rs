// Wrong mypipe

extern crate clap;
use clap::{App, Arg};
use std::process::Command;

fn cmd2str(cmd: &mut Command) -> String {
    String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string()
}

fn main() {
    let matches = App::new("mypipe")
        .arg(
            Arg::with_name("input")
                .long("in")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .long("out")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap().to_string();
    let output = matches.value_of("output").unwrap().to_string();
    let buffer = cmd2str(&mut Command::new(input));
    println!("{}", cmd2str(&mut Command::new(output).arg(buffer)));
}