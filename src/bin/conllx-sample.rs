extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
extern crate rand;
extern crate reservoir;
extern crate stdinout;

use std::env::args;
use std::io::BufWriter;

use conllx::io::{ReadSentence, Reader, WriteSentence, Writer};
use conllx_utils::or_exit;
use getopts::Options;
use rand::{Rng, SeedableRng, XorShiftRng};
use stdinout::{Input, OrExit, Output};

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
    opts.optopt("s", "seed", "RNG seed", "SEED");
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
    let reader = Reader::new(or_exit(input.buf_read()));

    let output = Output::from(matches.free.get(2));
    let mut writer = Writer::new(BufWriter::new(or_exit(output.write())));

    let seed = if let Some(seed_str) = matches.opt_str("s") {
        let seed_val: u32 = seed_str
            .parse()
            .or_exit(format!("Cannot not parse '{}' as an integer", seed_str), 1);
        [seed_val; 4]
    } else {
        rand::thread_rng().gen()
    };

    let mut rng = XorShiftRng::from_seed(seed);

    let sample = reservoir::sample(
        &mut rng,
        sample_size,
        reader.sentences().map(|s| or_exit(s)),
    );

    for sentence in sample {
        or_exit(writer.write_sentence(&sentence));
    }
}
