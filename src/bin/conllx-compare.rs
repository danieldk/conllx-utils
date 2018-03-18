extern crate clap;
extern crate colored;
extern crate conllx;
extern crate conllx_utils;
#[macro_use]
extern crate failure;

use std::borrow::Cow;
use std::collections::BTreeSet;
use std::io::BufRead;
use std::process;

use clap::{App, Arg};
use colored::*;
use conllx::Token;
use conllx_utils::{open_reader, or_exit, LayerCallback, DEFAULT_CLAP_SETTINGS, LAYER_CALLBACKS};
use failure::Error;

fn main() {
    let matches = App::new("conllx-compare")
        .settings(DEFAULT_CLAP_SETTINGS)
        .arg(
            Arg::with_name("layer")
                .short("l")
                .long("layer")
                .value_name("LAYER[,LAYER]*")
                .help("layer(s) to compare (form, lemma, cpos, pos, features, head, headrel, phead, or pheadrel, default: headrel)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("show_layer")
                .short("s")
                .long("show")
                .value_name("LAYER[,LAYER]*")
                .help("extra layer(s) to show from the first file (form, lemma, cpos, pos, features, head, headrel, phead, or pheadrel, default: form)")
                .takes_value(true),
        )
        .arg(Arg::with_name("INPUT1").help("First CoNLL-X file to compare").index(1).required(true))
        .arg(
            Arg::with_name("INPUT2")
                .help("Second CoNLL-X file to compare")
                .index(2).required(true),
        )
        .get_matches();

    let callbacks = process_callbacks(matches.value_of("layer"), vec![&LAYER_CALLBACKS["headrel"]]);
    let show_callbacks = process_callbacks(
        matches.value_of("show_layer"),
        vec![&LAYER_CALLBACKS["form"]],
    );

    let reader1 = or_exit(open_reader(matches.value_of("INPUT1").unwrap()));
    let reader2 = or_exit(open_reader(matches.value_of("INPUT2").unwrap()));

    or_exit(compare_sentences(
        reader1,
        reader2,
        &callbacks,
        &show_callbacks,
    ));
}

fn process_callbacks(
    callback_option: Option<&str>,
    default: Vec<&'static LayerCallback>,
) -> Vec<&'static LayerCallback> {
    if callback_option.is_none() {
        return default;
    }

    let mut callbacks = Vec::new();
    for layer_str in callback_option.unwrap().split(',') {
        match LAYER_CALLBACKS.get(layer_str) {
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
    diff_callbacks: &[&LayerCallback],
    show_callbacks: &[&LayerCallback],
) -> Result<(), Error> {
    for (sent1, sent2) in reader1.into_iter().zip(reader2.into_iter()) {
        let (sent1, sent2) = (sent1?, sent2?);

        let diff = diff_indices(&sent1, &sent2, diff_callbacks)?;

        if !diff.is_empty() {
            print_diff(&sent1, &sent2, diff_callbacks, show_callbacks);
            println!();
        }
    }

    Result::Ok(())
}

fn print_diff(
    tokens1: &[Token],
    tokens2: &[Token],
    diff_callbacks: &[&LayerCallback],
    show_callbacks: &[&LayerCallback],
) {
    for idx in 0..tokens1.len() {
        let mut columns = Vec::new();

        for callback in show_callbacks {
            columns.push(
                callback(&tokens1[idx])
                    .unwrap_or(Cow::Borrowed("_"))
                    .into_owned(),
            );
        }

        for callback in diff_callbacks {
            let col1 = callback(&tokens1[idx]).unwrap_or(Cow::Borrowed("_"));
            let col2 = callback(&tokens2[idx]).unwrap_or(Cow::Borrowed("_"));

            if col1 != col2 {
                columns.push(format!("{}", col1.red()));
                columns.push(format!("{}", col2.red()));
            } else {
                columns.push(col1.into_owned());
                columns.push(col2.into_owned());
            }
        }

        println!("{}\t{}", idx + 1, columns.join("\t"));
    }
}

fn diff_indices(
    tokens1: &[Token],
    tokens2: &[Token],
    diff_callbacks: &[&LayerCallback],
) -> Result<BTreeSet<usize>, Error> {
    ensure!(
        tokens1.len() == tokens2.len(),
        "Different number of tokens: {} {}",
        tokens1.len(),
        tokens2.len()
    );

    let mut indices = BTreeSet::new();

    'tokenloop: for i in 0..tokens1.len() {
        for layer_callback in diff_callbacks {
            if layer_callback(&tokens1[i]) != layer_callback(&tokens2[i]) {
                indices.insert(i);
                continue 'tokenloop;
            }
        }
    }

    Result::Ok(indices)
}
