use std::env::args;
use std::io::BufWriter;

use conllx::{Sentence, WriteSentence};
use conllx_utils::{expand_tdz_morph, or_exit};
use getopts::Options;
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
    opts.optflag(
        "n",
        "no-preserve",
        "do not preserve original TüBa morph tag",
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

    let input = Input::from(matches.free.get(0));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));

    let output = Output::from(matches.free.get(1));
    let mut writer = conllx::Writer::new(BufWriter::new(or_exit(output.write())));
    for sentence in reader {
        let mut sentence = or_exit(sentence);
        expand(&mut sentence, !matches.opt_present("n"));
        or_exit(writer.write_sentence(&sentence))
    }
}

fn expand(sentence: &mut Sentence, preserve_orig: bool) {
    for token in sentence {
        or_exit(expand_tdz_morph(token, preserve_orig))
    }
}
