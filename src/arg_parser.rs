use clap::Parser;

/// Search for patterns in a file and display the lines that contain those patterns
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Georgi Petrov <georgi.petrov@cognyte.com>")]
pub struct ArgumentParser {
    /// Path to profile json that will be used to configure the settings for the script
    #[clap(short, long)]
    pub profile: String,

    /// Path to log that needs to be parsed
    #[clap(short, long, default_value = "")]
    pub log: String,

    /// Path to output html file
    #[clap(long)]
    pub html: Option<String>,

    /// Turn off printing to terminal
    #[clap(short, long, default_value = "false")]
    pub no_terminal: bool,

    /// Some identifier (like IP, port, etc) that will be used to further narrow the search
    #[clap(short, long, default_value = "")]
    pub identifier: String,

    /// Should the whole line be printed
    #[clap(short, long, default_value = "false")]
    pub whole_line: bool,
}
