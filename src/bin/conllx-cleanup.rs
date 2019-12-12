use std::env::args;
use std::io::BufWriter;

use conllx::{Sentence, WriteSentence};
use conllx_utils::{or_exit, simplify_unicode, Normalization};
use getopts::Options;
use stdinout::{Input, OrExit, Output};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [INPUT_FILE] [OUTPUT_FILE]", program);
    print!("{}", opts.usage(&brief));
}

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
    let args: Vec<String> = args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt(
        "u",
        "uninorm",
        "unicode normalization: none, nfd, nfkd, nfc, nfkc (default: none)",
        "NORMALIZATION",
    );
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.len() > 2 {
        print_usage(&program, opts);
        return;
    }

    let norm = matches
        .opt_str("u")
        .as_ref()
        .map(|s| normalization_from(s).or_exit("Unknown normalization", 1))
        .unwrap_or(Normalization::None);

    let input = Input::from(matches.free.get(0));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));

    let output = Output::from(matches.free.get(1));
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
