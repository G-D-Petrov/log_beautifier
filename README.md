# Beauty Log Analyzer

This is a tool for creating stuture and prettifying log files. \
Profile json files are used to do this. \
For example of such file, refer to Profile1.josn.

## Prerequisites

The main prerequisite is Python 3.6+.

## Example usage

The basic usege is to supply a log file and a profile and this will pretiffy it in the terminal:

    python main.py -p Profile1.json -l raw.log

If you want to save the output to a pretiffied html, you can run the following:

    python main.py -p Profile1.json -l raw.log -ht output.html   

If you don't ned to see the output in the terminal(this makes the script faster), you can run it with the -nt flag:

    python main.py -p Profile1.json -l "raw copy.log" -ht output.html -nt 

If you want to specify an identifier based on which to filter the log file, you can use the -i flag:

    python main.py -p Profile1.json -l raw.log -ht output.html -i "identifier"

    e.g. python main.py -p Profile1.json -l raw.log -i 192.168.137.245

If you want to see the whole line (not only what is after the key), you can use the -wl flag:

    python main.py -p Profile1.json -l raw.log -wl

For performance testing, you can use the -t flag:

    time ./log_beautifier -p /home/vagrant/mde/log_beautifier/VoIPSessionDebug.json -l /home/vagrant/mde/log_beautifier/loop_5_times.log --whole-line