pub mod arg_parser;
use arg_parser::ArgumentParser;
use clap::Parser;
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

fn read_files(args: &ArgumentParser) -> (Profile, String) {
    let profile_file = std::fs::read_to_string(&args.profile).expect(format!("Unable to read profile file: {}", args.profile).as_str());
    let log_file = std::fs::read_to_string(&args.log).expect("Unable to read log file");

    let profile: Profile = serde_json::from_str(profile_file.as_str()).expect("Unable to parse profile file");

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
    let mut filtered_lines: Vec<&str> = Vec::new();
    for line in lines {
        if line.contains(identifier) {
            filtered_lines.push(line);
        }
    }
    filtered_lines
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
    let mut filtered_lines: Vec<(&Filter, String)> = Vec::new();

    for mut line in lines {
        for filter in &profile.values {
            if !line.contains(filter.key.as_str()) {
                continue;
            }

            if !args.whole_line {
                let index = line.find(filter.key.as_str()).unwrap();
                line = line[index..].trim();
            }
            
            let line = line.to_owned();
            let to_append = (filter, line);
            filtered_lines.push(to_append);
        }
    } 

    for (filter, line)  in filtered_lines {
        print_line_to_terminal(&line, filter);
    } 
    
}