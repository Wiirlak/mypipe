#[macro_use]
extern crate clap;
use clap::{App, Arg};
use std::process::Command;

fn main() {
//    let yaml = load_yaml!("cli.yml");
//    let matches = App::from_yaml(yaml).get_matches();
    let matches = App::new("MyPipe a moi")
        .about("Pipe things")
        .arg(Arg::with_name("in")
            .short("i")
            .long("in")
            .help("input value for the pipe")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("out")
            .short("o")
            .long("out")
            .help("output value for the pipe")
            .takes_value(true)
            .required(true))
        .get_matches();
    let mut res;
    if let Some(i) = matches.value_of("in") {
        println!("Value for input: {}", i);
        res = Command::new(i.to_string())
            .output()
            .expect("Fail to execute process");
    }else{
        print!("Failed to parse input");
        return;
    }
    if let Some(o) = matches.value_of("out") {
        println!("Value for output: {}", o);
        res = Command::new(o.to_string())
            .arg(String::from_utf8_lossy(&res.stdout).to_string())
            .output()
            .expect("Fail to execute process");
    }else{
        print!("Failed to parse output");
        return;
    }
    println!("{}", String::from_utf8_lossy(&res.stdout).to_string());
}
