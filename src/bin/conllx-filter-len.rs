use std::env::args;
use std::io::BufWriter;
use std::process;

use conllx::WriteSentence;
use getopts::Options;
use stdinout::{Input, OrExit, Output};

fn print_usage(program: &str, opts: Options) {
    let brief = format!(
        "Usage: {} [options] MAX_LEN [INPUT_FILE] [OUTPUT_FILE]",
        program
    );
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = opts.parse(&args[1..]).or_exit("Cannot process options", 1);

    if matches.opt_present("h") {
        print_usage(&program, opts);
        process::exit(0);
    }

    if matches.free.is_empty() || matches.free.len() > 3 {
        print_usage(&program, opts);
        process::exit(1);
    }

    let max_len: usize = matches.free[0]
        .parse()
        .or_exit("Cannot parse maximum length", 1);

    let input = Input::from(matches.free.get(1));
    let reader = conllx::Reader::new(input.buf_read().or_exit("Cannot open input for reading", 1));

    let output = Output::from(matches.free.get(2));
    let mut writer = conllx::Writer::new(BufWriter::new(
        output.write().or_exit("Cannot open output for writing", 1),
    ));
    for sentence in reader {
        let sentence = sentence.or_exit("Cannot read sentence", 1);

        if sentence.len() <= max_len {
            writer
                .write_sentence(&sentence)
                .or_exit("Cannot write sentence", 1);
        }
    }
}
