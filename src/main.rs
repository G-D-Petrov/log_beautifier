pub mod arg_parser;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use arg_parser::ArgumentParser;
use clap::Parser;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

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

fn read_file(path: &str) -> BufReader<File> {
    let f = File::open(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path));

    BufReader::new(f)
}

fn read_profile(path: &str) -> Profile {
    let file = read_file(path);
    serde_json::from_reader(file).expect("Unable to parse profile file")
}

fn read_files(args: &ArgumentParser) -> (Profile, String) {
    let profile = read_profile(&args.profile);
    let log_file = std::fs::read_to_string(&args.log).expect("Unable to read log file");

    (profile, log_file)
}

fn get_colored_box(filter: &Filter) -> String {
    let color = format!(
        "\x1b[48;2;{};{};{}m",
        filter.color.r, filter.color.g, filter.color.b
    );
    let reset = "\x1b[0m";

    format!("{}{}{}", color, "    ", reset)
}

fn color_ips(line: &str, ips: &Vec<String>, identifier: &str) -> String {
    let mut colored_line = String::from(line);
    for ip in ips {
        let octets: Vec<&str> = ip.split('.').collect();
        colored_line = colored_line.replace(
            ip.as_str(),
            format!(
                "\x1b[38;2;{};{};{}m{}\x1b[0m",
                octets[1], octets[2], octets[3], ip
            )
            .as_str(),
        );
    }

    // if identifier is provided, color it in yellow
    if !identifier.is_empty() {
        colored_line = colored_line.replace(
            identifier,
            format!("\x1b[38;2;{};{};{}m{}\x1b[0m", 255, 255, 0, identifier).as_str(),
        );
    }

    colored_line
}

fn print_line_to_terminal(line: &FilteredLine, indetifier: &str) {
    let colored_box = get_colored_box(line.filter);
    let offset = " ".repeat(line.filter.offset as usize);

    println!(
        "{}{} {}",
        offset,
        colored_box,
        color_ips(line.line.as_str(), &line.ips, indetifier)
    );
}

fn remove_lines_without_identifier<'a>(lines: Vec<&'a str>, identifier: &'a str) -> Vec<&'a str> {
    lines
        .par_iter()
        .filter_map(|line| {
            if line.contains(identifier) {
                Some(*line)
            } else {
                None
            }
        })
        .collect()
}

struct FilteredLine<'a> {
    line: String,
    filter: &'a Filter,
    ips: Vec<String>,
}

fn filter_line<'a>(line: &'a str, profile: &'a Profile) -> Option<FilteredLine<'a>> {
    for filter in &profile.values {
        if line.contains(filter.key.as_str()) {
            let ip_regex = Regex::new(r"\b((?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b)|(\b(?:(?:[A-Fa-f0-9]{1,4}:){7}[A-Fa-f0-9]{1,4})\b)").expect("Could not create regex");
            let mut ips: Vec<String> = Vec::new();
            for cap in ip_regex.captures_iter(line) {
                ips.push(cap[0].to_string());
            }

            return Some(FilteredLine {
                line: line.trim().to_string(),
                filter,
                ips,
            });
        }
    }
    None
}

fn filter_lines<'a>(lines: Vec<&'a str>, profile: &'a Profile) -> Vec<FilteredLine<'a>> {
    lines
        .par_iter()
        .filter_map(|line| filter_line(line, profile))
        .collect()
}

fn process_stdin(args: &ArgumentParser) {
    let stdin = io::stdin();
    let profile = read_profile(&args.profile);
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line");
        let filtered_line = filter_line(line.as_str(), &profile);
        if let Some(line) = filtered_line {
            print_line_to_terminal(&line, args.identifier.as_str());
        }
    }
}

fn process_file(args: &ArgumentParser) {
    let (profile, log) = read_files(args);

    let lines: Vec<&str> = log.lines().collect();

    let lines = if !args.identifier.is_empty() {
        remove_lines_without_identifier(lines, args.identifier.as_str())
    } else {
        lines
    };
    // create a vector of tuples that will contain the filter and the line
    let filtered_lines: Vec<FilteredLine> = filter_lines(lines, &profile);

    for line in filtered_lines {
        print_line_to_terminal(&line, args.identifier.as_str());
    }
}

fn main() {
    let args = ArgumentParser::parse();
    if args.log.is_empty() {
        process_stdin(&args)
    } else {
        process_file(&args)
    }
}
