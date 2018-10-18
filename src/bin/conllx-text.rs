extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
extern crate itertools;
extern crate stdinout;

use std::borrow::Cow;
use std::env::args;
use std::io::{BufWriter, Write};
use std::process;

use conllx_utils::LAYER_CALLBACKS;
use getopts::Options;
use itertools::Itertools;
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
    opts.optopt(
        "l",
        "layer",
        "layer: form, lemma, cpos, pos, headrel, or pheadrel (default: form)",
        "LAYER",
    );
    let matches = opts.parse(&args[1..]).or_exit("Cannot process options", 1);

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.len() > 2 {
        print_usage(&program, opts);
        return;
    }

    let callback = matches
        .opt_str("l")
        .as_ref()
        .map(|layer| match LAYER_CALLBACKS.get(layer.as_str()) {
            Some(c) => c,
            None => {
                println!("Unknown layer: {}", layer);
                process::exit(1)
            }
        }).unwrap_or(&LAYER_CALLBACKS["form"]);

    let input = Input::from(matches.free.get(0));
    let reader = conllx::Reader::new(input.buf_read().or_exit("Cannot open input", 1));

    let output = Output::from(matches.free.get(1));
    let mut writer = BufWriter::new(output.write().or_exit("Cannot open output", 1));

    for sentence in reader {
        let sentence = sentence.or_exit("Cannot read sentence", 1);

        writeln!(
            writer,
            "{}",
            sentence
                .iter()
                .skip(1)
                .map(|n| {
                    let token = n.token().expect("Expected token");
                    callback(token).map(Cow::into_owned).unwrap_or("_".to_owned())
                })
                .join(" ")
        ).or_exit("Cannot write sentence", 1);
    }
}
