use clap::Parser;

// parser.add_argument('-p', "--profile", help='Path to profile json that will be used to configure the settings for the script', required=True)
// parser.add_argument('-l', "--log", help='Path to log that needs to be parsed', required=True)
// parser.add_argument('-ht', "--html", help='Path to output html file', default=None)
// parser.add_argument('-nt', "--no_terminal", help='Turn off printing to terminal', action='store_true', default=False)
// parser.add_argument('-i', "--identifier", help='Some identifier (like IP, port, etc) that will be used to further narrow the search', default=None)
// parser.add_argument('-wl', "--whole_line", help='Should the whole line be printed', action='store_true', default=False)

/// Search for a pattern in a file and display the lines that contain it.
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