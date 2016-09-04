extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
extern crate regex;

use std::env::args;

use conllx::{Sentence, WriteSentence};
use conllx_utils::{LAYER_CALLBACKS, LayerCallback, or_exit, or_stdin, or_stdout};
use getopts::Options;
use regex::Regex;
use std::process;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] EXPR [INPUT_FILE] [OUTPUT_FILE]",
                        program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("l",
                "layer",
                "layer: form, lemma, cpos, pos, headrel, or pheadrel (default: form)",
                "LAYER");
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let callback = matches.opt_str("l")
        .as_ref()
        .map(|layer| {
            match LAYER_CALLBACKS.get(layer.as_str()) {
                Some(c) => c,
                None => {
                    println!("Unknown layer: {}", layer);
                    process::exit(1)
                }
            }
        })
        .unwrap_or(&LAYER_CALLBACKS["form"]);

    if matches.free.len() == 0 || matches.free.len() > 3 {
        print_usage(&program, opts);
        return;
    }

    let re = or_exit(Regex::new(&matches.free[0]));
    let input = or_stdin(matches.free.get(1));

    let reader = conllx::Reader::new(or_exit(input.buf_read()));
    let mut writer = conllx::Writer::new(or_stdout(matches.free.get(2)));
    for sentence in reader.sentences() {
        let sentence = or_exit(sentence);
        if match_sentence(&re, callback, &sentence) {
            or_exit(writer.write_sentence(&sentence))
        }
    }
}

fn match_sentence(re: &Regex, callback: &LayerCallback, sentence: &Sentence) -> bool {
    for token in sentence {
        match callback(token).as_ref() {
            Some(token) => {
                if re.is_match(&token) {
                    return true;
                }
            }
            None => (),
        }
    }

    false
}
