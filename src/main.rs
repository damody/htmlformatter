use clap::{Arg, App};

use std::env;
use std::fs::File;
use std::fs;
use std::io::prelude::*;


fn readfile(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
}

fn main() {
    let matches = App::new("Html Formatter")
        .version("0.1.0")
        .author("damody <t1238142000@gmail.com>")
        .about("Html Formatter written in Rust")
        .arg(Arg::with_name("PATH")
            .required(false)
            .takes_value(true)
            .index(1)
            .help("path to format"))
        .arg(Arg::with_name("name")
            .short("n")
            .multiple(true)
            .help("format the file"))
        .get_matches();
    let cur_path:String = std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string();
    let path = matches.value_of("PATH").unwrap_or(cur_path.as_str());
    println!("{}", path);
    println!("{}", readfile(path));
}
