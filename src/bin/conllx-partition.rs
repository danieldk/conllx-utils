extern crate conllx;
extern crate conllx_utils;
extern crate flate2;
extern crate getopts;
extern crate stdinout;

use std::env::args;
use std::io::BufRead;

use conllx::{PartitioningWriter, WriteSentence};
use conllx_utils::{open_writer, or_exit};
use getopts::Options;
use stdinout::Input;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} N PREFIX SUFFIX [FILE]", program);
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

    if matches.free.len() < 3 || matches.free.len() > 4 {
        print_usage(&program, opts);
        return;
    }

    let n: usize = or_exit(matches.free[0].parse());
    let prefix = &matches.free[1];
    let suffix = &matches.free[2];

    let input = Input::from(matches.free.get(3));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));

    let writers: Vec<_> = (0..n)
        .map(|part| {
            or_exit(open_writer(&format!("{}{}{}", prefix, part, suffix)))
        })
        .collect();

    let writer = PartitioningWriter::new(writers);

    copy_sents(reader, writer)
}

fn copy_sents<R, W>(reader: conllx::Reader<R>, mut writer: W)
where
    R: BufRead,
    W: WriteSentence,
{
    for sentence in reader {
        let sentence = or_exit(sentence);
        or_exit(writer.write_sentence(&sentence))
    }
}
