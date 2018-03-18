extern crate clap;
extern crate conllx;
extern crate conllx_utils;
extern crate stdinout;

use std::io::BufWriter;

use clap::{App, Arg};
use conllx::{Sentence, WriteSentence};
use conllx_utils::{or_exit, simplify_unicode, Normalization, DEFAULT_CLAP_SETTINGS};
use stdinout::{Input, OrExit, Output};

fn normalization_from<S>(value: S) -> Option<Normalization>
where
    S: AsRef<str>,
{
    match value.as_ref() {
        "none" => Some(Normalization::None),
        "nfd" => Some(Normalization::NFD),
        "nfkd" => Some(Normalization::NFKD),
        "nfc" => Some(Normalization::NFC),
        "nfkc" => Some(Normalization::NFKC),
        _ => None,
    }
}

fn main() {
    let matches = App::new("conllx-cleanup")
        .settings(DEFAULT_CLAP_SETTINGS)
        .arg(
            Arg::with_name("uninorm")
                .short("u")
                .long("uninorm")
                .value_name("NORMALIZATION")
                .help("Unicode normalization: none, nfd, nfkd, nfc, nfkc (default: none)")
                .takes_value(true),
        )
        .arg(Arg::with_name("INPUT").help("Input CoNLL-X file").index(1))
        .arg(
            Arg::with_name("OUTPUT")
                .help("Output CoNLL-X file")
                .index(2),
        )
        .get_matches();

    let norm = matches
        .value_of("uninorm")
        .map(|s| normalization_from(s).or_exit("Unknown normalization", 1))
        .unwrap_or(Normalization::None);

    let input = Input::from(matches.value_of("INPUT"));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));

    let output = Output::from(matches.value_of("OUTPUT"));
    let mut writer = conllx::Writer::new(BufWriter::new(or_exit(output.write())));
    for sentence in reader {
        let mut sentence = or_exit(sentence);
        cleanup(&mut sentence, norm);
        or_exit(writer.write_sentence(&sentence))
    }
}

fn cleanup(sentence: &mut Sentence, norm: Normalization) {
    for token in sentence {
        let clean_form = simplify_unicode(token.form(), norm);
        token.set_form(clean_form);
    }
}
