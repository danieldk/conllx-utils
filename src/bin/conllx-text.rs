extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
extern crate itertools;
extern crate stdinout;

use std::env::args;
use std::io::{BufWriter, Write};

use conllx::Token;
use conllx_utils::or_exit;
use getopts::Options;
use itertools::Itertools;
use stdinout::{Input, Output};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [INPUT_FILE] [OUTPUT_FILE]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("l", "lemmas", "extract lemmas");
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.len() > 2 {
        print_usage(&program, opts);
        return;
    }

    let input = Input::from(matches.free.get(0));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));

    let output = Output::from(matches.free.get(1));
    let mut writer = BufWriter::new(or_exit(output.write()));

    for sentence in reader {
        let sentence = or_exit(sentence);

        let mut layer_f: Box<FnMut(&Token) -> &str> = if matches.opt_present("l") {
            Box::new(|t| t.lemma().unwrap_or("_"))
        } else {
            Box::new(|t| t.form())
        };

        or_exit(writeln!(
            writer,
            "{}",
            sentence.iter().map(|t| layer_f(t)).join(" ")
        ));
    }
}
