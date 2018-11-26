extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
extern crate itertools;
extern crate stdinout;

use std::env::args;
use std::io::{BufWriter, Write};

use conllx::io::Reader;
use conllx::token::Token;
use getopts::Options;
use stdinout::{Input, OrExit, Output};

macro_rules! ok_or {
    ($expr:expr, $ok_expr:expr) => {
        match $expr {
            Some(val) => val,
            None => $ok_expr,
        }
    };
}

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
    let reader = Reader::new(input.buf_read().or_exit("Cannot open input", 1));

    let output = Output::from(matches.free.get(1));
    let mut writer = BufWriter::new(output.write().or_exit("Cannot open output", 1));

    for sentence in reader {
        let sentence = sentence.or_exit("Cannot read sentence", 1);
        let graph = sentence.graph();

        for idx in 0..graph.len() {
            let triple = ok_or!(graph.head(idx), continue);

            writeln!(
                writer,
                "{}\t{}\t{}\t{}\t{}",
                triple.head(),
                sentence[triple.head()]
                    .token()
                    .map(Token::form)
                    .unwrap_or("ROOT"),
                triple.relation().unwrap_or("_"),
                idx,
                sentence[idx].token().map(Token::form).unwrap_or("ROOT"),
            );
        }
    }
}
