extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
extern crate itertools;
extern crate stdinout;

use std::env::args;
use std::io::{BufRead, BufWriter};

use conllx::WriteSentence;
use getopts::Options;
use stdinout::{Input, OrExit, Output};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [INPUT_FILE] [OUTPUT_FILE]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = opts.parse(&args[1..]).or_exit("Cannot process options", 1);

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.len() > 2 {
        print_usage(&program, opts);
        return;
    }

    let input = Input::from(matches.free.get(0));
    let reader = input.buf_read().or_exit("Cannot open input", 1);

    let output = Output::from(matches.free.get(1));
    let mut writer = conllx::Writer::new(BufWriter::new(
        output.write().or_exit("Cannot open output", 1),
    ));

    for line in reader.lines() {
        let line = line.or_exit("Cannot read sentence", 1);
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let mut tokens = Vec::new();
        for token in trimmed.split(" ") {
            tokens.push(conllx::TokenBuilder::new(token).token());
        }

        if !tokens.is_empty() {
            writer
                .write_sentence(&tokens)
                .or_exit("Cannot write sentence", 1);
        }
    }
}
