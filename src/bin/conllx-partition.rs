extern crate conllx;
extern crate conllx_utils;
extern crate getopts;

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufWriter};

use conllx::{PartitioningWriter, WriteSentence};
use conllx_utils::{or_exit, or_stdin};
use getopts::Options;

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

    let input = or_stdin(matches.free.get(1));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));

    let writers: Vec<_> = (0..n)
        .map(|part| {
            let file = or_exit(File::create(format!("{}{}{}", prefix, part, suffix)));
            conllx::Writer::new(BufWriter::new(file))
        })
        .collect();


    let writer = PartitioningWriter::new(writers);

    copy_sents(reader, writer)
}

fn copy_sents<R, W>(reader: conllx::Reader<R>, mut writer: W)
    where R: BufRead,
          W: WriteSentence
{
    for sentence in reader.sentences() {
        let sentence = or_exit(sentence);
        or_exit(writer.write_sentence(&sentence))
    }
}
