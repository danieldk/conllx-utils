extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
extern crate stdinout;

use std::env::args;
use std::io::BufWriter;

use conllx::{Deprojectivize, HeadProjectivizer, Projectivize, WriteSentence};
use conllx_utils::or_exit;
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
    opts.optflag("d", "deproj", "deprojectivize (head strategy)");
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.len() > 2 {
        print_usage(&program, opts);
        return;
    }

    let projectivizer = HeadProjectivizer::new();
    let deproj = matches.opt_present("d");

    let input = Input::from(matches.free.get(0));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));

    let output = Output::from(matches.free.get(1));
    let mut writer = conllx::Writer::new(BufWriter::new(or_exit(output.write())));
    for sentence in reader {
        let mut sentence = or_exit(sentence);

        if deproj {
            or_exit(projectivizer.deprojectivize(&mut sentence));
            or_exit(writer.write_sentence(&sentence));
        } else {
            or_exit(projectivizer.projectivize(&mut sentence));
            or_exit(writer.write_sentence(&sentence));
        }
    }
}
