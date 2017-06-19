extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
extern crate rand;
extern crate reservoir;
extern crate stdinout;

use std::env::args;
use std::io::BufWriter;

use conllx::{ReadSentence, WriteSentence};
use conllx_utils::or_exit;
use getopts::Options;
use stdinout::{Input, Output};

fn print_usage(program: &str, opts: Options) {
    let brief = format!(
        "Usage: {} [options] SAMPLE_SIZE [INPUT_FILE] [OUTPUT_FILE]",
        program
    );
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.is_empty() || matches.free.len() > 3 {
        print_usage(&program, opts);
        return;
    }

    let sample_size = or_exit(matches.free[0].parse());

    let input = Input::from(matches.free.get(1));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));

    let output = Output::from(matches.free.get(2));
    let mut writer = conllx::Writer::new(BufWriter::new(or_exit(output.write())));

    let mut rng = rand::weak_rng();
    let sample = reservoir::sample(
        &mut rng,
        sample_size,
        reader.sentences().map(|s| or_exit(s)),
    );
    for sentence in sample {
        or_exit(writer.write_sentence(&sentence));
    }
}
