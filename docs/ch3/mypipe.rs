// Good mypipe (by Théo H)

extern crate clap;
use clap::{App, Arg};
use std::process::{Command, Stdio};

fn main() {
    let matches = App::new("mypipe")
        .version("1.0")
        .about("Pipe program")
        .author("Théo H")
        .arg(
            Arg::with_name("in")
                .short("i")
                .long("in")
                .value_name("INPUT")
                .help("INPUT CMD")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .value_name("OUTPUT")
                .help("OUTPUT CMD")
                .takes_value(true),
        )
        .get_matches();

    let input_cmd = match matches.value_of("in") {
        Some(x) => x,
        None => panic!("No input command given"),
    };
    let output_cmd = match matches.value_of("out") {
        Some(x) => x,
        None => panic!("No output command given"),
    };

    let process_one = match Command::new(input_cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn {} : {}", input_cmd, why),
        Ok(process) => process,
    };

    let process_two = match Command::new(output_cmd)
        .stdin(process_one.stdout.unwrap())
        .output()
    {
        Err(why) => panic!("couldn't spawn {} : {}", input_cmd, why),
        Ok(process) => process,
    };

    println!("{}", String::from_utf8_lossy(&process_two.stdout));
}