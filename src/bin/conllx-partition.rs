extern crate conllx;
extern crate conllx_utils;
extern crate flate2;
extern crate getopts;
extern crate stdinout;

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};

use conllx::{PartitioningWriter, WriteSentence, Writer};
use conllx_utils::or_exit;
use flate2::Compression;
use flate2::write::GzEncoder;
use getopts::Options;
use stdinout::Input;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} N PREFIX SUFFIX [FILE]", program);
    print!("{}", opts.usage(&brief));
}

fn create_writer<S>(filename: S, gzip: bool) -> Writer<Box<Write>>
where
    S: Into<String>,
{
    let file = or_exit(File::create(filename.into()));
    if gzip {
        conllx::Writer::new(Box::new(
            BufWriter::new(GzEncoder::new(file, Compression::Default)),
        ))
    } else {
        conllx::Writer::new(Box::new(BufWriter::new(file)))
    }

}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("z", "gzip", "gzip-compress output files");
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
            create_writer(
                format!("{}{}{}", prefix, part, suffix),
                matches.opt_present("z"),
            )
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
