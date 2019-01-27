use clap::{Arg, App};

use std::env;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use quick_xml::Reader;
use quick_xml::events::Event;

fn readfile(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
}

fn buildfile(data: String) {
    let mut reader = Reader::from_str(data.as_str());
    reader.trim_text(true);
    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"div" => println!("attributes values: {:?}",
                                        e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()),
                    b"tr" => count += 1,
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => {
                ()//panic!("Error at position {}: {:?}", reader.buffer_position(), e)
            },
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
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
    let data = readfile(path);
    buildfile(data);
}
