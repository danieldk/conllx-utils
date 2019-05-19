extern crate conllx;
extern crate conllx_utils;
#[macro_use]
extern crate failure;
extern crate getopts;

use std::borrow::Cow;
use std::env::args;
use std::io::BufRead;
use std::process;

use conllx::Token;
use conllx_utils::{layer_callback, open_reader, or_exit, LayerCallback};
use failure::Error;
use getopts::Options;

fn feature_callback(feature: impl Into<String>) -> Box<Fn(&Token) -> Option<Cow<str>>> {
    let feature = feature.into();

    Box::new(move |token| match token.features() {
        Some(features) => features
            .as_map()
            .get(&feature)
            .map(Option::as_ref)
            .unwrap_or(None)
            .map(|s| Cow::Borrowed(s.as_str())),
        None => None,
    })
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE...", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("f", "feature", "feature to compare", "FEATURE");
    opts.optopt(
        "l",
        "layer",
        "layer(s) to compare (form, lemma, cpos, pos, features, \
         head, headrel, phead, or pheadrel, default: headrel)",
        "LAYER[,LAYER]",
    );
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let callbacks = match matches.opt_str("f") {
        Some(feature) => vec![feature_callback(feature)],
        None => process_callbacks(
            matches.opt_str("l"),
            vec![layer_callback("headrel").unwrap()],
        ),
    };

    if matches.free.len() != 2 {
        print_usage(&program, opts);
        return;
    }

    let reader1 = or_exit(open_reader(&matches.free[0]));
    let reader2 = or_exit(open_reader(&matches.free[1]));

    let (total, correct) = or_exit(compare_sentences(reader1, reader2, &callbacks));

    println!(
        "Accuracy: {:.2} ({}/{})",
        (100. * correct as f64) / total as f64,
        correct,
        total
    );
}

fn process_callbacks(
    callback_option: Option<String>,
    default: Vec<LayerCallback>,
) -> Vec<LayerCallback> {
    if callback_option.is_none() {
        return default;
    }

    let mut callbacks = Vec::new();
    for layer_str in callback_option.unwrap().split(',') {
        match layer_callback(layer_str) {
            Some(c) => callbacks.push(c),
            None => {
                println!("Unknown layer: {}", layer_str);
                process::exit(1)
            }
        }
    }

    callbacks
}

fn compare_sentences(
    reader1: conllx::Reader<Box<BufRead>>,
    reader2: conllx::Reader<Box<BufRead>>,
    diff_callbacks: &[LayerCallback],
) -> Result<(usize, usize), Error> {
    let mut total = 0;
    let mut correct = 0;

    for (sent1, sent2) in reader1.into_iter().zip(reader2.into_iter()) {
        let (sent1, sent2) = (sent1?, sent2?);

        ensure!(
            sent1.len() == sent2.len(),
            "Different number of tokens: {} {}",
            sent1.len(),
            sent2.len()
        );

        for i in 0..sent1.len() {
            for layer_callback in diff_callbacks {
                if layer_callback(&sent1[i]) == layer_callback(&sent2[i]) {
                    correct += 1
                }
            }
        }

        total += sent1.len();
    }

    Result::Ok((total, correct))
}
