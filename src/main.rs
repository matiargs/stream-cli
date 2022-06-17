use chrono::{DateTime, Local};
use clap::{Arg, Command};
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use url::Url;

fn main() {
    // obtain path from env and pass to write_to_file method
    let path = env::var("STREAM_FILE_PATH").expect("Couldn't read PATH");

    let app = Command::new("Stream Cli")
        .version("0.1.0")
        .author("Matias Micheltorena <m@matiargs.com>")
        .about("Simple CLI tool to log streams of thoughts and links.")
        .arg(
            Arg::new("thought")
                .help("Process thought")
                .short('t')
                .long("thought")
                .takes_value(true),
        )
        .arg(
            Arg::new("url")
            .help("Process url")
            .short('u')
            .long("url")
            .takes_value(true)
        )
        .get_matches();

    let thought = app.value_of("thought");
    match thought {
        None => {}
        Some(s) => {
            write_to_file(s.to_string(), path.to_string());
            println!("✨ Thought processed!");
        }
    }

    let b = app.value_of("url");
    match b {
        None => {}
        Some(s) => {
            let link_processed = process_link(s);
            write_to_file(link_processed, path.to_string());
            println!("✨ URL processed!");
        }
    }
}

fn process_link(link: &str) -> String {
    let link_data = Url::parse(link).expect("BAD URL");
    let mut path = link_data.path().to_string();

    if path.ends_with('/') {
        path.pop();
    }
    
    let last = path.split('/').last().unwrap();
    let link_formatted = format!("[{}]({})", last, link);
    link_formatted
}

fn write_to_file(text: String, path: String) {
    let mut note = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    let now: DateTime<Local> = Local::now();
    let text = format!("{} - {}", now.format("%F %H:%M:%S").to_string(), text);

    if let Err(e) = writeln!(note, "{}", text) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
