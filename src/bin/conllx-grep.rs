extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
extern crate regex;
extern crate stdinout;

use std::collections::{BTreeMap, HashSet};
use std::env::args;
use std::io::BufWriter;
use std::process;

use conllx::{Features, Node, Sentence, Token, WriteSentence};
use conllx_utils::{or_exit, LayerCallback, LAYER_CALLBACKS};
use getopts::Options;
use regex::Regex;
use stdinout::{Input, Output};

fn print_usage(program: &str, opts: Options) {
    let brief = format!(
        "Usage: {} [options] EXPR [INPUT_FILE] [OUTPUT_FILE]",
        program
    );
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
    opts.optopt(
        "m",
        "mark",
        "mark maching nodes using the given feature",
        "FEATURE",
    );
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let mark_feature = matches.opt_str("m");

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

    if matches.free.len() == 0 || matches.free.len() > 3 {
        print_usage(&program, opts);
        return;
    }

    let re = or_exit(Regex::new(&matches.free[0]));
    let input = Input::from(matches.free.get(1));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));

    let output = Output::from(matches.free.get(2));
    let mut writer = conllx::Writer::new(BufWriter::new(or_exit(output.write())));
    for sentence in reader {
        let mut sentence = or_exit(sentence);

        let matches = match_indexes(&re, callback, &sentence);
        if matches.is_empty() {
            continue;
        }

        if let Some(ref feature) = mark_feature {
            for idx in matches {
                let mut features = sentence[idx]
                    .token()
                    .and_then(Token::features)
                    .map(|f| f.as_map().clone())
                    .unwrap_or(BTreeMap::new());
                features.insert(feature.clone(), None);
                match sentence[idx] {
                    Node::Root => unreachable!(),
                    Node::Token(ref mut token) => {
                        token.set_features(Some(Features::from_iter(features)))
                    }
                };
            }
        }

        or_exit(writer.write_sentence(&sentence))
    }
}

fn match_indexes(re: &Regex, callback: &LayerCallback, sentence: &Sentence) -> HashSet<usize> {
    let mut indexes = HashSet::new();

    for (idx, node) in sentence.iter().enumerate() {
        if let Some(token) = node.token().and_then(callback) {
            if re.is_match(token.as_ref()) {
                indexes.insert(idx);
            }
        }
    }

    indexes
}
