# Beauty Log Analyzer

This is a tool for creating stuture and prettifying log files. \
Profile json files are used to do this. \
For example of such file, refer to Profile1.josn.

## Prerequisites

The main prerequisite is having Rust and Cargo set up.

## Example usage

The basic usege is to supply a log file and a profile and this will pretiffy it in the terminal:

    ./log_beautifier -p Profile1.json -l raw.log

If you want to save the output to a pretiffied html, you can run the following:

    ./log_beautifier -p Profile1.json -l raw.log -ht output.html   

If you don't ned to see the output in the terminal(this makes the script faster), you can run it with the -nt flag:

    ./log_beautifier -p Profile1.json -l "raw copy.log" -ht output.html -nt 

If you want to specify an identifier based on which to filter the log file, you can use the -i flag:

    ./log_beautifier -p Profile1.json -l raw.log -ht output.html -i "identifier"

    e.g. ./log_beautifier -p Profile1.json -l raw.log -i 192.168.137.245

If you want to see the whole line (not only what is after the key), you can use the -wl flag:

    ./log_beautifier -p Profile1.json -l raw.log -wl

If you want to used together with another program, you can pipe into it:

    cat raw.log | ./log_beautifier -p Profile1.json -wl