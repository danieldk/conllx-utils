extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
extern crate stdinout;

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::process;

use conllx::WriteSentence;
use conllx_utils::or_exit;
use getopts::Options;
use stdinout::{Input, Output};

fn print_usage(program: &str, opts: Options) {
    let brief = format!(
        "Usage: {} [options] ID_FILE [INPUT_FILE] [OUTPUT_FILE]",
        program
    );
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.is_empty() || matches.free.len() > 3 {
        print_usage(&program, opts);
        process::exit(1);
    }

    let ids = BufReader::new(or_exit(File::open(&matches.free[0])))
        .lines()
        .map(or_exit)
        .map(|s| or_exit(s.parse::<usize>()))
        .collect::<Vec<_>>();

    let input = Input::from(matches.free.get(1));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));
    let corpus = reader.into_iter().map(or_exit).collect::<Vec<_>>();
    let output = Output::from(matches.free.get(2));
    let mut writer = conllx::Writer::new(BufWriter::new(or_exit(output.write())));
    for id in ids {
        or_exit(writer.write_sentence(&corpus[id]));
    }
}
