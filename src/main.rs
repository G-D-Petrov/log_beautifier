pub mod arg_parser;
use std::{fs::File, io::BufReader};

use arg_parser::ArgumentParser;
use clap::Parser;
use serde::{Deserialize, Serialize};
use rayon::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Filter {
    key: String,
    #[serde(default = "default_offset")]
    offset: u8,
    #[serde(default = "default_color")]
    color: Color,
    #[serde(default = "default_icon")]
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn default_offset() -> u8 {
    0
}

fn default_color() -> Color {
    // generate random color
    Color {
        r: rand::random::<u8>(),
        g: rand::random::<u8>(),
        b: rand::random::<u8>(),
    }
}

fn default_icon() -> String {
    String::from("")
}

#[derive(Serialize, Deserialize, Debug)]
struct Profile {
    values: Vec<Filter>,
}

fn read_file(path: &String) -> BufReader<File> {
    let f = File::open(path).expect(format!("Unable to read file: {}", path).as_str());
    
    BufReader::new(f)
}

fn read_files(args: &ArgumentParser) -> (Profile, String) {
    let profile_file = read_file(&args.profile);
    let log_file = std::fs::read_to_string(&args.log).expect("Unable to read log file");

    let profile: Profile = serde_json::from_reader(profile_file).expect("Unable to parse profile file");

    (profile, log_file)
}

fn get_colored_box(filter: &Filter) -> String {
    let color = format!("\x1b[48;2;{};{};{}m", filter.color.r, filter.color.g, filter.color.b);
    let reset = "\x1b[0m";

    format!("{}{}{}", color, "    ", reset)
}

fn print_line_to_terminal(line: &str, filter: &Filter) {
    let colored_box = get_colored_box(filter);
    let mut offset = String::new();
    for _ in 0..filter.offset {
        offset += " ";
    }
    println!("{}{} {}", offset, colored_box, line);
}

fn remove_lines_without_identifier<'a>(lines: Vec<&'a str>, identifier: &'a str) -> Vec<&'a str> {
    lines.par_iter().filter(|line| line.contains(identifier)).map(|line| *line).collect()
}

fn filter_lines<'a>(lines: Vec<&'a str>, profile: &'a Profile) -> Vec<(&'a Filter, String)> {
    lines.par_iter().filter(|line| {
        for filter in &profile.values {
            if line.contains(filter.key.as_str()) {
                return true;
            }
        }
        false
    }).map(|line| {
        for filter in &profile.values {
            if line.contains(filter.key.as_str()) {
                return (filter, line.to_string());
            }
        }
        unreachable!()
    }).collect()
}

fn main() {
    let args = ArgumentParser::parse();
    let (profile, log) = read_files(&args);

    let lines:Vec<&str> = log.lines().collect();
    
    let lines = if args.identifier != "" {
        remove_lines_without_identifier(lines, args.identifier.as_str())
    } else {
        lines
    };
    // create a vector of tuples that will contain the filter and the line
    let filtered_lines: Vec<(&Filter, String)> = filter_lines(lines, &profile);

    for (filter, line)  in filtered_lines {
        print_line_to_terminal(&line, filter);
    } 
    
}